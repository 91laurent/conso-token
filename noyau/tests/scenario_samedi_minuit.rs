//! Scénario « remise à zéro le samedi à minuit » : la remise à zéro hebdomadaire tombe le SAMEDI à 0h00 (Paris), pas le jeudi à 13h.
//! Rien n'est écrit en dur dans le programme : tout vient de `resets_at`. Ces tests le prouvent avec
//! cette remise à zéro, y compris pour la semaine qui contient le passage à l'heure d'hiver (25/10/2026).

use noyau::fr::{borne_jour, carte_reset_avis, carte_reset_detail, definition_du_jour, fin_de_jour, jour_long_heure};
use noyau::modele::{Echantillon, EtatJour, Fenetre, construire, semaines};
use noyau::temps::{JOUR_S, SEMAINE_S};

/// samedi 24/10/2026 à 00:00 Paris (UTC+2) = vendredi 23/10 22:00 UTC : fin de la semaine du 17 au 24.
const RESET_A: i64 = 1_792_792_800;
/// vendredi 30/10 22:00 UTC = vendredi 30/10 à 23:00 Paris (UTC+1) : fin de la semaine du 24 au 30 si
/// la remise à zéro reste fixée en UTC.
const RESET_B: i64 = 1_793_397_600;

#[test]
fn les_bornes_de_la_semaine_du_17_au_24_sont_a_minuit() {
    let debut = RESET_A - SEMAINE_S;
    let attendu = [
        ("sam 0h", "→ dim 0h"),
        ("dim 0h", "→ lun 0h"),
        ("lun 0h", "→ mar 0h"),
        ("mar 0h", "→ mer 0h"),
        ("mer 0h", "→ jeu 0h"),
        ("jeu 0h", "→ ven 0h"),
        ("ven 0h", "→ sam 0h"),
    ];
    for (i, (a, b)) in attendu.iter().enumerate() {
        let t = debut + i as i64 * JOUR_S;
        assert_eq!(borne_jour(t), *a, "début du jour {}", i + 1);
        assert_eq!(fin_de_jour(t), *b, "fin du jour {}", i + 1);
    }
    assert_eq!(
        definition_du_jour(RESET_A),
        "Un jour = 24 h, de la remise à zéro (samedi 0h) à la même heure le lendemain, pas minuit à minuit."
    );
    assert_eq!(jour_long_heure(RESET_A), "samedi 24/10 à 00:00");
}

#[test]
fn la_semaine_du_passage_a_l_heure_d_hiver_dit_la_verite() {
    // semaine du 24/10 00:00 (été) au vendredi 30/10 23:00 (hiver) : sept blocs de 24 h pleines
    let debut = RESET_B - SEMAINE_S;
    assert_eq!(borne_jour(debut), "sam 0h");
    // le bloc 2 commence dim 25/10 à 0h (encore l'été) ; le bloc 3 à 23h (l'hiver est arrivé à 3h)
    assert_eq!(borne_jour(debut + JOUR_S), "dim 0h");
    assert_eq!(fin_de_jour(debut + JOUR_S), "→ dim 23h");
    assert_eq!(borne_jour(debut + 2 * JOUR_S), "dim 23h");
    assert_eq!(borne_jour(debut + 6 * JOUR_S), "jeu 23h");
    assert_eq!(fin_de_jour(debut + 6 * JOUR_S), "→ ven 23h");
    // la fin de la semaine est affichée à la pendule d'hiver
    assert_eq!(jour_long_heure(RESET_B), "vendredi 30/10 à 23:00");
    assert_eq!(
        definition_du_jour(RESET_B),
        "Un jour = 24 h, de la remise à zéro (vendredi 23h) à la même heure le lendemain, pas minuit à minuit."
    );
}

#[test]
fn une_premiere_lecture_en_milieu_de_semaine_donne_un_budget_juste() {
    // L'utilisateur installe le programme lundi 19/10 à 14:00 Paris (12:00 UTC) avec 22 % déjà utilisés
    let debut = RESET_A - SEMAINE_S;
    let lundi_14h = 1_792_411_200; // 2026-10-19T12:00:00Z
    assert_eq!(borne_jour(debut + 2 * JOUR_S), "lun 0h");
    let e = [Echantillon { t: lundi_14h, cinq: None, sept: Some(Fenetre { p: 22.0, r: RESET_A }) }];
    let s = semaines(&e);
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].reset, RESET_A);
    let v = construire(&s[0], lundi_14h + 60, None).expect("vue");
    assert_eq!(v.aujourdhui, 2, "lundi est le 3e jour de sa semaine");
    assert_eq!(v.jours[2].etat, EtatJour::EnCours);
    assert!(v.debut_estime);
    // jours d'avant la première lecture : inconnus (« — »), pas inventés
    assert_eq!(v.jours[0].conso, None);
    assert_eq!(v.jours[1].conso, None);
    // budget du lundi = ce qui reste (78) ÷ 5 jours restants (lun, mar, mer, jeu, ven)
    assert!((v.jours[2].budget.unwrap() - 78.0 / 5.0).abs() < 0.01, "budget {:?}", v.jours[2].budget);
}

#[test]
fn la_remise_a_zero_suivante_ouvre_une_nouvelle_semaine() {
    let e = [
        Echantillon { t: RESET_A - 3_600, cinq: None, sept: Some(Fenetre { p: 90.0, r: RESET_A - 1 }) },
        // juste après la remise à zéro du samedi 0h00, Claude Code donne le prochain reset (+7 jours)
        Echantillon { t: RESET_A + 600, cinq: None, sept: Some(Fenetre { p: 1.0, r: RESET_B - 1 }) },
    ];
    let s = semaines(&e);
    assert_eq!(s.len(), 2);
    assert_eq!(s[0].reset, RESET_A);
    assert_eq!(s[1].reset, RESET_B);
    let v = construire(&s[1], RESET_A + 700, None).expect("vue de la nouvelle semaine");
    assert_eq!(v.aujourdhui, 0);
    assert_eq!(v.debut, RESET_A);
    // la nouvelle semaine s'affiche avec ses propres bornes, lues dans le reset reçu (reset en UTC fixe :
    // elle finit « vendredi 23h » à Paris après le passage à l'heure d'hiver)
    assert_eq!(borne_jour(v.debut), "sam 0h");
    assert_eq!(jour_long_heure(v.reset), "vendredi 30/10 à 23:00");
}

/// La semaine du changement d'heure, remise à zéro fixée en UTC : la carte « Ton forfait revient » écrit
/// désormais la VRAIE heure de l'instant reçu (vendredi 30/10 à 23:00, comme le compte à rebours), plus
/// « samedi 31/10 à 00:00 » (début + 7 jours), et l'avis dit que la pendule a changé le 25/10.
#[test]
fn la_carte_ecrit_la_vraie_heure_pour_un_reset_du_samedi_la_semaine_du_changement_d_heure() {
    let e = [Echantillon { t: RESET_A + 600, cinq: None, sept: Some(Fenetre { p: 1.0, r: RESET_B - 1 }) }];
    let v = construire(&semaines(&e)[0], RESET_A + 700, None).expect("vue");
    assert_eq!(carte_reset_detail(v.reset), "vendredi 30/10 à 23:00 · l’heure est relue à chaque mise à jour");
    assert_eq!(
        carte_reset_avis(v.debut, v.reset).as_deref(),
        Some("Une heure plus tôt : passage à l’heure d’hiver le dimanche 25/10.")
    );
    // la puce du titre dit les mêmes vraies heures : du samedi 0h au vendredi 23h (la fin lue, 23:00Z)
    assert_eq!(noyau::vues::puce(&v), "jours de 24 h : sam 0h → ven 23h");
}
