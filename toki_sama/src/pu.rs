use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TokiPonaWord(u8);

#[derive(Debug, Clone)]
struct PuEntry {
    toki_pona: String,
    definition: String,
}

// The standard english <-> toki pona dictioanry
// TODO include ku words
// We use this to get string representations of TokiPonaWords and output full definitinos
pub struct Pu {
    lookup: HashMap<String, TokiPonaWord>,
    definitions: Vec<PuEntry>,
}

impl Pu {
    pub fn from_lines(lines : &Vec<String>) -> Self {
        let mut lookup = HashMap::with_capacity(lines.len());
        let mut definitions = Vec::with_capacity(lines.len());

        // First line is definitions
        let mut cur_word = 0;
        for line in &lines[1..] {
            let splits: Vec<&str> = line.split(',').collect();
            let toki_pona = splits[0].to_owned();
            let _alternative = splits[1];
            let definition = splits[2].to_owned();

            lookup.insert(toki_pona.clone(), TokiPonaWord(cur_word));
            definitions.push(PuEntry {
                toki_pona,
                definition,
            });

            cur_word += 1;
        }

        Pu {
            lookup,
            definitions,
        }
    }

    pub fn read(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        Self::from_lines(&reader.lines().map(|x| x.unwrap()).collect())
    }

    #[cfg(test)]
    pub fn from_subset(defs: &[(&'static str, &'static str)]) -> Self {
        let mut lookup = HashMap::new();
        let mut definitions = Vec::new();
        let mut cur_word = 0;

        for (toki_pona, english) in defs {
            lookup.insert(toki_pona.to_string(), TokiPonaWord(cur_word));
            definitions.push(PuEntry {
                toki_pona: toki_pona.to_string(),
                definition: english.to_string(),
            });

            cur_word += 1;
        }

        Self {
            lookup,
            definitions,
        }
    }

    pub fn get(&self, word: &TokiPonaWord) -> &str {
        &self.definitions[word.0 as usize].toki_pona
    }

    pub fn define(&self, word: &TokiPonaWord) -> &str {
        &self.definitions[word.0 as usize].definition
    }

    pub fn lookup(&self, s: &str) -> Option<TokiPonaWord> {
        self.lookup.get(s).cloned()
    }
}
