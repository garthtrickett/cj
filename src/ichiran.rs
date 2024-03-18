// ichiran.rs
use regex::Regex;
use std::io::Error;
use std::process::Command;

pub fn run_docker_command(input: &str) -> Result<String, std::io::Error> {
    let output = Command::new("docker")
        .args(["exec", "ichiran-main-1", "ichiran-cli", "-i", input])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.into_owned())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error::new(std::io::ErrorKind::Other, stderr))
    }
}

pub fn ichiran_output_to_bracket_furigana(
    lines: Vec<&str>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let new_list = ichiran_output_to_kanji_hirigana_array(lines)?;
    let result_list = process_kanji_hirigana_into_kanji_with_furigana(new_list)?;
    Ok(result_list)
}

fn ichiran_output_to_kanji_hirigana_array(lines: Vec<&str>) -> Result<Vec<String>, Error> {
    let star_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with('*'))
        .copied()
        .collect();

    let star_lines = remove_compound_words(star_lines);

    let re = Regex::new(r"(【[^】]*】)").unwrap();
    let star_lines: Vec<String> = star_lines
        .iter()
        .map(|s| {
            re.replace_all(s, |caps: &regex::Captures| caps[0].replace(' ', ""))
                .to_string()
        }) // Convert Cow<str> to String
        .collect();

    let mut new_list: Vec<String> = Vec::new();
    for string in &star_lines {
        let split_string: Vec<&str> = string.split(' ').collect(); // Now in wider scope

        if string.contains('【') {
            let index = split_string
                .iter()
                .position(|word| word.contains('【'))
                .unwrap();
            new_list.push(format!(
                "{} {}",
                split_string[index - 1],
                split_string[index]
            ));
        } else {
            new_list.push(split_string.last().unwrap().to_string());
        }
    }

    new_list = new_list
        .iter()
        .map(|item| item.replace('【', "[").replace('】', "]"))
        .collect();

    Ok(new_list)
}

fn process_kanji_hirigana_into_kanji_with_furigana(
    new_list: Vec<String>,
) -> Result<Vec<String>, Error> {
    let result_list: Vec<String> = new_list
        .into_iter()
        .map(|item| add_furigana(&item))
        .collect();
    Ok(result_list)
}

fn remove_compound_words(strings: Vec<&str>) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    for s in strings {
        if s.contains("Compound word") {
            let index = s.find("Compound word").unwrap();
            result.push(s[..index].trim_end());
        } else {
            result.push(s);
        }
    }
    result
}

fn add_furigana(s: &str) -> String {
    if !s.contains('[') || !s.contains(']') {
        return s.to_string();
    }

    let parts: Vec<&str> = s.split('[').collect();
    let outside = parts[0].trim().to_string();
    let mut inside = parts[1].split(']').next().unwrap().trim().to_string();

    let jap_comma_after_brackets = inside.contains('、');
    if jap_comma_after_brackets {
        inside = inside.replace('、', "");
    }

    let outside_chars: Vec<char> = outside.chars().collect();
    let inside_chars: Vec<char> = inside.chars().collect();

    let n = std::cmp::min(outside_chars.len(), inside_chars.len());
    let mut common_start = 0;
    let mut common_end = 0;

    for i in 0..n {
        if outside_chars[i] == inside_chars[i] {
            common_start += 1;
        } else {
            break;
        }
    }

    for i in 0..n {
        if outside_chars[outside_chars.len() - i - 1] == inside_chars[inside_chars.len() - i - 1] {
            common_end += 1;
        } else {
            break;
        }
    }

    let output = if common_start == 0 && common_end == 0 {
        format!(" {}[{}]", outside, inside)
    } else if common_start > 0 {
        format!(
            " {}[{}]",
            outside,
            inside_chars[common_start..].iter().collect::<String>()
        )
    } else {
        format!(
            " {}[{}]{}",
            outside_chars[..outside_chars.len() - common_end]
                .iter()
                .collect::<String>(),
            inside_chars[..inside_chars.len() - common_end]
                .iter()
                .collect::<String>(),
            outside_chars[outside_chars.len() - common_end..]
                .iter()
                .collect::<String>()
        )
    };

    if jap_comma_after_brackets {
        format!("{},", output)
    } else {
        output
    }
}

//... If these are always the same maybe they don't need to go into DB .. //

// Non-past Affirmative Plain: 食べる
// Non-past Affirmative Formal: 食べ
// Non-past Negative Plain: 食べない
// Non-past Negative Formal: たべません
// Non-past Negative Plain: 食べられない
// Past (~ta) Affirmative Formal: 食べました
// Past (~ta) Affirmative Plain: 食べた
// Past (~ta) Negative Plain: 食べなかった
// Past (~ta) Negative Formal: 食べませんでした
// Conjunctive (~te) Affirmative Plain: 食べて
// Volitional Affirmative Plain: 食べよう
// Volitional Affirmative Formal: 食べましょう
// Volitional Negative Plain: 食べまい
// Volitional Formal: でしょう
// Volitional Plain: 食べるだろう
// Causative Affirmative Plain: 食べさせる
// Causative Affirmative Formal: 食べさせます
// Causative Negative Plain: 食べさせない
// Causative Negative Formal: 食べさせません
// Causative-Passive Affirmative Plain:  食べさせられる
// Causative-Passive Negative Plain: 食べさせられない
// Causative-Passive Affirmative Formal: 食べさせられます
// Causative-Passive Negative Formal: 食べさせられません
// Imperative Affirmative Plain: 食べろ
// Provisional (~eba) Affirmative Plain: 食べれば
// Provisional (~eba) Negative Plain: 食べなければ
// Conditional (~tara) Affirmative Plain: 食べたら
// Conditional (~tara) Negative Plain: 食べなかったら
// Passive Affirmative Plain: 食べられる
// Passive Affirmative Formal: 食べられま
// Passive Negative Formal: 食べられません
// Passive Negative Plain: 食べられない
// Potential Affirmative Plain: 食べられる
// Potential Affirmative Formal: 食べられます
// Potential Negative Plain: 食べられない
// Potential Negative Formal: 食べられません

// Verbs that often follow the te form
// いる (iru): Used to indicate an ongoing action (progressive tense), a habitual action, or a state of being.
// くる (kuru): Used to indicate that an action is coming or going to happen.
// いく (iku): Used to indicate that an action is going or moving away.
// みる (miru): Used to indicate trying out an action or seeing what happens.
// おく (oku): Used to indicate doing something in preparation for something.
// ある (aru): Used to indicate that an action has been completed (perfect aspect).
// しまう (shimau): Used to indicate completion of an action, often with a sense of regret.
// くれる (kureru): Used to indicate that someone does something for the speaker.
// あげる (ageru): Used to indicate that the speaker does something for someone else.
// もらう (morau): Used to indicate that the speaker receives the action from someone else.

// ADJECTIVES
// Non-past

// Affirmative Plain: 速い (hayai)
// Affirmative Formal: 速いです (hayai desu)
// Negative Plain: 速くない (hayakunai)
// Negative Formal: 速くないです (hayakunai desu)
// Past

// Affirmative Plain: 速かった (hayakatta)
// Affirmative Formal: 速かったです (hayakatta desu)
// Negative Plain: 速くなかった (hayakunakatta)
// Negative Formal: 速くなかったです (hayakunakatta desu)
// Provisional (~eba)

// Affirmative Plain: 速ければ (hayakereba)
// Negative Plain: 速くなければ (hayakunakereba)
// Conditional (~tara)

// Affirmative Plain: 速かったら (hayakattara)
// Negative Plain: 速くなかったら (hayakunakattara)
// Volitional

// Affirmative Plain: 速かろう (hayakarou)
// Affirmative Formal: 速いでしょう (hayai deshou)
// Negative Plain: 速くなかろう (hayakunakarou)
// Negative Formal: 速くないでしょう (hayakunai deshou)

// nouns/na-adjectives

// Non-past

// Affirmative Plain: 静かだ (shizuka da)
// Affirmative Formal: 静かです (shizuka desu)
// Negative Plain: 静かではない (shizuka dewa nai)
// Negative Formal: 静かではありません (shizuka dewa arimasen)

// // Past

// Affirmative Plain: 静かだった (shizuka datta)
// Affirmative Formal: 静かでした (shizuka deshita)
// Negative Plain: 静かではなかった (shizuka dewa nakatta)
// Negative Formal: 静かではありませんでした (shizuka dewa arimasen deshita)

// // Provisional (~eba)

// Affirmative Plain: 静かならば (shizuka naraba)
// Negative Plain: 静かでなければ (shizuka denakereba)

// // Conditional (~tara)

// Affirmative Plain: 静かだったら (shizuka dattara)
// Negative Plain: 静かではなかったら (shizuka dewa nakattara)

// // Volitional

// Affirmative Plain: 静かだろう (shizuka darou)
// Affirmative Formal: 静かでしょう (shizuka deshou)
// Negative Plain: 静かではなかろう (shizuka dewa nakarou)
// Negative Formal: 静かではないでしょう (shizuka dewa nai deshou)

#[derive(Debug)]
struct Conjugation {
    pos: String,
    base_form: String,
    conjugation_type: String,
}

#[derive(Debug)]
struct Word {
    word: String,
    pos: String,
}

#[derive(Debug)]
struct CompoundWord {
    first: Conjugation,
    second: Conjugation,
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
        println!("Processing line: {}", line);
        if line.contains("Compound word") {
            println!("Line contains 'Compound word'");
            if i + 3 < lines.len() && lines[i + 3].contains("Conjugation") {
                println!("Found 'Conjugation' in line {}", i + 3);
                let re = Regex::new(r"\[ Conjugation: \[(.*?)\] (.*)").unwrap();
                let captures = re.captures(lines[i + 3]).unwrap();
                let pos = captures[1].to_string();
                let conjugation_type = captures[2].to_string();
                let next_line = lines[i + 4];
                let re_base = Regex::new(r"  (.*?) 【").unwrap();
                let captures = re_base.captures(next_line).unwrap();
                let base_form = captures[1].to_string();
                println!(
                    "First conjugation: pos={}, base_form={}, conjugation_type={}",
                    pos, base_form, conjugation_type
                );
                let first = Conjugation {
                    pos,
                    base_form,
                    conjugation_type,
                };
                let captures = re.captures(lines[i + 6]).unwrap();
                let pos = captures[1].to_string();
                let conjugation_type = captures[2].to_string();
                let next_line = lines[i + 7];
                let captures = re_base.captures(next_line).unwrap();
                let base_form = captures[1].to_string();
                println!(
                    "Second conjugation: pos={}, base_form={}, conjugation_type={}",
                    pos, base_form, conjugation_type
                );
                let second = Conjugation {
                    pos,
                    base_form,
                    conjugation_type,
                };
                let compound_word = CompoundWord { first, second };
                result.push(Token::CompoundWord(compound_word));
            }
        } else if line.contains("Conjugation") {
            println!("Line contains 'Conjugation'");
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
                        "".to_string()
                    };
                    println!(
                        "Conjugation: pos={}, base_form={}, conjugation_type={}",
                        pos, base_form, conjugation_type
                    );
                    let conjugation = Conjugation {
                        pos,
                        base_form,
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
            println!("Line starts with '* '");
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                let word = if parts[2].starts_with("<") {
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
                println!("Word: word={}, pos={}", word, pos);
                let word = Word { word, pos };
                result.push(Token::Word(word));
            }
        }
    }

    Ok(result)
}

#[derive(Clone, Debug)]
pub enum Rule {
    PosWord(PosWordRule),
    Pos(PosRule),
}

struct WordRule {
    word: String,
}

#[derive(Clone, Debug)]
pub struct PosWordRule {
    pos: Vec<String>,
    word: String,
}

struct PosWordPosRule {
    pos1: Vec<String>,
    word: String,
    pos2: Vec<String>,
}

#[derive(Clone, Debug)]
struct PosRule {
    pos: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct RuleSet {
    pos_word_rules: Vec<PosWordRule>,
    pos_rules: Vec<PosRule>,
}

pub fn load_rules() -> RuleSet {
    let pos_word_rules = vec![
        PosWordRule {
            pos: vec![
                "n".to_string(),
                "pn".to_string(),
                "adj-i".to_string(),
                "adj-na".to_string(),
            ],
            word: "です".to_string(),
        },
        PosWordRule {
            pos: vec!["n".to_string(), "pn".to_string(), "adj-na".to_string()],
            word: "だ".to_string(),
        },
        PosWordRule {
            pos: vec!["n".to_string(), "pn".to_string()],
            word: "も".to_string(),
        },
        PosWordRule {
            pos: vec!["adj-na".to_string()],
            word: "な".to_string(),
        },
    ];

    let pos_rules = vec![
        PosRule {
            pos: vec!["v1".to_string()], // ichidan verbs
        },
        PosRule {
            pos: vec!["v5r".to_string()], // godan verbs
        },
        PosRule {
            pos: vec!["v5k".to_string()], // godan verbs
        },
    ];

    RuleSet {
        pos_word_rules,
        pos_rules,
    }
}

fn match_pos_word_rules(
    tokens: &Vec<Token>,
    pos_word_rules: &Vec<PosWordRule>,
) -> Vec<PosWordRule> {
    let mut matched_rules = Vec::new();

    for token in tokens {
        if let Token::Word(word) = token {
            for rule in pos_word_rules {
                if rule.pos.contains(&word.pos) && rule.word == word.word {
                    matched_rules.push((*rule).clone());
                }
            }
        }
    }

    matched_rules
}

fn match_pos_rules(tokens: &Vec<Token>, pos_rules: &Vec<PosRule>) -> Vec<PosRule> {
    let mut matched_rules = Vec::new();

    for token in tokens {
        match token {
            Token::Word(word) => {
                for rule in pos_rules {
                    if rule.pos.contains(&word.pos) {
                        matched_rules.push((*rule).clone());
                    }
                }
            }
            Token::Conjugation(conj) => {
                for rule in pos_rules {
                    if rule.pos.contains(&conj.pos) {
                        matched_rules.push((*rule).clone());
                    }
                }
            }
            _ => {}
        }
    }

    if matched_rules.is_empty() {}

    matched_rules
}

pub fn match_rules(tokens: Vec<Token>, rules: RuleSet) -> Vec<Rule> {
    let mut matched_rules = Vec::new();

    let matched_pos_word_rules = match_pos_word_rules(&tokens, &rules.pos_word_rules);
    for rule in matched_pos_word_rules {
        matched_rules.push(Rule::PosWord(rule));
    }

    let matched_pos_rules = match_pos_rules(&tokens, &rules.pos_rules);
    for rule in matched_pos_rules {
        matched_rules.push(Rule::Pos(rule));
    }

    matched_rules
}
