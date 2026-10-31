// Conso Token : code commun aux deux pages (présentation et guide de lecture).
// Une ligne de texte = une clé et huit cellules, dans l'ordre ORDER (comme la table de l'appli).
const ORDER = ["en", "fr", "es", "de", "pt", "it", "zh", "ja"];
const HTML_LANG = { en: "en", fr: "fr", es: "es", de: "de", pt: "pt-BR", it: "it", zh: "zh-CN", ja: "ja" };
const L = {};
ORDER.forEach(l => { L[l] = {}; });
function add(cle, cellules) { ORDER.forEach((l, i) => { L[l][cle] = cellules[i]; }); }
function merge(ancien) { for (const l of ORDER) Object.assign(L[l], ancien[l] || {}); }

// ---------------------------------------------------------------- navigation et visite
add("nav_tour", ["The window", "La fenêtre", "La ventana", "Das Fenster", "A janela", "La finestra", "窗口", "ウィンドウ"]);
add("nav_guide", ["Reading guide", "Guide de lecture", "Guía de lectura", "Lesehilfe", "Guia de leitura", "Guida alla lettura", "阅读指南", "読み方ガイド"]);
add("nav_home", ["Presentation", "Présentation", "Presentación", "Vorstellung", "Apresentação", "Presentazione", "介绍", "紹介"]);
add("nav_how", ["Install", "Installer", "Instalar", "Installieren", "Instalar", "Installa", "安装", "導入"]);
add("nav_trust", ["Trust", "Confiance", "Confianza", "Vertrauen", "Confiança", "Fiducia", "可信", "安心"]);
add("tour_hint", ["Click a card to see what it says.", "Clique sur une carte pour voir ce qu'elle dit.", "Haz clic en una tarjeta para ver qué dice.", "Klicke auf eine Karte, um zu sehen, was sie sagt.", "Clique em um cartão para ver o que ele diz.", "Fai clic su una scheda per vedere cosa dice.", "点击一张卡片，查看它的含义。", "カードをクリックすると、その意味が表示されます。"]);
add("sc_avance", ["A week where you are ahead", "Une semaine en avance", "Una semana con margen", "Eine Woche mit Spielraum", "Uma semana com folga", "Una settimana con margine", "一周有富余", "余裕のある一週間"]);
add("sc_retard", ["A week where you are over", "Une semaine en retard", "Una semana pasada de ritmo", "Eine Woche über dem Soll", "Uma semana acima do ritmo", "Una settimana oltre il ritmo", "一周超出节奏", "ペースを超えた一週間"]);
add("shot_note", ["Real screenshots of the app. The numbers are made up for illustration.", "Vraies captures de l'appli. Les chiffres sont inventés pour l'illustration.", "Capturas reales de la app. Las cifras son inventadas para ilustrar.", "Echte Screenshots der App. Die Zahlen sind zur Illustration erfunden.", "Capturas reais do app. Os números são inventados para ilustrar.", "Schermate reali dell'app. I numeri sono inventati a scopo illustrativo.", "应用的真实截图，数字为示意而虚构。", "アプリの実際のスクリーンショットです。数字はイラスト用の架空のものです。"]);

// titres des cartes : ceux de l'appli, dans chaque langue
add("k_rythme", ["Your pace", "Ton rythme", "Tu ritmo", "Dein Tempo", "Seu ritmo", "Il tuo ritmo", "你的节奏", "あなたのペース"]);
add("k_reste", ["You have left", "Il te reste", "Te queda", "Du hast noch", "Você ainda tem", "Ti resta", "你还剩", "残り"]);
add("k_reset", ["Your plan resets to 100% in", "Ton forfait revient à 100 % dans", "Tu plan vuelve al 100 % en", "Dein Abo ist wieder voll in", "Seu plano volta a 100% em", "Il tuo piano torna al 100% tra", "额度恢复至 100% 还需", "100%に戻るまで"]);
add("k_semaine", ["Your week at a glance", "Ta semaine en un coup d’œil", "Tu semana de un vistazo", "Deine Woche auf einen Blick", "Sua semana em resumo", "La tua settimana a colpo d’occhio", "本周概览", "今週のひと目まとめ"]);
add("k_jours", ["Your usage, day by day", "Ta conso jour par jour", "Tu consumo día a día", "Dein Verbrauch Tag für Tag", "Seu consumo dia a dia", "Il tuo consumo giorno per giorno", "每日用量", "日ごとの使用量"]);
add("k_reste2", ["What you have left to use", "Ce qu’il te reste à consommer", "Lo que te queda por consumir", "Was du noch verbrauchen kannst", "O que resta para consumir", "Quello che ti resta da consumare", "剩余可用额度", "残りの使用可能量"]);

// onglets de la carte jour par jour, et leurs légendes : celles de l'appli
add("o0", ["Today", "Aujourd’hui", "Hoy", "Heute", "Hoje", "Oggi", "今天", "今日"]);
add("o1", ["Lines", "Lignes", "Líneas", "Zeilen", "Linhas", "Righe", "线条", "ライン"]);
add("o2", ["Trajectory", "Trajectoire", "Trayectoria", "Verlauf", "Trajetória", "Traiettoria", "轨迹", "推移"]);
add("o3", ["Batteries", "Batteries", "Baterías", "Batterien", "Baterias", "Batterie", "电池", "バッテリー"]);
add("o4", ["Week", "Semaine", "Semana", "Woche", "Semana", "Settimana", "本周", "週"]);
add("a0", ["Tiles: green + = set aside · pink − = exceeded · gold = today · grey = budget of upcoming days.", "Pastilles : + vert = mis de côté · − rose = dépassé · doré = aujourd’hui · gris = budget des jours à venir.", "Casillas: + verde = ahorrado · − rosa = superado · dorado = hoy · gris = presupuesto de los días por venir.", "Kacheln: grünes + = übrig · rosa − = überschritten · gold = heute · grau = Budget der kommenden Tage.", "Pastilhas: + verde = guardado · − rosa = excedido · dourado = hoje · cinza = orçamento dos dias por vir.", "Tessere: + verde = risparmiato · − rosa = superato · oro = oggi · grigio = budget dei giorni a venire.", "色块：绿色 + = 有结余 · 粉色 − = 已超出 · 金色 = 今天 · 灰色 = 未来几天的预算。", "タイル：緑の＋ = 余り · ピンクの − = 超過 · 金色 = 今日 · グレー = これからの日の予算。"]);
add("a1", ["Light marker = day’s budget · pink = beyond the budget · percentages are those of the weekly plan.", "Repère clair = budget du jour · rose = au-delà du budget · les pourcentages sont ceux du forfait de la semaine.", "Marca clara = presupuesto del día · rosa = por encima del presupuesto · los porcentajes son los del plan semanal.", "Helle Marke = Tagesbudget · rosa = über dem Budget · die Prozentwerte beziehen sich auf das Wochenkontingent.", "Marca clara = orçamento do dia · rosa = acima do orçamento · as porcentagens são as do plano da semana.", "Segno chiaro = budget del giorno · rosa = oltre il budget · le percentuali sono quelle del piano settimanale.", "浅色标记 = 当天预算 · 粉色 = 超出预算 · 百分比均为占本周额度的比例。", "明るい印 = 1日の予算 · ピンク = 予算超過 · パーセントは週のプランに対する割合です。"]);
add("a2", ["Gold curve = your cumulative usage · light line = exactly on pace · green = to spare, pink = over · thin line = allowed path.", "Courbe dorée = ta conso cumulée · trait clair = pile au rythme · vert = en avance, rose = en retard · tracé fin = chemin permis.", "Curva dorada = tu consumo acumulado · línea clara = justo al ritmo · verde = con margen, rosa = de más · trazo fino = camino permitido.", "Goldene Kurve = dein kumulierter Verbrauch · helle Linie = genau im Soll · grün = Spielraum, rosa = zu viel · dünne Linie = erlaubter Verlauf.", "Curva dourada = seu consumo acumulado · linha clara = exatamente no ritmo · verde = com folga, rosa = a mais · traço fino = caminho permitido.", "Curva dorata = il tuo consumo cumulato · linea chiara = esattamente in linea · verde = con margine, rosa = in più · tratto sottile = percorso consentito.", "金色曲线 = 你的累计用量 · 浅色线 = 恰好符合节奏 · 绿色 = 有富余，粉色 = 超出 · 细线 = 允许的路径。", "金色の曲線 = 累計使用量 · 明るい線 = ちょうどペース通り · 緑 = 余裕、ピンク = 超過 · 細い線 = 許容される経路。"]);
add("a3", ["Full battery = day’s budget intact · empty = used up · pink = exceeded. Figure: what is left (today) or the day’s difference (past).", "Batterie pleine = budget du jour intact · vide = épuisé · rose = dépassé. Chiffre : ce qu’il reste (aujourd’hui) ou l’écart du jour (passé).", "Batería llena = presupuesto del día intacto · vacía = agotado · rosa = superado. Cifra: lo que queda (hoy) o la diferencia del día (pasado).", "Volle Batterie = Tagesbudget unberührt · leer = aufgebraucht · rosa = überschritten. Zahl: Rest (heute) oder Differenz des Tages (vergangen).", "Bateria cheia = orçamento do dia intacto · vazia = esgotado · rosa = excedido. Número: o que resta (hoje) ou a diferença do dia (passado).", "Batteria piena = budget del giorno intatto · vuota = esaurito · rosa = superato. Cifra: ciò che resta (oggi) o lo scarto del giorno (passato).", "电池满 = 当天预算未动 · 空 = 已用完 · 粉色 = 已超出。数字：今天显示剩余量，过去的日子显示当天的差额。", "バッテリー満タン = 1日の予算が手つかず · 空 = 使い切り · ピンク = 超過。数字：今日は残り、過去の日はその日の差。"]);
add("a4", ["Gold = used · light = today, so far · hatched = allowed today · boxes = share of each upcoming day · marker = where the week stands.", "Doré = consommé · clair = aujourd’hui, déjà · hachuré = permis aujourd’hui · cases = part de chaque jour à venir · repère = où en est la semaine.", "Dorado = consumido · claro = hoy, hasta ahora · rayado = permitido hoy · casillas = parte de cada día por venir · marca = en qué punto va la semana.", "Gold = verbraucht · hell = heute bisher · schraffiert = heute erlaubt · Felder = Anteil jedes kommenden Tages · Marke = Stand der Woche.", "Dourado = consumido · claro = hoje, até agora · hachurado = permitido hoje · caixas = parte de cada dia por vir · marca = em que ponto está a semana.", "Oro = consumato · chiaro = oggi, finora · tratteggiato = consentito oggi · caselle = quota di ogni giorno a venire · segno = a che punto è la settimana.", "金色 = 已用 · 浅色 = 今天已用 · 斜纹 = 今天还可用 · 方块 = 未来每天的份额 · 标记 = 本周进度。", "金色 = 使用済み · 明るい色 = 今日これまで · 斜線 = 今日の許容分 · 枠 = これからの各日の配分 · 印 = 週の進み具合。"]);

// explication de chaque carte
add("e_rythme", ["The card that answers “am I OK?”. It compares the share of your weekly plan you have used with the share of the week already gone. Green “to spare” means you use your plan slower than the week goes by. Pink “over” means faster: slow down a little.", "La carte qui répond à « est-ce que je vais bien ? ». Elle compare la part de ton forfait déjà utilisée à la part de la semaine déjà écoulée. Vert « d’avance » : tu consommes moins vite que la semaine ne passe. Rose « de trop » : tu consommes plus vite, ralentis un peu.", "La tarjeta que responde a «¿voy bien?». Compara la parte de tu plan que ya has usado con la parte de la semana que ya ha pasado. Verde «de margen»: consumes más despacio de lo que pasa la semana. Rosa «de más»: consumes más rápido, frena un poco.", "Die Karte, die „Bin ich im Plan?“ beantwortet. Sie vergleicht den schon verbrauchten Teil deines Wochenkontingents mit dem schon vergangenen Teil der Woche. Grün „Spielraum“: Du verbrauchst langsamer, als die Woche vergeht. Rosa „zu viel“: Du verbrauchst schneller, bremse etwas.", "O cartão que responde a “estou bem?”. Compara a parte do plano que você já usou com a parte da semana que já passou. Verde “de folga”: você consome mais devagar do que a semana passa. Rosa “a mais”: consome mais rápido, desacelere um pouco.", "La scheda che risponde a «sto andando bene?». Confronta la parte del piano già usata con la parte di settimana già trascorsa. Verde «di margine»: consumi più lentamente di come passa la settimana. Rosa «in più»: consumi più in fretta, rallenta un po’.", "回答“我还好吗？”的卡片。它把你已用掉的本周额度比例，和本周已过去的比例做对比。绿色“富余”：你消耗得比时间流逝慢。粉色“超出”：你消耗得更快，可以放慢一点。", "「大丈夫？」に答えるカードです。使ったプランの割合と、経過した週の割合を比べます。緑の「余裕」は、時間より遅いペースで使っているということ。ピンクの「オーバー」は、時間より速いペースなので、少し控えめに。"]);
add("e_reste", ["What is left of your weekly plan, as a percentage. It is simply 100 % minus what you have used.", "Ce qu’il te reste de ton forfait de la semaine, en pourcentage. C’est simplement 100 % moins ce que tu as utilisé.", "Lo que te queda de tu plan semanal, en porcentaje. Es simplemente 100 % menos lo que has usado.", "Was von deinem Wochenkontingent noch übrig ist, in Prozent. Das ist einfach 100 % minus das, was du verbraucht hast.", "O que resta do seu plano da semana, em porcentagem. É simplesmente 100% menos o que você usou.", "Quanto resta del tuo piano settimanale, in percentuale. È semplicemente 100% meno quello che hai usato.", "你本周额度还剩多少，以百分比表示。就是 100% 减去已用的部分。", "今週のプランの残りをパーセントで表示します。100% から使った分を引いた数字です。"]);
add("e_reset", ["The countdown until your plan is back to 100 %, with the real day and hour of your reset. The hour is read again at every update and shown in your own time zone. In the week of a clock change, a line under the card explains it.", "Le compte à rebours avant que ton forfait revienne à 100 %, avec le vrai jour et la vraie heure de ta remise à zéro. L’heure est relue à chaque mise à jour et affichée dans ton fuseau horaire. La semaine d’un changement d’heure, une ligne sous la carte l’explique.", "La cuenta atrás hasta que tu plan vuelva al 100 %, con el día y la hora reales de tu reinicio. La hora se vuelve a leer en cada actualización y se muestra en tu zona horaria. La semana de un cambio de hora, una línea bajo la tarjeta lo explica.", "Der Countdown, bis dein Abo wieder bei 100 % ist, mit dem echten Tag und der echten Uhrzeit deines Zurücksetzens. Die Uhrzeit wird bei jeder Aktualisierung neu gelesen und in deiner Zeitzone angezeigt. In der Woche einer Zeitumstellung erklärt eine Zeile unter der Karte den Unterschied.", "A contagem regressiva até o seu plano voltar a 100%, com o dia e a hora reais da sua reinicialização. O horário é relido a cada atualização e mostrado no seu fuso. Na semana de uma mudança de horário, uma linha sob o cartão explica.", "Il conto alla rovescia prima che il tuo piano torni al 100%, con il giorno e l’ora reali del tuo reset. L’orario viene riletto a ogni aggiornamento e mostrato nel tuo fuso orario. Nella settimana di un cambio d’ora, una riga sotto la scheda lo spiega.", "距离额度恢复至 100% 的倒计时，显示你重置的真实日期和时间。时间在每次更新时重新读取，并按你的时区显示。遇到夏令时/冬令时切换的那一周，卡片下方会有一行说明。", "プランが 100% に戻るまでのカウントダウンです。リセットの実際の曜日と時刻を表示します。時刻は更新のたびに読み直され、あなたのタイムゾーンで表示されます。時刻切り替えのある週は、カードの下に説明が出ます。"]);
add("e_semaine", ["The whole week in one bar. Gold = what you have used. The light marker = how far the week has gone. Gold stops before the marker: you are ahead. Gold goes past it: you are over.", "Toute la semaine en une barre. Doré = ce que tu as utilisé. Le repère clair = où en est la semaine. L’or s’arrête avant le repère : tu es en avance. L’or le dépasse : tu es en retard.", "Toda la semana en una barra. Dorado = lo que has usado. La marca clara = en qué punto va la semana. El dorado se queda antes de la marca: vas con margen. La pasa: vas de más.", "Die ganze Woche in einem Balken. Gold = was du verbraucht hast. Die helle Marke = Stand der Woche. Gold endet vor der Marke: Du hast Spielraum. Gold geht darüber hinaus: Du liegst darüber.", "A semana inteira em uma barra. Dourado = o que você usou. A marca clara = em que ponto está a semana. O dourado para antes da marca: você está com folga. Passa dela: está acima.", "L’intera settimana in una barra. Oro = quanto hai usato. Il segno chiaro = a che punto è la settimana. L’oro si ferma prima del segno: hai margine. Lo supera: sei in più.", "整个星期放在一条横条里。金色 = 你已用掉的部分。浅色标记 = 本周的进度。金色停在标记之前：你有富余。金色越过标记：你超出了。", "一週間を一本のバーにまとめたものです。金色は使った分、明るい印は週の進み具合です。金色が印の手前で止まっていれば余裕あり、印を越えていればオーバーです。"]);
add("e_jours", ["Your usage, day by day. A day lasts 24 hours from your reset time, not midnight to midnight. The same figures can be shown five ways: pick the one that speaks to you, the app remembers it.", "Ta conso, jour par jour. Un jour dure 24 h à partir de l’heure de ta remise à zéro, pas de minuit à minuit. Les mêmes chiffres se montrent de cinq façons : choisis celle qui te parle, l’appli s’en souvient.", "Tu consumo, día a día. Un día dura 24 h desde la hora de tu reinicio, no de medianoche a medianoche. Las mismas cifras se muestran de cinco formas: elige la que prefieras, la app la recuerda.", "Dein Verbrauch, Tag für Tag. Ein Tag dauert 24 Stunden ab der Uhrzeit deines Zurücksetzens, nicht von Mitternacht bis Mitternacht. Dieselben Zahlen gibt es in fünf Darstellungen: Wähle die, die dir gefällt, die App merkt sie sich.", "Seu consumo, dia a dia. Um dia dura 24 h a partir da hora da sua reinicialização, não da meia-noite à meia-noite. Os mesmos números aparecem de cinco jeitos: escolha o que preferir, o app lembra.", "Il tuo consumo, giorno per giorno. Un giorno dura 24 h a partire dall’ora del tuo reset, non da mezzanotte a mezzanotte. Gli stessi numeri si vedono in cinque modi: scegli quello che preferisci, l’app lo ricorda.", "你的每日用量。一天是从你的重置时间开始的 24 小时，而不是从午夜到午夜。同样的数字有五种显示方式：选你喜欢的，应用会记住。", "日ごとの使用量です。1日はリセット時刻からの24時間で、午前0時から午前0時ではありません。同じ数字を5つの見せ方で表示でき、選んだものはアプリが覚えています。"]);
add("e_reste2", ["What you can still use: to finish the current day, then for each following day. It is recalculated by itself at every update. What you did not use is already carried over; what you went over is taken off the following days.", "Ce que tu peux encore consommer : pour finir le jour en cours, puis pour chacun des jours suivants. Tout se recalcule tout seul à chaque mise à jour. Ce que tu n’as pas consommé est déjà reporté ; ce que tu as dépassé est retiré des jours suivants.", "Lo que aún puedes consumir: para terminar el día en curso y luego para cada día siguiente. Se recalcula solo en cada actualización. Lo que no has consumido ya se traslada; lo que te has pasado se resta de los días siguientes.", "Was du noch verbrauchen kannst: bis zum Ende des laufenden Tags, dann für jeden folgenden Tag. Alles wird bei jeder Aktualisierung automatisch neu berechnet. Was du nicht verbraucht hast, ist schon übertragen; was du überzogen hast, wird den folgenden Tagen abgezogen.", "O que você ainda pode consumir: para terminar o dia atual e depois para cada dia seguinte. Tudo é recalculado sozinho a cada atualização. O que você não consumiu já foi repassado; o que excedeu é descontado dos dias seguintes.", "Quello che puoi ancora consumare: per finire il giorno in corso e poi per ciascun giorno successivo. Tutto si ricalcola da solo a ogni aggiornamento. Ciò che non hai consumato è già riportato; ciò che hai superato viene tolto dai giorni successivi.", "你还能用多少：先是完成今天，然后是之后的每一天。每次更新都会自动重新计算。没用完的部分已经顺延；超出的部分会从之后的日子里扣除。", "あとどれだけ使えるかです。今日を終えるまでの分と、その後の各日の分を表示します。更新のたびに自動で再計算され、使わなかった分は繰り越し済み、超過した分は以降の日から差し引かれます。"]);
add("adapt_t", ["Advance and delay, on the day-by-day card too", "L’avance et le retard, aussi sur la carte jour par jour", "El margen y el exceso, también en la tarjeta día a día", "Spielraum und Überschreitung, auch auf der Tageskarte", "Folga e excesso, também no cartão dia a dia", "Margine e superamento, anche nella scheda giorno per giorno", "富余与超出，同样体现在每日卡片上", "余裕とオーバーは、日ごとのカードにも反映されます"]);
add("adapt_d", ["Every day gets a budget: what is left at the start of the day ÷ the days left. A day under budget shows a green + and makes the next budgets bigger. A day over budget shows a pink − and makes the next budgets smaller. That is how the advance or delay of your Pace card is spread over the days, with nothing to calculate yourself.", "Chaque jour reçoit un budget : ce qu’il reste au début du jour ÷ le nombre de jours restants. Un jour sous le budget montre un + vert et agrandit les budgets suivants. Un jour au-dessus montre un − rose et rétrécit les budgets suivants. C’est ainsi que l’avance ou le retard de la carte « Ton rythme » se répartit sur les jours, sans rien calculer toi-même.", "Cada día recibe un presupuesto: lo que queda al empezar el día ÷ los días que quedan. Un día por debajo del presupuesto muestra un + verde y agranda los presupuestos siguientes. Un día por encima muestra un − rosa y los reduce. Así se reparte por los días el margen o el exceso de la tarjeta «Tu ritmo», sin que calcules nada.", "Jeder Tag bekommt ein Budget: was zu Tagesbeginn übrig ist ÷ die verbleibenden Tage. Ein Tag unter dem Budget zeigt ein grünes + und vergrößert die nächsten Budgets. Ein Tag darüber zeigt ein rosa − und verkleinert sie. So verteilen sich Spielraum oder Überschreitung der Karte „Dein Tempo“ auf die Tage, ohne dass du etwas rechnen musst.", "Cada dia recebe um orçamento: o que resta no início do dia ÷ os dias restantes. Um dia abaixo do orçamento mostra um + verde e aumenta os próximos orçamentos. Um dia acima mostra um − rosa e os diminui. É assim que a folga ou o excesso do cartão “Seu ritmo” se distribui pelos dias, sem você calcular nada.", "Ogni giorno riceve un budget: quanto resta a inizio giornata ÷ i giorni rimasti. Un giorno sotto il budget mostra un + verde e ingrandisce i budget successivi. Un giorno sopra mostra un − rosa e li riduce. Così il margine o il superamento della scheda «Il tuo ritmo» si distribuisce sui giorni, senza che tu debba calcolare nulla.", "每一天都有一个预算：当天开始时的剩余额度 ÷ 剩余天数。低于预算的一天显示绿色 +，并让之后的预算变大；高于预算的一天显示粉色 −，并让之后的预算变小。“你的节奏”卡片上的富余或超出，就是这样分摊到每一天，你不用自己计算。", "毎日に予算があります。その日の初めの残り ÷ 残り日数です。予算より少なければ緑の＋が付き、以降の予算が増えます。予算を超えるとピンクの−が付き、以降の予算が減ります。「あなたのペース」の余裕やオーバーは、こうして各日に配分されるので、自分で計算する必要はありません。"]);

// ---------------------------------------------------------------- la visite : captures réelles, une carte = une zone cliquable
const CARTES = [
  { id: "rythme", r: [53, 113, 375, 296] },
  { id: "reste", r: [390, 113, 712, 296] },
  { id: "reset", r: [727, 113, 1049, 296] },
  { id: "semaine", r: [53, 312, 1049, 414] },
  { id: "jours", r: [53, 432, 642, 822] },
  { id: "reste2", r: [657, 432, 1049, 822] }
];
const IMG_W = 1102, IMG_H = 872;
let lang = "en";
const etat = { sc: "app", carte: "rythme", vue: 3 };
const $ = id => document.getElementById(id);
const t = k => (L[lang] && L[lang][k]) || L.en[k] || k;
function el(tag, attrs, ...kids) {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs || {})) { if (k === "class") e.className = v; else e.setAttribute(k, v); }
  kids.forEach(c => e.append(c));
  return e;
}

function dessinerVisite() {
  const racine = $("tour"); if (!racine) return;
  racine.textContent = "";
  const scTabs = el("div", { class: "tabs", role: "group" });
  [["app", "sc_avance"], ["retard", "sc_retard"]].forEach(([id, k]) => {
    const b = el("button", { class: "tab", type: "button", "aria-pressed": String(etat.sc === id) }, t(k));
    b.onclick = () => { etat.sc = id; dessinerVisite(); };
    scTabs.append(b);
  });
  const cadre = el("div", { class: "frame" });
  const img = el("img", { src: `img/${etat.sc}_${lang}_${etat.vue}.png`, width: IMG_W, height: IMG_H, alt: `${t("sc_" + (etat.sc === "app" ? "avance" : "retard"))} — ${t("k_" + etat.carte)}` });
  cadre.append(img);
  CARTES.forEach(c => {
    const [x0, y0, x1, y1] = c.r;
    const b = el("button", { class: "hot", type: "button", "aria-pressed": String(etat.carte === c.id), "aria-label": t("k_" + c.id),
      style: `left:${x0 / IMG_W * 100}%;top:${y0 / IMG_H * 100}%;width:${(x1 - x0) / IMG_W * 100}%;height:${(y1 - y0) / IMG_H * 100}%` });
    b.onclick = () => { etat.carte = c.id; dessinerVisite(); };
    cadre.append(b);
  });
  const vueTabs = el("div", { class: "tabs", role: "group", style: "margin-top:14px" });
  for (let i = 0; i < 5; i++) {
    const b = el("button", { class: "tab", type: "button", "aria-pressed": String(etat.vue === i) }, t("o" + i));
    b.onclick = () => { etat.vue = i; etat.carte = "jours"; dessinerVisite(); };
    vueTabs.append(b);
  }
  const gauche = el("div", {}, scTabs, cadre, vueTabs, el("p", { class: "hint" }, t("tour_hint") + " " + t("shot_note")));
  const panneau = el("div", { class: "panel", "aria-live": "polite" }, el("h3", {}, t("k_" + etat.carte)), el("p", {}, t("e_" + etat.carte)));
  if (etat.carte === "jours") {
    panneau.append(el("p", { class: "legend" }, t("o" + etat.vue) + " — " + t("a" + etat.vue)));
    panneau.append(el("p", { class: "adapt" }, el("b", {}, t("adapt_t") + ". "), t("adapt_d")));
  }
  racine.append(gauche, panneau);
}

// ---------------------------------------------------------------- langue
function choisirLangue() {
  let q = null; try { q = new URLSearchParams(location.search).get("lang"); } catch (e) {}
  if (q && L[q]) return q;
  try { const s = localStorage.getItem("ct-lang"); if (s && L[s]) return s; } catch (e) {}
  const n = (navigator.language || "en").toLowerCase();
  for (const k of ORDER) if (n.startsWith(k)) return k;
  return "en";
}
let auChangement = [];
function appliquer(l) {
  lang = l;
  document.documentElement.lang = HTML_LANG[l];
  document.querySelectorAll("[data-i]").forEach(e => { const v = L[l][e.dataset.i]; if (typeof v === "string") e.textContent = v; });
  const sel = $("lang"); if (sel) sel.value = l;
  try { localStorage.setItem("ct-lang", l); } catch (e) {}
  dessinerVisite();
  auChangement.forEach(f => f());
}
function demarrer(hook) {
  if (hook) auChangement.push(hook);
  const sel = $("lang"); if (sel) sel.addEventListener("change", e => appliquer(e.target.value));
  appliquer(choisirLangue());
}
