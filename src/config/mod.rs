/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

pub mod cli_config;

use crate::error::{Error, Result};

use dotenv::dotenv;
use log::{debug, error};
use std::process;
use uuid::Uuid;

#[derive(Debug)]
pub struct DailyScryConfig {
    pub mastodon_url: Option<String>,
    pub mastodon_access_token: Option<String>,
    pub mastodon_character_limit: Option<usize>,
    pub telegram_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub telegram_character_limit: Option<usize>,
    pub image_path: String,
    pub ignored_oracle_ids: Option<Vec<Uuid>>,
    pub version: String,
}

impl DailyScryConfig {
    pub fn new() -> Self {
        dotenv().ok();
        return match DailyScryConfig::load_config() {
            Err(error) => {
                error!("Encountered error: {}", error);
                process::exit(1)
            }
            Ok(config) => {
                debug!("Config parsed {:?}", config);
                config
            }
        };
    }

    fn load_config() -> Result<DailyScryConfig> {
        let oracle_ids_env =
            std::env::var("DAILY_SCRY_IGNORED_ORACLE_IDS").unwrap_or("".to_owned());
        let oracle_ids_result: std::result::Result<Vec<Uuid>, _> = oracle_ids_env
            .split(",")
            .map(|string_value| Uuid::parse_str(string_value))
            .collect();
        return Ok(DailyScryConfig {
            mastodon_url: std::env::var("DAILY_SCRY_MASTODON_URL").ok(),
            mastodon_access_token: std::env::var("DAILY_SCRY_MASTODON_ACCESS_TOKEN").ok(),
            mastodon_character_limit: std::env::var("DAILY_SCRY_MASTODON_CHARCTER_LIMIT")
                .unwrap_or("500".to_owned())
                .parse()
                .ok(),
            telegram_token: std::env::var("DAILY_SCRY_TELEGRAM_TOKEN").ok(),
            telegram_chat_id: std::env::var("DAILY_SCRY_TELEGRAM_CHAT_ID").ok(),
            telegram_character_limit: std::env::var("DAILY_SCRY_TELEGRAM_CHARCTER_LIMIT")
                .unwrap_or("4096".to_owned())
                .parse()
                .ok(),
            ignored_oracle_ids: if oracle_ids_env.is_empty() {
                Some(vec![])
            } else {
                oracle_ids_result.ok()
            },
            image_path: String::from("/tmp"),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        });
    }

    pub fn validate(&self) -> Result<()> {
        self.check_oracle_ids()?;
        Ok(())
    }

    fn check_oracle_ids(&self) -> Result<()> {
        if self.ignored_oracle_ids.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_IGNORED_ORACLE_IDS".to_string(),
            });
        }

        Ok(())
    }

    pub fn check_mastodon_config(&self) -> Result<()> {
        if self.mastodon_url.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_MASTODON_URL".to_string(),
            });
        }

        if self.mastodon_access_token.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_MASTODON_ACCESS_TOKEN".to_string(),
            });
        }

        if self.mastodon_character_limit.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_MASTODON_CHARCTER_LIMIT".to_string(),
            });
        }

        Ok(())
    }

    pub fn check_telegram_config(&self) -> Result<()> {
        if self.telegram_token.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_TELEGRAM_TOKEN".to_string(),
            });
        }

        if self.telegram_chat_id.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_TELEGRAM_CHAT_ID".to_string(),
            });
        }

        if self.telegram_character_limit.is_none() {
            return Err(Error::ReadConfiguration {
                key: "DAILY_SCRY_TELEGRAM_CHARCTER_LIMIT".to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let mastodon_url = "test_mastodon_url";
        let mastodon_access_token = "test_mastodon_access_token";
        let telegram_token = "test_telegram_token";
        let telegram_chat_id = "test_telegram_chat_id";
        temp_env::with_vars(
            [
                ("DAILY_SCRY_MASTODON_URL", Some(mastodon_url)),
                (
                    "DAILY_SCRY_MASTODON_ACCESS_TOKEN",
                    Some(mastodon_access_token),
                ),
                ("DAILY_SCRY_TELEGRAM_TOKEN", Some(telegram_token)),
                ("DAILY_SCRY_TELEGRAM_CHAT_ID", Some(telegram_chat_id)),
                ("DAILY_SCRY_IGNORED_ORACLE_IDS", None),
            ],
            || {
                let config = DailyScryConfig::load_config().unwrap();
                assert_eq!(config.mastodon_url.unwrap(), mastodon_url);
                assert_eq!(config.mastodon_access_token.unwrap(), mastodon_access_token);
                assert_eq!(config.telegram_token.unwrap(), telegram_token);
                assert_eq!(config.telegram_chat_id.unwrap(), telegram_chat_id);
                assert_eq!(config.mastodon_character_limit.unwrap(), 500);
                assert_eq!(config.telegram_character_limit.unwrap(), 4096);
                assert_eq!(config.ignored_oracle_ids.unwrap().len(), 0);
            },
        );
    }

    #[cfg(test)]
    mod validate {
        use super::super::*;

        #[test]
        fn test_oracle_ids() {
            let mastodon_url = "test_mastodon_url";
            let mastodon_access_token = "test_mastodon_access_token";
            let telegram_token = "test_telegram_token";
            let telegram_chat_id = "test_telegram_chat_id";
            let ignored_oracle_id_1 = "bc71ebf6-2056-41f7-be35-b2e5c34afa99";
            let ignored_oracle_id_2 = "b2c6aa39-2d2a-459c-a555-fb48ba993373";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_MASTODON_URL", Some(mastodon_url)),
                    (
                        "DAILY_SCRY_MASTODON_ACCESS_TOKEN",
                        Some(mastodon_access_token),
                    ),
                    ("DAILY_SCRY_TELEGRAM_TOKEN", Some(telegram_token)),
                    ("DAILY_SCRY_TELEGRAM_CHAT_ID", Some(telegram_chat_id)),
                    (
                        "DAILY_SCRY_IGNORED_ORACLE_IDS",
                        Some(format!("{},{}", ignored_oracle_id_1, ignored_oracle_id_2).as_str()),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.validate().is_ok(), true);
                    let ignored_oracle_ids = config.ignored_oracle_ids.unwrap();
                    assert_eq!(ignored_oracle_ids.len(), 2);
                    assert_eq!(
                        ignored_oracle_ids[0],
                        Uuid::parse_str(ignored_oracle_id_1).unwrap()
                    );
                    assert_eq!(
                        ignored_oracle_ids[1],
                        Uuid::parse_str(ignored_oracle_id_2).unwrap()
                    );
                },
            );
        }

        #[test]
        fn test_inavlid_oracle_ids() {
            let mastodon_url = "test_mastodon_url";
            let mastodon_access_token = "test_mastodon_access_token";
            let telegram_token = "test_telegram_token";
            let telegram_chat_id = "test_telegram_chat_id";
            let ignored_oracle_id_1 = "bc71ebf6-2056-41f7-be35-b2e5c34afa99";
            let ignored_oracle_id_2 = "invalid_uuid";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_MASTODON_URL", Some(mastodon_url)),
                    (
                        "DAILY_SCRY_MASTODON_ACCESS_TOKEN",
                        Some(mastodon_access_token),
                    ),
                    ("DAILY_SCRY_TELEGRAM_TOKEN", Some(telegram_token)),
                    ("DAILY_SCRY_TELEGRAM_CHAT_ID", Some(telegram_chat_id)),
                    (
                        "DAILY_SCRY_IGNORED_ORACLE_IDS",
                        Some(format!("{},{}", ignored_oracle_id_1, ignored_oracle_id_2).as_str()),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.validate().is_err(), true);
                },
            );
        }
    }

    #[cfg(test)]
    mod check_mastodon_config {
        use super::super::*;

        #[test]
        fn test_works() {
            let mastodon_url = "test_mastodon_url";
            let mastodon_access_token = "test_mastodon_access_token";
            let mastodon_character_limit = "1";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_MASTODON_URL", Some(mastodon_url)),
                    (
                        "DAILY_SCRY_MASTODON_ACCESS_TOKEN",
                        Some(mastodon_access_token),
                    ),
                    (
                        "DAILY_SCRY_MASTODON_CHARCTER_LIMIT",
                        Some(mastodon_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.mastodon_url.clone().unwrap(), mastodon_url);
                    assert_eq!(
                        config.mastodon_access_token.clone().unwrap(),
                        mastodon_access_token
                    );
                    assert_eq!(config.mastodon_character_limit.clone().unwrap(), 1);
                    assert_eq!(config.check_mastodon_config().is_ok(), true);
                },
            );
        }

        #[test]
        fn test_url_fail() {
            let mastodon_access_token = "test_mastodon_access_token";
            let mastodon_character_limit = "not_a_number";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_MASTODON_URL", None),
                    (
                        "DAILY_SCRY_MASTODON_ACCESS_TOKEN",
                        Some(mastodon_access_token),
                    ),
                    (
                        "DAILY_SCRY_MASTODON_CHARCTER_LIMIT",
                        Some(mastodon_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.check_mastodon_config().is_err(), true);
                },
            );
        }

        #[test]
        fn test_access_token_fail() {
            let mastodon_url = "test_mastodon_url";
            let mastodon_character_limit = "not_a_number";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_MASTODON_URL", Some(mastodon_url)),
                    ("DAILY_SCRY_MASTODON_ACCESS_TOKEN", None),
                    (
                        "DAILY_SCRY_MASTODON_CHARCTER_LIMIT",
                        Some(mastodon_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.check_mastodon_config().is_err(), true);
                },
            );
        }

        #[test]
        fn test_character_limit_fail() {
            let mastodon_url = "test_mastodon_url";
            let mastodon_access_token = "test_mastodon_access_token";
            let mastodon_character_limit = "not_a_number";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_MASTODON_URL", Some(mastodon_url)),
                    (
                        "DAILY_SCRY_MASTODON_ACCESS_TOKEN",
                        Some(mastodon_access_token),
                    ),
                    (
                        "DAILY_SCRY_MASTODON_CHARCTER_LIMIT",
                        Some(mastodon_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.check_mastodon_config().is_err(), true);
                },
            );
        }
    }

    mod check_telegram_config {
        use super::super::*;

        #[test]
        fn test_works() {
            let telegram_token = "test_telegram_token";
            let telegram_chat_id = "test_telegram_chat_id";
            let telegram_character_limit = "2";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_TELEGRAM_TOKEN", Some(telegram_token)),
                    ("DAILY_SCRY_TELEGRAM_CHAT_ID", Some(telegram_chat_id)),
                    (
                        "DAILY_SCRY_TELEGRAM_CHARCTER_LIMIT",
                        Some(telegram_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::new();
                    assert_eq!(config.telegram_token.clone().unwrap(), telegram_token);
                    assert_eq!(config.telegram_chat_id.clone().unwrap(), telegram_chat_id);
                    assert_eq!(config.telegram_character_limit.clone().unwrap(), 2);
                    assert_eq!(config.check_telegram_config().is_ok(), true);
                },
            )
        }

        #[test]
        fn test_token_fail() {
            let telegram_chat_id = "test_telegram_chat_id";
            let telegram_character_limit = "2";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_TELEGRAM_TOKEN", None),
                    ("DAILY_SCRY_TELEGRAM_CHAT_ID", Some(telegram_chat_id)),
                    (
                        "DAILY_SCRY_TELEGRAM_CHARCTER_LIMIT",
                        Some(telegram_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.check_telegram_config().is_err(), true);
                },
            );
        }

        #[test]
        fn test_chat_id_fail() {
            let telegram_token = "test_telegram_token";
            let telegram_character_limit = "not_a_number";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_TELEGRAM_TOKEN", Some(telegram_token)),
                    ("DAILY_SCRY_TELEGRAM_CHAT_ID", None),
                    (
                        "DAILY_SCRY_TELEGRAM_CHARCTER_LIMIT",
                        Some(telegram_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.check_telegram_config().is_err(), true);
                },
            );
        }

        #[test]
        fn test_character_limit_fail() {
            let telegram_token = "test_telegram_token";
            let telegram_chat_id = "test_telegram_chat_id";
            let telegram_character_limit = "not_a_number";
            temp_env::with_vars(
                [
                    ("DAILY_SCRY_TELEGRAM_TOKEN", Some(telegram_token)),
                    ("DAILY_SCRY_TELEGRAM_CHAT_ID", Some(telegram_chat_id)),
                    (
                        "DAILY_SCRY_TELEGRAM_CHARCTER_LIMIT",
                        Some(telegram_character_limit),
                    ),
                ],
                || {
                    let config = DailyScryConfig::load_config().unwrap();
                    assert_eq!(config.check_telegram_config().is_err(), true);
                },
            );
        }
    }
}
