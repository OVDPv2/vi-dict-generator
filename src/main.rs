use std::{env, fs::File, io::{BufRead, BufReader}, path::Path};
use anyhow::{Context, Ok};
use glossary::add_glossaries_to_api;

mod utils;
mod glossary;


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let dictionary_file = Path::new(&args[1]);
    let dataset = Path::new(&args[2]);
    
    println!("Vietnamese Dictionary API Generator");
    println!("Processing {}, using dataset: {}", dictionary_file.to_str().unwrap(), dataset.to_str().unwrap());
    let file_handler =
        File::open(dataset).context("Couldn't read the dataset")?;
    let reader = BufReader::new(file_handler);

    let lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .collect();
    
    add_glossaries_to_api(dictionary_file, &lines)?;
    Ok(())
}
