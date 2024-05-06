// ichiran_extract.rs
// For Cecil Hiring Manager - Japanese language learning tool that extracts unique words from ichi.moe tokenizer
use regex::Regex;

#[derive(Debug)]
pub struct Conjugation {
    pub pos: String,
    pub jap: String,
    pub conjugation_type: String,
}

#[derive(Debug)]
pub struct Word {
    pub jap: String,
    pub pos: String,
}

#[derive(Debug)]
pub struct CompoundWord {
    pub first: Conjugation,
    pub second: Conjugation,
}

#[derive(Debug)]
pub enum Token {
    Word(Word),
    Conjugation(Conjugation),
    CompoundWord(CompoundWord),
}

pub fn process_lines(lines: Vec<&str>) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];
        if line.contains("Compound word") {
            if i + 3 < lines.len() && lines[i + 3].contains("Conjugation") {
                let re = Regex::new(r"\[ Conjugation: \[(.*?)\] (.*)").unwrap();
                let captures = re.captures(lines[i + 3]).unwrap();
                let pos = captures[1].to_string();
                let conjugation_type = captures[2].to_string();
                let next_line = lines[i + 4];
                let re_base = Regex::new(r"  (.*?) 【").unwrap();
                let captures = re_base.captures(next_line).unwrap();
                let base_form = captures[1].to_string();
                let first = Conjugation {
                    pos,
                    jap: base_form,
                    conjugation_type,
                };
                let captures = re.captures(lines[i + 6]).unwrap();
                let pos = captures[1].to_string();
                let conjugation_type = captures[2].to_string();
                let next_line = lines[i + 7];
                let captures = re_base.captures(next_line).unwrap();
                let base_form = captures[1].to_string();
                let second = Conjugation {
                    pos,
                    jap: base_form,
                    conjugation_type,
                };
                let compound_word = CompoundWord { first, second };
                result.push(Token::CompoundWord(compound_word));
            }
        } else if line.contains("Conjugation") {
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
                    let conjugation = Conjugation {
                        pos,
                        jap: base_form,
                        conjugation_type,
                    };
                    if let Some(Token::Word(_)) = result.last() {
                        if !conjugation.conjugation_type.contains("Conjunctive (~te)") {
                            result.pop();
                        }
                    }
                    result.push(Token::Conjugation(conjugation));
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
                let word = Word { jap, pos };
                result.push(Token::Word(word));
            }
        }
    }

    Ok(result)
}
