# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),

## Unreleased

### Added

- new environment variable `DAILY_SCRY_IGNORED_ORACLE_IDS`. All oracle_id (seperated by `,`) in it will trigger a new random card, if chosen.

### Changed

- rotate cards that are formatted vertically
- prevenet image download if only the text output is requested

## [1.1.1] - 2024-03-12

This updates multiple dependencies.

### Changed

- Upgrade dependencies: snafu, megalodon, clap, url, log, tokio

## [1.1.0] - 2024-02-28

This release handles cards with flavor names correctly.

### Changed

- name generation (for cards with flavor names).

## [1.0.0] - 2024-02-27

This is the initial release of the software.

### Added

- random card selection
- `--mastodon`: This options posts the selected card to mastodon
- `--telegram`: This options posts the selected card to telegram
- `--help`: Show the help dialog
- `--version`: Show the version
- `--verbose`: Increases the log level. Each instance will increase the verbosity by one stage
  - Error (default)
  - Warn
  - Info
  - Debug
  - Trace
- `--quiet`: Removes all logging output (normal outputs will still be shown)
- `--dry-run`: This option prevents actual posting to anything, but stdout

[1.1.1]: https://github.com/DerMolly/DailyScry/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/DerMolly/DailyScry/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/DerMolly/DailyScry/releases/tag/v1.0.0
