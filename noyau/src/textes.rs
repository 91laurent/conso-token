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

    // ------------------------------------------------ la page « Conversations » et la fenêtre de réglage
    // Les noms des groupes se tapent à la main (rien à traduire) ; seuls « Groupe {0} » et « Toutes les
    // conversations » sont des textes de l'appli. « Passation » = le moment de passer le relais à une
    // nouvelle conversation, quand le contexte de celle-ci est trop plein.
    OngletConversations => ["Conversations", "Conversations", "Conversaciones", "Unterhaltungen", "Conversas", "Conversazioni", "对话", "会話"],
    ConvTitre => [
        "Open conversations",
        "Conversations ouvertes",
        "Conversaciones abiertas",
        "Offene Unterhaltungen",
        "Conversas abertas",
        "Conversazioni aperte",
        "进行中的对话",
        "開いている会話"
    ],
    ConvSousTitre => [
        "context at the last exchange · window ≈ set by model (adjustable in Settings) · hover a long name to scroll it · click a tag to move the whole folder to the other group",
        "contexte au dernier échange · fenêtre ≈ selon le modèle (réglable dans Config) · le survol d’un nom long le fait défiler · un clic sur l’étiquette change le camp du dossier",
        "contexto en el último intercambio · ventana ≈ según el modelo (ajustable en Ajustes) · pasa el cursor sobre un nombre largo para verlo entero · un clic en la etiqueta cambia de grupo toda la carpeta",
        "Kontext beim letzten Austausch · Fenster ≈ je nach Modell (einstellbar unter Optionen) · über einem langen Namen mit der Maus verweilen lässt ihn laufen · ein Klick auf das Kürzel verschiebt den ganzen Ordner in die andere Gruppe",
        "contexto na última troca · janela ≈ conforme o modelo (ajustável em Ajustes) · passe o mouse sobre um nome longo para vê-lo inteiro · um clique na etiqueta muda a pasta inteira de grupo",
        "contesto all’ultimo scambio · finestra ≈ secondo il modello (regolabile in Opzioni) · passa il mouse su un nome lungo per farlo scorrere · un clic sull’etichetta sposta tutta la cartella nell’altro gruppo",
        "上一次交流时的上下文 · 窗口 ≈ 按模型而定（可在“设置”中调整）· 鼠标悬停在长名称上会滚动显示 · 点击标签可将整个文件夹移到另一组",
        "直近のやり取り時点のコンテキスト · ウィンドウ ≈ モデルごと（「設定」で変更可）· 長い名前にカーソルを乗せるとスクロール · ラベルをクリックするとフォルダごと別のグループへ"
    ],
    ConvLegende => [
        "cream mark: hand-off · dotted mark: warning · the bar goes from 0 to 100% of the window · “summarized”: the conversation was compacted (hand-off now)",
        "trait crème : passation · pointillé : attention · barre de 0 à 100 % de la fenêtre · « résumée » : la conversation a été compactée (passation tout de suite)",
        "marca crema: relevo · línea de puntos: atención · la barra va del 0 al 100 % de la ventana · «resumida»: la conversación se compactó (relevo inmediato)",
        "cremefarbene Marke: Übergabe · gepunktete Marke: Achtung · Balken von 0 bis 100 % des Fensters · „zusammengefasst“: die Unterhaltung wurde komprimiert (sofort übergeben)",
        "marca creme: transferência · tracejado: atenção · a barra vai de 0 a 100% da janela · «resumida»: a conversa foi compactada (transferência imediata)",
        "segno crema: consegna · tratteggio: attenzione · la barra va da 0 a 100% della finestra · «riassunta»: la conversazione è stata compattata (consegna subito)",
        "奶油色标记：交接 · 虚线：注意 · 进度条为窗口的 0 到 100% · “已摘要”：对话已被压缩（立即交接）",
        "クリーム色の線：引き継ぎ · 点線：注意 · バーはウィンドウの0〜100% · 「要約済み」：会話が圧縮されました（すぐに引き継ぎ）"
    ],
    // {0} = seuil d'attention, {1} = seuil de passation (en % de la fenêtre)
    ConvSeuils => [
        "warning {0} · hand-off {1}",
        "attention {0} · passation {1}",
        "atención {0} · relevo {1}",
        "Achtung {0} · Übergabe {1}",
        "atenção {0} · transferência {1}",
        "attenzione {0} · consegna {1}",
        "注意 {0} · 交接 {1}",
        "注意 {0} · 引き継ぎ {1}"
    ],
    // {0} = pourcentage de la fenêtre déjà pris (avec son signe %), {1} = seuil de passation
    ConvSur => ["≈ {0} of {1}", "≈ {0} sur {1}", "≈ {0} de {1}", "≈ {0} von {1}", "≈ {0} de {1}", "≈ {0} su {1}", "≈ {0} / {1}", "≈ {0} / {1}"],
    // {0} = points de pourcentage qui restent avant la passation
    ConvReste => ["{0} left", "reste {0}", "quedan {0}", "noch {0}", "faltam {0}", "restano {0}", "还剩 {0}", "残り {0}"],
    ConvPassation => ["hand-off", "passation", "relevo", "Übergabe", "transferência", "consegna", "交接", "引き継ぎ"],
    ConvResumee => ["summarized", "résumée", "resumida", "zusammengefasst", "resumida", "riassunta", "已摘要", "要約済み"],
    // {0} = nombre de conversations de moins de 24 h, repliées
    ConvAutres => [
        "+ {0} more, under 24 h",
        "+ {0} autres, moins de 24 h",
        "+ {0} más, menos de 24 h",
        "+ {0} weitere, unter 24 Std.",
        "+ {0} outras, menos de 24 h",
        "+ altre {0}, meno di 24 h",
        "另有 {0} 个，24 小时内",
        "他 {0} 件（24時間以内）"
    ],
    ConvVide => [
        "No active conversation in this group.",
        "Aucune conversation active dans ce groupe.",
        "Ninguna conversación activa en este grupo.",
        "Keine aktive Unterhaltung in dieser Gruppe.",
        "Nenhuma conversa ativa neste grupo.",
        "Nessuna conversazione attiva in questo gruppo.",
        "此组中没有活跃的对话。",
        "このグループにアクティブな会話はありません。"
    ],
    ConvIndispo => [
        "Conversations cannot be read: the format of Claude’s transcripts may have changed. Nothing is shown rather than wrong figures.",
        "Les conversations ne peuvent pas être lues : le format des transcriptions de Claude a peut-être changé. Rien n’est affiché plutôt que de faux chiffres.",
        "No se pueden leer las conversaciones: puede que haya cambiado el formato de las transcripciones de Claude. Es mejor no mostrar nada que mostrar cifras falsas.",
        "Die Unterhaltungen lassen sich nicht lesen: Das Format der Claude-Protokolle hat sich möglicherweise geändert. Lieber nichts anzeigen als falsche Zahlen.",
        "Não foi possível ler as conversas: o formato das transcrições do Claude pode ter mudado. Melhor não mostrar nada do que números errados.",
        "Impossibile leggere le conversazioni: il formato delle trascrizioni di Claude potrebbe essere cambiato. Meglio non mostrare nulla che dati sbagliati.",
        "无法读取对话：Claude 记录文件的格式可能已更改。宁可不显示，也不显示错误的数字。",
        "会話を読み取れません。Claude の記録ファイルの形式が変わった可能性があります。誤った数値を表示するより、何も表示しません。"
    ],
    // {0} = 1 ou 2
    GroupeN => ["Group {0}", "Groupe {0}", "Grupo {0}", "Gruppe {0}", "Grupo {0}", "Gruppo {0}", "第 {0} 组", "グループ{0}"],
    GroupeTous => [
        "All conversations",
        "Toutes les conversations",
        "Todas las conversaciones",
        "Alle Unterhaltungen",
        "Todas as conversas",
        "Tutte le conversazioni",
        "所有对话",
        "すべての会話"
    ],
    BoutonConfig => ["SETTINGS", "CONFIG", "AJUSTES", "OPTIONEN", "AJUSTES", "OPZIONI", "设置", "設定"],
    ConfigTitre => ["Settings", "Configuration", "Configuración", "Einstellungen", "Configuração", "Impostazioni", "设置", "設定"],
    ConfigSeparer => [
        "Split into two groups",
        "Séparer en deux groupes",
        "Separar en dos grupos",
        "In zwei Gruppen aufteilen",
        "Separar em dois grupos",
        "Dividere in due gruppi",
        "分为两组",
        "2つのグループに分ける"
    ],
    // {0} = 1 ou 2, {1} = nom du groupe tapé à la main
    ConfigGroupeNom => ["Group {0}: {1}", "Groupe {0} : {1}", "Grupo {0}: {1}", "Gruppe {0}: {1}", "Grupo {0}: {1}", "Gruppo {0}: {1}", "第 {0} 组：{1}", "グループ{0}：{1}"],
    ConfigAttention => ["Warning", "Attention", "Atención", "Achtung", "Atenção", "Attenzione", "注意", "注意"],
    ConfigPassation => ["Hand-off", "Passation", "Relevo", "Übergabe", "Transferência", "Consegna", "交接", "引き継ぎ"],
    // la barre d'essai : {0} = pourcentage d'exemple (avec son signe %), {1} = points qui restent
    ConfigEssaiOk => [
        "Example at {0}: {1} left before the hand-off",
        "Exemple à {0} : reste {1} avant la passation",
        "Ejemplo al {0}: quedan {1} antes del relevo",
        "Beispiel bei {0}: noch {1} bis zur Übergabe",
        "Exemplo em {0}: faltam {1} para a transferência",
        "Esempio al {0}: restano {1} prima della consegna",
        "示例 {0}：距离交接还剩 {1}",
        "例：{0} のとき、引き継ぎまで残り {1}"
    ],
    ConfigEssaiAttention => [
        "Example at {0}: warning, {1} left",
        "Exemple à {0} : attention, reste {1}",
        "Ejemplo al {0}: atención, quedan {1}",
        "Beispiel bei {0}: Achtung, noch {1}",
        "Exemplo em {0}: atenção, faltam {1}",
        "Esempio al {0}: attenzione, restano {1}",
        "示例 {0}：注意，还剩 {1}",
        "例：{0} のとき、注意、残り {1}"
    ],
    ConfigEssaiPassation => [
        "Example at {0}: hand-off",
        "Exemple à {0} : passation",
        "Ejemplo al {0}: relevo",
        "Beispiel bei {0}: Übergabe",
        "Exemplo em {0}: transferência",
        "Esempio al {0}: consegna",
        "示例 {0}：交接",
        "例：{0} のとき、引き継ぎ"
    ],
    ConfigFenetre => [
        "Context window, by model",
        "Fenêtre de contexte, par modèle",
        "Ventana de contexto, por modelo",
        "Kontextfenster je Modell",
        "Janela de contexto, por modelo",
        "Finestra di contesto, per modello",
        "各模型的上下文窗口",
        "モデルごとのコンテキストウィンドウ"
    ],
    // {0} = le plus grand contexte déjà vu pour ce modèle
    ConfigVu => [
        "seen up to {0} tokens",
        "vu jusqu’à {0} tokens",
        "visto hasta {0} tokens",
        "gesehen bis {0} Tokens",
        "visto até {0} tokens",
        "visto fino a {0} token",
        "最高见过 {0} 个 token",
        "最大 {0} トークンまで確認"
    ],
    // {0} = la fenêtre que « Auto » donnerait
    ConfigAutoVaut => ["Auto: {0}", "Auto : {0}", "Auto: {0}", "Auto: {0}", "Auto: {0}", "Auto: {0}", "自动：{0}", "自動：{0}"],
    ConfigAuto => ["Auto", "Auto", "Auto", "Auto", "Auto", "Auto", "自动", "自動"],
    ConfigFenetreNote => [
        "The window is not written in any file: “Auto” takes 1,000,000 if this model has ever gone past 200,000 tokens, otherwise 200,000. The percentage always stays approximate (≈).",
        "La fenêtre n’est écrite dans aucun fichier : « Auto » prend 1 000 000 si ce modèle a déjà dépassé 200 000 tokens, sinon 200 000. Le pourcentage reste toujours un « ≈ ».",
        "La ventana no está escrita en ningún archivo: «Auto» toma 1 000 000 si este modelo ya ha superado los 200 000 tokens, y si no, 200 000. El porcentaje siempre es aproximado (≈).",
        "Das Fenster steht in keiner Datei: „Auto“ nimmt 1 000 000, wenn dieses Modell schon über 200 000 Tokens hinausgekommen ist, sonst 200 000. Der Prozentwert bleibt immer ein Näherungswert (≈).",
        "A janela não está escrita em nenhum arquivo: «Auto» usa 1 000 000 se este modelo já passou de 200 000 tokens, senão 200 000. A porcentagem é sempre aproximada (≈).",
        "La finestra non è scritta in nessun file: «Auto» usa 1 000 000 se questo modello ha già superato i 200 000 token, altrimenti 200 000. La percentuale resta sempre approssimativa (≈).",
        "窗口大小没有写在任何文件里：如果该模型曾超过 200 000 个 token，“自动”取 1 000 000，否则取 200 000。百分比始终是近似值（≈）。",
        "ウィンドウの大きさはどのファイルにも書かれていません。「自動」は、そのモデルが200 000トークンを超えたことがあれば1 000 000、なければ200 000とします。パーセンテージは常に概算（≈）です。"
    ],
    ConfigAlerte => [
        "Blinking dot on the Conversations tab",
        "Point clignotant sur le bouton Conversations",
        "Punto parpadeante en el botón Conversaciones",
        "Blinkender Punkt auf dem Reiter Unterhaltungen",
        "Ponto piscante no botão Conversas",
        "Punto lampeggiante sul pulsante Conversazioni",
        "在“对话”按钮上显示闪烁的圆点",
        "「会話」ボタンに点滅するドットを表示"
    ],
    ConfigAnimer => [
        "Animate the figures on the Week screen",
        "Animer les chiffres de l’écran Semaine",
        "Animar las cifras de la pantalla Semana",
        "Zahlen auf dem Bildschirm Woche animieren",
        "Animar os números da tela Semana",
        "Animare i numeri della schermata Settimana",
        "为“本周”页面的数字添加动画",
        "「週」画面の数字をアニメーションで表示"
    ],
    ConfigPied => [
        "Changes apply right away.",
        "Les changements s’appliquent tout de suite.",
        "Los cambios se aplican al instante.",
        "Änderungen gelten sofort.",
        "As alterações valem na hora.",
        "Le modifiche si applicano subito.",
        "更改立即生效。",
        "変更はすぐに反映されます。"
    ],
    ConfigRetablir => ["Reset", "Rétablir", "Restablecer", "Zurücksetzen", "Restaurar", "Ripristina", "恢复默认", "初期値に戻す"],
    // Le premier clic sur « Rétablir » change le bouton en ce texte ; un deuxième clic (dans les 5 s) rétablit.
    ConfigRetablirConfirmer => ["Click again to confirm", "Cliquer pour confirmer", "Haz clic para confirmar", "Zum Bestätigen klicken", "Clique para confirmar", "Clicca per confermare", "再次点击确认", "もう一度押して確定"],

    // ------------------------------------------------ la fenêtre d'accueil de la page « Conversations »
    // Montrée une fois, à la première ouverture de la page : c'est l'utilisateur qui désigne les dossiers du
    // premier groupe et qui nomme les groupes (l'appli ne peut pas deviner le rôle d'une conversation).
    // Le bouton « Séparer en deux groupes » est celui de Config (`ConfigSeparer`).
    AccueilTitre => [
        "Group your conversations?",
        "Regrouper tes conversations ?",
        "¿Agrupar tus conversaciones?",
        "Unterhaltungen gruppieren?",
        "Agrupar suas conversas?",
        "Raggruppare le conversazioni?",
        "要把对话分组吗？",
        "会話をグループに分けますか？"
    ],
    AccueilTexte => [
        "Conso Token cannot know which conversation plays which role (for example one that directs the others). You decide: tick the folders that form the first group and name each group. Each group has its own thresholds. You can change everything at any time with “Choose groups…”.",
        "Conso Token ne peut pas savoir quel rôle joue chaque conversation (par exemple celle qui pilote les autres). C’est toi qui le dis : coche les dossiers qui forment le premier groupe et donne un nom à chaque groupe. Chaque groupe a ses propres seuils. Tu pourras tout changer à tout moment avec « Choisir les groupes… ».",
        "Conso Token no puede saber qué papel tiene cada conversación (por ejemplo, una que dirige a las demás). Lo decides tú: marca las carpetas que forman el primer grupo y ponle nombre a cada grupo. Cada grupo tiene sus propios umbrales. Podrás cambiarlo todo en cualquier momento con «Elegir grupos…».",
        "Conso Token kann nicht wissen, welche Unterhaltung welche Rolle hat (zum Beispiel eine, die die anderen steuert). Das legst du fest: Hake die Ordner an, die die erste Gruppe bilden, und gib jeder Gruppe einen Namen. Jede Gruppe hat eigene Schwellen. Alles lässt sich jederzeit mit „Gruppen wählen…“ ändern.",
        "O Conso Token não consegue saber que papel cada conversa tem (por exemplo, uma que dirige as outras). Quem decide é você: marque as pastas que formam o primeiro grupo e dê um nome a cada grupo. Cada grupo tem seus próprios limites. Você pode mudar tudo a qualquer momento com «Escolher grupos…».",
        "Conso Token non può sapere che ruolo abbia ogni conversazione (per esempio una che dirige le altre). Decidi tu: spunta le cartelle che formano il primo gruppo e dai un nome a ciascun gruppo. Ogni gruppo ha le sue soglie. Potrai cambiare tutto in qualsiasi momento con «Scegli i gruppi…».",
        "Conso Token 无法知道每个对话扮演什么角色（例如负责指挥其他对话的那个）。由你来决定：勾选属于第一组的文件夹，并给每组起个名字。每组有各自的阈值。之后可随时通过“选择分组…”按钮更改。",
        "Conso Token は、どの会話がどんな役割か（たとえば他の会話を取りまとめるもの）を判断できません。あなたが決めます。最初のグループに入れるフォルダーにチェックを入れ、各グループに名前を付けてください。グループごとにしきい値があります。「グループを選ぶ…」ボタンから、いつでも変更できます。"
    ],
    AccueilDossiers => [
        "Folders of the first group",
        "Dossiers du premier groupe",
        "Carpetas del primer grupo",
        "Ordner der ersten Gruppe",
        "Pastas do primeiro grupo",
        "Cartelle del primo gruppo",
        "第一组的文件夹",
        "最初のグループのフォルダー"
    ],
    AccueilNom1 => [
        "Name of the first group (ticked folders)",
        "Nom du premier groupe (dossiers cochés)",
        "Nombre del primer grupo (carpetas marcadas)",
        "Name der ersten Gruppe (angehakte Ordner)",
        "Nome do primeiro grupo (pastas marcadas)",
        "Nome del primo gruppo (cartelle spuntate)",
        "第一组的名称（已勾选的文件夹）",
        "最初のグループの名前（チェックしたフォルダー）"
    ],
    AccueilNom2 => [
        "Name of the second group (all the others)",
        "Nom du second groupe (tous les autres)",
        "Nombre del segundo grupo (todas las demás)",
        "Name der zweiten Gruppe (alle anderen)",
        "Nome do segundo grupo (todas as outras)",
        "Nome del secondo gruppo (tutte le altre)",
        "第二组的名称（其余所有文件夹）",
        "2番目のグループの名前（それ以外すべて）"
    ],
    AccueilGarder => [
        "Keep one group",
        "Garder un seul groupe",
        "Mantener un solo grupo",
        "Bei einer Gruppe bleiben",
        "Manter um só grupo",
        "Tenere un solo gruppo",
        "只保留一组",
        "グループは1つのまま"
    ],
    // le bouton de Config qui rouvre la fenêtre d'accueil
    ConfigGroupes => [
        "Choose groups…",
        "Choisir les groupes…",
        "Elegir grupos…",
        "Gruppen wählen…",
        "Escolher grupos…",
        "Scegli i gruppi…",
        "选择分组…",
        "グループを選ぶ…"
    ],

    // ------------------------------------------------ masquer une conversation (1.3.4)
    // L'info-bulle du ✕ d'une ligne de la page « Conversations ».
    ConvMasquer => ["Hide", "Masquer", "Ocultar", "Ausblenden", "Ocultar", "Nascondi", "隐藏", "非表示"],
    // Config : la ligne « n conversations masquées » (une forme pour 1, une pour les autres : {0} = leur nombre) et son bouton.
    ConfigMasqueeUne => ["1 conversation hidden", "1 conversation masquée", "1 conversación oculta", "1 Unterhaltung ausgeblendet", "1 conversa oculta", "1 conversazione nascosta", "已隐藏 1 个对话", "非表示の会話：1件"],
    ConfigMasquees => ["{0} conversations hidden", "{0} conversations masquées", "{0} conversaciones ocultas", "{0} Unterhaltungen ausgeblendet", "{0} conversas ocultas", "{0} conversazioni nascoste", "已隐藏 {0} 个对话", "非表示の会話：{0}件"],
    ConfigReafficher => ["Show again", "Réafficher", "Mostrar de nuevo", "Wieder einblenden", "Mostrar novamente", "Mostra di nuovo", "重新显示", "再表示"],

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
        "Version {0}\nConso Token shows you where your Claude usage stands. It guesses nothing and predicts nothing: it only keeps the figures your Claude app passes to it, the token counters of your conversations and the title of each conversation as the Claude app shows it, on your computer. The title comes from the files already read for the counters and is kept in memory while it is displayed; it is never written or sent. The text of a message is never kept, written or sent. When you hide a conversation with ×, the app writes that conversation’s session identifier (a plain code, neither its title nor its contents) in its own data folder so that it stays hidden; it is forgotten once the conversation’s last message is more than 24 h old, or as soon as you click “Show again”. Nothing is sent over the Internet.",
        "Version {0}\nConso Token te montre où en est ta consommation de Claude. Il ne devine rien et ne prédit rien : il ne retient que les chiffres que ton appli Claude lui transmet, les compteurs de tokens de tes conversations et le titre de chaque conversation tel que l’appli Claude l’affiche, sur ton ordinateur. Le titre est lu dans les fichiers déjà lus pour les compteurs et est gardé en mémoire le temps de l’affichage ; il n’est jamais écrit ni envoyé. Le texte d’un message n’est jamais gardé, ni écrit, ni envoyé. Quand tu masques une conversation avec ×, l’appli écrit l’identifiant de session de cette conversation (un simple code, ni son titre ni son contenu) dans son dossier de données pour qu’elle reste masquée ; il est oublié quand son dernier message a plus de 24 h, ou dès que tu cliques sur « Réafficher ». Rien n’est envoyé sur Internet.",
        "Versión {0}\nConso Token te muestra cómo va tu consumo de Claude. No adivina ni predice nada: solo conserva las cifras que le pasa tu app de Claude, los contadores de tokens de tus conversaciones y el título de cada conversación tal como lo muestra la app de Claude, en tu ordenador. El título se lee en los archivos que ya se leen para los contadores y se guarda en la memoria mientras se muestra; nunca se escribe ni se envía. El texto de un mensaje nunca se guarda, ni se escribe ni se envía. Cuando ocultas una conversación con ×, la app escribe el identificador de sesión de esa conversación (un simple código, ni su título ni su contenido) en su carpeta de datos para que siga oculta; se olvida cuando su último mensaje tiene más de 24 h, o en cuanto haces clic en «Mostrar de nuevo». No se envía nada por Internet.",
        "Version {0}\nConso Token zeigt dir, wie weit dein Claude-Verbrauch ist. Es rät nichts und sagt nichts voraus: Es behält nur die Zahlen, die deine Claude-App weitergibt, die Token-Zähler deiner Unterhaltungen und den Titel jeder Unterhaltung, so wie ihn die Claude-App anzeigt, auf deinem Computer. Der Titel wird in den Dateien gelesen, die ohnehin für die Zähler gelesen werden, und wird nur für die Anzeige im Speicher gehalten; er wird nie geschrieben oder gesendet. Der Text einer Nachricht wird nie behalten, geschrieben oder gesendet. Wenn du eine Unterhaltung mit × ausblendest, schreibt die App die Sitzungskennung dieser Unterhaltung (nur einen Code, weder Titel noch Inhalt) in ihren Datenordner, damit sie ausgeblendet bleibt; sie wird vergessen, sobald die letzte Nachricht älter als 24 Std. ist, oder sobald du auf „Wieder einblenden“ klickst. Nichts wird über das Internet gesendet.",
        "Versão {0}\nO Conso Token mostra como está o seu consumo do Claude. Ele não adivinha nem prevê nada: guarda apenas os números que o seu app do Claude lhe passa, os contadores de tokens das suas conversas e o título de cada conversa como o app do Claude o mostra, no seu computador. O título é lido nos arquivos que já são lidos para os contadores e é mantido na memória enquanto é exibido; nunca é gravado nem enviado. O texto de uma mensagem nunca é guardado, gravado nem enviado. Quando você oculta uma conversa com ×, o app grava o identificador de sessão dessa conversa (um simples código, nem o título nem o conteúdo) na sua pasta de dados para que ela continue oculta; ele é esquecido quando a última mensagem tem mais de 24 h, ou assim que você clica em “Mostrar novamente”. Nada é enviado pela Internet.",
        "Versione {0}\nConso Token ti mostra a che punto è il tuo consumo di Claude. Non indovina e non prevede nulla: conserva solo i numeri che la tua app Claude gli trasmette, i contatori di token delle tue conversazioni e il titolo di ogni conversazione così come lo mostra l’app Claude, sul tuo computer. Il titolo è letto nei file già letti per i contatori e viene tenuto in memoria finché è mostrato; non viene mai scritto né inviato. Il testo di un messaggio non viene mai conservato, scritto né inviato. Quando nascondi una conversazione con ×, l’app scrive l’identificatore di sessione di quella conversazione (un semplice codice, né il titolo né il contenuto) nella sua cartella dati perché resti nascosta; viene dimenticato quando l’ultimo messaggio ha più di 24 h, oppure appena clicchi su «Mostra di nuovo». Non viene inviato nulla su Internet.",
        "版本 {0}\nConso Token 展示你的 Claude 用量进展。它不做任何猜测或预测：只保留你的 Claude 应用传来的数字、对话的 token 计数，以及 Claude 应用所显示的每个对话的标题，全部在你的电脑上完成。标题取自已为计数而读取的文件，只在显示期间保存在内存中，绝不写入或发送；消息的文字从不保留，也绝不写入或发送。当你用 × 隐藏一个对话时，应用会把该对话的会话标识符（只是一串代码，不含标题和内容）写入自己的数据文件夹，使它保持隐藏；当其最后一条消息超过 24 小时，或你点击“重新显示”时，该标识符即被清除。不会通过互联网发送任何内容。",
        "バージョン {0}\nConso Token は、Claude の使用状況を表示します。推測や予測は一切行いません。保持するのは、Claude アプリから渡される数値、会話のトークンカウンター、そして Claude アプリに表示されるものと同じ各会話のタイトルだけで、すべてお使いのパソコン上で行われます。タイトルはカウンターのためにすでに読んでいるファイルから取得し、表示している間だけメモリに保持します。書き込みも送信もしません。メッセージの文章は一切保持せず、書き込みも送信もしません。× で会話を非表示にすると、アプリはその会話のセッション ID（ただのコードで、タイトルも内容も含みません）を自分のデータフォルダーに書き込み、非表示を保ちます。最後のメッセージから 24 時間が過ぎるか、「再表示」を押すと、この ID は消去されます。インターネットには何も送信されません。"
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
