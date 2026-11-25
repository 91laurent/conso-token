CONSO TOKEN — one-page guide

Conso Token is free: free to use, copy, modify and share (MIT licence, see LICENSE.txt).

WHAT IT IS FOR
A small window that shows where you stand on your Claude plan: how much you have used, what is
left day by day, and whether you are ahead or behind. It predicts nothing, connects to no website
and asks for no password.

WHAT YOU NEED
- Windows 10 or 11. Nothing else to install.
- The Claude desktop app (where you use Claude Code) with a paid Claude plan whose usage figures
  the app provides: some plans do not provide them. Conso Token has been tried with one plan only.

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
   Otherwise it says "No figures received yet": tell the person who gave you the app.

THE CONVERSATIONS TAB (SINCE 1.3.0), AND WHAT CAME SINCE
Version 1.3.8. New since 1.3.4: groups 3 to 8 written by hand (see below); the send log moved to
the data folder; plainer words on some screens, in the eight languages.
Next to "Week", the "Conversations" tab shows each Claude Code conversation of the last 24 hours:
how full it is (its context; approximate, ≈) and how much is left before the hand-off, the moment to
carry on in a fresh conversation. On a bar, the cream mark is the hand-off and the dotted mark is
the warning; "summarized" means Claude summarized that conversation to shorten it (hand-off now). A conversation
idle for more than 30 minutes is folded under "+ n more".
- At the start: one group, warning at 70%, hand-off at 80%. The SETTINGS button opens a window:
  split into two groups (you type their names; a click on the tag of a line then moves the whole
  folder to the other group), two threshold sliders with a test bar, "Conversation capacity, by
  model" (Auto, 200,000 or 1,000,000), the blinking dot, the animated figures. Everything applies right
  away and is kept in conversations.txt, in the data folder. "Reset" restores the starting values (it asks for a second click to confirm).
- The first time you are on the tab and there are conversations to propose, a window asks whether you want to group your conversations (it opens by itself only once): you tick the folders of the first group and name the groups yourself (the only thing you type is the names). The "Choose groups…" button (top right of the Conversations tab, and in SETTINGS) reopens it at any time, as often as you like: back to one group, rename the groups, untick everything. Nothing is lost: the choice is kept in conversations.txt.
- MORE THAN TWO GROUPS (since 1.3.5), written by hand: groups 3 to 8, each with its own levels.
  First tick "Split into two groups" in SETTINGS (these groups only show when it is ticked). Close
  the Conso Token window (while it is open, it rewrites the file from its memory, and your changes
  are lost). Open conversations.txt, in the data folder, with Notepad, add lines at the end and
  save it as UTF-8. The group number ends each word, in lower case; for example:
      nom3=Reviews
      attention3=40
      passation3=55
      titre3=Review
  nom3= is the name shown (24 characters at most; without it: "Group 3"). attention3= and
  passation3= are the warning and the hand-off (10 to 95; 70 and 80 if you write nothing).
  titre3= is the beginning of a title; groupe3= is the name of a work folder, spelled exactly. Write
  one line per title beginning or per folder. A conversation is placed by its title first, then by
  its folder, and only then in the two groups of the window. A title beginning counts as whole
  words, upper or lower case alike: a space must follow it, or the title must end there ("Review"
  takes "Review of the plan", not "Review: plan" nor "Reviews"). The tag of a line in such a group
  cannot be clicked. SETTINGS does not show these groups; the program keeps your lines when it
  rewrites the file. "Reset" erases them; "Keep one group" hides them without erasing them. What
  you write there (group names, folder names, title beginnings) stays in that file on your PC.
- A blinking dot on the "Conversations" button, also visible on the Week screen: pink at hand-off,
  amber at warning. No sound, no Windows notification.
- The three cards of the Week screen roll their figures when the window opens and at each update.
  To stop it: the box in SETTINGS, or the Windows setting "Animation effects".
- A ✕ at the right of each row hides the conversation: the row disappears from the list, from "+ n more" and from the blinking dot, even if a new message arrives (handy for an archived or test conversation). One click is enough. SETTINGS shows "n conversations hidden" with a "Show again" button (only when there are some); "Reset" does not touch them. When you hide a conversation with ✕, the app writes that conversation's session identifier (a plain code, neither its title nor its contents) in its data folder (conversations.txt) so that it stays hidden; it is forgotten once the conversation's last message is more than 24 h old, or as soon as you click "Show again".
- Only counters are kept from the Claude Code files of this PC (date, model, token counts, folder
  name), plus the title of each conversation as the Claude app shows it (to name it in the list; read in the
  files already read, kept in memory while it is displayed, never written or sent); the text of a message is never kept, written or sent. Scheduled runs and sub-agents are left out. How much a conversation
  can hold is written in no file: the percentage stays approximate (≈); if it looks off, set the
  right capacity by model in SETTINGS. If Claude changes its file format, the list says so instead of
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
- The task shows under the title "Conso Token feed" (its identifier is conso-token-feed), in the
  "Scheduled" section of the left bar, or under "Routines" in the "Code" tab, depending on your
  version of the Claude app (the app may show these names in another language). It uses a little of
  your plan, like any task: it is included in the figures shown. If you delete it, the figures stop
  updating. Each run archives itself at the end: you may see "Conso Token feed" sessions in the
  archives, this is normal.
- Data is stored in C:\Users\<your name>\AppData\Local\ConsoToken.
- You can change the language with the button at the top right of the window.
- Licences (fonts and Slint) are in the "licences" folder.
- Updating from an earlier version: close the window, replace the WHOLE folder, then give Claude the
  sentence of step 2 again. Your data folder is kept.

HOW TO REMOVE IT
1. Close the Conso Token window.
2. Delete the task "Conso Token feed" (conso-token-feed) in the Claude desktop app ("Scheduled", or
   "Routines" in the "Code" tab). Deleting the folder alone is not enough: the task would go on
   opening a short session every 15 minutes, which would send nothing and still use a little of
   your plan.
3. Only if a Claude Code status line that runs Conso Token was added (the setup of step 2 never
   adds one): it is in settings.json, in C:\Users\<your name>\.claude, in the "statusLine" part that
   contains "conso_token.exe --pont". Make a copy of the file first. If nothing follows "--pont",
   remove that "statusLine" part. If it contains "--suite" followed by another command, that is the
   status line you had before: keep "statusLine", and leave in its "command" only the command that
   follows "--suite". Touch nothing else.
4. Delete the Conso Token folder.
5. Optional: to erase your settings and the history of your figures too, delete the data folder
   C:\Users\<your name>\AppData\Local\ConsoToken.
Claude works the same without Conso Token.

IF NOTHING ARRIVES
Wait 15 to 20 minutes after the setup, with the Claude desktop app open. Still nothing? In File
Explorer, click the address bar, type %USERPROFILE%\AppData\Local\ConsoToken and press Enter, then
open dernier_envoi.txt with Notepad (full path:
%USERPROFILE%\AppData\Local\ConsoToken\dernier_envoi.txt). It is no longer in the "outils" folder.
A send that worked has "rc=0" on that line. Tell the person who gave you the app what it contains
(or that it does not exist), with a screenshot of the window. If nothing ever comes, you can remove
the app as explained above.
