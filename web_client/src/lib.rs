use wasm_bindgen::prelude::*;

use toki_sama::{TokiSama, Dictionary, Translation};
use toki_sama::pu::Pu;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn parse_wordset(input : &str, pu : &Pu) -> Dictionary {
    let mut entries = Vec::with_capacity(200);
    for line in input.lines() {
        match Translation::try_parse(&line, pu) {
            Some(translations) => {
                entries.extend(translations);
            }
            _ => {
                log!("Could not parse line: {}", line);
            }
        }
    }

    Dictionary { entries }
}

fn read_model(model : &str, pu : &Pu) -> Dictionary {
    let mut entries = Vec::with_capacity(2000);

    for line in model.lines() {
        match Translation::try_from_model(&line, pu) {
            Some(translations) => {
                entries.extend(translations);
            }
            _ => {
                log!("Could not parse line: {}", line);
            }
        }
    }

    Dictionary { entries }
}

#[wasm_bindgen]
pub struct TokiSamaSearch {
    toki_sama : TokiSama,
    pu : Pu,
}

#[wasm_bindgen]
impl TokiSamaSearch {
    #[wasm_bindgen(constructor)]
    pub fn new(pu_data : &str, nimi_pu_str : &str, compounds_str : &str, model_str : &str) -> Self {
        let pu_lines = pu_data.lines().map(|x| x.to_string()).collect();
        let pu = Pu::from_lines(&pu_lines);

        let mut dict = Dictionary::new();

        log!("Reading nimi pu...");
        let nimi_pu = parse_wordset(nimi_pu_str, &pu);
        dict.merge_with(nimi_pu);

        log!("Reading compounds...");
        let compounds = parse_wordset(compounds_str, &pu);
        dict.merge_with(compounds);

        // TODO improve lookup perf in wasm before enabling this
        // build vantage point tree or smth
        log!("Reading model");
        let model = read_model(model_str, &pu);
        dict.merge_with(model);

        let toki_sama = TokiSama::new(dict);

        TokiSamaSearch {
            toki_sama,
            pu,
        }
    }

    pub fn search(&self, prefix : &str) -> String {
        let results = self.toki_sama.lookup(prefix, &self.pu);
        serde_json::to_string(&results).unwrap()
    }
}