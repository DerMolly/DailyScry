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
    pub ignored_oracle_ids: Vec<Uuid>,
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
        return Ok(DailyScryConfig {
            mastodon_url: std::env::var("DAILY_SCRY_MASTODON_URL").ok(),
            mastodon_access_token: std::env::var("DAILY_SCRY_MASTODON_ACCESS_TOKEN").ok(),
            telegram_token: std::env::var("DAILY_SCRY_TELEGRAM_TOKEN").ok(),
            telegram_chat_id: std::env::var("DAILY_SCRY_TELEGRAM_CHAT_ID").ok(),
            ignored_oracle_ids: std::env::var("DAILY_SCRY_IGNORED_ORACLE_IDS")
                .unwrap_or("".to_owned())
                .split(",")
                .map(|string_value| string_value.parse().unwrap())
                .collect(),
            image_path: String::from("/tmp"),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        });
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
