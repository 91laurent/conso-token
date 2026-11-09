//! Les conversations de Claude Code : pour chacune, le contexte au dernier échange (entrée + création
//! de cache + lecture de cache du dernier message de Claude), le modèle, le dossier de travail et si
//! elle a été résumée (compactée). Tiré des transcriptions (`~/.claude/projects/**/*.jsonl`).
//!
//! GARDE-FOU (comme `transcripts`) : seuls ces champs sont lus : `type`, `subtype`, `timestamp`,
//! `sessionId`, `isSidechain`, `cwd`, `turnOrigin`, `message.model`, les compteurs `message.usage.*` et
//! `customTitle` (le titre que la personne a donné à la conversation, lu SEULEMENT sur une ligne de type
//! `custom-title`, pour la nommer dans la liste ; gardé en mémoire seulement, jamais écrit ni envoyé). Le
//! contenu d'un message n'est jamais gardé : `serde` saute tout le reste sans le garder.
//!
//! `sessionId` sert aussi, depuis la 1.3.4, à RECONNAÎTRE une conversation que la personne a masquée avec le ✕
//! d'une ligne (voir `reglages.rs` et `suivi::calculer`). C'est la SEULE chose, tirée d'une conversation, qui soit
//! écrite sur le disque : cet identifiant (un simple code, ligne `masquee=<identifiant>` de `conversations.txt`,
//! oubliée quand le dernier message a plus de 24 h ou au clic sur « Réafficher »). Jamais le titre, jamais le nom du
//! dossier de travail, jamais le contenu d'un message : ceux-là restent en mémoire, le temps de l'affichage, ou ne
//! sont pas gardés du tout.
//!
//! Ce qui est lu de chaque fichier :
//! - la TÊTE (16 Ko, élargie par pas jusqu'à 1 Mo si la première ligne qui porte `turnOrigin` est plus
//!   loin) une seule fois, pour écarter les exécutions planifiées (`turnOrigin = scheduled`) ;
//! - la QUEUE (64 Ko, élargie jusqu'à 4 Mo si le dernier message de Claude est plus loin) pour le
//!   dernier message de Claude et le dernier titre donné à la conversation ;
//! - le fichier entier, une seule fois puis seulement ce qui s'ajoute, pour repérer une ligne dont le
//!   sous-type JSON de premier niveau est `compact_boundary` (jamais une simple recherche du mot : un
//!   résultat d'outil le contient aussi).
//!
//! Écartés : les fichiers `subagents\`, les messages `isSidechain`, le modèle `<synthetic>`, les
//! sessions planifiées. Les copies de travail `.claude\worktrees\…` sont ramenées à leur projet.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use jiff::Timestamp;
use serde::Deserialize;

pub const QUEUE: u64 = 64 * 1024;
const QUEUE_MAX: u64 = 4 * 1024 * 1024;
pub const TETE: u64 = 16 * 1024;
const TETE_MAX: u64 = 1024 * 1024;
/// Une conversation reste visible 24 h après son dernier message.
pub const RECENTE_S: i64 = 24 * 3_600;
const MARQUE: &str = "compact_boundary";
/// Le type de la ligne qui porte le titre donné à la conversation.
const TYPE_TITRE: &str = "custom-title";
/// Un titre plus long est coupé (la liste le fait défiler au survol, mais rien ne sert de garder un roman).
pub const TITRE_MAX: usize = 60;

#[derive(Deserialize)]
struct Ligne {
    #[serde(rename = "type")]
    genre: Option<String>,
    subtype: Option<String>,
    timestamp: Option<String>,
    #[serde(rename = "sessionId")]
    session: Option<String>,
    #[serde(rename = "isSidechain")]
    lateral: Option<bool>,
    cwd: Option<String>,
    #[serde(rename = "turnOrigin")]
    origine: Option<String>,
    message: Option<Message>,
}

/// Une ligne lue UNIQUEMENT pour son titre : `dernier_titre` ne l'applique qu'aux lignes qui contiennent `custom-title`.
/// `customTitle` n'est volontairement PAS un champ de `Ligne` : sinon une ligne d'un autre type (une réponse de Claude)
/// qui porterait un `customTitle` d'un autre type que du texte serait rejetée en entier, et sa conversation disparaîtrait
/// sans alarme. Ici, au pire, c'est le titre d'une ligne `custom-title` mal formée qui est ignoré (le nom redevient
/// « dossier · 4 caractères »).
#[derive(Deserialize)]
struct LigneTitre {
    #[serde(rename = "type")]
    genre: Option<String>,
    #[serde(rename = "customTitle")]
    titre: Option<String>,
}

#[derive(Deserialize)]
struct Message {
    model: Option<String>,
    usage: Option<Usage>,
}

#[derive(Deserialize, Clone, Copy, Default)]
struct Usage {
    input_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
}

impl Usage {
    /// Le contexte : tout ce que Claude a relu pour répondre (entrée + création + lecture de cache).
    fn contexte(&self) -> u64 {
        self.input_tokens.unwrap_or(0) + self.cache_creation_input_tokens.unwrap_or(0) + self.cache_read_input_tokens.unwrap_or(0)
    }
}

/// Une conversation, au dernier message de Claude.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conversation {
    pub session: String,
    /// Dernier segment du dossier de travail (une copie `.claude\worktrees\…` est ramenée au projet).
    pub dossier: String,
    /// Identifiant du modèle, normalisé (voir `cle_modele`).
    pub modele: String,
    pub contexte: u64,
    /// Instant du dernier message de Claude (secondes Unix).
    pub dernier: i64,
    /// Une ligne `compact_boundary` a été vue : la conversation a été résumée.
    pub resumee: bool,
    /// Le dernier titre que la personne a donné à la conversation (`Notes de voyage`), s'il y en a un.
    pub titre: Option<String>,
}

/// Ce qu'une lecture a trouvé.
#[derive(Clone, Debug, Default)]
pub struct Lecture {
    pub conversations: Vec<Conversation>,
    /// Fichiers de moins de 24 h qui pourraient être de vraies conversations (ni sous-agents, ni planifiées)
    /// et où Claude a déjà répondu (une conversation qui vient de s'ouvrir n'en est pas un).
    pub candidats: usize,
    /// Parmi eux, ceux dont le dernier message de Claude a pu être lu.
    pub lisibles: usize,
}

impl Lecture {
    /// Des fichiers récents existent mais aucun n'est lisible : le format des transcriptions a
    /// sans doute changé. Mieux vaut cacher la liste que montrer de faux chiffres.
    pub fn format_incompris(&self) -> bool {
        self.candidats >= 3 && self.lisibles == 0
    }
}

// ---------------------------------------------------------------- noms

/// Un texte venu d'une transcription (titre, nom de dossier, modèle) rendu sûr pour l'AFFICHAGE : les caractères de
/// contrôle (retour à la ligne, tabulation, ESC, NUL…) et les séparateurs de ligne ou de paragraphe deviennent une
/// espace (une suite en donne une seule) ; les caractères qui changent le SENS de lecture (U+202A à U+202E,
/// U+2066 à U+2069, U+200E, U+200F, U+061C : U+202E retournerait aussi le reste de la ligne, nom du dossier compris)
/// et les caractères sans largeur (U+200B à U+200D, U+2060, U+FEFF) sont retirés. Le texte gardé en mémoire n'est PAS
/// changé : seul ce qu'on montre l'est (la clé d'un dossier reste son nom exact).
pub fn pour_affichage(texte: &str) -> String {
    let mut sortie = String::with_capacity(texte.len());
    let mut espace_en_attente = false;
    for c in texte.chars() {
        match c {
            '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{200E}' | '\u{200F}' | '\u{061C}' | '\u{200B}'..='\u{200D}' | '\u{2060}' | '\u{FEFF}' => {}
            c if c.is_control() || c == '\u{2028}' || c == '\u{2029}' => espace_en_attente = true,
            c => {
                if espace_en_attente && !sortie.is_empty() {
                    sortie.push(' ');
                }
                espace_en_attente = false;
                sortie.push(c);
            }
        }
    }
    sortie
}

/// `.claude\worktrees\x` est une copie de travail du projet qui la contient ; sinon le dernier
/// segment du dossier de travail (Windows ou Unix).
pub fn dossier_de_cwd(cwd: &str) -> String {
    let norme = cwd.replace('\\', "/");
    let segments: Vec<&str> = norme.split('/').filter(|s| !s.is_empty()).collect();
    if let Some(i) = segments.windows(2).position(|w| w[0] == ".claude" && w[1] == "worktrees") {
        if i >= 1 {
            return segments[i - 1].to_string();
        }
    }
    segments.last().map(|s| s.to_string()).unwrap_or_else(|| "?".to_string())
}

/// L'identifiant d'un modèle, sans date ni suffixe : `claude-sonnet-4-5-20250929` → `claude-sonnet-4-5`.
pub fn cle_modele(brut: &str) -> String {
    let mut s = brut.trim().to_lowercase();
    if let Some(i) = s.find('[') {
        s.truncate(i);
    }
    if let Some(i) = s.rfind('-') {
        let fin = &s[i + 1..];
        if fin.len() == 8 && fin.chars().all(|c| c.is_ascii_digit()) {
            s.truncate(i);
        }
    }
    s.trim().to_string()
}

/// Le nom montré : `claude-opus-5-5` → `Opus 5.5`, `claude-3-5-sonnet` → `Sonnet 3.5`.
pub fn nom_modele(cle: &str) -> String {
    let s = cle.strip_prefix("claude-").unwrap_or(cle);
    let morceaux: Vec<&str> = s.split('-').filter(|m| !m.is_empty()).collect();
    let famille: Vec<String> = morceaux
        .iter()
        .filter(|m| m.chars().any(char::is_alphabetic))
        .map(|m| {
            let mut c = m.chars();
            c.next().map(|p| p.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
        })
        .collect();
    let version: Vec<&str> = morceaux.iter().copied().filter(|m| m.chars().all(|c| c.is_ascii_digit())).collect();
    if famille.is_empty() {
        return cle.to_string();
    }
    if version.is_empty() { famille.join(" ") } else { format!("{} {}", famille.join(" "), version.join(".")) }
}

// ---------------------------------------------------------------- lecture d'un fichier

#[derive(Clone, Default)]
struct Tete {
    /// La première ligne qui porte `turnOrigin` a été trouvée (ou la limite de lecture atteinte).
    fixee: bool,
    planifiee: bool,
}

#[derive(Clone)]
struct Dernier {
    t: i64,
    session: Option<String>,
    modele: String,
    contexte: u64,
    cwd: Option<String>,
}

#[derive(Default)]
struct EtatFichier {
    taille: u64,
    lu_marques: u64,
    resumee: bool,
    tete: Tete,
    dernier: Option<Dernier>,
    /// Claude a répondu au moins une fois dans ce fichier (voir `a_une_reponse`).
    repondu: bool,
    /// Le dernier titre vu dans la queue (gardé tant qu'un titre plus récent n'a pas été lu).
    titre: Option<String>,
    fait: bool,
}

fn lire_plage(chemin: &Path, debut: u64, fin: u64) -> Option<Vec<u8>> {
    let mut f = File::open(chemin).ok()?;
    f.seek(SeekFrom::Start(debut)).ok()?;
    let mut v = Vec::with_capacity(fin.saturating_sub(debut).min(QUEUE_MAX) as usize);
    f.take(fin.saturating_sub(debut)).read_to_end(&mut v).ok()?;
    Some(v)
}

fn analyser(ligne: &[u8]) -> Option<Ligne> {
    serde_json::from_slice::<Ligne>(ligne).ok()
}

/// Les lignes de `octets` qui contiennent `mot` (le préfiltre : on n'analyse que celles-là).
fn lignes_avec<'a>(octets: &'a [u8], mot: &'a str) -> impl Iterator<Item = &'a [u8]> {
    octets.split(|b| *b == b'\n').filter(move |l| std::str::from_utf8(l).is_ok_and(|t| t.contains(mot)))
}

fn est_marque(ligne: &[u8]) -> bool {
    analyser(ligne).is_some_and(|l| l.subtype.as_deref() == Some(MARQUE))
}

/// La tête : la première ligne qui porte `turnOrigin` dit si la session est planifiée.
fn lire_tete(chemin: &Path, taille: u64) -> Tete {
    let mut largeur = TETE;
    loop {
        let fin = largeur.min(taille);
        let Some(octets) = lire_plage(chemin, 0, fin) else { return Tete::default() };
        // lignes complètes seulement (la dernière peut être coupée par la limite de lecture)
        let utile: &[u8] = if fin < taille {
            match octets.iter().rposition(|b| *b == b'\n') {
                Some(i) => &octets[..=i],
                None => &[],
            }
        } else {
            &octets
        };
        for ligne in lignes_avec(utile, "turnOrigin") {
            if let Some(o) = analyser(ligne).and_then(|l| l.origine) {
                return Tete { fixee: true, planifiee: o == "scheduled" };
            }
        }
        if fin >= taille {
            return Tete::default(); // rien encore : peut-être plus tard, quand le fichier aura grandi
        }
        if largeur >= TETE_MAX {
            return Tete { fixee: true, planifiee: false };
        }
        largeur *= 4;
    }
}

/// Parcourt le fichier de `depuis` à la fin, ligne complète par ligne complète : rend l'octet où
/// reprendre, et si une ligne `compact_boundary` a été vue.
fn chercher_marques(chemin: &Path, depuis: u64) -> (u64, bool) {
    let Ok(mut f) = File::open(chemin) else { return (depuis, false) };
    if f.seek(SeekFrom::Start(depuis)).is_err() {
        return (depuis, false);
    }
    let mut lecteur = BufReader::with_capacity(256 * 1024, f);
    let (mut position, mut trouve) = (depuis, false);
    let mut tampon = Vec::new();
    loop {
        tampon.clear();
        let n = match lecteur.read_until(b'\n', &mut tampon) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        let complete = tampon.last() == Some(&b'\n');
        let marque = std::str::from_utf8(&tampon).is_ok_and(|t| t.contains(MARQUE)) && est_marque(&tampon);
        // une dernière ligne sans retour à la ligne est reprise à la lecture suivante, sauf si elle est déjà une marque entière
        if !complete && !marque {
            break;
        }
        position += n as u64;
        trouve |= marque;
    }
    (position, trouve)
}

/// Le dernier message de Claude d'un bloc de lignes : hors `isSidechain`, hors `<synthetic>`, avec des compteurs.
fn dernier_message(octets: &[u8]) -> Option<Dernier> {
    let mut dernier: Option<Dernier> = None;
    let mut cwd: Option<String> = None;
    for ligne in lignes_avec(octets, "\"usage\"") {
        let Some(l) = analyser(ligne) else { continue };
        if l.cwd.is_some() {
            cwd = l.cwd.clone();
        }
        if l.genre.as_deref() != Some("assistant") || l.lateral == Some(true) {
            continue;
        }
        let Some(m) = l.message else { continue };
        let (Some(modele), Some(usage), Some(ts)) = (m.model, m.usage, l.timestamp) else { continue };
        if modele == "<synthetic>" || modele.trim().is_empty() {
            continue;
        }
        let contexte = usage.contexte();
        let Ok(t) = Timestamp::from_str(&ts) else { continue };
        if contexte == 0 {
            continue;
        }
        dernier = Some(Dernier { t: t.as_second(), session: l.session, modele, contexte, cwd: l.cwd.or_else(|| cwd.clone()) });
    }
    dernier
}

/// Claude a-t-il déjà répondu dans ce bloc de lignes ? Une ligne `assistant` (hors `isSidechain`, hors
/// `<synthetic>`) suffit, qu'on sache lire ses compteurs ou non. Sert au garde-fou « format incompris » : une
/// conversation qui vient de s'ouvrir (le premier message de l'utilisateur, rien d'autre) ne dit rien du format.
fn a_une_reponse(octets: &[u8]) -> bool {
    lignes_avec(octets, "\"assistant\"").any(|ligne| {
        analyser(ligne).is_some_and(|l| {
            l.genre.as_deref() == Some("assistant")
                && l.lateral != Some(true)
                && l.message.as_ref().and_then(|m| m.model.as_deref()) != Some("<synthetic>")
        })
    })
}

/// Le dernier titre donné à la conversation dans ce bloc de lignes : la dernière ligne dont le type de premier
/// niveau est `custom-title` (jamais une simple recherche du mot : un message qui le cite ne compte pas), sans
/// les espaces autour, coupée à `TITRE_MAX` caractères. Un titre vide ne compte pas.
fn dernier_titre(octets: &[u8]) -> Option<String> {
    let mut titre: Option<String> = None;
    for ligne in lignes_avec(octets, TYPE_TITRE) {
        let Some(l) = serde_json::from_slice::<LigneTitre>(ligne).ok() else { continue };
        if l.genre.as_deref() != Some(TYPE_TITRE) {
            continue;
        }
        let Some(t) = l.titre else { continue };
        let t: String = t.trim().chars().take(TITRE_MAX).collect();
        if !t.is_empty() {
            titre = Some(t);
        }
    }
    titre
}

/// La queue du fichier (64 Ko, élargie si le dernier message de Claude est plus loin) : le dernier message de
/// Claude, si Claude a répondu au moins une fois dans ce qui a été lu, et le dernier titre donné à la conversation.
fn lire_queue(chemin: &Path, taille: u64) -> (Option<Dernier>, bool, Option<String>) {
    let mut largeur = QUEUE;
    loop {
        let debut = taille.saturating_sub(largeur);
        let Some(octets) = lire_plage(chemin, debut, taille) else { return (None, false, None) };
        // si on ne part pas du début du fichier, la première ligne est coupée : on la saute
        let utile: &[u8] = if debut > 0 {
            match octets.iter().position(|b| *b == b'\n') {
                Some(i) => &octets[i + 1..],
                None => &[],
            }
        } else {
            &octets
        };
        let trouve = dernier_message(utile);
        if trouve.is_some() || debut == 0 || largeur >= QUEUE_MAX {
            let repondu = trouve.is_some() || a_une_reponse(utile);
            return (trouve, repondu, dernier_titre(utile));
        }
        largeur *= 4;
    }
}

fn actualiser(etat: &mut EtatFichier, chemin: &Path, taille: u64) {
    if taille < etat.taille {
        *etat = EtatFichier::default(); // fichier réécrit plus court : on repart du début
    }
    if etat.fait && taille == etat.taille {
        return;
    }
    if !etat.tete.fixee {
        etat.tete = lire_tete(chemin, taille);
    }
    etat.taille = taille;
    etat.fait = true;
    if etat.tete.planifiee {
        return;
    }
    let (position, trouve) = chercher_marques(chemin, etat.lu_marques);
    etat.lu_marques = position;
    etat.resumee |= trouve;
    let (dernier, repondu, titre) = lire_queue(chemin, taille);
    etat.repondu |= repondu;
    if titre.is_some() {
        etat.titre = titre;
    }
    if let Some(d) = dernier {
        etat.dernier = Some(d);
    }
}

// ---------------------------------------------------------------- le lecteur

fn secondes(t: SystemTime) -> i64 {
    t.duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

fn lister(dossier: &Path, depuis: i64, sortie: &mut Vec<(PathBuf, u64)>) {
    let Ok(entrees) = fs::read_dir(dossier) else { return };
    for e in entrees.flatten() {
        let chemin = e.path();
        let Ok(meta) = e.metadata() else { continue };
        if meta.is_dir() {
            // les sous-agents d'une session ont leurs propres fichiers : ce ne sont pas des conversations
            if chemin.file_name().is_some_and(|n| n == "subagents") {
                continue;
            }
            lister(&chemin, depuis, sortie);
        } else if chemin.extension().is_some_and(|x| x == "jsonl") && meta.modified().map(secondes).unwrap_or(0) >= depuis {
            sortie.push((chemin, meta.len()));
        }
    }
}

/// Relit les transcriptions en gardant en mémoire ce qui a déjà été lu : un fichier qui n'a pas
/// changé n'est pas rouvert, un fichier qui grandit n'est lu qu'à partir de l'endroit où on s'était arrêté.
#[derive(Default)]
pub struct LecteurConversations {
    fichiers: HashMap<PathBuf, EtatFichier>,
}

impl LecteurConversations {
    /// Les conversations de moins de 24 h au dernier message de Claude, la plus récente d'abord.
    pub fn lire(&mut self, racine: &Path, maintenant: i64) -> Lecture {
        let mut liste = Vec::new();
        lister(racine, maintenant - RECENTE_S - 60, &mut liste);
        let presents: Vec<&PathBuf> = liste.iter().map(|(p, _)| p).collect();
        self.fichiers.retain(|p, _| presents.contains(&p));
        let mut lecture = Lecture::default();
        let mut par_session: HashMap<String, Conversation> = HashMap::new();
        for (chemin, taille) in &liste {
            let etat = self.fichiers.entry(chemin.clone()).or_default();
            actualiser(etat, chemin, *taille);
            if etat.tete.planifiee {
                continue;
            }
            // une conversation où Claude n'a pas encore répondu (elle vient de s'ouvrir) n'est pas un candidat :
            // trois conversations neuves ne signifient pas que le format a changé
            if !etat.repondu && etat.dernier.is_none() {
                continue;
            }
            lecture.candidats += 1;
            let Some(d) = &etat.dernier else { continue };
            lecture.lisibles += 1;
            if maintenant - d.t > RECENTE_S {
                continue;
            }
            let session = d
                .session
                .clone()
                .or_else(|| chemin.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .unwrap_or_default();
            let c = Conversation {
                session: session.clone(),
                dossier: d.cwd.as_deref().map(dossier_de_cwd).unwrap_or_else(|| "?".to_string()),
                modele: cle_modele(&d.modele),
                contexte: d.contexte,
                dernier: d.t,
                resumee: etat.resumee,
                titre: etat.titre.clone(),
            };
            // une même session copiée dans deux fichiers : on garde la plus récente
            match par_session.get(&session) {
                Some(autre) if autre.dernier >= c.dernier => {}
                _ => {
                    par_session.insert(session, c);
                }
            }
        }
        let mut conversations: Vec<Conversation> = par_session.into_values().collect();
        conversations.sort_by(|a, b| b.dernier.cmp(&a.dernier).then_with(|| a.session.cmp(&b.session)));
        lecture.conversations = conversations;
        lecture
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_dossier_est_le_dernier_segment() {
        assert_eq!(dossier_de_cwd(r"D:\travail\Conso Token"), "Conso Token");
        assert_eq!(dossier_de_cwd("/home/x/projet-a"), "projet-a");
        assert_eq!(dossier_de_cwd(r"C:\"), "C:");
        assert_eq!(dossier_de_cwd(""), "?");
        assert_eq!(dossier_de_cwd(r"D:\projet-a\"), "projet-a", "barre finale");
    }

    #[test]
    fn l_affichage_retire_ce_qui_retourne_ou_casse_la_ligne() {
        // U+202E (inversion du sens de lecture), isolats et marques : retirés
        assert_eq!(pour_affichage("abc\u{202E}def"), "abcdef");
        assert_eq!(pour_affichage("\u{2066}x\u{2069}\u{200F}y\u{200E}\u{061C}"), "xy");
        // sans largeur : retirés
        assert_eq!(pour_affichage("a\u{200B}b\u{200D}c\u{FEFF}d\u{2060}e"), "abcde");
        // contrôle : une espace (une suite n'en donne qu'une), jamais en tête ni en queue
        assert_eq!(pour_affichage("ligne 1\nligne 2"), "ligne 1 ligne 2");
        assert_eq!(pour_affichage("a\r\n\t\u{1b}\u{0}b"), "a b");
        assert_eq!(pour_affichage("\nhaut\u{2028}bas\u{2029}"), "haut bas");
        // le texte ordinaire ne bouge pas : accents, CJK, emoji, espaces simples
        assert_eq!(pour_affichage("Été  à Paris 東京 😀"), "Été  à Paris 東京 😀");
        // rien que du retirable : rien
        assert_eq!(pour_affichage("\u{202E}\n\u{200B}"), "");
    }

    #[test]
    fn une_copie_de_travail_est_ramenee_au_projet() {
        assert_eq!(dossier_de_cwd(r"D:\projet-a\.claude\worktrees\essai-12"), "projet-a");
        assert_eq!(dossier_de_cwd("/home/x/projet-a/.claude/worktrees/essai-12/sous/dossier"), "projet-a");
        // un dossier qui se contente de s'appeler « worktrees » n'est pas une copie de travail
        assert_eq!(dossier_de_cwd(r"D:\projet-a\worktrees"), "worktrees");
    }

    #[test]
    fn les_noms_de_modele() {
        assert_eq!(cle_modele("claude-opus-5-5"), "claude-opus-5-5");
        assert_eq!(cle_modele("Claude-Sonnet-4-5-20250929"), "claude-sonnet-4-5");
        assert_eq!(cle_modele("claude-opus-5-5[1m]"), "claude-opus-5-5");
        assert_eq!(nom_modele("claude-opus-5-5"), "Opus 5.5");
        assert_eq!(nom_modele("claude-sonnet-5-5"), "Sonnet 5.5");
        assert_eq!(nom_modele("claude-3-5-sonnet"), "Sonnet 3.5");
        assert_eq!(nom_modele("claude-haiku-5-5"), "Haiku 5.5");
        assert_eq!(nom_modele("claude-fable"), "Fable");
        assert_eq!(nom_modele("1234"), "1234", "rien de reconnaissable : le nom brut");
    }
}
