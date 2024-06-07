// ichiran_extract.rs
// For Cecil Hiring Manager - Japanese language learning tool that extracts unique words from ichi.moe tokenizer
use regex::Regex;

#[derive(Debug)]
pub struct Conjugation {
    pub pos: String,
    pub jap: String,
    pub conjugation_type: String,
    pub meanings: Vec<String>,
}

#[derive(Debug)]
pub struct Word {
    pub jap: String,
    pub pos: String,
    pub meanings: Vec<String>,
}

#[derive(Debug)]
pub enum Token {
    Word(Word),
    Conjugation(Conjugation),
}

// TODO: next is to get all the meanings for words, conjugations and first word in compound word and put them in an array
pub fn process_lines(lines: Vec<&str>) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    let mut is_compound_word = false;

    let mut current_meanings: Vec<String> = vec![];

    let mut conjugation_within_word = false;

    for i in 0..lines.len() {
        let line = lines[i];
        println!("{:?}\n", line);
        if line.contains("Compound word") {
            is_compound_word = true;
            // Parse the compound word...
            let re_word = Regex::new(r"\* (.*?) ").unwrap();
            let captures_word = re_word.captures(lines[i + 1]).unwrap();
            let jap_word = captures_word[1].to_string();

            let word = Word {
                jap: jap_word,        // Japanese word extracted from line
                pos: "n".to_string(), // Replace with actual part of speech
                meanings: vec![],
            };

            result.push(Token::Word(word));
        } else if line.contains("Conjugation") && !conjugation_within_word {
            if is_compound_word {
                is_compound_word = false;
                continue;
            }
            let re = Regex::new(r"\[ Conjugation: \[(.*?)\] (.*)").unwrap();
            if let Some(captures) = re.captures(line) {
                let pos = captures[1].to_string();
                let conjugation_type = captures[2].to_string();
                if i + 1 < lines.len() {
                    let next_line = lines[i + 1];
                    let re_base = Regex::new(r"  (.*?) 【").unwrap();
                    let base_form = if let Some(captures) = re_base.captures(next_line) {
                        captures[1].to_string()
                    } else {
                        // If the base form is not found in the next line, use the word itself as the base form
                        if let Some(Token::Word(word)) = result.last() {
                            word.jap.clone()
                        } else {
                            "".to_string()
                        }
                    };

                    // START: Dealing with meanings vector
                    // Start from the next line and continue until an empty line or a line containing "Conjugation" is hit
                    let mut j = i + 1;
                    while j < lines.len()
                        && !lines[j].trim().is_empty()
                        && !lines[j].contains("Conjugation")
                    {
                        // This is a meaning line, add it to the current meanings
                        current_meanings.push(lines[j].to_string());
                        j += 1;
                    }

                    // END: Dealing with meanings vector
                    let conjugation = Conjugation {
                        pos,
                        jap: base_form,
                        conjugation_type,
                        meanings: current_meanings.clone(),
                    };
                    result.push(Token::Conjugation(conjugation));
                    current_meanings.clear()
                }
            }
        } else if line.starts_with("* ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                let jap = if parts[2].starts_with('<') {
                    parts[3]
                        .trim_matches(|c| c == '【' || c == '】')
                        .to_string()
                } else {
                    parts[2]
                        .trim_matches(|c| c == '【' || c == '】')
                        .to_string()
                };
                let pos = if i + 1 < lines.len() && lines[i + 1].starts_with("1.") {
                    let re = Regex::new(r"\[([a-z0-9,-]+)\]")?;
                    let match_ = re.captures(lines[i + 1]);
                    if let Some(match_) = match_ {
                        match_[1].to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };

                // START: Dealing with meanings vector
                // Start from the next line and continue until an empty line or a line containing "Conjugation" is hit
                let mut j = i + 1;
                conjugation_within_word = true;
                while j < lines.len() && !lines[j].contains("Conjugation") {
                    if lines[j].trim().is_empty() {
                        conjugation_within_word = false; // Set the flag to true if an empty line is hit
                        break; // Break the loop if an empty line is hit
                    }
                    // This is a meaning line, add it to the current meanings
                    current_meanings.push(lines[j].to_string());
                    j += 1;
                }

                // END: Dealing with meanings vector

                let word = Word {
                    jap,
                    pos,
                    meanings: current_meanings.clone(),
                };
                result.push(Token::Word(word));
                current_meanings.clear()
            }
        }
    }

    Ok(result)
}
