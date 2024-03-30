/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use std::path::PathBuf;

use megalodon::megalodon::{PostStatusInputOptions, PostStatusOutput, UploadMediaInputOptions};
use megalodon::{entities, error, generator, Megalodon};

use crate::config::DailyScryConfig;
use crate::error::{Error, Result};
use crate::util::{split_text, Additional};

pub async fn post(
    config: &DailyScryConfig,
    card_texts: Vec<String>,
    artist: Option<String>,
    images: Vec<PathBuf>,
    link: &str,
) -> Result<Vec<PostStatusOutput>> {
    let client = create_client(config).await?;

    let text = card_texts.join("\n");
    let hashtags = "\n#MagicTheGathering #DailyScry".to_owned();
    let artist = format!("\n{}", artist.unwrap_or_default());

    let splitted_texts = split_text(
        text,
        config.mastodon_character_limit.unwrap(),
        vec![
            Additional::Text(hashtags.clone()),
            Additional::Text(artist.clone()),
            Additional::Number(23), // This is for the link as links in mastodon always take up 23 characters See: https://docs.joinmastodon.org/user/posting/#links
        ],
    );

    let images_and_texts = images.iter().zip(card_texts.iter());

    let media_ids_futures = images_and_texts
        .map(|(image, card_text)| upload_media_file(&client, &image, card_text.to_string()));

    let media_ids = futures::future::join_all(media_ids_futures)
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, megalodon::error::Error>>()?;

    let status = format!("{}{}{}{}", splitted_texts[0], artist, link, hashtags);

    let result = post_status(&client, &status, Some(media_ids), None)
        .await
        .map_err(|error| Error::MegalodonError { error: error })?;

    let mut reply_id = match result.clone() {
        PostStatusOutput::Status(status) => status.id,
        PostStatusOutput::ScheduledStatus(_) => "".to_owned(),
    };

    let mut results = vec![result];

    for splitted_text in splitted_texts.into_iter().skip(1) {
        let additional_status = format!("{}{}{}{}", splitted_text, artist, link, hashtags);
        let additional_result = post_status(&client, &additional_status, None, Some(reply_id))
            .await
            .map_err(|error| Error::MegalodonError { error: error })?;
        reply_id = match additional_result.clone() {
            PostStatusOutput::Status(status) => status.id,
            PostStatusOutput::ScheduledStatus(_) => "".to_owned(),
        };
        results.push(additional_result);
    }

    return Ok(results);
}

async fn create_client(config: &DailyScryConfig) -> Result<Box<dyn Megalodon + Send + Sync>> {
    let client = generator(
        megalodon::SNS::Mastodon,
        config.mastodon_url.clone().unwrap().clone(),
        Some(config.mastodon_access_token.clone().unwrap().clone()),
        Some("DailyScry".to_string()),
    );

    let res = client.verify_account_credentials().await;

    if res.is_err() {
        return Err(Error::MegalodonError {
            error: res.expect_err("Should not happen"),
        });
    }

    Ok(client)
}

async fn wait_until_uploaded(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    id: &str,
) -> std::result::Result<entities::Attachment, megalodon::error::Error> {
    loop {
        let res = client.get_media(id.to_string()).await;
        return match res {
            Ok(res) => Ok(res.json()),
            Err(err) => match err {
                error::Error::OwnError(ref own_err) => match own_err.kind {
                    error::Kind::HTTPPartialContentError => continue,
                    _ => Err(err),
                },
                _ => Err(err),
            },
        };
    }
}

async fn upload_media_file(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    file_path: &PathBuf,
    description: String,
) -> std::result::Result<String, megalodon::error::Error> {
    let options = UploadMediaInputOptions {
        description: Some(description),
        focus: None,
    };
    let res = client
        .upload_media(
            file_path.clone().into_os_string().into_string().unwrap(),
            Some(&options),
        )
        .await?;

    let uploaded_media = res.json();

    let media: entities::Attachment;

    match uploaded_media {
        entities::UploadMedia::AsyncAttachment(m) => {
            match wait_until_uploaded(&client, &m.id).await {
                Ok(attachment) => media = attachment,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        entities::UploadMedia::Attachment(m) => {
            media = m;
        }
    }

    return Ok(media.id);
}

async fn post_status(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    status: &str,
    media_ids: Option<Vec<String>>,
    in_reply_to_id: Option<String>,
) -> std::result::Result<megalodon::megalodon::PostStatusOutput, megalodon::error::Error> {
    let res = client
        .post_status(
            status.to_string(),
            Some(&PostStatusInputOptions {
                media_ids: media_ids,
                sensitive: Some(false),
                visibility: Some(entities::StatusVisibility::Public),
                in_reply_to_id: in_reply_to_id,
                language: Some("en".to_string()),
                ..Default::default()
            }),
        )
        .await?;
    Ok(res.json())
}
