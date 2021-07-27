#![allow(unused_parens)]

mod parsing;
mod analysis;

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use toki_sama::pu::{Pu, TokiPonaWord};

fn corpus_path() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("corpus");
    path
}

fn get_data_path() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("..");
    path.push("data");
    path
}

fn main() {
    let mut pu_path = get_data_path();
    pu_path.push("pu.csv");
    println!("Reading pu from {:?}...", &pu_path);
    let pu = Pu::read(&pu_path);

    let mut bags = Vec::new();

    for subdir in &["beatrix", "pepper"] {
        let mut path = corpus_path();
        path.push(subdir);
        for m_entry in std::fs::read_dir(path).unwrap() {
            let entry_path = m_entry.unwrap().path();
            println!("Reading {:?}", &entry_path);
            let file = std::fs::File::open(&entry_path).unwrap();
            let buf_reader = BufReader::new(file);
            let lines : Vec<String> = buf_reader.lines().map(|x| x.unwrap()).collect();

            if let Some(parsed) = parsing::parse(lines) {
                println!("Parsed {:?}", &entry_path);
                let this_bags : Vec<_> = parsed.iter().map(|x| analysis::TranslationBag::new(&x.english, &x.toki_pona, &pu)).collect();
                bags.extend(this_bags);
            }
        }
    }

    {
        let mut path = corpus_path();
        path.push("sentence_pairs.tsv");
        let file = std::fs::File::open(&path).unwrap();
        let buf_reader = BufReader::new(file);
        let lines : Vec<String> = buf_reader.lines().map(|x| x.unwrap()).collect();
        if let Some(parsed) = parsing::parse_tsv(&lines) {
            println!("Parsed {:?}", &path);
            let this_bags : Vec<_> = parsed.iter().map(|x| analysis::TranslationBag::new(&x.english, &x.toki_pona, &pu)).collect();
            bags.extend(this_bags);
        }
    }

    let analysis = analysis::Analysis::new(bags);

    let mut path = corpus_path();
    path.push("model.tsv");
    let file_out = std::fs::File::create(&path).unwrap();
    let mut writer = BufWriter::new(file_out);

    const WEIGHT_CUTOFF : f64 = 100.0;

    let mut i = 0;
    for word in &analysis.all_english {
        if i % 1000 == 0 {
            println!("writing {}, i = {}", word, i);
        }
        
        i += 1;

        let res = analysis.lookup(&word);
        if (res.len() > 0 && res[0].1 > WEIGHT_CUTOFF) {
            let mut line = String::with_capacity(256);
            line.push_str(&word[..]);

            for (toki_pona, weight) in &res {
                if (*weight > WEIGHT_CUTOFF) {
                    line.push('\t');
                    line.push_str(pu.get(toki_pona));
                    line.push(':');
                    line.push_str(&weight.floor().to_string());
                }
                else {
                    break;
                }
            }
            line.push('\n');
            writer.write(line.as_bytes()).unwrap();
        }
    }

    /*
    println!("\n-- counts --\n");

    for m_line in std::io::stdin().lock().lines() {
        let line = m_line.unwrap();

        if line.is_empty() {
            continue;
        }

        println!("\n");

        let res = analysis.lookup(&line);
        for (word, weight) in res {
            //println!("{:#?}", c);
            println!("{} - {}", pu.get(&word), weight);
        }
        println!("------\n\n");
    }
        */
}