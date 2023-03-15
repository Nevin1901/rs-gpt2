use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde_json::Value;

use crate::util::read_lines;

pub struct Encoder {
    encoder: HashMap<String, u64>,
    bpe_data: Vec<Vec<String>>,
}

impl Encoder {
    pub fn new(base_dir: &Path) -> Self {
        let encoder = get_encoder(&base_dir);
        let bpe_data = get_bpe_data(&base_dir);

        return Self { encoder, bpe_data };
    }
}

pub fn get_encoder(base_dir: &Path) -> HashMap<String, u64> {
    let mut e: HashMap<String, u64> = HashMap::new();

    let encoder_path = Path::join(base_dir, "encoder.json");
    let encoder_text = read_to_string(encoder_path).unwrap();

    let encoder_json: Value = serde_json::from_str(encoder_text.as_str()).unwrap();

    for (key, value) in encoder_json.as_object().unwrap() {
        // value could be u16, I don't think there is a key value about 65k
        e.insert(key.clone(), value.as_u64().unwrap());
    }

    return e;
}

pub fn get_bpe_data(base_dir: &Path) -> Vec<Vec<String>> {
    let mut bpe_vocab = Vec::new();

    let lines = read_lines(Path::join(base_dir, "vocab.bpe")).unwrap();
    for (i, line) in lines.enumerate() {
        if i == 0 {
            continue;
        }

        let l = line.unwrap();

        let split: Vec<String> = l.split(" ").map(|s| s.to_string()).collect();

        bpe_vocab.push(split);
    }

    return bpe_vocab;
}

pub mod test {
    #[test]
    fn test_get_encoder() {
        // idx: 50257
    }

    #[test]
    fn test_get_bpe() {
        // first line (after comment), G t
        // last line, Gg azed
    }
}
