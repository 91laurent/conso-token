// Conso Token : montre où en est la conso du forfait Claude, sans rien prédire.
// Un seul programme, deux usages :
//   conso_token.exe            ouvre la fenêtre ;
//   conso_token.exe --pont     reçoit les chiffres de Claude Code (ligne d'état) et les range.
// Aucun réseau, aucun jeton, aucun réglage lu : seulement ce que Claude Code envoie au pont et,
// pour les tokens par jour, l'horodatage et les compteurs des transcriptions (jamais leur contenu).
#![cfg_attr(windows, windows_subsystem = "windows")]

slint::include_modules!();

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod ecran;

use ecran::{brancher, maj_compteurs, rafraichir_conversations};
use noyau::conversations::{self, Lecture};
use noyau::langue::{self, Langue};
use noyau::modele::{self, Echantillon, Fenetre, Vue};
use noyau::reglages::{self, Reglages};
use noyau::suivi::{self, Suivi};
use noyau::temps::{JOUR_S, SEMAINE_S};
use noyau::textes::{Cle, fmt, t};
use noyau::{fr, pont, transcripts, vues};
use slint::{Color, ModelRc, SharedString, Timer, TimerMode, VecModel};

const OK: (u8, u8, u8) = (0x9c, 0xcf, 0xae);
const PAUSE: (u8, u8, u8) = (0xe0, 0xbd, 0x75);
const ERREUR: (u8, u8, u8) = (0xe4, 0x9a, 0x9a);
const OR_CLAIR: (u8, u8, u8) = (0xe6, 0xc9, 0x87);

fn couleur(c: (u8, u8, u8)) -> Color {
    Color::from_rgb_u8(c.0, c.1, c.2)
}

fn maintenant_s() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

struct Tokens {
    debut: i64,
    par_jour: [u64; 7],
}

/// Les trois valeurs animées de l'écran Semaine : chacune est une suite de morceaux (chiffres et texte)
/// que l'interface fait rouler, retourner ou compter.
struct Compteurs {
    rythme: Rc<VecModel<Glyphe>>,
    reste: Rc<VecModel<Glyphe>>,
    reset: Rc<VecModel<Glyphe>>,
    /// Le dernier texte posé pour chacune (rien à refaire s'il n'a pas changé).
    textes: RefCell<[String; 3]>,
    /// « Il te reste » : la dernière valeur, d'où repart le compte progressif.
    reste_valeur: Cell<Option<f64>>,
    minuteur_reste: Rc<Timer>,
    /// Vrai au départ et au retour sur l'onglet Semaine : les chiffres repartent de zéro.
    intro: Cell<bool>,
}

struct Contexte {
    donnees: PathBuf,
    demo: bool,
    tokens: Arc<Mutex<Option<Tokens>>>,
    /// La dernière lecture des conversations (faite toutes les minutes par le fil de lecture).
    conversations: Arc<Mutex<Option<Lecture>>>,
    reglages: RefCell<Reglages>,
    /// Le plus grand contexte vu par modèle (rangé dans `contextes_vus.txt`).
    vus: RefCell<BTreeMap<String, u64>>,
    /// Les conversations de moins de 24 h, mais de plus de 30 min : repliées tant qu'on n'a pas cliqué.
    recentes_ouvertes: Cell<bool>,
    /// Le pourcentage de l'exemple de la barre d'essai de chaque groupe (rien : à mi-chemin des seuils).
    essai: Cell<[Option<u8>; 2]>,
    /// Le dernier calcul de la page Conversations (la fenêtre de réglage en tire la liste des modèles).
    dernier_suivi: RefCell<Option<Suivi>>,
    /// Les dossiers de la fenêtre d'accueil et leur case (l'interface garde l'aspect, ici l'état).
    accueil: RefCell<Vec<(String, bool)>>,
    /// La fenêtre d'accueil s'est déjà ouverte toute seule pendant cette séance.
    accueil_montre: Cell<bool>,
    compteurs: Compteurs,
}

impl Contexte {
    /// Range les réglages (sauf en démonstration : rien n'est écrit).
    fn sauver(&self) {
        if !self.demo {
            let _ = reglages::ecrire(&self.donnees, &self.reglages.borrow());
        }
    }

    /// Les chiffres s'animent si la case de Config est cochée ET si Windows n'a pas coupé les effets d'animation.
    fn animer(&self) -> bool {
        self.reglages.borrow().animer && animations_windows()
    }
}

/// Le réglage Windows « effets d'animation » (Accessibilité) ; `CONSO_TOKEN_ANIMATIONS=0` ou `1` le force (captures).
fn animations_windows() -> bool {
    if let Ok(v) = std::env::var("CONSO_TOKEN_ANIMATIONS") {
        return v.trim() != "0";
    }
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn SystemParametersInfoW(action: u32, param: u32, pv: *mut core::ffi::c_void, win_ini: u32) -> i32;
        }
        const SPI_GETCLIENTAREAANIMATION: u32 = 0x1042;
        let mut actif: i32 = 1;
        // SAFETY : `actif` est un BOOL de 4 octets, ce que l'action demandée attend ; en cas d'échec on garde « oui ».
        let ok = unsafe { SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION, 0, (&mut actif as *mut i32).cast(), 0) };
        ok == 0 || actif != 0
    }
    #[cfg(not(windows))]
    {
        true
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a = |nom: &str| args.iter().any(|x| x == nom);
    let valeur = |nom: &str| args.iter().position(|x| x == nom).and_then(|i| args.get(i + 1)).cloned();
    let donnees = valeur("--donnees").map(PathBuf::from).unwrap_or_else(pont::dossier_donnees);

    if a("--pont") {
        pont_main(&donnees, &args);
        return;
    }

    // La langue : CONSO_TOKEN_LANGUE (essais, captures : rien n'est écrit) > langue.txt > langue de Windows > anglais.
    let langue_forcee = langue::depuis_env();
    langue::definir(langue::choisir(&donnees));

    let demo = a("--demo");
    let contexte = Rc::new(Contexte {
        donnees: donnees.clone(),
        demo,
        tokens: Arc::new(Mutex::new(None)),
        conversations: Arc::new(Mutex::new(None)),
        reglages: RefCell::new(reglages::lire(&donnees)),
        vus: RefCell::new(if demo { suivi::vus_demo() } else { reglages::lire_vus(&donnees) }),
        recentes_ouvertes: Cell::new(false),
        essai: Cell::new([None, None]),
        dernier_suivi: RefCell::new(None),
        accueil: RefCell::new(Vec::new()),
        accueil_montre: Cell::new(false),
        compteurs: Compteurs {
            rythme: Rc::new(VecModel::default()),
            reste: Rc::new(VecModel::default()),
            reset: Rc::new(VecModel::default()),
            textes: RefCell::new([String::new(), String::new(), String::new()]),
            reste_valeur: Cell::new(None),
            minuteur_reste: Rc::new(Timer::default()),
            intro: Cell::new(true),
        },
    });
    if demo {
        // des conversations d'exemple, pour voir la page (rien n'est lu dans les fichiers)
        let maintenant = maintenant_s();
        if let Ok(mut g) = contexte.conversations.lock() {
            *g = Some(Lecture { conversations: suivi::exemple(maintenant), candidats: 9, lisibles: 9 });
        }
    } else if !a("--sans-tokens") {
        lancer_lecture(donnees, racine_transcriptions(), Arc::clone(&contexte.tokens), Arc::clone(&contexte.conversations));
    }

    let Some(app) = creer_fenetre(&contexte.donnees, &args) else { return };
    app.set_version(env!("CARGO_PKG_VERSION").into());
    app.set_langue_index(langue::courante().index() as i32);
    app.set_rythme_glyphes(ModelRc::from(Rc::clone(&contexte.compteurs.rythme)));
    app.set_reste_glyphes(ModelRc::from(Rc::clone(&contexte.compteurs.reste)));
    app.set_reset_glyphes(ModelRc::from(Rc::clone(&contexte.compteurs.reset)));
    appliquer_textes(&app);
    brancher(&app, &contexte);
    // Pour les captures : CONSO_TOKEN_ONGLET=conversations ouvre l'onglet Conversations, CONSO_TOKEN_CONFIG=1 la fenêtre de réglage.
    if std::env::var("CONSO_TOKEN_ONGLET").is_ok_and(|v| v.trim().eq_ignore_ascii_case("conversations")) {
        app.set_onglet(1);
    }
    if std::env::var_os("CONSO_TOKEN_CONFIG").is_some() {
        app.set_cfg_ouverte(true);
    }
    if std::env::var_os("CONSO_TOKEN_AUTRES").is_some() {
        contexte.recentes_ouvertes.set(true);
    }
    let ne_pas_ecrire_langue = demo || langue_forcee.is_some();
    {
        let faible = app.as_weak();
        let ctx = Rc::clone(&contexte);
        app.on_langue_choisie(move |i| {
            let Some(l) = usize::try_from(i).ok().and_then(Langue::depuis_index) else { return };
            langue::definir(l);
            if !ne_pas_ecrire_langue {
                let _ = langue::ecrire_langue(&ctx.donnees, l);
            }
            if let Some(app) = faible.upgrade() {
                appliquer_textes(&app);
                rafraichir(&app, &ctx);
            }
        });
    }

    // La vue de la carte « jour par jour » : celle rangée dans vue.txt (batteries par défaut).
    // Pour les captures : CONSO_TOKEN_VUE=<0 à 4> la force, sans rien écrire dans vue.txt.
    let vue_forcee = std::env::var("CONSO_TOKEN_VUE").ok().and_then(|v| v.parse::<usize>().ok()).filter(|v| *v < vues::NB_VUES);
    app.set_vue(vue_forcee.unwrap_or_else(|| vues::lire_vue(&contexte.donnees)) as i32);
    let ne_pas_ecrire = demo || vue_forcee.is_some();
    let dossier = contexte.donnees.clone();
    app.on_vue_choisie(move |v| {
        if !ne_pas_ecrire && v >= 0 {
            let _ = vues::ecrire_vue(&dossier, v as usize);
        }
    });
    rafraichir(&app, &contexte);

    // les chiffres sont relus toutes les 15 secondes (lecture de deux petits fichiers)
    let faible = app.as_weak();
    let ctx15 = Rc::clone(&contexte);
    let minuteur = Timer::default();
    minuteur.start(TimerMode::Repeated, Duration::from_secs(15), move || {
        if let Some(app) = faible.upgrade() {
            rafraichir(&app, &ctx15);
        }
    });
    // Au départ, la première lecture des conversations peut prendre un instant (le fil de lecture parcourt
    // les transcriptions une fois en entier) : on regarde chaque seconde jusqu'à ce qu'elle soit là.
    let faible = app.as_weak();
    let ctx1 = Rc::clone(&contexte);
    let attente = Rc::new(Timer::default());
    let attente2 = Rc::clone(&attente);
    let essais = Cell::new(0u32);
    attente.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        essais.set(essais.get() + 1);
        let prete = ctx1.conversations.lock().is_ok_and(|g| g.is_some());
        if prete {
            if let Some(app) = faible.upgrade() {
                rafraichir_conversations(&app, &ctx1, maintenant_s());
            }
        }
        if prete || essais.get() >= 60 {
            attente2.stop();
        }
    });

    // Fermeture automatique pour les captures de contrôle : CONSO_TOKEN_AUTOQUIT=<secondes>.
    if let Some(s) = std::env::var("CONSO_TOKEN_AUTOQUIT").ok().and_then(|v| v.parse::<u64>().ok()) {
        Timer::single_shot(Duration::from_secs(s), || {
            let _ = slint::quit_event_loop();
        });
    }
    // Pour les captures : CONSO_TOKEN_APROPOS=1 ouvre l'écran « À propos » dès le départ.
    if std::env::var_os("CONSO_TOKEN_APROPOS").is_some() {
        app.set_apropos(true);
    }
    let _ = app.run();
    drop(minuteur);
}

/// Ouvre la fenêtre. Si la carte graphique pose problème, le programme se relance UNE fois en rendu
/// logiciel (`SLINT_BACKEND=winit-software`) ; si cela échoue aussi, il laisse un `erreur.txt`.
fn creer_fenetre(donnees: &Path, args: &[String]) -> Option<App> {
    if let Ok(app) = App::new() {
        return Some(app);
    }
    if std::env::var_os("SLINT_BACKEND").is_none() {
        if let Ok(exe) = std::env::current_exe() {
            let _ = std::process::Command::new(exe).args(args).env("SLINT_BACKEND", "winit-software").spawn();
            return None;
        }
    }
    let _ = std::fs::create_dir_all(donnees);
    let _ = std::fs::write(donnees.join("erreur.txt"), t(Cle::ErreurFenetre));
    None
}

/// Pousse les textes fixes de l'interface dans la langue en cours (au démarrage et à chaque changement de langue).
fn appliquer_textes(app: &App) {
    let g = app.global::<Textes>();
    let (texte, titre) = match langue::courante() {
        Langue::Zh => ("Microsoft YaHei", "Microsoft YaHei"),
        Langue::Ja => ("Yu Gothic", "Yu Gothic"),
        _ => ("Jost", "Cormorant Garamond"),
    };
    g.set_police_texte(texte.into());
    g.set_police_titre(titre.into());
    g.set_bouton_apropos(t(Cle::BoutonApropos).into());
    g.set_carte_rythme(t(Cle::CarteRythme).into());
    g.set_carte_reste(t(Cle::CarteReste).into());
    g.set_carte_reset(t(Cle::CarteReset).into());
    g.set_semaine_titre(t(Cle::SemaineTitre).into());
    g.set_titre_jours(t(Cle::TitreJours).into());
    g.set_onglet_aujourdhui(t(Cle::OngletAujourdhui).into());
    g.set_onglet_lignes(t(Cle::OngletLignes).into());
    g.set_onglet_trajectoire(t(Cle::OngletTrajectoire).into());
    g.set_onglet_batteries(t(Cle::OngletBatteries).into());
    g.set_onglet_semaine(t(Cle::OngletSemaine).into());
    g.set_a_venir(t(Cle::AVenir).into());
    g.set_mot_tokens(t(Cle::MotTokens).into());
    g.set_titre_reste(t(Cle::TitreReste).into());
    g.set_sous_reste(t(Cle::SousReste).into());
    g.set_branche_etape1(t(Cle::BrancheEtape1).into());
    g.set_branche_etape2(t(Cle::BrancheEtape2).into());
    g.set_branche_etape3(t(Cle::BrancheEtape3).into());
    g.set_apropos_titre(t(Cle::AproposTitre).into());
    g.set_apropos_corps(fmt(Cle::AproposCorps, &[&env!("CARGO_PKG_VERSION")]).into());
    g.set_apropos_slint(t(Cle::AproposSlint).into());
    g.set_apropos_polices(t(Cle::AproposPolices).into());
    g.set_bouton_retour(t(Cle::BoutonRetour).into());
    g.set_onglet_conversations(t(Cle::OngletConversations).into());
    g.set_bouton_config(t(Cle::BoutonConfig).into());
    g.set_conv_titre(t(Cle::ConvTitre).into());
    g.set_conv_sous_titre(t(Cle::ConvSousTitre).into());
    g.set_conv_legende(t(Cle::ConvLegende).into());
    g.set_config_titre(t(Cle::ConfigTitre).into());
    g.set_config_separer(t(Cle::ConfigSeparer).into());
    g.set_config_attention(t(Cle::ConfigAttention).into());
    g.set_config_passation(t(Cle::ConfigPassation).into());
    g.set_config_fenetre(t(Cle::ConfigFenetre).into());
    g.set_config_auto(t(Cle::ConfigAuto).into());
    g.set_config_fenetre_note(t(Cle::ConfigFenetreNote).into());
    g.set_config_alerte(t(Cle::ConfigAlerte).into());
    g.set_config_animer(t(Cle::ConfigAnimer).into());
    g.set_config_pied(t(Cle::ConfigPied).into());
    g.set_config_retablir(t(Cle::ConfigRetablir).into());
    g.set_config_retablir_confirmer(t(Cle::ConfigRetablirConfirmer).into());
    g.set_accueil_titre(t(Cle::AccueilTitre).into());
    g.set_accueil_texte(t(Cle::AccueilTexte).into());
    g.set_accueil_dossiers(t(Cle::AccueilDossiers).into());
    g.set_accueil_nom1(t(Cle::AccueilNom1).into());
    g.set_accueil_nom2(t(Cle::AccueilNom2).into());
    g.set_accueil_garder(t(Cle::AccueilGarder).into());
    g.set_config_groupes(t(Cle::ConfigGroupes).into());
    g.set_conv_masquer(t(Cle::ConvMasquer).into());
    g.set_config_reafficher(t(Cle::ConfigReafficher).into());
    g.set_fenetre_petite(suivi::entier(reglages::FENETRE_PETITE).into());
    g.set_fenetre_grande(suivi::entier(reglages::FENETRE_GRANDE).into());
}

// ---------------------------------------------------------------- le pont

/// `--pont` : lit le JSON de la ligne d'état sur l'entrée standard et le range. Silencieux : il
/// ne rend jamais d'erreur à Claude Code. `--pont --suite <commande…>` transmet ensuite le même
/// JSON à une ligne d'état déjà en place et rend sa sortie, pour s'y ajouter sans la remplacer.
fn pont_main(donnees: &Path, args: &[String]) {
    let mut entree = String::new();
    let _ = std::io::stdin().take(2_000_000).read_to_string(&mut entree);
    if let Some(e) = pont::lire_json(&entree, maintenant_s()) {
        let _ = pont::enregistrer(donnees, &e);
    }
    if let Some(i) = args.iter().position(|x| x == "--suite") {
        let commande = &args[i + 1..];
        if commande.iter().any(|a| !a.trim().is_empty()) {
            suite(commande, &entree);
        }
    }
}

/// Lance la ligne d'état déjà en place (`--suite <commande…>`) avec le même JSON, et rend sa sortie.
/// Elle passe par `cmd`, pas par Git Bash comme chez Claude Code : pour un script bash, donner le
/// chemin complet du `bash.exe` de Git en premier (voir LISEZ_MOI). Délai maximum de 8 secondes.
#[cfg(windows)]
fn suite(args: &[String], entree: &str) {
    use std::io::Write;
    use std::os::windows::process::CommandExt;
    use std::process::{Command, Stdio};
    use std::time::Instant;
    const SANS_FENETRE: u32 = 0x0800_0000;
    // Le shell de Claude Code a déjà découpé les arguments : on remet des guillemets autour de
    // ceux qui contiennent un espace (un chemin « Program Files »).
    let ligne = args
        .iter()
        .map(|a| if a.is_empty() || a.contains(' ') { format!("\"{a}\"") } else { a.clone() })
        .collect::<Vec<_>>()
        .join(" ");
    let racine = std::env::var_os("SystemRoot").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("C:\\Windows"));
    let Ok(mut enfant) = Command::new(racine.join("System32").join("cmd.exe"))
        .raw_arg(format!("/S /C \"{ligne}\""))
        .creation_flags(SANS_FENETRE)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return;
    };
    let Some(mut sortie) = enfant.stdout.take() else { return };
    let lecteur = std::thread::spawn(move || {
        let mut v = Vec::new();
        let _ = sortie.read_to_end(&mut v);
        v
    });
    if let Some(mut stdin) = enfant.stdin.take() {
        let _ = stdin.write_all(entree.as_bytes());
    }
    let limite = Instant::now() + Duration::from_secs(8);
    loop {
        match enfant.try_wait() {
            Ok(Some(_)) | Err(_) => break,
            Ok(None) if Instant::now() > limite => {
                let _ = enfant.kill();
                return; // trop long : rien n'est rendu (le fil de lecture s'arrête avec le tube)
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
        }
    }
    if let Ok(v) = lecteur.join() {
        let _ = std::io::stdout().write_all(&v);
    }
}

#[cfg(not(windows))]
fn suite(_args: &[String], _entree: &str) {}

// ---------------------------------------------------------------- les tokens

fn racine_transcriptions() -> PathBuf {
    let base = std::env::var_os("CLAUDE_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(|u| PathBuf::from(u).join(".claude")))
        .unwrap_or_else(|| PathBuf::from(".claude"));
    base.join("projects")
}

/// Relit les transcriptions toutes les minutes, hors de l'interface : les tokens par jour (si on connaît
/// la semaine) et les conversations (contexte au dernier échange).
fn lancer_lecture(donnees: PathBuf, racine: PathBuf, sortie: Arc<Mutex<Option<Tokens>>>, conversations: Arc<Mutex<Option<Lecture>>>) {
    std::thread::spawn(move || {
        let mut lecteur = transcripts::Lecteur::default();
        let mut lecteur_conv = conversations::LecteurConversations::default();
        loop {
            let (histo, dernier) = pont::charger(&donnees);
            let mut tous = histo;
            tous.extend(dernier);
            if let Some(semaine) = modele::semaines(&tous).last() {
                let debut = semaine.reset - SEMAINE_S;
                let par_jour = lecteur.tokens_par_jour(&racine, debut);
                if let Ok(mut g) = sortie.lock() {
                    *g = Some(Tokens { debut, par_jour });
                }
            }
            let lecture = lecteur_conv.lire(&racine, maintenant_s());
            if let Ok(mut g) = conversations.lock() {
                *g = Some(lecture);
            }
            std::thread::sleep(Duration::from_secs(60));
        }
    });
}

// ---------------------------------------------------------------- la démonstration

/// Semaine de l'exemple du 14/10/2026 à 05:48 (données factices, sauf 64 % et 81,4 % qui sont réels).
/// Pour les captures : CONSO_TOKEN_DEMO=debut ou milieu donne les deux autres semaines d'essai des
/// maquettes (jour 1 ; jour 4 avec un jour dépassé) ; sans la variable, c'est la fin de semaine.
fn donnees_demo() -> (Vec<Echantillon>, Option<Echantillon>, i64, [u64; 7]) {
    // CONSO_TOKEN_DEMO=hiver / mars : la semaine de la carte « Ton forfait revient » qui contient le
    // passage à l'heure d'hiver (finit jeudi 29/10 à 12:00 à Paris) / d'été (finit jeudi 01/04/2027 à 13:00).
    let demo = std::env::var("CONSO_TOKEN_DEMO").unwrap_or_default();
    let (reset, variante) = match demo.as_str() {
        "hiver" => (1_793_271_600, "milieu"),
        "mars" => (1_806_577_200, "milieu"),
        "debut" | "milieu" => (1_792_062_000, demo.as_str()),
        _ => (1_792_062_000, ""), // jeudi 15/10/2026 à 13:00 (Paris)
    };
    let debut = reset - SEMAINE_S;
    // % consommé au début de chaque jour jusqu'à aujourd'hui, % total maintenant, secondes écoulées
    let (au_debut_du_jour, utilise, ecoule_s, tokens): (&[f64], f64, i64, [u64; 7]) = match variante {
        "debut" => (&[0.0], 6.0, 12 * 3_600, [1_500_000, 0, 0, 0, 0, 0, 0]),
        "milieu" => (&[0.0, 18.0, 27.0, 40.0], 45.0, 3 * JOUR_S + 2 * JOUR_S / 5, [3_800_000, 2_000_000, 2_900_000, 1_100_000, 0, 0, 0]),
        _ => (&[0.0, 9.0, 19.0, 27.0, 39.0, 54.0], 64.0, 136 * 3_600 + 48 * 60, [2_400_000, 2_700_000, 2_100_000, 3_200_000, 4_000_000, 2_700_000, 0]),
    };
    let maintenant = debut + ecoule_s;
    let histo = au_debut_du_jour
        .iter()
        .enumerate()
        .map(|(i, p)| Echantillon { t: debut + i as i64 * JOUR_S, cinq: None, sept: Some(Fenetre { p: *p, r: reset }) })
        .collect();
    let dernier = Echantillon { t: maintenant - 120, cinq: None, sept: Some(Fenetre { p: utilise, r: reset }) };
    (histo, Some(dernier), maintenant, tokens)
}

// ---------------------------------------------------------------- l'affichage

/// Tout remettre à jour : l'écran Semaine, puis la page Conversations (et le point clignotant, la fenêtre de réglage).
fn rafraichir(app: &App, ctx: &Contexte) {
    app.set_animer(ctx.animer());
    rafraichir_semaine(app, ctx);
    rafraichir_conversations(app, ctx, maintenant_s());
}

fn rafraichir_semaine(app: &App, ctx: &Contexte) {
    let (histo, dernier, maintenant, tokens_demo) = if ctx.demo {
        let (h, d, m, t) = donnees_demo();
        (h, d, m, Some(t))
    } else {
        let (h, d) = pont::charger(&ctx.donnees);
        (h, d, maintenant_s(), None)
    };
    let vu_global = histo.iter().map(|e| e.t).chain(dernier.iter().map(|e| e.t)).max();
    let branche = dernier.is_some();
    let mut tous = histo;
    tous.extend(dernier);
    let semaines = modele::semaines(&tous);

    let Some(semaine) = semaines.last() else {
        page_accueil(app, if branche { Accueil::SansChiffres } else { Accueil::PremiereFois });
        source(app, vu_global, maintenant, ctx.demo);
        return;
    };
    let debut_semaine = semaine.reset - SEMAINE_S;
    let tokens: Option<[u64; 7]> = tokens_demo.or_else(|| {
        let g = ctx.tokens.lock().ok()?;
        let t = g.as_ref()?;
        ((t.debut - debut_semaine).abs() < 3_600).then_some(t.par_jour)
    });
    match modele::construire(semaine, maintenant, tokens.as_ref()) {
        Some(vue) => {
            afficher(app, ctx, &vue, tokens.is_some());
            app.set_page(0);
        }
        None => page_accueil(app, Accueil::NouvelleSemaine),
    }
    source(app, vu_global, maintenant, ctx.demo);
}

/// L'en-tête : l'âge des chiffres, avec un point vert, jaune ou rose.
fn source(app: &App, vu: Option<i64>, maintenant: i64, demo: bool) {
    if demo {
        app.set_source_texte(t(Cle::SourceDemo).into());
        app.set_source_couleur(couleur(PAUSE));
        return;
    }
    match vu {
        Some(t) => {
            let age = (maintenant - t).max(0);
            app.set_source_texte(fmt(Cle::SourceAge, &[&fr::age(age)]).into());
            app.set_source_couleur(couleur(if age <= 30 * 60 {
                OK
            } else if age <= 3 * 3_600 {
                PAUSE
            } else {
                ERREUR
            }));
        }
        None => {
            app.set_source_texte(t(Cle::SourceAucun).into());
            app.set_source_couleur(couleur(ERREUR));
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Accueil {
    /// Aucune lecture, jamais : il faut brancher le pont.
    PremiereFois,
    /// Le pont a été appelé, mais le compte ne donne pas la fenêtre de 7 jours.
    SansChiffres,
    /// La semaine est finie et la suivante n'a pas encore de lecture.
    NouvelleSemaine,
}

fn page_accueil(app: &App, cas: Accueil) {
    app.set_page(1);
    // Conso Token se nourrit de l'appli Claude de bureau (tâche planifiée « conso-token-feed ») :
    // plus de page « branche la ligne d'état », elle ne concerne pas ceux qui n'utilisent pas le terminal.
    app.set_montrer_branchement(false);
    let (titre, message) = match cas {
        Accueil::SansChiffres => (Cle::AccueilSansTitre, Cle::AccueilSansMessage),
        Accueil::PremiereFois => (Cle::AccueilPremiereTitre, Cle::AccueilPremiereMessage),
        Accueil::NouvelleSemaine => (Cle::AccueilNouvelleTitre, Cle::AccueilNouvelleMessage),
    };
    app.set_accueil_titre(t(titre).into());
    app.set_accueil_message(t(message).into());
}

fn pourcent(x: f64) -> String {
    fr::pourcent(x)
}

fn afficher(app: &App, ctx: &Contexte, v: &Vue, tokens_connus: bool) {
    // --- les trois cartes
    let ecart = v.ecart_rythme;
    let (valeur, tag, c) = if ecart.abs() < 0.5 {
        (t(Cle::RythmeDans).to_string(), t(Cle::TagDans), OR_CLAIR)
    } else if ecart < 0.0 {
        (fmt(Cle::RythmeAvance, &[&(ecart.abs().round() as i64)]), t(Cle::TagAvance), OK)
    } else {
        (fmt(Cle::RythmeTrop, &[&(ecart.round() as i64)]), t(Cle::TagTrop), ERREUR)
    };
    maj_compteurs(ctx, &valeur, &pourcent(v.reste), v.reste, &fr::duree(v.reste_s));
    app.set_rythme_tag(tag.into());
    app.set_rythme_couleur(couleur(c));
    app.set_rythme_detail(fmt(Cle::RythmeDetail, &[&pourcent(v.utilise.round()), &pourcent(v.ecoule.round())]).into());
    app.set_reste_detail(t(Cle::ResteDetail).into());
    app.set_reset_detail(fr::carte_reset_detail(v.reset).into());
    app.set_reset_avis(fr::carte_reset_avis(v.debut, v.reset).unwrap_or_default().into());

    // --- la semaine d'un coup d'œil
    app.set_utilise(v.utilise as f32);
    app.set_ecoule(v.ecoule as f32);
    app.set_semaine_texte(fmt(Cle::SemaineTexte, &[&pourcent(v.utilise), &pourcent(v.ecoule)]).into());

    // --- jour par jour : les cinq vues (calculées dans noyau::vues, à partir des mêmes chiffres)
    app.set_puce(vues::puce(v).into());
    let jours: Vec<JourVue> = vues::jours_carte(v)
        .into_iter()
        .map(|j| JourVue {
            nom: j.nom.into(),
            genre: j.genre,
            depasse: j.depasse,
            valeur: j.valeur.into(),
            court: j.court.into(),
            sous: j.sous.into(),
            ligne: j.ligne.into(),
            tokens: j.tokens.into(),
            niveau: j.niveau,
            conso_part: j.conso_part,
            exces_part: j.exces_part,
            repere: j.repere,
        })
        .collect();
    app.set_jours(ModelRc::new(VecModel::from(jours)));
    let aides: Vec<SharedString> = vues::aides(v, tokens_connus).into_iter().map(SharedString::from).collect();
    app.set_aides(ModelRc::new(VecModel::from(aides)));

    let a = vues::aujourdhui(v);
    app.set_v1_arc(a.arc.into());
    app.set_v1_depasse(a.depasse);
    app.set_v1_etiquette(a.etiquette.into());
    app.set_v1_valeur(a.valeur.into());
    app.set_v1_sous(a.sous.into());
    app.set_v1_resume(a.resume.into());

    let t = vues::trajectoire(v);
    app.set_v3_aire_avance(t.aire_avance.into());
    app.set_v3_aire_retard(t.aire_retard.into());
    app.set_v3_rythme(t.rythme.into());
    app.set_v3_cumul(t.cumul.into());
    app.set_v3_permis(t.permis.into());
    app.set_v3_texte(t.texte.into());
    app.set_v3_en_ce_moment(t.en_ce_moment.into());
    app.set_v3_x(t.x);
    app.set_v3_y(t.y);

    let s = vues::semaine(v);
    let segments: Vec<SegVue> = s
        .segments
        .into_iter()
        .map(|g| SegVue { debut: g.debut, part: g.part, genre: g.genre, texte: g.texte.into(), etiquette: g.etiquette.into() })
        .collect();
    app.set_segments(ModelRc::new(VecModel::from(segments)));
    app.set_repere_semaine(s.repere);
    app.set_repere_semaine_texte(s.repere_texte.into());
    app.set_semaine_resume(s.resume.into());

    // --- ce qu'il reste à consommer
    let fin_du_jour = v.jours[v.aujourdhui].debut + JOUR_S;
    app.set_reste1_nom(fmt(Cle::Reste1Nom, &[&fr::jour_long_heure(fin_du_jour)]).into());
    app.set_reste1_texte(pourcent(v.reste_jour).into());
    app.set_reste1(v.reste_jour as f32);
    let mut plus_grand = v.reste_jour;
    match v.reste_suivants {
        Some(part) => {
            plus_grand = plus_grand.max(part);
            app.set_reste2_visible(true);
            app.set_reste2_nom(
                if v.nb_suivants == 1 {
                    fmt(Cle::Reste2Dernier, &[&fr::jour_long_heure(fin_du_jour), &fr::jour_long_heure(v.reset)])
                } else if v.reste_jour > 0.0 {
                    fmt(Cle::Reste2NReport, &[&v.nb_suivants])
                } else {
                    fmt(Cle::Reste2N, &[&v.nb_suivants])
                }
                .into(),
            );
            app.set_reste2_texte(pourcent(part).into());
            app.set_reste2(part as f32);
        }
        None => app.set_reste2_visible(false),
    }
    app.set_reste_maxi((plus_grand.max(1.0) * 1.1) as f32);
    let resume = match v.reste_suivants {
        Some(part) => {
            let cle = match (v.nb_suivants == 1, v.reste_jour <= 0.0) {
                (true, false) => Cle::ResumeDernierOk,
                (true, true) => Cle::ResumeDernierDepasse,
                (false, false) => Cle::ResumeNOk,
                (false, true) => Cle::ResumeNDepasse,
            };
            fmt(cle, &[&pourcent(v.reste), &pourcent(v.reste_jour), &pourcent(part), &v.nb_suivants])
        }
        None => fmt(Cle::ResumeFinal, &[&pourcent(v.reste)]),
    };
    app.set_reste_resume(SharedString::from(resume));
}
