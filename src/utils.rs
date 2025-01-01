use std::{fs::{File, OpenOptions}, io::{BufWriter, Write}};
use regex::Regex;

use anyhow::Ok;

use crate::glossary::Glossary;

fn filter_character(first_char: &str) -> String {
    // To create corresponding files based on its first characters
    // static CHARACTERS: &[&'static str;22] = &[
    //     "a", "b", "c", "d", "e", "g", "h", "i", "k", "l", "m", 
    //     "n", "o", "p", "q", "r", "s", "t", "u", "v", "x", "y"
    // ];
    static CONSONANTS: &[&'static str;16] = &[
        // b, c, d, đ, g, h, k, l, m, n, p, q, r, s, t, v, x, "y"
        "b", "c", "g", "h", "k", "l", "m", 
        "n", "p", "q", "r", "s", "t", "v", "x", "y"
    ];

    static D_CHAR: &[&'static str;2] = &["d", "đ"];
    
    static A_VOWEL: &[&'static str;18] = &[    
        "a", "à", "á", "ả", "ã", "ạ", 
        "ă", "ằ", "ắ", "ẳ", "ẵ", "ặ", 
        "â", "ầ", "ấ", "ẩ", "ẫ", "ậ"
    ];
    static E_VOWEL: &[&'static str;12] = &[
        "e", "è", "é", "ẻ", "ẽ", "ẹ", 
        "ê", "ề", "ế", "ể", "ễ", "ệ"
    ];
    static I_VOWEL: &[&'static str;6] = &[
        "i", "ì", "í", "ỉ", "ĩ", "ị" 
    ];
    static O_VOWEL: &[&'static str;18] = &[
        "o", "ò", "ó" ,"ỏ", "õ", "ọ", 
        "ô", "ồ", "ố", "ổ", "ỗ", "ộ", 
        "ơ", "ờ", "ớ", "ở", "ỡ", "ợ"
    ];
    static U_VOWEL: &[&'static str;12] = &[
        "u", "ù", "ú", "ủ", "ũ", "ụ", 
        "ư", "ừ", "ứ", "ử", "ữ", "ự"
    ];
    if A_VOWEL.contains(&first_char) {
        String::from("a.json")
    } else if E_VOWEL.contains(&first_char) {
        String::from("e.json")
    } else if I_VOWEL.contains(&first_char) {
        String::from("i.json")
    } else if O_VOWEL.contains(&first_char) {
        String::from("o.json")
    } else if U_VOWEL.contains(&first_char) {
        String::from("u.json")
    } else if D_CHAR.contains(&first_char) {
        String::from("d.json")
    } else if CONSONANTS.contains(&first_char) {
        find_char(&first_char)
    } else {
        String::from("other.json")
    }
}


fn find_char(char: &str) -> String {
    format!("{}.json", char)
}

pub fn find_first_char(word: &str) -> String {
    word.chars().next().unwrap().to_string().to_lowercase()
}

pub fn find_example_sentences(word: &str, content: &Vec<String>) -> anyhow::Result<Vec<String>> {
    let mut lines = Vec::new();

    for line in content {
        if lines.len() > 10 {
            break;
        }

        if line.contains(word) {
            lines.push(line.clone());
        }
    }

    Ok(lines)
}


pub fn write_to_corresponding_files(
    entry: Glossary
) -> anyhow::Result<()> {
    let first_char = find_first_char(&entry.word);
    let file_name = filter_character(&first_char);
    let json_file = create_writable_file(&file_name)?;
    /*
    Write to file - json
    */
    let mut writer = BufWriter::new(json_file);
    serde_json::to_writer(&mut writer, &entry)?;
    writer.flush()?;
    Ok(())
}

pub fn extract_term_definition(text: &str) -> Option<(&str, &str)> {
    let pattern = r"^(.+?)\s:\s*(.*)$";
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text);

    match captures {
        Some(captures) => {
            let term = captures.get(1).unwrap().as_str();
            let definition = captures.get(2).unwrap().as_str();
            Some((term, definition))
        }
        None => None,
    }
}


fn create_writable_file(file_name: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open(file_name)
}


#[cfg(test)]
mod tests {
    use crate::utils::{extract_term_definition, find_first_char};

    use super::filter_character;

    #[test]
    fn test_entry_extract() {
        let sample_entry = extract_term_definition("term : definition");
        assert_eq!(sample_entry, Some(("term", "definition")));
    }
    #[test]
    fn find_first_char_test() {
        let first_char = find_first_char("learning");
        assert_eq!(first_char, String::from("l"));
    }

    #[test]
    fn filter_character_test() {
        let new_filename = filter_character("c");
        assert_eq!(new_filename, String::from("c.json"));
    }
}