//! Tokens par jour, tirés des transcriptions de Claude Code (`~/.claude/projects/**/*.jsonl`).
//!
//! GARDE-FOU (notes §9) : on ne lit que l'horodatage et les compteurs `usage` de chaque message,
//! jamais son contenu. Les autres champs sont ignorés par la lecture (`serde` les saute sans les
//! garder). Les fichiers de plus de deux jours avant le début de la semaine ne sont pas ouverts.

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use jiff::Timestamp;
use serde::Deserialize;

use crate::temps::{JOUR_S, SEMAINE_S};

#[derive(Deserialize)]
struct Ligne {
    timestamp: Option<String>,
    #[serde(rename = "requestId")]
    request_id: Option<String>,
    message: Option<Message>,
}

#[derive(Deserialize)]
struct Message {
    id: Option<String>,
    usage: Option<Usage>,
}

#[derive(Deserialize, Clone, Copy, Default)]
struct Usage {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
}

impl Usage {
    /// Tokens comptés : entrée + sortie + création de cache. La lecture du cache, très
    /// fréquente et presque gratuite, n'est PAS comptée (choix à valider).
    fn total(&self) -> u64 {
        self.input_tokens.unwrap_or(0) + self.output_tokens.unwrap_or(0) + self.cache_creation_input_tokens.unwrap_or(0)
    }

    fn maxi(self, autre: Usage) -> Usage {
        let m = |a: Option<u64>, b: Option<u64>| Some(a.unwrap_or(0).max(b.unwrap_or(0)));
        Usage {
            input_tokens: m(self.input_tokens, autre.input_tokens),
            output_tokens: m(self.output_tokens, autre.output_tokens),
            cache_creation_input_tokens: m(self.cache_creation_input_tokens, autre.cache_creation_input_tokens),
        }
    }
}

#[derive(Clone)]
struct Enreg {
    cle: Option<(String, String)>,
    t: i64,
    usage: Usage,
}

struct EtatFichier {
    /// Octets déjà lus (jusqu'à la fin de la dernière ligne complète).
    lu: u64,
    enregs: Vec<Enreg>,
}

/// Lit les transcriptions en gardant en mémoire ce qui a déjà été lu : un fichier qui grandit
/// n'est lu qu'à partir de l'endroit où on s'était arrêté (les transcriptions ne font que grandir).
#[derive(Default)]
pub struct Lecteur {
    fichiers: HashMap<PathBuf, EtatFichier>,
}

fn secondes(t: SystemTime) -> u64 {
    t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn lister(dossier: &Path, depuis: u64, sortie: &mut Vec<(PathBuf, u64, u64)>) {
    let Ok(entrees) = fs::read_dir(dossier) else { return };
    for e in entrees.flatten() {
        let chemin = e.path();
        let Ok(meta) = e.metadata() else { continue };
        if meta.is_dir() {
            lister(&chemin, depuis, sortie);
        } else if chemin.extension().is_some_and(|x| x == "jsonl") {
            let modif = meta.modified().map(secondes).unwrap_or(0);
            if modif >= depuis {
                sortie.push((chemin, modif, meta.len()));
            }
        }
    }
}

/// Une ligne de transcription -> (instant, compteurs), ou `None` si elle n'en porte pas.
fn analyser(ligne: &str) -> Option<Enreg> {
    // préfiltre : seules les lignes qui portent un compteur sont lues
    if !ligne.contains("\"usage\"") {
        return None;
    }
    let l = serde_json::from_str::<Ligne>(ligne).ok()?;
    let (ts, m) = (l.timestamp?, l.message?);
    let usage = m.usage?;
    let t = Timestamp::from_str(&ts).ok()?;
    let cle = match (m.id, l.request_id) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    };
    Some(Enreg { cle, t: t.as_second(), usage })
}

/// Lit `chemin` à partir de l'octet `depuis`, ligne par ligne (un transcript peut peser plusieurs
/// dizaines de Mo). Rend l'octet où reprendre : une dernière ligne encore incomplète (en cours
/// d'écriture) n'est pas consommée, elle sera relue complète à la lecture suivante.
fn lire_depuis(chemin: &Path, depuis: u64, sortie: &mut Vec<Enreg>) -> u64 {
    let Ok(mut fichier) = fs::File::open(chemin) else { return depuis };
    if fichier.seek(SeekFrom::Start(depuis)).is_err() {
        return depuis;
    }
    let mut lecteur = BufReader::new(fichier);
    let mut position = depuis;
    let mut tampon = Vec::new();
    loop {
        tampon.clear();
        let n = match lecteur.read_until(b'\n', &mut tampon) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        let complete = tampon.last() == Some(&b'\n');
        let enreg = std::str::from_utf8(&tampon).ok().and_then(analyser);
        // sans retour à la ligne final : acceptée seulement si elle est déjà un JSON entier
        if !complete && enreg.is_none() {
            break;
        }
        position += n as u64;
        sortie.extend(enreg);
    }
    position
}

impl Lecteur {
    /// Tokens de chacun des sept jours (blocs de 24 h) de la semaine qui commence à `debut`.
    pub fn tokens_par_jour(&mut self, racine: &Path, debut: i64) -> [u64; 7] {
        let depuis = (debut - 2 * JOUR_S).max(0) as u64;
        let mut liste = Vec::new();
        lister(racine, depuis, &mut liste);
        let presents: Vec<PathBuf> = liste.iter().map(|(p, _, _)| p.clone()).collect();
        self.fichiers.retain(|p, _| presents.contains(p));
        for (chemin, _modif, taille) in liste {
            let etat = self.fichiers.entry(chemin.clone()).or_insert(EtatFichier { lu: 0, enregs: Vec::new() });
            if taille < etat.lu {
                // fichier réécrit plus court : on repart du début
                etat.lu = 0;
                etat.enregs.clear();
            }
            if taille > etat.lu {
                etat.lu = lire_depuis(&chemin, etat.lu, &mut etat.enregs);
            }
        }
        agreger(self.fichiers.values().flat_map(|e| e.enregs.iter()), debut)
    }
}

/// Additionne par jour, en comptant une seule fois un même message (même identifiant de message
/// et de requête) même s'il apparaît dans plusieurs fichiers ou plusieurs fois.
fn agreger<'a>(enregs: impl Iterator<Item = &'a Enreg>, debut: i64) -> [u64; 7] {
    let mut par_cle: HashMap<(String, String), (i64, Usage)> = HashMap::new();
    let mut total = [0u64; 7];
    let mut ajouter = |t: i64, u: Usage| {
        let rel = t - debut;
        if (0..SEMAINE_S).contains(&rel) {
            total[(rel / JOUR_S) as usize] += u.total();
        }
    };
    let mut sans_cle = Vec::new();
    for e in enregs {
        match &e.cle {
            Some(c) => {
                par_cle
                    .entry(c.clone())
                    .and_modify(|(t, u)| {
                        *t = (*t).min(e.t);
                        *u = u.maxi(e.usage);
                    })
                    .or_insert((e.t, e.usage));
            }
            None => sans_cle.push((e.t, e.usage)),
        }
    }
    for (t, u) in par_cle.into_values().chain(sans_cle) {
        ajouter(t, u);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(cle: Option<(&str, &str)>, t: i64, entree: u64, sortie: u64, cache: u64) -> Enreg {
        Enreg {
            cle: cle.map(|(a, b)| (a.to_string(), b.to_string())),
            t,
            usage: Usage { input_tokens: Some(entree), output_tokens: Some(sortie), cache_creation_input_tokens: Some(cache) },
        }
    }

    #[test]
    fn range_chaque_message_dans_son_jour() {
        let debut = 1_000_000;
        let v = vec![
            e(Some(("a", "1")), debut + 10, 100, 50, 0),
            e(Some(("b", "2")), debut + JOUR_S + 5, 10, 20, 30),
            e(None, debut + 6 * JOUR_S + 99, 1, 1, 1),
            e(None, debut - 1, 999, 0, 0),            // avant la semaine : ignoré
            e(None, debut + SEMAINE_S, 999, 0, 0),    // après la semaine : ignoré
        ];
        let t = agreger(v.iter(), debut);
        assert_eq!(t, [150, 60, 0, 0, 0, 0, 3]);
    }

    #[test]
    fn un_meme_message_n_est_compte_qu_une_fois() {
        let debut = 0;
        let v = vec![
            e(Some(("a", "1")), 10, 100, 5, 0),
            e(Some(("a", "1")), 12, 100, 50, 0), // même message, compteur final plus grand
            e(Some(("a", "2")), 20, 7, 0, 0),    // autre requête : compté
        ];
        let t = agreger(v.iter(), debut);
        assert_eq!(t[0], 100 + 50 + 7);
    }

    #[test]
    fn la_lecture_du_cache_n_est_pas_comptee() {
        // (le garde-fou « jamais le contenu » tient à la forme des structures `Ligne` et `Message`,
        // qui n'ont aucun champ de contenu : ce test ne le prouve pas, il vérifie ce qui est compté)
        let ligne = r#"{"type":"assistant","timestamp":"2026-10-14T03:48:00.000Z","requestId":"r1","message":{"id":"m1","role":"assistant","content":[{"type":"text","text":"SECRET"}],"usage":{"input_tokens":3,"output_tokens":4,"cache_creation_input_tokens":5,"cache_read_input_tokens":6000}}}"#;
        let e = analyser(ligne).expect("ligne analysable");
        assert_eq!(e.usage.total(), 3 + 4 + 5);
        assert_eq!(e.cle, Some(("m1".to_string(), "r1".to_string())));
        assert!(analyser(r#"{"type":"user","message":{"content":"pas de compteur"}}"#).is_none());
    }
}
