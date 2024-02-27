/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use log::trace;
use scryfall::card::{Card, CardFace, Layout};
use string_builder::Builder;

use crate::error::{Error, Result};

enum CardOrFace<'a> {
    Card(&'a Card),
    Face(&'a CardFace),
}

/// Returns a string representation of a [`scryfall::card::Card`]
///
/// # Arguments
///
/// * `card` - A borrowed [`scryfall::card::Card`]
///
/// # Example
///
/// using https://scryfall.com/card/lea/199/grizzly-bears
///
/// ```
/// Grizzly Bears   {1}{G}
/// Creature — Bear
///
/// Don't try to outrun one of Dominia's Grizzlies; it'll catch you, knock you down, and eat you. Of course, you could run up a tree. In that case you'll get a nice view before it knocks the tree down and eats you.
///
/// 2/2
///
/// Illustrated by Jeff A. Menges
/// ```
pub fn format_card(card: &Card) -> Result<Vec<String>> {
    trace!("formatting card…");
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
        | Layout::Case => format_normal_layout(card),
        Layout::Split | Layout::Flip | Layout::Adventure => {
            format_single_image_multiple_faces_layout(card)
        }
        Layout::Transform
        | Layout::ModalDfc
        | Layout::ReversibleCard
        | Layout::DoubleFacedToken
        | Layout::ArtSeries => format_multiple_faces_layout(card),
        _ => Err(Error::UnknownCardLayout {
            layout: card.layout,
        }),
    }
}

pub fn get_artist(card: &Card) -> Result<Option<String>> {
    match card.layout.clone() {
        Layout::Transform
        | Layout::ModalDfc
        | Layout::ReversibleCard
        | Layout::DoubleFacedToken
        | Layout::ArtSeries => {
            let faces = card.card_faces.clone().unwrap();
            let mut builder = Builder::default();
            artist(&mut builder, &CardOrFace::Face(&faces[0]));
            return builder
                .string()
                .map(|string| Some(string))
                .map_err(|_| Error::TextNotFound);
        }
        _ => Ok(None),
    }
}

fn format_normal_layout(card: &Card) -> Result<Vec<String>> {
    let mut builder = Builder::default();

    let type_line = card.type_line.clone().unwrap();

    if type_line.contains("Creature") {
        format_creature(&mut builder, &CardOrFace::Card(card));
        artist(&mut builder, &CardOrFace::Card(card));
        return builder
            .string()
            .map(|str| vec![str])
            .map_err(|_| Error::TextNotFound);
    }

    if type_line.contains("Planeswalker") {
        format_planeswalker(&mut builder, &CardOrFace::Card(card));
        artist(&mut builder, &CardOrFace::Card(card));
        return builder
            .string()
            .map(|str| vec![str])
            .map_err(|_| Error::TextNotFound);
    }

    if type_line.contains("Vanguard") {
        format_vanguard(&mut builder, &CardOrFace::Card(card));
    }

    if type_line.contains("Instant")
        || type_line.contains("Sorcery")
        || type_line.contains("Artifact")
        || type_line.contains("Enchantment")
        || type_line.contains("Land")
        || type_line.contains("Phenomenon")
        || type_line.contains("Plane")
        || type_line.contains("Scheme")
        || type_line.contains("Emblem")
        || type_line.contains("Battle")
    {
        format_non_creature(&mut builder, &CardOrFace::Card(card));
    }

    if type_line == "Token" {
        format_token(&mut builder, &CardOrFace::Card(card));
    }

    artist(&mut builder, &CardOrFace::Card(card));
    return builder
        .string()
        .map(|str| vec![str])
        .map_err(|_| Error::TextNotFound);
}

fn format_multiple_faces_layout(card: &Card) -> Result<Vec<String>> {
    let faces = card.card_faces.clone().unwrap();
    faces
        .iter()
        .map(|face| {
            let mut builder = Builder::default();
            let type_line = face.type_line.clone().unwrap();

            if type_line.contains("Creature") {
                format_creature(&mut builder, &CardOrFace::Face(&face));
                return builder.string().map_err(|_| Error::TextNotFound);
            }

            if type_line.contains("Planeswalker") {
                format_planeswalker(&mut builder, &CardOrFace::Face(&face));
            }

            if type_line.contains("Instant")
                || type_line.contains("Sorcery")
                || type_line.contains("Artifact")
                || type_line.contains("Enchantment")
                || type_line.contains("Land")
                || type_line.contains("Emblem")
                || type_line.contains("Battle")
            {
                format_non_creature(&mut builder, &CardOrFace::Face(&face));
            }

            if type_line == "Token" {
                format_token(&mut builder, &CardOrFace::Face(&face));
            }

            if type_line == "Card" {
                format_art_card(&mut builder, &CardOrFace::Face(&face));
            }

            return builder.string().map_err(|_| Error::TextNotFound);
        })
        .collect()
}

fn format_single_image_multiple_faces_layout(card: &Card) -> Result<Vec<String>> {
    let faces = format_multiple_faces_layout(card)?;
    let mut builder = Builder::default();

    builder.append(format!("{}", faces.join("\n\n")));

    artist(&mut builder, &CardOrFace::Card(&card));
    return builder
        .string()
        .map(|str| vec![str])
        .map_err(|_| Error::TextNotFound);
}

fn format_creature(builder: &mut Builder, card_or_face: &CardOrFace) {
    name_and_mana_cost(builder, card_or_face);
    type_line(builder, card_or_face);
    oracle_text(builder, card_or_face);
    flavour_text(builder, card_or_face);
    power_and_toughness(builder, card_or_face);
}

fn format_non_creature(builder: &mut Builder, card_or_face: &CardOrFace) {
    name_and_mana_cost(builder, card_or_face);
    type_line(builder, card_or_face);
    oracle_text(builder, card_or_face);
    flavour_text(builder, card_or_face);
}

fn format_planeswalker(builder: &mut Builder, card_or_face: &CardOrFace) {
    name_and_mana_cost(builder, card_or_face);
    type_line(builder, card_or_face);
    oracle_text(builder, card_or_face);
    loyalty(builder, card_or_face);
}

fn format_token(builder: &mut Builder, card_or_face: &CardOrFace) {
    name_and_mana_cost(builder, card_or_face);
    type_line(builder, card_or_face);
}

fn format_vanguard(builder: &mut Builder, card_or_face: &CardOrFace) {
    name_and_mana_cost(builder, card_or_face);
    type_line(builder, card_or_face);
    oracle_text(builder, card_or_face);
    vanguard_stats(builder, card_or_face);
    flavour_text(builder, card_or_face);
}

fn format_art_card(builder: &mut Builder, card_or_face: &CardOrFace) {
    name_and_mana_cost(builder, card_or_face);
    type_line(builder, card_or_face);
}

fn name_and_mana_cost(builder: &mut Builder, card_or_face: &CardOrFace) {
    let name: String;
    let mana_cost: String;
    match card_or_face {
        &CardOrFace::Card(card) => {
            name = card.name.clone();
            mana_cost = card.mana_cost.clone().unwrap_or_default();
        }
        &CardOrFace::Face(face) => {
            name = face.name.clone();
            mana_cost = face.mana_cost.clone();
        }
    }
    builder.append(format!("{}", name));
    if !mana_cost.is_empty() {
        builder.append(format!("\t{}", mana_cost));
    }
}

fn type_line(builder: &mut Builder, card_or_face: &CardOrFace) {
    let type_line: String;
    match card_or_face {
        &CardOrFace::Card(card) => {
            type_line = card.type_line.clone().unwrap_or_default();
        }
        &CardOrFace::Face(face) => {
            type_line = face.type_line.clone().unwrap_or_default();
        }
    }
    builder.append(format!("\n{}", type_line));
}

fn oracle_text(builder: &mut Builder, card_or_face: &CardOrFace) {
    let oracle_text: String;
    match card_or_face {
        &CardOrFace::Card(card) => {
            oracle_text = card.oracle_text.clone().unwrap_or_default();
        }
        &CardOrFace::Face(face) => {
            oracle_text = face.oracle_text.clone().unwrap_or_default();
        }
    }
    if !oracle_text.is_empty() {
        builder.append(format!("\n{}", oracle_text));
    }
}

fn flavour_text(builder: &mut Builder, card_or_face: &CardOrFace) {
    let flavour_text: Option<String>;
    match card_or_face {
        &CardOrFace::Card(card) => {
            flavour_text = card.flavor_text.clone();
        }
        &CardOrFace::Face(face) => {
            flavour_text = face.flavor_text.clone();
        }
    }
    if flavour_text.is_some() {
        builder.append(format!("\n\n{}", flavour_text.unwrap()));
    }
}

fn power_and_toughness(builder: &mut Builder, card_or_face: &CardOrFace) {
    let power: String;
    let toughness: String;
    match card_or_face {
        &CardOrFace::Card(card) => {
            power = card.power.clone().unwrap_or_default();
            toughness = card.toughness.clone().unwrap_or_default();
        }
        &CardOrFace::Face(face) => {
            power = face.power.clone().unwrap_or_default();
            toughness = face.toughness.clone().unwrap_or_default();
        }
    }
    builder.append(format!("\n\n{}/{}", power, toughness));
}

fn artist(builder: &mut Builder, card_or_face: &CardOrFace) {
    let artist: Option<String>;
    match card_or_face {
        &CardOrFace::Card(card) => {
            artist = card.artist.clone();
        }
        &CardOrFace::Face(face) => {
            artist = face.artist.clone();
        }
    }
    if artist.is_some() {
        builder.append(format!("\n\nIllustrated by {}", artist.unwrap()));
    }
}

fn loyalty(builder: &mut Builder, card_or_face: &CardOrFace) {
    let loyalty: Option<String>;
    match card_or_face {
        &CardOrFace::Card(card) => {
            loyalty = card.loyalty.clone();
        }
        &CardOrFace::Face(face) => {
            loyalty = face.loyalty.clone();
        }
    }
    if loyalty.is_some() {
        builder.append(format!("\nLoyalty: {}", loyalty.unwrap()));
    }
}

fn vanguard_stats(builder: &mut Builder, card_or_face: &CardOrFace) {
    let hand_modifier: Option<String>;
    let life_modifier: Option<String>;
    match card_or_face {
        &CardOrFace::Card(card) => {
            hand_modifier = card.hand_modifier.clone();
            life_modifier = card.life_modifier.clone();
        }
        &CardOrFace::Face(_) => {
            // There are no multifaced vanguard cards
            return;
        }
    }
    if hand_modifier.is_some() && life_modifier.is_some() {
        builder.append(format!(
            "\n\nHand Size: {}\nStarting Life: {}",
            hand_modifier.unwrap(),
            life_modifier.unwrap()
        ));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_format_card_grizzly_bears() {
        let expected_string = "Grizzly Bears\t{1}{G}\n\
        Creature — Bear\n\
        \n\
        Don't try to outrun one of Dominia's Grizzlies; it'll catch you, knock you down, and eat you. Of course, you could run up a tree. In that case you'll get a nice view before it knocks the tree down and eats you.\n\
        \n\
        2/2\n\
        \n\
        Illustrated by Jeff A. Menges".to_owned();
        let grizzly_bears = Card::multiverse(155).await.unwrap();
        assert_eq!(format_card(&grizzly_bears).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&grizzly_bears).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_brainstorm() {
        let expected_string = "Brainstorm\t{U}\n\
        Instant\n\
        Draw three cards, then put two cards from your hand on top of your library in any order.\n\
        \n\
        \"I reeled from the blow, and then suddenly, I knew exactly what to do. Within moments, victory was mine.\"\n\
        —Gustha Ebbasdotter,\n\
        Kjeldoran Royal Mage\n\
        \n\
        Illustrated by Christopher Rush".to_owned();
        let brainstorm = Card::multiverse(2497).await.unwrap();
        assert_eq!(format_card(&brainstorm).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&brainstorm).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_fireball() {
        let expected_string = "Fireball\t{X}{R}\n\
        Sorcery\n\
        This spell costs {1} more to cast for each target beyond the first.\nFireball deals X damage divided evenly, rounded down, among any number of targets.\n\
        \n\
        Illustrated by Mark Tedin".to_owned();
        let fireball = Card::multiverse(197).await.unwrap();
        assert_eq!(format_card(&fireball).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&fireball).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_black_lotus() {
        let expected_string = "Black Lotus\t{0}\n\
        Artifact\n\
        {T}, Sacrifice Black Lotus: Add three mana of any one color.\n\
        \n\
        Illustrated by Christopher Rush"
            .to_owned();
        let black_lotus = Card::multiverse(3).await.unwrap();
        assert_eq!(format_card(&black_lotus).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&black_lotus).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_fastbond() {
        let expected_string = "Fastbond\t{G}\n\
        Enchantment\n\
        You may play any number of lands on each of your turns.\nWhenever you play a land, if it wasn't the first land you played this turn, Fastbond deals 1 damage to you.\n\
        \n\
        Illustrated by Mark Poole".to_owned();
        let fastbond = Card::multiverse(148).await.unwrap();
        assert_eq!(format_card(&fastbond).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&fastbond).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_ajani() {
        let expected_string = "Ajani Goldmane\t{2}{W}{W}\n\
        Legendary Planeswalker — Ajani\n\
        +1: You gain 2 life.\n−1: Put a +1/+1 counter on each creature you control. Those creatures gain vigilance until end of turn.\n−6: Create a white Avatar creature token. It has \"This creature's power and toughness are each equal to your life total.\"\n\
        Loyalty: 4\n\
        \n\
        Illustrated by Aleksi Briclot".to_owned();
        let ajani = Card::multiverse(140233).await.unwrap();
        assert_eq!(format_card(&ajani).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&ajani).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_badlands() {
        let expected_string = "Badlands\n\
        Land — Swamp Mountain\n\
        ({T}: Add {B} or {R}.)\n\
        \n\
        Illustrated by Rob Alexander"
            .to_owned();
        let badlands: Card = Card::multiverse(279).await.unwrap();
        assert_eq!(format_card(&badlands).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&badlands).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_stand_and_deliver() {
        let expected_string = "Stand\t{W}\n\
        Instant\n\
        Prevent the next 2 damage that would be dealt to target creature this turn.\n\
        \n\
        Deliver\t{2}{U}\n\
        Instant\n\
        Return target permanent to its owner's hand.\n\
        \n\
        Illustrated by David Martin"
            .to_owned();
        let stand_and_deliver: Card = Card::multiverse(20573).await.unwrap();
        assert_eq!(format_card(&stand_and_deliver).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&stand_and_deliver).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_alive_and_well() {
        let expected_string = "Alive\t{3}{G}\n\
        Sorcery\n\
        Create a 3/3 green Centaur creature token.\n\
        Fuse (You may cast one or both halves of this card from your hand.)\n\
        \n\
        Well\t{W}\n\
        Sorcery\n\
        You gain 2 life for each creature you control.\n\
        Fuse (You may cast one or both halves of this card from your hand.)\n\
        \n\
        Illustrated by Nils Hamm"
            .to_owned();
        let alive_and_well: Card = Card::multiverse(369041).await.unwrap();
        assert_eq!(format_card(&alive_and_well).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&alive_and_well).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_crime_and_punishment() {
        let expected_string = "Crime\t{3}{W}{B}\n\
        Sorcery\n\
        Put target creature or enchantment card from an opponent's graveyard onto the battlefield under your control.\n\
        \n\
        Punishment\t{X}{B}{G}\n\
        Sorcery\n\
        Destroy each artifact, creature, and enchantment with mana value X.\n\
        \n\
        Illustrated by Randy Gallegos"
            .to_owned();
        let crime_and_punishment: Card = Card::multiverse(107285).await.unwrap();
        assert_eq!(
            format_card(&crime_and_punishment).unwrap()[0],
            expected_string
        );
        assert_eq!(None, get_artist(&crime_and_punishment).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_discovery_and_dispersal() {
        let expected_string = "Discovery\t{1}{U/B}\n\
        Sorcery\n\
        Surveil 2, then draw a card. (To surveil 2, look at the top two cards of your library, then put any number of them into your graveyard and the rest on top of your library in any order.)\n\
        \n\
        Dispersal\t{3}{U}{B}\n\
        Instant\n\
        Each opponent returns a nonland permanent they control with the highest mana value among permanents they control to its owner's hand, then discards a card.\n\
        \n\
        Illustrated by Mark Behm"
            .to_owned();
        let discovery_and_dispersal: Card = Card::multiverse(452973).await.unwrap();
        assert_eq!(
            format_card(&discovery_and_dispersal).unwrap()[0],
            expected_string
        );
        assert_eq!(None, get_artist(&discovery_and_dispersal).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_start_to_finish() {
        let expected_string = "Start\t{2}{W}\n\
        Instant\n\
        Create two 1/1 white Warrior creature tokens with vigilance.\n\
        \n\
        Finish\t{2}{B}\n\
        Sorcery\n\
        Aftermath (Cast this spell only from your graveyard. Then exile it.)\n\
        As an additional cost to cast Finish, sacrifice a creature.\n\
        Destroy target creature.\n\
        \n\
        Illustrated by Magali Villeneuve"
            .to_owned();
        let start_to_finish: Card = Card::multiverse(426917).await.unwrap();
        assert_eq!(format_card(&start_to_finish).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&start_to_finish).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_rever_to_return() {
        let expected_string = "Never\t{1}{B}{B}\n\
        Sorcery\n\
        Destroy target creature or planeswalker.\n\
        \n\
        Return\t{3}{B}\n\
        Sorcery\n\
        Aftermath (Cast this spell only from your graveyard. Then exile it.)\n\
        Exile target card from a graveyard. Create a 2/2 black Zombie creature token.\n\
        \n\
        Illustrated by Daarken"
            .to_owned();
        let rever_to_return: Card = Card::multiverse(426914).await.unwrap();
        assert_eq!(format_card(&rever_to_return).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&rever_to_return).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_bushi_tenderfoot() {
        let expected_string = "Bushi Tenderfoot\t{W}\n\
        Creature — Human Soldier\n\
        When a creature dealt damage by Bushi Tenderfoot this turn dies, flip Bushi Tenderfoot.\n\
        \n\
        1/1\n\
        \n\
        Kenzo the Hardhearted\n\
        Legendary Creature — Human Samurai\n\
        Double strike; bushido 2 (Whenever this creature blocks or becomes blocked, it gets +2/+2 until end of turn.)\n\
        \n\
        3/4\n\
        \n\
        Illustrated by Mark Zug"
            .to_owned();
        let bushi_tenderfoot: Card = Card::multiverse(78600).await.unwrap();
        assert_eq!(format_card(&bushi_tenderfoot).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&bushi_tenderfoot).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_rune_tail() {
        let expected_string = "Rune-Tail, Kitsune Ascendant\t{2}{W}\n\
        Legendary Creature — Fox Monk\n\
        When you have 30 or more life, flip Rune-Tail, Kitsune Ascendant.\n\
        \n\
        2/2\n\
        \n\
        Rune-Tail's Essence\n\
        Legendary Enchantment\n\
        Prevent all damage that would be dealt to creatures you control.\n\
        \n\
        Illustrated by Randy Gallegos"
            .to_owned();
        let rune_tail: Card = Card::multiverse(87600).await.unwrap();
        assert_eq!(format_card(&rune_tail).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&rune_tail).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_kytheon() {
        let face1 = "Kytheon, Hero of Akros\t{W}\n\
        Legendary Creature — Human Soldier\n\
        At end of combat, if Kytheon, Hero of Akros and at least two other creatures attacked this combat, exile Kytheon, then return him to the battlefield transformed under his owner's control.\n\
        {2}{W}: Kytheon gains indestructible until end of turn.\n\
        \n\
        2/1"
            .to_owned();
        let face2 = "Gideon, Battle-Forged\n\
        Legendary Planeswalker — Gideon\n\
        +2: Up to one target creature an opponent controls attacks Gideon, Battle-Forged during its controller's next turn if able.\n\
        +1: Until your next turn, target creature gains indestructible. Untap that creature.\n\
        0: Until end of turn, Gideon, Battle-Forged becomes a 4/4 Human Soldier creature with indestructible that's still a planeswalker. Prevent all damage that would be dealt to him this turn.\n\
        Loyalty: 3".to_owned();
        let kytheon: Card = Card::multiverse(398428).await.unwrap();
        let result = format_card(&kytheon).unwrap();
        assert_eq!(result[0], face1);
        assert_eq!(result[1], face2);
        assert_eq!(
            Some("\n\nIllustrated by Willian Murai"),
            get_artist(&kytheon).unwrap().as_deref()
        );
    }

    #[tokio::test]
    async fn test_format_card_emerias_call() {
        let face1 = "Emeria's Call\t{4}{W}{W}{W}\n\
        Sorcery\n\
        Create two 4/4 white Angel Warrior creature tokens with flying. Non-Angel creatures you control gain indestructible until your next turn.\n\
        \n\
        \"Iona no longer guards this place. We do.\"\n\
        —Kasla, Emeria shepherd"
            .to_owned();
        let face2 = "Emeria, Shattered Skyclave\n\
        Land\n\
        As Emeria, Shattered Skyclave enters the battlefield, you may pay 3 life. If you don't, it enters the battlefield tapped.\n\
        {T}: Add {W}.\n\
        \n\
        \"You called it the castle of a god. It is less than that, and so much more.\"\n\
        —Kasla, Emeria shepherd"
            .to_owned();
        let emerias_call: Card = Card::multiverse(491633).await.unwrap();
        let result = format_card(&emerias_call).unwrap();
        assert_eq!(result[0], face1);
        assert_eq!(result[1], face2);
        assert_eq!(
            Some("\n\nIllustrated by Matt Stewart"),
            get_artist(&emerias_call).unwrap().as_deref()
        );
    }

    #[tokio::test]
    async fn test_format_card_gisela() {
        let expected_string = "Gisela, the Broken Blade\t{2}{W}{W}\n\
        Legendary Creature — Angel Horror\n\
        Flying, first strike, lifelink\n\
        At the beginning of your end step, if you both own and control Gisela, the Broken Blade and a creature named Bruna, the Fading Light, exile them, then meld them into Brisela, Voice of Nightmares.\n\
        \n\
        She now hears only Emrakul's murmurs.\n\
        \n\
        4/3\n\
        \n\
        Illustrated by Clint Cearley".to_owned();
        let gisela: Card = Card::multiverse(414319).await.unwrap();
        assert_eq!(format_card(&gisela).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&gisela).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_artificer_class() {
        let expected_string = "Artificer Class\t{1}{U}\n\
        Enchantment — Class\n\
        (Gain the next level as a sorcery to add its ability.)\n\
        The first artifact spell you cast each turn costs {1} less to cast.\n\
        {1}{U}: Level 2\n\
        When this Class becomes level 2, reveal cards from the top of your library until you reveal an artifact card. Put that card into your hand and the rest on the bottom of your library in a random order.\n\
        {5}{U}: Level 3\n\
        At the beginning of your end step, create a token that's a copy of target artifact you control.\n\
        \n\
        Illustrated by Jim Nelson".to_owned();
        let artificer_class: Card = Card::multiverse(567228).await.unwrap();
        assert_eq!(format_card(&artificer_class).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&artificer_class).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_history_of_benalia() {
        let expected_string = "History of Benalia\t{1}{W}{W}\n\
        Enchantment — Saga\n\
        (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)\n\
        I, II — Create a 2/2 white Knight creature token with vigilance.\n\
        III — Knights you control get +2/+1 until end of turn.\n\
        \n\
        Illustrated by Noah Bradley"
            .to_owned();
        let history_of_benalia: Card = Card::multiverse(442909).await.unwrap();
        assert_eq!(
            format_card(&history_of_benalia).unwrap()[0],
            expected_string
        );
        assert_eq!(None, get_artist(&history_of_benalia).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_brazen_borrower() {
        let expected_string = "Brazen Borrower\t{1}{U}{U}\n\
        Creature — Faerie Rogue\n\
        Flash\n\
        Flying\n\
        Brazen Borrower can block only creatures with flying.\n\
        \n\
        3/1\n\
        \n\
        Petty Theft\t{1}{U}\n\
        Instant — Adventure\n\
        Return target nonland permanent an opponent controls to its owner's hand.\n\
        \n\
        Illustrated by Eric Deschamps"
            .to_owned();
        let brazen_borrower: Card = Card::multiverse(473001).await.unwrap();
        assert_eq!(format_card(&brazen_borrower).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&brazen_borrower).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_arcane_proxy() {
        let expected_string = "Arcane Proxy\t{7}\n\
        Artifact Creature — Wizard\n\
        Prototype {1}{U}{U} — 2/1 (You may cast this spell with different mana cost, color, and size. It keeps its abilities and types.)\n\
        When Arcane Proxy enters the battlefield, if you cast it, exile target instant or sorcery card with mana value less than or equal to Arcane Proxy's power from your graveyard. Copy that card. You may cast the copy without paying its mana cost.\n\
        \n\
        4/3\n\
        \n\
        Illustrated by Kekai Kotaki"
            .to_owned();
        let arcane_proxy: Card = Card::multiverse(583660).await.unwrap();
        assert_eq!(format_card(&arcane_proxy).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&arcane_proxy).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_adorable_kitten() {
        let expected_string = "Adorable Kitten\t{W}\n\
        Host Creature — Cat\n\
        When this creature enters the battlefield, roll a six-sided die. You gain life equal to the result.\n\
        \n\
        1/1\n\
        \n\
        Illustrated by Andrea Radeck"
            .to_owned();
        let adorable_kitten: Card = Card::multiverse(479485).await.unwrap();
        assert_eq!(format_card(&adorable_kitten).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&adorable_kitten).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_half_kitten_half() {
        let expected_string = "Half-Kitten, Half-\n\
        Creature — Cat\n\
        Whenever you're dealt damage,\n\
        Augment {2}{W} ({2}{W}, Reveal this card from your hand: Combine it with target host. Augment only as a sorcery.)\n\
        \n\
        +1/+2\n\
        \n\
        Illustrated by Andrea Radeck"
            .to_owned();
        let half_kitten_half: Card = Card::multiverse(439398).await.unwrap();
        assert_eq!(format_card(&half_kitten_half).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&half_kitten_half).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_sheep() {
        let expected_string = "Sheep\n\
        Token Creature — Sheep\n\
        \n\
        2/2\n\
        \n\
        Illustrated by Kev Walker"
            .to_owned();
        let sheep = Card::scryfall_id("281d2c14-2343-44c9-a589-7f4da37978a2".parse().unwrap())
            .await
            .unwrap();
        assert_eq!(format_card(&sheep).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&sheep).unwrap());
    }

    #[tokio::test]
    async fn test_format_card_ajani_reversable() {
        let face1 = "Ajani Goldmane\t{2}{W}{W}\n\
        Legendary Planeswalker — Ajani\n\
        +1: You gain 2 life.\n−1: Put a +1/+1 counter on each creature you control. Those creatures gain vigilance until end of turn.\n−6: Create a white Avatar creature token. It has \"This creature's power and toughness are each equal to your life total.\"\n\
        Loyalty: 4"
            .to_owned();
        let face2 = "Ajani Goldmane\t{2}{W}{W}\n\
        Legendary Planeswalker — Ajani\n\
        +1: You gain 2 life.\n−1: Put a +1/+1 counter on each creature you control. Those creatures gain vigilance until end of turn.\n−6: Create a white Avatar creature token. It has \"This creature's power and toughness are each equal to your life total.\"\n\
        Loyalty: 4"
            .to_owned();
        let ajani_reversable =
            Card::scryfall_id("9cd6a16f-1eff-4624-8f7f-4d9e70a694bb".parse().unwrap())
                .await
                .unwrap();
        let result = format_card(&ajani_reversable).unwrap();
        assert_eq!(result[0], face1);
        assert_eq!(result[1], face2);
        assert_eq!(
            Some("\n\nIllustrated by Fay Dalton"),
            get_artist(&ajani_reversable).unwrap().as_deref()
        );
    }

    #[tokio::test]
    async fn test_format_card_angel_angel() {
        let face1 = "Angel\n\
        Token Creature — Angel\n\
        Flying\n\
        \n\
        4/4"
        .to_owned();
        let face2 = "Angel\n\
        Token"
            .to_owned();
        let angel_angel =
            Card::scryfall_id("e2235007-b02e-463b-95e1-a8bea74a0f9d".parse().unwrap())
                .await
                .unwrap();
        let result = format_card(&angel_angel).unwrap();
        assert_eq!(result[0], face1);
        assert_eq!(result[1], face2);
        assert_eq!(
            Some("\n\nIllustrated by Magali Villeneuve"),
            get_artist(&angel_angel).unwrap().as_deref()
        );
    }

    #[tokio::test]
    async fn test_format_sorin_emblem() {
        let expected_string = "Sorin, Lord of Innistrad Emblem\n\
        Emblem — Sorin\n\
        Creatures you control get +1/+0.\n\
        \n\
        Illustrated by Michael Komarck"
            .to_owned();
        let sorin_emblem =
            Card::scryfall_id("327ddaaf-b6a7-4c80-9b38-5ab68181b3d6".parse().unwrap())
                .await
                .unwrap();
        assert_eq!(format_card(&sorin_emblem).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&sorin_emblem).unwrap());
    }

    #[tokio::test]
    async fn test_format_interplanar_tunnel() {
        let expected_string = "Interplanar Tunnel\n\
        Phenomenon\n\
        When you encounter Interplanar Tunnel, reveal cards from the top of your planar deck until you reveal five plane cards. Put a plane card from among them on top of your planar deck, then put the rest of the revealed cards on the bottom in a random order. (Then planeswalk away from this phenomenon.)\n\
        \n\
        Illustrated by Chuck Lukacs".to_owned();
        let interplanar_tunnel =
            Card::scryfall_id("56e4874c-9d3d-4a1c-a027-186a33ce0da7".parse().unwrap())
                .await
                .unwrap();
        assert_eq!(
            format_card(&interplanar_tunnel).unwrap()[0],
            expected_string
        );
        assert_eq!(None, get_artist(&interplanar_tunnel).unwrap());
    }

    #[tokio::test]
    async fn test_format_academy_at_tolaria_west() {
        let expected_string = "Academy at Tolaria West\n\
        Plane — Dominaria\n\
        At the beginning of your end step, if you have no cards in hand, draw seven cards.\n\
        Whenever chaos ensues, discard your hand.\n\
        \n\
        Illustrated by James Paick"
            .to_owned();
        let academy_at_tolaria_west =
            Card::scryfall_id("ed4f4210-9871-4cec-9b46-100c80f93cd4".parse().unwrap())
                .await
                .unwrap();
        assert_eq!(
            format_card(&academy_at_tolaria_west).unwrap()[0],
            expected_string
        );
        assert_eq!(None, get_artist(&academy_at_tolaria_west).unwrap());
    }

    #[tokio::test]
    async fn test_format_ertai() {
        let expected_string = "Ertai\n\
        Vanguard\n\
        Creatures you control have hexproof. (They can't be the targets of spells or abilities your opponents control.)\n\
        \n\
        Hand Size: -1\n\
        Starting Life: +4\n\
        \n\
        After serving his apprenticeship under Barrin of Tolaria, Ertai graced the *Weatherlight* crew with his presence as the ship's \"resident\" wizard. He realizes that few recognize his greatness—but then how could they, when they lack his insight and wisdom?\n\
        \n\
        Illustrated by Randy Gallegos"
            .to_owned();
        let ertai = Card::scryfall_id("5cbb9b5d-9199-4a5b-957d-8fa681caeb7c".parse().unwrap())
            .await
            .unwrap();
        assert_eq!(format_card(&ertai).unwrap()[0], expected_string);
        assert_eq!(None, get_artist(&ertai).unwrap());
    }

    #[tokio::test]
    async fn test_format_chillerpillar_art_card() {
        let face1 = "Chillerpillar\n\
        Card"
            .to_owned();
        let face2 = "Chillerpillar\n\
        Card"
            .to_owned();
        let chillerpillar_art_card =
            Card::scryfall_id("8de2ff37-fdb7-4f77-9d48-e99afac9a79e".parse().unwrap())
                .await
                .unwrap();
        let result = format_card(&chillerpillar_art_card).unwrap();
        assert_eq!(result[0], face1);
        assert_eq!(result[1], face2);
        assert_eq!(
            Some("\n\nIllustrated by Suzanne Helmigh"),
            get_artist(&chillerpillar_art_card).unwrap().as_deref()
        );
    }

    #[tokio::test]
    async fn test_format_invasion_of_fiora() {
        let face1 = "Invasion of Fiora\t{4}{B}{B}\n\
        Battle — Siege\n\
        (As a Siege enters, choose an opponent to protect it. You and others can attack it. When it's defeated, exile it, then cast it transformed.)\n\
        When Invasion of Fiora enters the battlefield, choose one or both —\n\
        • Destroy all legendary creatures.\n\
        • Destroy all nonlegendary creatures."
            .to_owned();
        let face2 = "Marchesa, Resolute Monarch\n\
        Legendary Creature — Human Noble\n\
        Menace, deathtouch\n\
        Whenever Marchesa, Resolute Monarch attacks, remove all counters from up to one target permanent.\n\
        At the beginning of your upkeep, if you haven't been dealt combat damage since your last turn, you draw a card and you lose 1 life.\n\
        \n\
        3/6"
            .to_owned();
        let invasion_of_fiora =
            Card::scryfall_id("b3af679b-6ee6-4a1d-8ec3-b659bdd90b4a".parse().unwrap())
                .await
                .unwrap();
        let result = format_card(&invasion_of_fiora).unwrap();
        println!("{:#?}", result);
        assert_eq!(result[0], face1);
        assert_eq!(result[1], face2);
        assert_eq!(
            Some("\n\nIllustrated by Joshua Raphael"),
            get_artist(&invasion_of_fiora).unwrap().as_deref()
        );
    }

    #[tokio::test]
    async fn test_format_case_of_the_filched_falcon() {
        let expected_string = "Case of the Filched Falcon\t{U}\n\
        Enchantment — Case\n\
        When this Case enters the battlefield, investigate. (Create a Clue token. It's an artifact with \"{2}, Sacrifice this artifact: Draw a card.\")\n\
        To solve — You control three or more artifacts. (If unsolved, solve at the beginning of your end step.)\n\
        Solved — {2}{U}, Sacrifice this Case: Put four +1/+1 counters on target noncreature artifact. It becomes a 0/0 Bird creature with flying in addition to its other types.\n\
        \n\
        Illustrated by Julia Metzger".to_owned();
        let case_of_the_filched_falcon =
            Card::scryfall_id("266be5bd-71ba-4511-8b71-d0b03885a28d".parse().unwrap())
                .await
                .unwrap();
        assert_eq!(
            format_card(&case_of_the_filched_falcon).unwrap()[0],
            expected_string
        );
        assert_eq!(None, get_artist(&case_of_the_filched_falcon).unwrap());
    }
}
