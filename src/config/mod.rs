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
    pub telegram_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub image_path: String,
    pub ignored_oracle_ids: Option<Vec<Uuid>>,
    pub version: String,
}

impl DailyScryConfig {
    pub fn new() -> DailyScryConfig {
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
            telegram_token: std::env::var("DAILY_SCRY_TELEGRAM_TOKEN").ok(),
            telegram_chat_id: std::env::var("DAILY_SCRY_TELEGRAM_CHAT_ID").ok(),
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
}
