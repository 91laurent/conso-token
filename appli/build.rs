use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Thème sombre uniquement (choix de conception) : même style pour les éléments standard
    // (écran « À propos », champs de texte).
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/app.slint", config).expect("compilation de l'interface Slint");
    icone_de_l_exe();
}

/// L'icône du fichier .exe (Explorateur, barre des tâches épinglée) : `icone/icone.rc` est compilé par
/// `rc.exe` du kit Windows et lié au programme. Sans `rc.exe`, la compilation continue : l'exe n'a
/// alors pas d'icône, mais la fenêtre garde la sienne (`icon:` dans app.slint).
fn icone_de_l_exe() {
    println!("cargo:rerun-if-changed=icone/icone.rc");
    println!("cargo:rerun-if-changed=icone/icone.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let Some(rc) = trouver_rc() else {
        println!("cargo:warning=rc.exe introuvable : l'exe n'aura pas d'icône (la fenêtre garde la sienne)");
        return;
    };
    let res = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("icone.res");
    let ok = Command::new(rc).current_dir("icone").args(["/nologo", "/fo"]).arg(&res).arg("icone.rc").status().map(|s| s.success()).unwrap_or(false);
    if ok {
        println!("cargo:rustc-link-arg-bins={}", res.display());
    } else {
        println!("cargo:warning=rc.exe a échoué : l'exe n'aura pas d'icône");
    }
}

/// `rc.exe` : la variable `RC`, sinon le plus récent des kits Windows 10/11 installés.
fn trouver_rc() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("RC").map(PathBuf::from).filter(|p| p.is_file()) {
        return Some(p);
    }
    let racine = std::env::var_os("ProgramFiles(x86)").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("C:\\Program Files (x86)"));
    let bin = racine.join("Windows Kits").join("10").join("bin");
    let mut versions: Vec<PathBuf> = std::fs::read_dir(&bin).ok()?.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).collect();
    versions.sort();
    versions.iter().rev().map(|v| v.join("x64").join("rc.exe")).find(|p: &PathBuf| Path::is_file(p))
}
