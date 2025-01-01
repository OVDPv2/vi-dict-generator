use std::{env, fs, path::Path};
use anyhow::Ok;
use glossary::add_glossaries_to_api;

mod utils;
mod glossary;


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let dictionary_file = Path::new(&args[1]);
    let dataset = Path::new(&args[2]);

    let binding = fs::read_to_string(&dataset)?;
    let lines: Vec<&str> = binding.lines().collect();

    add_glossaries_to_api(dictionary_file, &lines)?;
    Ok(())
}
