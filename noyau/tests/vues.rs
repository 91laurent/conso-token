//! La vue choisie est rangée dans `vue.txt`, dans le dossier de données.

use std::path::PathBuf;

use noyau::vues::{NB_VUES, NOMS_VUES, VUE_DEFAUT, ecrire_vue, lire_vue};

fn dossier(nom: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("vues").join(nom);
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[test]
fn la_vue_par_defaut_est_les_batteries() {
    assert_eq!(NOMS_VUES[VUE_DEFAUT], "batteries");
    assert_eq!(NOMS_VUES.len(), NB_VUES);
    // dossier absent, fichier absent : les batteries
    assert_eq!(lire_vue(&dossier("absent")), VUE_DEFAUT);
}

#[test]
fn chaque_vue_se_range_et_se_relit() {
    let d = dossier("aller_retour");
    for v in 0..NB_VUES {
        assert!(ecrire_vue(&d, v), "écriture de la vue {v}");
        assert_eq!(lire_vue(&d), v, "relecture de la vue {v}");
    }
    // le fichier est lisible à la main
    assert_eq!(std::fs::read_to_string(d.join("vue.txt")).unwrap(), "semaine\n");
}

#[test]
fn un_fichier_illisible_ou_inconnu_rend_la_vue_par_defaut() {
    let d = dossier("inconnu");
    std::fs::create_dir_all(&d).unwrap();
    for contenu in ["", "n'importe quoi", "7", "Lignes\r\n"] {
        std::fs::write(d.join("vue.txt"), contenu).unwrap();
        let attendu = if contenu.trim().eq_ignore_ascii_case("lignes") { 1 } else { VUE_DEFAUT };
        assert_eq!(lire_vue(&d), attendu, "contenu {contenu:?}");
    }
    // pas de vue au-delà de la cinquième
    assert!(!ecrire_vue(&d, NB_VUES));
    assert_eq!(lire_vue(&d), 1, "un refus n'écrase rien");
}
