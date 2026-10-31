//! Le modèle de la semaine : écart au rythme, budget de chaque jour, report.
//!
//! Règles (notes de spécification §2, validées) :
//! - une semaine = de `resets_at − 7 j` à `resets_at` (168 h), `resets_at` relu à chaque lecture ;
//! - un jour = un bloc de 24 h compté depuis le début de la semaine (sept jours égaux) ;
//! - budget du jour = ce qui reste au début du jour ÷ nombre de jours restants (jour compris) :
//!   le report est automatique, le budget du lendemain contient déjà ce qui a été économisé
//!   ou dépassé ;
//! - écart du jour = budget − conso du jour.

use serde::{Deserialize, Serialize};

use crate::temps::{JOUR_S, SEMAINE_S, arrondi_minute, meme_reset};

/// Une fenêtre de limite : pourcentage utilisé et heure de reset (secondes Unix).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fenetre {
    pub p: f64,
    pub r: i64,
}

/// Une lecture des chiffres de Claude Code, à l'instant `t` (secondes Unix).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Echantillon {
    pub t: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinq: Option<Fenetre>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sept: Option<Fenetre>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transition {
    /// Même semaine, même heure de reset.
    Meme,
    /// Même semaine, mais Anthropic a décalé l'heure de reset.
    ResetDeplace,
    /// Une nouvelle semaine commence.
    NouvelleSemaine,
    /// Lecture d'une semaine déjà finie (session restée ouverte) : à ignorer.
    Perimee,
}

/// Compare deux lectures successives de la fenêtre de 7 jours.
///
/// Une nouvelle semaine ne se voit QUE par `resets_at` qui avance de plus de 3 jours. Le
/// pourcentage ne sert pas : chaque session de Claude Code envoie ses propres chiffres, ceux de
/// SA dernière réponse, donc une session au repos renvoie un pourcentage plus ancien, donc plus
/// bas, sans qu'aucune semaine n'ait changé (relecture Opus du 14/10, H1). Une « remise à plein »
/// sans nouveau `resets_at` n'est pas documentée : elle n'est pas détectée (question ouverte).
/// Un `resets_at` qui recule de plus de 3 jours vient d'une session restée sur l'ancienne semaine.
pub fn transition(avant: &Fenetre, apres: &Fenetre) -> Transition {
    let ecart = apres.r - avant.r;
    if ecart > 3 * JOUR_S {
        Transition::NouvelleSemaine
    } else if ecart < -3 * JOUR_S {
        Transition::Perimee
    } else if meme_reset(avant.r, apres.r) {
        Transition::Meme
    } else {
        Transition::ResetDeplace
    }
}

/// Les lectures d'une même semaine : `(instant, % utilisé)` rangés par instant.
#[derive(Clone, Debug, PartialEq)]
pub struct Semaine {
    /// Dernière heure de reset lue (la semaine finit là).
    pub reset: i64,
    pub points: Vec<(i64, f64)>,
}

/// Découpe l'historique en semaines (la plus récente en dernier).
pub fn semaines(echantillons: &[Echantillon]) -> Vec<Semaine> {
    let mut tries: Vec<&Echantillon> = echantillons.iter().filter(|e| e.sept.is_some()).collect();
    tries.sort_by_key(|e| e.t);
    let mut sortie: Vec<Semaine> = Vec::new();
    let mut derniere: Option<Fenetre> = None;
    for e in tries {
        let Some(f) = e.sept else { continue };
        let genre = derniere.map(|avant| transition(&avant, &f));
        match genre {
            None | Some(Transition::NouvelleSemaine) => {
                sortie.push(Semaine { reset: arrondi_minute(f.r), points: vec![(e.t, f.p)] });
                derniere = Some(f);
            }
            Some(Transition::Perimee) => {}
            Some(_) => {
                if let Some(s) = sortie.last_mut() {
                    s.reset = arrondi_minute(f.r);
                    // le pourcentage d'une semaine ne descend jamais : une valeur plus basse est
                    // celle d'une session en retard, pas une baisse
                    let plus_haut = s.points.last().map_or(f.p, |(_, p)| p.max(f.p));
                    s.points.push((e.t, plus_haut));
                }
                derniere = Some(f);
            }
        }
    }
    sortie
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EtatJour {
    Passe,
    EnCours,
    AVenir,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Jour {
    /// Début du jour (secondes Unix).
    pub debut: i64,
    pub etat: EtatJour,
    /// % du forfait consommé ce jour-là (`None` si l'appli ne le connaît pas : avant sa première lecture).
    pub conso: Option<f64>,
    pub budget: Option<f64>,
    /// budget − conso : positif = en avance.
    pub ecart: Option<f64>,
    pub tokens: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vue {
    pub utilise: f64,
    /// Part de la semaine déjà passée, en %.
    pub ecoule: f64,
    /// utilisé − écoulé : négatif = en avance.
    pub ecart_rythme: f64,
    /// 100 − utilisé.
    pub reste: f64,
    pub debut: i64,
    pub reset: i64,
    pub reste_s: i64,
    /// Rang (0 à 6) du jour en cours.
    pub aujourdhui: usize,
    pub jours: Vec<Jour>,
    /// Encore permis pour finir le jour en cours.
    pub reste_jour: f64,
    /// Budget de chaque jour suivant (`None` le dernier jour).
    pub reste_suivants: Option<f64>,
    pub nb_suivants: usize,
    /// Vrai si le jour de la première lecture est calculé en supposant que rien n'avait été
    /// consommé avant cette lecture.
    pub debut_estime: bool,
    /// Instant de la dernière lecture.
    pub vu: i64,
}

fn pct_a(points: &[(i64, f64)], t: i64) -> Option<f64> {
    points.iter().rev().find(|(pt, _)| *pt <= t).map(|(_, p)| *p)
}

/// Construit la vue de la semaine à l'instant `maintenant`. `tokens` : tokens par jour (sept
/// blocs de 24 h depuis le début de la semaine), s'ils sont connus.
///
/// Rend `None` s'il n'y a aucune lecture ou si la semaine est finie (`maintenant ≥ reset`).
pub fn construire(s: &Semaine, maintenant: i64, tokens: Option<&[u64; 7]>) -> Option<Vue> {
    let (premier_t, premier_p) = *s.points.first()?;
    let (vu, p_now) = *s.points.last()?;
    if maintenant >= s.reset {
        return None;
    }
    let debut = s.reset - SEMAINE_S;
    let maintenant = maintenant.max(debut);
    let jour_de = |t: i64| ((t - debut).div_euclid(JOUR_S)).clamp(0, 6) as usize;
    let aujourdhui = jour_de(maintenant);
    // (si l'horloge recule, la première lecture ne peut pas être après aujourd'hui)
    let premier_jour = jour_de(premier_t).min(aujourdhui);
    let debut_jour = |i: usize| debut + i as i64 * JOUR_S;

    // % utilisé au début de chaque jour connu (jusqu'à aujourd'hui). Le début de la semaine est
    // toujours à 0 %. Pour un autre jour d'avant la première lecture on ne sait rien : le jour de
    // la première lecture suppose que rien n'avait été consommé avant elle.
    let mut ps = [0.0_f64; 7];
    for i in premier_jour..=aujourdhui {
        let avant_toute_lecture = if i == 0 { 0.0 } else { premier_p };
        ps[i] = pct_a(&s.points, debut_jour(i)).unwrap_or(avant_toute_lecture);
    }
    let debut_estime = premier_jour > 0 && premier_t > debut_jour(premier_jour);

    let mut jours: Vec<Jour> = Vec::with_capacity(7);
    let mut reste_jour = 0.0;
    for i in 0..7 {
        let tk = tokens.map(|t| t[i]);
        if i < premier_jour {
            jours.push(Jour { debut: debut_jour(i), etat: EtatJour::Passe, conso: None, budget: None, ecart: None, tokens: tk });
        } else if i <= aujourdhui {
            let fin = if i == aujourdhui { p_now } else { ps[i + 1] };
            let conso = (fin - ps[i]).max(0.0);
            let budget = (100.0 - ps[i]).max(0.0) / (7 - i) as f64;
            let etat = if i == aujourdhui { EtatJour::EnCours } else { EtatJour::Passe };
            if i == aujourdhui {
                reste_jour = (budget - conso).max(0.0);
            }
            jours.push(Jour { debut: debut_jour(i), etat, conso: Some(conso), budget: Some(budget), ecart: Some(budget - conso), tokens: tk });
        }
    }
    let reste = (100.0 - p_now).max(0.0);
    let nb_suivants = 6 - aujourdhui;
    // Les jours suivants se partagent à parts égales ce qui reste une fois le budget du jour
    // en cours consommé (c'est le « dernier jour » de la maquette quand il n'en reste qu'un).
    let part = if nb_suivants > 0 { Some(((reste - reste_jour).max(0.0)) / nb_suivants as f64) } else { None };
    for i in (aujourdhui + 1)..7 {
        jours.push(Jour { debut: debut_jour(i), etat: EtatJour::AVenir, conso: None, budget: part, ecart: None, tokens: tokens.map(|t| t[i]) });
    }

    let ecoule = ((maintenant - debut) as f64 / SEMAINE_S as f64 * 100.0).clamp(0.0, 100.0);
    Some(Vue {
        utilise: p_now,
        ecoule,
        ecart_rythme: p_now - ecoule,
        reste,
        debut,
        reset: s.reset,
        reste_s: s.reset - maintenant,
        aujourdhui,
        jours,
        reste_jour,
        reste_suivants: part,
        nb_suivants,
        debut_estime,
        vu,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Semaine de l'exemple du 14/10/2026 : jeu 08/10 13:00 → jeu 15/10 13:00 (Paris, UTC+2).
    const RESET: i64 = 1_792_062_000; // 2026-10-15T11:00:00Z
    const DEBUT: i64 = RESET - SEMAINE_S;
    // 14/10 à 05:48 Paris = 03:48Z : 136,8 h écoulées sur 168 h.
    const MAINTENANT: i64 = DEBUT + 136 * 3_600 + 48 * 60;

    fn fen(p: f64) -> Option<Fenetre> {
        Some(Fenetre { p, r: RESET })
    }

    fn ech(t: i64, p: f64) -> Echantillon {
        Echantillon { t, cinq: None, sept: fen(p) }
    }

    /// Conso par jour de la maquette : 9, 10, 8, 12, 15, puis 10 en cours (jour 6, 64 % en tout).
    fn semaine_maquette() -> Semaine {
        let ps = [0.0, 9.0, 19.0, 27.0, 39.0, 54.0];
        let mut points: Vec<(i64, f64)> = ps.iter().enumerate().map(|(i, p)| (DEBUT + i as i64 * JOUR_S, *p)).collect();
        points.push((MAINTENANT, 64.0));
        Semaine { reset: RESET, points }
    }

    fn proche(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn exemple_chiffre_de_la_maquette() {
        let v = construire(&semaine_maquette(), MAINTENANT, None).expect("vue");
        assert_eq!(v.aujourdhui, 5);
        assert!(proche(v.utilise, 64.0));
        assert!(proche(v.ecoule, 81.43), "écoulé {}", v.ecoule);
        assert!(proche(v.ecart_rythme, -17.43), "écart {}", v.ecart_rythme);
        assert!(proche(v.reste, 36.0));
        let budgets: Vec<f64> = v.jours.iter().filter_map(|j| j.budget).collect();
        let attendus = [100.0 / 7.0, 91.0 / 6.0, 81.0 / 5.0, 73.0 / 4.0, 61.0 / 3.0, 23.0, 23.0];
        assert_eq!(budgets.len(), 7);
        for (b, a) in budgets.iter().zip(attendus) {
            assert!(proche(*b, a), "budget {b} attendu {a}");
        }
        // « 13 % encore permis » aujourd'hui, « 23 % » pour le dernier jour (notes §2)
        assert!(proche(v.reste_jour, 13.0));
        assert_eq!(v.reste_suivants.map(|x| x.round()), Some(23.0));
        assert_eq!(v.nb_suivants, 1);
        assert!(!v.debut_estime);
        let conso: Vec<f64> = v.jours.iter().filter_map(|j| j.conso).collect();
        assert_eq!(conso, vec![9.0, 10.0, 8.0, 12.0, 15.0, 10.0]);
        assert_eq!(v.jours[5].etat, EtatJour::EnCours);
        assert_eq!(v.jours[6].etat, EtatJour::AVenir);
    }

    #[test]
    fn le_report_est_automatique() {
        // un jour sous le budget : le lendemain gagne la différence
        let v = construire(&semaine_maquette(), MAINTENANT, None).unwrap();
        let j0 = &v.jours[0];
        let j1 = &v.jours[1];
        assert!(j0.ecart.unwrap() > 0.0); // 14,3 − 9 : en avance
        assert!(j1.budget.unwrap() > j0.budget.unwrap());
        // un jour dépassé : le budget du lendemain baisse (jour 5 : 15 % pour 20,3 → +5,3 ; ici l'inverse)
        let mut s = semaine_maquette();
        s.points[1].1 = 30.0; // jour 1 : 30 % consommés, très au-dessus des 14,3 permis
        let v = construire(&s, MAINTENANT, None).unwrap();
        assert!(v.jours[0].ecart.unwrap() < 0.0);
        assert!(v.jours[1].budget.unwrap() < 100.0 / 7.0);
    }

    #[test]
    fn dernier_jour_tout_le_reste_est_permis() {
        let maintenant = DEBUT + 6 * JOUR_S + 3_600; // jour 7, 1 h après son début
        let mut s = semaine_maquette();
        s.points.push((DEBUT + 6 * JOUR_S, 70.0));
        s.points.push((maintenant, 72.0));
        let v = construire(&s, maintenant, None).unwrap();
        assert_eq!(v.aujourdhui, 6);
        assert_eq!(v.nb_suivants, 0);
        assert_eq!(v.reste_suivants, None);
        assert!(proche(v.reste_jour, 28.0)); // 30 % de budget − 2 % consommés
        assert!(proche(v.jours[6].budget.unwrap(), 30.0));
    }

    #[test]
    fn jour_en_cours_depasse_rien_de_permis() {
        let mut s = semaine_maquette();
        s.points.last_mut().unwrap().1 = 54.0 + 30.0; // 30 % aujourd'hui pour 23 permis
        let v = construire(&s, MAINTENANT, None).unwrap();
        assert!(proche(v.reste_jour, 0.0));
        assert!(v.jours[5].ecart.unwrap() < 0.0);
        // tout ce qui reste va aux jours suivants
        assert!(proche(v.reste_suivants.unwrap(), 16.0));
    }

    #[test]
    fn premiere_lecture_en_cours_de_semaine() {
        // l'appli n'a rien vu avant le jour 6 : seuls les tokens existeraient pour les jours d'avant
        let s = Semaine { reset: RESET, points: vec![(MAINTENANT, 64.0)] };
        let tokens = [1, 2, 3, 4, 5, 6, 0u64];
        let v = construire(&s, MAINTENANT, Some(&tokens)).unwrap();
        assert!(v.debut_estime);
        for i in 0..5 {
            assert_eq!(v.jours[i].conso, None);
            assert_eq!(v.jours[i].budget, None);
            assert_eq!(v.jours[i].tokens, Some(i as u64 + 1));
        }
        // aujourd'hui : on suppose 0 consommé avant la première lecture, budget = 36 ÷ 2
        assert!(proche(v.jours[5].budget.unwrap(), 18.0));
        assert!(proche(v.jours[5].conso.unwrap(), 0.0));
        assert!(proche(v.reste_jour, 18.0));
    }

    #[test]
    fn semaine_finie_ou_vide() {
        let s = semaine_maquette();
        assert!(construire(&s, RESET, None).is_none());
        assert!(construire(&s, RESET + 5, None).is_none());
        assert!(construire(&Semaine { reset: RESET, points: vec![] }, MAINTENANT, None).is_none());
    }

    #[test]
    fn pourcentage_au_dessus_de_100_ne_casse_rien() {
        let mut s = semaine_maquette();
        s.points.last_mut().unwrap().1 = 103.0;
        let v = construire(&s, MAINTENANT, None).unwrap();
        assert!(proche(v.reste, 0.0));
        assert!(proche(v.reste_jour, 0.0));
        assert!(v.reste_suivants.unwrap() >= 0.0);
    }

    #[test]
    fn transitions_de_semaine() {
        let a = Fenetre { p: 60.0, r: RESET };
        // gigue d'une seconde : même semaine
        assert_eq!(transition(&a, &Fenetre { p: 61.0, r: RESET - 1 }), Transition::Meme);
        // reset décalé d'une heure, % qui continue de monter : même semaine, heure déplacée
        assert_eq!(transition(&a, &Fenetre { p: 62.0, r: RESET + 3_600 }), Transition::ResetDeplace);
        // reset avancé de 7 jours : nouvelle semaine
        assert_eq!(transition(&a, &Fenetre { p: 2.0, r: RESET + SEMAINE_S }), Transition::NouvelleSemaine);
        // % plus bas sans que le reset bouge : une session au repos qui renvoie un vieux chiffre,
        // PAS une nouvelle semaine (relecture Opus, H1)
        assert_eq!(transition(&a, &Fenetre { p: 40.0, r: RESET }), Transition::Meme);
        assert_eq!(transition(&a, &Fenetre { p: 0.0, r: RESET }), Transition::Meme);
        // reset de la semaine d'avant (session restée sur l'ancienne semaine) : périmé
        assert_eq!(transition(&a, &Fenetre { p: 90.0, r: RESET - SEMAINE_S }), Transition::Perimee);
    }

    #[test]
    fn une_session_au_repos_ne_casse_pas_la_semaine() {
        // 60 % puis 63 % (autre session), puis le vieux 60 % d'une session au repos, puis 63,2 %
        let suite = [(10_000, 60.0), (20_000, 63.0), (21_000, 60.0), (22_000, 63.2)];
        let e: Vec<Echantillon> = suite.iter().map(|(t, p)| ech(DEBUT + t, *p)).collect();
        let s = semaines(&e);
        assert_eq!(s.len(), 1, "une seule semaine : {s:?}");
        assert_eq!(s[0].points.len(), 4);
        assert!(s[0].points.windows(2).all(|w| w[0].1 <= w[1].1), "la série ne descend jamais : {:?}", s[0].points);
        assert_eq!(s[0].points.last().unwrap().1, 63.2);
        // une lecture de la semaine d'avant, arrivée en retard : ignorée
        let mut e = e;
        e.push(Echantillon { t: DEBUT + 30_000, cinq: None, sept: Some(Fenetre { p: 88.0, r: RESET - SEMAINE_S }) });
        let s = semaines(&e);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].points.len(), 4);
        assert_eq!(s[0].reset, RESET);
    }

    #[test]
    fn un_reset_lu_a_10_59_59_est_le_reset_de_11_00() {
        // les vrais chiffres du 14/10 : Claude Code donne `…10:59:59`, l'heure du reset est 11:00
        let e = vec![Echantillon { t: DEBUT + 5_000, cinq: None, sept: Some(Fenetre { p: 71.0, r: RESET - 1 }) }];
        let s = semaines(&e);
        assert_eq!(s[0].reset, RESET);
        let v = construire(&s[0], DEBUT + 5_000, None).unwrap();
        assert_eq!(v.reset, RESET);
        assert_eq!(v.debut, DEBUT);
    }

    #[test]
    fn le_jour_un_part_de_zero_et_n_est_pas_estime() {
        // première lecture 2 h après le reset (3 %), puis 12 % au début du jour 2 : le jour 1 a
        // consommé 12 % (la relecture Opus a vu 9 avec l'ancien code, M1)
        let points = vec![(DEBUT + 7_200, 3.0), (DEBUT + JOUR_S, 12.0), (DEBUT + JOUR_S + 600, 12.5)];
        let s = Semaine { reset: RESET, points };
        let v = construire(&s, DEBUT + JOUR_S + 600, None).unwrap();
        assert_eq!(v.aujourdhui, 1);
        assert!(proche(v.jours[0].conso.unwrap(), 12.0), "conso jour 1 = {:?}", v.jours[0].conso);
        assert!(proche(v.jours[0].budget.unwrap(), 100.0 / 7.0));
        assert!(!v.debut_estime);
    }

    #[test]
    fn l_horloge_qui_recule_ne_double_pas_les_jours() {
        // première lecture au jour 6, puis l'horloge recule au jour 3 : toujours 7 jours
        let s = Semaine { reset: RESET, points: vec![(DEBUT + 5 * JOUR_S + 100, 50.0)] };
        let v = construire(&s, DEBUT + 2 * JOUR_S + 100, None).unwrap();
        assert_eq!(v.jours.len(), 7);
        assert_eq!(v.aujourdhui, 2);
        assert_eq!(v.jours.iter().filter(|j| j.etat == EtatJour::EnCours).count(), 1);
    }

    #[test]
    fn decoupe_en_semaines() {
        let suivante = RESET + SEMAINE_S;
        let e = vec![
            ech(DEBUT + 100, 5.0),
            ech(DEBUT + 5_000, 8.0),
            Echantillon { t: RESET + 60, cinq: None, sept: Some(Fenetre { p: 1.0, r: suivante }) },
            Echantillon { t: RESET + 120, cinq: fen(3.0), sept: None }, // pas de fenêtre de 7 jours : ignoré
            Echantillon { t: RESET + 9_000, cinq: None, sept: Some(Fenetre { p: 4.0, r: suivante }) },
        ];
        let s = semaines(&e);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].reset, RESET);
        assert_eq!(s[0].points.len(), 2);
        assert_eq!(s[1].reset, suivante);
        assert_eq!(s[1].points, vec![(RESET + 60, 1.0), (RESET + 9_000, 4.0)]);
    }

    #[test]
    fn decoupe_tolere_le_desordre_des_lignes() {
        let e = vec![ech(DEBUT + 5_000, 8.0), ech(DEBUT + 100, 5.0)];
        let s = semaines(&e);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].points, vec![(DEBUT + 100, 5.0), (DEBUT + 5_000, 8.0)]);
    }
}
