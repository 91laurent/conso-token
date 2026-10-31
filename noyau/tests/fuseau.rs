//! Le fuseau n'est plus Paris en dur : un utilisateur à Montréal lit SES heures, avec SON changement
//! d'heure (le 1er novembre 2026, pas le 25 octobre). Un seul test dans ce fichier : il fixe
//! `CONSO_TOKEN_TZ` pour tout le processus (chaque fichier de `tests/` est un processus à part).

use noyau::fr::{borne_jour, carte_reset_avis, carte_reset_detail, jour_long_heure, nom_du_jour};
use noyau::modele::{Semaine, construire};
use noyau::temps::JOUR_S;
use noyau::vues::puce;

/// jeudi 29/10/2026 à 13:00 UTC = jeudi 9h à Montréal (heure d'été de l'Est, jusqu'au 1er novembre),
/// et 14h à Paris (heure d'hiver depuis le 25/10).
const RESET: i64 = 1_793_278_800;

#[test]
fn un_utilisateur_de_montreal_lit_ses_heures() {
    // SAFETY : un seul test dans ce processus, rien d'autre ne lit l'environnement en même temps.
    unsafe { std::env::set_var("CONSO_TOKEN_TZ", "America/Toronto") };
    let debut = RESET - 7 * JOUR_S;
    assert_eq!(borne_jour(debut), "jeu 9h");
    assert_eq!(borne_jour(RESET), "jeu 9h", "pas de changement d'heure au Canada avant le 1er novembre");
    assert_eq!(jour_long_heure(RESET), "jeudi 29/10 à 09:00");
    assert_eq!(carte_reset_detail(RESET), "jeudi 29/10 à 09:00 · l’heure est relue à chaque mise à jour");
    assert_eq!(carte_reset_avis(debut, RESET), None, "Montréal ne change pas d'heure le 25/10 : aucun avis (Paris, lui, en a un)");
    assert_eq!(nom_du_jour(debut, RESET), "jeu→ven");
    let maintenant = debut + 20 * 3_600;
    let v = construire(&Semaine { reset: RESET, points: vec![(maintenant, 10.0)] }, maintenant, None).unwrap();
    assert_eq!(puce(&v), "jours de 24 h : jeu 9h → jeu 9h");

    // témoin : le même instant, en fuseau de Paris, s'écrit autrement (le test peut échouer)
    unsafe { std::env::set_var("CONSO_TOKEN_TZ", "Europe/Paris") };
    assert_eq!(borne_jour(RESET), "jeu 14h");
    // et la semaine qui contient le changement d'heure canadien : le 1er novembre à 2h → 1h
    unsafe { std::env::set_var("CONSO_TOKEN_TZ", "America/Toronto") };
    // remise à zéro fixée en UTC (13:00Z) : jeudi 29/10 = 9h EDT, jeudi 5/11 = 8h EST (UTC-5)
    let reset_nov = RESET + 7 * JOUR_S;
    assert_eq!(borne_jour(reset_nov - 7 * JOUR_S), "jeu 9h");
    assert_eq!(borne_jour(reset_nov), "jeu 8h", "l'instant reçu, à l'heure d'hiver de Montréal");
    // la carte écrit la VRAIE heure de l'instant reçu (8h EST), plus celle du début de semaine (9h EDT) ;
    // l'avis dit que la pendule de Montréal a reculé le dimanche 1er novembre
    assert_eq!(carte_reset_detail(reset_nov), "jeudi 05/11 à 08:00 · l’heure est relue à chaque mise à jour");
    assert_eq!(
        carte_reset_avis(reset_nov - 7 * JOUR_S, reset_nov).as_deref(),
        Some("Une heure plus tôt : passage à l’heure d’hiver le dimanche 01/11.")
    );
    // la puce du titre dit les mêmes vraies heures : 9h (EDT) → 8h (EST)
    let maintenant = reset_nov - 7 * JOUR_S + 20 * 3_600;
    let v = construire(&Semaine { reset: reset_nov, points: vec![(maintenant, 10.0)] }, maintenant, None).unwrap();
    assert_eq!(puce(&v), "jours de 24 h : jeu 9h → jeu 8h");
    // et la semaine d'après : plus d'avis
    assert_eq!(carte_reset_avis(reset_nov, reset_nov + 7 * JOUR_S), None);
}
