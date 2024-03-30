/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

pub enum Additional {
    Text(String),
    Number(usize),
}

pub fn split_text(
    text: String,
    character_limit: usize,
    additional_texts: Vec<Additional>,
) -> Vec<String> {
    let character_already_used = additional_texts
        .into_iter()
        .map(|additional| match additional {
            Additional::Text(text) => text.len(),
            Additional::Number(number) => number,
        })
        .fold(0, |accumulator, number| accumulator + number);
    let number_of_characters = character_limit - character_already_used;

    let mut texts = vec![];
    let mut text_to_split = text.clone();
    while text_to_split.len() > 0 {
        if number_of_characters >= text_to_split.len() {
            texts.push(text_to_split);
            break;
        }
        texts.push(format!(
            "{}{}",
            text_to_split[..(number_of_characters - 1)].to_owned(),
            "…".to_owned()
        ));
        text_to_split = text_to_split[number_of_characters - 1..].to_owned();
    }
    return texts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_text_longer() {
        let text = "0123456789".to_owned();
        let result = split_text(text.clone(), 15, vec![]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], text);
    }

    #[test]
    fn test_limit_text_shorter() {
        let text = "0123456789".to_owned();
        let result = split_text(text.clone(), 5, vec![]);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "0123…");
        assert_eq!(result[1], "4567…");
        assert_eq!(result[2], "89");
    }

    #[test]
    fn test_limit_text_additional_only_text() {
        let text = "0123456789".to_owned();
        let result = split_text(
            text.clone(),
            10,
            vec![Additional::Text("a".into()), Additional::Text("bc".into())],
        );
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "012345…");
        assert_eq!(result[1], "6789");
    }

    #[test]
    fn test_limit_text_additional_only_number() {
        let text = "0123456789".to_owned();
        let result = split_text(
            text.clone(),
            10,
            vec![Additional::Number(4), Additional::Number(3)],
        );
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "01…");
        assert_eq!(result[1], "23…");
        assert_eq!(result[2], "45…");
        assert_eq!(result[3], "67…");
        assert_eq!(result[4], "89");
    }
    #[test]
    fn test_limit_text_additional_mixed() {
        let text = "0123456789".to_owned();
        let result = split_text(
            text.clone(),
            10,
            vec![Additional::Number(4), Additional::Text("a".into())],
        );
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "0123…");
        assert_eq!(result[1], "4567…");
        assert_eq!(result[2], "89");
    }
}
