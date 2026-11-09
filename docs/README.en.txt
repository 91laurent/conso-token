CONSO TOKEN — one-page guide

Conso Token is free: free to use, copy, modify and share (MIT licence, see LICENSE.txt).

WHAT IT IS FOR
A small window that shows where you stand on your Claude plan: how much you have used, what is
left day by day, and whether you are ahead or behind. It predicts nothing, connects to no website
and asks for no password.

WHAT YOU NEED
- Windows 10 or 11. Nothing else to install.
- The Claude desktop app (where you use Claude Code) with a Pro or Max plan: the plan figures do
  not exist for other plans.

SETUP (10 minutes, once)
1. Put the "Conso Token" folder you received on a drive of your computer (for example H:). If it
   came as a zip file: right-click the zip, "Extract all". Keep the WHOLE folder: conso_token.exe,
   the "outils" folder, the "licences" folder and the text files. Avoid a USB stick or a network
   drive. Avoid an apostrophe in the folder name.
2. In the Claude desktop app, open a new Claude Code session and write this sentence, replacing
   the path with your folder's path:
       Read the file PASTE_INTO_CLAUDE.txt in the folder "H:\Conso Token" and do
       exactly what it says.
   Claude creates a task that sends your figures to the window every 15 minutes, tries it once in
   front of you and tells you whether it worked. Answer "yes" or "always allow" when the app asks
   for permission for PowerShell: this is normal, it is needed to send the figures.
3. Double-click conso_token.exe. If Windows shows "Windows protected your PC" (unknown
   publisher), click "More info" then "Run anyway". This is not a virus: Conso Token is a small
   home-made program that is not signed by a paid publisher, so Windows flags it the first time.
   It uses no network function. If step 2 worked, the window already shows your figures.
   Otherwise it says "No figures received": tell the person who gave you the app.

NEW IN 1.3.0: THE CONVERSATIONS TAB
Next to "Week", the "Conversations" tab shows each Claude Code conversation of the last 24 hours:
how full its context is (approximate, ≈) and how much is left before the hand-off, the moment to
carry on in a fresh conversation. On a bar, the cream mark is the hand-off and the dotted mark is
the warning; "summarized" means Claude compacted that conversation (hand-off now). A conversation
idle for more than 30 minutes is folded under "+ n more".
- At the start: one group, warning at 70%, hand-off at 80%. The SETTINGS button opens a window:
  split into two groups (you type their names; a click on the tag of a line then moves the whole
  folder to the other group), two threshold sliders with a test bar, the context window by model
  (Auto, 200,000 or 1,000,000), the blinking dot, the animated figures. Everything applies right
  away and is kept in conversations.txt, in the data folder. "Reset" restores the starting values (it asks for a second click to confirm).
- The first time you open the tab, a window asks whether you want to group your conversations: you tick the folders of the first group and name the groups yourself (the only thing you type is the names). The "Choose groups…" button (top right of the Conversations tab, and in SETTINGS) reopens it at any time, as often as you like: back to one group, rename the groups, untick everything. Nothing is lost: the choice is kept in conversations.txt.
- A blinking dot on the "Conversations" button, also visible on the Week screen: pink at hand-off,
  amber at warning. No sound, no Windows notification.
- The three cards of the Week screen roll their figures when the window opens and at each update.
  To stop it: the box in SETTINGS, or the Windows setting "Animation effects".
- A ✕ at the right of each row hides the conversation: the row disappears from the list, from "+ n more" and from the blinking dot, even if a new message arrives (handy for an archived or test conversation). One click is enough. SETTINGS shows "n conversations hidden" with a "Show again" button (only when there are some); "Reset" does not touch them. When you hide a conversation with ✕, the app writes that conversation's session identifier (a plain code, neither its title nor its contents) in its data folder (conversations.txt) so that it stays hidden; it is forgotten once the conversation's last message is more than 24 h old, or as soon as you click "Show again".
- Only counters are kept from the Claude Code files of this PC (date, model, token counts, folder
  name), plus the title of each conversation as the Claude app shows it (to name it in the list; read in the
  files already read, kept in memory while it is displayed, never written or sent); the text of a message is never kept, written or sent. Scheduled runs and sub-agents are left out. The size of the
  window is written in no file: the percentage stays approximate (≈); if it looks off, set the
  right window by model in SETTINGS. If Claude changes its file format, the list says so instead of
  showing wrong figures.
  The list is based on Claude's last message, not on the state of the window: an archived or closed conversation stays listed as long as its last message is less than 24 hours old (hide it with ✕); a conversation that is open but has had no message for more than 24 hours does not appear (it comes back as soon as Claude answers in it).
  Limits: if Claude changes the type of its lines, or if its answers can no longer be found, the list
  stays empty with NO warning; if the line that carries the title changes, names go back to
  "folder · 4 characters", with no error. The title is looked for in the last 64 KB of the file: it is kept while the program runs, but after a restart a very talkative conversation with no recent title line goes back to "folder · 4 characters". The model list in SETTINGS has a fixed order (Opus, Sonnet,
  Haiku, then the others), but it holds the five most recent models: with six or more seen in 24
  hours, a sixth one answering changes it.
  From the first reading, the program also creates contextes_vus.txt in the data folder (the largest
  context seen for each model: numbers only).
GOOD TO KNOW
- The window reads the time your week resets by itself and shows your own time zone and clock
  changes (summer/winter time). In the week of a clock change, a line under the countdown card
  explains it (for example "One hour earlier: clocks go back to standard time on Sunday 10/25."):
  this is normal, nothing to adjust.
- A "day" in the window = 24 h from that reset, as stated above the chart.
- The Claude desktop app must be open (not necessarily in front) for the figures to update. The
  Conso Token window itself can be closed: reopen it with a double-click on conso_token.exe.
- If you move the folder later, the task will no longer find the program: give Claude the
  sentence from step 2 again with the new path.
- The "conso-token-feed" task (Scheduled section of the left bar) itself uses a little of your
  plan, like any task: it is included in the figures shown. If you delete it, the figures stop
  updating. Each run archives itself at the end: you may see "Conso Token feed" sessions in the
  archives, this is normal.
- To stop everything: delete that task, then the Conso Token folder.
- Data is stored in C:\Users\<your name>\AppData\Local\ConsoToken.
- You can change the language with the button at the top right of the window.
- Licences (fonts and Slint) are in the "licences" folder.

IF NOTHING ARRIVES
Open "outils\dernier_envoi.txt" (in the Conso Token folder) with Notepad and tell the person who
gave you the app what it contains (or that it does not exist), with a screenshot of the window.
