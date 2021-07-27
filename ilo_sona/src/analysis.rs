use std::collections::{HashSet, HashMap};

use toki_sama::pu::{Pu, TokiPonaWord};

// "Low value" words for discovering compound nounds
pub const ignore_words : [&str; 5] = [
    "pi",
    //"ona",
    //"mi",
    "li",
    "la",
    "a",
    "e",
];

#[derive(Debug, Clone)]
pub struct CountMap<T> {
    pub map : HashMap<T, u32>,
}

impl<T> CountMap<T> where T : Eq + std::hash::Hash + Clone {
    pub fn new() -> Self {
        CountMap {
            map : HashMap::new(),
        }
    }

    pub fn contains_key(&self, k : &T) -> bool {
        self.map.contains_key(k)
    }

    pub fn get(&self, k : &T) -> u32 {
        self.map.get(k).map(|x| *x).unwrap_or(0)
    }

    pub fn incr(&mut self, k : &T, incr : u32) {
        if self.map.contains_key(k) {
            let value_reference = self.map.get_mut(k).unwrap();
            *value_reference += incr;
        }
        else {
            self.map.insert(k.clone(), incr);
        }
    }

    pub fn incr_other(&mut self, other : &Self) {
        for (k, v) in &other.map {
            self.incr(k, *v);
        }
    }
}

#[derive(Debug, Clone)]
pub struct TranslationBag {
    pub english_words : CountMap<String>,
    pub toki_pona_words : CountMap<TokiPonaWord>,
}

impl TranslationBag {
    pub fn new(english : &str, toki_pona : &str, pu : &Pu) -> Self {

        let mut english_words = crate::analysis::CountMap::new();
        let mut toki_pona_words = crate::analysis::CountMap::new();

        // Todo dont constrcut each time
        let ignore_toki_pona : Vec<_> = ignore_words.iter().flat_map(|x| pu.lookup(x)).collect();

        for word in english.split_whitespace() {
            english_words.incr(&word.to_owned(), 1);
        }

        for toki_str in toki_pona.split_whitespace() {
            if let Some(toki_pona_word) = pu.lookup(toki_str) {
                if (!ignore_toki_pona.contains(&toki_pona_word)) {
                    toki_pona_words.incr(&toki_pona_word, 1);
                }
            }
        }

        Self {
            english_words,
            toki_pona_words,
        }
    }
}

impl TranslationBag {
    fn contains_english(&self, x : &String) -> bool {
        self.english_words.contains_key(x)
    }

    fn contains_toki_pona(&self, toki : &TokiPonaWord) -> bool {
        self.toki_pona_words.contains_key(toki)
    }
}

pub struct Analysis {
    pub bags : Vec<TranslationBag>,
    pub counts : CountMap<TokiPonaWord>,
    pub all_english : HashSet<String>,
}

impl Analysis {
    pub fn new(bags : Vec<TranslationBag>) -> Self {
        let mut counts = CountMap::new();

        let mut all_english = HashSet::new();

        for bag in &bags {
            counts.incr_other(&bag.toki_pona_words);
            for (word, _count) in &bag.english_words.map {
                let _ = all_english.insert(word.to_string());
            }
        }

        println!("Created analysis from {} bags, {} words", bags.len(), all_english.len());

        Analysis {
            bags,
            counts,
            all_english,
        }
    }

    pub fn lookup(&self, english : &str) -> Vec<(TokiPonaWord, f64)> {
        let english_normalized = english.to_ascii_lowercase();

        let mut counts = CountMap::new();

        for bag in &self.bags {
            if (bag.contains_english(&english_normalized)) {
                counts.incr_other(&bag.toki_pona_words);
            }
        }

        let mut res = Vec::new();

        for (k, v) in counts.map {
            let denom = (self.counts.get(&k) + 1) as f64;
            res.push((k, self.bags.len() as f64 * v as f64 / denom));
        }

        res.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

        res
    }
}

