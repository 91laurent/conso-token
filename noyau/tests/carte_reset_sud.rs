//! L'hémisphère sud : à Sydney l'heure d'été COMMENCE début octobre (l'heure locale avance : « plus tard »)
//! et finit début avril (elle recule : « plus tôt »). Le sens et la saison de l'avis viennent du sens du
//! changement, pas du calendrier de Paris. Un seul test dans ce fichier : il fixe `CONSO_TOKEN_TZ` pour
//! tout le processus (chaque fichier de `tests/` est un processus à part).

use noyau::fr::{carte_reset_avis, carte_reset_detail};
use noyau::langue::{Langue, avec};
use noyau::modele::{Semaine, construire};
use noyau::temps::SEMAINE_S;
use noyau::vues::puce;

/// 2026-10-07T22:00:00Z = jeudi 08/10 à 09:00 à Sydney (heure d'été, UTC+11) ; le début de semaine
/// (30/09 22:00Z) est à 08:00 (heure d'hiver, UTC+10) : le passage a eu lieu le dimanche 04/10 à 2h → 3h.
const RESET_OCTOBRE: i64 = 1_791_410_400;
/// 2027-04-07T23:00:00Z = jeudi 08/04 à 09:00 à Sydney (UTC+10) ; début de semaine à 10:00 (UTC+11) :
/// retour à l'heure d'hiver le dimanche 04/04/2027 à 3h → 2h.
const RESET_AVRIL: i64 = 1_807_138_800;
/// 2026-10-15T22:00:00Z : une semaine ordinaire d'heure d'été.
const RESET_ORDINAIRE: i64 = 1_792_101_600;

#[test]
fn a_sydney_le_sens_et_la_saison_de_l_avis_suivent_le_changement() {
    // SAFETY : un seul test dans ce processus, rien d'autre ne lit l'environnement en même temps.
    unsafe { std::env::set_var("CONSO_TOKEN_TZ", "Australia/Sydney") };
    let avis = |reset: i64| carte_reset_avis(reset - SEMAINE_S, reset);

    // octobre : l'heure locale AVANCE → « plus tard », heure d'ÉTÉ (alors que c'est le printemps austral)
    assert_eq!(carte_reset_detail(RESET_OCTOBRE), "jeudi 08/10 à 09:00 · l’heure est relue à chaque mise à jour");
    assert_eq!(avis(RESET_OCTOBRE).as_deref(), Some("Une heure plus tard : passage à l’heure d’été le dimanche 04/10."));
    // avril : l'heure locale RECULE → « plus tôt », heure d'HIVER
    assert_eq!(carte_reset_detail(RESET_AVRIL), "jeudi 08/04 à 09:00 · l’heure est relue à chaque mise à jour");
    assert_eq!(avis(RESET_AVRIL).as_deref(), Some("Une heure plus tôt : passage à l’heure d’hiver le dimanche 04/04."));
    // semaine ordinaire, et semaine d'après : aucun avis ; la semaine du 25/10 de Paris n'est pas un événement ici
    assert_eq!(avis(RESET_ORDINAIRE), None);
    assert_eq!(avis(RESET_OCTOBRE + SEMAINE_S), None);
    assert_eq!(avis(1_793_271_600), None, "le 25/10 est un dimanche ordinaire à Sydney");
    // la puce dit les mêmes vraies heures : jeu 8h (heure d'hiver) → jeu 9h (heure d'été), puis l'inverse en avril
    let la_puce = |reset: i64| {
        let maintenant = reset - SEMAINE_S + 20 * 3_600;
        let v = construire(&Semaine { reset, points: vec![(maintenant, 10.0)] }, maintenant, None).unwrap();
        puce(&v)
    };
    assert_eq!(la_puce(RESET_OCTOBRE), "jours de 24 h : jeu 8h → jeu 9h");
    assert_eq!(la_puce(RESET_AVRIL), "jours de 24 h : jeu 10h → jeu 9h");
    assert_eq!(la_puce(RESET_ORDINAIRE), "jours de 24 h : ven 9h → ven 9h"); // 2026-10-15T22:00Z = vendredi 16/10 à 09:00
    // la même règle en anglais et en japonais
    avec(Langue::En, || assert_eq!(avis(RESET_OCTOBRE).as_deref(), Some("One hour later: clocks go forward to daylight saving time on Sunday 10/04.")));
    avec(Langue::Ja, || assert_eq!(avis(RESET_AVRIL).as_deref(), Some("1時間早い：4月4日 日曜日に冬時間へ切り替わります。")));
}
