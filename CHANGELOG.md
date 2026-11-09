# Changelog

All notable changes to Conso Token are listed here, newest first. Versions follow the number shown on the **About** screen of the program. Dates are 2026.

## 1.3.4 — 9 November — Hide a conversation, regroup your conversations at any time, follow-ups of an independent review of 1.3.0, and the title of each conversation

A correction release with two new controls. Five things are added to the screens: a ✕ on each row of the Conversations tab that hides the conversation, a “Show again” button in SETTINGS, a “Choose groups…” button on the Conversations tab, the second click on “Reset”, and the title of each conversation in the list. (Test builds numbered 1.3.1, 1.3.2 and 1.3.3 were made on 8–9 November; none was ever put into service nor published, and their changes are part of this entry.)

**THE PROMISE CHANGES in this release.** Until now the program wrote nothing about a particular conversation (only, in `conversations.txt`, the names of the folders that you ticked for the first group). Now, when you hide a conversation with ✕, the program writes that conversation's session identifier (a plain code, neither its title nor its contents) in its own data folder (`conversations.txt`, one line `masquee=<identifier>` per hidden conversation), so that it stays hidden; it is forgotten once the conversation's last message is more than 24 h old, or as soon as you click “Show again”. The sentences about the title (kept in memory while it is displayed, never written or sent) and about the text of a message (never kept, written or sent) are unchanged. Nothing is sent over the Internet.

### Added
- **A ✕ on each row of the Conversations tab, to hide a conversation** (at the right end of the row, in the unfolded section and in the folded “+ n more” section; a small “Hide” label appears under the pointer). The row disappears from the list, from “+ n more” and from the blinking dot, **even if a new message arrives** in that conversation. One click is enough. It was asked for because the program cannot see that a conversation was archived in the Claude app: an archived conversation that had been summarized turned the dot pink until 30 minutes after its last message. The conversation is recognised by its session identifier only. **SETTINGS shows “n conversations hidden” with a “Show again” button** (only when there are some; n counts the conversations of the last 24 h); the button brings all of them back. “Reset” does not touch them. The identifier of a hidden conversation is forgotten when its last message is more than 24 h old, so that the file does not grow; nothing is forgotten while no conversation at all has been read, nor if the file format is not understood.
- **A permanent “Choose groups…” button on the Conversations tab** (top right of the page, also when there is a single group, “All conversations”). It reopens the window of the first opening as often as you like, to tick the folders of the first group, give the groups their names, go back to ONE group, or untick everything. Nothing is lost: the choice is kept in `conversations.txt`; going back to one group keeps the list of folders and the names, which come back the next time you split. The click on the tag of a line still moves the whole folder to the other group (with two groups). The window of the first opening still appears once on its own; its text now says where the button is.
- **The “Choose groups…” button of SETTINGS is now always there.** Before, it only appeared when the program had folders to propose. If the window is opened before the first reading of the conversations is done, the folders already saved in the first group are offered ticked, so that validating cannot erase them.

### Fixed
- **No path of the build computer in the program.** The 1.3.0 program contained three paths of the folder it was built in (inside the code that Slint generates). The build now replaces that folder's prefix. The finished program was searched, in ASCII and in UTF-16, for the user name, drive letters, the names of the work folders and of the build folder: none is left. (This holds only if the build folder is under one of the replaced prefixes, which are set in the author's private `.cargo/config.toml`, not in the one of this repository; the finished program has to be searched again after every build.)
- **The model list in SETTINGS no longer moves under the click.** A click was turned into a model by its row number in a list recomputed after it was displayed, so a list that had moved could change the window of the wrong model. Each row now carries the identifier of its model, and the order is fixed: Opus, Sonnet, Haiku, then the others; within a family, the newest version first.
- **“Reset” asks for a second click** (within 5 seconds) instead of erasing at once the list of folders, the group names and the thresholds.
- **Three new conversations no longer look like a changed file format.** A conversation where Claude has not answered yet is no longer counted by the check that hides the list and the alert when no file can be read.
- **The footer of the SETTINGS window** no longer says “nothing to confirm” next to the “Click to confirm” button of “Reset”: it says “Changes apply right away.”
- **A line that carries a `customTitle` of another type can no longer hide a conversation.** The title field was read on every line of the conversation files: a line of any other type (an answer of Claude, for instance) carrying a `customTitle` that is not text was rejected as a whole and its conversation disappeared from the list without a warning. The field is now read only on the lines of type `custom-title`; on such a line, a title that is not text is ignored (the name goes back to “folder · 4 characters”) and the conversation stays.
- **Titles and folder names are cleaned for display.** A writing-direction control (U+202E and its relatives) in a title also reversed the folder name next to it, and a line break made the row overflow. Control characters and line breaks now become a space, and the writing-direction and zero-width characters are removed, in the title, in the folder name, in the model name and in the folder list of the group window. The folder's real name is unchanged (it is the key of the group choice).

### Changed
- **Conversations are named by their title.** The list showed “project-a · c95f” for three conversations of the same folder. It now shows the title of each conversation as the Claude app shows it (whether you typed it or Claude generated it), then its folder (“Travel notes · project-a”). The title is read from the line of the conversation file that carries it (`custom-title`), in the files that are already read for the counters; it is kept in memory while it is displayed and never written or sent. A conversation without a title keeps “folder · 4 characters”; two conversations with the same title in the same folder also get the 4 characters.
- **The sentence of the promise is now exact.** It used to say that the text of a message is “never read”; the program does run every line of the conversation files through a JSON parser (and a byte pre-filter) and drops what it does not need. The About screen and the eight guides now say that the program keeps only the counters and the title, that the title is kept in memory while it is displayed and never written or sent, and that the text of a message is never kept, written or sent. (The sentence is longer, so the lower part of the About screen moved down a little.) In this release the About screen and the eight guides also carry the new sentence about the identifier of the hidden conversations (see above).
- **The About screen** lays its four blocks (text, Slint line, logo, fonts) one after the other according to the real height of the text, and its text writes × where the guides write ✕. The new sentence makes the text 5 to 8 lines long at the minimum window width (7 in German, 8 in Japanese): with fixed positions the last lines covered the Slint line, and the font has no ✕ (it showed as a blank, or as an empty box in Chinese and Japanese). Found on screen before delivery.
- The eight guides say that `contextes_vus.txt` is created in the data folder from the first reading, that “Reset” asks for a confirmation, where to find the “Choose groups…” button, what the ✕ and “Show again” do, and what happens when Claude changes its file format (see the limits below).
- The text count of 1.3.0 is corrected below (39 new texts, not 32). Four texts were added in this release (the “Hide” label, the two forms of “n conversations hidden”, “Show again”: 152 texts in all); the last sentence of the first-opening window and the About sentence were reworded in the eight languages.
- The sources: personal names, work-folder names and workstation paths are gone from comments, notes and test data; the feeder scripts of the sources (`envoyer_pont.cmd`, `envoyer_pont_cache.vbs`) are the general ones of the package (they accept `-` `-` for a plan without a 5-hour window).

### Tests added, nothing fixed
- **Scheduled runs.** The program already treated only the origin `scheduled` as a scheduled run. The tests now also use the origin that real desktop-app sessions carry (`sdk`), and a test fails if the rule ever becomes “everything but `human` is scheduled”: a guard against a future regression, not a fix.
- The test of the model list now puts an OLD Opus as the sixth model: it fails if the list were sorted first and cut to five afterwards (the previous test would have passed both ways).
- New in 1.3.4: tests of the group choice (two groups, back to one group without losing the list, untick everything, the saved file read back, the folders proposed with and without a first reading) and of the click on a tag in both directions with its tags.
- New in 1.3.4: tests of the hiding (hidden in the unfolded and in the folded section, the “+ n more” count, the dot switched off, a new message does not unhide, the identifier forgotten after 24 h and nothing forgotten without a reading, “Show again”, “Reset” leaves the hidden ones, an identifier that cannot be written on one line, a damaged file, only the identifier in the file) and one test on fake conversation files from the files to the screen model.

### Tested
- 163 tests in 13 suites for the whole project, 0 failures, 0 warnings. Two of the 13 suites contain no test (`main.rs` and the Doc-tests): the 163 tests are in 11 suites. The 3 tests of the bridge (`--pont`) are among them and are replayed in release mode, the way the program is delivered.
- Sabotages: 42 deliberate breakages of the code (the 25 of 1.3.3 and 17 new ones, one per rule of the hiding), each of which makes at least one test fail; the copy was restored byte for byte afterwards. The earlier version of the model-order test did not notice one of them (sorting first and cutting to five afterwards); the new one does.
- By eye, in the window (data folder thrown away afterwards, never the real one; mouse messages posted to the window, not a real mouse): the new button, the window, one group and two groups, the click on a tag, the ✕ on the rows, “n conversations hidden” (one: in the eight languages; two: in French and German) and “Show again” in SETTINGS, the minimum window width (1100 px) in the eight languages, and the About screen of the final program in the eight languages. The “Hide” label under the pointer was seen only twice, by chance (German, Chinese): a pointer move posted alone does not show it, so it cannot be tested without a real mouse. No automatic test covers the Slint screens (see the limits).

### Known limits (not corrected, decided on 9 November)
- The “summarized” mark keeps the dot pink as long as the conversation is active, even after its context has dropped (on purpose: a compacted conversation has lost its details).
- A date or a counter that cannot be read in Claude's last message: that line is dropped and the previous message is shown, without a warning. A clock that runs ahead keeps a conversation “active” until the real time catches up.
- Folders are compared with their case: “Projet_A” and “projet_a” count as two folders.
- `conversations.txt` saved in ANSI (not UTF-8) is ignored, then overwritten at the next change: ALL the settings go back to their defaults, the hidden conversations included (they reappear). The keys are case-sensitive (`MASQUEE=` is ignored). A group name that you type or paste can contain a writing-direction control character (U+202E); only the titles, the folder names and the model names are cleaned for display.
- A last message of Claude that is more than 4 MB before the end of its file makes the conversation invisible.
- **A changed file format can give an empty list WITHOUT a warning.** The “file format not understood” warning only fires when Claude has answered in files whose counters cannot be read. If Claude Code renames the field `type` of its lines, or makes its answers impossible to find, no file counts as a conversation any more and the list is simply empty, with no alarm.
- **The title depends on the same format, and on the last 64 KB of the file.** It is looked for in the last 64 KB of the file, only on a line whose type is `custom-title`. Once seen it is kept while the program runs, but it is NOT kept across a restart: if the title line is no longer in the last 64 KB when the program starts (a very talkative conversation that has had no new title line), the name goes back to “folder · 4 characters”, with no error. Same fallback if Claude Code changes that line or if a conversation never got a title. A title that the person empties does not replace the previous one. Checked on the 79 conversation files of the last 24 hours of the author's PC (the desktop app): the 8 titles the app showed were found word for word. The independent review of 1.3.2 counted 85 files whose last title line was at most 32 KB from the end (about half of the 64 KB that are read). Claude Code used alone in a terminal was not checked.
- **The list follows Claude's last message, not the state of the window.** An archived or closed conversation stays listed as long as its last message is less than 24 hours old (active up to 30 minutes, then folded; hide it with the ✕ of its row); a conversation that is open but has had no message for more than 24 hours does not appear, and comes back as soon as Claude answers in it. The program cannot see in the files whether a conversation is open or archived.
- **Hiding a conversation (1.3.4).** (a) A conversation is recognised by its session identifier: if Claude Code resumes it under a NEW identifier (a resume that creates a new file), it is no longer hidden. (b) The identifier is forgotten as soon as the session is no longer in the reading, that is when its last message is more than 24 h old (no margin): if it answers afterwards, it comes back. (c) Nothing is forgotten while no conversation is read: after a long stop, old identifiers stay in `conversations.txt` until the first reading that shows a conversation. (d) “n conversations hidden” counts only those of the last 24 h; “Show again” empties the whole list (no way to bring back a single one). (e) The identifier is not compared with the folder: two files that carry the same identifier are hidden together. (f) The ✕ asks for no confirmation (one click; undone by “Show again”).
- **“Reset” and the 5 seconds.** The first click arms the second one for 5 seconds; other clicks (changing a window, a threshold) do not disarm it, and everything set in between is reset too if the second click comes in time.
- **The model list in SETTINGS has a fixed order, not a fixed content.** It holds the five most recent models seen in the last 24 hours: with six or more, the content changes when a sixth one answers.
- **No automatic test covers the Slint screens.** The second click of “Reset”, the new “Choose groups…” button and its window, the ✕ of a row (click, label) and the “n conversations hidden” line with “Show again”, the click on a tag, the order of the lines on screen, how a title is drawn, a row that overflows: all of that was checked by eye, in the window, with mouse messages sent to it (not with a real pointer: hover and double clicks are not covered). The logic behind them (groups, folders proposed, titles, cleaning) is covered by tests.
- **The build is not reproducible to the byte.** From one build folder to another the replaced paths differ; in the same folder, two builds differ by 24 bytes (the link time stamp and the GUID of the `.pdb`). The proof that a program comes from the sources is the list of sha1 taken at the freeze (sources, program, package), not a rebuild. The cleanliness of the program also depends on the prefixes replaced in the author's private `.cargo/config.toml` (the one of this repository carries no path remapping): a build folder outside them leaves its path in the program, and nothing stops it. Three strings `cible\release\build\…` remain by construction (no name of a person or of a computer).
- **A hidden conversation can be forgotten on a partial reading.** Its identifier is forgotten as soon as a reading shows other conversations but not it: normally because its last message is more than 24 hours old, but also, rarely, if a project sub-folder cannot be read for a moment (the program ignores that error). The conversation then comes back in the list; hide it again with ✕. When nothing at all can be read, nothing is forgotten.
- **The ✕ of a conversation whose identifier is refused does nothing and says nothing.** An identifier of more than 128 characters, or with a control character, is not stored (so it cannot write another setting into `conversations.txt`); Claude's own identifiers have 36 characters.
- **A hand-made `conversations.txt` with thousands of `masquee=` lines slows the window.** The duplicates are removed by a comparison that grows with the square of the list: 10,000 lines took about 8 seconds at each reading and each change of a setting (debug build), on the interface thread. The program cannot write such a file, because it forgets identifiers at every reading.
- The 40 new translations since 1.2.1, and the reworded sentences of the About screen, of the first-opening window and of the guides, have not been reviewed by native speakers.

## 1.3.0 — 8 November — Conversations

A new tab that shows how full each Claude Code conversation is, and figures that roll.

### Added
- **A “Conversations” tab** next to “Week”. For each Claude Code conversation of the last 24 hours: the context at the last exchange as a percentage of the model's window (approximate, ≈), the tokens, and the points left before the hand-off. On the bar, a cream mark is the hand-off and a dotted mark the warning; “summarized” marks a conversation that Claude has compacted (hand-off now). A conversation idle for more than 30 minutes is folded. Only counters are read (date, model, token counts, work folder), never the text of a message; scheduled runs and sub-agents are left out.
- **A SETTINGS button and window**: one group by default (warning 70%, hand-off 80%) or two groups with names you type, a pair of thresholds per group with a test bar, the context window by model (Auto, 200,000 or 1,000,000), the blinking dot, the animated figures, Reset. Applied right away; kept in `conversations.txt` and `contextes_vus.txt` in the data folder.
- **A first-opening window** on the Conversations tab: the program cannot know which conversation plays which role, so the person decides. The window asks whether to group the conversations; the person ticks the folders of the first group (the list is built from the conversations that are shown) and types the names of the two groups. It appears once (`accueil=1` in `conversations.txt`), can be closed or refused (“Keep one group”), and the “Choose groups…” button in SETTINGS reopens it. Nothing is guessed, nothing is read from the content of a conversation.
- **A blinking dot** on the “Conversations” button, also visible on the Week screen: pink at hand-off, amber at warning. No sound, no notification.
- **Rolling figures** on the three cards of the Week screen (odometer, progressive count, flip panels), when the window opens and at each update. Off with the SETTINGS box or with the Windows “Animation effects” setting.
- 39 new texts in the eight languages (counted in `noyau/src/textes.rs`: 108 texts in 1.2.1, 147 in 1.3.0; the first version of this entry said 32).

### Changed
- The window is at least 1100 pixels wide: the new header no longer fits in 1000.

### Known limits
- The size of the context window is in no file, so the percentage is approximate. “Auto” takes 1,000,000 if the model has ever gone past 200,000 tokens, otherwise 200,000; a conversation limited to 200,000 on a model that normally has 1,000,000 shows a percentage that is too low until the setting is changed.
- “Open” cannot be seen in the files: a conversation counts as active when its last message is less than 30 minutes old.
- The “summarized” mark stays as long as the conversation exists.
- The 39 new translations have not been reviewed by native speakers.

## 1.2.1 — 31 October

A packaging and documentation release. The program itself is unchanged apart from its version number.

### Fixed
- **The version number now means something.** After 1.2.0 the package files (feeder scripts, setup text, one-page guides) were edited several times while the number stayed at 1.2.0, so two different packages carried the same version. The number is now the same in the About screen, `README.txt`, `VERSION.txt` and `CHANGELOG.txt`, and `VERSION.txt` gives the checksum of the program.
- **Plans without a 5-hour window stopped receiving figures.** Some plans (for example Ultra, from 31 October) no longer have a “5-hour limit” window. The scheduled task required it and silently stopped sending. Only the weekly window is required now; the 5-hour pair is optional.

### Added
- `feeder/envoyer_pont.cmd` accepts `-` `-` in place of the 5-hour percentage and reset time, and nothing is invented in its place. The setup text (`docs/PASTE_INTO_CLAUDE.txt`) says which window is required and which is optional.
- A presentation page and a reading guide in all eight languages (`docs/site/`), with real screenshots of every card of the window, a “where do I find the version number” entry, and a comparison of a week ahead of the pace with a week over it.
- The app package carries its own `LICENSE.txt` (MIT), and the README, the eight one-page guides and the web pages say plainly that Conso Token is free and under the MIT licence.
- The full changelog (this file) is shipped in the package as `CHANGELOG.txt`, so the package and the repository tell the same story.
- First public source release: README with a screenshot, MIT licence, third-party notices, contribution guide, issue templates, tests, and the tools the scheduled task calls.

### Changed
- Personal names and workstation paths removed from the sources, tests and tool headers. One test file was renamed to describe its scenario (a weekly reset on Saturday at midnight) instead of a person.

## 1.2.0 — 25 October — International version

One program, eight languages: English, French, Spanish, German, Portuguese (Brazil), Italian, Chinese (simplified) and Japanese.

### Added
- The language follows Windows, falls back to English, and a button at the top right changes it. The choice is kept in `langue.txt` in the data folder.
- The time zone is read from Windows (fallback Paris). Before, every hour was written in Paris time.
- Every text of the program lives in one table, `noyau/src/textes.rs`, one line per text and eight cells. The tests check that no cell is empty and that the placeholders are the same in the eight languages.
- One-page guides in the eight languages, and a setup text that Claude follows in the language of the person who is installing.
- The clock-change line is written in the eight languages. Direction (earlier or later), season (winter or summer time, deduced from the direction, so it is right in the southern hemisphere too) and day are computed, not written by hand.

### Tested
- 90 tests in 12 suites, 0 failures, 0 warnings. Cases: Paris (25 October, end of March, an ordinary week, the eight languages), Montréal (1 November), Sydney (October “later”, April “earlier”) and a weekly reset on Saturday at midnight.

### Known limits
- The translations have not been reviewed by native speakers, the wording of the clock-change line most of all.
- Traditional Chinese is shown as simplified.
- Not yet tested on a clean Windows, on a PC set to another time zone than Paris, or with another Windows display language.

## 1.1.1 — 25 October

### Fixed
- The title tag “24-hour days: start → end” now writes the real local hours of the start and the end of the week, so it says the same thing as the “Your plan resets to 100% in” card. In the week of the move to winter time it reads “Thu 1pm → Thu 12pm”; at the end of March “Thu 12pm → Thu 1pm”; an ordinary week is unchanged.

### Tested
- 73 tests, 0 failures, 0 warnings. A new test file fails on the 1.1.0 behaviour (25 October and end of March), the ordinary week is a guard.

## 1.1.0 — 25 October

### Fixed
- The “Your plan resets to 100% in” card wrote the hour at which the week started instead of the hour at which it ends. In the week of the move to winter time the reset is on Thursday 29 October at 12:00, not 1pm.

### Added
- A notice on its own line, shown only in the week when the clock changes between the start and the end of the week (“One hour earlier: clocks go back to standard time on Sunday 25 October.”; at the end of March “later … daylight saving time”). Direction and date are computed from the two instants and the time-zone table.
- Two demo weeks for screenshots (`CONSO_TOKEN_DEMO=hiver` and `mars`).

### Tested
- 71 tests, 0 failures, 0 warnings. Three of the four new tests of the reset card fail on the 1.0.0 behaviour; the fourth (an ordinary week with no notice) is a guard.

## 1.0.0 — 24 October — first versioned release

What this first numbered version contains (the first working version, 0.1.0, already had the same views):
- The five views of the “Your usage, day by day” card with a selector: Today, Lines, Trajectory, Batteries (default) and Week. The choice is kept in `vue.txt`.
- Day labels that name the two days a 24-hour block overlaps (`thu→fri`), and the title tag “24-hour days: Thu 1pm → Thu 1pm”.
- The reset hour written the same way as the tag, for two real reset schedules (Thursday 1pm and Saturday 0:00).
- An icon for the window and for the `.exe`.
- A program with no trace of the machine it was built on: the build replaces the workstation paths with neutral names.

### Tested
- 68 tests, 0 failures, 0 warnings.

## 0.1.0 — mid-October — first working version

### Added
- The window, in a dark “luxury” style, with the five views of the usage day by day. Texts in French, hours in Paris time.
- The bridge: `conso_token.exe --pont` reads the JSON of Claude Code's status line on its standard input and stores the figures in `%LOCALAPPDATA%\ConsoToken` (`dernier.json` and `historique.jsonl`); `--pont --suite <command>` forwards the same JSON to a status line already in place.
- A week is detected from `resets_at`, which must move forward by more than 3 days; the percentage alone proves nothing, because every Claude Code session reports the percentage of its own last answer.
- The budget of a day is what is left at the start of the day divided by the days left, so carry-over is automatic.
- Tokens per day, counted from the timestamps and `usage` counters of the Claude Code transcripts, never from their content.
- A first-opening page, an About screen with the Slint attribution, and a demo mode (`--demo`).
- A scheduled task of the Claude desktop app that reads the plan usage with the app's own `get_usage` tool every 15 minutes and feeds the bridge, so the figures also follow the desktop app.
