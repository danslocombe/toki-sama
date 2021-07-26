#[allow(unused_parens)]

use std::path::{Path, PathBuf};
use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct PuEntry {
    toki_pona : String,
    definition : String,
}

// The standard english <-> toki pona dictioanry
// TODO include ku words
// We use this to get string representations of TokiPonaWords and output full definitinos
struct Pu {
    lookup : HashMap<String, TokiPonaWord>,
    definitions : Vec<PuEntry>,
}

impl Pu {
    pub fn read() -> Self {
        let mut path = get_data_path();
        path.push("pu.csv");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut lookup = HashMap::with_capacity(150);
        let mut definitions = Vec::with_capacity(150);

        // First line is definitions
        let mut cur_word = 0;
        for read_line in reader.lines().skip(1) {
            let line = read_line.unwrap();
            let splits : Vec<&str> = line.split(',').collect();
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

    pub fn get(&self, word : TokiPonaWord) -> &str {
        &self.definitions[word.0 as usize].toki_pona
    }

    pub fn define(&self, word : TokiPonaWord) -> &str {
        &self.definitions[word.0 as usize].definition
    }

    pub fn lookup(&self, s : &str) -> Option<TokiPonaWord> {
        self.lookup.get(s).map(|x| *x)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct TokiPonaWord(u32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompoundWord {
    // Almost all words won't be longer than 4 
    toki_pona : smallvec::SmallVec::<[TokiPonaWord; 4]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Translation {
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
        let english_definitions : &str = &english_definitions_array[start..end];
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
struct Dictionary {
    entries : Vec<Translation>,
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

fn read_wordset(path : &Path, pu : &Pu) -> Dictionary {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut entries = Vec::with_capacity(200);

    for try_line in reader.lines() {
        let line = try_line.unwrap();
        match Translation::try_parse(&line, pu) {
            Some(translations) => {
                entries.extend(translations);
            },
            _ => {
                println!("Could not parse line: {}", line);
            },
        }
    }

    Dictionary {
        entries,
    }
}

fn read_nimi_pu(pu : &Pu) -> Dictionary {
    let mut path = get_data_path();
    path.push("nimi_pu.txt");
    read_wordset(&path, pu)
}

fn read_compounds(pu : &Pu) -> Dictionary {
    let mut path = get_data_path();
    path.push("compounds.txt");
    read_wordset(&path, pu)
}


fn get_data_path() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("data");
    path
}

pub fn main() {
    println!("Reading pu");
    let pu = Pu::read();

    //for def in pu.definitions {
        //println!("{:?}", def);
    //}

    let mut dict = Dictionary::new(); 

    println!("Reading nimi pu...");
    let nimi_pu = read_nimi_pu(&pu);
    dict.merge_with(nimi_pu);

    println!("Reading compounds...");
    let compounds = read_compounds(&pu);
    dict.merge_with(compounds);


}