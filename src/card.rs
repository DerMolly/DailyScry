/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{config::DailyScryConfig, error::Result};
use log::{debug, trace};
use scryfall::Card;

pub async fn random_card(config: &DailyScryConfig) -> Result<Card> {
    debug!("calling scryfall to get random card…");
    let mut card: Card;
    loop {
        card = Card::random().await?;
        let oracle_id_option = card.oracle_id.clone();
        if oracle_id_option.is_none() {
            println!("card has no oracle_id url: {:?}", card.clone().scryfall_uri);
            break;
        }
        let oracle_id = oracle_id_option.unwrap();
        if !(config
            .ignored_oracle_ids
            .clone()
            .unwrap()
            .contains(&oracle_id))
        {
            break;
        }
        println!("random card '{}' will be ignored ", card.clone().name);
    }

    trace!("got card with id {}", card.oracle_id.unwrap());
    trace!("{:#?}", card);
    Ok(card)
}
