//! La carte « Ton forfait revient » (v1.2.0, version internationale) : l'heure écrite est la VRAIE heure
//! locale de l'instant de remise, et un avis n'apparaît que si l'heure locale change entre le début et la
//! fin de la semaine. Fuseau de Paris (`.cargo/config.toml`) ; sens, date et saison de l'avis sont calculés,
//! jamais écrits en dur dans le programme. Montréal : `fuseau.rs` ; hémisphère sud : `carte_reset_sud.rs`.

use noyau::fr::{carte_reset_avis, carte_reset_detail};
use noyau::langue::{Langue, avec};
use noyau::temps::SEMAINE_S;

const SUITE: &str = " · l’heure est relue à chaque mise à jour";

/// 2026-10-29T11:00:00Z = jeudi 29/10 à 12:00 Paris : la semaine du passage à l'heure d'hiver (25/10).
const RESET_SEMAINE_25_10: i64 = 1_793_271_600;
/// 2026-11-05T11:00:00Z = jeudi 05/11 à 12:00 : semaine ordinaire d'hiver.
const RESET_ORDINAIRE_HIVER: i64 = 1_793_876_400;
/// 2026-10-15T11:00:00Z = jeudi 15/10 à 13:00 : semaine ordinaire d'été.
const RESET_ORDINAIRE_ETE: i64 = 1_792_062_000;
/// 2027-04-01T11:00:00Z = jeudi 01/04 à 13:00 : la semaine du passage à l'heure d'été (28/03/2027).
const RESET_FIN_MARS: i64 = 1_806_577_200;

fn avis(reset: i64) -> Option<String> {
    carte_reset_avis(reset - SEMAINE_S, reset)
}

#[test]
fn semaine_du_25_10_la_carte_dit_12h_et_l_avis_dit_une_heure_plus_tot() {
    assert_eq!(carte_reset_detail(RESET_SEMAINE_25_10), format!("jeudi 29/10 à 12:00{SUITE}"));
    assert_eq!(avis(RESET_SEMAINE_25_10).as_deref(), Some("Une heure plus tôt : passage à l’heure d’hiver le dimanche 25/10."));
}

#[test]
fn semaine_ordinaire_aucun_avis() {
    assert_eq!(carte_reset_detail(RESET_ORDINAIRE_HIVER), format!("jeudi 05/11 à 12:00{SUITE}"));
    assert_eq!(avis(RESET_ORDINAIRE_HIVER), None);
    assert_eq!(carte_reset_detail(RESET_ORDINAIRE_ETE), format!("jeudi 15/10 à 13:00{SUITE}"));
    assert_eq!(avis(RESET_ORDINAIRE_ETE), None);
}

#[test]
fn fin_mars_la_carte_dit_13h_et_l_avis_dit_une_heure_plus_tard() {
    assert_eq!(carte_reset_detail(RESET_FIN_MARS), format!("jeudi 01/04 à 13:00{SUITE}"));
    assert_eq!(avis(RESET_FIN_MARS).as_deref(), Some("Une heure plus tard : passage à l’heure d’été le dimanche 28/03."));
}

/// Le jour du changement est calculé : décalé d'une semaine, l'avis suit ; avant et après, il disparaît.
#[test]
fn l_avis_suit_la_semaine_qui_contient_le_changement() {
    assert_eq!(avis(RESET_SEMAINE_25_10 - SEMAINE_S), None);
    assert_eq!(avis(RESET_SEMAINE_25_10 + SEMAINE_S), None);
    assert_eq!(avis(RESET_FIN_MARS - SEMAINE_S), None);
    assert_eq!(avis(RESET_FIN_MARS + SEMAINE_S), None);
    // la semaine qui finit le mardi 27/10 (début mardi 20/10, pendule d'été) : le changement est dedans
    let mardi_27_10 = RESET_SEMAINE_25_10 - 2 * 86_400;
    assert!(avis(mardi_27_10).is_some_and(|a| a.ends_with("le dimanche 25/10.")));
}

/// L'avis et la ligne de détail dans les huit langues : langue, détail (semaine du 25/10), avis heure
/// d'hiver (plus tôt), avis heure d'été (fin mars, plus tard).
#[test]
fn l_avis_et_le_detail_dans_les_huit_langues() {
    use Langue::*;
    let attendu = [
        (
            En,
            "Thursday 10/29 at 12:00 · the time is re-read at every update",
            "One hour earlier: clocks go back to standard time on Sunday 10/25.",
            "One hour later: clocks go forward to daylight saving time on Sunday 03/28.",
        ),
        (
            Fr,
            "jeudi 29/10 à 12:00 · l’heure est relue à chaque mise à jour",
            "Une heure plus tôt : passage à l’heure d’hiver le dimanche 25/10.",
            "Une heure plus tard : passage à l’heure d’été le dimanche 28/03.",
        ),
        (
            Es,
            "jueves 29/10 a las 12:00 · la hora se vuelve a leer en cada actualización",
            "Una hora antes: cambio a la hora de invierno el domingo 25/10.",
            "Una hora después: cambio a la hora de verano el domingo 28/03.",
        ),
        (
            De,
            "Donnerstag, 29.10., 12:00 Uhr · die Uhrzeit wird bei jeder Aktualisierung neu gelesen",
            "Eine Stunde früher: Umstellung auf Winterzeit am Sonntag, 25.10.",
            "Eine Stunde später: Umstellung auf Sommerzeit am Sonntag, 28.03.",
        ),
        (
            Pt,
            "quinta-feira 29/10 às 12:00 · o horário é relido a cada atualização",
            "Uma hora mais cedo: mudança para o horário de inverno (domingo 25/10).",
            "Uma hora mais tarde: mudança para o horário de verão (domingo 28/03).",
        ),
        (
            It,
            "giovedì 29/10 alle 12:00 · l’orario viene riletto a ogni aggiornamento",
            "Un’ora prima: passaggio all’ora solare, domenica 25/10.",
            "Un’ora dopo: passaggio all’ora legale, domenica 28/03.",
        ),
        (
            Zh,
            "10月29日 星期四 12:00 · 每次更新都会重新读取时间",
            "提前1小时：10月25日 星期日起改用冬令时。",
            "推迟1小时：3月28日 星期日起改用夏令时。",
        ),
        (
            Ja,
            "10月29日 木曜日 12:00 · 更新のたびに時刻を再取得します",
            "1時間早い：10月25日 日曜日に冬時間へ切り替わります。",
            "1時間遅い：3月28日 日曜日に夏時間へ切り替わります。",
        ),
    ];
    for (l, detail, hiver, ete) in attendu {
        avec(l, || {
            assert_eq!(carte_reset_detail(RESET_SEMAINE_25_10), detail, "{l:?}");
            assert_eq!(avis(RESET_SEMAINE_25_10).as_deref(), Some(hiver), "{l:?}");
            assert_eq!(avis(RESET_FIN_MARS).as_deref(), Some(ete), "{l:?}");
            assert_eq!(avis(RESET_ORDINAIRE_HIVER), None, "{l:?}");
        });
    }
}
