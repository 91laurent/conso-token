# Conso Token

**Free and open source** · [MIT licence](LICENSE) · Windows 10 and 11 · 8 languages

A small Windows desktop window that shows where you stand on your Claude plan: how much of the week you have used, what you can still use today and on each following day, and whether you are ahead of or over the even pace. It predicts nothing, uses no network and asks for no password.

Since 1.3 a second tab, **Conversations**, shows each Claude Code conversation of the last 24 hours: how full its context is (approximate) and how much is left before it is time to carry on in a fresh conversation. You can split the conversations into two named groups (and, since 1.3.5, [add up to six more by hand](#more-than-two-groups-since-135)), and hide a conversation with a ✕ (handy for an archived one). The first screenshot below shows the **Week** tab, the second the **Conversations** tab; the SETTINGS window is shown further down, in “How it works”.

![The Conso Token window, Week tab: pace, what is left, reset countdown, the week at a glance, usage day by day and what you can still use](docs/images/window.png)

The **Conversations** tab, with made-up conversations in three groups: the two groups of SETTINGS (“Main chats”, warning 70 and hand-off 80; “Side tasks”, 50 and 65) and a third one written by hand, “Reviews” (40 and 55). Each row shows how full the context of the conversation is, with the warning mark (dotted) and the hand-off mark (cream) and a ✕ at the right to hide it; a blinking dot is on the tab:

![The Conversations tab: three groups of conversations, the third one, “Reviews”, written by hand with its own thresholds (warning 40, hand-off 55); each row has its title, its folder, a bar showing how full its context is, what is left before the hand-off, and a ✕ to hide it](docs/images/conversations.png)

> Independent project, not affiliated with or endorsed by Anthropic. “Claude” belongs to its owner.

**It is free.** Nothing to buy, no account, no subscription. The code is open under the [MIT licence](LICENSE): you may use, copy, modify and share it. The parts it is built with (Slint, two fonts) keep their own licences, listed in [Licences](#licences). The screenshots show demo figures and made-up conversations.

Eight languages: English, Français, Español, Deutsch, Português (BR), Italiano, 中文, 日本語. The program follows the language and the time zone of Windows; a button at the top right changes the language.

## Download

Both files are on the [latest release](../../releases/latest). The checksums (SHA-256) are in the release notes.

| File | Size | What it is |
|---|---|---|
| `conso_token_1.3.8.zip` | 5.6 MB | **The app.** `conso_token.exe` with the one-page guides in eight languages, the setup text and the licences. Unzip the whole `Conso Token` folder, then follow the guide below. |
| `conso_token_presentation.zip` | 15.4 MB | **The presentation and the reading guide**, remade for this version: the screens of 1.3.8 and a section on the Conversations tab. Unzip, then open `index.html` (presentation) or `guide.html` (how to read every number) in a browser. Eight languages, nothing to install, no network. |

The same two pages are in this repository, in [`docs/site/`](docs/site/).

## Install

Unzip the whole `Conso Token` folder of `conso_token_1.3.8.zip` on a drive of your PC, then follow the one-page guide in your language (`README.en.txt` and its translations). About 10 minutes, once: you write one sentence in Claude Code, and Claude sets up a task that sends your figures to the window every 15 minutes.

Needs Windows 10 or 11, the Claude desktop app and a paid Claude plan whose usage figures the app provides: some plans do not provide them, and Conso Token has been tried with one plan only. The program is not signed by a paid publisher, so Windows shows a warning the first time (*More info → Run anyway*).

**Updating from an earlier version:** close the window, replace the whole `Conso Token` folder, then give Claude the setup sentence again so that it updates the task and tests the send (the send log is now in the data folder). Your data folder is kept.

**Removing it:** delete the scheduled task too, not only the folder. The steps are in each one-page guide (“How to remove it”).

## How it works

- `conso_token.exe` opens the window.
- `conso_token.exe --pont` receives a small JSON on its standard input (the same format as Claude Code's status line) and stores the plan figures in `%LOCALAPPDATA%\ConsoToken` (`dernier.json` for the last reading, `historique.jsonl` for every change). It is silent. `--pont --suite <command>` forwards the same JSON to a status line already in place.
- The desktop app of Claude cannot call a status line, so a scheduled task of the app (`conso-token-feed`, every 15 minutes) reads the plan usage with the app's own `get_usage` tool and calls [`feeder/envoyer_pont_cache.vbs`](feeder/envoyer_pont_cache.vbs) with four integers (weekly and 5-hour percentages and reset times in Unix seconds; the 5-hour pair can be `- -` for plans that have no 5-hour window). No token is involved. The second script, [`feeder/envoyer_pont.cmd`](feeder/envoyer_pont.cmd), looks for `conso_token.exe` next to itself, then in the parent folder, and writes the send log (`dernier_envoi.txt`) and its temporary input (`entree.json`) in the data folder, `%USERPROFILE%\AppData\Local\ConsoToken`, never next to the program. The task shows in the Claude app under the title “Conso Token feed”, among its scheduled tasks (“Scheduled”, or “Routines” in the “Code” tab, depending on the app version).
- A week is `resets_at − 7 days` to `resets_at`, read again at every update, so your own reset day and hour are used. A new week is detected only when `resets_at` moves forward by more than 3 days: the percentage alone proves nothing, because every Claude Code session reports the percentage of *its* last answer.
- A day is a block of 24 h counted from the start of the week. The budget of a day is *what is left at the start of the day ÷ the number of days left (today included)*, so carry-over is automatic. The gap of a day is its budget minus what was used.
- Tokens per day come from the timestamps and the `usage` counters of the Claude Code transcripts (`~/.claude/projects`, or `CLAUDE_CONFIG_DIR`), never from their content. Counted: input + output + cache creation; cache reads are not counted.

- The **Conversations** tab also reads the Claude Code transcripts (the same files), only for the conversations whose last message is less than 24 hours old: the context at Claude's last message (input + cache creation + cache read), the model, the name of the working folder, whether Claude compacted the conversation, and the title of the conversation as the Claude app shows it. Its settings (groups and their names, the names of the folders of the first group, thresholds, capacity of a conversation by model, hidden conversations, and the lines you write by hand for [groups 3 to 8](#more-than-two-groups-since-135): names, folder names, title beginnings) are kept in `conversations.txt` in the data folder; `contextes_vus.txt` keeps the largest context seen for each model (numbers only).
- **What is kept, written and sent.** From the Claude Code files the program keeps the counters (dates, model, token counts), the name of the working folder and the title of each conversation. The title is read in the files already read for the counters, kept in memory while it is displayed, and never written or sent; the text of a message is never kept, written or sent. Folder names reach `conversations.txt` only when you choose folders for a group (ticked in the window, moved by a click on a tag, or typed by hand); what you type there yourself, title beginnings included, is written back as part of your settings. When you hide a conversation with ✕, the program writes that conversation's session identifier (a plain code, neither its title nor its contents) in `conversations.txt`, so that it stays hidden; it is forgotten once the conversation's last message is more than 24 h old, or as soon as you click “Show again”. Nothing is sent over the Internet.

The **SETTINGS** window (button at the top right of the window): one or two groups with their names and their warning and hand-off thresholds, the capacity of a conversation by model (“Conversation capacity, by model”), the blinking dot, the animated figures, “Reset”, and “Show again” when conversations are hidden. It does not show the hand-written groups. The figures in this screenshot are made up:

![The SETTINGS window: split into two groups, a name and warning and hand-off thresholds for each group, the capacity of a conversation by model (Auto, 200,000 or 1,000,000), the blinking dot on the Conversations tab, and the animated figures](docs/images/configuration.png)

Options: `--demo` (a made-up week, to see the screen), `--donnees <folder>` (data folder), `--sans-tokens`. Environment variables for tests and screenshots: `CONSO_TOKEN_DATA`, `CONSO_TOKEN_LANGUE`, `CONSO_TOKEN_TZ`, `CONSO_TOKEN_VUE` (0 to 4), `CONSO_TOKEN_DEMO` (`debut`, `milieu`, `hiver`, `mars`), `CONSO_TOKEN_AUTOQUIT` (seconds), `CONSO_TOKEN_APROPOS`.

## More than two groups (since 1.3.5)

SETTINGS gives you one or two groups. You can add up to six more, **groups 3 to 8**, each with its own name and its own warning and hand-off thresholds, by writing them by hand in `conversations.txt` in the data folder. This is useful when two kinds of conversations share a work folder but should not be warned at the same level.

1. In SETTINGS, tick **“Split into two groups”**. Hand-written groups are shown only when it is ticked (this also makes sure the file exists).
2. **Close Conso Token.** While it runs it rewrites the whole file from its memory, so a change made then is ignored and overwritten.
3. Add lines at the end of the file and save it as UTF-8. Keys are lower case, with the group number at the end. For example, the third group of the screenshot above:

```
nom3=Reviews
attention3=40
passation3=55
titre3=Review
```

To also send every conversation of one work folder to a fourth group:

```
nom4=Experiments
groupe4=project-a
```

| Key (here for group 3) | Meaning |
|---|---|
| `nom3=` | The name shown, 24 characters at most. Without it: “Group 3”. |
| `attention3=` | Warning threshold, 10 to 95 (default 70; never above the hand-off). |
| `passation3=` | Hand-off threshold, 10 to 95 (default 80). |
| `titre3=` | The beginning of a title. One line per beginning. |
| `groupe3=` | A work folder: the last part of its path, spelled exactly (case counts). One line per folder. |

- **Order of the rules:** first the title (the matching group with the lowest number), then the folder (same rule), then the two groups of SETTINGS. On screen, groups 1 and 2 come first, then groups 3 to 8 by number.
- **A title beginning is a whole word** (or words), upper or lower case alike: it must be followed by a space or end the title. `Review` takes “Review of the plan”, but not “Review: plan”, “Reviews” or “The review”.
- The tag of a row in a hand-written group **cannot be clicked** (in groups 1 and 2 a click still moves the whole folder to the other one).
- SETTINGS does not show or change these groups. The program reads them at start-up and writes them back each time it rewrites the file (trimmed, duplicates removed, thresholds brought within their limits). **“Reset” erases them**; **“Keep one group”** hides them without erasing them.
- What you type there (group names, folder names, title beginnings) stays in that file on your PC.

## Build and test

Rust 1.99 and Slint 1.18.1. Windows only, PowerShell 7 (`pwsh`). Keep the build folder **short**: a path of more than 260 characters makes `link.exe` fail (LNK1104). Download the libraries once, then build with [`construire.ps1`](construire.ps1), at the root of the repository (it works offline):

```
cargo fetch --locked
pwsh -File construire.ps1 -Cible C:\ct -Action test
pwsh -File construire.ps1 -Cible C:\ct -Action test-release
pwsh -File construire.ps1 -Cible C:\ct -Action build
```

`test` runs the tests of the calculations (`noyau`); `test-release` builds the program in release mode and runs its tests of the bridge (`--pont`); `build` (the default) writes `C:\ct\release\conso_token.exe`.

`.cargo/config.toml` links the C runtime into the program (`crt-static`), so no Visual C++ redistributable is needed, and fixes the test time zone to `Europe/Paris` and the default language to French (tests that need another language say so). It contains no path of any computer. Why the script: it works out, on the computer where it runs, the paths that must not be written into the program (your user profile, the Cargo and Rust folders, the source and build folders) and replaces them while compiling. Plain `cargo` works too, but leaves such paths in the program. After each build, search the program for the names and paths you do not want to ship.

Layout:

- `noyau/` — pure calculations, no UI: week, budget, gap, carry-over (`modele`), the bridge (`pont`), the transcripts (`transcripts`), time zones (`fr`, `temps`), texts in eight languages (`textes`), the five views (`vues`), the conversations of the last 24 hours (`conversations`, `suivi`) and their settings, hidden conversations included (`reglages`). Tested alone.
- `appli/` — the Slint window (`ui/app.slint`), `src/main.rs`, `src/ecran.rs` (what the screen reads, writes and shows), fonts, icon.
- `feeder/` — the two scripts that send the figures; the scheduled task calls the first, which runs the second (in a release they live in `outils\`).
- `docs/` — the one-page guides in eight languages, the text Claude follows during setup (`PASTE_INTO_CLAUDE.txt`) and the web pages in `docs/site/`.
- `licences/` — licence texts shipped with the program.
- `construire.ps1` — builds and tests without writing the paths of your computer into the program.

Tips: Slint centres an element without `x`/`y` inside a `Rectangle`: set `x: 0px; y: 0px`. A test that writes files must use `CARGO_TARGET_TMPDIR`, not the system temp folder. The Claude desktop app is a Windows package: programs launched by its tools see a redirected `AppData\Local`, so the setup launches the feeder outside the package (WMI).

## Known limits

The full lists are in the [changelog](CHANGELOG.md) (entries 1.3.4, 1.3.5 and 1.3.8). The main ones:

- How much a conversation can hold is in no file, so the percentage is approximate (≈); set the right capacity by model in SETTINGS if it looks off.
- **Hand-written groups:** six at most (3 to 8); changed only with the program closed; not shown in SETTINGS; their rows use the same tag colour as group 2. The short tag of a row is made of 4 characters of the group name at most, so two names that start alike give the same tag. A conversation placed by its title falls back to its folder if the title can no longer be read after a restart (it is looked for in the last 64 KB of the conversation file).
- The program has been tried with one Claude plan only; some plans do not give their usage figures to the app.
- The Conversations list follows Claude's last message, not the state of the window: an archived or closed conversation stays listed for 24 hours (hide it with ✕), and an open conversation with no message for more than 24 hours does not appear.
- If Claude Code changes its file format the list can be empty **without a warning**.
- **Hidden conversations.** A hidden conversation is forgotten as soon as a reading shows other conversations but not it (normally because its last message is more than 24 hours old; also, rarely, if a project sub-folder cannot be read for a moment: the program ignores that error), and it then comes back in the list. When nothing at all can be read, nothing is forgotten.
- The ✕ of a conversation whose identifier is refused (more than 128 characters, or a control character: Claude's own identifiers have 36 characters) does nothing and says nothing: the identifier is not stored.
- A hand-made `conversations.txt` with thousands of `masquee=` lines slows the window (the duplicates are removed by a comparison that grows with the square of the list: 10,000 lines took about 8 seconds at each reading in a debug build). The program cannot write such a file, because it forgets identifiers at every reading.
- `conversations.txt` is read as UTF-8 and its keys are case-sensitive: a file saved in ANSI (Latin-1 bytes) makes ALL the settings go back to their defaults, hidden conversations included, and the file is replaced at the next change; `MASQUEE=` is ignored.
- No automatic test covers the Slint screens (they are checked by eye), and the translations have not been reviewed by native speakers (Chinese and Japanese above all).
- The build is not reproducible to the byte: a build of these sources is not identical to the released `conso_token.exe` (the replaced paths depend on the folders used, and the linker writes a time stamp and an identifier). The SHA-256 of the released exe is in the release notes.

## Translations

All texts of the program are in one table (`noyau/src/textes.rs`): one line per text, eight cells. The translations have not been reviewed by native speakers yet: corrections are welcome (open an issue or a pull request).

## Licences

MIT for the code, and the licences of the components Conso Token is built with:

- **Conso Token** (this repository): [MIT](LICENSE).
- **Slint** (user interface): “Slint Royalty-free 2.0”. It requires an attribution, which the About screen of the program shows.
- **Cormorant Garamond** and **Jost** (fonts): SIL Open Font License 1.1.
- **serde, serde_json, jiff** and their dependencies: MIT, Apache-2.0 or Unlicense, crate by crate.

The full texts are in [`licences/`](licences/) and the details in [NOTICE.md](NOTICE.md). See also the [changelog](CHANGELOG.md).
