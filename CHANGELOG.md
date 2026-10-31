# Changelog

All notable changes to Conso Token are listed here, newest first. Versions follow the number shown on the **About** screen of the program. Dates are 2026.

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
