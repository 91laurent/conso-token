//! Tous les textes de l'appli, dans les huit langues, dans UNE table : une ligne = un texte, huit
//! cellules dans l'ordre `en, fr, es, de, pt, it, zh, ja` (comme `langue::TOUTES`). Les parties qui
//! changent sont écrites `{0}`, `{1}`… et remplies par `fmt` ; chaque langue place les siennes où sa
//! grammaire le demande. Les tests vérifient qu'aucune cellule n'est vide et que les `{n}` sont les
//! mêmes dans les huit langues.
//!
//! Tutoiement partout (comme le français d'origine) ; portugais neutre (« seu »), chinois simplifié.

use std::fmt::Display;

use crate::langue::{self, NB_LANGUES};

macro_rules! textes {
    ($($cle:ident => [$en:expr, $fr:expr, $es:expr, $de:expr, $pt:expr, $it:expr, $zh:expr, $ja:expr],)*) => {
        /// Un texte de l'appli.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Cle { $($cle,)* }
        pub const CLES: &[Cle] = &[$(Cle::$cle,)*];
        const TABLE: &[[&str; NB_LANGUES]] = &[$([$en, $fr, $es, $de, $pt, $it, $zh, $ja],)*];
    };
}

textes! {
    // ------------------------------------------------ jours de la semaine (lundi d'abord)
    JourC0 => ["Mon", "lun", "lun", "Mo", "seg", "lun", "周一", "月"],
    JourC1 => ["Tue", "mar", "mar", "Di", "ter", "mar", "周二", "火"],
    JourC2 => ["Wed", "mer", "mié", "Mi", "qua", "mer", "周三", "水"],
    JourC3 => ["Thu", "jeu", "jue", "Do", "qui", "gio", "周四", "木"],
    JourC4 => ["Fri", "ven", "vie", "Fr", "sex", "ven", "周五", "金"],
    JourC5 => ["Sat", "sam", "sáb", "Sa", "sáb", "sab", "周六", "土"],
    JourC6 => ["Sun", "dim", "dom", "So", "dom", "dom", "周日", "日"],
    JourL0 => ["Monday", "lundi", "lunes", "Montag", "segunda-feira", "lunedì", "星期一", "月曜日"],
    JourL1 => ["Tuesday", "mardi", "martes", "Dienstag", "terça-feira", "martedì", "星期二", "火曜日"],
    JourL2 => ["Wednesday", "mercredi", "miércoles", "Mittwoch", "quarta-feira", "mercoledì", "星期三", "水曜日"],
    JourL3 => ["Thursday", "jeudi", "jueves", "Donnerstag", "quinta-feira", "giovedì", "星期四", "木曜日"],
    JourL4 => ["Friday", "vendredi", "viernes", "Freitag", "sexta-feira", "venerdì", "星期五", "金曜日"],
    JourL5 => ["Saturday", "samedi", "sábado", "Samstag", "sábado", "sabato", "星期六", "土曜日"],
    JourL6 => ["Sunday", "dimanche", "domingo", "Sonntag", "domingo", "domenica", "星期日", "日曜日"],

    // ------------------------------------------------ formats (heures, dates, durées, âges, pourcentages)
    // {0} = jour, {1} = mois (deux chiffres, sauf chinois et japonais)
    Date => ["{1}/{0}", "{0}/{1}", "{0}/{1}", "{0}.{1}.", "{0}/{1}", "{0}/{1}", "{1}月{0}日", "{1}月{0}日"],
    // {0} = jour de la semaine, {1} = date, {2} = heure (24 h)
    JourLongHeure => ["{0} {1} at {2}", "{0} {1} à {2}", "{0} {1} a las {2}", "{0}, {1}, {2} Uhr", "{0} {1} às {2}", "{0} {1} alle {2}", "{1} {0} {2}", "{1} {0} {2}"],
    HeureRonde => ["{0}:00", "{0}h", "{0}:00", "{0}:00", "{0}:00", "{0}:00", "{0}:00", "{0}:00"],
    HeureMinutes => ["{0}:{1}", "{0}h{1}", "{0}:{1}", "{0}:{1}", "{0}:{1}", "{0}:{1}", "{0}:{1}", "{0}:{1}"],
    Pourcent => ["{0}%", "{0} %", "{0} %", "{0} %", "{0}%", "{0}%", "{0}%", "{0}%"],
    DureeHeures => ["{0} h {1} min", "{0} h {1}", "{0} h {1} min", "{0} Std. {1} Min.", "{0} h {1} min", "{0} h {1} min", "{0}小时{1}分钟", "{0}時間{1}分"],
    DureeMinutes => ["{0} min", "{0} min", "{0} min", "{0} Min.", "{0} min", "{0} min", "{0}分钟", "{0}分"],
    DureeMoinsUneMinute => ["less than 1 min", "moins d'1 min", "menos de 1 min", "weniger als 1 Min.", "menos de 1 min", "meno di 1 min", "不到1分钟", "1分未満"],
    AgeInstant => ["just now", "à l'instant", "ahora mismo", "gerade eben", "agora mesmo", "adesso", "刚刚", "たった今"],
    AgeMinutes => ["{0} min ago", "il y a {0} min", "hace {0} min", "vor {0} Min.", "há {0} min", "{0} min fa", "{0}分钟前", "{0}分前"],
    AgeHeures => ["{0} h ago", "il y a {0} h", "hace {0} h", "vor {0} Std.", "há {0} h", "{0} h fa", "{0}小时前", "{0}時間前"],
    AgeJours => ["{0} d ago", "il y a {0} j", "hace {0} d", "vor {0} T.", "há {0} d", "{0} g fa", "{0}天前", "{0}日前"],
    MotTokens => ["tokens", "tokens", "tokens", "Tokens", "tokens", "token", "Token", "トークン"],
    // {0} = jour et heure de la remise à zéro
    DefinitionJour => [
        "A day = 24 h, from the reset ({0}) to the same time the next day, not midnight to midnight.",
        "Un jour = 24 h, de la remise à zéro ({0}) à la même heure le lendemain, pas minuit à minuit.",
        "Un día = 24 h, desde el reinicio ({0}) hasta la misma hora del día siguiente, no de medianoche a medianoche.",
        "Ein Tag = 24 Std., vom Zurücksetzen ({0}) bis zur selben Uhrzeit am nächsten Tag, nicht von Mitternacht bis Mitternacht.",
        "Um dia = 24 h, da reinicialização ({0}) até a mesma hora do dia seguinte, não da meia-noite à meia-noite.",
        "Un giorno = 24 h, dal reset ({0}) alla stessa ora del giorno dopo, non da mezzanotte a mezzanotte.",
        "一天 = 24 小时，从重置时间（{0}）到次日同一时间，而不是从午夜到午夜。",
        "1日 = 24時間。リセット時刻（{0}）から翌日の同じ時刻までで、午前0時から午前0時ではありません。"
    ],

    // ------------------------------------------------ en-tête et cartes
    BoutonApropos => ["ABOUT", "À PROPOS", "ACERCA DE", "INFO", "SOBRE", "INFO", "关于", "概要"],
    SourceDemo => [
        "Demo mode: fake figures",
        "Mode démonstration : chiffres factices",
        "Modo demostración: cifras ficticias",
        "Demomodus: erfundene Zahlen",
        "Modo de demonstração: números fictícios",
        "Modalità dimostrativa: dati fittizi",
        "演示模式：数字为虚构",
        "デモモード：架空の数値"
    ],
    SourceAge => [
        "Figures updated {0}",
        "Chiffres mis à jour {0}",
        "Cifras actualizadas {0}",
        "Zahlen aktualisiert {0}",
        "Números atualizados {0}",
        "Dati aggiornati {0}",
        "数据更新于{0}",
        "データ更新：{0}"
    ],
    SourceAucun => [
        "No figures received yet",
        "Aucun chiffre reçu pour l’instant",
        "Aún no se ha recibido ninguna cifra",
        "Noch keine Zahlen empfangen",
        "Nenhum número recebido ainda",
        "Nessun dato ricevuto per ora",
        "暂未收到任何数字",
        "まだ数値を受信していません"
    ],
    CarteRythme => ["YOUR PACE", "TON RYTHME", "TU RITMO", "DEIN TEMPO", "SEU RITMO", "IL TUO RITMO", "你的节奏", "あなたのペース"],
    CarteReste => ["YOU HAVE LEFT", "IL TE RESTE", "TE QUEDA", "DU HAST NOCH", "VOCÊ AINDA TEM", "TI RESTA", "你还剩", "残り"],
    CarteReset => [
        "YOUR PLAN RESETS TO 100% IN",
        "TON FORFAIT REVIENT À 100 % DANS",
        "TU PLAN VUELVE AL 100 % EN",
        "DEIN ABO IST WIEDER VOLL IN",
        "SEU PLANO VOLTA A 100% EM",
        "IL TUO PIANO TORNA AL 100% TRA",
        "额度恢复至 100% 还需",
        "100%に戻るまで"
    ],
    RythmeDans => ["On pace", "Dans le rythme", "En el ritmo", "Im Soll", "No ritmo", "In linea", "正常节奏", "ペース通り"],
    TagDans => [
        "You are right on pace",
        "Tu consommes au rythme prévu",
        "Vas justo al ritmo previsto",
        "Du liegst genau im Plan",
        "Você está no ritmo previsto",
        "Stai consumando come previsto",
        "你的消耗正好符合预期",
        "予定どおりのペースです"
    ],
    RythmeAvance => ["{0}% to spare", "{0} % d’avance", "{0} % de margen", "{0} % Spielraum", "{0}% de folga", "{0}% di margine", "富余 {0}%", "{0}%の余裕"],
    TagAvance => [
        "You are using it slower than planned",
        "Tu consommes moins vite que prévu",
        "Consumes más despacio de lo previsto",
        "Du verbrauchst langsamer als geplant",
        "Seu consumo está abaixo do previsto",
        "Consumi più lentamente del previsto",
        "你消耗得比预期慢",
        "予定より遅いペースです"
    ],
    RythmeTrop => ["{0}% over", "{0} % de trop", "{0} % de más", "{0} % zu viel", "{0}% a mais", "{0}% in più", "超出 {0}%", "{0}%オーバー"],
    TagTrop => [
        "You are using it faster than planned",
        "Tu consommes plus vite que prévu",
        "Consumes más rápido de lo previsto",
        "Du verbrauchst schneller als geplant",
        "Seu consumo está acima do previsto",
        "Consumi più velocemente del previsto",
        "你消耗得比预期快",
        "予定より速いペースです"
    ],
    RythmeDetail => [
        "You have used {0} of your weekly plan, while {1} of the week has already passed.",
        "Tu as utilisé {0} de ton forfait de la semaine, alors que {1} de la semaine est déjà passée.",
        "Has usado el {0} de tu plan semanal, mientras que ya ha pasado el {1} de la semana.",
        "Du hast {0} deines Wochenkontingents verbraucht, während {1} der Woche schon vergangen sind.",
        "Você usou {0} do seu plano da semana, enquanto {1} da semana já passou.",
        "Hai usato il {0} del tuo piano settimanale, mentre è già trascorso il {1} della settimana.",
        "你已用掉本周额度的 {0}，而本周已过去 {1}。",
        "今週のプランの{0}を使用しました。一方、週はすでに{1}が過ぎています。"
    ],
    ResteDetail => [
        "of your weekly plan",
        "de ton forfait de la semaine",
        "de tu plan semanal",
        "deines Wochenkontingents",
        "do seu plano da semana",
        "del tuo piano settimanale",
        "本周额度",
        "今週のプラン分"
    ],
    ResetDetail => [
        "{0} · the time is re-read at every update",
        "{0} · l’heure est relue à chaque mise à jour",
        "{0} · la hora se vuelve a leer en cada actualización",
        "{0} · die Uhrzeit wird bei jeder Aktualisierung neu gelesen",
        "{0} · o horário é relido a cada atualização",
        "{0} · l’orario viene riletto a ogni aggiornamento",
        "{0} · 每次更新都会重新读取时间",
        "{0} · 更新のたびに時刻を再取得します"
    ],
    // l'avis de la carte, la seule semaine où l'heure locale change : {0} = de combien (`AvisUneHeure`, ou
    // une durée si l'écart n'est pas d'une heure), {1} = jour et date du changement (`JourDate`). Le sens
    // (plus tôt / plus tard) et la saison (heure d'hiver / d'été) viennent du SENS du changement : l'heure
    // locale recule = heure d'hiver, elle avance = heure d'été, dans les deux hémisphères. (En allemand la
    // date finit par un point : la phrase n'en ajoute pas un second.)
    AvisUneHeure => ["One hour", "Une heure", "Una hora", "Eine Stunde", "Uma hora", "Un’ora", "1小时", "1時間"],
    // {0} = jour de la semaine, {1} = date
    JourDate => ["{0} {1}", "{0} {1}", "{0} {1}", "{0}, {1}", "{0} {1}", "{0} {1}", "{1} {0}", "{1} {0}"],
    AvisPlusTot => [
        "{0} earlier: clocks go back to standard time on {1}.",
        "{0} plus tôt : passage à l’heure d’hiver le {1}.",
        "{0} antes: cambio a la hora de invierno el {1}.",
        "{0} früher: Umstellung auf Winterzeit am {1}",
        "{0} mais cedo: mudança para o horário de inverno ({1}).",
        "{0} prima: passaggio all’ora solare, {1}.",
        "提前{0}：{1}起改用冬令时。",
        "{0}早い：{1}に冬時間へ切り替わります。"
    ],
    AvisPlusTard => [
        "{0} later: clocks go forward to daylight saving time on {1}.",
        "{0} plus tard : passage à l’heure d’été le {1}.",
        "{0} después: cambio a la hora de verano el {1}.",
        "{0} später: Umstellung auf Sommerzeit am {1}",
        "{0} mais tarde: mudança para o horário de verão ({1}).",
        "{0} dopo: passaggio all’ora legale, {1}.",
        "推迟{0}：{1}起改用夏令时。",
        "{0}遅い：{1}に夏時間へ切り替わります。"
    ],

    // ------------------------------------------------ la semaine d'un coup d'œil
    SemaineTitre => [
        "Your week at a glance",
        "Ta semaine en un coup d’œil",
        "Tu semana de un vistazo",
        "Deine Woche auf einen Blick",
        "Sua semana em resumo",
        "La tua settimana a colpo d’occhio",
        "本周概览",
        "今週のひと目まとめ"
    ],
    SemaineTexte => [
        "you have used {0}  ·  the week is {1} through  ·  light marker = where the week stands",
        "tu as utilisé {0}  ·  la semaine est passée à {1}  ·  le repère clair = où en est la semaine",
        "has usado {0}  ·  la semana va por el {1}  ·  marca clara = en qué punto va la semana",
        "du hast {0} verbraucht  ·  die Woche ist zu {1} vorbei  ·  helle Marke = Stand der Woche",
        "você usou {0}  ·  a semana está em {1}  ·  marca clara = em que ponto está a semana",
        "hai usato {0}  ·  la settimana è al {1}  ·  segno chiaro = a che punto è la settimana",
        "你已用 {0}  ·  本周已过 {1}  ·  浅色标记 = 本周进度",
        "使用 {0}  ·  週の経過 {1}  ·  明るい印 = 週の進み具合"
    ],

    // ------------------------------------------------ jour par jour
    TitreJours => [
        "Your usage, day by day",
        "Ta conso jour par jour",
        "Tu consumo día a día",
        "Dein Verbrauch Tag für Tag",
        "Seu consumo dia a dia",
        "Il tuo consumo giorno per giorno",
        "每日用量",
        "日ごとの使用量"
    ],
    // {0} = début d'un jour, {1} = fin (la même heure)
    Puce => [
        "24 h days: {0} → {1}",
        "jours de 24 h : {0} → {1}",
        "días de 24 h: {0} → {1}",
        "24-Std.-Tage: {0} → {1}",
        "dias de 24 h: {0} → {1}",
        "giorni di 24 h: {0} → {1}",
        "24 小时为一天：{0} → {1}",
        "24時間単位：{0} → {1}"
    ],
    OngletAujourdhui => ["Today", "Aujourd’hui", "Hoy", "Heute", "Hoje", "Oggi", "今天", "今日"],
    OngletLignes => ["Lines", "Lignes", "Líneas", "Zeilen", "Linhas", "Righe", "线条", "ライン"],
    OngletTrajectoire => ["Trajectory", "Trajectoire", "Trayectoria", "Verlauf", "Trajetória", "Traiettoria", "轨迹", "推移"],
    OngletBatteries => ["Batteries", "Batteries", "Baterías", "Batterien", "Baterias", "Batterie", "电池", "バッテリー"],
    OngletSemaine => ["Week", "Semaine", "Semana", "Woche", "Semana", "Settimana", "本周", "週"],
    AVenir => ["upcoming", "à venir", "por venir", "kommend", "por vir", "in arrivo", "未到", "これから"],
    AvantLecture => [
        "before 1st reading",
        "avant la 1re lecture",
        "antes de la 1.ª lectura",
        "vor der 1. Messung",
        "antes da 1ª leitura",
        "prima della 1ª lettura",
        "首次读取前",
        "初回取得前"
    ],
    // {0} = ce qu'il reste aujourd'hui
    Reste => ["{0} left", "reste {0}", "quedan {0}", "noch {0}", "restam {0}", "restano {0}", "剩 {0}", "残り{0}"],
    // {0} = consommé, {1} = budget du jour (nombres, sans « % »)
    SousBatterie => ["{0} of {1}%", "{0} sur {1} %", "{0} de {1} %", "{0} von {1} %", "{0} de {1}%", "{0} su {1}%", "{0} / {1}%", "{0} / {1}%"],
    LigneConso => ["{0}% of {1}", "{0} % sur {1}", "{0} % de {1}", "{0} % von {1}", "{0}% de {1}", "{0}% su {1}", "{0}% / {1}", "{0}% / {1}"],
    LigneBudget => ["budget {0}%", "budget {0} %", "presupuesto {0} %", "Budget {0} %", "orçamento {0}%", "budget {0}%", "预算 {0}%", "予算 {0}%"],

    // vue « Aujourd'hui »
    EtiqDepasse => ["OVER", "DÉPASSÉ", "SUPERADO", "ÜBERSCHRITTEN", "EXCEDIDO", "SUPERATO", "已超出", "超過"],
    AujSous => ["of {0}% allowed", "sur {0} % permis", "de {0} % permitido", "von {0} % erlaubt", "de {0}% permitidos", "su {0}% consentito", "允许 {0}%", "許容{0}%"],
    AujDernierJour => [
        "It’s the last day.",
        "C’est le dernier jour.",
        "Es el último día.",
        "Es ist der letzte Tag.",
        "É o último dia.",
        "È l’ultimo giorno.",
        "今天是最后一天。",
        "今日が最終日です。"
    ],
    AujSuiteUn => [
        "The last day: {0}%.",
        "Le dernier jour : {0} %.",
        "El último día: {0} %.",
        "Der letzte Tag: {0} %.",
        "O último dia: {0}%.",
        "L’ultimo giorno: {0}%.",
        "最后一天：{0}%。",
        "最終日：{0}%。"
    ],
    // {0} = nombre de jours, {1} = part de chacun
    AujSuiteN => [
        "The next {0} days: {1}% each.",
        "Les {0} jours suivants : {1} % chacun.",
        "Los {0} días siguientes: {1} % cada uno.",
        "Die folgenden {0} Tage: je {1} %.",
        "Os {0} dias seguintes: {1}% cada um.",
        "I {0} giorni successivi: {1}% ciascuno.",
        "之后 {0} 天：每天 {1}%。",
        "以降{0}日間：各日{1}%。"
    ],
    // {0} = ce qui reste du forfait, {1} = ce qui reste aujourd'hui, {2} = la suite (AujSuite…)
    AujResume => [
        "You have {0}% of your plan left: {1}% to finish today. {2}",
        "Il te reste {0} % du forfait : {1} % pour finir aujourd’hui. {2}",
        "Te queda el {0} % del plan: {1} % para terminar hoy. {2}",
        "Du hast noch {0} % des Kontingents: {1} % bis zum Ende von heute. {2}",
        "Restam {0}% do plano: {1}% para terminar hoje. {2}",
        "Ti resta il {0}% del piano: {1}% per finire oggi. {2}",
        "你还剩套餐的 {0}%：其中 {1}% 用于今天。{2}",
        "プランの残りは{0}%で、今日の分は{1}%です。{2}"
    ],

    // vue « Trajectoire »
    TrajPile => [
        "Right now: exactly on pace",
        "En ce moment : pile au rythme",
        "Ahora: justo en el ritmo",
        "Aktuell: genau im Soll",
        "Agora: exatamente no ritmo",
        "Adesso: esattamente in linea",
        "当前：恰好符合节奏",
        "現在：ちょうどペース通り"
    ],
    TrajAvance => [
        "Right now: {0}% to spare",
        "En ce moment : {0} % d’avance",
        "Ahora: {0} % de margen",
        "Aktuell: {0} % Spielraum",
        "Agora: {0}% de folga",
        "Adesso: {0}% di margine",
        "当前：富余 {0}%",
        "現在：{0}%の余裕"
    ],
    TrajRetard => [
        "Right now: {0}% over",
        "En ce moment : {0} % de retard",
        "Ahora: {0} % de más",
        "Aktuell: {0} % zu viel",
        "Agora: {0}% a mais",
        "Adesso: {0}% in più",
        "当前：超出 {0}%",
        "現在：{0}%オーバー"
    ],
    TrajUtilises => ["{0}% used", "{0} % utilisés", "{0} % usado", "{0} % verbraucht", "{0}% usados", "{0}% usato", "已用 {0}%", "使用済み {0}%"],

    // vue « Semaine »
    SemRepere => ["week {0}% through", "semaine passée à {0} %", "semana al {0} %", "Woche {0} % vorbei", "semana em {0}%", "settimana al {0}%", "本周已过 {0}%", "週の経過 {0}%"],
    SemResume => [
        "You have used {0}%, {1}% left.",
        "Tu as utilisé {0} %, il te reste {1} %.",
        "Has usado el {0} %, te queda el {1} %.",
        "Du hast {0} % verbraucht, es bleiben {1} %.",
        "Você usou {0}%, restam {1}%.",
        "Hai usato il {0}%, ti resta il {1}%.",
        "你已用 {0}%，还剩 {1}%。",
        "使用 {0}%、残り {1}%。"
    ],

    // légendes sous la vue, une par onglet (puis deux remarques ajoutées au besoin)
    Aide0 => [
        "Tiles: green + = set aside · pink − = exceeded · gold = today · grey = budget of upcoming days.",
        "Pastilles : + vert = mis de côté · − rose = dépassé · doré = aujourd’hui · gris = budget des jours à venir.",
        "Casillas: + verde = ahorrado · − rosa = superado · dorado = hoy · gris = presupuesto de los días por venir.",
        "Kacheln: grünes + = übrig · rosa − = überschritten · gold = heute · grau = Budget der kommenden Tage.",
        "Pastilhas: + verde = guardado · − rosa = excedido · dourado = hoje · cinza = orçamento dos dias por vir.",
        "Tessere: + verde = risparmiato · − rosa = superato · oro = oggi · grigio = budget dei giorni a venire.",
        "色块：绿色 + = 有结余 · 粉色 − = 已超出 · 金色 = 今天 · 灰色 = 未来几天的预算。",
        "タイル：緑の＋ = 余り · ピンクの − = 超過 · 金色 = 今日 · グレー = これからの日の予算。"
    ],
    Aide1 => [
        "Light marker = day’s budget · pink = beyond the budget · percentages are those of the weekly plan.",
        "Repère clair = budget du jour · rose = au-delà du budget · les pourcentages sont ceux du forfait de la semaine.",
        "Marca clara = presupuesto del día · rosa = por encima del presupuesto · los porcentajes son los del plan semanal.",
        "Helle Marke = Tagesbudget · rosa = über dem Budget · die Prozentwerte beziehen sich auf das Wochenkontingent.",
        "Marca clara = orçamento do dia · rosa = acima do orçamento · as porcentagens são as do plano da semana.",
        "Segno chiaro = budget del giorno · rosa = oltre il budget · le percentuali sono quelle del piano settimanale.",
        "浅色标记 = 当天预算 · 粉色 = 超出预算 · 百分比均为占本周额度的比例。",
        "明るい印 = 1日の予算 · ピンク = 予算超過 · パーセントは週のプランに対する割合です。"
    ],
    Aide2 => [
        "Gold curve = your cumulative usage · light line = exactly on pace · green = to spare, pink = over · thin line = allowed path.",
        "Courbe dorée = ta conso cumulée · trait clair = pile au rythme · vert = en avance, rose = en retard · tracé fin = chemin permis.",
        "Curva dorada = tu consumo acumulado · línea clara = justo al ritmo · verde = con margen, rosa = de más · trazo fino = camino permitido.",
        "Goldene Kurve = dein kumulierter Verbrauch · helle Linie = genau im Soll · grün = Spielraum, rosa = zu viel · dünne Linie = erlaubter Verlauf.",
        "Curva dourada = seu consumo acumulado · linha clara = exatamente no ritmo · verde = com folga, rosa = a mais · traço fino = caminho permitido.",
        "Curva dorata = il tuo consumo cumulato · linea chiara = esattamente in linea · verde = con margine, rosa = in più · tratto sottile = percorso consentito.",
        "金色曲线 = 你的累计用量 · 浅色线 = 恰好符合节奏 · 绿色 = 有富余，粉色 = 超出 · 细线 = 允许的路径。",
        "金色の曲線 = 累計使用量 · 明るい線 = ちょうどペース通り · 緑 = 余裕、ピンク = 超過 · 細い線 = 許容される経路。"
    ],
    Aide3 => [
        "Full battery = day’s budget intact · empty = used up · pink = exceeded. Figure: what is left (today) or the day’s difference (past).",
        "Batterie pleine = budget du jour intact · vide = épuisé · rose = dépassé. Chiffre : ce qu’il reste (aujourd’hui) ou l’écart du jour (passé).",
        "Batería llena = presupuesto del día intacto · vacía = agotado · rosa = superado. Cifra: lo que queda (hoy) o la diferencia del día (pasado).",
        "Volle Batterie = Tagesbudget unberührt · leer = aufgebraucht · rosa = überschritten. Zahl: Rest (heute) oder Differenz des Tages (vergangen).",
        "Bateria cheia = orçamento do dia intacto · vazia = esgotado · rosa = excedido. Número: o que resta (hoje) ou a diferença do dia (passado).",
        "Batteria piena = budget del giorno intatto · vuota = esaurito · rosa = superato. Cifra: ciò che resta (oggi) o lo scarto del giorno (passato).",
        "电池满 = 当天预算未动 · 空 = 已用完 · 粉色 = 已超出。数字：今天显示剩余量，过去的日子显示当天的差额。",
        "バッテリー満タン = 1日の予算が手つかず · 空 = 使い切り · ピンク = 超過。数字：今日は残り、過去の日はその日の差。"
    ],
    Aide4 => [
        "Gold = used · light = today, so far · hatched = allowed today · boxes = share of each upcoming day · marker = where the week stands.",
        "Doré = consommé · clair = aujourd’hui, déjà · hachuré = permis aujourd’hui · cases = part de chaque jour à venir · repère = où en est la semaine.",
        "Dorado = consumido · claro = hoy, hasta ahora · rayado = permitido hoy · casillas = parte de cada día por venir · marca = en qué punto va la semana.",
        "Gold = verbraucht · hell = heute bisher · schraffiert = heute erlaubt · Felder = Anteil jedes kommenden Tages · Marke = Stand der Woche.",
        "Dourado = consumido · claro = hoje, até agora · hachurado = permitido hoje · caixas = parte de cada dia por vir · marca = em que ponto está a semana.",
        "Oro = consumato · chiaro = oggi, finora · tratteggiato = consentito oggi · caselle = quota di ogni giorno a venire · segno = a che punto è la settimana.",
        "金色 = 已用 · 浅色 = 今天已用 · 斜纹 = 今天还可用 · 方块 = 未来每天的份额 · 标记 = 本周进度。",
        "金色 = 使用済み · 明るい色 = 今日これまで · 斜線 = 今日の許容分 · 枠 = これからの各日の配分 · 印 = 週の進み具合。"
    ],
    AideAvantLecture => [
        "“—”: before the first reading.",
        "« — » : avant la première lecture.",
        "«—»: antes de la primera lectura.",
        "„—“: vor der ersten Messung.",
        "“—”: antes da primeira leitura.",
        "«—»: prima della prima lettura.",
        "“—”：首次读取之前。",
        "「—」：初回取得前。"
    ],
    AideTokens => [
        "Tokens = small pieces of text exchanged with Claude.",
        "Tokens = petits morceaux de texte échangés avec Claude.",
        "Tokens = pequeños fragmentos de texto intercambiados con Claude.",
        "Tokens = kleine Textbausteine, die mit Claude ausgetauscht werden.",
        "Tokens = pequenos pedaços de texto trocados com o Claude.",
        "Token = piccoli frammenti di testo scambiati con Claude.",
        "Token = 与 Claude 往来的小段文本。",
        "トークン = Claude とやり取りするテキストの小さな断片。"
    ],

    // ------------------------------------------------ ce qu'il reste à consommer
    TitreReste => [
        "What you have left to use",
        "Ce qu’il te reste à consommer",
        "Lo que te queda por consumir",
        "Was du noch verbrauchen kannst",
        "O que resta para consumir",
        "Quello che ti resta da consumare",
        "剩余可用额度",
        "残りの使用可能量"
    ],
    SousReste => [
        "Recalculated automatically at every update: whatever you haven’t used is already carried over.",
        "Calculé tout seul à chaque mise à jour : ce que tu n’as pas consommé est déjà reporté.",
        "Se calcula solo en cada actualización: lo que no has consumido ya se traslada.",
        "Wird bei jeder Aktualisierung automatisch berechnet: Was du nicht verbraucht hast, ist schon übertragen.",
        "Calculado automaticamente a cada atualização: o que você não consumiu já foi repassado.",
        "Calcolato in automatico a ogni aggiornamento: ciò che non hai consumato è già riportato.",
        "每次更新时自动计算：你没用完的部分已经顺延。",
        "更新のたびに自動で計算します。使わなかった分はすでに繰り越されています。"
    ],
    // {0} = jour et heure de la fin du jour en cours
    Reste1Nom => [
        "To finish the current day (until {0})",
        "Pour finir le jour en cours (jusqu’à {0})",
        "Para terminar el día en curso (hasta {0})",
        "Bis zum Ende des laufenden Tags (bis {0})",
        "Para terminar o dia atual (até {0})",
        "Per finire il giorno in corso (fino a {0})",
        "用于完成今天（至 {0}）",
        "今日を終えるまで（{0}まで）"
    ],
    Reste2Dernier => [
        "For the last day ({0} → {1})",
        "Pour le dernier jour ({0} → {1})",
        "Para el último día ({0} → {1})",
        "Für den letzten Tag ({0} → {1})",
        "Para o último dia ({0} → {1})",
        "Per l’ultimo giorno ({0} → {1})",
        "最后一天（{0} → {1}）",
        "最終日（{0} → {1}）"
    ],
    Reste2NReport => [
        "For each of the next {0} days (in equal shares, if you use all of today’s budget)",
        "Pour chacun des {0} jours suivants (à parts égales, si tu consommes tout ton budget d’aujourd’hui)",
        "Para cada uno de los {0} días siguientes (a partes iguales, si consumes todo tu presupuesto de hoy)",
        "Für jeden der {0} folgenden Tage (gleich verteilt, wenn du das heutige Budget ganz nutzt)",
        "Para cada um dos {0} dias seguintes (em partes iguais, se você consumir todo o orçamento de hoje)",
        "Per ciascuno dei {0} giorni successivi (in parti uguali, se consumi tutto il budget di oggi)",
        "之后 {0} 天中的每一天（平均分配，前提是你用完今天的预算）",
        "以降{0}日間の各日（均等配分。今日の予算を使い切った場合）"
    ],
    Reste2N => [
        "For each of the next {0} days (in equal shares)",
        "Pour chacun des {0} jours suivants (à parts égales)",
        "Para cada uno de los {0} días siguientes (a partes iguales)",
        "Für jeden der {0} folgenden Tage (gleich verteilt)",
        "Para cada um dos {0} dias seguintes (em partes iguais)",
        "Per ciascuno dei {0} giorni successivi (in parti uguali)",
        "之后 {0} 天中的每一天（平均分配）",
        "以降{0}日間の各日（均等配分）"
    ],
    // {0} = ce qui reste du forfait, {1} = ce qui reste aujourd'hui, {2} = la part du dernier jour
    ResumeDernierOk => [
        "You have {0} left: {1} to finish the current day and {2} for the last day. What you don’t use today carries over to the following days; what you go over is taken off them.",
        "Il te reste {0} : {1} pour finir le jour en cours et {2} pour le dernier jour. Ce que tu ne consommes pas aujourd’hui passe aux jours suivants ; ce que tu dépasses en est retiré.",
        "Te quedan {0}: {1} para terminar el día en curso y {2} para el último día. Lo que no consumas hoy pasa a los días siguientes; lo que te pases se les resta.",
        "Du hast noch {0}: {1} bis zum Ende des laufenden Tags und {2} für den letzten Tag. Was du heute nicht verbrauchst, geht auf die folgenden Tage über; was du überziehst, wird davon abgezogen.",
        "Restam {0}: {1} para terminar o dia atual e {2} para o último dia. O que você não consumir hoje passa para os dias seguintes; o que exceder é descontado deles.",
        "Ti restano {0}: {1} per finire il giorno in corso e {2} per l’ultimo giorno. Ciò che non consumi oggi passa ai giorni successivi; ciò che superi viene tolto da questi.",
        "你还剩 {0}：其中 {1} 用于完成今天，{2} 用于最后一天。今天没用完的部分会顺延到之后几天；超出的部分会从之后几天中扣除。",
        "残りは{0}です。今日を終えるまでに{1}、最終日に{2}を使えます。今日使わなかった分は以降の日に繰り越され、超過した分はそこから差し引かれます。"
    ],
    ResumeDernierDepasse => [
        "You have {0} left: {1} to finish the current day and {2} for the last day. Today’s budget is already exceeded: everything that is left goes to the following days.",
        "Il te reste {0} : {1} pour finir le jour en cours et {2} pour le dernier jour. Ton budget d’aujourd’hui est déjà dépassé : tout ce qui reste va aux jours suivants.",
        "Te quedan {0}: {1} para terminar el día en curso y {2} para el último día. Tu presupuesto de hoy ya está superado: todo lo que queda va a los días siguientes.",
        "Du hast noch {0}: {1} bis zum Ende des laufenden Tags und {2} für den letzten Tag. Dein heutiges Budget ist schon überschritten: Alles, was übrig ist, geht an die folgenden Tage.",
        "Restam {0}: {1} para terminar o dia atual e {2} para o último dia. O orçamento de hoje já foi excedido: tudo o que resta vai para os dias seguintes.",
        "Ti restano {0}: {1} per finire il giorno in corso e {2} per l’ultimo giorno. Il budget di oggi è già superato: tutto ciò che resta va ai giorni successivi.",
        "你还剩 {0}：其中 {1} 用于完成今天，{2} 用于最后一天。今天的预算已经超出：剩下的全部归之后几天。",
        "残りは{0}です。今日を終えるまでに{1}、最終日に{2}を使えます。今日の予算はすでに超過しているため、残りはすべて以降の日に回ります。"
    ],
    // {3} = nombre de jours suivants, {2} = la part de chacun
    ResumeNOk => [
        "You have {0} left: {1} to finish the current day and {2} for each of the next {3} days. What you don’t use today carries over to the following days; what you go over is taken off them.",
        "Il te reste {0} : {1} pour finir le jour en cours et {2} pour chacun des {3} jours suivants. Ce que tu ne consommes pas aujourd’hui passe aux jours suivants ; ce que tu dépasses en est retiré.",
        "Te quedan {0}: {1} para terminar el día en curso y {2} para cada uno de los {3} días siguientes. Lo que no consumas hoy pasa a los días siguientes; lo que te pases se les resta.",
        "Du hast noch {0}: {1} bis zum Ende des laufenden Tags und {2} für jeden der {3} folgenden Tage. Was du heute nicht verbrauchst, geht auf die folgenden Tage über; was du überziehst, wird davon abgezogen.",
        "Restam {0}: {1} para terminar o dia atual e {2} para cada um dos {3} dias seguintes. O que você não consumir hoje passa para os dias seguintes; o que exceder é descontado deles.",
        "Ti restano {0}: {1} per finire il giorno in corso e {2} per ciascuno dei {3} giorni successivi. Ciò che non consumi oggi passa ai giorni successivi; ciò che superi viene tolto da questi.",
        "你还剩 {0}：其中 {1} 用于完成今天，之后 {3} 天每天 {2}。今天没用完的部分会顺延到之后几天；超出的部分会从之后几天中扣除。",
        "残りは{0}です。今日を終えるまでに{1}、以降{3}日間は各日{2}を使えます。今日使わなかった分は以降の日に繰り越され、超過した分はそこから差し引かれます。"
    ],
    ResumeNDepasse => [
        "You have {0} left: {1} to finish the current day and {2} for each of the next {3} days. Today’s budget is already exceeded: everything that is left goes to the following days.",
        "Il te reste {0} : {1} pour finir le jour en cours et {2} pour chacun des {3} jours suivants. Ton budget d’aujourd’hui est déjà dépassé : tout ce qui reste va aux jours suivants.",
        "Te quedan {0}: {1} para terminar el día en curso y {2} para cada uno de los {3} días siguientes. Tu presupuesto de hoy ya está superado: todo lo que queda va a los días siguientes.",
        "Du hast noch {0}: {1} bis zum Ende des laufenden Tags und {2} für jeden der {3} folgenden Tage. Dein heutiges Budget ist schon überschritten: Alles, was übrig ist, geht an die folgenden Tage.",
        "Restam {0}: {1} para terminar o dia atual e {2} para cada um dos {3} dias seguintes. O orçamento de hoje já foi excedido: tudo o que resta vai para os dias seguintes.",
        "Ti restano {0}: {1} per finire il giorno in corso e {2} per ciascuno dei {3} giorni successivi. Il budget di oggi è già superato: tutto ciò che resta va ai giorni successivi.",
        "你还剩 {0}：其中 {1} 用于完成今天，之后 {3} 天每天 {2}。今天的预算已经超出：剩下的全部归之后几天。",
        "残りは{0}です。今日を終えるまでに{1}、以降{3}日間は各日{2}を使えます。今日の予算はすでに超過しているため、残りはすべて以降の日に回ります。"
    ],
    ResumeFinal => [
        "You have {0} left to finish the week: it is the last day.",
        "Il te reste {0} pour finir la semaine : c’est le dernier jour.",
        "Te quedan {0} para terminar la semana: es el último día.",
        "Du hast noch {0} bis zum Ende der Woche: Es ist der letzte Tag.",
        "Restam {0} para terminar a semana: é o último dia.",
        "Ti restano {0} per finire la settimana: è l’ultimo giorno.",
        "你还剩 {0} 可用到本周结束：今天是最后一天。",
        "週の終わりまで残り{0}です。今日が最終日です。"
    ],

    // ------------------------------------------------ pages d'accueil
    AccueilSansTitre => [
        "A delivery arrived, but without the figures",
        "Un envoi est arrivé, mais sans les chiffres",
        "Ha llegado un envío, pero sin las cifras",
        "Es ist eine Sendung angekommen, aber ohne Zahlen",
        "Chegou um envio, mas sem os números",
        "È arrivato un invio, ma senza i dati",
        "收到了一次发送，但没有数字",
        "データは届きましたが、数値が含まれていません"
    ],
    AccueilSansMessage => [
        "Conso Token did receive a delivery from the Claude app, but it does not contain your plan’s figures. These figures only exist for Pro and Max plans. Wait for the next delivery (15 minutes): if they never arrive, your plan does not provide them.",
        "Conso Token a bien reçu un envoi de l’appli Claude, mais il ne contient pas les chiffres de ton forfait. Ces chiffres n’existent que pour les forfaits Pro et Max. Attends le prochain envoi (15 minutes) : s’ils n’arrivent jamais, ton forfait ne les fournit pas.",
        "Conso Token ha recibido un envío de la app de Claude, pero no contiene las cifras de tu plan. Estas cifras solo existen para los planes Pro y Max. Espera al próximo envío (15 minutos): si nunca llegan, tu plan no las proporciona.",
        "Conso Token hat zwar eine Sendung der Claude-App erhalten, aber sie enthält nicht die Zahlen deines Abos. Diese Zahlen gibt es nur für die Abos Pro und Max. Warte auf die nächste Sendung (15 Minuten): Kommen sie nie an, liefert dein Abo sie nicht.",
        "O Conso Token recebeu um envio do app do Claude, mas ele não contém os números do seu plano. Esses números só existem para os planos Pro e Max. Aguarde o próximo envio (15 minutos): se nunca chegarem, o seu plano não os fornece.",
        "Conso Token ha ricevuto un invio dall’app Claude, ma non contiene i dati del tuo piano. Questi dati esistono solo per i piani Pro e Max. Attendi il prossimo invio (15 minuti): se non arrivano mai, il tuo piano non li fornisce.",
        "Conso Token 已收到 Claude 应用的一次发送，但其中没有你的套餐数字。这些数字只有 Pro 和 Max 套餐才有。请等待下一次发送（15 分钟）：如果一直没有，说明你的套餐不提供这些数字。",
        "Conso Token は Claude アプリからのデータを受信しましたが、プランの数値が含まれていません。この数値は Pro と Max プランにのみあります。次の送信（15分後）をお待ちください。いつまでも届かない場合、お使いのプランでは提供されません。"
    ],
    AccueilPremiereTitre => [
        "Conso Token is waiting for its first figures",
        "Conso Token attend ses premiers chiffres",
        "Conso Token espera sus primeras cifras",
        "Conso Token wartet auf die ersten Zahlen",
        "O Conso Token está esperando os primeiros números",
        "Conso Token attende i primi dati",
        "Conso Token 正在等待首批数字",
        "Conso Token は最初の数値を待っています"
    ],
    AccueilPremiereMessage => [
        "So far, Conso Token has not received any figures. They arrive on their own, every 15 minutes, as soon as the scheduled task “conso-token-feed” is set up in the Claude desktop app (see the user guide). If it was just created, wait 15 to 20 minutes; the Claude app must stay open.",
        "Pour l’instant, Conso Token n’a reçu aucun chiffre. Ils arrivent tout seuls, toutes les 15 minutes, dès que la tâche planifiée « conso-token-feed » est en place dans l’appli Claude de bureau (voir le mode d’emploi). Si elle vient d’être créée, patiente 15 à 20 minutes ; l’appli Claude doit rester ouverte.",
        "Por ahora, Conso Token no ha recibido ninguna cifra. Llegan solas, cada 15 minutos, en cuanto la tarea programada «conso-token-feed» está activa en la app de escritorio de Claude (consulta el manual). Si acaba de crearse, espera de 15 a 20 minutos; la app de Claude debe seguir abierta.",
        "Bisher hat Conso Token noch keine Zahlen erhalten. Sie kommen von selbst alle 15 Minuten, sobald die geplante Aufgabe „conso-token-feed“ in der Claude-Desktop-App eingerichtet ist (siehe Anleitung). Wurde sie gerade erst erstellt, warte 15 bis 20 Minuten; die Claude-App muss geöffnet bleiben.",
        "Por enquanto, o Conso Token não recebeu nenhum número. Eles chegam sozinhos, a cada 15 minutos, assim que a tarefa agendada “conso-token-feed” estiver ativa no app do Claude para computador (veja o guia de uso). Se ela acabou de ser criada, aguarde de 15 a 20 minutos; o app do Claude precisa ficar aberto.",
        "Per ora Conso Token non ha ricevuto alcun dato. Arrivano da soli, ogni 15 minuti, appena l’attività pianificata «conso-token-feed» è attiva nell’app desktop di Claude (vedi le istruzioni). Se è stata appena creata, attendi 15–20 minuti; l’app di Claude deve restare aperta.",
        "目前 Conso Token 还没有收到任何数字。只要 Claude 桌面应用中设置好计划任务“conso-token-feed”，数字就会每 15 分钟自动送达（见使用说明）。如果任务刚创建，请等待 15 到 20 分钟；Claude 应用必须保持打开。",
        "現在、Conso Token はまだ数値を受信していません。Claude デスクトップアプリにスケジュールタスク「conso-token-feed」が設定されると、15分ごとに自動で届きます（使い方を参照）。作成したばかりの場合は、15〜20分お待ちください。Claude アプリは開いたままにしておいてください。"
    ],
    AccueilNouvelleTitre => [
        "A new week begins",
        "Une nouvelle semaine commence",
        "Empieza una nueva semana",
        "Eine neue Woche beginnt",
        "Começa uma nova semana",
        "Inizia una nuova settimana",
        "新的一周开始了",
        "新しい週が始まります"
    ],
    AccueilNouvelleMessage => [
        "Your plan has just been reset to 100%. The new week’s figures will arrive here on their own, usually within a quarter of an hour (the Claude desktop app must be open).",
        "Ton forfait vient d’être remis à 100 %. Les chiffres de la nouvelle semaine arriveront ici tout seuls, en général dans un quart d’heure (l’appli Claude de bureau doit être ouverte).",
        "Tu plan acaba de volver al 100 %. Las cifras de la nueva semana llegarán aquí solas, normalmente en un cuarto de hora (la app de escritorio de Claude debe estar abierta).",
        "Dein Abo wurde gerade wieder auf 100 % gesetzt. Die Zahlen der neuen Woche kommen von selbst hier an, meist innerhalb einer Viertelstunde (die Claude-Desktop-App muss geöffnet sein).",
        "O seu plano acabou de voltar a 100%. Os números da nova semana chegarão aqui sozinhos, em geral em um quarto de hora (o app do Claude para computador precisa estar aberto).",
        "Il tuo piano è appena tornato al 100%. I dati della nuova settimana arriveranno qui da soli, in genere entro un quarto d’ora (l’app desktop di Claude deve essere aperta).",
        "你的套餐刚刚恢复到 100%。新一周的数字会自动送达这里，通常在一刻钟内（Claude 桌面应用必须保持打开）。",
        "プランが100%に戻りました。新しい週の数値は自動でここに届きます（通常は15分以内、Claude デスクトップアプリを開いておく必要があります）。"
    ],
    // l'ancienne page « branche la ligne d'état » (cachée aujourd'hui, gardée dans l'interface)
    BrancheEtape1 => [
        "1.  Open this file with Notepad (if it does not exist, create it):",
        "1.  Ouvre ce fichier avec le Bloc-notes (s’il n’existe pas, crée-le) :",
        "1.  Abre este archivo con el Bloc de notas (si no existe, créalo):",
        "1.  Öffne diese Datei mit dem Editor (falls sie nicht existiert, lege sie an):",
        "1.  Abra este arquivo com o Bloco de Notas (se não existir, crie-o):",
        "1.  Apri questo file con il Blocco note (se non esiste, crealo):",
        "1.  用记事本打开此文件（如果不存在，请新建）：",
        "1.  このファイルをメモ帳で開きます（なければ作成してください）："
    ],
    BrancheEtape2 => [
        "2.  Add these lines to it (if the file already contains something else, add only the “statusLine” part):",
        "2.  Ajoute ces lignes dedans (si le fichier contient déjà autre chose, ajoute seulement la partie « statusLine ») :",
        "2.  Añade estas líneas (si el archivo ya contiene otra cosa, añade solo la parte «statusLine»):",
        "2.  Füge diese Zeilen ein (enthält die Datei bereits etwas anderes, ergänze nur den Teil „statusLine“):",
        "2.  Adicione estas linhas nele (se o arquivo já tiver outro conteúdo, adicione só a parte “statusLine”):",
        "2.  Aggiungi queste righe (se il file contiene già altro, aggiungi solo la parte «statusLine»):",
        "2.  把这些行加进去（如果文件里已有其他内容，只需添加“statusLine”部分）：",
        "2.  次の行を追加します（ファイルに他の内容がある場合は「statusLine」の部分だけを追加）："
    ],
    BrancheEtape3 => [
        "3.  Restart Claude Code. As soon as you use it, the figures arrive here on their own.",
        "3.  Redémarre Claude Code. Dès que tu t’en sers, les chiffres arrivent ici tout seuls.",
        "3.  Reinicia Claude Code. En cuanto lo uses, las cifras llegan aquí solas.",
        "3.  Starte Claude Code neu. Sobald du es benutzt, kommen die Zahlen von selbst hier an.",
        "3.  Reinicie o Claude Code. Assim que você o usar, os números chegam aqui sozinhos.",
        "3.  Riavvia Claude Code. Appena lo usi, i dati arrivano qui da soli.",
        "3.  重新启动 Claude Code。一旦你开始使用，数字就会自动出现在这里。",
        "3.  Claude Code を再起動します。使い始めると、数値が自動でここに届きます。"
    ],

    // ------------------------------------------------ « À propos »
    AproposTitre => [
        "About Conso Token",
        "À propos de Conso Token",
        "Acerca de Conso Token",
        "Über Conso Token",
        "Sobre o Conso Token",
        "Informazioni su Conso Token",
        "关于 Conso Token",
        "Conso Token について"
    ],
    // {0} = numéro de version
    AproposCorps => [
        "Version {0}\nConso Token shows you where your Claude usage stands. It guesses nothing and predicts nothing: it only reads the figures your Claude app passes to it and the token counters of your conversations (never their content), on your computer. Nothing is sent over the Internet.",
        "Version {0}\nConso Token te montre où en est ta consommation de Claude. Il ne devine rien et ne prédit rien : il ne lit que les chiffres que ton appli Claude lui transmet et les compteurs de tokens de tes conversations (jamais leur contenu), sur ton ordinateur. Rien n’est envoyé sur Internet.",
        "Versión {0}\nConso Token te muestra cómo va tu consumo de Claude. No adivina ni predice nada: solo lee las cifras que le pasa tu app de Claude y los contadores de tokens de tus conversaciones (nunca su contenido), en tu ordenador. No se envía nada por Internet.",
        "Version {0}\nConso Token zeigt dir, wie weit dein Claude-Verbrauch ist. Es rät nichts und sagt nichts voraus: Es liest nur die Zahlen, die deine Claude-App weitergibt, und die Token-Zähler deiner Unterhaltungen (nie deren Inhalt), auf deinem Computer. Nichts wird über das Internet gesendet.",
        "Versão {0}\nO Conso Token mostra como está o seu consumo do Claude. Ele não adivinha nem prevê nada: lê apenas os números que o seu app do Claude lhe passa e os contadores de tokens das suas conversas (nunca o conteúdo), no seu computador. Nada é enviado pela Internet.",
        "Versione {0}\nConso Token ti mostra a che punto è il tuo consumo di Claude. Non indovina e non prevede nulla: legge solo i numeri che la tua app Claude gli trasmette e i contatori di token delle tue conversazioni (mai il contenuto), sul tuo computer. Non viene inviato nulla su Internet.",
        "版本 {0}\nConso Token 展示你的 Claude 用量进展。它不做任何猜测或预测：只读取你的 Claude 应用传来的数字，以及对话的 token 计数（绝不读取对话内容），全部在你的电脑上完成。不会通过互联网发送任何内容。",
        "バージョン {0}\nConso Token は、Claude の使用状況を表示します。推測や予測は一切行いません。読み取るのは、Claude アプリから渡される数値と、会話のトークンカウンター（内容は決して読みません）だけで、すべてお使いのパソコン上で行われます。インターネットには何も送信されません。"
    ],
    AproposSlint => [
        "Built with Slint (slint.dev), under the “Slint Royalty-free 2.0” license",
        "Construit avec Slint (slint.dev), sous la licence « Slint Royalty-free 2.0 »",
        "Creado con Slint (slint.dev), bajo la licencia «Slint Royalty-free 2.0»",
        "Erstellt mit Slint (slint.dev) unter der Lizenz „Slint Royalty-free 2.0“",
        "Feito com Slint (slint.dev), sob a licença “Slint Royalty-free 2.0”",
        "Realizzato con Slint (slint.dev), con licenza «Slint Royalty-free 2.0»",
        "使用 Slint (slint.dev) 构建，许可证为“Slint Royalty-free 2.0”",
        "Slint（slint.dev）で構築。ライセンス：「Slint Royalty-free 2.0」"
    ],
    AproposPolices => [
        "Fonts: Cormorant Garamond (© 2015 The Cormorant Project Authors) and Jost (© 2020 The Jost Project Authors), under the SIL Open Font License 1.1.\nThe full license texts are in the “licences” folder shipped with the program.",
        "Polices : Cormorant Garamond (© 2015 The Cormorant Project Authors) et Jost (© 2020 The Jost Project Authors), sous la licence SIL Open Font License 1.1.\nLes textes complets des licences se trouvent dans le dossier « licences » livré avec le programme.",
        "Fuentes: Cormorant Garamond (© 2015 The Cormorant Project Authors) y Jost (© 2020 The Jost Project Authors), bajo la licencia SIL Open Font License 1.1.\nLos textos completos de las licencias están en la carpeta «licences» incluida con el programa.",
        "Schriften: Cormorant Garamond (© 2015 The Cormorant Project Authors) und Jost (© 2020 The Jost Project Authors) unter der SIL Open Font License 1.1.\nDie vollständigen Lizenztexte befinden sich im Ordner „licences“, der dem Programm beiliegt.",
        "Fontes: Cormorant Garamond (© 2015 The Cormorant Project Authors) e Jost (© 2020 The Jost Project Authors), sob a licença SIL Open Font License 1.1.\nOs textos completos das licenças estão na pasta “licences” que acompanha o programa.",
        "Caratteri: Cormorant Garamond (© 2015 The Cormorant Project Authors) e Jost (© 2020 The Jost Project Authors), con licenza SIL Open Font License 1.1.\nI testi completi delle licenze si trovano nella cartella «licences» fornita con il programma.",
        "字体：Cormorant Garamond（© 2015 The Cormorant Project Authors）和 Jost（© 2020 The Jost Project Authors），许可证为 SIL Open Font License 1.1。\n完整的许可证文本位于随程序提供的“licences”文件夹中。",
        "フォント：Cormorant Garamond（© 2015 The Cormorant Project Authors）と Jost（© 2020 The Jost Project Authors）、ライセンスは SIL Open Font License 1.1。\nライセンスの全文は、プログラムに同梱の「licences」フォルダーにあります。"
    ],
    BoutonRetour => ["BACK", "RETOUR", "VOLVER", "ZURÜCK", "VOLTAR", "INDIETRO", "返回", "戻る"],

    // ------------------------------------------------ le fichier laissé si la fenêtre ne s'ouvre pas
    ErreurFenetre => [
        "Conso Token could not open its window (graphics card?).\n",
        "Conso Token n'a pas pu ouvrir sa fenêtre (carte graphique ?).\n",
        "Conso Token no ha podido abrir su ventana (¿tarjeta gráfica?).\n",
        "Conso Token konnte sein Fenster nicht öffnen (Grafikkarte?).\n",
        "O Conso Token não conseguiu abrir a janela (placa de vídeo?).\n",
        "Conso Token non è riuscito ad aprire la finestra (scheda grafica?).\n",
        "Conso Token 无法打开窗口（显卡问题？）。\n",
        "Conso Token はウィンドウを開けませんでした（グラフィックカードの問題？）。\n"
    ],
}

fn cellule(c: Cle, langue: langue::Langue) -> &'static str {
    TABLE[c as usize][langue.index()]
}

/// Le texte `c` dans la langue en cours (anglais si la table n'a pas de cellule, ce que le test interdit).
pub fn t(c: Cle) -> &'static str {
    cellule(c, langue::courante())
}

/// Le texte `c` dont les `{0}`, `{1}`… sont remplacés par `args`.
pub fn fmt(c: Cle, args: &[&dyn Display]) -> String {
    remplir(t(c), args)
}

fn remplir(modele: &str, args: &[&dyn Display]) -> String {
    let mut sortie = String::with_capacity(modele.len() + 16);
    let mut reste = modele;
    while let Some(debut) = reste.find('{') {
        sortie.push_str(&reste[..debut]);
        let suite = &reste[debut..];
        let numero = suite.strip_prefix('{').and_then(|s| s.split_once('}')).and_then(|(n, _)| n.parse::<usize>().ok());
        match (numero, suite.find('}')) {
            (Some(n), Some(fin)) => {
                if let Some(a) = args.get(n) {
                    sortie.push_str(&a.to_string());
                }
                reste = &suite[fin + 1..];
            }
            _ => {
                sortie.push('{');
                reste = &suite[1..];
            }
        }
    }
    sortie.push_str(reste);
    sortie
}

/// Le nom court du jour de la semaine (`rang` 0 = lundi).
pub fn jour_court(rang: usize) -> &'static str {
    TABLE[Cle::JourC0 as usize + rang.min(6)][langue::courante().index()]
}

/// Le nom du jour de la semaine en toutes lettres (`rang` 0 = lundi).
pub fn jour_long(rang: usize) -> &'static str {
    TABLE[Cle::JourL0 as usize + rang.min(6)][langue::courante().index()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::langue::TOUTES;

    /// Les paramètres `{n}` d'une cellule, triés, sans doublon.
    fn parametres(texte: &str) -> Vec<String> {
        let mut v = Vec::new();
        let mut reste = texte;
        while let Some(i) = reste.find('{') {
            let Some(j) = reste[i..].find('}') else { break };
            v.push(reste[i..=i + j].to_string());
            reste = &reste[i + j + 1..];
        }
        v.sort();
        v.dedup();
        v
    }

    #[test]
    fn la_table_a_une_ligne_par_cle() {
        assert_eq!(CLES.len(), TABLE.len());
        for (i, c) in CLES.iter().enumerate() {
            assert_eq!(*c as usize, i, "la clé {c:?} n'est pas à sa place");
        }
    }

    #[test]
    fn aucune_cellule_n_est_vide() {
        for (i, ligne) in TABLE.iter().enumerate() {
            for (l, cellule) in TOUTES.iter().zip(ligne) {
                assert!(!cellule.trim().is_empty(), "{:?} est vide en {}", CLES[i], l.code());
            }
        }
    }

    #[test]
    fn les_parametres_sont_les_memes_dans_les_huit_langues() {
        for (i, ligne) in TABLE.iter().enumerate() {
            let attendu = parametres(ligne[0]);
            for (l, cellule) in TOUTES.iter().zip(ligne) {
                assert_eq!(parametres(cellule), attendu, "{:?} : les paramètres de {} diffèrent de ceux de l'anglais", CLES[i], l.code());
                // pas d'accolade perdue : tout `{` ouvre un `{n}` complet
                assert_eq!(cellule.matches('{').count(), cellule.matches('}').count(), "{:?} en {} : accolades déséquilibrées", CLES[i], l.code());
                for p in parametres(cellule) {
                    assert!(p[1..p.len() - 1].parse::<usize>().is_ok(), "{:?} en {} : paramètre {p} illisible", CLES[i], l.code());
                }
            }
        }
    }

    #[test]
    fn les_colonnes_sont_dans_l_ordre_des_langues() {
        assert_eq!(TABLE[Cle::JourL3 as usize], ["Thursday", "jeudi", "jueves", "Donnerstag", "quinta-feira", "giovedì", "星期四", "木曜日"]);
        for (l, attendu) in TOUTES.iter().zip(["Thursday", "jeudi", "jueves", "Donnerstag", "quinta-feira", "giovedì", "星期四", "木曜日"]) {
            langue::avec(*l, || assert_eq!(jour_long(3), attendu));
        }
    }

    #[test]
    fn un_texte_a_trous_se_remplit() {
        langue::avec(langue::Langue::Fr, || {
            assert_eq!(fmt(Cle::AgeMinutes, &[&5]), "il y a 5 min");
            assert_eq!(fmt(Cle::Date, &[&"05", &"11"]), "05/11");
        });
        langue::avec(langue::Langue::Ja, || assert_eq!(fmt(Cle::Date, &[&"5", &"11"]), "11月5日"));
        langue::avec(langue::Langue::En, || assert_eq!(fmt(Cle::Date, &[&"05", &"11"]), "11/05"));
        // un paramètre sans valeur disparaît, une accolade seule reste
        assert_eq!(remplir("a {3} b", &[]), "a  b");
        assert_eq!(remplir("a { b", &[]), "a { b");
    }
}
