//! La langue de l'appli : huit langues, l'anglais en secours. Le choix vit dans un petit fichier
//! `langue.txt` du dossier de données (comme `vue.txt`) ; à défaut, la langue d'affichage de Windows.
//!
//! Ordre de décision (voir `choisir`) : `CONSO_TOKEN_LANGUE` (essais et captures) > `langue.txt` >
//! langue de Windows > anglais.

use std::cell::Cell;
use std::path::Path;
use std::sync::atomic::{AtomicU8, Ordering};

pub const NB_LANGUES: usize = 8;
const FICHIER_LANGUE: &str = "langue.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Langue {
    En,
    Fr,
    Es,
    De,
    Pt,
    It,
    /// Chinois simplifié.
    Zh,
    Ja,
}

/// Dans l'ordre des colonnes de la table des textes (`textes.rs`) et du sélecteur.
pub const TOUTES: [Langue; NB_LANGUES] = [Langue::En, Langue::Fr, Langue::Es, Langue::De, Langue::Pt, Langue::It, Langue::Zh, Langue::Ja];

impl Langue {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn depuis_index(i: usize) -> Option<Langue> {
        TOUTES.get(i).copied()
    }

    /// Le code écrit dans `langue.txt`.
    pub fn code(self) -> &'static str {
        ["en", "fr", "es", "de", "pt", "it", "zh", "ja"][self.index()]
    }

    /// Le nom de la langue dans la langue elle-même (le sélecteur).
    pub fn nom(self) -> &'static str {
        ["English", "Français", "Español", "Deutsch", "Português", "Italiano", "简体中文", "日本語"][self.index()]
    }

    /// `fr`, `fr-FR`, `pt_BR`, `zh-Hans-CN`, `ZH-TW`… : la langue de base compte, pas la région. Tout
    /// le chinois s'affiche en simplifié (la seule écriture chinoise de l'appli).
    pub fn depuis_code(code: &str) -> Option<Langue> {
        let code = code.trim().to_lowercase();
        let base = code.split(['-', '_']).next().unwrap_or("");
        TOUTES.iter().copied().find(|l| l.code() == base)
    }

    /// Un identifiant de langue de Windows (`LANGID` : 10 bits de langue, 6 bits de région).
    pub fn depuis_langid(id: u16) -> Option<Langue> {
        match id & 0x03ff {
            0x09 => Some(Langue::En),
            0x0c => Some(Langue::Fr),
            0x0a => Some(Langue::Es),
            0x07 => Some(Langue::De),
            0x16 => Some(Langue::Pt),
            0x10 => Some(Langue::It),
            0x04 => Some(Langue::Zh),
            0x11 => Some(Langue::Ja),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------- la langue en cours

/// 255 = pas encore choisie.
static COURANTE: AtomicU8 = AtomicU8::new(255);

thread_local! {
    /// Pour les tests : chaque test tourne dans son fil, il peut donc dire sa langue sans gêner les autres.
    static LOCALE: Cell<Option<Langue>> = const { Cell::new(None) };
}

/// La langue des textes : celle du fil (tests), sinon celle choisie au démarrage, sinon
/// `CONSO_TOKEN_LANGUE` (les tests la fixent à `fr` dans `.cargo/config.toml`), sinon l'anglais.
pub fn courante() -> Langue {
    if let Some(l) = LOCALE.with(Cell::get) {
        return l;
    }
    Langue::depuis_index(COURANTE.load(Ordering::Relaxed) as usize).or_else(depuis_env).unwrap_or(Langue::En)
}

/// Choisit la langue de tout le programme (au démarrage, puis à chaque clic du sélecteur).
pub fn definir(l: Langue) {
    COURANTE.store(l.index() as u8, Ordering::Relaxed);
}

/// Joue `f` dans la langue `l`, pour ce fil seulement (tests).
pub fn avec<T>(l: Langue, f: impl FnOnce() -> T) -> T {
    let avant = LOCALE.with(|c| c.replace(Some(l)));
    let r = f();
    LOCALE.with(|c| c.set(avant));
    r
}

/// `CONSO_TOKEN_LANGUE=de` : force la langue sans rien écrire dans `langue.txt`.
pub fn depuis_env() -> Option<Langue> {
    std::env::var("CONSO_TOKEN_LANGUE").ok().and_then(|v| Langue::depuis_code(&v))
}

// ---------------------------------------------------------------- le fichier et Windows

/// La langue rangée dans le dossier de données ; rien si le fichier manque ou ne dit rien de connu.
pub fn lire_langue(donnees: &Path) -> Option<Langue> {
    let texte = std::fs::read_to_string(donnees.join(FICHIER_LANGUE)).ok()?;
    Langue::depuis_code(&texte)
}

/// Range la langue choisie. Rend faux si le fichier n'a pas pu être écrit (l'appli continue sans).
pub fn ecrire_langue(donnees: &Path, l: Langue) -> bool {
    std::fs::create_dir_all(donnees).is_ok() && std::fs::write(donnees.join(FICHIER_LANGUE), format!("{}\n", l.code())).is_ok()
}

#[cfg(windows)]
unsafe extern "system" {
    /// kernel32 : la langue d'affichage de l'utilisateur. Aucun processus lancé, aucune dépendance en plus.
    safe fn GetUserDefaultUILanguage() -> u16;
}

/// La langue d'affichage de Windows, si l'appli la parle.
pub fn langue_de_windows() -> Option<Langue> {
    #[cfg(windows)]
    {
        Langue::depuis_langid(GetUserDefaultUILanguage())
    }
    #[cfg(not(windows))]
    {
        std::env::var("LANG").ok().and_then(|v| Langue::depuis_code(&v))
    }
}

/// `CONSO_TOKEN_LANGUE` > `langue.txt` > langue de Windows > anglais.
pub fn choisir(donnees: &Path) -> Langue {
    depuis_env().or_else(|| lire_langue(donnees)).or_else(langue_de_windows).unwrap_or(Langue::En)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dossier(nom: &str) -> PathBuf {
        let d = std::env::temp_dir().join("conso_token_langue_tests").join(nom);
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn huit_langues_dans_l_ordre_des_colonnes() {
        assert_eq!(TOUTES.len(), NB_LANGUES);
        for (i, l) in TOUTES.iter().enumerate() {
            assert_eq!(l.index(), i);
            assert_eq!(Langue::depuis_index(i), Some(*l));
            assert_eq!(Langue::depuis_code(l.code()), Some(*l));
            assert!(!l.nom().is_empty());
        }
        assert_eq!(Langue::depuis_index(NB_LANGUES), None);
    }

    #[test]
    fn les_codes_de_windows_et_d_ailleurs() {
        for (code, attendu) in [
            ("fr-FR", Langue::Fr),
            ("fr_CA", Langue::Fr),
            ("EN-us", Langue::En),
            ("pt-BR", Langue::Pt),
            ("pt-PT", Langue::Pt),
            ("zh-CN", Langue::Zh),
            ("zh-Hans-CN", Langue::Zh),
            ("zh-TW", Langue::Zh),
            ("ja-JP", Langue::Ja),
            ("de-AT", Langue::De),
            ("es-MX", Langue::Es),
            ("it", Langue::It),
            (" de \r\n", Langue::De),
        ] {
            assert_eq!(Langue::depuis_code(code), Some(attendu), "code {code:?}");
        }
        for inconnu in ["", "ko-KR", "ru", "n'importe quoi", "frances"] {
            assert_eq!(Langue::depuis_code(inconnu), None, "code {inconnu:?}");
        }
    }

    #[test]
    fn les_identifiants_de_langue_de_windows() {
        for (id, attendu) in [
            (0x0409, Some(Langue::En)), // en-US
            (0x0809, Some(Langue::En)), // en-GB
            (0x040c, Some(Langue::Fr)), // fr-FR
            (0x0c0c, Some(Langue::Fr)), // fr-CA
            (0x0c0a, Some(Langue::Es)), // es-ES
            (0x0407, Some(Langue::De)),
            (0x0416, Some(Langue::Pt)), // pt-BR
            (0x0816, Some(Langue::Pt)), // pt-PT
            (0x0410, Some(Langue::It)),
            (0x0804, Some(Langue::Zh)), // zh-CN
            (0x0404, Some(Langue::Zh)), // zh-TW
            (0x0411, Some(Langue::Ja)),
            (0x0412, None), // ko-KR
            (0x0419, None), // ru-RU
        ] {
            assert_eq!(Langue::depuis_langid(id), attendu, "LANGID {id:#06x}");
        }
    }

    #[test]
    fn le_choix_se_range_et_se_relit() {
        let d = dossier("aller_retour");
        assert_eq!(lire_langue(&d), None, "dossier absent");
        for l in TOUTES {
            assert!(ecrire_langue(&d, l));
            assert_eq!(lire_langue(&d), Some(l));
        }
        // le fichier se lit à la main
        assert_eq!(std::fs::read_to_string(d.join("langue.txt")).unwrap(), "ja\n");
    }

    #[test]
    fn un_fichier_illisible_ou_inconnu_ne_dit_rien() {
        let d = dossier("inconnu");
        std::fs::create_dir_all(&d).unwrap();
        for contenu in ["", "klingon", "42"] {
            std::fs::write(d.join("langue.txt"), contenu).unwrap();
            assert_eq!(lire_langue(&d), None, "contenu {contenu:?}");
        }
        std::fs::write(d.join("langue.txt"), "Deutsch\r\n").unwrap();
        assert_eq!(lire_langue(&d), None, "on écrit le code, pas le nom");
        std::fs::write(d.join("langue.txt"), "DE\r\n").unwrap();
        assert_eq!(lire_langue(&d), Some(Langue::De));
    }

    #[test]
    fn la_langue_d_un_test_ne_deborde_pas() {
        let avant = courante();
        avec(Langue::Ja, || {
            assert_eq!(courante(), Langue::Ja);
            avec(Langue::De, || assert_eq!(courante(), Langue::De));
            assert_eq!(courante(), Langue::Ja, "retour à la langue de l'appel englobant");
        });
        assert_eq!(courante(), avant);
    }
}
