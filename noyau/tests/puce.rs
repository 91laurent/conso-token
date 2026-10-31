//! La puce du titre (v1.2.0, version internationale) dit les VRAIES heures locales du début et de la fin de
//! la semaine, comme la carte « Ton forfait revient » : « jeu 13h → jeu 12h » la semaine du passage à
//! l'heure d'hiver (25/10), « jeu 12h → jeu 13h » la semaine du passage à l'heure d'été (fin mars),
//! inchangée une semaine ordinaire. Fuseau de Paris (`.cargo/config.toml`) ; Montréal : `fuseau.rs` ;
//! hémisphère sud : `carte_reset_sud.rs`. Rien n'est écrit en dur : les heures viennent des deux instants.

use noyau::langue::{Langue, avec};
use noyau::modele::{Semaine, construire};
use noyau::temps::SEMAINE_S;
use noyau::vues::puce;

/// 2026-10-29T11:00:00Z = jeudi 29/10 à 12:00 Paris : la semaine du passage à l'heure d'hiver (25/10).
const RESET_SEMAINE_25_10: i64 = 1_793_271_600;
/// 2026-11-05T11:00:00Z = jeudi 05/11 à 12:00 : semaine ordinaire d'hiver.
const RESET_ORDINAIRE_HIVER: i64 = 1_793_876_400;
/// 2026-10-15T11:00:00Z = jeudi 15/10 à 13:00 : semaine ordinaire d'été.
const RESET_ORDINAIRE_ETE: i64 = 1_792_062_000;
/// 2027-04-01T11:00:00Z = jeudi 01/04 à 13:00 : la semaine du passage à l'heure d'été (28/03/2027).
const RESET_FIN_MARS: i64 = 1_806_577_200;

fn la_puce(reset: i64) -> String {
    let maintenant = reset - SEMAINE_S + 20 * 3_600;
    let v = construire(&Semaine { reset, points: vec![(maintenant, 10.0)] }, maintenant, None).expect("vue");
    puce(&v)
}

#[test]
fn semaine_du_25_10_la_puce_dit_13h_puis_12h() {
    assert_eq!(la_puce(RESET_SEMAINE_25_10), "jours de 24 h : jeu 13h → jeu 12h");
}

#[test]
fn fin_mars_la_puce_dit_12h_puis_13h() {
    assert_eq!(la_puce(RESET_FIN_MARS), "jours de 24 h : jeu 12h → jeu 13h");
}

/// Garde (verte avant comme après) : une semaine ordinaire dit la même heure aux deux bouts.
#[test]
fn semaine_ordinaire_la_puce_dit_la_meme_heure_deux_fois() {
    assert_eq!(la_puce(RESET_ORDINAIRE_ETE), "jours de 24 h : jeu 13h → jeu 13h");
    assert_eq!(la_puce(RESET_ORDINAIRE_HIVER), "jours de 24 h : jeu 12h → jeu 12h");
}

/// La semaine du 25/10 à Paris, la puce dans les huit langues.
#[test]
fn la_puce_dans_les_huit_langues() {
    use Langue::*;
    let attendu = [
        (En, "24 h days: Thu 13:00 → Thu 12:00"),
        (Fr, "jours de 24 h : jeu 13h → jeu 12h"),
        (Es, "días de 24 h: jue 13:00 → jue 12:00"),
        (De, "24-Std.-Tage: Do 13:00 → Do 12:00"),
        (Pt, "dias de 24 h: qui 13:00 → qui 12:00"),
        (It, "giorni di 24 h: gio 13:00 → gio 12:00"),
        (Zh, "24 小时为一天：周四 13:00 → 周四 12:00"),
        (Ja, "24時間単位：木 13:00 → 木 12:00"),
    ];
    for (l, texte) in attendu {
        avec(l, || {
            assert_eq!(la_puce(RESET_SEMAINE_25_10), texte, "{l:?}");
            assert_eq!(la_puce(RESET_ORDINAIRE_HIVER), texte.replace("13:00", "12:00").replace("13h", "12h"), "{l:?} : semaine ordinaire");
        });
    }
}
