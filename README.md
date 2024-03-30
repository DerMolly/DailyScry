# DailyScry

This bot allows you to post a random image from [Scryfall][scryfall] to an array of
services e.g Mastodon, Telegram or locally.

Posts of this bot can be found at

- <a rel="me" href="https://botsin.space/@dailyscry">@dailyscry@botsin.space</a>
- <https://telegram.me/DailyScry>

## Installation

### cargo

Run `cargo install daily_scry`

### binary release

You can download the latest release form the [release page][releases].

### DIY

1. `git clone https://github.com/DerMolly/DailyScry.git`
2. `cargo build --release`

## Usage

```sh
Post random scryfall image to mastodon, telegram or stdout


Usage: daily_scry [OPTIONS]

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
      --mastodon    Post to mastodon
      --telegram    Post to telegram
      --dry-run     Run the command without posting anything
  -h, --help        Print help
  -V, --version     Print version

Version: 1.0.0

Author: Philip Molares <philip.molares@udo.edu>
```

## Configuration

|        environment variable        |                                                       description                                                         |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `DAILY_SCRY_MASTODON_URL`          | The url of the mastodon instance, where your account is hosted.                                                           |
| `DAILY_SCRY_MASTODON_ACCESS_TOKEN` | The access token for your application.                          							         | 
| `DAILY_SCRY_TELEGRAM_TOKEN`        | The telegram bot token you can get from [@BotFather][botfather]                                                           |
| `DAILY_SCRY_TELEGRAM_CHAT_ID`      | The chat id where the bot should post its message. This can be determinded with [@username_to_id_bot][username_to_id_bot] |
| `DAILY_SCRY_IGNORED_ORACLE_IDS`    | List of oracle_ids that should be ignored and not be randomly selected. Items should be seperated by `,` |

[scryfall]: https://scryfall.com
[releases]: https://github.com/DerMolly/DailyScry/releases
[botfather]: https://telegram.me/BotFather
[username_to_id_bot]: https://telegram.me/username_to_id_bot
