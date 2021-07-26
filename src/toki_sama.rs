use std::str::FromStr;
use radix_trie::{Trie, TrieCommon};

use crate::pu::Pu;
use crate::TokiPonaWord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundWord {
    // Almost all words won't be longer than 4 
    pub toki_pona : smallvec::SmallVec::<[TokiPonaWord; 4]>,
}


impl CompoundWord {
    fn dist(&self, other : &Self) -> u32 {
        //let mut unique : smallvec::SmallVec::<[TokiPonaWord;8]> = smallvec::SmallVec::new();

        //for x in &self.toki_pona {
            //unique.push(*x);
        //}

        let mut dist = self.toki_pona.len() + other.toki_pona.len();

        for y in &other.toki_pona {
            if (self.toki_pona.contains(y)) {
                dist -= 2;
            }
        }

        dist as u32
    }
}
/*
impl vpsearch::MetricSpace for CompoundWord {
    type UserData = ();
    type Distance = u32;

    fn distance(&self, other : &Self, _: &Self::UserData) -> Self::Distance {
        let (min, max) = if self.toki_pona.len() > other.toki_pona.len() {
            (other, self)
        }
        else {
            (self, other)
        };

        let mut dist = max.toki_pona.len();

        for x in &min.toki_pona {
            for y in &max.toki_pona {
                if (x == y) {
                    dist -= 1;
                    break;
                }
            }
        }

        dist as u32
    }
}
*/



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    weight : u32,
    toki_pona : CompoundWord,
    english : String,
}

impl Translation {
    pub fn try_parse(line : &str, pu : &Pu) -> Option<Vec<Self>> {
        if (line.is_empty() || line.starts_with("#")) {
            return Some(Vec::new());
        }

        let (toki_pona, english_definitions_array) = line.split_once(":")?;

        let toki_pona_words : Vec<&str> = toki_pona.split_whitespace().collect();
        let mut compound_word_parts = smallvec::SmallVec::new();
        for word in toki_pona_words {
            match (pu.lookup(word)) {
                Some(w) => compound_word_parts.push(w),
                _ => {
                    println!("Unknown toki pona word {}", word);
                    return None;
                },
            }
        }

        let compound_word = CompoundWord {
            toki_pona : compound_word_parts,
        };

        let start = english_definitions_array.find('[')?;
        let end = english_definitions_array.find(']')?;
        let english_definitions : &str = &english_definitions_array[start + 1..end];
        let def_splits : Vec<&str> = english_definitions.split(',').collect();

        let mut defs = Vec::new();

        for def_split in def_splits {
            let trimmed = def_split.trim();
            let (english, weight_str) = trimmed.rsplit_once(" ")?;
            let weight = u32::from_str(weight_str).ok()?;
            defs.push(Self {
                weight,
                english : english.to_owned(),
                toki_pona : compound_word.clone(),
            })
        }

        Some(defs)
    }
}

#[derive(Debug)]
pub struct Dictionary {
    pub entries : Vec<Translation>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            entries : Vec::new(),
        }
    }
    pub fn merge_with(&mut self, other : Self) {
        self.entries.extend(other.entries.into_iter());
    }
}

pub struct TokiSama {
    dictionary : Dictionary,
    posting_lists : Vec<Vec<usize>>,
    trie : Trie<String, usize>,
    //petal_neighbors::BallTree<'_, >
    //vp_tree : vpsearch::Tree<CompoundWord>,
}

impl TokiSama {
    pub fn new(mut dictionary : Dictionary) -> Self {
        dictionary.entries.sort_by(|x, y| x.weight.cmp(&y.weight));

        let mut trie = Trie::new();
        let mut posting_lists = Vec::new();

        let mut compound_words = Vec::new();

        for entry_rank in 0..dictionary.entries.len() {
            let entry = &dictionary.entries[entry_rank];
            let value_rank = trie.get(&entry.english).cloned().unwrap_or_else(|| {
                let new_value_rank = posting_lists.len();
                trie.insert(entry.english.clone(), new_value_rank);
                posting_lists.push(Vec::with_capacity(1));
                new_value_rank
            });

            posting_lists.get_mut(value_rank).unwrap().push(entry_rank);
            compound_words.push(entry.toki_pona.clone());
        }

        //let vp_tree = vpsearch::Tree::new(&compound_words);
        //let ball_tree = petal_neighbors::BallTree::euclidean(&compound_words);

        TokiSama {
            dictionary,
            posting_lists,
            trie,
            //ball_tree,
        }
    }

    fn populate_completion(&self, search_string : &str, entry_rank : usize, pu : &Pu) -> Completion {
        let entry = &self.dictionary.entries[entry_rank];

        let mut similar = Vec::new();

        // DUMB impl
        const MAX : usize = 5;
        for i in 0..self.dictionary.entries.len() {
            if (i == entry_rank) {
                continue;
            }

            let e = &self.dictionary.entries[i];
            let dist = e.toki_pona.dist(&entry.toki_pona);
            if (dist < 2) {
                similar.push(ThesaurusResult {
                    english : e.english.clone(),
                    toki_pona: e.toki_pona.clone(),
                    toki_pona_string : pu.get_string(&e.toki_pona),
                    dist,
                });

                if (similar.len() >= MAX) {
                    break;
                }
            }
        }

        Completion {
            english_search : search_string.to_owned(),
            entry_english : entry.english.to_owned(),
            original_translation : entry.toki_pona.clone(),
            original_translation_string : pu.get_string(&entry.toki_pona),
            weight : entry.weight,
            similar,
        }
    }

    pub fn lookup(&self, prefix : &str, pu : &Pu) -> Vec<Completion> {
        let mut completions = Vec::new();
        let m_sub_trie = self.trie.subtrie(prefix);

        if (m_sub_trie.is_none()) {
            return completions
        }

        let sub_trie = m_sub_trie.unwrap();

        const MAX : usize = 25;

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

#[derive(Debug)]
pub struct ThesaurusResult {
    english : String,
    toki_pona : CompoundWord,
    toki_pona_string : String,
    dist : u32,
}

#[derive(Debug)]
pub struct Completion {
    english_search : String,
    entry_english : String,
    original_translation : CompoundWord,
    original_translation_string : String,
    weight : u32,
    similar : Vec<ThesaurusResult>,
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