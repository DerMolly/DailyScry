/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use crate::card::filter::{CardFilter, ContentWarningFilter, IgnoredOracleIdFilter};
use crate::card::random::RandomCardGetter;
use crate::config::DailyScryConfig;
use crate::error::Result;

use log::{debug, trace};
use scryfall::Card;

pub use crate::card::random::DefaultRandomCardGetter;

mod filter;
mod random;

pub async fn random_card<T: RandomCardGetter>(
    config: &DailyScryConfig,
    mut random_card_getter: T,
) -> Result<Card> {
    debug!("calling scryfall to get random card…");
    let filters_vec: Vec<&dyn CardFilter> =
        vec![&IgnoredOracleIdFilter {}, &ContentWarningFilter {}];
    let filters = filters_vec.into_iter();

    let mut card: Card;
    loop {
        card = random_card_getter.get_random_card().await?;

        let filter_results = filters.clone().map(|card_filter| {
            return (card_filter.filter(config, card.clone()), card_filter.name());
        });

        if filter_results.clone().all(|(filtered, _)| filtered) {
            debug!("all card filters return true");
            break;
        }

        let option_name = filter_results.clone().find_map(|(filter, name)| {
            if filter {
                return None;
            }
            return Some(name);
        });

        println!(
            "'{}' filters '{}' and it will be ignored",
            option_name.unwrap(),
            card.clone().name,
        );
    }

    trace!("got card with id {}", card.oracle_id.unwrap());
    trace!("{:#?}", card);
    Ok(card)
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;

    struct TestCardGetter {
        call_index: usize,
        cards: Vec<Card>,
    }

    impl TestCardGetter {
        pub fn new(cards: Vec<Card>) -> Self {
            TestCardGetter {
                call_index: 0,
                cards: cards,
            }
        }
    }

    impl RandomCardGetter for TestCardGetter {
        async fn get_random_card(&mut self) -> Result<Card> {
            if self.call_index >= self.cards.len() {
                return Err(Error::ScryfallError {
                    error: scryfall::Error::Other("TooManyRequest to get_random_card".to_owned()),
                });
            }
            let card = self.cards[self.call_index].clone();
            self.call_index += 1;
            Ok(card)
        }
    }

    fn build_config(ignored_oracle_id: Option<&str>) -> DailyScryConfig {
        let ignored_oracle_ids =
            ignored_oracle_id.map(|oracle_id| vec![oracle_id.parse().unwrap()]);
        DailyScryConfig {
            mastodon_url: None,
            mastodon_access_token: None,
            mastodon_character_limit: None,
            telegram_token: None,
            telegram_chat_id: None,
            telegram_character_limit: None,
            image_path: "test/".to_string(),
            ignored_oracle_ids: ignored_oracle_ids,
            version: "Test_Version".to_string(),
        }
    }

    #[tokio::test]
    async fn test_ignored_oracle_id() {
        let config = build_config(Some("56719f6a-1a6c-4c0a-8d21-18f7d7350b68"));

        let card_getter = TestCardGetter::new(vec![
            Card::scryfall_id("ddaa0be1-7358-4ea2-8c40-be6d699a6631".parse().unwrap())
                .await
                .unwrap(),
            Card::scryfall_id("b0faa7f2-b547-42c4-a810-839da50dadfe".parse().unwrap())
                .await
                .unwrap(),
        ]);

        let card = random_card(&config, card_getter).await.unwrap();
        assert_eq!(card.name, "Black Lotus")
    }

    #[tokio::test]
    async fn test_ignored_oracle_id_no_ids_ignored() {
        let config = build_config(None);

        let card_getter = TestCardGetter::new(vec![Card::scryfall_id(
            "ddaa0be1-7358-4ea2-8c40-be6d699a6631".parse().unwrap(),
        )
        .await
        .unwrap()]);

        let card = random_card(&config, card_getter).await.unwrap();
        assert_eq!(card.name, "Swamp")
    }

    #[tokio::test]
    async fn test_content_warning() {
        let config = build_config(Some("56719f6a-1a6c-4c0a-8d21-18f7d7350b68"));

        let card_getter = TestCardGetter::new(vec![
            Card::scryfall_id("b6c7705a-2987-4ef1-92b1-2c55d989ec6f".parse().unwrap())
                .await
                .unwrap(),
            Card::scryfall_id("b0faa7f2-b547-42c4-a810-839da50dadfe".parse().unwrap())
                .await
                .unwrap(),
        ]);

        let card = random_card(&config, card_getter).await.unwrap();
        assert_eq!(card.name, "Black Lotus")
    }
}
