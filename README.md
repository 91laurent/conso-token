# Conso Token

**Free and open source** · [MIT licence](LICENSE) · Windows 10 and 11 · 8 languages

A small Windows desktop window that shows where you stand on your Claude plan: how much of the week you have used, what you can still use today and on each following day, and whether you are ahead of or over the even pace. It predicts nothing, uses no network and asks for no password.

![The Conso Token window: pace, what is left, reset countdown, the week at a glance, usage day by day and what you can still use](docs/images/window.png)

> Independent project, not affiliated with or endorsed by Anthropic. “Claude” belongs to its owner.

**It is free.** Nothing to buy, no account, no subscription. The code is open under the [MIT licence](LICENSE): you may use, copy, modify and share it. The parts it is built with (Slint, two fonts) keep their own licences, listed in [Licences](#licences). The screenshot shows demo figures.

Eight languages: English, Français, Español, Deutsch, Português (BR), Italiano, 中文, 日本語. The program follows the language and the time zone of Windows; a button at the top right changes the language.

## Download

Both files are on the [latest release](../../releases/latest). The checksums (SHA-256) are in the release notes.

| File | Size | What it is |
|---|---|---|
| `conso_token_1.2.1.zip` | 5.5 MB | **The app.** `conso_token.exe` with the one-page guides in eight languages, the setup text and the licences. Unzip the whole `Conso Token` folder, then follow the guide below. |
| `conso_token_presentation.zip` | 15.4 MB | **The presentation and the reading guide.** Unzip, then open `index.html` (presentation) or `guide.html` (how to read every number) in a browser. Eight languages, nothing to install, no network. |

The same two pages are in this repository, in [`docs/site/`](docs/site/).

## Install

Unzip the whole `Conso Token` folder of `conso_token_1.2.1.zip` on a drive of your PC, then follow the one-page guide in your language (`README.en.txt` and its translations). About 10 minutes, once: you write one sentence in Claude Code, and Claude sets up a task that sends your figures to the window every 15 minutes.

Needs Windows 10 or 11 and a Claude plan that exposes its usage figures. The program is not signed by a paid publisher, so Windows shows a warning the first time (*More info → Run anyway*).

## How it works

- `conso_token.exe` opens the window.
- `conso_token.exe --pont` receives a small JSON on its standard input (the same format as Claude Code's status line) and stores the plan figures in `%LOCALAPPDATA%\ConsoToken` (`dernier.json` for the last reading, `historique.jsonl` for every change). It is silent. `--pont --suite <command>` forwards the same JSON to a status line already in place.
- The desktop app of Claude cannot call a status line, so a scheduled task of the app (`conso-token-feed`, every 15 minutes) reads the plan usage with the app's own `get_usage` tool and calls [`feeder/envoyer_pont_cache.vbs`](feeder/envoyer_pont_cache.vbs) with four integers (weekly and 5-hour percentages and reset times in Unix seconds; the 5-hour pair can be `- -` for plans that have no 5-hour window). No token is involved.
- A week is `resets_at − 7 days` to `resets_at`, read again at every update, so your own reset day and hour are used. A new week is detected only when `resets_at` moves forward by more than 3 days: the percentage alone proves nothing, because every Claude Code session reports the percentage of *its* last answer.
- A day is a block of 24 h counted from the start of the week. The budget of a day is *what is left at the start of the day ÷ the number of days left (today included)*, so carry-over is automatic. The gap of a day is its budget minus what was used.
- Tokens per day come from the timestamps and the `usage` counters of the Claude Code transcripts (`~/.claude/projects`, or `CLAUDE_CONFIG_DIR`), never from their content. Counted: input + output + cache creation; cache reads are not counted.

Options: `--demo` (a made-up week, to see the screen), `--donnees <folder>` (data folder), `--sans-tokens`. Environment variables for tests and screenshots: `CONSO_TOKEN_DATA`, `CONSO_TOKEN_LANGUE`, `CONSO_TOKEN_TZ`, `CONSO_TOKEN_VUE` (0 to 4), `CONSO_TOKEN_DEMO` (`debut`, `milieu`, `hiver`, `mars`), `CONSO_TOKEN_AUTOQUIT` (seconds), `CONSO_TOKEN_APROPOS`.

## Build and test

Rust 1.99 and Slint 1.18.1. Windows only. Keep the build folder **short**: a path of more than 260 characters makes `link.exe` fail (LNK1104).

```
cargo test --workspace --target-dir C:\ct
cargo build --release -p conso_token --target-dir C:\ct
```

`.cargo/config.toml` links the C runtime into the program (`crt-static`), so no Visual C++ redistributable is needed, and fixes the test time zone to `Europe/Paris` and the default language to French (tests that need another language say so).

Layout:

- `noyau/` — pure calculations, no UI: week, budget, gap, carry-over (`modele`), the bridge (`pont`), the transcripts (`transcripts`), time zones (`fr`, `temps`), texts in eight languages (`textes`), the five views (`vues`). Tested alone.
- `appli/` — the Slint window (`ui/app.slint`), `src/main.rs`, fonts, icon.
- `feeder/` — the two scripts the scheduled task calls (in a release they live in `outils\`).
- `docs/` — the one-page guides in eight languages, the text Claude follows during setup (`PASTE_INTO_CLAUDE.txt`) and the web pages in `docs/site/`.
- `licences/` — licence texts shipped with the program.

Tips: Slint centres an element without `x`/`y` inside a `Rectangle`: set `x: 0px; y: 0px`. A test that writes files must use `CARGO_TARGET_TMPDIR`, not the system temp folder. The Claude desktop app is a Windows package: programs launched by its tools see a redirected `AppData\Local`, so the setup launches the feeder outside the package (WMI).

## Translations

All texts of the program are in one table (`noyau/src/textes.rs`): one line per text, eight cells. The translations have not been reviewed by native speakers yet: corrections are welcome (open an issue or a pull request).

## Licences

MIT for the code, and the licences of the components Conso Token is built with:

- **Conso Token** (this repository): [MIT](LICENSE).
- **Slint** (user interface): “Slint Royalty-free 2.0”. It requires an attribution, which the About screen of the program shows.
- **Cormorant Garamond** and **Jost** (fonts): SIL Open Font License 1.1.
- **serde, serde_json, jiff** and their dependencies: MIT, Apache-2.0 or Unlicense, crate by crate.

The full texts are in [`licences/`](licences/) and the details in [NOTICE.md](NOTICE.md). See also the [changelog](CHANGELOG.md).
