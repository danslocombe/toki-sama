#![allow(unused_parens)]

mod pu;
mod toki_sama;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use pu::Pu;
use toki_sama::*;

fn read_wordset(path: &Path, pu: &Pu) -> Dictionary {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut entries = Vec::with_capacity(200);

    for try_line in reader.lines() {
        let line = try_line.unwrap();
        match Translation::try_parse(&line, pu) {
            Some(translations) => {
                entries.extend(translations);
            }
            _ => {
                println!("Could not parse line: {}", line);
            }
        }
    }

    Dictionary { entries }
}

fn read_nimi_pu(pu: &Pu) -> Dictionary {
    let mut path = get_data_path();
    path.push("nimi_pu.txt");
    read_wordset(&path, pu)
}

fn read_compounds(pu: &Pu) -> Dictionary {
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
    let mut pu_path = get_data_path();
    pu_path.push("pu.csv");
    println!("Reading pu from {:?}...", &pu_path);
    let pu = Pu::read(&pu_path);

    let mut dict = Dictionary::new();

    println!("Reading nimi pu...");
    let nimi_pu = read_nimi_pu(&pu);
    dict.merge_with(nimi_pu);

    println!("Reading compounds...");
    let compounds = read_compounds(&pu);
    dict.merge_with(compounds);

    let toki_sama = TokiSama::new(dict);

    println!("\n-- toki sama --\n");

    for m_line in std::io::stdin().lock().lines() {
        let line = m_line.unwrap();

        if line.is_empty() {
            continue;
        }

        let res = toki_sama.lookup(&line, &pu);
        for c in res {
            println!("{:#?}", c);
        }

    }
}
