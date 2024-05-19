/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use std::any::type_name;

use scryfall::Card;

use crate::config::DailyScryConfig;

pub trait CardFilter {
    fn name(&self) -> &'static str {
        return type_name::<Self>().split("::").last().unwrap();
    }

    fn filter(&self, config: &DailyScryConfig, card: Card) -> bool;
}
