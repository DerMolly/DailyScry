/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use scryfall::Card;

use crate::card::filter::CardFilter;
use crate::config::DailyScryConfig;

#[derive(Clone)]
pub struct ContentWarningFilter {}

impl CardFilter for ContentWarningFilter {
    fn filter(&self, _: &DailyScryConfig, card: Card) -> bool {
        return !card.content_warning;
    }
}
