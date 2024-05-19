/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use crate::error::Result;

use scryfall::Card;

pub trait RandomCardGetter {
    async fn get_random_card(&mut self) -> Result<Card>;
}

pub struct DefaultRandomCardGetter();

impl RandomCardGetter for DefaultRandomCardGetter {
    async fn get_random_card(&mut self) -> Result<Card> {
        Ok(Card::random().await?)
    }
}
