use std::{collections::HashMap, fs::read_to_string, path::Path};

use rs_gpt2::encoder::{get_bpe_data, get_encoder};
use serde_json::Value;

fn main() {
    let base_dir = Path::new("/media/nevin/Trash Games1/gpt2/gpt-2/models/124M/");

    let encoder = get_encoder(&base_dir);
    let bpe = get_bpe_data(&base_dir);

    println!("Hello, world!");
}
