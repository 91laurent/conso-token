//! Tests du pont : lecture du JSON de la ligne d'état et rangement sur le disque.
//! Les fichiers d'essai vont dans `CARGO_TARGET_TMPDIR` (jamais dans le dossier temporaire du système).

use std::fs;
use std::path::PathBuf;

use noyau::modele::{Echantillon, Fenetre};
use noyau::pont::{charger, enregistrer, lire_dernier, lire_json};

const RESET: i64 = 1_792_062_000;

fn dossier(nom: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("pont_{nom}"));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

const JSON_COMPLET: &str = r#"{
  "model": {"id": "x"},
  "rate_limits": {
    "five_hour": {"used_percentage": 4.0, "resets_at": 1792045200},
    "seven_day": {"used_percentage": 64.0, "resets_at": 1792062000}
  },
  "autre": {"truc": [1, 2, 3]}
}"#;

#[test]
fn lit_les_deux_fenetres() {
    let e = lire_json(JSON_COMPLET, 100).expect("échantillon");
    assert_eq!(e.t, 100);
    assert_eq!(e.cinq, Some(Fenetre { p: 4.0, r: 1_792_045_200 }));
    assert_eq!(e.sept, Some(Fenetre { p: 64.0, r: RESET }));
}

#[test]
fn json_sans_chiffres_donne_un_echantillon_vide() {
    // JSON valide sans fenêtre : un échantillon vide (il prouve que le branchement marche)
    for entree in ["{}", r#"{"rate_limits": {}}"#, r#"{"rate_limits":{"seven_day":{"used_percentage":5}}}"#] {
        let e = lire_json(entree, 1).unwrap_or_else(|| panic!("échantillon attendu pour {entree}"));
        assert!(e.cinq.is_none() && e.sept.is_none(), "{entree}");
    }
    // pas du JSON : rien
    assert!(lire_json("pas du json", 1).is_none());
    assert!(lire_json("", 1).is_none());
}

#[test]
fn un_echantillon_vide_ne_touche_que_dernier_json() {
    let d = dossier("vide");
    let e = lire_json("{}", 77).unwrap();
    enregistrer(&d, &e).unwrap();
    assert_eq!(lire_dernier(&d).unwrap().t, 77);
    assert!(charger(&d).0.is_empty(), "aucune ligne d'historique");
    assert!(!d.join("historique.jsonl").exists());
}

#[test]
fn un_echantillon_vide_n_efface_pas_les_chiffres_connus() {
    // une session qui n'a pas encore reçu de chiffres appelle le pont entre deux sessions qui en ont
    let d = dossier("vide_apres_plein");
    enregistrer(&d, &lire_json(JSON_COMPLET, 100).unwrap()).unwrap();
    let dernier_avant = fs::read(d.join("dernier.json")).unwrap();
    let historique_avant = fs::read(d.join("historique.jsonl")).unwrap();
    for (entree, t) in [("{}", 200), (r#"{"rate_limits":{"five_hour":{"used_percentage":9,"resets_at":1792045200}}}"#, 250)] {
        enregistrer(&d, &lire_json(entree, t).unwrap()).unwrap();
        assert_eq!(fs::read(d.join("dernier.json")).unwrap(), dernier_avant, "dernier.json doit rester intact pour {entree}");
        assert_eq!(fs::read(d.join("historique.jsonl")).unwrap(), historique_avant, "{entree}");
    }
    // la session suivante renvoie les mêmes chiffres : pas de ligne en double, et l'heure avance
    enregistrer(&d, &lire_json(JSON_COMPLET, 300).unwrap()).unwrap();
    assert_eq!(charger(&d).0.len(), 1, "une seule ligne d'historique");
    assert_eq!(lire_dernier(&d).unwrap().t, 300);
    assert_eq!(lire_dernier(&d).unwrap().sept.unwrap().p, 64.0);
}

#[test]
fn des_sessions_qui_s_alternent_n_ajoutent_pas_de_lignes() {
    // session fraîche (63 %) et session au repos (60 %) qui se relancent à tour de rôle
    let d = dossier("alternance");
    let sept = |p: f64| Some(Fenetre { p, r: RESET });
    let mut t = 100;
    for p in [60.0, 63.0, 60.0, 63.0, 60.0, 63.5, 60.0] {
        enregistrer(&d, &Echantillon { t, cinq: None, sept: sept(p) }).unwrap();
        t += 60;
    }
    let (histo, dernier) = charger(&d);
    let valeurs: Vec<f64> = histo.iter().map(|e| e.sept.unwrap().p).collect();
    assert_eq!(valeurs, vec![60.0, 63.0, 63.5], "historique : {histo:?}");
    // dernier.json garde le chiffre le plus haut, avec l'heure du dernier appel
    let dernier = dernier.unwrap();
    assert_eq!(dernier.sept.unwrap().p, 63.5);
    assert_eq!(dernier.t, 100 + 6 * 60);
    // une nouvelle semaine (reset avancé de 7 jours) repart de son propre chiffre, même plus bas
    let suivante = Fenetre { p: 1.0, r: RESET + 7 * 86_400 };
    enregistrer(&d, &Echantillon { t: t + 10, cinq: None, sept: Some(suivante) }).unwrap();
    assert_eq!(charger(&d).0.last().unwrap().sept.unwrap().p, 1.0);
}

#[test]
fn une_seule_fenetre_suffit() {
    let e = lire_json(r#"{"rate_limits":{"five_hour":{"used_percentage":12.5,"resets_at":1792045200}}}"#, 7).unwrap();
    assert!(e.sept.is_none());
    assert_eq!(e.cinq.unwrap().p, 12.5);
}

#[test]
fn l_historique_ne_grossit_que_sur_changement() {
    let d = dossier("changement");
    let a = lire_json(JSON_COMPLET, 100).unwrap();
    enregistrer(&d, &a).unwrap();
    // même chiffres, 5 min plus tard, reset lu avec une seconde d'écart : rien de neuf
    let mut b = a;
    b.t = 400;
    b.sept = Some(Fenetre { p: 64.0, r: RESET - 1 });
    enregistrer(&d, &b).unwrap();
    // la fenêtre de 5 h bouge seule : pas de ligne de plus (elle ne sert à rien dans l'historique)
    let mut cinq = a;
    cinq.t = 600;
    cinq.cinq = Some(Fenetre { p: 9.0, r: 1_792_045_200 });
    enregistrer(&d, &cinq).unwrap();
    // le pourcentage de la semaine monte : une ligne de plus
    let mut c = a;
    c.t = 900;
    c.sept = Some(Fenetre { p: 65.0, r: RESET });
    enregistrer(&d, &c).unwrap();

    let (histo, dernier) = charger(&d);
    assert_eq!(histo.len(), 2, "historique : {histo:?}");
    assert_eq!(histo[0].t, 100);
    assert_eq!(histo[1].t, 900);
    // dernier.json garde la lecture la plus récente, même sans changement (âge des chiffres)
    assert_eq!(dernier.unwrap().t, 900);
    enregistrer(&d, &Echantillon { t: 1_200, cinq: c.cinq, sept: c.sept }).unwrap();
    assert_eq!(lire_dernier(&d).unwrap().t, 1_200);
    assert_eq!(charger(&d).0.len(), 2);
}

#[test]
fn les_lignes_abimees_sont_ignorees() {
    let d = dossier("abimee");
    let a = lire_json(JSON_COMPLET, 100).unwrap();
    enregistrer(&d, &a).unwrap();
    let chemin = d.join("historique.jsonl");
    let mut s = fs::read_to_string(&chemin).unwrap();
    s.push_str("{ceci n'est pas du json\n\n");
    fs::write(&chemin, s).unwrap();
    let (histo, _) = charger(&d);
    assert_eq!(histo.len(), 1);
}

#[test]
fn dossier_vide_ou_absent() {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("pont_inexistant_zzz");
    let (histo, dernier) = charger(&d);
    assert!(histo.is_empty());
    assert!(dernier.is_none());
}
