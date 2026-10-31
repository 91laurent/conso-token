//! Nombres, durées, jours et heures, dans la langue en cours (`langue::courante`) ; le nom du module
//! est historique (c'était le français seul). Fuseau : celui de Windows (`jiff`), lu au démarrage
//! (jamais un décalage fixe : l'heure d'hiver arrive le 25/10/2026).

use std::sync::OnceLock;

use jiff::{Timestamp, Zoned, civil::Weekday, tz::TimeZone};

use crate::langue::{self, Langue};
use crate::temps::JOUR_S;
use crate::textes::{Cle, fmt, jour_court, jour_long, t};

/// Le fuseau de l'utilisateur : `CONSO_TOKEN_TZ` (nom IANA, pour les essais et les captures), sinon le
/// fuseau de Windows, sinon `Europe/Paris` (comportement d'avant la version internationale). Jamais
/// un décalage fixe : le changement d'heure de chaque pays suit son propre calendrier.
fn fuseau() -> TimeZone {
    if let Some(z) = std::env::var("CONSO_TOKEN_TZ").ok().and_then(|n| TimeZone::get(&n).ok()) {
        return z;
    }
    static SYSTEME: OnceLock<TimeZone> = OnceLock::new();
    SYSTEME
        .get_or_init(|| {
            let z = TimeZone::system();
            if z.iana_name().is_some() { z } else { TimeZone::get("Europe/Paris").unwrap_or(TimeZone::UTC) }
        })
        .clone()
}

/// Heure LOCALE d'un instant en secondes Unix (le nom « paris » est historique : c'était le seul fuseau).
pub fn paris(t: i64) -> Zoned {
    let ts = Timestamp::from_second(t).unwrap_or(Timestamp::UNIX_EPOCH);
    ts.to_zoned(fuseau())
}

fn rang_jour(z: &Zoned) -> usize {
    match z.weekday() {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}

/// `81,4` ou `64` : virgule (point en anglais, chinois, japonais), une décimale seulement si elle compte.
pub fn nombre(x: f64) -> String {
    let arrondi = (x * 10.0).round() / 10.0;
    let arrondi = if arrondi == 0.0 { 0.0 } else { arrondi }; // évite « -0 »
    if (arrondi - arrondi.round()).abs() < 1e-9 {
        format!("{}", arrondi.round() as i64)
    } else if matches!(langue::courante(), Langue::En | Langue::Zh | Langue::Ja) {
        format!("{arrondi:.1}")
    } else {
        format!("{arrondi:.1}").replace('.', ",")
    }
}

/// `64 %` (ou `64%` selon la langue) : un pourcentage tout fait, pour les textes qui le prennent en `{0}`.
pub fn pourcent(x: f64) -> String {
    fmt(Cle::Pourcent, &[&nombre(x)])
}

/// Nombre avec son signe typographique : `+2,1`, `−1,3` (vrai signe moins), `0`.
pub fn nombre_signe(x: f64) -> String {
    let n = nombre(x.abs());
    if n == "0" {
        "0".to_string()
    } else if x > 0.0 {
        format!("+{n}")
    } else {
        format!("−{n}")
    }
}

/// `2,4 M`, `850 k`, `12` : tokens en abrégé.
pub fn tokens(n: u64) -> String {
    let milliers = (n as f64 / 1_000.0).round();
    if n >= 1_000_000 || milliers >= 1_000.0 {
        format!("{} M", nombre(n as f64 / 1_000_000.0))
    } else if n >= 1_000 {
        format!("{} k", nombre(milliers))
    } else {
        n.to_string()
    }
}

/// `13h`, ou `13h05` en français ; `13:00` ailleurs : l'heure locale, les minutes (en français)
/// seulement si elles ne sont pas nulles.
fn heure_courte(z: &Zoned) -> String {
    if z.minute() == 0 {
        fmt(Cle::HeureRonde, &[&z.hour()])
    } else {
        fmt(Cle::HeureMinutes, &[&z.hour(), &format!("{:02}", z.minute())])
    }
}

/// `29/10` (`10/29`, `29.10.`, `10月29日`…) : deux chiffres partout sauf en chinois et japonais.
fn date_courte(z: &Zoned) -> String {
    if matches!(langue::courante(), Langue::Zh | Langue::Ja) {
        fmt(Cle::Date, &[&z.day(), &z.month()])
    } else {
        fmt(Cle::Date, &[&format!("{:02}", z.day()), &format!("{:02}", z.month())])
    }
}

/// `jeudi 29/10 à 13:00` : jour, date et heure (sur 24 h) de cet instant local.
fn jour_date_heure(z: &Zoned) -> String {
    fmt(Cle::JourLongHeure, &[&jour_long(rang_jour(z)), &date_courte(z), &format!("{:02}:{:02}", z.hour(), z.minute())])
}

/// `jeu 13h` : la borne d'un jour (de 24 h), jour de la semaine et heure locale de cet instant.
/// Un jour de la semaine de Claude ne commence pas à minuit mais à l'heure de la remise à zéro.
pub fn borne_jour(t: i64) -> String {
    let z = paris(t);
    format!("{} {}", jour_court(rang_jour(&z)), heure_courte(&z))
}

/// `→ ven 13h` : la fin d'un jour de 24 h qui commence en `debut` (avec `borne_jour(debut)`, on lit
/// `jeu 13h` puis `→ ven 13h`). Le jour qui contient le passage à l'heure d'hiver reste de 24 h
/// pleines : il finit donc une heure plus tôt à la pendule (`→ dim 12h`), et l'étiquette le montre.
pub fn fin_de_jour(debut: i64) -> String {
    format!("→ {}", borne_jour(debut + JOUR_S))
}

/// `jeu→ven` : le nom d'un bloc de 24 h = les DEUX jours de la semaine qu'il chevauche (le dernier
/// bloc de la semaine finit le jeudi de la remise à zéro, qui ainsi apparaît). Si la remise à zéro
/// est à minuit (`reset` : un samedi à 0h, par exemple), il n'y a pas de chevauchement : un seul nom, celui
/// du jour qui contient le milieu du bloc. Un bloc de 25 h ou 23 h qui retombe sur le même jour de
/// la semaine garde aussi un seul nom.
pub fn nom_du_jour(debut: i64, reset: i64) -> String {
    let r = paris(reset);
    if r.hour() == 0 && r.minute() == 0 {
        return jour_court(rang_jour(&paris(debut + JOUR_S / 2))).to_string();
    }
    let (a, b) = (jour_court(rang_jour(&paris(debut))), jour_court(rang_jour(&paris(debut + JOUR_S))));
    if a == b { a.to_string() } else { format!("{a}→{b}") }
}

/// `jeudi 13h` : jour de la semaine (en toutes lettres) et heure locale de cet instant.
pub fn jour_et_heure(t: i64) -> String {
    let z = paris(t);
    format!("{} {}", jour_long(rang_jour(&z)), heure_courte(&z))
}

/// La phrase de la légende qui dit ce qu'est un jour ; l'heure vient de la remise à zéro lue
/// (`reset`, secondes Unix), jamais d'un chiffre écrit en dur.
pub fn definition_du_jour(reset: i64) -> String {
    fmt(Cle::DefinitionJour, &[&jour_et_heure(reset)])
}

/// `jeudi 15/10 à 13:00`.
pub fn jour_long_heure(t: i64) -> String {
    jour_date_heure(&paris(t))
}

/// La ligne de détail de la carte « Ton forfait revient » : la VRAIE heure locale de l'instant de remise à
/// zéro reçu (`jeudi 29/10 à 12:00` la semaine du passage à l'heure d'hiver à Paris), la même que celle du
/// compte à rebours. Elle ne reprend plus l'heure du début de semaine (« 13h »).
pub fn carte_reset_detail(reset: i64) -> String {
    fmt(Cle::ResetDetail, &[&jour_long_heure(reset)])
}

/// Décalage local d'un instant par rapport à UTC, en secondes.
fn decalage(t: i64) -> i32 {
    paris(t).offset().seconds()
}

/// L'avis de la carte, seulement si l'heure locale change entre le début de la semaine (`debut`) et sa fin
/// (`reset`) : « plus tôt » si le décalage baisse (heure d'hiver), « plus tard » s'il monte (heure d'été),
/// avec le jour du changement, dans la langue en cours. Tout est calculé à partir des deux instants et du
/// fuseau de l'utilisateur : aucune date, aucune heure, aucun sens, aucune saison écrits à la main (le sens
/// du changement dit la saison, donc l'avis est juste aussi dans l'hémisphère sud). `None` la semaine ordinaire.
pub fn carte_reset_avis(debut: i64, reset: i64) -> Option<String> {
    let (avant, apres) = (decalage(debut), decalage(reset));
    if debut >= reset || avant == apres {
        return None;
    }
    // premier instant au décalage de la fin (un seul changement en une semaine) : dichotomie
    let (mut bas, mut haut) = (debut, reset);
    while haut - bas > 1 {
        let milieu = bas + (haut - bas) / 2;
        if decalage(milieu) == avant { bas = milieu } else { haut = milieu }
    }
    let ecart = i64::from(apres - avant);
    let quantite = if ecart.abs() == 3_600 { t(Cle::AvisUneHeure).to_string() } else { duree(ecart.abs()) };
    let z = paris(haut);
    let jour = fmt(Cle::JourDate, &[&jour_long(rang_jour(&z)), &date_courte(&z)]);
    Some(fmt(if ecart < 0 { Cle::AvisPlusTot } else { Cle::AvisPlusTard }, &[&quantite, &jour]))
}

/// `31 h 12` ; `45 min` sous une heure ; `moins d'1 min`.
pub fn duree(secondes: i64) -> String {
    let minutes = secondes.max(0) / 60;
    if minutes >= 60 {
        fmt(Cle::DureeHeures, &[&(minutes / 60), &format!("{:02}", minutes % 60)])
    } else if minutes >= 1 {
        fmt(Cle::DureeMinutes, &[&minutes])
    } else {
        fmt(Cle::DureeMoinsUneMinute, &[])
    }
}

/// `à l'instant`, `il y a 2 min`, `il y a 3 h`, `il y a 2 j`.
pub fn age(secondes: i64) -> String {
    let s = secondes.max(0);
    if s < 60 {
        fmt(Cle::AgeInstant, &[])
    } else if s < 3_600 {
        fmt(Cle::AgeMinutes, &[&(s / 60)])
    } else if s < 86_400 {
        fmt(Cle::AgeHeures, &[&(s / 3_600)])
    } else {
        fmt(Cle::AgeJours, &[&(s / 86_400)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2026-10-15T11:00:00Z = jeudi 15/10 à 13:00 (UTC+2)
    const RESET: i64 = 1_792_062_000;

    #[test]
    fn nombres_francais() {
        assert_eq!(nombre(64.0), "64");
        assert_eq!(nombre(81.43), "81,4");
        assert_eq!(nombre(14.285714), "14,3");
        assert_eq!(nombre(-0.04), "0");
        assert_eq!(nombre_signe(2.14), "+2,1");
        assert_eq!(nombre_signe(-1.3), "−1,3");
        assert_eq!(nombre_signe(0.0), "0");
    }

    #[test]
    fn tokens_abreges() {
        assert_eq!(tokens(2_400_000), "2,4 M");
        assert_eq!(tokens(850_400), "850 k");
        assert_eq!(tokens(12), "12");
        assert_eq!(tokens(999_600), "1 M"); // pas « 1000 k »
        assert_eq!(tokens(0), "0");
    }

    #[test]
    fn jour_et_heure_de_paris_en_ete() {
        assert_eq!(jour_long_heure(RESET), "jeudi 15/10 à 13:00");
        // l'étiquette d'un jour montre ses vraies bornes (13h), pas une date de calendrier
        assert_eq!(borne_jour(RESET - 7 * 86_400), "jeu 13h");
        assert_eq!(fin_de_jour(RESET - 7 * 86_400), "→ ven 13h");
        assert_eq!(borne_jour(RESET), "jeu 13h");
        assert_eq!(fin_de_jour(RESET), "→ ven 13h");
        assert_eq!(borne_jour(RESET + 5 * 3_600), "jeu 18h");
    }

    #[test]
    fn les_sept_jours_de_la_semaine_ont_leurs_bornes() {
        // semaine du 15/10 : sept blocs de 24 h, du jeudi 13h au jeudi 13h
        let debut = RESET;
        let attendu = [
            ("jeu 13h", "→ ven 13h"),
            ("ven 13h", "→ sam 13h"),
            ("sam 13h", "→ dim 13h"),
            ("dim 13h", "→ lun 13h"),
            ("lun 13h", "→ mar 13h"),
            ("mar 13h", "→ mer 13h"),
            ("mer 13h", "→ jeu 13h"),
        ];
        // (les 15 → 22/10 sont en heure d'été : le passage à l'heure d'hiver est le 25/10)
        for (i, (a, b)) in attendu.iter().enumerate() {
            let t = debut + i as i64 * 86_400;
            assert_eq!(borne_jour(t), *a, "début du jour {}", i + 1);
            assert_eq!(fin_de_jour(t), *b, "fin du jour {}", i + 1);
        }
    }

    #[test]
    fn le_nom_d_un_bloc_porte_les_deux_jours_qu_il_chevauche() {
        let attendu = ["jeu→ven", "ven→sam", "sam→dim", "dim→lun", "lun→mar", "mar→mer", "mer→jeu"];
        for (i, a) in attendu.iter().enumerate() {
            assert_eq!(nom_du_jour(RESET + i as i64 * 86_400, RESET), *a, "bloc {}", i + 1);
        }
        // le bloc du passage à l'heure d'hiver (sam 24/10 13h → dim 25/10 12h) garde ses deux jours
        assert_eq!(nom_du_jour(RESET + 9 * 86_400, RESET + 7 * 86_400), "sam→dim");
    }

    #[test]
    fn remise_a_zero_a_minuit_un_seul_nom() {
        // 2026-10-16T22:00:00Z = samedi 17/10 à 00:00 (UTC+2) : cas d'une remise à zéro à minuit
        let reset = 1_792_188_000;
        assert_eq!(borne_jour(reset), "sam 0h");
        let debut = reset - 7 * 86_400;
        let attendu = ["sam", "dim", "lun", "mar", "mer", "jeu", "ven"];
        for (i, a) in attendu.iter().enumerate() {
            assert_eq!(nom_du_jour(debut + i as i64 * 86_400, reset), *a, "bloc {}", i + 1);
        }
        // semaine à cheval sur l'heure d'hiver : le premier bloc commence à 1h (23h de moins la veille),
        // le nom reste celui du jour du bloc
        let reset = 1_792_188_000 + 14 * 86_400 + 3_600; // sam 31/10 à 00:00 (UTC+1)
        assert_eq!(nom_du_jour(reset - 7 * 86_400, reset), "sam");
    }

    #[test]
    fn une_borne_avec_des_minutes_les_montre() {
        assert_eq!(borne_jour(RESET + 5 * 60), "jeu 13h05");
        assert_eq!(borne_jour(RESET - 60), "jeu 12h59");
    }

    #[test]
    fn la_definition_du_jour_vient_de_la_remise_a_zero_lue() {
        assert_eq!(jour_et_heure(RESET), "jeudi 13h");
        assert_eq!(
            definition_du_jour(RESET),
            "Un jour = 24 h, de la remise à zéro (jeudi 13h) à la même heure le lendemain, pas minuit à minuit."
        );
        // si Anthropic déplace la remise à zéro, la phrase suit (ici vendredi 7h)
        assert!(definition_du_jour(RESET + 18 * 3_600).contains("de la remise à zéro (vendredi 7h) à la même heure"));
    }

    #[test]
    fn le_jour_du_passage_a_l_heure_d_hiver_montre_sa_vraie_fin() {
        // le bloc de 24 h qui commence sam 24/10 à 13h (UTC+2) finit dim 25/10 à 12h (UTC+1) :
        // 24 h pleines, et l'étiquette le dit au lieu de mentir
        let t = RESET + 9 * 86_400; // sam 24/10 13:00
        assert_eq!(borne_jour(t), "sam 13h");
        assert_eq!(fin_de_jour(t), "→ dim 12h");
    }

    #[test]
    fn l_heure_d_hiver_est_prise_en_compte() {
        // le 25/10/2026 à 03:00 on passe à l'heure d'hiver (UTC+1) :
        // 2026-11-05T12:00:00Z = jeudi 05/11 à 13:00 (UTC+1), pas à 14:00
        let t = RESET + 21 * 86_400 + 3_600; // 2026-11-05T12:00:00Z
        assert_eq!(jour_long_heure(t), "jeudi 05/11 à 13:00");
    }

    #[test]
    fn les_formats_de_chaque_langue() {
        use crate::langue::{Langue::*, avec};
        let (fin_novembre, ecoule) = (RESET + 21 * 86_400 + 3_600, 31 * 3_600 + 12 * 60 + 5);
        // (langue, jeudi 15/10 13:00, borne, bloc, jeudi 05/11 13:00, 81,43, durée, âge de 2 min 30)
        let attendu = [
            (En, "Thursday 10/15 at 13:00", "Thu 13:00", "Thu→Fri", "Thursday 11/05 at 13:00", "81.4", "31 h 12 min", "2 min ago"),
            (Fr, "jeudi 15/10 à 13:00", "jeu 13h", "jeu→ven", "jeudi 05/11 à 13:00", "81,4", "31 h 12", "il y a 2 min"),
            (Es, "jueves 15/10 a las 13:00", "jue 13:00", "jue→vie", "jueves 05/11 a las 13:00", "81,4", "31 h 12 min", "hace 2 min"),
            (De, "Donnerstag, 15.10., 13:00 Uhr", "Do 13:00", "Do→Fr", "Donnerstag, 05.11., 13:00 Uhr", "81,4", "31 Std. 12 Min.", "vor 2 Min."),
            (Pt, "quinta-feira 15/10 às 13:00", "qui 13:00", "qui→sex", "quinta-feira 05/11 às 13:00", "81,4", "31 h 12 min", "há 2 min"),
            (It, "giovedì 15/10 alle 13:00", "gio 13:00", "gio→ven", "giovedì 05/11 alle 13:00", "81,4", "31 h 12 min", "2 min fa"),
            (Zh, "10月15日 星期四 13:00", "周四 13:00", "周四→周五", "11月5日 星期四 13:00", "81.4", "31小时12分钟", "2分钟前"),
            (Ja, "10月15日 木曜日 13:00", "木 13:00", "木→金", "11月5日 木曜日 13:00", "81.4", "31時間12分", "2分前"),
        ];
        for (l, jlh, borne, bloc, nov, nb, dur, age_) in attendu {
            avec(l, || {
                assert_eq!(jour_long_heure(RESET), jlh, "{l:?}");
                assert_eq!(borne_jour(RESET), borne, "{l:?}");
                assert_eq!(nom_du_jour(RESET, RESET), bloc, "{l:?}");
                assert_eq!(jour_long_heure(fin_novembre), nov, "{l:?}");
                assert_eq!(nombre(81.43), nb, "{l:?}");
                assert_eq!(duree(ecoule), dur, "{l:?}");
                assert_eq!(age(150), age_, "{l:?}");
            });
        }
        avec(En, || assert_eq!(nombre_signe(-1.3), "−1.3"));
        avec(De, || assert_eq!(nombre_signe(2.14), "+2,1"));
        avec(Fr, || assert_eq!(pourcent(81.43), "81,4 %"));
        avec(En, || assert_eq!(pourcent(81.43), "81.4%"));
    }

    #[test]
    fn durees_et_ages() {
        assert_eq!(duree(31 * 3_600 + 12 * 60 + 5), "31 h 12");
        assert_eq!(duree(45 * 60), "45 min");
        assert_eq!(duree(10), "moins d'1 min");
        assert_eq!(duree(-5), "moins d'1 min");
        assert_eq!(age(10), "à l'instant");
        assert_eq!(age(2 * 60 + 30), "il y a 2 min");
        assert_eq!(age(3 * 3_600), "il y a 3 h");
        assert_eq!(age(2 * 86_400), "il y a 2 j");
    }
}
