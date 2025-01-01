use regex::Regex;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

use anyhow::Ok;

use crate::glossary::Glossary;

fn filter_character(first_char: &str) -> &str {
    static CONSONANTS: &[&'static str; 16] = &[
        "b", "c", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x", "y",
    ];

    static D_CHAR: &[&'static str; 2] = &["d", "đ"];

    static A_VOWEL: &[&'static str; 18] = &[
        "a", "à", "á", "ả", "ã", "ạ", "ă", "ằ", "ắ", "ẳ", "ẵ", "ặ", "â", "ầ", "ấ", "ẩ", "ẫ", "ậ",
    ];
    static E_VOWEL: &[&'static str; 12] =
        &["e", "è", "é", "ẻ", "ẽ", "ẹ", "ê", "ề", "ế", "ể", "ễ", "ệ"];
    static I_VOWEL: &[&'static str; 6] = &["i", "ì", "í", "ỉ", "ĩ", "ị"];
    static O_VOWEL: &[&'static str; 18] = &[
        "o", "ò", "ó", "ỏ", "õ", "ọ", "ô", "ồ", "ố", "ổ", "ỗ", "ộ", "ơ", "ờ", "ớ", "ở", "ỡ", "ợ",
    ];
    static U_VOWEL: &[&'static str; 12] =
        &["u", "ù", "ú", "ủ", "ũ", "ụ", "ư", "ừ", "ứ", "ử", "ữ", "ự"];
    if A_VOWEL.contains(&first_char) {
        "a"
    } else if E_VOWEL.contains(&first_char) {
        "e"
    } else if I_VOWEL.contains(&first_char) {
        "i"
    } else if O_VOWEL.contains(&first_char) {
        "o"
    } else if U_VOWEL.contains(&first_char) {
        "u"
    } else if D_CHAR.contains(&first_char) {
        "d"
    } else if CONSONANTS.contains(&first_char) {
        first_char
    } else {
        "other"
    }
}

fn json_file_from_char(character: &str) -> String {
    format!("{}.json", character)
}

pub fn find_first_char(word: &str) -> String {
    word.chars().next().unwrap().to_string().to_lowercase()
}

pub fn find_example_sentences(word: &str, content: &Vec<&str>) -> anyhow::Result<Vec<String>> {
    let word_regex = Regex::new(&format!(r"\b{}\b", regex::escape(word)))?;

    let lines: Vec<String> = content
        .iter()
        .filter(|line| word_regex.is_match(line))
        .take(5)
        .copied()
        .map(|f| {
            word_regex
                .replace_all(f, format!("<b>{}</b>", word))
                .to_string()
        })
        .collect();

    Ok(lines)
}

// pub fn write_to_corresponding_files(
//     entry: Glossary
// ) -> anyhow::Result<()> {
//     let first_char = find_first_char(&entry.word);
//     let file_name = json_file_from_char(filter_character(&first_char));
//     let json_file = create_writable_file(&file_name)?;
//     let mut writer = BufWriter::new(json_file);
//     serde_json::to_writer_pretty(&mut writer, &entry)?;
//     writeln!(&mut writer, ",")?;
//     writer.flush()?;
//     Ok(())
// }

pub fn write_to_corresponding_files(entries: Vec<Glossary>) -> anyhow::Result<()> {
    let mut a_word: Vec<Glossary> = Vec::new();
    let mut e_word: Vec<Glossary> = Vec::new();
    let mut i_word: Vec<Glossary> = Vec::new();
    let mut o_word: Vec<Glossary> = Vec::new();
    let mut u_word: Vec<Glossary> = Vec::new();
    let mut b_word: Vec<Glossary> = Vec::new();
    let mut c_word: Vec<Glossary> = Vec::new();
    let mut g_word: Vec<Glossary> = Vec::new();
    let mut h_word: Vec<Glossary> = Vec::new();
    let mut k_word: Vec<Glossary> = Vec::new();
    let mut l_word: Vec<Glossary> = Vec::new();
    let mut m_word: Vec<Glossary> = Vec::new();
    let mut n_word: Vec<Glossary> = Vec::new();
    let mut p_word: Vec<Glossary> = Vec::new();
    let mut q_word: Vec<Glossary> = Vec::new();
    let mut r_word: Vec<Glossary> = Vec::new();
    let mut s_word: Vec<Glossary> = Vec::new();
    let mut t_word: Vec<Glossary> = Vec::new();
    let mut v_word: Vec<Glossary> = Vec::new();
    let mut x_word: Vec<Glossary> = Vec::new();
    let mut y_word: Vec<Glossary> = Vec::new();
    let mut d_word: Vec<Glossary> = Vec::new();
    let mut other_word: Vec<Glossary> = Vec::new();

    for entry in entries {
        match filter_character(find_first_char(&entry.word).as_str()) {
            "a" => {
                a_word.push(entry);
            }
            "e" => {
                e_word.push(entry);
            }
            "i" => {
                i_word.push(entry);
            }
            "o" => {
                o_word.push(entry);
            }
            "u" => {
                u_word.push(entry);
            }
            "d" => {
                d_word.push(entry);
            }
            "b" => {
                b_word.push(entry);
            }
            "c" => {
                c_word.push(entry);
            }
            "g" => {
                g_word.push(entry);
            }
            "h" => {
                h_word.push(entry);
            }
            "k" => {
                k_word.push(entry);
            }
            "l" => {
                l_word.push(entry);
            }
            "m" => {
                m_word.push(entry);
            }
            "n" => {
                n_word.push(entry);
            }
            "p" => {
                p_word.push(entry);
            }
            "q" => {
                q_word.push(entry);
            }
            "r" => {
                r_word.push(entry);
            }
            "s" => {
                s_word.push(entry);
            }
            "t" => {
                t_word.push(entry);
            }
            "v" => {
                v_word.push(entry);
            }
            "x" => {
                x_word.push(entry);
            }
            "y" => {
                y_word.push(entry);
            }
            _ => {
                other_word.push(entry);
            }
        }
    }

    let file_name_a = json_file_from_char("a");
    let json_file_a = create_writable_file(&file_name_a)?;
    let mut writer_a = BufWriter::new(json_file_a);
    serde_json::to_writer_pretty(&mut writer_a, &a_word)?;
    writer_a.flush()?;

    let file_name_e = json_file_from_char("e");
    let json_file_e = create_writable_file(&file_name_e)?;
    let mut writer_e = BufWriter::new(json_file_e);
    serde_json::to_writer_pretty(&mut writer_e, &e_word)?;
    writer_e.flush()?;

    let file_name_i = json_file_from_char("i");
    let json_file_i = create_writable_file(&file_name_i)?;
    let mut writer_i = BufWriter::new(json_file_i);
    serde_json::to_writer_pretty(&mut writer_i, &i_word)?;
    writer_i.flush()?;

    let file_name_o = json_file_from_char("o");
    let json_file_o = create_writable_file(&file_name_o)?;
    let mut writer_o = BufWriter::new(json_file_o);
    serde_json::to_writer_pretty(&mut writer_o, &o_word)?;
    writer_o.flush()?;

    let file_name_u = json_file_from_char("u");
    let json_file_u = create_writable_file(&file_name_u)?;
    let mut writer_u = BufWriter::new(json_file_u);
    serde_json::to_writer_pretty(&mut writer_u, &u_word)?;
    writer_u.flush()?;

    let file_name_d = json_file_from_char("d");
    let json_file_d = create_writable_file(&file_name_d)?;
    let mut writer_d = BufWriter::new(json_file_d);
    serde_json::to_writer_pretty(&mut writer_d, &d_word)?;
    writer_d.flush()?;

    let file_name_b = json_file_from_char("b");
    let json_file_b = create_writable_file(&file_name_b)?;
    let mut writer_b = BufWriter::new(json_file_b);
    serde_json::to_writer_pretty(&mut writer_b, &b_word)?;
    writer_b.flush()?;

    let file_name_c = json_file_from_char("c");
    let json_file_c = create_writable_file(&file_name_c)?;
    let mut writer_c = BufWriter::new(json_file_c);
    serde_json::to_writer_pretty(&mut writer_c, &c_word)?;
    writer_c.flush()?;

    let file_name_g = json_file_from_char("g");
    let json_file_g = create_writable_file(&file_name_g)?;
    let mut writer_g = BufWriter::new(json_file_g);
    serde_json::to_writer_pretty(&mut writer_g, &g_word)?;
    writer_g.flush()?;

    let file_name_h = json_file_from_char("h");
    let json_file_h = create_writable_file(&file_name_h)?;
    let mut writer_h = BufWriter::new(json_file_h);
    serde_json::to_writer_pretty(&mut writer_h, &h_word)?;
    writer_h.flush()?;

    let file_name_k = json_file_from_char("k");
    let json_file_k = create_writable_file(&file_name_k)?;
    let mut writer_k = BufWriter::new(json_file_k);
    serde_json::to_writer_pretty(&mut writer_k, &k_word)?;
    writer_k.flush()?;

    let file_name_l = json_file_from_char("l");
    let json_file_l = create_writable_file(&file_name_l)?;
    let mut writer_l = BufWriter::new(json_file_l);
    serde_json::to_writer_pretty(&mut writer_l, &l_word)?;
    writer_l.flush()?;

    let file_name_m = json_file_from_char("m");
    let json_file_m = create_writable_file(&file_name_m)?;
    let mut writer_m = BufWriter::new(json_file_m);
    serde_json::to_writer_pretty(&mut writer_m, &m_word)?;
    writer_m.flush()?;

    let file_name_n = json_file_from_char("n");
    let json_file_n = create_writable_file(&file_name_n)?;
    let mut writer_n = BufWriter::new(json_file_n);
    serde_json::to_writer_pretty(&mut writer_n, &n_word)?;
    writer_n.flush()?;

    let file_name_p = json_file_from_char("p");
    let json_file_p = create_writable_file(&file_name_p)?;
    let mut writer_p = BufWriter::new(json_file_p);
    serde_json::to_writer_pretty(&mut writer_p, &p_word)?;
    writer_p.flush()?;

    let file_name_q = json_file_from_char("q");
    let json_file_q = create_writable_file(&file_name_q)?;
    let mut writer_q = BufWriter::new(json_file_q);
    serde_json::to_writer_pretty(&mut writer_q, &q_word)?;
    writer_q.flush()?;

    let file_name_r = json_file_from_char("r");
    let json_file_r = create_writable_file(&file_name_r)?;
    let mut writer_r = BufWriter::new(json_file_r);
    serde_json::to_writer_pretty(&mut writer_r, &r_word)?;
    writer_r.flush()?;

    let file_name_s = json_file_from_char("s");
    let json_file_s = create_writable_file(&file_name_s)?;
    let mut writer_s = BufWriter::new(json_file_s);
    serde_json::to_writer_pretty(&mut writer_s, &s_word)?;
    writer_s.flush()?;

    let file_name_t = json_file_from_char("t");
    let json_file_t = create_writable_file(&file_name_t)?;
    let mut writer_t = BufWriter::new(json_file_t);
    serde_json::to_writer_pretty(&mut writer_t, &t_word)?;
    writer_t.flush()?;

    let file_name_v = json_file_from_char("v");
    let json_file_v = create_writable_file(&file_name_v)?;
    let mut writer_v = BufWriter::new(json_file_v);
    serde_json::to_writer_pretty(&mut writer_v, &v_word)?;
    writer_v.flush()?;

    let file_name_x = json_file_from_char("x");
    let json_file_x = create_writable_file(&file_name_x)?;
    let mut writer_x = BufWriter::new(json_file_x);
    serde_json::to_writer_pretty(&mut writer_x, &x_word)?;
    writer_x.flush()?;

    let file_name_y = json_file_from_char("y");
    let json_file_y = create_writable_file(&file_name_y)?;
    let mut writer_y = BufWriter::new(json_file_y);
    serde_json::to_writer_pretty(&mut writer_y, &y_word)?;
    writer_y.flush()?;

    let file_name_other = json_file_from_char("other");
    let json_file_other = create_writable_file(&file_name_other)?;
    let mut writer_other = BufWriter::new(json_file_other);
    serde_json::to_writer_pretty(&mut writer_other, &other_word)?;
    writer_other.flush()?;

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

    use super::{filter_character, find_example_sentences};

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

    #[test]
    fn find_sentences() {
        let result = find_example_sentences(
            "word",
            &vec![
                "word alone can't describe word btw",
                "never mind",
                "wordyyyyy now time",
            ],
        )
        .unwrap();
        let expected = vec![String::from(
            "<b>word</b> alone can't describe <b>word</b> btw",
        )];
        assert_eq!(result, expected);
    }
}
