/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use scryfall::card::{Card, Layout};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use url::Url;

use crate::config::DailyScryConfig;
use crate::error::{Error, Result};

/// Downloads the images and returns a vector of file paths for a [`scryfall::card::Card`]
///
/// # Arguments
///
/// * `card` - A borrowed [`scryfall::card::Card`]
pub async fn download_images(config: &DailyScryConfig, card: &Card) -> Result<Vec<PathBuf>> {
    match card.layout.clone() {
        Layout::Normal
        | Layout::Meld
        | Layout::Leveler
        | Layout::Class
        | Layout::Saga
        | Layout::Prototype
        | Layout::Host
        | Layout::Augment
        | Layout::Token
        | Layout::Emblem
        | Layout::Mutate
        | Layout::Planar
        | Layout::Scheme
        | Layout::Vanguard
        | Layout::Split
        | Layout::Flip
        | Layout::Adventure
        | Layout::Case => download_single_image(config, card).await,
        Layout::Transform
        | Layout::ModalDfc
        | Layout::ReversibleCard
        | Layout::DoubleFacedToken
        | Layout::ArtSeries => download_multiple_images(config, card).await,
        _ => Err(Error::ImageNotFound),
    }
}

async fn download_single_image(config: &DailyScryConfig, card: &Card) -> Result<Vec<PathBuf>> {
    let image_uris: Result<Url> = Ok(card.image_uris.clone().ok_or(Error::ImageNotFound)?.png);
    let file_location = download_file(config, image_uris, None).await?;
    Ok(vec![file_location])
}

async fn download_multiple_images(config: &DailyScryConfig, card: &Card) -> Result<Vec<PathBuf>> {
    let faces = card.card_faces.clone().unwrap();
    let image_paths =
        futures::future::join_all(faces.iter().enumerate().map(|(index, face)| async move {
            let image_uri = face
                .image_uris
                .clone()
                .unwrap_or_default()
                .get("png")
                .ok_or(Error::ImageNotFound)
                .cloned();
            download_file(config, image_uri, Some(format!("face_{}.png", index)))
                .await
                .unwrap()
        }))
        .await;
    Ok(image_paths)
}

async fn download_file(
    config: &DailyScryConfig,
    image_uris: Result<Url>,
    optional_file_name: Option<String>,
) -> Result<PathBuf> {
    let file_name = optional_file_name.unwrap_or("test.png".to_string());
    let response = reqwest::get(image_uris?).await?;
    let path = Path::new(&config.image_path).join(file_name.clone());
    let mut file = std::fs::File::create(path.clone())?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(path)
}
