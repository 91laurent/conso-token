# Contributing

Conso Token is free and open source (MIT). Corrections, translations and bug reports are welcome. By contributing you agree that your contribution is released under the same MIT licence.

## Report a problem

Open an issue and give:

- the **version number**: click **ABOUT** at the top right of the window, it is the first line;
- the day and hour your plan resets (the “Your plan resets to 100% in” card shows it);
- what you expected, what you saw, and a screenshot of the window if you can (the figures are only percentages).

If nothing arrives in the window, open `outils\dernier_envoi.txt` in the Conso Token folder and paste its last line.

## Fix a translation

All the texts of the program are in one table, [`noyau/src/textes.rs`](noyau/src/textes.rs): one line per text and eight cells, in the order `en, fr, es, de, pt, it, zh, ja`. Keep the `{0}`, `{1}` placeholders: the tests check that they are the same in the eight languages and that no cell is empty. The one-page guides are in [`docs/`](docs/) and the web pages in [`docs/site/`](docs/site/) (the texts are at the top of `shared.js`, `index.html` and `guide.html`).

The translations have not been reviewed by native speakers yet, so a native reader's correction is the most useful contribution.

## Change the code

Windows only. Rust 1.99, Slint 1.18.1. See “Build and test” in the [README](README.md): keep the build folder short (a path of more than 260 characters makes `link.exe` fail), and run the whole test suite before you open a pull request.

```
cargo test --workspace --target-dir C:\ct
```

- The calculation lives in `noyau/` and has no user interface, so it can be tested alone. Add a test with every change of rule.
- A change in the package files (feeder scripts, setup text, guides) needs a new version number: two different packages must never carry the same one. Add a line to [CHANGELOG.md](CHANGELOG.md).
- No network code, no secrets, nothing read from the content of the conversations: the program only reads counters.
