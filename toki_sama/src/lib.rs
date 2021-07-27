#![allow(unused_parens)]

pub mod pu;

use radix_trie::{Trie, TrieCommon};
use std::str::FromStr;
use serde::Serialize;

use pu::{Pu, TokiPonaWord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundWord {
    // Almost all words won't be longer than 4
    toki_pona: smallvec::SmallVec<[TokiPonaWord; 4]>,
}

impl CompoundWord {
    fn dist(&self, other: &Self) -> u32 {
        let mut dist = self.toki_pona.len() + other.toki_pona.len();

        for y in &other.toki_pona {
            if (self.toki_pona.contains(y)) {
                dist -= 2;
            }
        }

        dist as u32
    }

    pub fn len(&self) -> usize {
        self.toki_pona.len()
    }

    fn to_string(&self, pu: &Pu) -> String {
        let mut word = String::new();
        for tp in &self.toki_pona {
            if (!word.is_empty()) {
                word.push(' ');
            }

            word.push_str(pu.get(tp));
        }

        word
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    weight: u32,
    toki_pona: CompoundWord,
    english: String,
}

impl Translation {
    pub fn try_parse(line: &str, pu: &Pu) -> Option<Vec<Self>> {
        if (line.is_empty() || line.starts_with("#")) {
            return Some(Vec::new());
        }

        let (toki_pona, english_definitions_array) = line.split_once(":")?;

        let toki_pona_words: Vec<&str> = toki_pona.split_whitespace().collect();
        let mut compound_word_parts = smallvec::SmallVec::new();
        for word in toki_pona_words {
            match (pu.lookup(word)) {
                Some(w) => compound_word_parts.push(w),
                _ => {
                    println!("Unknown toki pona word {}", word);
                    return None;
                }
            }
        }

        let compound_word = CompoundWord {
            toki_pona: compound_word_parts,
        };

        let start = english_definitions_array.find('[')?;
        let end = english_definitions_array.find(']')?;
        let english_definitions: &str = &english_definitions_array[start + 1..end];
        let def_splits: Vec<&str> = english_definitions.split(',').collect();

        let mut defs = Vec::new();

        for def_split in def_splits {
            let trimmed = def_split.trim();
            let (english, weight_str) = trimmed.rsplit_once(" ")?;
            let weight = u32::from_str(weight_str).ok()?;
            defs.push(Self {
                weight,
                english: english.to_owned(),
                toki_pona: compound_word.clone(),
            })
        }

        Some(defs)
    }

    pub fn try_from_model(line : &str, pu : &Pu) -> Option<Vec<Self>> {
        let mut translations = Vec::new();
        let splits : Vec<_> = line.split('\t').collect();
        let english = splits[0];

        let mut weighted_toki_pona = Vec::new();

        for i in 1..splits.len() {
            let (toki, weight_str) = splits[i].split_once(':')?;
            let weight = u32::from_str(weight_str).ok()?;
            let toki_res = pu.lookup(toki)?;
            weighted_toki_pona.push((toki_res, weight));
        }

        // TODO improve this
        // We want to use these weights to forma a compound word
        // for now keep going down adding until weights drop off

        if weighted_toki_pona.is_empty() {
            return Some(translations);
        }

        let initial_weight = weighted_toki_pona[0].1;

        let mut compound : smallvec::SmallVec<[TokiPonaWord; 4]> = smallvec::SmallVec::new();

        for (tp, weight) in weighted_toki_pona {
            if (weight as f64) > ((initial_weight as f64) / 2.25) { 
                compound.push(tp);
            }
            else {
                break;
            }
        }

        translations.push(Translation {
            weight : initial_weight / 10,
            english : english.to_owned(),
            toki_pona : CompoundWord { toki_pona : compound },
        });

        Some(translations)
    }
}

#[derive(Debug)]
pub struct Dictionary {
    pub entries: Vec<Translation>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            entries: Vec::new(),
        }
    }
    pub fn merge_with(&mut self, other: Self) {
        // TODO merge properly
        self.entries.extend(other.entries.into_iter());
    }
}

pub struct TokiSama {
    dictionary: Dictionary,
    posting_lists: Vec<Vec<usize>>,
    trie: Trie<String, usize>,
}

impl TokiSama {
    pub fn new(mut dictionary: Dictionary) -> Self {
        dictionary.entries.sort_by(|x, y| x.weight.cmp(&y.weight));

        let mut trie = Trie::new();
        let mut posting_lists = Vec::new();

        for entry_rank in 0..dictionary.entries.len() {
            let entry = &dictionary.entries[entry_rank];
            let value_rank = trie.get(&entry.english).cloned().unwrap_or_else(|| {
                let new_value_rank = posting_lists.len();
                trie.insert(entry.english.clone(), new_value_rank);
                posting_lists.push(Vec::with_capacity(1));
                new_value_rank
            });

            posting_lists.get_mut(value_rank).unwrap().push(entry_rank);
        }

        for posting_list in posting_lists.iter_mut() {
            posting_list.sort_by(|x, y| {
                dictionary.entries[*y as usize].weight.cmp(&dictionary.entries[*x as usize].weight)
            })
        }

        TokiSama {
            dictionary,
            posting_lists,
            trie,
        }
    }

    fn populate_completion(&self, search_string: &str, entry_rank: usize, pu: &Pu) -> Completion {
        let entry = &self.dictionary.entries[entry_rank];

        let mut similar = Vec::new();

        // DUMB impl
        const MAX: usize = 5;
        let max_dist = entry.toki_pona.len().max(1) as u32;

        for i in 0..self.dictionary.entries.len() {
            if (i == entry_rank) {
                continue;
            }

            let e = &self.dictionary.entries[i];
            let dist = e.toki_pona.dist(&entry.toki_pona);
            if (dist <= max_dist) {
                similar.push(ThesaurusResult {
                    english: e.english.clone(),
                    toki_pona_len : e.toki_pona.len() as u32,
                    toki_pona_string: e.toki_pona.to_string(pu),
                    dist,
                });
            }
        }

        similar.sort_by(|x, y| {
            x.dist.cmp(&y.dist)
                .then(x.toki_pona_len.cmp(&y.toki_pona_len))
        });

        similar = similar.into_iter().take(MAX).collect();

        Completion {
            english_search: search_string.to_owned(),
            entry_english: entry.english.to_owned(),
            entry_weight : entry.weight,
            original_translation_string: entry.toki_pona.to_string(pu),
            similar,
        }
    }

    pub fn lookup(&self, prefix: &str, pu: &Pu) -> Vec<Completion> {
        let mut completions = Vec::new();
        let m_sub_trie = self.trie.get_raw_descendant(prefix);

        if (m_sub_trie.is_none()) {
            return completions;
        }

        let sub_trie = m_sub_trie.unwrap();

        const MAX: usize = 5;

        for (completion, value_rank) in sub_trie.iter() {
            for entry_rank in &self.posting_lists[*value_rank] {
                if (completions.len() >= MAX) {
                    return completions;
                }

                completions.push(self.populate_completion(completion, *entry_rank, pu));
            }
        }

        completions
    }
}

#[derive(Debug, Serialize)]
pub struct ThesaurusResult {
    english: String,
    toki_pona_len : u32,
    toki_pona_string: String,
    dist: u32,
}

#[derive(Debug, Serialize)]
pub struct Completion {
    english_search: String,
    entry_english: String,
    entry_weight : u32,
    original_translation_string: String,
    similar: Vec<ThesaurusResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_translation() {
        let pu = Pu::from_subset(&[("lipu", "paper"), ("moku", "food")]);
        let parsed = Translation::try_parse("lipu moku: [menu 50]", &pu);
        assert!(parsed.is_some());
        assert_eq!("menu", parsed.unwrap()[0].english);
    }
}
