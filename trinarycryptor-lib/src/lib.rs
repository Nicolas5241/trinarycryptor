pub mod tables;

use crate::tables::SubstitutionTable;

pub fn decode_text(text: &str, substitution_table: &SubstitutionTable<&str, char>) -> String {
    let text = unsquish_text(text.trim());
    let words: Vec<Vec<String>> = text
        .split("0")
        .map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect();

    let words_decoded: Vec<String> = words
        .iter()
        .map(|word| {
            word.iter()
                .map(|char| {
                    substitution_table
                        .decoding
                        .get(&squish_text(&char))
                        .unwrap_or(&'?')
                })
                .collect()
        })
        .collect();

    words_decoded.join(" ").into()
}

pub fn encode_text(text: &str, substitution_table: &SubstitutionTable<&str, char>) -> String {
    text.trim()
        .split(" ")
        .map(|word| {
            word.chars()
                .map(|char| *substitution_table.encoding.get(&char).unwrap_or(&"?"))
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("0")
}

#[inline]
pub fn squish_text(text: &str) -> String {
    text.replace("11", "4")
        .replace("22", "5")
        .replace("33", "6")
}

#[inline]
pub fn unsquish_text(text: &str) -> String {
    text.replace("4", "11")
        .replace("5", "22")
        .replace("6", "33")
}
