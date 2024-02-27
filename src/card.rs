/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{config::DailyScryConfig, error::Result};
use log::{debug, trace};
use scryfall::Card;

pub async fn random_card(_config: &DailyScryConfig) -> Result<Card> {
    debug!("calling scryfall to get random card…");
    let card = Card::random().await?;
    trace!("got card with id {}", card.oracle_id.unwrap());
    trace!("{:#?}", card);
    Ok(card)
}
