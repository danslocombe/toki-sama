use unicase::UniCase;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Translation {
    english : String,
    toki_pona : String,
}

// [dan] hello there -> hello there
fn strip_talker(x : &str) -> &str {
    let talk_def_start = x.find('[');
    let talk_def_end = x.find(']');

    match (talk_def_start, talk_def_end) {
        (Some(start), Some(end)) if start < end => {
            if x[0..start].chars().all(|x| x.is_whitespace()) {
                &x[end+1..]
            }
            else {
                x
            }
        }
        _ => {
            x
        }
    }
}

fn strip_punct_normalize(x : &str) -> String {
    x.chars().map(|c| if c.is_ascii_punctuation() || !c.is_ascii() {
        ' '
    }
    else {
        c.to_ascii_lowercase()
    }).collect()
}

fn normalize_for_def(x : &str) -> String {
    strip_punct_normalize(strip_talker(x))
}

impl Translation {
    fn new(x : &str, y : &str) -> Self {
        Translation {
            english : normalize_for_def(x),
            toki_pona : normalize_for_def(y),
        }
    }
}

#[derive(Debug)]
struct CsvDefinitions {
    definitions : Vec<UniCase<String>>,
}

impl CsvDefinitions {
    fn new(line : &str) -> Self {
        let splits = line.split(',');
        let definitions = splits.map(|x| to_unicase_owned(x)).collect();
        CsvDefinitions {
            definitions,
        }
    }

    fn get_index(&self, s : &str) -> Option<usize> {
        let unicased = UniCase::new(s);
        for i in 0..self.definitions.len() {
            if unicased == self.definitions[i] {
                return Some(i);
            }

        }

        None
    }
}

fn to_unicase_owned(x : &str) -> UniCase<String> {
    UniCase::new(x.trim_matches(|c| c == '"').to_owned())
}

fn to_unicase(x : &str) -> UniCase<&str> {
    UniCase::new(x.trim_matches(|c| c == '"'))
}

fn str_eq(x : &str, y : &str) -> bool {
    to_unicase(x) == to_unicase(y)
}

fn split_csv(line : &str) -> Vec<&str> {
    let mut start = 0;
    let mut splits = Vec::new();
    let mut in_quotes = false;
    let mut escaping = false;
    for (i, c) in line.char_indices() {
        match c {
            ',' if !in_quotes => {
                escaping = false;
                splits.push(&line[start..i]);
                start = i+1;
            },
            '"' => {
                if (!escaping) {
                    in_quotes = !in_quotes;
                }

                escaping = false;
            }
            '\\' => {
                escaping = true
            }
            _ => {
                escaping = false;
            },
        }
    }

    splits.push(&line[start..line.len()]);
    splits

    /*
    let chars = line.char_indices()
    let mut cur = 0;
    let mut prev_quot = None;
    let mut next_quot = line.find('"');
    loop {
        if (next_quot == Some(cur)) {
            next_quot = line[cur..].find('"').map(|x| x + cur);
            cur = next_quot.unwrap();

            prev_quot = next_quot;
            next_quot = line[cur..].find('"').map(|x| x + cur);
        }
    }

    splits
    */
}

fn parse(lines : Vec<String>) -> Option<Vec<Translation>> {

    // Handle BOM, fml
    let mut def_line = lines[0].to_owned();
    def_line = if def_line.starts_with("\u{feff}") {
        def_line[3..].to_owned()
    } else {
        def_line
    };
    let definitions = CsvDefinitions::new(&def_line);

    let translation_idx = definitions.get_index("translation")?;
    let original_idx = definitions.get_index("original")?;
    let toki_pona_idx = definitions.get_index("jan ke tami")?;
    let max_idx = translation_idx.max(original_idx).max(toki_pona_idx);

    let mut predefines = Vec::new();
    let mut translations = Vec::new();
    let mut text_started = false;
    for line in lines.iter().skip(1) {
        let mut splits = split_csv(line);

        if text_started || str_eq(splits[translation_idx], "text start") {
            text_started = true;
            translations.push(Translation::new(splits[original_idx], splits[toki_pona_idx]));
        }
        else if str_eq(splits[translation_idx], "predefine") {
            predefines.push(Translation::new(splits[original_idx], splits[toki_pona_idx]));
        }
    }

    Some(translations)
}

fn corpus_path() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("corpus");
    path
}


fn main() {
    let mut path = corpus_path();
    path.push("beatrixpotter2.csv");
    let file = std::fs::File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines : Vec<String> = buf_reader.lines().map(|x| x.unwrap()).collect();

    let parsed = parse(lines).unwrap();

    for translation in parsed {
        println!("{:?}", translation);
    }
}