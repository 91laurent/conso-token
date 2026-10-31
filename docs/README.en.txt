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
