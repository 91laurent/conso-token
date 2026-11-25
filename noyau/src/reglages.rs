//! Les réglages de la page « Conversations » et de l'animation des chiffres. Ils vivent dans un petit
//! fichier texte du dossier de données, `conversations.txt` (comme `vue.txt` et `langue.txt`), jamais
//! dans les réglages de Claude. Une ligne = une clé, `cle=valeur` ; `#` commence un commentaire ; une
//! clé inconnue ou une valeur illisible est ignorée (on garde alors la valeur par défaut), le fichier
//! peut donc se modifier à la main sans rien casser.
//!
//! Défauts publics : UN seul groupe, attention 70 / passation 80, fenêtre « Auto », alerte et
//! animation allumées. Les réglages d'un utilisateur précis (deux groupes, 50 / 60…) ne vivent que
//! dans SON dossier de données.
//!
//! Depuis la 1.3.4, ce fichier garde aussi l'identifiant de session des conversations que la personne a MASQUÉES
//! (`masquee=<identifiant>`, un par ligne) : lui seul, jamais le titre, le nom du dossier ni le contenu. Il est oublié
//! quand la conversation sort de la fenêtre des 24 h, ou quand on clique « Réafficher ».
//!
//! Depuis la 1.3.5, le fichier peut aussi décrire des MAISONS en plus des deux groupes de l'écran (groupes 3 à 8) :
//! `nom3=`, `attention3=`, `passation3=`, `groupe3=<dossier>` (une ligne par dossier) et `titre3=<début de titre>` (une ligne
//! par début). Elles se rédigent à la main ; l'appli les relit, les garde et les réécrit telles quelles (voir `Maison`).
//! Un fichier sans maison se lit et se réécrit exactement comme avant.
//!
//! Un second fichier, `contextes_vus.txt`, garde le plus grand contexte vu pour chaque modèle : il
//! sert à la fenêtre « Auto » (un modèle qui a déjà dépassé 200 000 tokens a une fenêtre d'1 M).

use std::collections::BTreeMap;
use std::path::Path;

pub const FICHIER: &str = "conversations.txt";
pub const FICHIER_VUS: &str = "contextes_vus.txt";
pub const SEUIL_MIN: u8 = 10;
pub const SEUIL_MAX: u8 = 95;
/// Longueur maximale d'un nom de groupe tapé à la main (en caractères).
pub const NOM_MAX: usize = 24;
/// Le plus grand numéro de groupe qu'on peut écrire dans le fichier : les deux groupes de l'écran, plus les maisons 3 à 8.
pub const GROUPE_MAX: usize = 8;
pub const FENETRE_PETITE: u64 = 200_000;
pub const FENETRE_GRANDE: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seuils {
    pub attention: u8,
    pub passation: u8,
}

impl Seuils {
    pub const DEFAUT: Seuils = Seuils { attention: 70, passation: 80 };

    /// Bornes 10 à 95 ; l'attention ne dépasse jamais la passation.
    pub fn normalise(self) -> Seuils {
        let passation = self.passation.clamp(SEUIL_MIN, SEUIL_MAX);
        let attention = self.attention.clamp(SEUIL_MIN, SEUIL_MAX).min(passation);
        Seuils { attention, passation }
    }
}

/// La fenêtre de contexte d'un modèle : choisie à la main, ou devinée (« Auto »).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChoixFenetre {
    #[default]
    Auto,
    Petite,
    Grande,
}

impl ChoixFenetre {
    pub const TOUS: [ChoixFenetre; 3] = [ChoixFenetre::Auto, ChoixFenetre::Petite, ChoixFenetre::Grande];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn depuis_index(i: usize) -> Option<ChoixFenetre> {
        Self::TOUS.get(i).copied()
    }

    fn ecrit(self) -> &'static str {
        match self {
            ChoixFenetre::Auto => "auto",
            ChoixFenetre::Petite => "200000",
            ChoixFenetre::Grande => "1000000",
        }
    }

    fn lu(texte: &str) -> Option<ChoixFenetre> {
        match texte.trim().to_lowercase().replace([' ', '_', '\u{a0}'], "").as_str() {
            "auto" => Some(ChoixFenetre::Auto),
            "200000" | "200k" => Some(ChoixFenetre::Petite),
            "1000000" | "1m" => Some(ChoixFenetre::Grande),
            _ => None,
        }
    }

    /// La fenêtre en tokens : `vu` est le plus grand contexte déjà vu pour ce modèle.
    pub fn tokens(self, vu: u64) -> u64 {
        match self {
            ChoixFenetre::Petite => FENETRE_PETITE,
            ChoixFenetre::Grande => FENETRE_GRANDE,
            ChoixFenetre::Auto => {
                if vu > FENETRE_PETITE {
                    FENETRE_GRANDE
                } else {
                    FENETRE_PETITE
                }
            }
        }
    }
}

/// Une MAISON : un groupe de plus que les deux de l'écran (groupe 3 et suivants), écrit à la main dans `conversations.txt`,
/// avec ses propres seuils. Une conversation y entre par le DÉBUT DE SON TITRE (`titres`) ou, à défaut, par son dossier
/// (`dossiers`) ; le titre l'emporte sur le dossier (voir `suivi::groupe_de`). Aucune maison n'est écrite dans le programme :
/// tout vient du fichier. L'écran Config ne les montre pas ; il ne les modifie pas non plus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Maison {
    /// Le numéro écrit dans le fichier (3 à `GROUPE_MAX`) : `nom3`, `attention3`, `passation3`, `groupe3`, `titre3`.
    pub numero: usize,
    /// Nom tapé à la main ; vide = « Groupe 3 », traduit.
    pub nom: String,
    pub seuils: Seuils,
    /// Les dossiers de travail (dernier segment, comme `groupe1`) qui entrent dans cette maison.
    pub dossiers: Vec<String>,
    /// Les débuts de titre qui font entrer une conversation dans cette maison.
    pub titres: Vec<String>,
}

impl Maison {
    fn vide(numero: usize) -> Maison {
        Maison { numero, nom: String::new(), seuils: Seuils::DEFAUT, dossiers: Vec::new(), titres: Vec::new() }
    }

    /// Le titre d'une conversation (déjà nettoyé pour l'affichage) commence-t-il par l'un des débuts de cette maison ?
    /// Sans tenir compte de la casse, et suivi d'une espace ou de la fin du titre : `Pilote` reconnaît « Pilote 3 » et
    /// « pilote », pas « Pilotes 3 », « Pilote-bis » ni « Le pilote ». Un titre qui se ferait prendre pour une maison à tort
    /// serait lu avec des seuils trop tardifs : on préfère ne pas le reconnaître.
    pub fn reclame_titre(&self, titre: &str) -> bool {
        let titre = titre.trim().to_lowercase();
        self.titres.iter().any(|debut| {
            let debut = debut.trim().to_lowercase();
            !debut.is_empty() && titre.strip_prefix(&debut).is_some_and(|suite| suite.is_empty() || suite.starts_with(char::is_whitespace))
        })
    }

    pub fn reclame_dossier(&self, dossier: &str) -> bool {
        self.dossiers.iter().any(|d| d == dossier)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reglages {
    /// Deux groupes (décoché : un seul bloc « Tous », les réglages des deux groupes sont gardés).
    pub separer: bool,
    /// Noms tapés à la main ; vide = nom par défaut (« Groupe 1 », traduit).
    pub noms: [String; 2],
    pub seuils: [Seuils; 2],
    /// Les dossiers du groupe 1 ; tous les autres sont dans le groupe 2.
    pub groupe1: Vec<String>,
    /// Les maisons ajoutées à la main (groupes 3 et suivants), dans l'ordre de leur numéro. Vide dans les défauts publics.
    pub maisons: Vec<Maison>,
    /// Fenêtre choisie par modèle (clé : identifiant du modèle, voir `conversations::cle_modele`).
    pub fenetres: BTreeMap<String, ChoixFenetre>,
    pub alerte: bool,
    pub animer: bool,
    /// La fenêtre d'accueil de la page Conversations (qui désigne les dossiers du groupe 1) a été vue ou
    /// fermée : elle ne revient pas toute seule (le bouton de Config la rouvre).
    pub accueil_vu: bool,
    /// Les conversations MASQUÉES à la main (le ✕ d'une ligne), reconnues par leur identifiant de session : lui seul est
    /// rangé dans `conversations.txt` (`masquee=<identifiant>`), jamais le titre, le nom du dossier ni le contenu. Elles
    /// restent masquées même si un nouveau message arrive ; l'identifiant est oublié quand la conversation sort de la
    /// fenêtre des 24 h (voir `oublier_masquees_absentes`).
    pub masquees: Vec<String>,
}

impl Default for Reglages {
    fn default() -> Self {
        Reglages {
            separer: false,
            noms: [String::new(), String::new()],
            seuils: [Seuils::DEFAUT; 2],
            groupe1: Vec::new(),
            maisons: Vec::new(),
            fenetres: BTreeMap::new(),
            alerte: true,
            animer: true,
            accueil_vu: false,
            masquees: Vec::new(),
        }
    }
}

/// Longueur maximale (en caractères) d'un identifiant de session rangé dans la liste des masquées.
pub const ID_MAX: usize = 128;

/// Un identifiant de session qu'on peut ranger tel quel dans le fichier : non vide, assez court, une seule ligne sans
/// caractère de contrôle (sinon il pourrait écrire une autre ligne de réglage dans `conversations.txt`).
pub fn id_masquable(id: &str) -> bool {
    let id = id.trim();
    !id.is_empty() && id.chars().count() <= ID_MAX && !id.chars().any(char::is_control)
}

/// Un nom de groupe nettoyé : sans retour à la ligne, rogné, au plus `NOM_MAX` caractères.
pub fn nom_propre(texte: &str) -> String {
    let sans: String = texte.chars().filter(|c| !c.is_control()).collect();
    sans.trim().chars().take(NOM_MAX).collect::<String>().trim_end().to_string()
}

/// Une liste de mots rognés, sans vide ni doublon, dans l'ordre où ils se présentent.
fn liste_propre(liste: Vec<String>) -> Vec<String> {
    let mut vus: Vec<String> = Vec::new();
    for d in liste {
        let d = d.trim().to_string();
        if !d.is_empty() && !vus.contains(&d) {
            vus.push(d);
        }
    }
    vus
}

/// Une clé de maison : `nom3`, `attention4`, `passation5`, `groupe6`, `titre7` → (le champ, le numéro). Seuls les numéros
/// de 3 à `GROUPE_MAX` comptent (1 et 2 sont les groupes de l'écran), écrits sur un seul chiffre.
fn cle_de_maison(cle: &str) -> Option<(&str, usize)> {
    let chiffre = cle.chars().last()?;
    let numero = chiffre.to_digit(10)? as usize;
    let champ = &cle[..cle.len() - chiffre.len_utf8()];
    ((3..=GROUPE_MAX).contains(&numero) && matches!(champ, "nom" | "attention" | "passation" | "groupe" | "titre")).then_some((champ, numero))
}

impl Reglages {
    /// Combien de groupes la page Conversations montre : un seul sans séparation ; avec, les deux de l'écran et les maisons.
    pub fn nb_groupes(&self) -> usize {
        if self.separer { 2 + self.maisons.len() } else { 1 }
    }

    /// Les seuils du groupe `i` (0 et 1 : les deux de l'écran ; 2 et plus : les maisons, dans l'ordre). Un numéro trop grand
    /// donne ceux du groupe 2.
    pub fn seuils_du_groupe(&self, i: usize) -> Seuils {
        match i {
            0 | 1 => self.seuils[i],
            _ => self.maisons.get(i - 2).map_or(self.seuils[1], |m| m.seuils),
        }
    }

    /// Ramène toutes les valeurs dans leurs bornes et nettoie les listes.
    pub fn normalise(mut self) -> Reglages {
        for i in 0..2 {
            self.seuils[i] = self.seuils[i].normalise();
            self.noms[i] = nom_propre(&self.noms[i]);
        }
        self.groupe1 = liste_propre(std::mem::take(&mut self.groupe1));
        // les maisons : numéro entre 3 et GROUPE_MAX, une seule fois, rangées par numéro ; seuils bornés, nom nettoyé
        let mut maisons: Vec<Maison> = Vec::new();
        for mut m in self.maisons.drain(..) {
            if !(3..=GROUPE_MAX).contains(&m.numero) || maisons.iter().any(|x| x.numero == m.numero) {
                continue;
            }
            m.seuils = m.seuils.normalise();
            m.nom = nom_propre(&m.nom);
            m.dossiers = liste_propre(std::mem::take(&mut m.dossiers));
            m.titres = liste_propre(std::mem::take(&mut m.titres));
            maisons.push(m);
        }
        maisons.sort_by_key(|m| m.numero);
        self.maisons = maisons;
        // les conversations masquées : sans doublon, sans vide, sans identifiant qui ne tiendrait pas sur une ligne
        let mut masquees: Vec<String> = Vec::new();
        for m in self.masquees.drain(..) {
            let m = m.trim().to_string();
            if id_masquable(&m) && !masquees.contains(&m) {
                masquees.push(m);
            }
        }
        self.masquees = masquees;
        self
    }

    /// Cette conversation (son identifiant de session) est-elle masquée ?
    pub fn est_masquee(&self, session: &str) -> bool {
        let session = session.trim();
        self.masquees.iter().any(|m| m == session)
    }

    /// Masque une conversation, par son identifiant de session. Rend faux si l'identifiant ne peut pas être rangé
    /// (vide, trop long, caractère de contrôle) : la conversation reste alors affichée.
    pub fn masquer(&mut self, session: &str) -> bool {
        let session = session.trim();
        if !id_masquable(session) {
            return false;
        }
        if !self.masquees.iter().any(|m| m == session) {
            self.masquees.push(session.to_string());
        }
        true
    }

    /// « Réafficher » : toutes les conversations masquées reviennent (la liste est vidée).
    pub fn reafficher_tout(&mut self) {
        self.masquees.clear();
    }

    /// « Rétablir » : les réglages de départ (les maisons écrites à la main s'en vont avec le reste, comme la liste du groupe 1),
    /// mais l'accueil déjà vu ne revient pas et les conversations masquées RESTENT masquées (« Réafficher » est un bouton à
    /// part, dans Config).
    pub fn retablir(&mut self) {
        let (vu, masquees) = (self.accueil_vu, std::mem::take(&mut self.masquees));
        *self = Reglages::default();
        self.accueil_vu = vu;
        self.masquees = masquees;
    }

    /// Oublie les identifiants masqués dont la conversation n'est plus dans la fenêtre des 24 h : `presentes` est la liste
    /// des identifiants de session de la dernière lecture (les conversations de moins de 24 h, masquées comprises).
    /// Rend vrai si la liste a changé. Une liste `presentes` VIDE (aucune lecture encore, rien de lisible, format des
    /// transcriptions incompris : l'appelant ne passe alors rien) n'oublie RIEN : un démarrage à vide ne doit pas effacer
    /// les masquées ; elles s'oublieront à la première lecture qui montre des conversations.
    pub fn oublier_masquees_absentes(&mut self, presentes: &[&str]) -> bool {
        if presentes.is_empty() {
            return false;
        }
        let avant = self.masquees.len();
        self.masquees.retain(|m| presentes.contains(&m.as_str()));
        self.masquees.len() != avant
    }

    pub fn choix_fenetre(&self, modele: &str) -> ChoixFenetre {
        self.fenetres.get(modele).copied().unwrap_or_default()
    }

    /// Fixe la fenêtre d'un modèle, désigné par sa CLÉ (jamais par sa place dans une liste qui peut changer
    /// entre l'affichage et le clic). « Auto » est le défaut : il n'est pas rangé.
    pub fn fixer_fenetre(&mut self, modele: &str, choix: ChoixFenetre) {
        if choix == ChoixFenetre::Auto {
            self.fenetres.remove(modele);
        } else {
            self.fenetres.insert(modele.to_string(), choix);
        }
    }

    /// Change le camp d'un dossier (le clic sur l'étiquette) : du groupe 1 vers le 2, ou l'inverse.
    pub fn changer_de_camp(&mut self, dossier: &str) {
        if let Some(i) = self.groupe1.iter().position(|d| d == dossier) {
            self.groupe1.remove(i);
        } else {
            self.groupe1.push(dossier.to_string());
        }
    }

    /// Applique le choix de la fenêtre « Choisir les groupes… », qu'on peut rouvrir à tout moment et autant de fois qu'on veut
    /// (depuis la page Conversations ou depuis Config). `separer` vrai : deux groupes, les dossiers `coches` forment le groupe 1
    /// (aucun coché : tout est dans le groupe 2), avec les `noms` tapés. `separer` faux : un seul groupe, et RIEN n'est perdu :
    /// la liste du groupe 1 et les noms restent rangés tels quels (dans `conversations.txt`) et reviennent au prochain choix
    /// « deux groupes ». Dans les deux cas la fenêtre est « vue » : elle ne s'ouvre plus toute seule.
    pub fn appliquer_choix_groupes(&mut self, separer: bool, coches: Vec<String>, noms: [String; 2]) {
        self.accueil_vu = true;
        self.separer = separer;
        if separer {
            self.groupe1 = coches;
            self.noms = noms;
        }
    }

    pub fn en_texte(&self) -> String {
        let mut s = String::from("# Conso Token : réglages de la page Conversations (modifiable à la main)\n");
        let oui = |b: bool| if b { "1" } else { "0" };
        s.push_str(&format!("separer={}\n", oui(self.separer)));
        for i in 0..2 {
            s.push_str(&format!("nom{}={}\n", i + 1, self.noms[i]));
            s.push_str(&format!("attention{}={}\n", i + 1, self.seuils[i].attention));
            s.push_str(&format!("passation{}={}\n", i + 1, self.seuils[i].passation));
        }
        for d in &self.groupe1 {
            s.push_str(&format!("groupe1={d}\n"));
        }
        // les maisons ajoutées à la main (rien du tout sans maison : le fichier reste ce qu'il était avant la 1.3.5)
        for m in &self.maisons {
            let n = m.numero;
            s.push_str(&format!("nom{n}={}\n", m.nom));
            s.push_str(&format!("attention{n}={}\n", m.seuils.attention));
            s.push_str(&format!("passation{n}={}\n", m.seuils.passation));
            for d in &m.dossiers {
                s.push_str(&format!("groupe{n}={d}\n"));
            }
            for t in &m.titres {
                s.push_str(&format!("titre{n}={t}\n"));
            }
        }
        for (modele, choix) in &self.fenetres {
            s.push_str(&format!("fenetre.{modele}={}\n", choix.ecrit()));
        }
        // identifiants de session SEULS (ni titre, ni nom de dossier, ni contenu)
        for m in &self.masquees {
            s.push_str(&format!("masquee={m}\n"));
        }
        s.push_str(&format!("alerte={}\n", oui(self.alerte)));
        s.push_str(&format!("animer={}\n", oui(self.animer)));
        s.push_str(&format!("accueil={}\n", oui(self.accueil_vu)));
        s
    }

    pub fn depuis_texte(texte: &str) -> Reglages {
        let mut r = Reglages::default();
        let mut groupe1 = Vec::new();
        let mut masquees = Vec::new();
        let mut maisons: BTreeMap<usize, Maison> = BTreeMap::new();
        let booleen = |v: &str| match v.trim().to_lowercase().as_str() {
            "1" | "oui" | "true" | "yes" | "on" => Some(true),
            "0" | "non" | "false" | "no" | "off" => Some(false),
            _ => None,
        };
        for ligne in texte.lines() {
            let ligne = ligne.trim_start_matches('\u{feff}').trim();
            if ligne.is_empty() || ligne.starts_with('#') {
                continue;
            }
            let Some((cle, valeur)) = ligne.split_once('=') else { continue };
            let (cle, valeur) = (cle.trim(), valeur.trim());
            let seuil = |v: &str| v.parse::<u8>().ok();
            match cle {
                "separer" => r.separer = booleen(valeur).unwrap_or(r.separer),
                "alerte" => r.alerte = booleen(valeur).unwrap_or(r.alerte),
                "animer" => r.animer = booleen(valeur).unwrap_or(r.animer),
                "accueil" => r.accueil_vu = booleen(valeur).unwrap_or(r.accueil_vu),
                "nom1" => r.noms[0] = valeur.to_string(),
                "nom2" => r.noms[1] = valeur.to_string(),
                "attention1" => r.seuils[0].attention = seuil(valeur).unwrap_or(r.seuils[0].attention),
                "passation1" => r.seuils[0].passation = seuil(valeur).unwrap_or(r.seuils[0].passation),
                "attention2" => r.seuils[1].attention = seuil(valeur).unwrap_or(r.seuils[1].attention),
                "passation2" => r.seuils[1].passation = seuil(valeur).unwrap_or(r.seuils[1].passation),
                "groupe1" => groupe1.push(valeur.to_string()),
                "masquee" => masquees.push(valeur.to_string()),
                _ => {
                    if let Some(modele) = cle.strip_prefix("fenetre.") {
                        if let Some(c) = ChoixFenetre::lu(valeur) {
                            r.fenetres.insert(modele.trim().to_string(), c);
                        }
                    } else if let Some((champ, numero)) = cle_de_maison(cle) {
                        let m = maisons.entry(numero).or_insert_with(|| Maison::vide(numero));
                        match champ {
                            "nom" => m.nom = valeur.to_string(),
                            "attention" => m.seuils.attention = seuil(valeur).unwrap_or(m.seuils.attention),
                            "passation" => m.seuils.passation = seuil(valeur).unwrap_or(m.seuils.passation),
                            "groupe" => m.dossiers.push(valeur.to_string()),
                            _ => m.titres.push(valeur.to_string()),
                        }
                    }
                }
            }
        }
        r.groupe1 = groupe1;
        r.maisons = maisons.into_values().collect();
        r.masquees = masquees;
        r.normalise()
    }
}

/// Les réglages rangés dans le dossier de données ; les défauts si le fichier manque.
pub fn lire(donnees: &Path) -> Reglages {
    match std::fs::read_to_string(donnees.join(FICHIER)) {
        Ok(t) => Reglages::depuis_texte(&t),
        Err(_) => Reglages::default(),
    }
}

/// Range les réglages. Rend faux si le fichier n'a pas pu être écrit (l'appli continue sans).
pub fn ecrire(donnees: &Path, r: &Reglages) -> bool {
    std::fs::create_dir_all(donnees).is_ok() && std::fs::write(donnees.join(FICHIER), r.clone().normalise().en_texte()).is_ok()
}

// ---------------------------------------------------------------- les contextes vus

/// Le plus grand contexte vu pour chaque modèle (clé : identifiant du modèle).
pub fn lire_vus(donnees: &Path) -> BTreeMap<String, u64> {
    let mut m = BTreeMap::new();
    let Ok(texte) = std::fs::read_to_string(donnees.join(FICHIER_VUS)) else { return m };
    for ligne in texte.lines() {
        if let Some((cle, v)) = ligne.trim().split_once('=') {
            if let Ok(n) = v.trim().parse::<u64>() {
                if !cle.trim().is_empty() {
                    m.insert(cle.trim().to_string(), n);
                }
            }
        }
    }
    m
}

pub fn ecrire_vus(donnees: &Path, vus: &BTreeMap<String, u64>) -> bool {
    let mut s = String::new();
    for (modele, n) in vus {
        s.push_str(&format!("{modele}={n}\n"));
    }
    std::fs::create_dir_all(donnees).is_ok() && std::fs::write(donnees.join(FICHIER_VUS), s).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dossier(nom: &str) -> PathBuf {
        let d = std::env::temp_dir().join("conso_token_reglages_tests").join(nom);
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    fn deux_groupes() -> Reglages {
        Reglages {
            separer: true,
            noms: ["Leads".into(), "Fils".into()],
            seuils: [Seuils { attention: 50, passation: 60 }, Seuils { attention: 40, passation: 50 }],
            groupe1: vec!["projet-a".into(), "projet-b".into()],
            maisons: Vec::new(),
            fenetres: BTreeMap::from([("claude-opus-5-5".to_string(), ChoixFenetre::Grande), ("claude-haiku-5-5".to_string(), ChoixFenetre::Petite)]),
            alerte: false,
            animer: false,
            accueil_vu: true,
            masquees: vec!["3f9c01aa-1111".into(), "91ab02bb-2222".into()],
        }
    }

    #[test]
    fn l_accueil_se_range_et_ne_revient_pas_tout_seul() {
        // au départ il n'a pas été vu ; une fois vu, il se range et se relit
        assert!(!Reglages::default().accueil_vu);
        assert!(!Reglages::depuis_texte("separer=1\nnom1=Leads\n").accueil_vu, "un fichier sans la clé : à montrer");
        let d = dossier("accueil");
        let mut r = Reglages::default();
        r.accueil_vu = true;
        assert!(ecrire(&d, &r));
        assert!(std::fs::read_to_string(d.join("conversations.txt")).unwrap().contains("accueil=1"));
        assert!(lire(&d).accueil_vu);
        // une valeur illisible garde le défaut
        assert!(!Reglages::depuis_texte("accueil=peut-etre\n").accueil_vu);
    }

    #[test]
    fn les_defauts_publics() {
        let r = Reglages::default();
        assert!(!r.separer, "un seul groupe");
        assert_eq!(r.seuils, [Seuils { attention: 70, passation: 80 }; 2]);
        assert!(r.alerte && r.animer);
        assert!(r.fenetres.is_empty(), "fenêtre Auto partout");
        assert_eq!(r.choix_fenetre("claude-opus-5-5"), ChoixFenetre::Auto);
        // dossier absent : les défauts, rien n'est créé
        assert_eq!(lire(&dossier("absent")), Reglages::default());
    }

    #[test]
    fn aller_retour_dans_le_fichier() {
        let d = dossier("aller_retour");
        let r = deux_groupes();
        assert!(ecrire(&d, &r));
        assert_eq!(lire(&d), r);
        // le fichier se lit et se corrige à la main
        let texte = std::fs::read_to_string(d.join("conversations.txt")).unwrap();
        assert!(texte.contains("passation1=60"), "{texte}");
        assert!(texte.contains("fenetre.claude-opus-5-5=1000000"), "{texte}");
    }

    #[test]
    fn un_fichier_abime_garde_les_defauts() {
        let r = Reglages::depuis_texte("n'importe quoi\nseparer=peut-etre\nattention1=abc\npassation1=999\n\n=\nfenetre.x=42\n# commentaire\n");
        let defaut = Reglages::default();
        assert_eq!(r.separer, defaut.separer);
        assert_eq!(r.seuils[0].attention, 70);
        // 999 ne tient pas dans un octet : ignoré ; 42 n'est pas une fenêtre connue
        assert_eq!(r.seuils[0].passation, 80);
        assert!(r.fenetres.is_empty());
        assert_eq!(Reglages::depuis_texte(""), defaut);
    }

    #[test]
    fn les_bornes_sont_tenues() {
        let r = Reglages::depuis_texte("attention1=5\npassation1=99\nattention2=90\npassation2=30\n");
        assert_eq!(r.seuils[0], Seuils { attention: 10, passation: 95 });
        // l'attention ne dépasse jamais la passation
        assert_eq!(r.seuils[1], Seuils { attention: 30, passation: 30 });
        assert_eq!(Seuils { attention: 60, passation: 50 }.normalise(), Seuils { attention: 50, passation: 50 });
    }

    #[test]
    fn les_noms_tapes_a_la_main_sont_nettoyes() {
        assert_eq!(nom_propre("  Leads\r\n"), "Leads");
        assert_eq!(nom_propre("a\nb=c"), "ab=c");
        assert_eq!(nom_propre(&"é".repeat(40)).chars().count(), NOM_MAX);
        // un nom avec « = » survit à l'aller-retour
        let mut r = Reglages::default();
        r.noms[0] = "a=b".into();
        assert_eq!(Reglages::depuis_texte(&r.en_texte()).noms[0], "a=b");
    }

    #[test]
    fn le_clic_sur_l_etiquette_change_le_camp_du_dossier() {
        let mut r = Reglages::default();
        r.changer_de_camp("projet-x");
        assert_eq!(r.groupe1, vec!["projet-x".to_string()]);
        r.changer_de_camp("projet-x");
        assert!(r.groupe1.is_empty());
        // des doublons écrits à la main disparaissent
        assert_eq!(Reglages::depuis_texte("groupe1=a\ngroupe1=a\ngroupe1= b \n").groupe1, vec!["a".to_string(), "b".to_string()]);
    }

    fn texte(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn le_choix_des_groupes_se_refait_autant_de_fois_qu_on_veut() {
        let d = dossier("choix_groupes");
        let noms = |a: &str, b: &str| [a.to_string(), b.to_string()];
        let mut r = Reglages::default();
        // 1) deux groupes, deux dossiers dans le premier, des noms tapés
        r.appliquer_choix_groupes(true, texte(&["projet-a", "projet-b"]), noms("Leads", "Fils"));
        assert!(r.separer && r.accueil_vu);
        assert_eq!((r.groupe1.clone(), r.noms.clone()), (texte(&["projet-a", "projet-b"]), noms("Leads", "Fils")));
        // 2) retour à UN seul groupe : rien n'est perdu (la liste et les noms restent rangés)
        r.appliquer_choix_groupes(false, texte(&["ignore"]), noms("ignore", "ignore"));
        assert!(!r.separer, "un seul groupe");
        assert_eq!((r.groupe1.clone(), r.noms.clone()), (texte(&["projet-a", "projet-b"]), noms("Leads", "Fils")), "l'ancien choix reste");
        // ... aussi dans le fichier : il se relit tel quel
        assert!(ecrire(&d, &r));
        let relu = lire(&d);
        assert!(!relu.separer);
        assert_eq!((relu.groupe1.clone(), relu.noms.clone()), (texte(&["projet-a", "projet-b"]), noms("Leads", "Fils")));
        // 3) de nouveau deux groupes, la liste rouverte reprend ce qui était rangé, on en retire un
        let mut r = relu;
        r.appliquer_choix_groupes(true, texte(&["projet-b"]), noms("Pilotes", ""));
        assert!(r.separer);
        assert_eq!((r.groupe1.clone(), r.noms.clone()), (texte(&["projet-b"]), noms("Pilotes", "")));
        // 4) TOUT décocher : deux groupes, le premier vide (tout est dans le second)
        r.appliquer_choix_groupes(true, Vec::new(), noms("", ""));
        assert!(r.separer && r.groupe1.is_empty());
        // le fichier d'un état « tout décoché » se relit sans dossier
        assert!(ecrire(&d, &r));
        let relu = lire(&d);
        assert!(relu.separer && relu.groupe1.is_empty());
        // les réglages qui n'ont rien à voir (seuils, fenêtres, alerte) ne bougent jamais dans tout cela
        let avant = deux_groupes();
        let mut apres = avant.clone();
        apres.appliquer_choix_groupes(false, Vec::new(), noms("", ""));
        apres.appliquer_choix_groupes(true, texte(&["projet-a", "projet-b"]), noms("Leads", "Fils"));
        assert_eq!(apres, avant);
    }

    #[test]
    fn les_conversations_masquees_se_rangent_par_identifiant_seul() {
        let d = dossier("masquees");
        let mut r = Reglages::default();
        assert!(r.masquees.is_empty(), "au départ : rien de masqué");
        assert!(r.masquer("3f9c01aa-1111"));
        assert!(r.masquer("  91ab02bb-2222 "), "l'identifiant est rogné");
        assert!(r.masquer("3f9c01aa-1111"), "masquer deux fois : une seule ligne");
        assert_eq!(r.masquees, ["3f9c01aa-1111", "91ab02bb-2222"]);
        assert!(r.est_masquee("3f9c01aa-1111") && r.est_masquee(" 91ab02bb-2222") && !r.est_masquee("autre") && !r.est_masquee(""));
        assert!(ecrire(&d, &r));
        let texte = std::fs::read_to_string(d.join("conversations.txt")).unwrap();
        assert!(texte.contains("masquee=3f9c01aa-1111\n") && texte.contains("masquee=91ab02bb-2222\n"), "{texte}");
        assert_eq!(texte.lines().filter(|l| l.starts_with("masquee=")).count(), 2, "{texte}");
        // ce qui se relit est ce qui a été masqué, dans l'ordre
        assert_eq!(lire(&d).masquees, r.masquees);
        // sans conversation masquée : aucune ligne `masquee=` (le fichier ne parle de conversations que si on en a masqué)
        assert!(!Reglages::default().en_texte().contains("masquee"));
    }

    #[test]
    fn un_identifiant_qui_ne_tient_pas_sur_une_ligne_n_est_pas_range() {
        let mut r = Reglages::default();
        assert!(!r.masquer(""));
        assert!(!r.masquer("   "));
        assert!(!r.masquer("abc\nseparer=1"), "un retour à la ligne écrirait un autre réglage");
        assert!(!r.masquer("abc\u{7}def"));
        assert!(!r.masquer(&"x".repeat(ID_MAX + 1)));
        assert!(r.masquees.is_empty() && !r.en_texte().contains("masquee"));
        assert!(r.masquer(&"x".repeat(ID_MAX)), "la limite elle-même passe");
        // un fichier abîmé ou écrit à la main : les lignes illisibles sont ignorées sans erreur, les bonnes gardées
        let r = Reglages::depuis_texte("masquee=\nmasquee=a\nmasquee=a\nmasquee= b \nmasquee=x\u{7}y\nmasquee\nmasquee=c=d\nmasqueee=z\n# masquee=commentaire\n");
        assert_eq!(r.masquees, ["a", "b", "c=d"]);
        // un identifiant qui contient « = » fait l'aller-retour
        assert_eq!(Reglages::depuis_texte(&r.en_texte()).masquees, ["a", "b", "c=d"]);
        // un fichier sans la clé : rien n'est masqué
        assert!(Reglages::depuis_texte("separer=1\n").masquees.is_empty());
    }

    #[test]
    fn l_oubli_des_masquees_hors_de_la_fenetre_des_24_h() {
        let d = dossier("oubli");
        let mut r = Reglages::default();
        for id in ["a", "b", "c"] {
            r.masquer(id);
        }
        // aucune lecture (ou rien de lisible) : on ne sait rien, on n'oublie RIEN
        assert!(!r.oublier_masquees_absentes(&[]));
        assert_eq!(r.masquees, ["a", "b", "c"]);
        // une lecture montre « a » et « z » : « b » et « c » sont sortis de la fenêtre, ils sont oubliés ; « z » n'est pas masqué
        assert!(r.oublier_masquees_absentes(&["a", "z"]));
        assert_eq!(r.masquees, ["a"]);
        assert!(!r.oublier_masquees_absentes(&["a", "z"]), "rien de plus à oublier");
        // le fichier ne grossit pas : seul « a » y reste
        assert!(ecrire(&d, &r));
        let texte = std::fs::read_to_string(d.join("conversations.txt")).unwrap();
        assert_eq!(texte.lines().filter(|l| l.starts_with("masquee=")).collect::<Vec<_>>(), ["masquee=a"]);
        // la dernière conversation masquée sort de la fenêtre alors que d'autres sont lues : plus rien n'est masqué
        assert!(r.oublier_masquees_absentes(&["z"]));
        assert!(r.masquees.is_empty());
    }

    #[test]
    fn reafficher_vide_la_liste_et_retablir_n_y_touche_pas() {
        let d = dossier("reafficher");
        // « Rétablir » : les réglages de départ, l'accueil vu et les masquées restent
        let mut r = deux_groupes();
        r.retablir();
        assert_eq!(r.masquees, ["3f9c01aa-1111", "91ab02bb-2222"], "Rétablir ne réaffiche rien");
        assert!(r.accueil_vu, "l'accueil déjà vu ne revient pas");
        assert!(!r.separer && r.groupe1.is_empty() && r.fenetres.is_empty() && r.alerte && r.animer, "le reste est remis au départ");
        assert_eq!(r.seuils, [Seuils::DEFAUT; 2]);
        // « Réafficher » : la liste est vidée (et le fichier la perd), le reste ne bouge pas
        let mut r = deux_groupes();
        r.reafficher_tout();
        assert!(r.masquees.is_empty());
        let mut attendu = deux_groupes();
        attendu.masquees.clear();
        assert_eq!(r, attendu, "Réafficher ne touche à aucun autre réglage");
        assert!(ecrire(&d, &r));
        assert!(!std::fs::read_to_string(d.join("conversations.txt")).unwrap().contains("masquee"));
        assert!(lire(&d).masquees.is_empty());
    }

    #[test]
    fn la_fenetre_se_fixe_par_la_cle_du_modele() {
        let mut r = Reglages::default();
        r.fixer_fenetre("claude-opus-5-5", ChoixFenetre::Grande);
        r.fixer_fenetre("claude-haiku-5-5", ChoixFenetre::Petite);
        assert_eq!(r.choix_fenetre("claude-opus-5-5"), ChoixFenetre::Grande);
        assert_eq!(r.choix_fenetre("claude-haiku-5-5"), ChoixFenetre::Petite);
        assert_eq!(r.choix_fenetre("claude-sonnet-5-5"), ChoixFenetre::Auto, "un autre modèle n'est pas touché");
        // changer l'un ne touche pas l'autre ; « Auto » efface le choix (le défaut n'est pas rangé)
        r.fixer_fenetre("claude-opus-5-5", ChoixFenetre::Petite);
        assert_eq!(r.choix_fenetre("claude-opus-5-5"), ChoixFenetre::Petite);
        assert_eq!(r.choix_fenetre("claude-haiku-5-5"), ChoixFenetre::Petite);
        r.fixer_fenetre("claude-opus-5-5", ChoixFenetre::Auto);
        assert!(!r.fenetres.contains_key("claude-opus-5-5"));
        assert_eq!(r.fenetres.len(), 1);
        // une clé inconnue ne fait rien d'autre qu'être rangée ; Auto sur une clé absente ne casse rien
        r.fixer_fenetre("claude-inconnu", ChoixFenetre::Auto);
        assert_eq!(r.fenetres.len(), 1);
    }

    #[test]
    fn la_fenetre_auto_suit_le_plus_grand_contexte_vu() {
        assert_eq!(ChoixFenetre::Auto.tokens(0), 200_000);
        assert_eq!(ChoixFenetre::Auto.tokens(200_000), 200_000, "pile 200 000 ne dépasse pas");
        assert_eq!(ChoixFenetre::Auto.tokens(200_001), 1_000_000);
        assert_eq!(ChoixFenetre::Auto.tokens(648_622), 1_000_000);
        assert_eq!(ChoixFenetre::Petite.tokens(900_000), 200_000, "le choix à la main prime");
        assert_eq!(ChoixFenetre::Grande.tokens(0), 1_000_000);
        assert_eq!(ChoixFenetre::lu("200 000"), Some(ChoixFenetre::Petite));
        assert_eq!(ChoixFenetre::lu("1 000 000"), Some(ChoixFenetre::Grande));
        assert_eq!(ChoixFenetre::lu("Auto"), Some(ChoixFenetre::Auto));
        assert_eq!(ChoixFenetre::lu("500000"), None);
    }

    #[test]
    fn les_contextes_vus_se_rangent() {
        let d = dossier("vus");
        assert!(lire_vus(&d).is_empty());
        let m = BTreeMap::from([("claude-opus-5-5".to_string(), 648_622u64), ("claude-sonnet-5-5".to_string(), 625_696)]);
        assert!(ecrire_vus(&d, &m));
        assert_eq!(lire_vus(&d), m);
        std::fs::write(d.join("contextes_vus.txt"), "abc\nclaude-x=oups\n=5\nclaude-y=7\n").unwrap();
        assert_eq!(lire_vus(&d), BTreeMap::from([("claude-y".to_string(), 7)]));
    }

    // ------------------------------------------------------------ les maisons (1.3.5)

    /// Le fichier d'un utilisateur d'avant la 1.3.5 : lu comme avant (aucune maison), réécrit à l'identique, octet pour octet.
    #[test]
    fn un_fichier_d_avant_la_1_3_5_se_relit_et_se_reecrit_a_l_identique() {
        let ancien = "# Conso Token : réglages de la page Conversations (modifiable à la main)\nseparer=1\nnom1=Leads\nattention1=50\npassation1=55\n\
nom2=Fils\nattention2=40\npassation2=45\ngroupe1=projet-a\ngroupe1=projet-b\nfenetre.claude-opus-5-5=1000000\nmasquee=aaaa-1111\n\
masquee=bbbb-2222\nalerte=1\nanimer=1\naccueil=1\n";
        let r = Reglages::depuis_texte(ancien);
        assert!(r.maisons.is_empty());
        assert_eq!(r.nb_groupes(), 2);
        assert_eq!(r.en_texte(), ancien, "aucune ligne de maison n'apparaît quand il n'y en a pas");
        // et les défauts publics n'en ont pas non plus
        assert!(Reglages::default().maisons.is_empty());
        assert!(!Reglages::default().en_texte().contains("titre"));
    }

    #[test]
    fn les_maisons_se_lisent_et_font_l_aller_retour() {
        // la maison 4 est écrite AVANT la maison 3 : elles sont rangées par numéro
        let texte = "separer=1\nnom1=Leads\nattention1=50\npassation1=55\nnom2=Fils\nattention2=40\npassation2=45\ngroupe1=d1\n\
nom4=Quatre\nattention4=40\npassation4=45\ngroupe4=d4\ngroupe4=d5\ntitre4=Fil\n\
nom3=Trois\nattention3=50\npassation3=55\ntitre3=Chef\ntitre3=Chef adjoint\n";
        let r = Reglages::depuis_texte(texte);
        assert_eq!(r.maisons.iter().map(|m| m.numero).collect::<Vec<_>>(), [3, 4]);
        let (m3, m4) = (&r.maisons[0], &r.maisons[1]);
        assert_eq!((m3.nom.as_str(), m3.seuils), ("Trois", Seuils { attention: 50, passation: 55 }));
        assert_eq!((m3.titres.clone(), m3.dossiers.clone()), (vec!["Chef".to_string(), "Chef adjoint".to_string()], Vec::<String>::new()));
        assert_eq!((m4.nom.as_str(), m4.seuils), ("Quatre", Seuils { attention: 40, passation: 45 }));
        assert_eq!((m4.titres.clone(), m4.dossiers.clone()), (vec!["Fil".to_string()], vec!["d4".to_string(), "d5".to_string()]));
        // les groupes de l'écran ne bougent pas
        assert_eq!((r.groupe1.clone(), r.seuils, r.noms.clone()), (vec!["d1".to_string()], [Seuils { attention: 50, passation: 55 }, Seuils { attention: 40, passation: 45 }], ["Leads".to_string(), "Fils".to_string()]));
        // le nombre de groupes et les seuils de chacun : 0 et 1 sont ceux de l'écran, 2 et 3 ceux des maisons
        assert_eq!(r.nb_groupes(), 4);
        assert_eq!((r.seuils_du_groupe(0), r.seuils_du_groupe(1)), (r.seuils[0], r.seuils[1]));
        assert_eq!((r.seuils_du_groupe(2), r.seuils_du_groupe(3)), (m3.seuils, m4.seuils));
        assert_eq!(r.seuils_du_groupe(9), r.seuils[1], "un numéro trop grand : les seuils du groupe 2");
        // aller-retour par le texte, puis par le fichier : rien ne se perd quand l'appli réécrit le fichier
        assert_eq!(Reglages::depuis_texte(&r.en_texte()), r);
        let d = dossier("maisons");
        assert!(ecrire(&d, &r));
        assert_eq!(lire(&d), r);
        let ecrit = std::fs::read_to_string(d.join("conversations.txt")).unwrap();
        for ligne in ["nom3=Trois\n", "passation3=55\n", "titre3=Chef adjoint\n", "groupe4=d5\n", "titre4=Fil\n"] {
            assert!(ecrit.contains(ligne), "{ligne} manque : {ecrit}");
        }
        // un autre réglage qui change (masquer, fenêtre) réécrit le fichier : les maisons y restent
        let mut autre = r.clone();
        autre.masquer("zz");
        autre.fixer_fenetre("claude-opus-5-5", ChoixFenetre::Grande);
        assert_eq!(Reglages::depuis_texte(&autre.en_texte()).maisons, r.maisons);
        // sans séparation : un seul groupe montré, mais les maisons restent rangées (elles reviennent avec « deux groupes »)
        let mut seul = r.clone();
        seul.appliquer_choix_groupes(false, Vec::new(), [String::new(), String::new()]);
        assert_eq!(seul.nb_groupes(), 1);
        assert_eq!(Reglages::depuis_texte(&seul.en_texte()).maisons, r.maisons);
        // « Rétablir » remet les réglages de départ : les maisons s'en vont avec le reste
        let mut rien = r.clone();
        rien.retablir();
        assert!(rien.maisons.is_empty());
    }

    #[test]
    fn les_cles_de_maison_hors_numero_sont_ignorees_et_les_valeurs_abimees_gardent_le_defaut() {
        let r = Reglages::depuis_texte(
            "groupe2=zzz\ntitre1=t\ntitre2=u\nnom9=neuf\nnom0=zero\nnom10=dix\nnom3x=q\nnouveau3=q\ntitre=t\n\
attention3=5\npassation3=99\nattention5=90\npassation5=30\nattention6=abc\npassation6=999\n\
groupe7=\ngroupe7= a \ngroupe7=a\ntitre7=\ntitre7= Pilote \ntitre7=Pilote\nnom8=e\u{7}é\n",
        );
        // ni les numéros 0, 1, 2, 9 ou 10, ni un champ inconnu, ni une clé sans numéro ne créent de groupe ou ne touchent aux deux de l'écran
        assert_eq!(r.maisons.iter().map(|m| m.numero).collect::<Vec<_>>(), [3, 5, 6, 7, 8]);
        assert!(r.groupe1.is_empty() && r.noms == [String::new(), String::new()]);
        let seuils = |n: usize| r.maisons.iter().find(|m| m.numero == n).unwrap().seuils;
        assert_eq!(seuils(3), Seuils { attention: 10, passation: 95 }, "bornes 10 à 95");
        assert_eq!(seuils(5), Seuils { attention: 30, passation: 30 }, "l'attention ne dépasse pas la passation");
        assert_eq!(seuils(6), Seuils::DEFAUT, "une valeur illisible garde le défaut");
        let m7 = r.maisons.iter().find(|m| m.numero == 7).unwrap();
        assert_eq!((m7.dossiers.clone(), m7.titres.clone()), (vec!["a".to_string()], vec!["Pilote".to_string()]), "rognés, sans vide ni doublon");
        assert_eq!(r.maisons.iter().find(|m| m.numero == 8).unwrap().nom, "eé", "un nom est nettoyé comme les autres");
        // une maison construite à la main avec un numéro qui ne va pas, ou deux fois le même : écartée à la normalisation
        let faux = Reglages { maisons: vec![Maison::vide(2), Maison::vide(9), Maison::vide(4), Maison::vide(4)], ..Reglages::default() }.normalise();
        assert_eq!(faux.maisons.iter().map(|m| m.numero).collect::<Vec<_>>(), [4]);
    }

    #[test]
    fn un_debut_de_titre_est_un_mot_entier_sans_tenir_compte_de_la_casse() {
        let m = Maison { titres: vec!["Pilote".into(), "Chef adjoint".into()], ..Maison::vide(3) };
        for oui in ["Pilote 3", "pilote", "PILOTE 12 : suite", "Pilote\t4", " Pilote 3 ", "chef adjoint 2", "Chef adjoint"] {
            assert!(m.reclame_titre(oui), "« {oui} » devrait entrer");
        }
        // pas un autre mot qui commence pareil, pas un titre qui contient le début ailleurs, pas un titre voisin
        for non in ["Pilotes 3", "Pilote-bis", "Pilote3", "Le pilote 3", "", "Chef 2", "Chef adjointe", "Pilot"] {
            assert!(!m.reclame_titre(non), "« {non} » ne devrait PAS entrer");
        }
        // un début vide ne reconnaît rien (surtout pas tous les titres)
        let vide = Maison { titres: vec![String::new(), "  ".into()], ..Maison::vide(3) };
        assert!(!vide.reclame_titre("n'importe quoi") && !vide.reclame_titre(""));
        // un dossier se reconnaît par son nom exact, comme ceux du groupe 1
        let d = Maison { dossiers: vec!["projet-a".into()], ..Maison::vide(3) };
        assert!(d.reclame_dossier("projet-a") && !d.reclame_dossier("projet-b") && !d.reclame_dossier("Projet-a"));
    }
}
