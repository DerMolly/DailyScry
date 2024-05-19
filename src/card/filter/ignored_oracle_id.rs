/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use scryfall::Card;

use crate::card::filter::CardFilter;
use crate::config::DailyScryConfig;

#[derive(Clone)]
pub struct IgnoredOracleIdFilter {}

impl CardFilter for IgnoredOracleIdFilter {
    fn filter(&self, config: &DailyScryConfig, card: Card) -> bool {
        let oracle_id_option = card.oracle_id.clone();

        if oracle_id_option.is_none() {
            println!("card has no oracle_id url: {:?}", card.clone().scryfall_uri);
            return true;
        }

        let oracle_id = oracle_id_option.unwrap();

        let ignored_ids = config.ignored_oracle_ids.clone();

        if ignored_ids.is_none() {
            return true;
        }

        return !(ignored_ids.unwrap().contains(&oracle_id));
    }
}
