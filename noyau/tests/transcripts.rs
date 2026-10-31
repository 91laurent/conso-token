//! Test de lecture de vraies transcriptions FACTICES (fichiers fabriqués ici) : dossiers
//! imbriqués, doublon entre deux fichiers, lignes sans compteur, fichier ancien ignoré.
//! Les fichiers d'essai vont dans `CARGO_TARGET_TMPDIR`.

use std::fs;
use std::path::PathBuf;

use noyau::temps::{JOUR_S, SEMAINE_S};
use noyau::transcripts::Lecteur;

fn dossier(nom: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("transcripts_{nom}"));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn iso(t: i64) -> String {
    jiff::Timestamp::from_second(t).unwrap().to_string()
}

fn ligne(t: i64, id: &str, req: &str, entree: u64, sortie: u64, cache: u64) -> String {
    format!(
        r#"{{"type":"assistant","timestamp":"{}","requestId":"{req}","message":{{"id":"{id}","content":[{{"type":"text","text":"JAMAIS LU"}}],"usage":{{"input_tokens":{entree},"output_tokens":{sortie},"cache_creation_input_tokens":{cache},"cache_read_input_tokens":99999}}}}}}"#,
        iso(t)
    )
}

#[test]
fn lit_les_dossiers_imbriques_et_dedoublonne() {
    let racine = dossier("imbriques");
    // la semaine commence maintenant moins 3 jours (les fichiers sont tout juste écrits : ils sont récents)
    let maintenant = jiff::Timestamp::now().as_second();
    let debut = maintenant - 3 * JOUR_S;

    let projet_a = racine.join("projet-a");
    let projet_b = racine.join("projet-b").join("sous");
    fs::create_dir_all(&projet_a).unwrap();
    fs::create_dir_all(&projet_b).unwrap();

    let contenu_a = [
        ligne(debut + 100, "m1", "r1", 100, 50, 10),
        "{\"type\":\"user\",\"message\":{\"content\":\"pas de compteur ici\"}}".to_string(),
        ligne(debut + JOUR_S + 100, "m2", "r2", 1, 2, 3),
        "ligne cassée".to_string(),
    ]
    .join("\n");
    // le même message m1/r1 recopié dans un autre fichier (sous-agent) : compté une fois
    let contenu_b = [ligne(debut + 100, "m1", "r1", 100, 50, 10), ligne(debut + 2 * JOUR_S + 100, "m3", "r3", 7, 0, 0)].join("\n");
    fs::write(projet_a.join("s1.jsonl"), contenu_a).unwrap();
    fs::write(projet_b.join("s2.jsonl"), contenu_b).unwrap();
    fs::write(projet_a.join("notes.txt"), "pas un jsonl").unwrap();

    let mut lecteur = Lecteur::default();
    let t = lecteur.tokens_par_jour(&racine, debut);
    assert_eq!(t, [160, 6, 7, 0, 0, 0, 0]);

    // seconde lecture : mêmes fichiers, même résultat (le cache ne double rien)
    assert_eq!(lecteur.tokens_par_jour(&racine, debut), t);

    // un fichier qui grandit est relu
    let mut c = fs::read_to_string(projet_a.join("s1.jsonl")).unwrap();
    c.push('\n');
    c.push_str(&ligne(debut + 3 * JOUR_S - 10, "m4", "r4", 1_000, 0, 0));
    fs::write(projet_a.join("s1.jsonl"), c).unwrap();
    let t2 = lecteur.tokens_par_jour(&racine, debut);
    assert_eq!(t2[2], 7 + 1_000);
}

#[test]
fn une_ligne_en_cours_d_ecriture_est_lue_une_fois_complete() {
    let racine = dossier("incomplete");
    let maintenant = jiff::Timestamp::now().as_second();
    let debut = maintenant - JOUR_S;
    let l1 = ligne(debut + 10, "m1", "r1", 10, 0, 0);
    let l2 = ligne(debut + 20, "m2", "r2", 100, 0, 0);
    let (moitie, reste) = l2.split_at(l2.len() / 2);
    let chemin = racine.join("s.jsonl");
    // une ligne complète, puis la moitié d'une seconde (sans retour à la ligne)
    fs::write(&chemin, format!("{l1}\n{moitie}")).unwrap();
    let mut lecteur = Lecteur::default();
    assert_eq!(lecteur.tokens_par_jour(&racine, debut)[0], 10, "la ligne coupée n'est pas comptée");
    // elle se complète : comptée, et la première n'est pas comptée deux fois
    fs::write(&chemin, format!("{l1}\n{moitie}{reste}\n")).unwrap();
    assert_eq!(lecteur.tokens_par_jour(&racine, debut)[0], 110);
    // une dernière ligne complète mais SANS retour à la ligne final est comptée aussi
    let l3 = ligne(debut + 30, "m3", "r3", 1_000, 0, 0);
    fs::write(&chemin, format!("{l1}\n{moitie}{reste}\n{l3}")).unwrap();
    assert_eq!(lecteur.tokens_par_jour(&racine, debut)[0], 1_110);
}

#[test]
fn un_fichier_reecrit_plus_court_est_relu_du_debut() {
    let racine = dossier("reecrit");
    let maintenant = jiff::Timestamp::now().as_second();
    let debut = maintenant - JOUR_S;
    let chemin = racine.join("s.jsonl");
    fs::write(&chemin, format!("{}\n{}\n", ligne(debut + 10, "a", "1", 500, 0, 0), ligne(debut + 20, "b", "2", 500, 0, 0))).unwrap();
    let mut lecteur = Lecteur::default();
    assert_eq!(lecteur.tokens_par_jour(&racine, debut)[0], 1_000);
    fs::write(&chemin, format!("{}\n", ligne(debut + 10, "c", "3", 7, 0, 0))).unwrap();
    assert_eq!(lecteur.tokens_par_jour(&racine, debut)[0], 7, "l'ancien contenu n'est plus compté");
}

#[test]
fn dossier_inexistant_donne_des_zeros() {
    let mut lecteur = Lecteur::default();
    let t = lecteur.tokens_par_jour(&PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("zzz_absent"), 1_000_000);
    assert_eq!(t, [0; 7]);
}

#[test]
fn messages_hors_semaine_sont_ignores() {
    let racine = dossier("hors");
    let maintenant = jiff::Timestamp::now().as_second();
    let debut = maintenant - JOUR_S;
    fs::write(
        racine.join("s.jsonl"),
        [ligne(debut - 10, "a", "1", 500, 0, 0), ligne(debut + SEMAINE_S + 10, "b", "2", 500, 0, 0), ligne(debut + 10, "c", "3", 1, 1, 1)].join("\n"),
    )
    .unwrap();
    let mut lecteur = Lecteur::default();
    let t = lecteur.tokens_par_jour(&racine, debut);
    assert_eq!(t.iter().sum::<u64>(), 3);
}
