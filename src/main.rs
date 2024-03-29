/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use crate::config::cli_config::CLIConfig;
use crate::config::DailyScryConfig;
use crate::error::Result;
use format::get_artist;
use log::{debug, error, info, trace};
use megalodon::megalodon::PostStatusOutput;
use scryfall::Card;
use std::{path::PathBuf, process};

mod card;
mod config;
mod error;
mod format;
mod image;
mod mastodon;
mod telegram;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli_config = CLIConfig::new();

    if cli_config.dry_run {
        println!("dry run…")
    }

    let config = DailyScryConfig::new();

    let card = card::random_card(&config).await?;

    let link = link(&card);

    let card_texts = format_card(&card);

    let artist = get_artist(&card)?;

    if !cli_config.mastodon && !cli_config.telegram {
        println!(
            "{}{}\n\n{}",
            card_texts.join("\n\n"),
            artist.clone().unwrap_or_default(),
            link
        );
        return Ok(());
    }

    if cli_config.dry_run {
        debug!("This was a dry run. Exiting…");
        process::exit(0)
    }

    let image_paths = download_image(&config, &card).await?;

    if cli_config.mastodon {
        config.check_mastodon_config()?;
        post_to_mastodon(
            &config,
            card_texts.clone(),
            artist.clone(),
            image_paths.clone(),
            link,
        )
        .await?;
    }

    if cli_config.telegram {
        config.check_telegram_config()?;
        post_to_telegram(
            &config,
            card_texts.clone(),
            artist.clone(),
            image_paths.clone(),
            link,
        )
        .await?;
    }

    Ok(())
}

async fn download_image(config: &DailyScryConfig, card: &Card) -> Result<Vec<PathBuf>> {
    trace!("downloading card images…");
    let image_paths = image::download_images(&config, &card).await?;
    debug!("downloaded card images {:?}", image_paths);
    Ok(image_paths)
}

fn format_card(card: &Card) -> Vec<String> {
    return match format::format_card(&card) {
        Err(error) => {
            error!("encountered error: {}", error);
            process::exit(1)
        }
        Ok(texts) => {
            info!("got card texts.");
            debug!("card texts {:?}", texts);
            texts
        }
    };
}

fn link(card: &Card) -> &str {
    let link: &str = card.scryfall_uri.as_str();
    info!("link to card {}", link);
    link
}

async fn post_to_mastodon(
    config: &DailyScryConfig,
    card_texts: Vec<String>,
    artist: Option<String>,
    image_paths: Vec<PathBuf>,
    link: &str,
) -> Result<()> {
    debug!("creatiung mastodon post…");
    let output = mastodon::post(&config, card_texts, artist, image_paths, link).await?;

    match output {
        PostStatusOutput::Status(status) => println!("Posted: {}", status.url.unwrap()),
        PostStatusOutput::ScheduledStatus(scheduled_status) => {
            println!("Will post at {}", scheduled_status.scheduled_at)
        }
    }
    Ok(())
}

async fn post_to_telegram(
    config: &DailyScryConfig,
    card_texts: Vec<String>,
    artist: Option<String>,
    image_paths: Vec<PathBuf>,
    link: &str,
) -> Result<()> {
    debug!("creatiung telegram post…");
    telegram::post(config, card_texts, artist, image_paths, link).await?;
    println!("Posted to {}", config.telegram_chat_id.clone().unwrap());
    Ok(())
}
