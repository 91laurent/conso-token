//! Le mode `--pont` du vrai programme : silencieux (rien sur la sortie standard), code de sortie 0,
//! fichiers rangés dans le dossier demandé, `--suite` relaie la sortie d'une autre commande.
//! Les fichiers d'essai vont dans `CARGO_TARGET_TMPDIR`.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const EXE: &str = env!("CARGO_BIN_EXE_conso_token");

const JSON: &str = r#"{"rate_limits":{"five_hour":{"used_percentage":4.0,"resets_at":1792045200},"seven_day":{"used_percentage":64.0,"resets_at":1792062000}}}"#;

fn dossier(nom: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("binaire_{nom}"));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

/// Lance `conso_token --pont …` avec `entree` sur l'entrée standard ; rend (code, sortie standard).
fn pont(args: &[&str], d: &PathBuf, entree: &str) -> (Option<i32>, Vec<u8>) {
    let mut enfant = Command::new(EXE)
        .arg("--pont")
        .arg("--donnees")
        .arg(d)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("lancement du programme");
    enfant.stdin.take().unwrap().write_all(entree.as_bytes()).unwrap();
    let sortie = enfant.wait_with_output().unwrap();
    (sortie.status.code(), sortie.stdout)
}

#[test]
fn le_pont_est_silencieux_et_range_les_chiffres() {
    let d = dossier("silencieux");
    let (code, stdout) = pont(&[], &d, JSON);
    assert_eq!(code, Some(0));
    assert!(stdout.is_empty(), "le pont ne doit rien écrire : {:?}", String::from_utf8_lossy(&stdout));
    assert!(d.join("dernier.json").exists());
    assert_eq!(fs::read_to_string(d.join("historique.jsonl")).unwrap().lines().count(), 1);
    // même chiffres : l'historique ne grossit pas
    let (code, _) = pont(&[], &d, JSON);
    assert_eq!(code, Some(0));
    assert_eq!(fs::read_to_string(d.join("historique.jsonl")).unwrap().lines().count(), 1);
}

#[test]
fn entree_etrangere_ou_vide_ne_fait_pas_d_erreur() {
    let d = dossier("etranger");
    for entree in ["", "pas du json", "{\"truc\":1}", "[1,2,3]"] {
        let (code, stdout) = pont(&[], &d, entree);
        assert_eq!(code, Some(0), "entrée {entree:?}");
        assert!(stdout.is_empty(), "entrée {entree:?}");
    }
    assert!(!d.join("historique.jsonl").exists());
}

#[cfg(windows)]
#[test]
fn suite_relaie_la_sortie_de_l_autre_commande() {
    let d = dossier("suite");
    // `more` recopie son entrée : on doit retrouver le JSON tel quel, et le pont a quand même rangé les chiffres
    let (code, stdout) = pont(&["--suite", "more"], &d, JSON);
    assert_eq!(code, Some(0));
    assert!(String::from_utf8_lossy(&stdout).contains("\"seven_day\""), "sortie : {:?}", String::from_utf8_lossy(&stdout));
    assert!(d.join("dernier.json").exists());
    // une commande qui n'existe pas : rien n'est rendu, pas d'erreur
    let (code, stdout) = pont(&["--suite", "commande_qui_n_existe_pas_zzz"], &d, JSON);
    assert_eq!(code, Some(0));
    assert!(stdout.is_empty());
    // un chemin avec des espaces reste UN argument
    let (code, stdout) = pont(&["--suite", "cmd", "/C", "echo", "un deux"], &d, JSON);
    assert_eq!(code, Some(0));
    assert!(String::from_utf8_lossy(&stdout).contains("un deux"));
}
