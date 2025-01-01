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

/*
Chương trình này sẽ làm gì:

- Đọc nội dung từ từ điển
- Loop qua TỪNG TỪ MỘT, mỗi lần Loop thì:
    0. Đọc được Text kiểu gì? -> Use Regex
    1. Chạy generate phần câu ví dụ cho từng từ một
    2. create_individual_glossary để convert thành một Glossary Object
    3. Chuyển đổi dạng thành JSON
    4. ghi vào Writer tương ứng

*/

pub fn add_glossaries_to_api(file_name: &Path, sentence_content: &Vec<String>) -> anyhow::Result<()> {
    let file_handler = File::open(file_name).expect("Couldn't read the dictionary file");
    let reader = BufReader::new(file_handler);

    for line in reader.lines() {
        let line = line?; 
        match extract_term_definition(&line) {
            Some((word, definition)) => {
                let sentences = utils::find_example_sentences(word, &sentence_content)?;
                let current_glossary = create_individual_glossary(word, definition, sentences);
                utils::write_to_corresponding_files(current_glossary)?;
            },
            None => {
                println!("[ERROR] Couldn't process the current term and definition. Skipped.");
            },
        };
    }

    Ok(())
}