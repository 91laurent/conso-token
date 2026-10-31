//! Le pont : reçoit le JSON que Claude Code envoie à la ligne d'état (sur l'entrée standard),
//! en tire les deux fenêtres de limite et les range dans un petit historique sur le disque.
//!
//! Fichiers, dans le dossier de données :
//! - `dernier.json` : la dernière lecture (réécrit à chaque appel qui apporte des chiffres, sert à
//!   l'« âge » des chiffres ; un appel sans chiffres ne l'efface pas) ;
//! - `historique.jsonl` : une ligne par CHANGEMENT de pourcentage ou de reset.
//!
//! Le pont ne lit ni jeton ni réglages : seulement ce que Claude Code lui envoie.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::modele::{Echantillon, Fenetre};
use crate::temps::meme_reset;

pub const FICHIER_DERNIER: &str = "dernier.json";
pub const FICHIER_HISTORIQUE: &str = "historique.jsonl";

/// Dossier de données : `CONSO_TOKEN_DATA` s'il est défini (tests), sinon
/// `%LOCALAPPDATA%\ConsoToken`.
pub fn dossier_donnees() -> PathBuf {
    if let Some(d) = std::env::var_os("CONSO_TOKEN_DATA") {
        return PathBuf::from(d);
    }
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(|u| PathBuf::from(u).join("AppData").join("Local")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("ConsoToken")
}

#[derive(Deserialize)]
struct Entree {
    rate_limits: Option<Limites>,
}

#[derive(Deserialize)]
struct Limites {
    five_hour: Option<Brut>,
    seven_day: Option<Brut>,
}

#[derive(Deserialize)]
struct Brut {
    used_percentage: Option<f64>,
    resets_at: Option<f64>,
}

fn fenetre(b: Option<Brut>) -> Option<Fenetre> {
    let b = b?;
    let p = b.used_percentage?;
    let r = b.resets_at?;
    if p.is_finite() && p >= 0.0 && r.is_finite() && r > 0.0 { Some(Fenetre { p, r: r as i64 }) } else { None }
}

/// Lit le JSON de la ligne d'état. `None` si ce n'est pas du JSON (entrée vide ou étrangère).
/// Un JSON valide sans fenêtre de limite (compte sans ces chiffres, première requête de la
/// session) donne un échantillon VIDE : il prouve que le branchement marche (`dernier.json`, tant
/// qu'il n'y a pas encore de chiffres) sans rien ajouter à l'historique.
pub fn lire_json(entree: &str, t: i64) -> Option<Echantillon> {
    let e: Entree = serde_json::from_str(entree).ok()?;
    let (cinq, sept) = match e.rate_limits {
        Some(l) => (fenetre(l.five_hour), fenetre(l.seven_day)),
        None => (None, None),
    };
    Some(Echantillon { t, cinq, sept })
}

fn meme_fenetre(a: &Option<Fenetre>, b: &Option<Fenetre>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x.p == y.p && meme_reset(x.r, y.r),
        _ => false,
    }
}

/// Range une lecture : `dernier.json` à chaque lecture qui a des chiffres de 7 jours (ou tant que le
/// dossier est vide), `historique.jsonl` seulement si la fenêtre de 7 jours a changé (la fenêtre de 5 h bouge souvent et n'est utilisée nulle part).
pub fn enregistrer(dossier: &Path, e: &Echantillon) -> io::Result<()> {
    fs::create_dir_all(dossier)?;
    let avant = lire_dernier(dossier);
    // Un échantillon sans fenêtre de 7 jours (session qui n'a pas encore reçu de chiffres) prouve le
    // branchement mais n'a rien à ranger : s'il y a déjà des chiffres, on n'y touche pas. Sinon
    // `dernier.json` les perdrait, l'« âge » des chiffres paraîtrait neuf, et l'appel suivant
    // ajouterait une ligne en double à l'historique.
    if e.sept.is_none() && avant.is_some_and(|a| a.sept.is_some()) {
        return Ok(());
    }
    // Chaque session envoie ses propres chiffres : une session au repos renvoie un pourcentage plus
    // ancien, donc plus bas. On garde alors le chiffre le plus haut de la même semaine, sinon les
    // sessions s'alterneraient et chaque appel ajouterait une ligne.
    let mut garde = *e;
    if let (Some(a), Some(n)) = (avant.and_then(|a| a.sept), e.sept) {
        if meme_reset(a.r, n.r) && n.p < a.p {
            garde.sept = Some(a);
        }
    }
    let change = garde.sept.is_some()
        && match avant {
            Some(a) => !meme_fenetre(&a.sept, &garde.sept),
            None => true,
        };
    let e = &garde;
    if change {
        let mut ligne = serde_json::to_string(e).map_err(io::Error::other)?;
        ligne.push('\n');
        let mut f = OpenOptions::new().create(true).append(true).open(dossier.join(FICHIER_HISTORIQUE))?;
        f.write_all(ligne.as_bytes())?;
    }
    // un nom de fichier temporaire par processus : plusieurs sessions de Claude Code appellent le
    // pont en même temps (au reset de la fenêtre de 5 h, toutes se relancent ensemble)
    let tmp = dossier.join(format!("dernier.{}.tmp", std::process::id()));
    fs::write(&tmp, serde_json::to_string(e).map_err(io::Error::other)?)?;
    fs::rename(&tmp, dossier.join(FICHIER_DERNIER))
}

pub fn lire_dernier(dossier: &Path) -> Option<Echantillon> {
    let s = fs::read_to_string(dossier.join(FICHIER_DERNIER)).ok()?;
    serde_json::from_str(&s).ok()
}

/// Charge l'historique et la dernière lecture. Les lignes illisibles sont ignorées (deux sessions
/// de Claude Code peuvent écrire en même temps).
pub fn charger(dossier: &Path) -> (Vec<Echantillon>, Option<Echantillon>) {
    let historique = fs::read_to_string(dossier.join(FICHIER_HISTORIQUE))
        .map(|s| s.lines().filter_map(|l| serde_json::from_str::<Echantillon>(l).ok()).collect())
        .unwrap_or_default();
    (historique, lire_dernier(dossier))
}
