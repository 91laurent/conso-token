// Les nouveautés de la version 1.3.0 côté fenêtre : les compteurs animés de l'écran Semaine, la page
// « Conversations », la fenêtre de réglage (Config) et le câblage de leurs boutons. Les calculs sont
// dans `noyau` (suivi, conversations, reglages) ; ici on ne fait que pousser le résultat dans l'interface.

use std::rc::Rc;
use std::time::{Duration, Instant};

use noyau::conversations::{Conversation, Lecture, pour_affichage};
use noyau::reglages::{self, ChoixFenetre, Reglages};
use noyau::suivi::{self, Alerte, Suivi};
use noyau::textes::{Cle, fmt, t};
use slint::{ComponentHandle, Model, ModelRc, SharedString, Timer, TimerMode, VecModel};

use crate::{App, Contexte, DossierCfg, Glyphe, LigneConv, ModeleCfg, maintenant_s, pourcent, rafraichir};

// ---------------------------------------------------------------- les compteurs animés (écran Semaine)

#[derive(Clone, Copy, PartialEq)]
enum Forme {
    /// Un seul morceau de texte (animations coupées, ou compte progressif joué par le programme).
    Simple,
    /// Des chiffres isolés et des bouts de texte : l'interface fait rouler ou retourner les chiffres.
    Chiffres,
    /// Comme `Chiffres`, mais tous les chiffres à zéro : le point de départ de l'ouverture.
    Zeros,
}

/// Découpe un texte en morceaux : chaque chiffre isolé, et les suites de signes entre eux.
fn glyphes(texte: &str, forme: Forme) -> Vec<Glyphe> {
    if forme == Forme::Simple {
        return vec![Glyphe { texte: texte.into(), chiffre: false, valeur: 0, rang: 0 }];
    }
    let total = texte.chars().filter(char::is_ascii_digit).count();
    let (mut sortie, mut fixe, mut vus) = (Vec::new(), String::new(), 0usize);
    for c in texte.chars() {
        if c.is_ascii_digit() {
            if !fixe.is_empty() {
                sortie.push(Glyphe { texte: std::mem::take(&mut fixe).into(), chiffre: false, valeur: 0, rang: 0 });
            }
            vus += 1;
            let d = c.to_digit(10).unwrap_or(0) as i32;
            sortie.push(Glyphe { texte: c.to_string().into(), chiffre: true, valeur: if forme == Forme::Zeros { 0 } else { d }, rang: (total - vus) as i32 });
        } else {
            fixe.push(c);
        }
    }
    if !fixe.is_empty() {
        sortie.push(Glyphe { texte: fixe.into(), chiffre: false, valeur: 0, rang: 0 });
    }
    sortie
}

/// Met les morceaux dans le modèle. Même découpage : on ne change que les chiffres qui ont changé, ce
/// qui les fait rouler ou se retourner ; sinon le modèle est refait (rien ne bouge, tout est à sa place).
fn poser(modele: &VecModel<Glyphe>, nouveaux: Vec<Glyphe>) {
    let meme_decoupage = modele.row_count() == nouveaux.len()
        && nouveaux.iter().enumerate().all(|(i, n)| modele.row_data(i).is_some_and(|a| a.chiffre == n.chiffre && (n.chiffre || a.texte == n.texte)));
    if meme_decoupage {
        for (i, n) in nouveaux.into_iter().enumerate() {
            if modele.row_data(i).is_some_and(|a| a.valeur != n.valeur || a.rang != n.rang || a.texte != n.texte) {
                modele.set_row_data(i, n);
            }
        }
    } else {
        modele.set_vec(nouveaux);
    }
}

/// « Il te reste » : le chiffre compte de l'ancienne à la nouvelle valeur en un peu plus d'une seconde.
fn compter(modele: Rc<VecModel<Glyphe>>, minuteur: Rc<Timer>, depuis: f64, vers: f64, final_: String) {
    let t0 = Instant::now();
    let arret = Rc::clone(&minuteur);
    minuteur.start(TimerMode::Repeated, Duration::from_millis(33), move || {
        let p = (t0.elapsed().as_secs_f64() / 1.1).min(1.0);
        if p >= 1.0 {
            poser(&modele, glyphes(&final_, Forme::Simple));
            arret.stop();
        } else {
            let e = 1.0 - (1.0 - p).powi(3);
            poser(&modele, glyphes(&pourcent((depuis + (vers - depuis) * e).round()), Forme::Simple));
        }
    });
}

/// Met à jour les trois valeurs de l'écran Semaine. « Ton rythme » roule (odomètre), « Il te reste » compte,
/// « Ton forfait revient » se retourne (volets). À l'ouverture (et au retour sur l'onglet) tout repart de
/// zéro ; ensuite seuls les chiffres qui changent bougent. Sans animation, les valeurs sont posées d'un coup.
pub fn maj_compteurs(ctx: &Contexte, rythme: &str, reste_texte: &str, reste: f64, reset: &str) {
    let c = &ctx.compteurs;
    let animer = ctx.animer();
    let intro = c.intro.replace(false);
    let finale = if animer { Forme::Chiffres } else { Forme::Simple };
    let mut anciens = c.textes.borrow_mut();
    for (idx, modele, texte) in [(0usize, &c.rythme, rythme), (2, &c.reset, reset)] {
        if !intro && anciens[idx] == texte {
            continue;
        }
        anciens[idx] = texte.to_string();
        if animer && intro {
            poser(modele, glyphes(texte, Forme::Zeros));
            let (m, t) = (Rc::clone(modele), texte.to_string());
            Timer::single_shot(Duration::from_millis(80), move || poser(&m, glyphes(&t, Forme::Chiffres)));
        } else {
            poser(modele, glyphes(texte, finale));
        }
    }
    if intro || anciens[1] != reste_texte {
        anciens[1] = reste_texte.to_string();
        let depuis = if intro { 0.0 } else { c.reste_valeur.get().unwrap_or(reste) };
        c.reste_valeur.set(Some(reste));
        c.minuteur_reste.stop();
        if animer && (depuis - reste).abs() >= 0.5 {
            poser(&c.reste, glyphes(&pourcent(depuis.round()), Forme::Simple));
            compter(Rc::clone(&c.reste), Rc::clone(&c.minuteur_reste), depuis, reste, reste_texte.to_string());
        } else {
            poser(&c.reste, glyphes(reste_texte, Forme::Simple));
        }
    }
}

// ---------------------------------------------------------------- la page « Conversations »

const H_GROUPE: f32 = 46.0;
const H_LIGNE: f32 = 54.0;
const H_MESSAGE: f32 = 42.0;
const H_AUTRES: f32 = 36.0;

fn ligne_vide() -> LigneConv {
    LigneConv {
        genre: 0,
        y: 0.0,
        h: 0.0,
        titre: SharedString::new(),
        sous: SharedString::new(),
        etiquette: SharedString::new(),
        camp: 0,
        dossier: SharedString::new(),
        nom: SharedString::new(),
        meta: SharedString::new(),
        attention: 0.0,
        passation: 0.0,
        pct: 0.0,
        texte_pct: SharedString::new(),
        tokens: SharedString::new(),
        reste: SharedString::new(),
        etat: 0,
        recente: false,
        ouvert: false,
        session: SharedString::new(),
    }
}

fn ligne_conv(l: &suivi::LigneSuivi, recente: bool, y: f32) -> LigneConv {
    LigneConv {
        genre: 1,
        y,
        h: H_LIGNE,
        session: l.session.clone().into(),
        etiquette: l.etiquette.clone().into(),
        camp: l.groupe as i32,
        dossier: l.dossier.clone().into(),
        nom: l.nom.clone().into(),
        meta: l.meta.clone().into(),
        attention: l.seuils.attention as f32,
        passation: l.seuils.passation as f32,
        pct: l.pct.clamp(0.0, 100.0) as f32,
        texte_pct: l.texte_pct.clone().into(),
        tokens: l.tokens.clone().into(),
        reste: l.reste.clone().into(),
        etat: l.etat.numero(),
        recente,
        ..ligne_vide()
    }
}

/// Les lignes de la liste (titres de groupe, conversations, message, « + N autres ») et la hauteur totale.
/// Pas encore de lecture : la liste reste vide. Format des transcriptions incompris : un seul message.
fn construire_lignes(s: &Suivi, ouvert: bool, lecture: Option<&Lecture>) -> (Vec<LigneConv>, f32) {
    let mut lignes = Vec::new();
    let mut y = 0.0f32;
    let Some(lecture) = lecture else { return (lignes, y) };
    if lecture.format_incompris() {
        lignes.push(LigneConv { genre: 2, y, h: 64.0, titre: t(Cle::ConvIndispo).into(), ..ligne_vide() });
        return (lignes, y + 64.0);
    }
    for g in &s.groupes {
        lignes.push(LigneConv { genre: 0, y, h: H_GROUPE, titre: g.nom.clone().into(), sous: g.texte_seuils.clone().into(), ..ligne_vide() });
        y += H_GROUPE;
        if g.lignes.is_empty() {
            lignes.push(LigneConv { genre: 2, y, h: H_MESSAGE, titre: t(Cle::ConvVide).into(), ..ligne_vide() });
            y += H_MESSAGE;
        }
        for l in &g.lignes {
            lignes.push(ligne_conv(l, false, y));
            y += H_LIGNE;
        }
    }
    if !s.recentes.is_empty() {
        lignes.push(LigneConv { genre: 3, y: y + 10.0, h: H_AUTRES, titre: fmt(Cle::ConvAutres, &[&s.recentes.len()]).into(), ouvert, ..ligne_vide() });
        y += 10.0 + H_AUTRES;
        if ouvert {
            for l in &s.recentes {
                lignes.push(ligne_conv(l, true, y));
                y += H_LIGNE;
            }
        }
    }
    (lignes, y + 8.0)
}

/// Recalcule la page Conversations à partir de la dernière lecture et des réglages, puis le point clignotant
/// et la fenêtre de réglage.
pub fn rafraichir_conversations(app: &App, ctx: &Contexte, maintenant: i64) {
    let lecture = ctx.conversations.lock().ok().and_then(|g| g.clone());
    let convs: Vec<Conversation> = lecture.as_ref().map(|l| l.conversations.clone()).unwrap_or_default();
    // les conversations masquées dont la session n'est plus dans la fenêtre des 24 h sont oubliées (le fichier ne grossit pas) ;
    // rien n'est oublié sans lecture, avec une lecture vide ou si le format des transcriptions est incompris
    if lecture.as_ref().is_some_and(|l| !l.format_incompris()) {
        let presentes: Vec<&str> = convs.iter().map(|c| c.session.as_str()).collect();
        let change = ctx.reglages.borrow_mut().oublier_masquees_absentes(&presentes);
        if change {
            ctx.sauver();
        }
    }
    let r = ctx.reglages.borrow().clone();
    let s = suivi::calculer(&convs, &r, &ctx.vus.borrow(), maintenant);
    // le plus grand contexte vu par modèle : rangé dès qu'il change (jamais en démonstration)
    if s.vus != *ctx.vus.borrow() {
        if !ctx.demo {
            let _ = reglages::ecrire_vus(&ctx.donnees, &s.vus);
        }
        *ctx.vus.borrow_mut() = s.vus.clone();
    }
    let (lignes, total) = construire_lignes(&s, ctx.recentes_ouvertes.get(), lecture.as_ref());
    app.set_lignes_conv(ModelRc::new(VecModel::from(lignes)));
    app.set_conv_total(total);
    // la lecture est incomprise : pas de faux chiffres, pas d'alerte
    let incompris = lecture.as_ref().is_some_and(Lecture::format_incompris);
    app.set_alerte_niveau(match (s.alerte, incompris) {
        (_, true) | (Alerte::Aucune, _) => 0,
        (Alerte::Attention, _) => 1,
        (Alerte::Passation, _) => 2,
    });
    *ctx.dernier_suivi.borrow_mut() = Some(s);
    appliquer_reglages(app, ctx);
    proposer_accueil(app, ctx);
}

/// La fenêtre d'accueil (« Choisir les groupes… », qui désigne les dossiers du groupe 1 et nomme les groupes) : elle
/// s'ouvre toute seule UNE fois, à la première ouverture de la page Conversations et s'il y a des dossiers à proposer
/// (pas en démonstration, sauf `CONSO_TOKEN_ACCUEIL=1`, pour les captures). Ensuite elle ne revient jamais seule : le
/// bouton de la page Conversations et celui de Config la rouvrent à tout moment (ils sont toujours là).
fn proposer_accueil(app: &App, ctx: &Contexte) {
    let possible = ctx.dernier_suivi.borrow().as_ref().is_some_and(|s| !suivi::dossiers_a_choisir(s, &ctx.reglages.borrow()).is_empty());
    let forcer = std::env::var("CONSO_TOKEN_ACCUEIL").is_ok_and(|v| v.trim() == "1");
    let a_montrer = if ctx.demo { forcer } else { forcer || !ctx.reglages.borrow().accueil_vu };
    if possible && a_montrer && app.get_onglet() == 1 && !ctx.accueil_montre.get() && !app.get_accueil_ouverte() && !app.get_cfg_ouverte() {
        ouvrir_accueil(app, ctx);
    }
}

/// Remplit la liste de la fenêtre d'accueil (les dossiers qu'on voit, cochés s'ils sont déjà dans le groupe 1)
/// et l'ouvre. La liste ne change plus tant que la fenêtre est ouverte : les cases cochées ne sautent pas.
fn ouvrir_accueil(app: &App, ctx: &Contexte) {
    let liste = {
        let s = ctx.dernier_suivi.borrow();
        let r = ctx.reglages.borrow();
        // pas encore de lecture : au moins les dossiers déjà rangés dans le groupe 1 (valider une liste vide les effacerait)
        match s.as_ref() {
            Some(s) => suivi::dossiers_a_choisir(s, &r),
            None => suivi::dossiers_sans_lecture(&r),
        }
    };
    // le nom MONTRÉ est nettoyé (U+202E, retours à la ligne…) ; la liste gardée à côté (`ctx.accueil`) garde le nom exact
    let modele: Vec<DossierCfg> = liste.iter().map(|(d, c)| DossierCfg { nom: pour_affichage(d).into(), coche: *c }).collect();
    *ctx.accueil.borrow_mut() = liste;
    app.set_accueil_dossiers(ModelRc::new(VecModel::from(modele)));
    {
        let r = ctx.reglages.borrow();
        app.set_accueil_nom1(r.noms[0].clone().into());
        app.set_accueil_nom2(r.noms[1].clone().into());
    }
    app.set_accueil_ouverte(true);
    ctx.accueil_montre.set(true);
}

/// Pousse les réglages (et la liste des modèles) dans la fenêtre de réglage.
fn appliquer_reglages(app: &App, ctx: &Contexte) {
    let r = ctx.reglages.borrow();
    app.set_cfg_separer(r.separer);
    app.set_cfg_alerte(r.alerte);
    app.set_cfg_animer(r.animer);
    // un nom en cours de frappe n'est pas réécrit (le curseur sauterait)
    if reglages::nom_propre(&app.get_cfg_nom1()) != r.noms[0] {
        app.set_cfg_nom1(r.noms[0].clone().into());
    }
    if reglages::nom_propre(&app.get_cfg_nom2()) != r.noms[1] {
        app.set_cfg_nom2(r.noms[1].clone().into());
    }
    // les conversations masquées : « n conversations masquées » et « Réafficher » (visibles seulement si n > 0)
    let masquees = ctx.dernier_suivi.borrow().as_ref().map_or(0, |s| s.masquees);
    app.set_cfg_masquees(masquees as i32);
    app.set_cfg_masquees_texte(if masquees == 1 { t(Cle::ConfigMasqueeUne).to_string() } else { fmt(Cle::ConfigMasquees, &[&masquees]) }.into());
    app.set_cfg_defaut1(fmt(Cle::GroupeN, &[&1]).into());
    app.set_cfg_defaut2(fmt(Cle::GroupeN, &[&2]).into());
    let titre = |i: usize| {
        if !r.separer {
            t(Cle::GroupeTous).to_string()
        } else if r.noms[i].is_empty() {
            fmt(Cle::GroupeN, &[&(i + 1)])
        } else {
            fmt(Cle::ConfigGroupeNom, &[&(i + 1), &r.noms[i]])
        }
    };
    app.set_cfg_titre1(titre(0).into());
    app.set_cfg_titre2(titre(1).into());
    app.set_cfg_a1(r.seuils[0].attention as i32);
    app.set_cfg_p1(r.seuils[0].passation as i32);
    app.set_cfg_a2(r.seuils[1].attention as i32);
    app.set_cfg_p2(r.seuils[1].passation as i32);
    let essai = ctx.essai.get();
    let exemple = |i: usize| {
        let pct = essai[i].unwrap_or_else(|| suivi::essai_par_defaut(r.seuils[i]));
        let (etat, texte) = suivi::texte_essai(r.seuils[i], pct);
        let niveau = match etat {
            suivi::Etat::Ok => 0,
            suivi::Etat::Attention => 1,
            _ => 2,
        };
        (pct as i32, texte, niveau)
    };
    let (p1, t1, e1) = exemple(0);
    let (p2, t2, e2) = exemple(1);
    app.set_cfg_essai1(p1);
    app.set_cfg_essai_texte1(t1.into());
    app.set_cfg_essai_etat1(e1);
    app.set_cfg_essai2(p2);
    app.set_cfg_essai_texte2(t2.into());
    app.set_cfg_essai_etat2(e2);
    let modeles: Vec<ModeleCfg> = ctx
        .dernier_suivi
        .borrow()
        .as_ref()
        .map(|s| {
            s.modeles
                .iter()
                .map(|m| {
                    let mut info = fmt(Cle::ConfigVu, &[&suivi::entier(m.vu)]);
                    if m.choix == ChoixFenetre::Auto {
                        info.push_str(" · ");
                        info.push_str(&fmt(Cle::ConfigAutoVaut, &[&suivi::entier(m.auto)]));
                    }
                    ModeleCfg { cle: m.cle.clone().into(), nom: m.nom.clone().into(), info: info.into(), choix: m.choix.index() as i32 }
                })
                .collect()
        })
        .unwrap_or_default();
    app.set_cfg_modeles(ModelRc::new(VecModel::from(modeles)));
}

/// Change un réglage, le range, et remet la page Conversations à jour (rien à valider).
fn changer(app: &slint::Weak<App>, ctx: &Rc<Contexte>, f: impl FnOnce(&mut Reglages)) {
    {
        let mut r = ctx.reglages.borrow_mut();
        f(&mut r);
        *r = r.clone().normalise();
    }
    ctx.sauver();
    if let Some(app) = app.upgrade() {
        app.set_animer(ctx.animer());
        rafraichir_conversations(&app, ctx, maintenant_s());
    }
}

/// Branche tous les boutons de la page Conversations et de la fenêtre de réglage.
pub fn brancher(app: &App, ctx: &Rc<Contexte>) {
    let faible = app.as_weak();
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_onglet_change(move |o| {
            // au retour sur Semaine, les chiffres repartent de zéro
            if o == 0 {
                c.compteurs.intro.set(true);
            }
            if let Some(app) = f.upgrade() {
                rafraichir(&app, &c);
            }
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_autres_clique(move || {
            c.recentes_ouvertes.set(!c.recentes_ouvertes.get());
            if let Some(app) = f.upgrade() {
                rafraichir_conversations(&app, &c, maintenant_s());
            }
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_etiquette_cliquee(move |dossier| changer(&f, &c, |r| r.changer_de_camp(&dossier)));
    }
    {
        // le ✕ d'une ligne : la conversation est masquée (son identifiant de session est rangé dans conversations.txt) ; un clic suffit
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_masquer_cliquee(move |session| {
            changer(&f, &c, |r| {
                r.masquer(&session);
            })
        });
    }
    {
        // « Réafficher » (Config) : toutes les conversations masquées reviennent
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_reafficher(move || changer(&f, &c, |r| r.reafficher_tout()));
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_separer_change(move |v| changer(&f, &c, |r| r.separer = v));
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_alerte_change(move |v| changer(&f, &c, |r| r.alerte = v));
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_animer_change(move |v| {
            changer(&f, &c, |r| r.animer = v);
            // les chiffres de la semaine sont refaits (de zéro si on vient de rallumer l'animation)
            c.compteurs.intro.set(v);
            if let Some(app) = f.upgrade() {
                c.compteurs.textes.borrow_mut().iter_mut().for_each(String::clear);
                rafraichir(&app, &c);
            }
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_nom_change(move |g| {
            let Some(app) = f.upgrade() else { return };
            let brut = if g == 0 { app.get_cfg_nom1() } else { app.get_cfg_nom2() };
            // le champ ne dépasse pas la longueur permise
            if brut.chars().count() > reglages::NOM_MAX {
                let coupe: String = brut.chars().take(reglages::NOM_MAX).collect();
                if g == 0 {
                    app.set_cfg_nom1(coupe.into());
                } else {
                    app.set_cfg_nom2(coupe.into());
                }
            }
            let nom = reglages::nom_propre(&brut);
            changer(&f, &c, |r| r.noms[(g as usize).min(1)] = nom);
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_seuil_change(move |g, quel, v| {
            let v = v.clamp(reglages::SEUIL_MIN as i32, reglages::SEUIL_MAX as i32) as u8;
            changer(&f, &c, |r| {
                let s = &mut r.seuils[(g as usize).min(1)];
                // l'un pousse l'autre : l'attention ne dépasse pas la passation
                if quel == 0 {
                    s.attention = v;
                    s.passation = s.passation.max(v);
                } else {
                    s.passation = v;
                    s.attention = s.attention.min(v);
                }
            });
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_cfg_essai_change(move |g, v| {
            let mut e = c.essai.get();
            e[(g as usize).min(1)] = Some(v.clamp(0, 100) as u8);
            c.essai.set(e);
            if let Some(app) = f.upgrade() {
                appliquer_reglages(&app, &c);
            }
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        // la ligne cliquée porte la CLÉ de son modèle : même si la liste a changé d'ordre entre l'affichage et
        // le clic, c'est bien ce modèle-là qui change
        app.on_cfg_fenetre_change(move |cle, k| {
            let Some(choix) = ChoixFenetre::depuis_index(k as usize) else { return };
            changer(&f, &c, |r| r.fixer_fenetre(&cle, choix));
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_accueil_ouvrir(move || {
            if let Some(app) = f.upgrade() {
                ouvrir_accueil(&app, &c);
            }
        });
    }
    {
        let c = Rc::clone(ctx);
        app.on_accueil_coche(move |i, v| {
            if let Some(e) = c.accueil.borrow_mut().get_mut(i as usize) {
                e.1 = v;
            }
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_accueil_valider(move |separer| {
            let Some(app) = f.upgrade() else { return };
            let coches: Vec<String> = c.accueil.borrow().iter().filter(|(_, v)| *v).map(|(d, _)| d.clone()).collect();
            let noms = [reglages::nom_propre(&app.get_accueil_nom1()), reglages::nom_propre(&app.get_accueil_nom2())];
            app.set_accueil_ouverte(false);
            // séparer : les dossiers cochés forment le groupe 1 (aucun : tout est dans le groupe 2) ; un seul groupe : l'ancien
            // choix reste rangé dans conversations.txt (voir `Reglages::appliquer_choix_groupes`)
            changer(&f, &c, |r| r.appliquer_choix_groupes(separer, coches, noms));
        });
    }
    {
        let (f, c) = (faible.clone(), Rc::clone(ctx));
        app.on_accueil_fermer(move || {
            if let Some(app) = f.upgrade() {
                app.set_accueil_ouverte(false);
            }
            changer(&f, &c, |r| r.accueil_vu = true);
        });
    }
    {
        let (f, c) = (faible, Rc::clone(ctx));
        app.on_cfg_retablir(move || {
            c.essai.set([None, None]);
            // « Rétablir » remet les réglages de départ, mais l'accueil déjà vu ne revient pas et les conversations masquées
            // restent masquées (« Réafficher » est un bouton à part)
            changer(&f, &c, |r| r.retablir());
            c.compteurs.intro.set(true);
            if let Some(app) = f.upgrade() {
                c.compteurs.textes.borrow_mut().iter_mut().for_each(String::clear);
                rafraichir(&app, &c);
            }
        });
    }
}
