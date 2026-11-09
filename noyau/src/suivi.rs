//! La page « Conversations » mise en forme : états, groupes, fenêtres, alerte, textes. Sans interface,
//! donc testable sans Slint. Part des conversations lues (`conversations`) et des réglages (`reglages`).
//!
//! Règles :
//! - ACTIVE = dernier message de Claude il y a 30 min au plus ; RÉCENTE = entre 30 min et 24 h
//!   (repliée) ; plus vieille : absente. « Ouverte » est invisible dans les fichiers, on l'approche
//!   par la récence.
//! - Le pourcentage est arrondi AVANT la comparaison aux seuils : ce que la ligne affiche (« ≈ 60 % »)
//!   est ce qui décide (« passation »), jamais un 59,6 caché derrière un 60.
//! - « Résumée » (une ligne `compact_boundary` a été vue) vaut passation tout de suite, même sous le seuil.
//! - L'alerte ne regarde que les conversations ACTIVES : rose s'il y a une passation (ou une résumée),
//!   ambre s'il n'y a que de l'attention, éteinte sinon ou si la case est décochée.
//! - MASQUÉE (le ✕ d'une ligne, 1.3.4) : écartée avant les groupes, les récentes et l'alerte ; reconnue par son
//!   identifiant de session, elle reste masquée même si un nouveau message arrive.

use std::collections::BTreeMap;

use crate::conversations::{Conversation, RECENTE_S, nom_modele, pour_affichage};
use crate::langue::{self, Langue};
use crate::reglages::{ChoixFenetre, Reglages, Seuils};
use crate::textes::{Cle, fmt, t};
use crate::fr;

pub const ACTIVE_S: i64 = 30 * 60;
/// Combien de modèles la fenêtre de réglage montre (les plus récemment utilisés).
pub const MODELES_MAX: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etat {
    Ok,
    Attention,
    Passation,
    Resumee,
}

impl Etat {
    /// Le numéro donné à l'interface : 0 rien, 1 attention, 2 passation, 3 résumée.
    pub fn numero(self) -> i32 {
        self as i32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alerte {
    Aucune,
    Attention,
    Passation,
}

#[derive(Clone, Debug)]
pub struct LigneSuivi {
    /// L'identifiant de session : ce que le ✕ de la ligne désigne pour la masquer (il n'est jamais montré).
    pub session: String,
    pub dossier: String,
    /// `Notes de voyage · projet-a` : le titre donné à la conversation puis son dossier de travail ; sans titre,
    /// `projet-a · 3f9c` : le dossier et 4 caractères de l'identifiant (voir `nom_affiche`).
    pub nom: String,
    /// `LEAD`, `FIL`… : vide quand les groupes ne sont pas séparés.
    pub etiquette: String,
    pub groupe: usize,
    /// `Opus 5.5 · il y a 11 min`.
    pub meta: String,
    pub pct: f64,
    pub seuils: Seuils,
    pub etat: Etat,
    /// `≈ 41 % sur 60`.
    pub texte_pct: String,
    /// `405 000 tokens`.
    pub tokens: String,
    /// `reste 19`, `passation` ou `résumée`.
    pub reste: String,
    pub age_s: i64,
}

#[derive(Clone, Debug)]
pub struct GroupeSuivi {
    pub nom: String,
    pub seuils: Seuils,
    /// `attention 50 · passation 60`.
    pub texte_seuils: String,
    /// Les conversations ACTIVES de ce groupe, la plus récente d'abord.
    pub lignes: Vec<LigneSuivi>,
}

/// Un modèle vu, pour la fenêtre de réglage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModeleVu {
    pub cle: String,
    pub nom: String,
    pub vu: u64,
    pub choix: ChoixFenetre,
    /// La fenêtre que « Auto » donnerait.
    pub auto: u64,
}

#[derive(Clone, Debug)]
pub struct Suivi {
    pub groupes: Vec<GroupeSuivi>,
    /// Les conversations de 30 min à 24 h (repliées dans l'écran).
    pub recentes: Vec<LigneSuivi>,
    pub alerte: Alerte,
    pub modeles: Vec<ModeleVu>,
    /// Les plus grands contextes vus par modèle, à ranger si cela a changé.
    pub vus: BTreeMap<String, u64>,
    /// Combien de conversations de moins de 24 h sont masquées en ce moment (Config : « n conversations masquées »).
    pub masquees: usize,
}

// ---------------------------------------------------------------- les petits calculs

/// Les milliers, groupés comme la langue le veut : `405 000`, `405,000`, `405.000`.
pub fn entier(n: u64) -> String {
    let separateur = match langue::courante() {
        Langue::En | Langue::Zh | Langue::Ja => ",",
        Langue::Fr => "\u{a0}",
        _ => ".",
    };
    let chiffres = n.to_string();
    let mut sortie = String::new();
    for (i, c) in chiffres.chars().enumerate() {
        if i > 0 && (chiffres.len() - i) % 3 == 0 {
            sortie.push_str(separateur);
        }
        sortie.push(c);
    }
    sortie
}

/// Le pourcentage du contexte dans la fenêtre (0 si la fenêtre est nulle).
pub fn pourcentage(contexte: u64, fenetre: u64) -> f64 {
    if fenetre == 0 { 0.0 } else { contexte as f64 / fenetre as f64 * 100.0 }
}

/// L'état d'un pourcentage : arrondi d'abord, comparé aux seuils ensuite.
pub fn etat_de(pct: f64, s: Seuils, resumee: bool) -> Etat {
    let n = pct.round();
    if resumee {
        Etat::Resumee
    } else if n >= s.passation as f64 {
        Etat::Passation
    } else if n >= s.attention as f64 {
        Etat::Attention
    } else {
        Etat::Ok
    }
}

/// Points de pourcentage avant la passation (0 si elle est atteinte).
pub fn reste_avant_passation(pct: f64, s: Seuils) -> u32 {
    (s.passation as f64 - pct.round()).max(0.0) as u32
}

/// L'étiquette courte d'un groupe, tirée de son nom : `Leads` → `LEAD`, `Fils` → `FIL`,
/// `Groupe 2` → `G2`. Au plus 4 caractères, en majuscules.
pub fn etiquette_groupe(nom: &str) -> String {
    let nom = nom.trim();
    let mots: Vec<&str> = nom.split_whitespace().collect();
    if let (Some(premier), Some(dernier)) = (mots.first(), mots.last()) {
        if mots.len() > 1 && dernier.chars().all(|c| c.is_ascii_digit()) {
            let initiale: String = premier.chars().take(1).collect();
            return format!("{initiale}{dernier}").to_uppercase().chars().take(4).collect();
        }
    }
    let mut lettres: Vec<char> = nom.chars().filter(|c| c.is_alphanumeric()).collect();
    if lettres.len() > 3 && matches!(lettres.last(), Some('s') | Some('S')) {
        lettres.pop();
    }
    lettres.into_iter().take(4).collect::<String>().to_uppercase()
}

/// Le nom montré d'un groupe : celui tapé à la main, sinon « Groupe 1 » (traduit) ; « Toutes les conversations » sans séparation.
pub fn nom_du_groupe(r: &Reglages, i: usize) -> String {
    if !r.separer {
        t(Cle::GroupeTous).to_string()
    } else if r.noms[i].is_empty() {
        fmt(Cle::GroupeN, &[&(i + 1)])
    } else {
        r.noms[i].clone()
    }
}

/// Le groupe d'un dossier : 0 sans séparation ; avec, 0 si le dossier est dans la liste du groupe 1, sinon 1.
pub fn groupe_du_dossier(r: &Reglages, dossier: &str) -> usize {
    if !r.separer || r.groupe1.iter().any(|d| d == dossier) { 0 } else { 1 }
}

fn quatre_caracteres(session: &str) -> String {
    session.chars().take(4).collect()
}

/// Ce qu'on lit dans la liste : le titre que la personne a donné à la conversation, puis son dossier
/// (`Notes de voyage · projet-a`) ; sans titre, le dossier et 4 caractères de l'identifiant (`projet-a · c95f`). Trois
/// conversations du même dossier ne se ressemblent plus.
///
/// Le titre ET le nom du dossier passent par `pour_affichage` : un U+202E dans l'un retournerait aussi l'autre, un saut de
/// ligne ferait déborder la ligne. La CLÉ du dossier (`LigneSuivi::dossier`, le clic sur l'étiquette) reste son nom exact.
fn nom_affiche(c: &Conversation) -> String {
    let dossier = pour_affichage(&c.dossier);
    match titre_affiche(c) {
        Some(titre) => format!("{titre} · {dossier}"),
        None => format!("{dossier} · {}", quatre_caracteres(&c.session)),
    }
}

/// Le titre prêt à montrer : nettoyé pour l'affichage, rogné ; `None` s'il est absent ou s'il ne reste rien.
fn titre_affiche(c: &Conversation) -> Option<String> {
    c.titre.as_deref().map(|t| pour_affichage(t).trim().to_string()).filter(|t| !t.is_empty())
}

fn texte_age(age_s: i64) -> String {
    if age_s < 60 { t(Cle::AgeInstant).to_string() } else { fr::age(age_s) }
}

/// L'exemple de la barre d'essai de la fenêtre de réglage : l'état et la phrase pour un contexte
/// à `pct` % de la fenêtre.
pub fn texte_essai(s: Seuils, pct: u8) -> (Etat, String) {
    let etat = etat_de(pct as f64, s, false);
    let p = fr::pourcent(pct as f64);
    let texte = match etat {
        Etat::Passation | Etat::Resumee => fmt(Cle::ConfigEssaiPassation, &[&p]),
        Etat::Attention => fmt(Cle::ConfigEssaiAttention, &[&p, &reste_avant_passation(pct as f64, s)]),
        Etat::Ok => fmt(Cle::ConfigEssaiOk, &[&p, &reste_avant_passation(pct as f64, s)]),
    };
    (etat, texte)
}

/// L'exemple proposé au départ : à mi-chemin entre l'attention et la passation.
pub fn essai_par_defaut(s: Seuils) -> u8 {
    ((s.attention as u16 + s.passation as u16 + 1) / 2) as u8
}

/// L'ordre FIXE des modèles dans la fenêtre de réglage : par famille (Opus, Sonnet, Haiku, puis les autres),
/// dans une famille la version la plus récente d'abord, puis l'identifiant. Il ne dépend ni des conversations
/// ni de leur âge : la liste ne bouge pas sous le clic.
pub fn ordre_modeles(a: &str, b: &str) -> std::cmp::Ordering {
    fn rang(cle: &str) -> (usize, Vec<u64>) {
        let morceaux: Vec<&str> = cle.strip_prefix("claude-").unwrap_or(cle).split('-').collect();
        let famille = ["opus", "sonnet", "haiku"].iter().position(|f| morceaux.iter().any(|m| m.eq_ignore_ascii_case(f))).unwrap_or(3);
        (famille, morceaux.iter().filter_map(|m| m.parse::<u64>().ok()).collect())
    }
    let (famille_a, version_a) = rang(a);
    let (famille_b, version_b) = rang(b);
    famille_a.cmp(&famille_b).then_with(|| version_b.cmp(&version_a)).then_with(|| a.cmp(b))
}

// ---------------------------------------------------------------- le calcul d'ensemble

/// Le plus grand contexte vu par modèle : ce qui était rangé, plus ce que les conversations montrent.
pub fn contextes_vus(convs: &[Conversation], connus: &BTreeMap<String, u64>) -> BTreeMap<String, u64> {
    let mut vus = connus.clone();
    for c in convs {
        let v = vus.entry(c.modele.clone()).or_insert(0);
        *v = (*v).max(c.contexte);
    }
    vus
}

pub fn calculer(convs: &[Conversation], r: &Reglages, connus: &BTreeMap<String, u64>, maintenant: i64) -> Suivi {
    let vus = contextes_vus(convs, connus);
    let mut toutes: Vec<LigneSuivi> = Vec::new();
    let mut actives_ids: Vec<bool> = Vec::new();
    // pour chaque ligne qui porte un titre : les 4 caractères de son identifiant (voir plus bas)
    let mut ids_titre: Vec<Option<String>> = Vec::new();
    let mut nb_masquees = 0usize;
    for c in convs {
        let age = (maintenant - c.dernier).max(0);
        if age > RECENTE_S {
            continue;
        }
        // une conversation masquée à la main n'existe plus ni dans les groupes, ni dans « + n autres », ni dans l'alerte,
        // quel que soit son dernier message (seuls les contextes vus par modèle, calculés plus haut, la comptent encore)
        if r.est_masquee(&c.session) {
            nb_masquees += 1;
            continue;
        }
        let g = groupe_du_dossier(r, &c.dossier);
        let seuils = r.seuils[g];
        let vu = vus.get(&c.modele).copied().unwrap_or(0);
        let fenetre = r.choix_fenetre(&c.modele).tokens(vu);
        let pct = pourcentage(c.contexte, fenetre);
        let etat = etat_de(pct, seuils, c.resumee);
        let reste = match etat {
            Etat::Resumee => t(Cle::ConvResumee).to_string(),
            Etat::Passation => t(Cle::ConvPassation).to_string(),
            _ => fmt(Cle::ConvReste, &[&reste_avant_passation(pct, seuils)]),
        };
        let nom_g = nom_du_groupe(r, g);
        toutes.push(LigneSuivi {
            session: c.session.clone(),
            dossier: c.dossier.clone(),
            nom: nom_affiche(c),
            etiquette: if r.separer { etiquette_groupe(&nom_g) } else { String::new() },
            groupe: g,
            meta: format!("{} · {}", pour_affichage(&nom_modele(&c.modele)), texte_age(age)),
            pct,
            seuils,
            etat,
            texte_pct: fmt(Cle::ConvSur, &[&fr::pourcent(pct.round()), &seuils.passation]),
            tokens: format!("{} {}", entier(c.contexte), t(Cle::MotTokens)),
            reste,
            age_s: age,
        });
        actives_ids.push(age <= ACTIVE_S);
        ids_titre.push(titre_affiche(c).is_some().then(|| quatre_caracteres(&c.session)));
    }
    // deux conversations du même dossier qui portent le même titre : l'identifiant court les distingue
    let mut compte: BTreeMap<String, usize> = BTreeMap::new();
    for l in &toutes {
        *compte.entry(l.nom.clone()).or_insert(0) += 1;
    }
    for (l, id) in toutes.iter_mut().zip(&ids_titre) {
        let doublon = compte.get(&l.nom).copied().unwrap_or(0) > 1;
        if let (true, Some(id)) = (doublon, id) {
            l.nom = format!("{} · {}", l.nom, id);
        }
    }

    let nb_groupes = if r.separer { 2 } else { 1 };
    let mut groupes: Vec<GroupeSuivi> = (0..nb_groupes)
        .map(|i| GroupeSuivi {
            nom: nom_du_groupe(r, i),
            seuils: r.seuils[i],
            texte_seuils: fmt(Cle::ConvSeuils, &[&r.seuils[i].attention, &r.seuils[i].passation]),
            lignes: Vec::new(),
        })
        .collect();
    let mut recentes = Vec::new();
    let mut alerte = Alerte::Aucune;
    for (ligne, active) in toutes.into_iter().zip(actives_ids) {
        if active {
            match ligne.etat {
                Etat::Passation | Etat::Resumee => alerte = Alerte::Passation,
                Etat::Attention if alerte == Alerte::Aucune => alerte = Alerte::Attention,
                _ => {}
            }
            groupes[ligne.groupe].lignes.push(ligne);
        } else {
            recentes.push(ligne);
        }
    }
    let tri = |a: &LigneSuivi, b: &LigneSuivi| a.age_s.cmp(&b.age_s).then_with(|| a.nom.cmp(&b.nom));
    for g in &mut groupes {
        g.lignes.sort_by(tri);
    }
    recentes.sort_by(tri);

    // les modèles de la fenêtre de réglage : ceux qu'on voit en ce moment, le plus récent d'abord,
    // puis ceux dont la fenêtre a été choisie à la main
    let mut ordre: Vec<String> = Vec::new();
    for c in convs {
        if !ordre.contains(&c.modele) {
            ordre.push(c.modele.clone());
        }
    }
    for (m, c) in &r.fenetres {
        if *c != ChoixFenetre::Auto && !ordre.contains(m) {
            ordre.push(m.clone());
        }
    }
    // les MODELES_MAX plus récents, rangés ensuite dans un ordre FIXE (famille, version) : la liste ne saute plus
    // quand une autre conversation répond
    let mut gardes: Vec<String> = ordre.into_iter().take(MODELES_MAX).collect();
    gardes.sort_by(|a, b| ordre_modeles(a, b));
    let modeles = gardes
        .into_iter()
        .map(|cle| {
            let vu = vus.get(&cle).copied().unwrap_or(0);
            ModeleVu { nom: nom_modele(&cle), choix: r.choix_fenetre(&cle), auto: ChoixFenetre::Auto.tokens(vu), vu, cle }
        })
        .collect();

    Suivi { groupes, recentes, alerte: if r.alerte { alerte } else { Alerte::Aucune }, modeles, vus, masquees: nb_masquees }
}

// ---------------------------------------------------------------- la fenêtre d'accueil

/// Les dossiers que la fenêtre d'accueil propose de cocher, avec leur case : ceux des conversations qu'on
/// voit (les actives d'abord, puis les récentes), puis ceux déjà rangés dans le groupe 1 qu'on ne voit plus
/// (pour qu'une réouverture ne les perde pas). Sans doublon ; la case est cochée pour le groupe 1.
pub fn dossiers_a_choisir(s: &Suivi, r: &Reglages) -> Vec<(String, bool)> {
    let mut noms: Vec<String> = Vec::new();
    for l in s.groupes.iter().flat_map(|g| g.lignes.iter()).chain(s.recentes.iter()) {
        if !l.dossier.is_empty() && !noms.contains(&l.dossier) {
            noms.push(l.dossier.clone());
        }
    }
    for d in &r.groupe1 {
        if !noms.contains(d) {
            noms.push(d.clone());
        }
    }
    noms.into_iter()
        .map(|d| {
            let coche = r.groupe1.contains(&d);
            (d, coche)
        })
        .collect()
}

/// Les dossiers à proposer quand aucune lecture n'est encore faite (au démarrage, avant la première) : seulement ceux déjà
/// rangés dans le groupe 1, cochés. Ouvrir la fenêtre trop tôt ne doit pas effacer le choix rangé en validant une liste vide.
pub fn dossiers_sans_lecture(r: &Reglages) -> Vec<(String, bool)> {
    r.groupe1.iter().map(|d| (d.clone(), true)).collect()
}

// ---------------------------------------------------------------- la démonstration

/// Des conversations d'exemple pour `--demo` (rien n'est lu dans les fichiers) : un peu de tout, des états
/// ok, attention, passation et résumée, un dossier au nom très long, des conversations repliées.
pub fn exemple(maintenant: i64) -> Vec<Conversation> {
    let c = |dossier: &str, session: &str, modele: &str, contexte: u64, age_min: i64, resumee: bool| Conversation {
        session: session.into(),
        dossier: dossier.into(),
        modele: modele.into(),
        contexte,
        dernier: maintenant - age_min * 60,
        resumee,
        titre: None,
    };
    let mut v = vec![
        c("lead-alpha", "3f9c01", "claude-opus-5-5", 405_000, 2, false),
        c("lead-beta", "91ab02", "claude-sonnet-5-5", 560_000, 6, false),
        c("projet-a", "07de03", "claude-opus-5-5", 520_000, 11, false),
        c("projet-c", "a8e004", "claude-sonnet-5-5", 430_000, 20, false),
        c("projet-avec-un-nom-de-dossier-vraiment-tres-long", "7b2105", "claude-sonnet-5-5", 188_000, 25, false),
        c("projet-c", "52c106", "claude-sonnet-5-5", 126_000, 72, false),
        c("projet-d", "19fa07", "claude-haiku-5-5", 90_000, 95, false),
        c("lead-alpha", "5c0208", "claude-opus-5-5", 280_000, 190, false),
        c("projet-a", "e41709", "claude-opus-5-5", 23_000, 410, true),
    ];
    // quelques titres pour voir les deux formes côte à côte (la démonstration ne lit rien dans les fichiers)
    v[0].titre = Some("Pilote 3".into());
    v[1].titre = Some("Pilote 4".into());
    v[2].titre = Some("Rédaction du guide".into());
    v
}

/// Les contextes vus de la démonstration : Opus et Sonnet ont déjà dépassé 200 000 (fenêtre « Auto » d'1 M).
pub fn vus_demo() -> BTreeMap<String, u64> {
    BTreeMap::from([("claude-opus-5-5".to_string(), 648_622), ("claude-sonnet-5-5".to_string(), 625_696)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reglages::{FENETRE_GRANDE, FENETRE_PETITE};

    const MAINTENANT: i64 = 1_800_000_000;

    fn conv(dossier: &str, session: &str, modele: &str, contexte: u64, age_min: i64) -> Conversation {
        Conversation { session: session.into(), dossier: dossier.into(), modele: modele.into(), contexte, dernier: MAINTENANT - age_min * 60, resumee: false, titre: None }
    }

    fn avec_titre(mut c: Conversation, titre: &str) -> Conversation {
        c.titre = Some(titre.into());
        c
    }

    fn noms(convs: &[Conversation]) -> Vec<String> {
        let s = calculer(convs, &Reglages::default(), &vus_1m(), MAINTENANT);
        s.groupes.iter().flat_map(|g| g.lignes.iter()).chain(s.recentes.iter()).map(|l| l.nom.clone()).collect()
    }

    #[test]
    fn le_nom_montre_le_titre_puis_le_dossier() {
        // trois conversations du même dossier : sans titre on ne les distingue que par 4 caractères ; avec, par leur titre
        let convs = [
            avec_titre(conv("projet-a", "c95fb605", "claude-opus-5-5", 100_000, 3), "Notes de voyage"),
            avec_titre(conv("projet-a", "c6ac4b30", "claude-opus-5-5", 100_000, 5), "Essai : rejeu du lot 6"),
            conv("projet-a", "9c27cb3c", "claude-opus-5-5", 100_000, 7),
        ];
        assert_eq!(noms(&convs), ["Notes de voyage · projet-a", "Essai : rejeu du lot 6 · projet-a", "projet-a · 9c27"]);
    }

    #[test]
    fn deux_titres_identiques_dans_le_meme_dossier_gardent_l_identifiant() {
        let convs = [
            avec_titre(conv("projet-a", "aaaa1111", "claude-opus-5-5", 100_000, 3), "Brouillon"),
            avec_titre(conv("projet-a", "bbbb2222", "claude-opus-5-5", 100_000, 5), "Brouillon"),
            avec_titre(conv("autre", "cccc3333", "claude-opus-5-5", 100_000, 6), "Brouillon"),
        ];
        // les deux du même dossier reçoivent leur identifiant ; celle d'un autre dossier n'en a pas besoin
        assert_eq!(noms(&convs), ["Brouillon · projet-a · aaaa", "Brouillon · projet-a · bbbb", "Brouillon · autre"]);
    }

    #[test]
    fn un_titre_vide_ou_blanc_ne_change_pas_le_nom() {
        let convs = [avec_titre(conv("projet-a", "c95fb605", "claude-opus-5-5", 100_000, 3), "   ")];
        assert_eq!(noms(&convs), ["projet-a · c95f"]);
    }

    fn deux_groupes() -> Reglages {
        Reglages {
            separer: true,
            noms: ["Leads".into(), "Fils".into()],
            seuils: [Seuils { attention: 50, passation: 60 }, Seuils { attention: 40, passation: 50 }],
            groupe1: vec!["projet-a".into()],
            ..Reglages::default()
        }
    }

    fn vus_1m() -> BTreeMap<String, u64> {
        BTreeMap::from([("claude-opus-5-5".to_string(), 648_622), ("claude-sonnet-5-5".to_string(), 625_696)])
    }

    #[test]
    fn les_dossiers_de_la_fenetre_d_accueil() {
        // a : deux conversations actives ; b : une récente ; z : rangé dans le groupe 1 mais plus vu ;
        // c : plus vieux que 24 h, donc absent
        let convs = [
            conv("a", "1111", "claude-opus-5-5", 100_000, 1),
            conv("b", "2222", "claude-sonnet-5-5", 100_000, 60),
            conv("a", "3333", "claude-opus-5-5", 100_000, 2),
            conv("c", "4444", "claude-opus-5-5", 100_000, 3 * 24 * 60),
        ];
        let mut r = Reglages::default();
        r.groupe1 = vec!["b".into(), "z".into()];
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(
            dossiers_a_choisir(&s, &r),
            vec![("a".to_string(), false), ("b".to_string(), true), ("z".to_string(), true)],
            "sans doublon, vus d'abord, cochés = groupe 1"
        );
        // rien de vu, rien de rangé : une liste vide (la fenêtre ne s'ouvre pas toute seule)
        let vide = calculer(&[], &Reglages::default(), &BTreeMap::new(), MAINTENANT);
        assert!(dossiers_a_choisir(&vide, &Reglages::default()).is_empty());
    }

    #[test]
    fn la_fenetre_rouverte_propose_tout_ce_qui_etait_range() {
        // un seul groupe (« Toutes les conversations »), mais la liste du groupe 1 est restée rangée : rouvrir la fenêtre
        // propose les dossiers vus ET les dossiers rangés, ceux du groupe 1 cochés
        let convs = [conv("a", "1111", "claude-opus-5-5", 100_000, 1), conv("b", "2222", "claude-opus-5-5", 100_000, 2)];
        let mut r = Reglages::default();
        r.separer = false;
        r.groupe1 = vec!["b".into(), "ancien".into()];
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(s.groupes.len(), 1, "un seul groupe affiché");
        assert_eq!(
            dossiers_a_choisir(&s, &r),
            vec![("a".to_string(), false), ("b".to_string(), true), ("ancien".to_string(), true)]
        );
        // aucune lecture faite (ouverture trop tôt) : on propose quand même ce qui est rangé, coché, pour ne rien effacer
        assert_eq!(dossiers_sans_lecture(&r), vec![("b".to_string(), true), ("ancien".to_string(), true)]);
        assert!(dossiers_sans_lecture(&Reglages::default()).is_empty());
        // tout décocher puis séparer : le groupe 1 est vide, toutes les conversations passent dans le groupe 2
        r.appliquer_choix_groupes(true, Vec::new(), [String::new(), String::new()]);
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!((s.groupes.len(), s.groupes[0].lignes.len(), s.groupes[1].lignes.len()), (2, 0, 2));
        // revenir à un seul groupe : les deux conversations sont dans l'unique groupe, sans étiquette
        r.appliquer_choix_groupes(false, Vec::new(), [String::new(), String::new()]);
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!((s.groupes.len(), s.groupes[0].lignes.len()), (1, 2));
        assert!(s.groupes[0].lignes.iter().all(|l| l.etiquette.is_empty()));
    }

    #[test]
    fn le_titre_et_le_dossier_sont_nettoyes_pour_l_affichage() {
        // U+202E dans le titre ET dans le nom du dossier : ni l'un ni l'autre ne retourne la ligne
        let c = avec_titre(conv("dos\u{202E}sier", "c95fb605", "claude-opus-5-5", 100_000, 3), "Notes\u{202E} de voyage\nsuite\u{1b}");
        assert_eq!(noms(&[c]), ["Notes de voyage suite · dossier"]);
        // un titre qui n'est fait que de caractères retirés vaut « pas de titre » (dossier et 4 caractères)
        let c = avec_titre(conv("projet-a", "c95fb605", "claude-opus-5-5", 100_000, 3), "\u{202E}\n\u{200B}");
        assert_eq!(noms(&[c]), ["projet-a · c95f"]);
        // la CLÉ du dossier (clic sur l'étiquette, conversations.txt) reste le nom exact
        let c = conv("dos\u{202E}sier", "c95fb605", "claude-opus-5-5", 100_000, 3);
        let s = calculer(&[c], &Reglages::default(), &vus_1m(), MAINTENANT);
        assert_eq!(s.groupes[0].lignes[0].dossier, "dos\u{202E}sier");
        assert_eq!(s.groupes[0].lignes[0].nom, "dossier · c95f");
        // deux titres qui ne diffèrent que par un caractère retiré sont des doublons : l'identifiant les distingue
        let a = avec_titre(conv("p", "aaaa1111", "claude-opus-5-5", 1, 3), "Brouillon");
        let b = avec_titre(conv("p", "bbbb2222", "claude-opus-5-5", 1, 5), "Brouil\u{202E}lon");
        assert_eq!(noms(&[a, b]), ["Brouillon · p · aaaa", "Brouillon · p · bbbb"]);
    }

    #[test]
    fn le_clic_sur_l_etiquette_va_et_vient_entre_deux_groupes() {
        // deux groupes : projet-a chez les leads, projet-x chez les fils ; le clic change le camp du DOSSIER entier,
        // dans les deux sens, et l'étiquette de chaque ligne suit
        let mut r = deux_groupes();
        let convs = [
            conv("projet-a", "1111", "claude-opus-5-5", 1, 1),
            conv("projet-x", "2222", "claude-opus-5-5", 1, 2),
            conv("projet-x", "3333", "claude-sonnet-5-5", 1, 3),
        ];
        let etat = |r: &Reglages| {
            let s = calculer(&convs, r, &vus_1m(), MAINTENANT);
            let par_dossier = |d: &str| s.groupes.iter().flat_map(|g| g.lignes.iter()).filter(|l| l.dossier == d).map(|l| (l.groupe, l.etiquette.clone())).collect::<Vec<_>>();
            (par_dossier("projet-a"), par_dossier("projet-x"))
        };
        assert_eq!(etat(&r), (vec![(0, "LEAD".to_string())], vec![(1, "FIL".to_string()); 2]));
        r.changer_de_camp("projet-x"); // fils → leads
        assert_eq!(etat(&r), (vec![(0, "LEAD".to_string())], vec![(0, "LEAD".to_string()); 2]));
        r.changer_de_camp("projet-a"); // leads → fils
        assert_eq!(etat(&r), (vec![(1, "FIL".to_string())], vec![(0, "LEAD".to_string()); 2]));
        r.changer_de_camp("projet-a"); // et retour
        r.changer_de_camp("projet-x");
        assert_eq!(etat(&r), (vec![(0, "LEAD".to_string())], vec![(1, "FIL".to_string()); 2]), "revenu au départ");
    }

    #[test]
    fn les_quatre_etats_et_le_reste() {
        let s = Seuils { attention: 50, passation: 60 };
        assert_eq!(etat_de(41.0, s, false), Etat::Ok);
        assert_eq!(etat_de(50.0, s, false), Etat::Attention);
        assert_eq!(etat_de(59.4, s, false), Etat::Attention);
        assert_eq!(etat_de(60.0, s, false), Etat::Passation);
        assert_eq!(etat_de(95.0, s, false), Etat::Passation);
        assert_eq!(etat_de(5.0, s, true), Etat::Resumee, "résumée : passation tout de suite, même très bas");
        assert_eq!(reste_avant_passation(41.0, s), 19);
        assert_eq!(reste_avant_passation(75.0, s), 0);
    }

    #[test]
    fn ce_que_la_ligne_affiche_est_ce_qui_decide() {
        // 59,6 s'affiche « 60 » : la ligne ne peut pas dire « 60 sur 60 » et « reste 1 »
        let s = Seuils { attention: 50, passation: 60 };
        assert_eq!(etat_de(59.6, s, false), Etat::Passation);
        assert_eq!(reste_avant_passation(59.6, s), 0);
        assert_eq!(etat_de(59.4, s, false), Etat::Attention);
        assert_eq!(reste_avant_passation(59.4, s), 1);
    }

    #[test]
    fn une_ligne_complete() {
        let r = deux_groupes();
        let convs = [conv("projet-a", "3f9c1111", "claude-opus-5-5", 405_000, 2)];
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        langue::avec(Langue::Fr, || {
            let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
            let l = &s.groupes[0].lignes[0];
            assert_eq!(l.nom, "projet-a · 3f9c");
            assert_eq!(l.etiquette, "LEAD");
            assert_eq!(l.meta, "Opus 5.5 · il y a 2 min");
            assert_eq!(l.texte_pct, "≈ 41 % sur 60");
            assert_eq!(l.tokens, "405\u{a0}000 tokens");
            assert_eq!(l.reste, "reste 19");
            assert_eq!(l.etat, Etat::Ok);
        });
        assert_eq!(s.groupes.len(), 2);
    }

    #[test]
    fn les_seuils_sont_ceux_du_groupe_du_dossier() {
        let r = deux_groupes();
        // 520 000 sur 1 M = 52 % : passation pour un fil (50), pas pour un lead (60)
        let convs = [conv("projet-a", "aaaa", "claude-opus-5-5", 520_000, 3), conv("projet-x", "bbbb", "claude-opus-5-5", 520_000, 3)];
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(s.groupes[0].lignes[0].etat, Etat::Attention, "lead à 52 % sur 60");
        assert_eq!(s.groupes[1].lignes[0].etat, Etat::Passation, "fil à 52 % sur 50");
        assert_eq!(s.groupes[1].lignes[0].seuils.passation, 50);
        assert_eq!(s.groupes[1].texte_seuils, fmt(Cle::ConvSeuils, &[&40, &50]));
    }

    #[test]
    fn sans_separation_un_seul_groupe_avec_les_defauts() {
        let r = Reglages::default();
        let convs = [conv("projet-a", "aaaa", "claude-opus-5-5", 760_000, 3), conv("projet-x", "bbbb", "claude-opus-5-5", 100_000, 4)];
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(s.groupes.len(), 1);
        assert_eq!(s.groupes[0].lignes.len(), 2);
        assert_eq!(s.groupes[0].seuils, Seuils { attention: 70, passation: 80 });
        assert_eq!(s.groupes[0].lignes[0].etiquette, "", "pas d'étiquette sans séparation");
        assert_eq!(s.groupes[0].lignes[0].etat, Etat::Attention, "76 % entre 70 et 80");
        assert_eq!(s.groupes[0].nom, fmt(Cle::GroupeTous, &[]));
        // les réglages des deux groupes sont gardés quand on décoche, et reviennent quand on recoche
        let mut deux = deux_groupes();
        deux.separer = false;
        assert_eq!(deux.seuils[1], Seuils { attention: 40, passation: 50 });
        assert_eq!(calculer(&convs, &deux, &vus_1m(), MAINTENANT).groupes.len(), 1);
    }

    #[test]
    fn actives_et_recentes() {
        let r = Reglages::default();
        let convs = [
            conv("a", "1111", "claude-opus-5-5", 100_000, 29),
            conv("b", "2222", "claude-opus-5-5", 100_000, 31),
            conv("c", "3333", "claude-opus-5-5", 100_000, 23 * 60),
            conv("d", "4444", "claude-opus-5-5", 100_000, 25 * 60),
        ];
        let s = calculer(&convs, &r, &BTreeMap::new(), MAINTENANT);
        assert_eq!(s.groupes[0].lignes.len(), 1, "29 min : active");
        assert_eq!(s.recentes.len(), 2, "31 min et 23 h : récentes ; 25 h : absente");
        assert_eq!(s.recentes[0].dossier, "b", "la plus récente d'abord");
    }

    #[test]
    fn l_alerte_rose_ambre_ou_eteinte() {
        let r = Reglages::default();
        let ok = conv("a", "1111", "claude-opus-5-5", 100_000, 2);
        let attention = conv("b", "2222", "claude-opus-5-5", 750_000, 2);
        let passation = conv("c", "3333", "claude-opus-5-5", 850_000, 2);
        let vus = vus_1m();
        assert_eq!(calculer(&[ok.clone()], &r, &vus, MAINTENANT).alerte, Alerte::Aucune);
        assert_eq!(calculer(&[ok.clone(), attention.clone()], &r, &vus, MAINTENANT).alerte, Alerte::Attention);
        assert_eq!(calculer(&[attention.clone(), passation.clone()], &r, &vus, MAINTENANT).alerte, Alerte::Passation);
        // case décochée : jamais d'alerte
        let mut sans = r.clone();
        sans.alerte = false;
        assert_eq!(calculer(&[passation.clone()], &sans, &vus, MAINTENANT).alerte, Alerte::Aucune);
        // une conversation récente (pas active) ne fait pas clignoter
        let vieille = conv("c", "3333", "claude-opus-5-5", 850_000, 45);
        assert_eq!(calculer(&[vieille], &r, &vus, MAINTENANT).alerte, Alerte::Aucune);
    }

    #[test]
    fn resumee_passe_au_rose_tout_de_suite() {
        let r = Reglages::default();
        let mut c = conv("a", "1111", "claude-opus-5-5", 23_000, 1);
        c.resumee = true;
        let s = calculer(&[c], &r, &BTreeMap::new(), MAINTENANT);
        assert_eq!(s.alerte, Alerte::Passation);
        assert_eq!(s.groupes[0].lignes[0].etat, Etat::Resumee);
        assert_eq!(s.groupes[0].lignes[0].reste, t(Cle::ConvResumee));
    }

    fn dossiers_des_lignes(lignes: &[LigneSuivi]) -> Vec<String> {
        lignes.iter().map(|l| l.dossier.clone()).collect()
    }

    #[test]
    fn une_conversation_masquee_disparait_des_deux_sections_et_du_compte() {
        let mut r = Reglages::default();
        let active = conv("a", "aaaa1111", "claude-opus-5-5", 100_000, 2);
        let autre_active = conv("b", "bbbb2222", "claude-opus-5-5", 100_000, 3);
        let recente = conv("c", "cccc3333", "claude-opus-5-5", 100_000, 120);
        let autre_recente = conv("d", "dddd4444", "claude-opus-5-5", 100_000, 180);
        let convs = [active.clone(), autre_active, recente.clone(), autre_recente.clone()];
        let avant = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!((avant.groupes[0].lignes.len(), avant.recentes.len(), avant.masquees), (2, 2, 0));
        // masquer une active ET une repliée
        assert!(r.masquer(&active.session));
        assert!(r.masquer(&recente.session));
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(dossiers_des_lignes(&s.groupes[0].lignes), ["b"], "la section dépliée");
        assert_eq!(dossiers_des_lignes(&s.recentes), ["d"], "la section repliée");
        // le « + n autres » de l'écran est `recentes.len()` : il baisse avec elle
        assert_eq!(s.recentes.len(), 1);
        assert_eq!(s.masquees, 2);
        // c'est l'identifiant de session que le ✕ de la ligne désigne
        assert_eq!((s.groupes[0].lignes[0].session.as_str(), s.recentes[0].session.as_str()), ("bbbb2222", "dddd4444"));
        // toutes les repliées masquées : plus de ligne « + n autres » du tout
        r.masquer(&autre_recente.session);
        assert!(calculer(&convs, &r, &vus_1m(), MAINTENANT).recentes.is_empty());
        // un dossier dont toutes les conversations sont masquées n'est plus proposé par « Choisir les groupes… », sauf s'il est déjà rangé
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(dossiers_a_choisir(&s, &r), vec![("b".to_string(), false)]);
        r.groupe1 = vec!["a".into()];
        assert_eq!(dossiers_a_choisir(&s, &r), vec![("b".to_string(), false), ("a".to_string(), true)]);
    }

    #[test]
    fn masquer_eteint_l_alerte_rose_d_une_conversation_archivee() {
        // la fausse alerte constatée à l'usage : une conversation archivée puis « résumée » allume le rose tant que son dernier message a moins de 30 min
        let mut archivee = conv("archive", "arch0001", "claude-opus-5-5", 23_000, 5);
        archivee.resumee = true;
        let calme = conv("b", "bbbb2222", "claude-opus-5-5", 100_000, 2);
        let mut r = Reglages::default();
        assert_eq!(calculer(&[archivee.clone(), calme.clone()], &r, &vus_1m(), MAINTENANT).alerte, Alerte::Passation);
        r.masquer("arch0001");
        let s = calculer(&[archivee, calme], &r, &vus_1m(), MAINTENANT);
        assert_eq!(s.alerte, Alerte::Aucune, "masquée : plus de rose");
        assert_eq!(s.groupes[0].lignes.len(), 1);
        // masquer la passation laisse parler l'attention d'une autre conversation, puis plus rien
        let passation = conv("c", "cccc3333", "claude-opus-5-5", 850_000, 2);
        let attention = conv("d", "dddd4444", "claude-opus-5-5", 750_000, 2);
        let toutes = [passation, attention];
        let mut r = Reglages::default();
        assert_eq!(calculer(&toutes, &r, &vus_1m(), MAINTENANT).alerte, Alerte::Passation);
        r.masquer("cccc3333");
        assert_eq!(calculer(&toutes, &r, &vus_1m(), MAINTENANT).alerte, Alerte::Attention);
        r.masquer("dddd4444");
        assert_eq!(calculer(&toutes, &r, &vus_1m(), MAINTENANT).alerte, Alerte::Aucune);
    }

    #[test]
    fn un_nouveau_message_ne_demasque_pas_et_l_oubli_remet_a_zero() {
        let mut r = Reglages::default();
        let avant = conv("a", "aaaa1111", "claude-opus-5-5", 100_000, 45);
        r.masquer("aaaa1111");
        let s = calculer(&[avant], &r, &vus_1m(), MAINTENANT);
        assert!(s.recentes.is_empty() && s.groupes[0].lignes.is_empty());
        // un nouveau message arrive : plus récent, contexte plein, et même résumée : la conversation RESTE masquée, sans alerte
        let mut apres = conv("a", "aaaa1111", "claude-opus-5-5", 900_000, 1);
        apres.resumee = true;
        let s = calculer(&[apres.clone()], &r, &vus_1m(), MAINTENANT);
        assert!(s.groupes[0].lignes.is_empty() && s.recentes.is_empty());
        assert_eq!((s.alerte, s.masquees), (Alerte::Aucune, 1));
        // la lecture suivante ne montre plus cette session (plus de 24 h) mais une autre : son identifiant est oublié
        let autre = conv("z", "zzzz9999", "claude-opus-5-5", 1, 3);
        assert!(r.oublier_masquees_absentes(&[autre.session.as_str()]));
        assert!(r.masquees.is_empty());
        // si la session répond de nouveau, elle est visible (et résumée : le rose revient)
        let s = calculer(&[apres], &r, &vus_1m(), MAINTENANT);
        assert_eq!((s.groupes[0].lignes.len(), s.alerte, s.masquees), (1, Alerte::Passation, 0));
    }

    #[test]
    fn reafficher_ramene_les_conversations_masquees() {
        let mut r = Reglages::default();
        let convs = [conv("a", "aaaa1111", "claude-opus-5-5", 1, 2), conv("b", "bbbb2222", "claude-opus-5-5", 1, 90)];
        r.masquer("aaaa1111");
        r.masquer("bbbb2222");
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!((s.groupes[0].lignes.len(), s.recentes.len(), s.masquees), (0, 0, 2));
        r.reafficher_tout();
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!((s.groupes[0].lignes.len(), s.recentes.len(), s.masquees), (1, 1, 0));
    }

    #[test]
    fn une_conversation_masquee_compte_encore_pour_les_contextes_vus() {
        // la fenêtre « Auto » d'un modèle se fonde sur le plus grand contexte vu : masquer une conversation ne la fait pas redescendre
        let mut r = Reglages::default();
        let grosse = conv("a", "aaaa1111", "claude-opus-5-5", 700_000, 2);
        let petite = conv("b", "bbbb2222", "claude-opus-5-5", 100_000, 3);
        r.masquer("aaaa1111");
        let s = calculer(&[grosse, petite], &r, &BTreeMap::new(), MAINTENANT);
        assert_eq!(s.vus["claude-opus-5-5"], 700_000);
        assert!((s.groupes[0].lignes[0].pct - 10.0).abs() < 1e-9, "la visible est mesurée sur 1 M, comme si l'autre était là");
        assert_eq!(s.modeles.len(), 1);
        // deux conversations du même titre : si l'une est masquée, l'autre n'a plus besoin de l'identifiant court
        let a = avec_titre(conv("p", "aaaa1111", "claude-opus-5-5", 1, 3), "Brouillon");
        let b = avec_titre(conv("p", "bbbb2222", "claude-opus-5-5", 1, 5), "Brouillon");
        assert_eq!(noms(&[a.clone(), b.clone()]), ["Brouillon · p · aaaa", "Brouillon · p · bbbb"]);
        let mut r = Reglages::default();
        r.masquer("aaaa1111");
        let s = calculer(&[a, b], &r, &vus_1m(), MAINTENANT);
        assert_eq!(s.groupes[0].lignes[0].nom, "Brouillon · p");
    }

    #[test]
    fn la_fenetre_par_modele() {
        // Auto : un modèle qui a dépassé 200 000 tokens est en 1 M, les autres en 200 000
        let r = Reglages::default();
        let convs = [conv("a", "1111", "claude-opus-5-5", 300_000, 1), conv("b", "2222", "claude-haiku-5-5", 90_000, 1)];
        let s = calculer(&convs, &r, &BTreeMap::new(), MAINTENANT);
        let opus = s.modeles.iter().find(|m| m.cle == "claude-opus-5-5").unwrap();
        let haiku = s.modeles.iter().find(|m| m.cle == "claude-haiku-5-5").unwrap();
        assert_eq!(opus.auto, FENETRE_GRANDE);
        assert_eq!(haiku.auto, FENETRE_PETITE);
        assert_eq!(opus.nom, "Opus 5.5");
        let l = |nom: &str| s.groupes[0].lignes.iter().find(|l| l.dossier == nom).unwrap().pct;
        assert!((l("a") - 30.0).abs() < 1e-9);
        assert!((l("b") - 45.0).abs() < 1e-9);
        // choix à la main : 200 000 pour Opus
        let mut petit = r.clone();
        petit.fenetres.insert("claude-opus-5-5".into(), ChoixFenetre::Petite);
        let s2 = calculer(&convs, &petit, &BTreeMap::new(), MAINTENANT);
        assert!((s2.groupes[0].lignes.iter().find(|l| l.dossier == "a").unwrap().pct - 150.0).abs() < 1e-9);
    }

    #[test]
    fn les_contextes_vus_ne_redescendent_pas() {
        let connus = BTreeMap::from([("claude-opus-5-5".to_string(), 648_622u64)]);
        // la conversation vient d'être résumée : son contexte est bas, mais le modèle reste « vu » à 648 622
        let c = conv("a", "1111", "claude-opus-5-5", 23_000, 1);
        let s = calculer(&[c], &Reglages::default(), &connus, MAINTENANT);
        assert_eq!(s.vus["claude-opus-5-5"], 648_622);
        assert!((s.groupes[0].lignes[0].pct - 2.3).abs() < 1e-9, "toujours sur 1 M");
        // et un contexte plus grand monte le maximum
        let grand = conv("a", "1111", "claude-opus-5-5", 700_000, 1);
        assert_eq!(contextes_vus(&[grand], &connus)["claude-opus-5-5"], 700_000);
    }

    #[test]
    fn les_modeles_de_la_fenetre_de_reglage() {
        let mut r = Reglages::default();
        r.fenetres.insert("claude-vieux-1".into(), ChoixFenetre::Grande);
        r.fenetres.insert("claude-oublie-2".into(), ChoixFenetre::Auto);
        let convs = [conv("a", "1111", "claude-opus-5-5", 1, 1), conv("b", "2222", "claude-sonnet-5-5", 1, 2), conv("c", "3333", "claude-opus-5-5", 1, 3)];
        let noms: Vec<String> = calculer(&convs, &r, &BTreeMap::new(), MAINTENANT).modeles.into_iter().map(|m| m.cle).collect();
        // sans doublon, le plus récent d'abord, puis le modèle réglé à la main ; « Auto » seul n'a rien à montrer
        assert_eq!(noms, ["claude-opus-5-5", "claude-sonnet-5-5", "claude-vieux-1"]);
    }

    #[test]
    fn l_ordre_des_modeles_est_fixe() {
        let cles = |convs: &[Conversation]| -> Vec<String> { calculer(convs, &Reglages::default(), &BTreeMap::new(), MAINTENANT).modeles.into_iter().map(|m| m.cle).collect() };
        // les mêmes cinq modèles, avec chacun d'eux tour à tour le plus récent : toujours le même ordre
        let tout = ["claude-haiku-5-5", "claude-zeta-1", "claude-opus-4-1", "claude-sonnet-5-5", "claude-opus-5-5"];
        let attendu = ["claude-opus-5-5", "claude-opus-4-1", "claude-sonnet-5-5", "claude-haiku-5-5", "claude-zeta-1"];
        for decalage in 0..tout.len() {
            let convs: Vec<Conversation> = (0..tout.len()).map(|i| conv("a", &format!("{i:04}"), tout[(i + decalage) % tout.len()], 1, 1 + i as i64)).collect();
            assert_eq!(cles(&convs), attendu, "décalage {decalage} : l'ordre ne dépend pas de la conversation la plus récente");
        }
        // plus de MODELES_MAX modèles : on garde les plus récents (les premiers), puis on range ; le plus ancien est écarté
        let six_noms = ["claude-haiku-5-5", "claude-3-5-sonnet", "claude-opus-4-1", "claude-sonnet-5-5", "claude-opus-5-5", "claude-opus-3"];
        let six: Vec<Conversation> = six_noms.iter().enumerate().map(|(i, m)| conv("a", &format!("{i:04}"), m, 1, 1 + i as i64)).collect();
        let vus = cles(&six);
        assert_eq!(vus.len(), MODELES_MAX);
        assert_eq!(vus, ["claude-opus-5-5", "claude-opus-4-1", "claude-sonnet-5-5", "claude-3-5-sonnet", "claude-haiku-5-5"][..], "le sixième (un Opus ANCIEN, qui se rangerait avant les autres) est écarté : on garde les cinq plus récents PUIS on range, pas l'inverse");
        // la comparaison seule
        use std::cmp::Ordering::{Greater, Less};
        assert_eq!(ordre_modeles("claude-opus-5-5", "claude-sonnet-5-5"), Less);
        assert_eq!(ordre_modeles("claude-haiku-5-5", "claude-sonnet-5-5"), Greater);
        assert_eq!(ordre_modeles("claude-opus-4-1", "claude-opus-4"), Less, "4.1 avant 4");
        assert_eq!(ordre_modeles("claude-3-5-sonnet", "claude-sonnet-4"), Greater, "3.5 après 4 dans la famille Sonnet");
    }

    #[test]
    fn les_etiquettes_de_groupe() {
        assert_eq!(etiquette_groupe("Leads"), "LEAD");
        assert_eq!(etiquette_groupe("Fils"), "FIL");
        assert_eq!(etiquette_groupe("Groupe 1"), "G1");
        assert_eq!(etiquette_groupe("Group 2"), "G2");
        assert_eq!(etiquette_groupe("Perso"), "PERS");
        assert_eq!(etiquette_groupe("  "), "");
        assert_eq!(etiquette_groupe("Bus"), "BUS", "un nom court garde son s");
        assert_eq!(etiquette_groupe("主要"), "主要");
    }

    #[test]
    fn le_clic_sur_l_etiquette_deplace_tout_le_dossier() {
        let mut r = deux_groupes();
        let convs = [conv("projet-x", "1111", "claude-opus-5-5", 1, 1), conv("projet-x", "2222", "claude-sonnet-5-5", 1, 2)];
        assert_eq!(calculer(&convs, &r, &vus_1m(), MAINTENANT).groupes[1].lignes.len(), 2, "des fils");
        r.changer_de_camp("projet-x");
        let s = calculer(&convs, &r, &vus_1m(), MAINTENANT);
        assert_eq!(s.groupes[0].lignes.len(), 2, "les deux conversations du dossier passent chez les leads");
        assert!(s.groupes[1].lignes.is_empty());
    }

    #[test]
    fn la_barre_d_essai_a_trois_phrases() {
        let s = Seuils { attention: 50, passation: 60 };
        langue::avec(Langue::Fr, || {
            assert_eq!(texte_essai(s, 30), (Etat::Ok, "Exemple à 30 % : reste 30 avant la passation".to_string()));
            assert_eq!(texte_essai(s, 56), (Etat::Attention, "Exemple à 56 % : attention, reste 4".to_string()));
            assert_eq!(texte_essai(s, 60), (Etat::Passation, "Exemple à 60 % : passation".to_string()));
        });
        assert_eq!(essai_par_defaut(s), 55);
        assert_eq!(essai_par_defaut(Seuils { attention: 70, passation: 80 }), 75);
    }

    #[test]
    fn les_milliers_suivent_la_langue() {
        langue::avec(Langue::En, || assert_eq!(entier(1_234_567), "1,234,567"));
        langue::avec(Langue::Fr, || assert_eq!(entier(405_000), "405\u{a0}000"));
        langue::avec(Langue::De, || assert_eq!(entier(999), "999"));
        langue::avec(Langue::De, || assert_eq!(entier(12_345), "12.345"));
        langue::avec(Langue::Ja, || assert_eq!(entier(0), "0"));
    }

    #[test]
    fn les_noms_de_groupe_par_defaut_sont_traduits() {
        let mut r = Reglages { separer: true, ..Reglages::default() };
        langue::avec(Langue::Fr, || assert_eq!(nom_du_groupe(&r, 1), "Groupe 2"));
        langue::avec(Langue::De, || assert_eq!(nom_du_groupe(&r, 0), "Gruppe 1"));
        r.noms[0] = "Mes leads".into();
        assert_eq!(nom_du_groupe(&r, 0), "Mes leads", "un nom tapé à la main n'est pas traduit");
    }
    #[test]
    fn la_demonstration_montre_tous_les_etats() {
        let r = Reglages { separer: true, groupe1: vec!["lead-alpha".into(), "lead-beta".into()], ..deux_groupes() };
        let s = calculer(&exemple(MAINTENANT), &r, &vus_demo(), MAINTENANT);
        let actives: Vec<Etat> = s.groupes.iter().flat_map(|g| g.lignes.iter().map(|l| l.etat)).collect();
        assert!(actives.contains(&Etat::Ok) && actives.contains(&Etat::Attention) && actives.contains(&Etat::Passation));
        assert_eq!(s.recentes.len(), 4, "trois conversations de 30 min à 24 h, plus la résumée de 6 h 50");
        assert!(s.recentes.iter().any(|l| l.etat == Etat::Resumee));
        assert_eq!(s.alerte, Alerte::Passation);
    }
}
