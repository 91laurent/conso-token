//! Lecture des conversations dans des transcriptions FACTICES (fichiers fabriqués ici) : contexte du
//! dernier message de Claude, exclusions (sous-agents, messages latéraux, modèle synthétique, sessions
//! planifiées), dossier de travail, état « résumée ». Les fichiers d'essai vont dans `CARGO_TARGET_TMPDIR`.
//! Chaque ligne porte un texte « JAMAIS LU » : la lecture n'en garde rien.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use serde_json::{Value, json};

use noyau::conversations::{Conversation, LecteurConversations};

/// Les fichiers d'essai viennent d'être écrits : « maintenant » est le vrai maintenant.
fn maintenant() -> i64 {
    jiff::Timestamp::now().as_second()
}

fn dossier(nom: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("conversations_{nom}"));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn iso(t: i64) -> String {
    jiff::Timestamp::from_second(t).unwrap().to_string()
}

fn assistant_v(t: i64, session: &str, modele: &str, cwd: &str, entree: u64, creation: u64, lecture: u64) -> Value {
    json!({
        "type": "assistant", "timestamp": iso(t), "sessionId": session, "isSidechain": false, "cwd": cwd, "requestId": "r1",
        "message": {
            "id": "m1", "model": modele, "role": "assistant",
            "content": [{"type": "text", "text": "JAMAIS LU"}],
            "usage": {"input_tokens": entree, "cache_creation_input_tokens": creation, "cache_read_input_tokens": lecture, "output_tokens": 7}
        }
    })
}

fn assistant(t: i64, session: &str, cwd: &str, entree: u64, creation: u64, lecture: u64) -> String {
    assistant_v(t, session, "claude-opus-5-5", cwd, entree, creation, lecture).to_string()
}

fn utilisateur(t: i64, session: &str, origine: Option<&str>) -> String {
    let mut v = json!({"type": "user", "timestamp": iso(t), "sessionId": session, "cwd": "D:\\x", "message": {"role": "user", "content": "JAMAIS LU"}});
    if let Some(o) = origine {
        v["turnOrigin"] = json!(o);
    }
    v.to_string()
}

fn ecrire(chemin: &PathBuf, lignes: &[String]) {
    fs::create_dir_all(chemin.parent().unwrap()).unwrap();
    fs::write(chemin, lignes.join("\n") + "\n").unwrap();
}

fn ajouter(chemin: &PathBuf, lignes: &[String]) {
    let mut f = OpenOptions::new().append(true).open(chemin).unwrap();
    for l in lignes {
        writeln!(f, "{l}").unwrap();
    }
}

fn lire(racine: &PathBuf) -> Vec<Conversation> {
    LecteurConversations::default().lire(racine, maintenant()).conversations
}

#[test]
fn le_contexte_est_celui_du_dernier_message_de_claude() {
    let racine = dossier("contexte");
    let t = maintenant();
    ecrire(
        &racine.join("projet-a").join("s1.jsonl"),
        &[
            utilisateur(t - 300, "s1", Some("human")),
            assistant(t - 200, "s1", r"D:\travail\projet-a", 10, 20, 100_000),
            assistant(t - 100, "s1", r"D:\travail\projet-a", 3, 5, 300_000),
        ],
    );
    let v = lire(&racine);
    assert_eq!(v.len(), 1);
    let c = &v[0];
    assert_eq!(c.contexte, 3 + 5 + 300_000, "entrée + création + lecture de cache, du DERNIER message");
    assert_eq!(c.modele, "claude-opus-5-5");
    assert_eq!(c.dossier, "projet-a");
    assert_eq!(c.session, "s1");
    assert_eq!(c.dernier, t - 100);
    assert!(!c.resumee);
    assert!(!format!("{v:?}").contains("JAMAIS LU"), "aucun contenu n'est gardé");
}

#[test]
fn les_sous_agents_les_messages_lateraux_et_le_modele_synthetique_sont_ecartes() {
    let racine = dossier("exclusions");
    let t = maintenant();
    let bon = assistant(t - 500, "s1", r"D:\p", 1, 1, 50_000);
    let mut lateral = assistant_v(t - 400, "s1", "claude-opus-5-5", r"D:\p", 1, 1, 900_000);
    lateral["isSidechain"] = json!(true);
    let synthetique = assistant_v(t - 300, "s1", "<synthetic>", r"D:\p", 1, 1, 900_000);
    ecrire(&racine.join("p").join("s1.jsonl"), &[bon, lateral.to_string(), synthetique.to_string()]);
    // un fichier de sous-agent (même contenu qu'une vraie session) : pas une conversation
    ecrire(&racine.join("p").join("s1").join("subagents").join("agent-1.jsonl"), &[assistant(t - 10, "s9", r"D:\autre", 1, 1, 400_000)]);
    let v = lire(&racine);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].contexte, 50_002, "le dernier message PROPRE, pas le latéral ni le synthétique");
}

#[test]
fn les_sessions_planifiees_sont_ecartees() {
    let racine = dossier("planifiees");
    let t = maintenant();
    // planifiée : la première ligne qui porte turnOrigin dit « scheduled »
    ecrire(&racine.join("p").join("plan.jsonl"), &[utilisateur(t - 600, "plan", Some("scheduled")), assistant(t - 500, "plan", r"D:\p", 1, 1, 10_000)]);
    // planifiée aussi : « scheduled » d'abord, un « sdk » (l'origine des vraies sessions de l'appli de bureau, voir
    // le test suivant) sur une ligne ensuite ne la sauve pas : seule la PREMIÈRE origine compte
    ecrire(
        &racine.join("p").join("plan2.jsonl"),
        &[utilisateur(t - 590, "plan2", Some("scheduled")), assistant(t - 480, "plan2", r"D:\p", 1, 1, 15_000), utilisateur(t - 470, "plan2", Some("sdk"))],
    );
    // vraie session : « human » d'abord, un « scheduled » ensuite ne change rien
    ecrire(
        &racine.join("p").join("vraie.jsonl"),
        &[utilisateur(t - 600, "vraie", Some("human")), assistant(t - 500, "vraie", r"D:\p", 1, 1, 20_000), utilisateur(t - 400, "vraie", Some("scheduled"))],
    );
    // vraie session, telle que l'appli de bureau l'écrit : « sdk » d'abord, un « scheduled » ensuite ne change rien
    ecrire(
        &racine.join("p").join("vraie_sdk.jsonl"),
        &[utilisateur(t - 600, "vraie_sdk", Some("sdk")), assistant(t - 500, "vraie_sdk", r"D:\p", 1, 1, 25_000), utilisateur(t - 400, "vraie_sdk", Some("scheduled"))],
    );
    // aucune ligne ne porte turnOrigin : pas planifiée
    ecrire(&racine.join("p").join("sans.jsonl"), &[assistant(t - 300, "sans", r"D:\p", 1, 1, 30_000)]);
    let mut ids: Vec<String> = lire(&racine).into_iter().map(|c| c.session).collect();
    ids.sort();
    assert_eq!(ids, ["sans", "vraie", "vraie_sdk"], "plan et plan2 (planifiées) sont écartées");
}

#[test]
fn seule_l_origine_scheduled_ecarte_une_session() {
    // Dans les vrais fichiers (comptage du 08/11/2026 : 39 transcriptions de moins de 24 h, hors sous-agents),
    // la première origine est « scheduled » (27) ou « sdk » (12) ; « human » existe aussi côté terminal.
    // Une règle « tout sauf human est planifié » écarterait toutes les vraies sessions « sdk » : ce test
    // échouerait. Une origine qu'on ne connaît pas encore reste visible (une ligne en trop vaut mieux
    // qu'une conversation qui disparaît).
    let racine = dossier("origines");
    let t = maintenant();
    for (nom, origine) in [("o_sdk", Some("sdk")), ("o_human", Some("human")), ("o_cli", Some("cli")), ("o_inconnue", Some("origine-pas-encore-connue")), ("o_vide", Some("")), ("o_aucune", None)] {
        ecrire(&racine.join("p").join(format!("{nom}.jsonl")), &[utilisateur(t - 600, nom, origine), assistant(t - 500, nom, r"D:\p", 1, 1, 10_000)]);
    }
    ecrire(&racine.join("p").join("o_plan.jsonl"), &[utilisateur(t - 600, "o_plan", Some("scheduled")), assistant(t - 500, "o_plan", r"D:\p", 1, 1, 10_000)]);
    let mut ids: Vec<String> = lire(&racine).into_iter().map(|c| c.session).collect();
    ids.sort();
    assert_eq!(ids, ["o_aucune", "o_cli", "o_human", "o_inconnue", "o_sdk", "o_vide"], "seule l'origine « scheduled » écarte une session");
}

#[test]
fn la_premiere_ligne_turnorigin_peut_etre_loin_dans_la_tete() {
    let racine = dossier("tete_large");
    let t = maintenant();
    // une première ligne énorme (plus de 16 Ko) avant la ligne qui dit « scheduled »
    let mut gros = assistant_v(t - 900, "plan", "claude-opus-5-5", r"D:\p", 1, 1, 5_000);
    gros["message"]["content"][0]["text"] = json!("x".repeat(40_000));
    ecrire(&racine.join("p").join("plan.jsonl"), &[gros.to_string(), utilisateur(t - 800, "plan", Some("scheduled")), assistant(t - 700, "plan", r"D:\p", 1, 1, 6_000)]);
    assert!(lire(&racine).is_empty(), "la tête est élargie jusqu'à trouver turnOrigin");
}

#[test]
fn le_dossier_de_travail_et_les_copies_de_travail() {
    let racine = dossier("dossiers");
    let t = maintenant();
    ecrire(&racine.join("a").join("a.jsonl"), &[assistant(t - 30, "a", r"D:\travail\Projet Beta", 1, 1, 10_000)]);
    ecrire(&racine.join("b").join("b.jsonl"), &[assistant(t - 20, "b", r"D:\Projet Beta\.claude\worktrees\essai-3", 1, 1, 20_000)]);
    ecrire(&racine.join("c").join("c.jsonl"), &[assistant(t - 10, "c", "/home/u/projet-c/.claude/worktrees/x/sous", 1, 1, 30_000)]);
    let mut d: Vec<String> = lire(&racine).into_iter().map(|c| c.dossier).collect();
    d.sort();
    assert_eq!(d, ["Projet Beta", "Projet Beta", "projet-c"]);
}

#[test]
fn seule_la_ligne_compact_boundary_marque_la_conversation() {
    let racine = dossier("resumee");
    let t = maintenant();
    let chemin = racine.join("p").join("s1.jsonl");
    // un résultat d'outil qui CITE le mot, et même un faux « subtype » dans son contenu : pas une marque
    let cite = json!({"type": "user", "timestamp": iso(t - 400), "sessionId": "s1", "cwd": r"D:\p",
        "message": {"role": "user", "content": [{"type": "tool_result", "content": "\"subtype\":\"compact_boundary\" compact_boundary"}]}})
    .to_string();
    ecrire(&chemin, &[utilisateur(t - 500, "s1", Some("human")), assistant(t - 450, "s1", r"D:\p", 1, 1, 400_000), cite]);
    assert!(!lire(&racine)[0].resumee, "citer le mot ne suffit pas");

    // la vraie ligne système
    let vraie = json!({"type": "system", "subtype": "compact_boundary", "timestamp": iso(t - 100), "sessionId": "s1", "cwd": r"D:\p",
        "compactMetadata": {"trigger": "auto"}})
    .to_string();
    ajouter(&chemin, &[vraie, assistant(t - 50, "s1", r"D:\p", 1, 1, 23_000)]);
    let mut lecteur = LecteurConversations::default();
    let v = lecteur.lire(&racine, maintenant()).conversations;
    assert!(v[0].resumee);
    assert_eq!(v[0].contexte, 23_002, "le contexte est celui d'après la compaction");

    // la marque reste, même quand la conversation continue et que la ligne sort de la queue de 64 Ko
    let mut filler = assistant_v(t - 40, "s1", "claude-opus-5-5", r"D:\p", 1, 1, 24_000);
    filler["message"]["content"][0]["text"] = json!("y".repeat(100_000));
    ajouter(&chemin, &[filler.to_string(), assistant(t - 20, "s1", r"D:\p", 1, 1, 25_000)]);
    let v = lecteur.lire(&racine, maintenant()).conversations;
    assert!(v[0].resumee, "marque gardée");
    assert_eq!(v[0].contexte, 25_002);
}

#[test]
fn un_fichier_qui_grandit_est_relu_et_un_fichier_reecrit_plus_court_aussi() {
    let racine = dossier("grandit");
    let t = maintenant();
    let chemin = racine.join("p").join("s1.jsonl");
    ecrire(&chemin, &[assistant(t - 300, "s1", r"D:\p", 1, 1, 100_000)]);
    let mut lecteur = LecteurConversations::default();
    assert_eq!(lecteur.lire(&racine, maintenant()).conversations[0].contexte, 100_002);
    // rien n'a changé : même résultat
    assert_eq!(lecteur.lire(&racine, maintenant()).conversations[0].contexte, 100_002);
    // le fichier grandit
    ajouter(&chemin, &[assistant(t - 100, "s1", r"D:\p", 1, 1, 500_000)]);
    assert_eq!(lecteur.lire(&racine, maintenant()).conversations[0].contexte, 500_002);
    // le fichier est réécrit plus court
    ecrire(&chemin, &[assistant(t - 50, "s1", r"D:\p", 1, 1, 7)]);
    assert_eq!(lecteur.lire(&racine, maintenant()).conversations[0].contexte, 9);
}

#[test]
fn une_ligne_en_cours_d_ecriture_ne_casse_rien() {
    let racine = dossier("coupee");
    let t = maintenant();
    let chemin = racine.join("p").join("s1.jsonl");
    let entiere = assistant(t - 100, "s1", r"D:\p", 1, 1, 40_000);
    let coupee = assistant(t - 50, "s1", r"D:\p", 1, 1, 90_000);
    let moitie = &coupee[..coupee.len() / 2];
    fs::create_dir_all(chemin.parent().unwrap()).unwrap();
    fs::write(&chemin, format!("{entiere}\n{moitie}")).unwrap();
    let v = lire(&racine);
    assert_eq!(v[0].contexte, 40_002, "la ligne coupée est ignorée, la précédente est lue");
}

#[test]
fn le_dernier_message_plus_loin_que_la_queue_est_trouve() {
    let racine = dossier("loin");
    let t = maintenant();
    // 300 Ko de résultat d'outil APRÈS le dernier message de Claude : bien plus que la queue de 64 Ko
    let gros = json!({"type": "user", "timestamp": iso(t - 10), "sessionId": "s1", "cwd": r"D:\p",
        "message": {"role": "user", "content": [{"type": "tool_result", "content": "z".repeat(300_000)}]}})
    .to_string();
    ecrire(&racine.join("p").join("s1.jsonl"), &[assistant(t - 100, "s1", r"D:\p", 1, 1, 123_000), gros]);
    let v = lire(&racine);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].contexte, 123_002);
}

#[test]
fn les_fichiers_de_plus_de_24_heures_ne_sont_pas_ouverts() {
    let racine = dossier("anciens");
    let t = maintenant();
    let vieux = racine.join("p").join("vieux.jsonl");
    ecrire(&vieux, &[assistant(t - 30 * 3_600, "vieux", r"D:\p", 1, 1, 10_000)]);
    File::options().write(true).open(&vieux).unwrap().set_modified(SystemTime::now() - Duration::from_secs(30 * 3_600)).unwrap();
    // fichier récent, mais dont le dernier message de Claude a plus de 24 h
    ecrire(&racine.join("p").join("neuf.jsonl"), &[assistant(t - 26 * 3_600, "neuf", r"D:\p", 1, 1, 20_000)]);
    // et un vrai récent
    ecrire(&racine.join("p").join("ok.jsonl"), &[assistant(t - 26 * 60, "ok", r"D:\p", 1, 1, 30_000)]);
    let v = lire(&racine);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].session, "ok");
}

#[test]
fn un_dossier_absent_ne_donne_rien() {
    let lecture = LecteurConversations::default().lire(&PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("zzz_absent_conv"), maintenant());
    assert!(lecture.conversations.is_empty());
    assert!(!lecture.format_incompris(), "pas de fichier : rien d'incompris");
}

#[test]
fn si_rien_n_est_lisible_le_format_est_dit_incompris() {
    let racine = dossier("incompris");
    let t = maintenant();
    // des fichiers récents, mais aucune ligne de Claude avec des compteurs : le format a changé
    let sans_compteur = json!({"type": "assistant", "timestamp": iso(t), "sessionId": "x", "message": {"role": "assistant", "content": "JAMAIS LU"}}).to_string();
    for n in 0..2 {
        ecrire(&racine.join("p").join(format!("{n}.jsonl")), &[sans_compteur.clone()]);
    }
    let deux = LecteurConversations::default().lire(&racine, maintenant());
    assert_eq!((deux.candidats, deux.lisibles), (2, 0));
    assert!(!deux.format_incompris(), "deux fichiers vides de sens, c'est peut-être juste le début de la journée");
    ecrire(&racine.join("p").join("2.jsonl"), &[sans_compteur]);
    let trois = LecteurConversations::default().lire(&racine, maintenant());
    assert!(trois.format_incompris());
    // une seule ligne lisible suffit à lever le doute
    ecrire(&racine.join("p").join("3.jsonl"), &[assistant(t - 5, "ok", r"D:\p", 1, 1, 8_000)]);
    assert!(!LecteurConversations::default().lire(&racine, maintenant()).format_incompris());
}

#[test]
fn des_conversations_pas_encore_repondues_ne_sont_pas_un_format_incompris() {
    let racine = dossier("neuves");
    let t = maintenant();
    // quatre conversations qui viennent de s'ouvrir : le message de l'utilisateur, aucune réponse de Claude
    for n in 0..4 {
        ecrire(&racine.join("p").join(format!("n{n}.jsonl")), &[utilisateur(t - 20, &format!("n{n}"), Some("sdk"))]);
    }
    let lecture = LecteurConversations::default().lire(&racine, maintenant());
    assert_eq!((lecture.candidats, lecture.lisibles), (0, 0), "sans réponse de Claude, ce ne sont pas des candidats");
    assert!(!lecture.format_incompris(), "des conversations neuves ne disent rien du format");
    assert!(lecture.conversations.is_empty());

    // un vrai changement de format reste vu, même au milieu de conversations neuves : trois fichiers où Claude a
    // répondu sans compteurs lisibles
    let sans_compteur = json!({"type": "assistant", "timestamp": iso(t), "sessionId": "x", "message": {"role": "assistant", "content": "JAMAIS LU"}}).to_string();
    for n in 0..3 {
        ecrire(&racine.join("q").join(format!("c{n}.jsonl")), &[utilisateur(t - 30, &format!("c{n}"), Some("sdk")), sans_compteur.clone()]);
    }
    let melange = LecteurConversations::default().lire(&racine, maintenant());
    assert_eq!((melange.candidats, melange.lisibles), (3, 0), "les 4 neuves ne comptent pas, les 3 sans compteur oui");
    assert!(melange.format_incompris());

    // et dès que Claude répond dans une conversation neuve, elle est lue (le fichier qui grandit est relu)
    let mut lecteur = LecteurConversations::default();
    let racine2 = dossier("neuve_puis_repondue");
    let chemin = racine2.join("p").join("s1.jsonl");
    ecrire(&chemin, &[utilisateur(t - 20, "s1", Some("sdk"))]);
    assert!(lecteur.lire(&racine2, maintenant()).conversations.is_empty());
    ajouter(&chemin, &[assistant(t - 5, "s1", r"D:\p", 1, 1, 60_000)]);
    let apres = lecteur.lire(&racine2, maintenant());
    assert_eq!((apres.candidats, apres.lisibles), (1, 1));
    assert_eq!(apres.conversations[0].contexte, 60_002);
}

#[test]
fn les_reponses_laterales_ou_synthetiques_ne_comptent_pas_comme_une_reponse() {
    let racine = dossier("pas_de_reponse");
    let t = maintenant();
    // des lignes « assistant » qui ne sont pas de vraies réponses (message latéral, modèle synthétique) :
    // trois conversations neuves de plus, pas un format incompris
    let mut lateral = assistant_v(t - 40, "l", "claude-opus-5-5", r"D:\p", 1, 1, 50_000);
    lateral["isSidechain"] = json!(true);
    let synthetique = assistant_v(t - 40, "s", "<synthetic>", r"D:\p", 0, 0, 0);
    for n in 0..3 {
        let extra = if n == 0 { lateral.to_string() } else { synthetique.to_string() };
        ecrire(&racine.join("p").join(format!("x{n}.jsonl")), &[utilisateur(t - 60, &format!("x{n}"), None), extra]);
    }
    let lecture = LecteurConversations::default().lire(&racine, maintenant());
    assert_eq!(lecture.candidats, 0);
    assert!(!lecture.format_incompris());
}

#[test]
fn une_meme_session_dans_deux_fichiers_n_est_comptee_qu_une_fois() {
    let racine = dossier("doublon");
    let t = maintenant();
    ecrire(&racine.join("p").join("a.jsonl"), &[assistant(t - 300, "s1", r"D:\p", 1, 1, 100_000)]);
    ecrire(&racine.join("q").join("b.jsonl"), &[assistant(t - 100, "s1", r"D:\p", 1, 1, 200_000)]);
    let v = lire(&racine);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].contexte, 200_002, "la plus récente des deux");
}

#[test]
fn les_conversations_sont_rangees_de_la_plus_recente_a_la_plus_ancienne() {
    let racine = dossier("tri");
    let t = maintenant();
    for (nom, age) in [("a", 900), ("b", 60), ("c", 400)] {
        ecrire(&racine.join("p").join(format!("{nom}.jsonl")), &[assistant(t - age, nom, r"D:\p", 1, 1, 10_000)]);
    }
    let ordre: Vec<String> = lire(&racine).into_iter().map(|c| c.session).collect();
    assert_eq!(ordre, ["b", "c", "a"]);
}

/// Une ligne `custom-title` : le titre que la personne a donné à la conversation.
fn titre_ligne(session: &str, texte: &str) -> String {
    json!({"type": "custom-title", "customTitle": texte, "sessionId": session}).to_string()
}

#[test]
fn le_dernier_titre_donne_a_la_conversation_est_lu() {
    let racine = dossier("titre");
    let t = maintenant();
    ecrire(
        &racine.join("projet-a").join("a.jsonl"),
        &[
            utilisateur(t - 400, "a", Some("sdk")),
            titre_ligne("a", "Ancien nom"),
            assistant(t - 300, "a", r"D:\travail\projet-a", 1, 1, 50_000),
            titre_ligne("a", "  Notes de voyage  "),
            assistant(t - 100, "a", r"D:\travail\projet-a", 1, 1, 60_000),
        ],
    );
    ecrire(&racine.join("projet-a").join("b.jsonl"), &[utilisateur(t - 400, "b", Some("sdk")), assistant(t - 90, "b", r"D:\travail\projet-a", 1, 1, 40_000)]);
    let v = lire(&racine);
    let titre = |s: &str| v.iter().find(|c| c.session == s).unwrap().titre.clone();
    assert_eq!(titre("a"), Some("Notes de voyage".to_string()), "le DERNIER titre, sans les espaces autour");
    assert_eq!(titre("b"), None, "pas de titre : rien d'inventé");
    assert!(!format!("{v:?}").contains("JAMAIS LU"), "le contenu des messages reste ignoré");
}

#[test]
fn un_titre_cite_dans_un_message_n_est_pas_un_titre() {
    let racine = dossier("titre_cite");
    let t = maintenant();
    // un message de l'utilisateur dont le TEXTE contient une ligne custom-title : seul le type de premier niveau compte
    let citation = json!({"type": "user", "timestamp": iso(t - 350), "sessionId": "a", "cwd": "D:\\x",
        "message": {"role": "user", "content": "{\"type\":\"custom-title\",\"customTitle\":\"FAUX TITRE\"}"}})
    .to_string();
    // une ligne d'un autre type qui porte un champ customTitle (et le mot custom-title) : ignorée aussi
    let autre = json!({"type": "summary", "customTitle": "AUTRE FAUX", "note": "custom-title", "sessionId": "a"}).to_string();
    ecrire(&racine.join("p").join("a.jsonl"), &[citation, autre, assistant(t - 100, "a", r"D:\p", 1, 1, 50_000)]);
    assert_eq!(lire(&racine)[0].titre, None);
}

#[test]
fn un_titre_trop_long_est_coupe_et_un_titre_vide_est_ignore() {
    let racine = dossier("titre_long");
    let t = maintenant();
    let long: String = "é".repeat(200);
    ecrire(
        &racine.join("p").join("a.jsonl"),
        &[assistant(t - 300, "a", r"D:\p", 1, 1, 50_000), titre_ligne("a", &long), titre_ligne("a", "   "), titre_ligne("a", ""), assistant(t - 100, "a", r"D:\p", 1, 1, 60_000)],
    );
    let titre = lire(&racine)[0].titre.clone().expect("un titre");
    assert_eq!(titre.chars().count(), noyau::conversations::TITRE_MAX, "coupé par caractères, pas par octets");
    assert!(titre.chars().all(|c| c == 'é'), "les titres vides n'ont pas remplacé le titre précédent");
}

#[test]
fn le_titre_est_garde_quand_le_fichier_grandit_sans_nouveau_titre() {
    let racine = dossier("titre_cache");
    let t = maintenant();
    let chemin = racine.join("p").join("a.jsonl");
    ecrire(&chemin, &[utilisateur(t - 900, "a", Some("sdk")), titre_ligne("a", "Notes de voyage"), assistant(t - 800, "a", r"D:\p", 1, 1, 50_000)]);
    let mut lecteur = LecteurConversations::default();
    assert_eq!(lecteur.lire(&racine, maintenant()).conversations[0].titre.as_deref(), Some("Notes de voyage"));
    // plus de 64 Ko de réponses ensuite : le titre n'est plus dans la queue relue, mais il reste connu
    let beaucoup: Vec<String> = (0..300_i64).map(|i| assistant(t - 700 + i, "a", r"D:\p", 1, 1, 60_000 + i as u64)).collect();
    ajouter(&chemin, &beaucoup);
    assert!(fs::metadata(&chemin).unwrap().len() > 70_000, "le fichier dépasse la queue de 64 Ko : le titre n'y est plus");
    let apres = lecteur.lire(&racine, maintenant());
    assert_eq!(apres.conversations[0].titre.as_deref(), Some("Notes de voyage"));
    assert_eq!(apres.conversations[0].contexte, 1 + 1 + 60_000 + 299);
    // un nouveau titre le remplace
    ajouter(&chemin, &[titre_ligne("a", "Nouveau nom"), assistant(t - 50, "a", r"D:\p", 1, 1, 70_000)]);
    assert_eq!(lecteur.lire(&racine, maintenant()).conversations[0].titre.as_deref(), Some("Nouveau nom"));
}

/// Réserve C2 du contrôle de la 1.3.2 : `customTitle` n'est lu que sur une ligne `custom-title`. Une réponse de Claude (ou
/// n'importe quelle autre ligne) qui porterait un `customTitle` d'un AUTRE type que du texte faisait rejeter la ligne entière,
/// et sa conversation disparaissait sans alarme.
#[test]
fn un_custom_title_non_texte_sur_une_autre_ligne_ne_fait_pas_disparaitre_la_conversation() {
    let racine = dossier("titre_non_texte");
    let t = maintenant();
    for (nom, valeur) in [("nombre", json!(999)), ("liste", json!(["a", "b"])), ("objet", json!({"x": 1})), ("nul", Value::Null), ("booleen", json!(true))] {
        // la réponse de Claude elle-même porte un customTitle d'un type étranger
        let mut reponse = assistant_v(t - 100, "a", "claude-opus-5-5", r"D:\p", 1, 1, 50_000);
        reponse["customTitle"] = valeur.clone();
        ecrire(&racine.join("p").join(format!("{nom}.jsonl")), &[utilisateur(t - 400, "a", Some("sdk")), reponse.to_string()]);
        let lecture = LecteurConversations::default().lire(&racine, maintenant());
        assert_eq!(lecture.conversations.len(), 1, "{nom} : la conversation reste visible");
        assert_eq!(lecture.conversations[0].contexte, 1 + 1 + 50_000, "{nom} : ses chiffres aussi");
        assert_eq!(lecture.conversations[0].titre, None, "{nom} : une réponse n'est pas un titre");
        assert_eq!((lecture.candidats, lecture.lisibles), (1, 1), "{nom} : pas de faux « format incompris »");
        fs::remove_file(racine.join("p").join(format!("{nom}.jsonl"))).unwrap();
    }
}

/// Une ligne `custom-title` dont le `customTitle` n'est pas du texte : son titre est ignoré (le nom redevient « dossier · 4
/// caractères »), mais la conversation et ses chiffres sont intacts, et un titre valide qui suit est lu.
#[test]
fn une_ligne_custom_title_mal_formee_perd_son_titre_mais_pas_la_conversation() {
    let racine = dossier("titre_mal_forme");
    let t = maintenant();
    let mauvais = json!({"type": "custom-title", "customTitle": 999, "sessionId": "a"}).to_string();
    ecrire(&racine.join("p").join("a.jsonl"), &[assistant(t - 300, "a", r"D:\p", 1, 1, 50_000), mauvais.clone(), assistant(t - 100, "a", r"D:\p", 1, 1, 60_000)]);
    let v = lire(&racine);
    assert_eq!(v.len(), 1);
    assert_eq!((v[0].titre.clone(), v[0].contexte), (None, 1 + 1 + 60_000));
    // un bon titre APRÈS la ligne mal formée est lu
    ajouter(&racine.join("p").join("a.jsonl"), &[titre_ligne("a", "Notes de voyage"), assistant(t - 50, "a", r"D:\p", 1, 1, 61_000)]);
    assert_eq!(lire(&racine)[0].titre.as_deref(), Some("Notes de voyage"));
}

/// Masquer une conversation de bout en bout, sur de FAUX fichiers : la fausse alerte d'une conversation archivée puis
/// résumée disparaît avec elle ; seul son identifiant de session est rangé dans `conversations.txt` (ni le titre, ni le nom du
/// dossier, ni le contenu) ; un nouveau message ne la démasque pas ; l'identifiant s'oublie quand la session sort de la
/// fenêtre des 24 h, et le fichier ne grossit pas.
#[test]
fn masquer_une_conversation_de_bout_en_bout() {
    use noyau::reglages::{self, Reglages};
    use noyau::suivi::{self, Alerte};
    use std::collections::BTreeMap;

    let racine = dossier("masquer");
    let donnees = dossier("masquer_donnees");
    let t = maintenant();
    let archivee = racine.join("projet-archive").join("arch1.jsonl");
    let vivante = racine.join("projet-vivant").join("vivant2.jsonl");
    let marque = json!({"type": "system", "subtype": "compact_boundary", "timestamp": iso(t - 250), "sessionId": "arch1", "cwd": r"D:\travail\projet-archive"}).to_string();
    ecrire(
        &archivee,
        &[
            utilisateur(t - 500, "arch1", Some("human")),
            titre_ligne("arch1", "Titre PAS A ECRIRE"),
            assistant(t - 300, "arch1", r"D:\travail\projet-archive", 1, 1, 23_000),
            marque,
            assistant(t - 200, "arch1", r"D:\travail\projet-archive", 1, 1, 23_000),
        ],
    );
    ecrire(&vivante, &[utilisateur(t - 400, "vivant2", Some("human")), assistant(t - 100, "vivant2", r"D:\travail\projet-vivant", 1, 1, 50_000)]);
    let vus = BTreeMap::new();
    let mut r = Reglages::default();

    // avant : la conversation archivée « résumée » allume le rose
    let convs = lire(&racine);
    let s = suivi::calculer(&convs, &r, &vus, maintenant());
    assert_eq!((s.groupes[0].lignes.len(), s.alerte, s.masquees), (2, Alerte::Passation, 0));

    // le ✕ : elle disparaît de la liste et de l'alerte
    let ligne = s.groupes[0].lignes.iter().find(|l| l.dossier == "projet-archive").expect("la ligne archivée");
    assert!(r.masquer(&ligne.session));
    let s = suivi::calculer(&convs, &r, &vus, maintenant());
    assert_eq!((s.groupes[0].lignes.len(), s.alerte, s.masquees), (1, Alerte::Aucune, 1));
    assert_eq!(s.groupes[0].lignes[0].dossier, "projet-vivant");

    // le fichier : l'identifiant SEUL (ni le titre, ni le dossier, ni le contenu des messages)
    assert!(reglages::ecrire(&donnees, &r));
    let texte = fs::read_to_string(donnees.join("conversations.txt")).unwrap();
    assert!(texte.contains("masquee=arch1\n"), "{texte}");
    for interdit in ["Titre PAS A ECRIRE", "projet-archive", "projet-vivant", "JAMAIS LU", "vivant2"] {
        assert!(!texte.contains(interdit), "« {interdit} » ne doit pas être écrit : {texte}");
    }
    assert_eq!(reglages::lire(&donnees).masquees, ["arch1"]);

    // un nouveau message arrive dans la conversation masquée : elle reste masquée, sans alerte
    ajouter(&archivee, &[assistant(t - 20, "arch1", r"D:\travail\projet-archive", 1, 1, 30_000)]);
    let convs = lire(&racine);
    assert_eq!(convs.iter().find(|c| c.session == "arch1").map(|c| c.contexte), Some(30_002), "le nouveau message est bien lu");
    let s = suivi::calculer(&convs, &r, &vus, maintenant());
    assert_eq!((s.groupes[0].lignes.len(), s.alerte, s.masquees), (1, Alerte::Aucune, 1));

    // l'oubli : la lecture ne montre plus cette session (fichier disparu ou trop vieux) alors qu'une autre est lue
    fs::remove_file(&archivee).unwrap();
    let convs = lire(&racine);
    let presentes: Vec<&str> = convs.iter().map(|c| c.session.as_str()).collect();
    assert_eq!(presentes, ["vivant2"]);
    assert!(r.oublier_masquees_absentes(&presentes));
    assert!(r.masquees.is_empty());
    assert!(reglages::ecrire(&donnees, &r));
    assert!(!fs::read_to_string(donnees.join("conversations.txt")).unwrap().contains("masquee"), "le fichier a perdu l'identifiant");
}
