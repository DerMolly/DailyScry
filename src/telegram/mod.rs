/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use std::path::PathBuf;

use crate::config::DailyScryConfig;
use crate::error::Result;
use crate::util::{split_text, Additional};

use teloxide_core::{
    payloads::{SendMessageSetters, SendPhotoSetters},
    prelude::Request,
    requests::Requester,
    types::{InputFile, ParseMode},
    Bot,
};

pub async fn post(
    config: &DailyScryConfig,
    card_texts: Vec<String>,
    artist: Option<String>,
    images: Vec<PathBuf>,
    link: &str,
) -> Result<()> {
    let images_and_texts = images.iter().zip(card_texts.iter());

    let bot = Bot::new(&config.telegram_token.clone().unwrap());
    let chat_id = config.telegram_chat_id.clone().unwrap();

    let futures = images_and_texts.map(|(image, card_text)| {
        map_function(
            &bot,
            &chat_id,
            artist.clone(),
            &image,
            card_text,
            link,
            config,
        )
    });

    futures::future::join_all(futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}

async fn map_function(
    bot: &Bot,
    chat_id: &String,
    artist: Option<String>,
    image_path: &PathBuf,
    text: &String,
    link: &str,
    config: &DailyScryConfig,
) -> Result<()> {
    send_image(bot, chat_id, image_path, link).await?;
    let artist = artist.unwrap_or_default();
    let splitted_texts = split_text(
        text.to_string(),
        config.telegram_character_limit.unwrap(),
        vec![Additional::Text(artist.clone())],
    );
    for text in splitted_texts {
        send_message(bot, chat_id, &artist, text).await?;
    }
    Ok(())
}

async fn send_image(bot: &Bot, chat_id: &String, image_path: &PathBuf, link: &str) -> Result<()> {
    bot.send_photo(chat_id.clone(), InputFile::file(image_path))
        .caption(link)
        .send()
        .await?;
    Ok(())
}

async fn send_message(bot: &Bot, chat_id: &String, artist: &String, text: String) -> Result<()> {
    let text_with_artist = format!("{}{}", text, artist);
    bot.send_message(chat_id.clone(), text_with_artist)
        .parse_mode(ParseMode::Html)
        .send()
        .await?;
    Ok(())
}
