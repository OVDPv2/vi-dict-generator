use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use serde::{Deserialize, Serialize};

use crate::utils::{self, extract_term_definition};

#[derive(Serialize, Deserialize)]
pub struct Glossary {
    pub word: String,
    pub definition: String,
    pub sentences: Vec<String>
}

fn create_individual_glossary(
    word: &str, 
    definition: &str, 
    sentences: Vec<String>
) -> Glossary {
    Glossary { 
        word: String::from(word), 
        definition: String::from(definition), 
        sentences 
    }
}


pub fn add_glossaries_to_api(file_name: &Path, sentence_content: &Vec<&str>) -> anyhow::Result<()> {
    let file_handler = File::open(file_name)?;
    let reader = BufReader::new(file_handler);

    let mut glossary_list: Vec<Glossary> = Vec::new();
    
    for line in reader.lines() {
        let line = line?; 
        match extract_term_definition(&line) {
            Some((word, definition)) => {
                let sentences = utils::find_example_sentences(word, sentence_content)?;
                let current_glossary = create_individual_glossary(word, definition, sentences);
                glossary_list.push(current_glossary);
            },
            None => {
                println!("[ERROR] Couldn't process the current term and definition. Skipped.");
            },
        };
    }

    utils::write_to_corresponding_files(glossary_list)?;

    Ok(())
}