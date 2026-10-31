//! Les cinq vues de la carte « Ta conso jour par jour » (maquettes du 23/10/2026) : « Aujourd'hui
//! d'abord », « Les lignes », « La trajectoire », « Les batteries » (par défaut) et « La semaine en
//! une barre ». Toutes lisent les MÊMES chiffres (`modele::Vue`) ; ce module ne fait que les mettre
//! en forme (textes, fractions, tracés), sans interface, donc se teste sans Slint.
//!
//! Le choix de la vue vit dans un petit fichier `vue.txt` du dossier de données.

use std::f64::consts::TAU;
use std::path::Path;

use crate::fr;
use crate::modele::{EtatJour, Vue};
use crate::textes::{Cle, fmt, t};

pub const NB_VUES: usize = 5;
/// « Les batteries » (choix par défaut, 23/10/2026).
pub const VUE_DEFAUT: usize = 3;
/// Les noms écrits dans `vue.txt`, dans l'ordre des onglets.
pub const NOMS_VUES: [&str; NB_VUES] = ["aujourdhui", "lignes", "trajectoire", "batteries", "semaine"];
const FICHIER_VUE: &str = "vue.txt";

/// La vue rangée dans le dossier de données ; la vue par défaut si le fichier manque ou ne dit rien de connu.
pub fn lire_vue(donnees: &Path) -> usize {
    let Ok(texte) = std::fs::read_to_string(donnees.join(FICHIER_VUE)) else { return VUE_DEFAUT };
    let nom = texte.trim().to_lowercase();
    NOMS_VUES.iter().position(|n| *n == nom).unwrap_or(VUE_DEFAUT)
}

/// Range la vue choisie. Rend faux si le fichier n'a pas pu être écrit (l'appli continue sans).
pub fn ecrire_vue(donnees: &Path, vue: usize) -> bool {
    let Some(nom) = NOMS_VUES.get(vue) else { return false };
    std::fs::create_dir_all(donnees).is_ok() && std::fs::write(donnees.join(FICHIER_VUE), format!("{nom}\n")).is_ok()
}

// ---------------------------------------------------------------- les jours (communs aux vues)

pub const PASSE: i32 = 0;
pub const PASSE_DEPASSE: i32 = 1;
pub const EN_COURS: i32 = 2;
pub const A_VENIR: i32 = 3;
/// Jour d'avant la première lecture de l'appli : rien n'est connu.
pub const INCONNU: i32 = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct JourCarte {
    /// `jeu→ven` : les deux jours que le bloc chevauche (un seul nom si la remise à zéro est à minuit).
    pub nom: String,
    pub genre: i32,
    /// Le jour en cours a dépassé son budget.
    pub depasse: bool,
    /// Le chiffre à retenir : écart du jour passé (`+5,3`), `reste 13` aujourd'hui, `—`.
    pub valeur: String,
    /// La même chose en plus court, pour une pastille (`13` au lieu de `reste 13`).
    pub court: String,
    /// Sous la batterie : `9 sur 14,3 %`, `à venir`.
    pub sous: String,
    /// Dans une ligne : `9 % sur 14,3`, `budget 23 %`.
    pub ligne: String,
    /// `2,4 M`, ou vide.
    pub tokens: String,
    /// Batterie : part du budget du jour encore intacte (0 à 1).
    pub niveau: f32,
    /// Ligne : consommé (au plus le budget) sur l'échelle commune.
    pub conso_part: f32,
    /// Ligne : ce qui dépasse le budget, sur la même échelle.
    pub exces_part: f32,
    /// Ligne : place du repère de budget sur la même échelle.
    pub repere: f32,
}

/// La puce à côté du titre : les VRAIES heures locales du début et de la fin de la semaine de la remise à
/// zéro (`jeu 13h → jeu 12h` la semaine du passage à l'heure d'hiver à Paris, `jeu 12h → jeu 13h` en fin
/// mars, `jeu 13h → jeu 13h` une semaine ordinaire). Même règle que la carte « Ton forfait revient » :
/// l'heure que dit l'horloge, rien d'autre ; elles suivent les données.
pub fn puce(v: &Vue) -> String {
    fmt(Cle::Puce, &[&fr::borne_jour(v.debut), &fr::borne_jour(v.reset)])
}

pub fn jours_carte(v: &Vue) -> Vec<JourCarte> {
    let maxi = v.jours.iter().flat_map(|j| [j.budget.unwrap_or(0.0), j.conso.unwrap_or(0.0)]).fold(1.0_f64, f64::max) * 1.12;
    v.jours
        .iter()
        .map(|j| {
            let nom = fr::nom_du_jour(j.debut, v.reset);
            let tokens = if j.etat == EtatJour::AVenir { String::new() } else { j.tokens.map(fr::tokens).unwrap_or_default() };
            let vide = JourCarte {
                nom,
                genre: INCONNU,
                depasse: false,
                valeur: "—".into(),
                court: "—".into(),
                sous: t(Cle::AvantLecture).into(),
                ligne: t(Cle::AvantLecture).into(),
                tokens,
                niveau: 0.0,
                conso_part: 0.0,
                exces_part: 0.0,
                repere: 0.0,
            };
            if j.etat == EtatJour::AVenir {
                let b = j.budget.unwrap_or(0.0);
                return JourCarte {
                    genre: A_VENIR,
                    valeur: fr::nombre(b),
                    court: fr::nombre(b),
                    sous: t(Cle::AVenir).into(),
                    ligne: fmt(Cle::LigneBudget, &[&fr::nombre(b)]),
                    niveau: 1.0,
                    repere: (b / maxi) as f32,
                    ..vide
                };
            }
            let (Some(c), Some(b)) = (j.conso, j.budget) else { return vide };
            let ecart = b - c;
            let depasse = c > b + 1e-9;
            let (genre, valeur, court) = if j.etat == EtatJour::EnCours {
                if v.reste_jour > 0.0 {
                    (EN_COURS, fmt(Cle::Reste, &[&fr::nombre(v.reste_jour)]), fr::nombre(v.reste_jour))
                } else {
                    (EN_COURS, fr::nombre_signe(ecart), fr::nombre_signe(ecart))
                }
            } else {
                (if depasse { PASSE_DEPASSE } else { PASSE }, fr::nombre_signe(ecart), fr::nombre_signe(ecart))
            };
            JourCarte {
                genre,
                depasse,
                valeur,
                court,
                sous: fmt(Cle::SousBatterie, &[&fr::nombre(c), &fr::nombre(b)]),
                ligne: fmt(Cle::LigneConso, &[&fr::nombre(c), &fr::nombre(b)]),
                niveau: if depasse || b <= 0.0 { 0.0 } else { ((b - c) / b) as f32 },
                conso_part: (c.min(b) / maxi) as f32,
                exces_part: if depasse { ((c - b) / maxi) as f32 } else { 0.0 },
                repere: (b / maxi) as f32,
                ..vide
            }
        })
        .collect()
}

// ---------------------------------------------------------------- 1 : aujourd'hui d'abord

const RAYON: f64 = 78.0;

/// Tracé (boîte de 200 × 200) d'un arc qui part du haut et tourne dans le sens des aiguilles.
pub fn arc(prop: f64) -> String {
    if prop < 0.002 {
        return String::new();
    }
    let p = prop.min(0.9999);
    let a = p * TAU;
    format!("M 100 {:.2} A {R} {R} 0 {} 1 {:.2} {:.2}", 100.0 - RAYON, u8::from(p > 0.5), 100.0 + RAYON * a.sin(), 100.0 - RAYON * a.cos(), R = RAYON)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Aujourdhui {
    pub arc: String,
    pub depasse: bool,
    pub etiquette: String,
    pub valeur: String,
    pub sous: String,
    pub resume: String,
}

pub fn aujourdhui(v: &Vue) -> Aujourdhui {
    let j = &v.jours[v.aujourdhui];
    let (c, b) = (j.conso.unwrap_or(0.0), j.budget.unwrap_or(0.0));
    let depasse = c > b + 1e-9;
    let part = fr::nombre(v.reste_suivants.unwrap_or(0.0));
    let suite = match v.nb_suivants {
        0 => t(Cle::AujDernierJour).to_string(),
        1 => fmt(Cle::AujSuiteUn, &[&part]),
        n => fmt(Cle::AujSuiteN, &[&n, &part]),
    };
    Aujourdhui {
        arc: arc(if b > 0.0 { (c / b).min(1.0) } else { 0.0 }),
        depasse,
        etiquette: if depasse { t(Cle::EtiqDepasse) } else { t(Cle::CarteReste) }.into(),
        valeur: if depasse { fr::nombre_signe(b - c) } else { fr::nombre(v.reste_jour) },
        sous: fmt(Cle::AujSous, &[&fr::nombre(b)]),
        resume: fmt(Cle::AujResume, &[&fr::nombre(v.reste), &fr::nombre(v.reste_jour), &suite]),
    }
}

// ---------------------------------------------------------------- 3 : la trajectoire

/// Boîte des tracés : 700 (7 jours) × 300 (0 à 100 %).
pub const TRAJ_L: f64 = 700.0;
pub const TRAJ_H: f64 = 300.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Trajectoire {
    /// Zone entre la courbe et le rythme, là où tu es en avance.
    pub aire_avance: String,
    /// Idem là où tu es en retard.
    pub aire_retard: String,
    /// La ligne « pile au rythme ».
    pub rythme: String,
    pub cumul: String,
    /// Le chemin permis : fin du jour en cours, puis une part par jour jusqu'à 100 %.
    pub permis: String,
    /// Position de « maintenant » (0 à 1 sur chaque axe, y vers le bas).
    pub x: f32,
    pub y: f32,
    pub texte: String,
    pub en_ce_moment: String,
}

fn px(d: f64) -> f64 {
    TRAJ_L * d / 7.0
}
fn py(p: f64) -> f64 {
    TRAJ_H * (1.0 - p.clamp(0.0, 100.0) / 100.0)
}
fn poly(pts: &[(f64, f64)]) -> String {
    let mut s = String::new();
    for (i, (d, p)) in pts.iter().enumerate() {
        s.push_str(&format!("{} {:.2} {:.2} ", if i == 0 { "M" } else { "L" }, px(*d), py(*p)));
    }
    s
}

pub fn trajectoire(v: &Vue) -> Trajectoire {
    // Début de chaque jour connu : la part consommée avant lui se retrouve par son budget
    // (budget = ce qui reste ÷ jours restants).
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for (i, j) in v.jours.iter().enumerate().take(v.aujourdhui + 1) {
        if let Some(b) = j.budget {
            pts.push((i as f64, (100.0 - b * (7 - i) as f64).clamp(0.0, 100.0)));
        }
    }
    if pts.is_empty() {
        pts.push((0.0, 0.0));
    }
    let maintenant = (v.ecoule / 100.0 * 7.0).max(v.aujourdhui as f64).min(7.0);
    let utilise = v.utilise.clamp(0.0, 100.0);
    pts.push((maintenant, utilise));

    // Aires : chaque segment est coupé là où la courbe croise « pile au rythme » (y = 100 x / 7).
    let (mut avance, mut retard) = (String::new(), String::new());
    let mut zone = |xa: f64, aa: f64, xb: f64, ab: f64, en_avance: bool| {
        let s = format!("M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z ", px(xa), py(aa), px(xb), py(ab), px(xb), py(100.0 * xb / 7.0), px(xa), py(100.0 * xa / 7.0));
        if en_avance { avance.push_str(&s) } else { retard.push_str(&s) }
    };
    for w in pts.windows(2) {
        let ((x0, a0), (x1, a1)) = (w[0], w[1]);
        let (s0, s1) = (a0 - 100.0 * x0 / 7.0, a1 - 100.0 * x1 / 7.0);
        if s0 * s1 >= 0.0 {
            zone(x0, a0, x1, a1, s0 + s1 <= 0.0);
        } else {
            let t = s0 / (s0 - s1);
            let xc = x0 + t * (x1 - x0);
            let ac = 100.0 * xc / 7.0;
            zone(x0, a0, xc, ac, s0 < 0.0);
            zone(xc, ac, x1, a1, s1 < 0.0);
        }
    }

    let mut permis = vec![(maintenant, utilise)];
    let mut cumul_permis = (utilise + v.reste_jour).min(100.0);
    permis.push(((v.aujourdhui + 1) as f64, cumul_permis));
    for i in (v.aujourdhui + 1)..7 {
        cumul_permis = (cumul_permis + v.reste_suivants.unwrap_or(0.0)).min(100.0);
        permis.push(((i + 1) as f64, cumul_permis));
    }

    let ecart = v.ecart_rythme;
    let en_ce_moment = if ecart.abs() < 0.5 {
        t(Cle::TrajPile).to_string()
    } else if ecart < 0.0 {
        fmt(Cle::TrajAvance, &[&fr::nombre(ecart.abs())])
    } else {
        fmt(Cle::TrajRetard, &[&fr::nombre(ecart)])
    };
    Trajectoire {
        aire_avance: avance,
        aire_retard: retard,
        rythme: poly(&[(0.0, 0.0), (7.0, 100.0)]),
        cumul: poly(&pts),
        permis: poly(&permis),
        x: (maintenant / 7.0) as f32,
        y: (1.0 - utilise / 100.0) as f32,
        texte: fmt(Cle::TrajUtilises, &[&fr::nombre(v.utilise)]),
        en_ce_moment,
    }
}

// ---------------------------------------------------------------- 5 : la semaine en une barre

pub const SEG_PASSE: i32 = 0;
pub const SEG_PASSE_DEPASSE: i32 = 1;
pub const SEG_AUJOURDHUI: i32 = 2;
pub const SEG_PERMIS: i32 = 3;
pub const SEG_A_VENIR: i32 = 4;
pub const SEG_INCONNU: i32 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    /// Début et largeur, en part de la barre entière (0 à 1).
    pub debut: f32,
    pub part: f32,
    pub genre: i32,
    pub texte: String,
    pub etiquette: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Semaine {
    pub segments: Vec<Segment>,
    /// Où en est la semaine (0 à 1 sur la barre).
    pub repere: f32,
    pub repere_texte: String,
    pub resume: String,
}

pub fn semaine(v: &Vue) -> Semaine {
    // (largeur en % du forfait, genre, texte dans la barre, étiquette dessous)
    let mut brut: Vec<(f64, i32, String, String)> = Vec::new();
    let connu: f64 = v.jours.iter().filter(|j| j.etat != EtatJour::AVenir).filter_map(|j| j.conso).sum();
    if v.utilise - connu > 0.05 {
        brut.push((v.utilise - connu, SEG_INCONNU, String::new(), String::new()));
    }
    let nom = |i: usize| fr::nom_du_jour(v.jours[i].debut, v.reset);
    for (i, j) in v.jours.iter().enumerate().take(v.aujourdhui + 1) {
        let Some(c) = j.conso else { continue };
        let texte = if c >= 4.0 { fr::nombre(c) } else { String::new() };
        if i == v.aujourdhui {
            // le nom d'aujourd'hui se lit sous le morceau « permis » (le plus large des deux), sinon sous « déjà »
            let nom_deja = if v.reste_jour > 0.0 { String::new() } else { nom(i) };
            brut.push((c, SEG_AUJOURDHUI, texte, nom_deja));
            if v.reste_jour > 0.0 {
                let t = if v.reste_jour >= 5.0 { fr::nombre(v.reste_jour) } else { String::new() };
                brut.push((v.reste_jour, SEG_PERMIS, t, nom(i)));
            }
        } else {
            let depasse = j.ecart.is_some_and(|e| e < -1e-9);
            brut.push((c, if depasse { SEG_PASSE_DEPASSE } else { SEG_PASSE }, texte, nom(i)));
        }
    }
    for i in (v.aujourdhui + 1)..7 {
        let part = v.reste_suivants.unwrap_or(0.0);
        brut.push((part, SEG_A_VENIR, if part >= 6.0 { fr::nombre(part) } else { String::new() }, nom(i)));
    }
    let total: f64 = brut.iter().map(|b| b.0).sum::<f64>().max(1e-9);
    let mut debut = 0.0;
    let segments = brut
        .into_iter()
        .map(|(w, genre, texte, etiquette)| {
            let part = w / total;
            let s = Segment { debut: debut as f32, part: part as f32, genre, texte, etiquette };
            debut += part;
            s
        })
        .collect();
    Semaine {
        segments,
        repere: (v.ecoule / total).clamp(0.0, 1.0) as f32,
        repere_texte: fmt(Cle::SemRepere, &[&fr::nombre(v.ecoule)]),
        resume: fmt(Cle::SemResume, &[&fr::nombre(v.utilise), &fr::nombre(v.reste)]),
    }
}

// ---------------------------------------------------------------- l'aide, une ligne par vue

/// Une courte légende sous la vue choisie (dans l'ordre des onglets).
pub fn aides(v: &Vue, tokens_connus: bool) -> Vec<String> {
    let mut fin = String::new();
    if v.debut_estime || v.jours.iter().any(|j| j.etat != EtatJour::AVenir && j.conso.is_none()) {
        fin.push_str("  ");
        fin.push_str(t(Cle::AideAvantLecture));
    }
    if tokens_connus {
        fin.push_str("  ");
        fin.push_str(t(Cle::AideTokens));
    }
    [Cle::Aide0, Cle::Aide1, Cle::Aide2, Cle::Aide3, Cle::Aide4].iter().map(|a| format!("{}{fin}", t(*a))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modele::{Semaine as SemaineDonnees, construire};
    use crate::temps::JOUR_S;

    // Semaine de l'exemple du 14/10/2026 : jeu 08/10 13:00 → jeu 15/10 13:00 (Paris, UTC+2).
    const RESET: i64 = 1_792_062_000;
    const DEBUT: i64 = RESET - 7 * JOUR_S;
    const MAINTENANT: i64 = DEBUT + 136 * 3_600 + 48 * 60;

    /// Conso par jour : 9, 10, 8, 12, 15, puis 10 en cours (64 % en tout), comme l'exemple du noyau.
    fn vue_exemple() -> Vue {
        let ps = [0.0, 9.0, 19.0, 27.0, 39.0, 54.0];
        let mut points: Vec<(i64, f64)> = ps.iter().enumerate().map(|(i, p)| (DEBUT + i as i64 * JOUR_S, *p)).collect();
        points.push((MAINTENANT, 64.0));
        construire(&SemaineDonnees { reset: RESET, points }, MAINTENANT, Some(&[2_400_000, 2_700_000, 2_100_000, 3_200_000, 4_000_000, 2_700_000, 0])).expect("vue")
    }

    /// Le « milieu de semaine » de la maquette : 18, 9, 13, puis 5 en cours (jour 4, le jour 1 dépassé).
    fn vue_milieu() -> Vue {
        let ps = [0.0, 18.0, 27.0, 40.0];
        let mut points: Vec<(i64, f64)> = ps.iter().enumerate().map(|(i, p)| (DEBUT + i as i64 * JOUR_S, *p)).collect();
        let maintenant = DEBUT + 3 * JOUR_S + JOUR_S * 2 / 5;
        points.push((maintenant, 45.0));
        construire(&SemaineDonnees { reset: RESET, points }, maintenant, None).expect("vue")
    }

    fn proche(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn les_noms_portent_les_deux_jours_du_bloc() {
        let noms: Vec<String> = jours_carte(&vue_exemple()).into_iter().map(|j| j.nom).collect();
        assert_eq!(noms, ["jeu→ven", "ven→sam", "sam→dim", "dim→lun", "lun→mar", "mar→mer", "mer→jeu"]);
    }

    #[test]
    fn la_puce_dit_l_heure_une_seule_fois() {
        assert_eq!(puce(&vue_exemple()), "jours de 24 h : jeu 13h → jeu 13h");
    }

    #[test]
    fn les_batteries_et_les_lignes_de_l_exemple() {
        let j = jours_carte(&vue_exemple());
        assert_eq!(j.len(), 7);
        // jour 1 : 9 sur 14,3 → 5,3 d'avance, batterie à 37 %
        assert_eq!(j[0].genre, PASSE);
        assert_eq!(j[0].valeur, "+5,3");
        assert_eq!(j[0].sous, "9 sur 14,3 %");
        assert_eq!(j[0].ligne, "9 % sur 14,3");
        assert!(proche(j[0].niveau, (14.285714 - 9.0) / 14.285714), "niveau {}", j[0].niveau);
        assert_eq!(j[0].tokens, "2,4 M");
        // aujourd'hui (jour 6) : 10 sur 23, il reste 13
        assert_eq!(j[5].genre, EN_COURS);
        assert_eq!(j[5].valeur, "reste 13");
        assert_eq!(j[5].court, "13");
        assert!(!j[5].depasse);
        // jour à venir : le budget, la batterie pleine, pas de tokens
        assert_eq!(j[6].genre, A_VENIR);
        assert_eq!(j[6].valeur, "23");
        assert_eq!(j[6].ligne, "budget 23 %");
        assert!(proche(j[6].niveau, 1.0));
        assert_eq!(j[6].tokens, "");
    }

    #[test]
    fn un_jour_depasse_a_une_batterie_vide_et_un_exces() {
        let j = jours_carte(&vue_milieu());
        // jour 1 : 18 % pour 14,3 permis → dépassé de 3,7
        assert_eq!(j[0].genre, PASSE_DEPASSE);
        assert_eq!(j[0].valeur, "−3,7");
        assert!(proche(j[0].niveau, 0.0));
        assert!(j[0].exces_part > 0.0);
        assert!(proche(j[0].conso_part, j[0].repere), "le consommé s'arrête au repère de budget");
        // les parts de ligne tiennent dans l'échelle commune
        assert!(j.iter().all(|x| x.conso_part + x.exces_part <= 1.0 && x.repere <= 1.0));
    }

    #[test]
    fn un_jour_avant_la_premiere_lecture_est_inconnu() {
        let maintenant = DEBUT + 5 * JOUR_S + 3_600;
        let v = construire(&SemaineDonnees { reset: RESET, points: vec![(maintenant, 64.0)] }, maintenant, None).unwrap();
        let j = jours_carte(&v);
        assert_eq!(j[0].genre, INCONNU);
        assert_eq!(j[0].valeur, "—");
        assert!(proche(j[0].niveau, 0.0));
        assert_eq!(j[5].genre, EN_COURS);
        assert!(aides(&v, false)[3].contains("avant la première lecture"));
        assert!(!aides(&vue_exemple(), false)[3].contains("avant la première lecture"));
    }

    #[test]
    fn l_arc_de_aujourdhui() {
        assert_eq!(arc(0.0), "");
        assert!(arc(0.25).starts_with("M 100 22.00 A 78 78 0 0 1 178.00 100.00"), "{}", arc(0.25));
        assert!(arc(0.75).contains(" 0 1 1 "), "{}", arc(0.75));
        // un anneau plein ne dégénère pas en point
        assert!(arc(1.0).contains("A 78 78 0 1 1"));
        let a = aujourdhui(&vue_exemple());
        assert_eq!(a.etiquette, "IL TE RESTE");
        assert_eq!(a.valeur, "13");
        assert_eq!(a.sous, "sur 23 % permis");
        assert_eq!(a.resume, "Il te reste 36 % du forfait : 13 % pour finir aujourd’hui. Le dernier jour : 23 %.");
        assert!(!a.depasse);
    }

    #[test]
    fn aujourdhui_depasse() {
        // 30 % consommés aujourd'hui pour 23 permis
        let ps = [0.0, 9.0, 19.0, 27.0, 39.0, 54.0];
        let mut points: Vec<(i64, f64)> = ps.iter().enumerate().map(|(i, p)| (DEBUT + i as i64 * JOUR_S, *p)).collect();
        points.push((MAINTENANT, 84.0));
        let v = construire(&SemaineDonnees { reset: RESET, points }, MAINTENANT, None).unwrap();
        let a = aujourdhui(&v);
        assert!(a.depasse);
        assert_eq!(a.etiquette, "DÉPASSÉ");
        assert_eq!(a.valeur, "−7");
        let j = jours_carte(&v);
        assert!(j[5].depasse);
        assert_eq!(j[5].valeur, "−7");
    }

    #[test]
    fn la_trajectoire_croise_le_rythme() {
        let t = trajectoire(&vue_exemple());
        // toujours en avance dans l'exemple (64 % contre 81 % de semaine passée) : aucune zone de retard
        assert!(t.aire_avance.starts_with("M "));
        assert_eq!(t.aire_retard, "");
        assert_eq!(t.en_ce_moment, "En ce moment : 17,4 % d’avance");
        assert_eq!(t.texte, "64 % utilisés");
        assert!(proche(t.y, 0.36));
        // un jour 1 dépassé puis des jours sages : la courbe passe du retard à l'avance
        let m = trajectoire(&vue_milieu());
        assert!(m.aire_retard.starts_with("M "), "{}", m.aire_retard);
        assert!(m.aire_avance.starts_with("M "), "{}", m.aire_avance);
        // le cumul commence à l'origine et finit à « maintenant »
        assert!(m.cumul.starts_with("M 0.00 300.00 "), "{}", m.cumul);
        assert!(m.permis.starts_with("M "));
    }

    #[test]
    fn la_semaine_en_une_barre_fait_cent_pour_cent() {
        for v in [vue_exemple(), vue_milieu()] {
            let s = semaine(&v);
            let somme: f32 = s.segments.iter().map(|x| x.part).sum();
            assert!(proche(somme, 1.0), "somme {somme}");
            // les segments se suivent sans trou
            for w in s.segments.windows(2) {
                assert!(proche(w[0].debut + w[0].part, w[1].debut));
            }
            assert!(proche(s.segments[0].debut, 0.0));
        }
        let s = semaine(&vue_exemple());
        // 5 jours passés + aujourd'hui déjà + permis + 1 jour à venir
        assert_eq!(s.segments.len(), 8);
        assert_eq!(s.segments[5].genre, SEG_AUJOURDHUI);
        assert_eq!(s.segments[5].etiquette, "", "le nom d'aujourd'hui est sous le morceau permis");
        assert_eq!(s.segments[6].genre, SEG_PERMIS);
        assert_eq!(s.segments[6].etiquette, "mar→mer");
        assert_eq!(s.segments[6].texte, "13");
        assert_eq!(s.segments[7].genre, SEG_A_VENIR);
        assert!(proche(s.repere, 0.8143));
        assert_eq!(s.resume, "Tu as utilisé 64 %, il te reste 36 %.");
        // le jour 1 dépassé de la maquette
        assert_eq!(semaine(&vue_milieu()).segments[0].genre, SEG_PASSE_DEPASSE);
    }

    #[test]
    fn chaque_vue_parle_chaque_langue() {
        use crate::langue::{TOUTES, avec};
        // pas de trou ({0} resté dans un texte), pas de texte vide là où la vue en montre un
        let sain = |texte: &str, ou: &str| {
            assert!(!texte.contains('{') && !texte.contains('}'), "{ou} : paramètre non rempli dans {texte:?}");
        };
        // (semaine de fin, milieu avec un jour dépassé, avant la 1re lecture)
        let avant_lecture = {
            let maintenant = DEBUT + 5 * JOUR_S + 3_600;
            construire(&SemaineDonnees { reset: RESET, points: vec![(maintenant, 64.0)] }, maintenant, None).unwrap()
        };
        for l in TOUTES {
            avec(l, || {
                let ou = l.code();
                for v in [vue_exemple(), vue_milieu(), avant_lecture.clone()] {
                    sain(&puce(&v), ou);
                    assert!(puce(&v).contains(&fr::borne_jour(v.debut)), "{ou} : la puce dit l'heure de la remise à zéro");
                    for j in jours_carte(&v) {
                        for texte in [&j.nom, &j.valeur, &j.court, &j.sous, &j.ligne] {
                            assert!(!texte.is_empty(), "{ou} : un texte de jour est vide");
                            sain(texte, ou);
                        }
                    }
                    let a = aujourdhui(&v);
                    for texte in [&a.etiquette, &a.valeur, &a.sous, &a.resume] {
                        assert!(!texte.is_empty(), "{ou} : un texte d'« aujourd'hui » est vide");
                        sain(texte, ou);
                    }
                    let tr = trajectoire(&v);
                    for texte in [&tr.texte, &tr.en_ce_moment] {
                        assert!(!texte.is_empty());
                        sain(texte, ou);
                    }
                    let s = semaine(&v);
                    assert!(!s.repere_texte.is_empty() && !s.resume.is_empty());
                    sain(&s.repere_texte, ou);
                    sain(&s.resume, ou);
                    for a in aides(&v, true) {
                        assert!(a.len() > 20, "{ou} : une légende est vide");
                        sain(&a, ou);
                    }
                }
            });
        }
        // un exemple par langue, pour qu'une colonne échangée se voie
        avec(crate::langue::Langue::De, || {
            assert_eq!(aujourdhui(&vue_exemple()).sous, "von 23 % erlaubt");
            assert_eq!(semaine(&vue_exemple()).resume, "Du hast 64 % verbraucht, es bleiben 36 %.");
        });
        avec(crate::langue::Langue::Ja, || {
            assert_eq!(aujourdhui(&vue_exemple()).sous, "許容23%");
            assert_eq!(jours_carte(&vue_exemple())[0].nom, "木→金");
        });
        avec(crate::langue::Langue::En, || {
            assert_eq!(jours_carte(&vue_exemple())[5].valeur, "13 left");
            assert_eq!(trajectoire(&vue_exemple()).en_ce_moment, "Right now: 17.4% to spare");
        });
    }

    #[test]
    fn cinq_aides_une_par_onglet() {
        assert_eq!(aides(&vue_exemple(), true).len(), NB_VUES);
        assert!(aides(&vue_exemple(), true)[0].ends_with("Tokens = petits morceaux de texte échangés avec Claude."));
    }
}
