# Contributing

Conso Token is free and open source (MIT). Corrections, translations and bug reports are welcome. By contributing you agree that your contribution is released under the same MIT licence.

## Report a problem

Open an issue and give:

- the **version number**: click **ABOUT** at the top right of the window, it is the first line;
- the day and hour your plan resets (the “Your plan resets to 100% in” card shows it);
- what you expected, what you saw, and a screenshot of the window if you can (the figures are only percentages).

If nothing arrives in the window, open `%USERPROFILE%\AppData\Local\ConsoToken\dernier_envoi.txt` (type that path in the address bar of File Explorer) and paste its last line. That line can end with the path of your data folder, which contains your Windows user name: replace the name before you post it.

## Fix a translation

All the texts of the program are in one table, [`noyau/src/textes.rs`](noyau/src/textes.rs): one line per text and eight cells, in the order `en, fr, es, de, pt, it, zh, ja`. Keep the `{0}`, `{1}` placeholders: the tests check that they are the same in the eight languages and that no cell is empty. The one-page guides are in [`docs/`](docs/) and the web pages in [`docs/site/`](docs/site/) (the texts are at the top of `shared.js`, `index.html` and `guide.html`).

The translations have not been reviewed by native speakers yet, so a native reader's correction is the most useful contribution.

## Change the code

Windows only. Rust 1.99, Slint 1.18.1, PowerShell 7. See “Build and test” in the [README](README.md): build through `construire.ps1` (plain `cargo` leaves the paths of your computer in the program), keep the build folder short (a path of more than 260 characters makes `link.exe` fail), and run both sets of tests before you open a pull request.

```
cargo fetch --locked
pwsh -File construire.ps1 -Cible C:\ct -Action test
pwsh -File construire.ps1 -Cible C:\ct -Action test-release
```

- The calculation lives in `noyau/` and has no user interface, so it can be tested alone. Add a test with every change of rule.
- A change in the package files (feeder scripts, setup text, guides) needs a new version number: two different packages must never carry the same one. Add a line to [CHANGELOG.md](CHANGELOG.md).
- No network code, no secrets, nothing kept from the text of the messages: the program only reads counters and, to name the rows of the Conversations tab, the title of each conversation (kept in memory, never written or sent). The only thing it writes to name one particular conversation is the session identifier of a conversation hidden with ✕ (`conversations.txt`). That file also holds the user’s settings: the names of the folders chosen for a group, and what the user types in it by hand (group names, folder names and, for groups 3 to 8, title beginnings). Any change that reads, keeps, writes or sends more than that changes this promise and must be said in the README, the changelog and the About screen.
