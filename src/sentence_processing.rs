// sentence_processing.rs
use regex::Regex;

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
pub struct Conjugation {
    pos: String,
    word: String,
    conjugation_type: String,
}

#[derive(Debug)]
pub struct Word {
    word: String,
    pos: String,
}

#[derive(Debug)]
pub struct CompoundWord {
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
                    word: base_form,
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
                    word: base_form,
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
                            word.word.clone()
                        } else {
                            "".to_string()
                        }
                    };
                    let conjugation = Conjugation {
                        pos,
                        word: base_form,
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
                let word = if parts[2].starts_with('<') {
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
                let word = Word { word, pos };
                result.push(Token::Word(word));
            }
        }
    }

    Ok(result)
}

#[derive(Clone, Debug)]
pub struct WordRule {
    word: String,
}

#[derive(Clone, Debug)]
pub struct PosWordRule {
    pos: Vec<String>,
    word: String,
}

#[derive(Clone, Debug)]
pub struct PosWordPosRule {
    pos1: Vec<String>,
    word: String,
    pos2: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PosRule {
    pos: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct RuleSet {
    pos_word_rules: Vec<PosWordRule>,
    pos_rules: Vec<PosRule>,
    word_rules: Vec<WordRule>,
    pos_word_pos_rules: Vec<PosWordPosRule>,
}

pub fn load_rules() -> RuleSet {
    let word_rules = vec![
        WordRule {
            word: "は".to_string(),
        },
        WordRule {
            word: "これ".to_string(),
        },
        WordRule {
            word: "それ".to_string(),
        },
        WordRule {
            word: "あれ".to_string(),
        },
        WordRule {
            word: "いい".to_string(),
        },
        WordRule {
            word: "よくない".to_string(),
        },
        WordRule {
            word: "よかった".to_string(),
        },
        WordRule {
            word: "よくなかった".to_string(),
        },
        WordRule {
            word: "いいです".to_string(),
        },
        WordRule {
            word: "よくないです".to_string(),
        },
        WordRule {
            word: "よかったです".to_string(),
        },
        WordRule {
            word: "よくなかったです".to_string(),
        },
        WordRule {
            word: "か".to_string(),
        },
    ];

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

    let pos_word_pos_rules = vec![PosWordPosRule {
        pos1: vec!["n".to_string(), "pn".to_string()],
        word: "の".to_string(),
        pos2: vec!["n".to_string(), "pn".to_string()],
    }];

    RuleSet {
        pos_word_rules,
        pos_rules,
        word_rules,
        pos_word_pos_rules,
    }
}

fn match_word_rules(tokens: &Vec<Token>, word_rules: &Vec<WordRule>) -> Vec<WordRule> {
    let mut matched_rules = Vec::new();

    for token in tokens {
        if let Token::Word(word) = token {
            for rule in word_rules {
                if rule.word == word.word {
                    matched_rules.push((*rule).clone());
                }
            }
        }
    }

    matched_rules
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

    matched_rules
}

fn match_pos_word_pos_rules(
    tokens: &[Token],
    pos_word_pos_rules: &Vec<PosWordPosRule>,
) -> Vec<PosWordPosRule> {
    let mut matched_rules = Vec::new();

    for i in 0..tokens.len() {
        if i + 2 < tokens.len() {
            if let (Token::Word(word1), Token::Word(word2), Token::Word(word3)) =
                (&tokens[i], &tokens[i + 1], &tokens[i + 2])
            {
                for rule in pos_word_pos_rules {
                    if rule.pos1.contains(&word1.pos)
                        && rule.word == word2.word
                        && rule.pos2.contains(&word3.pos)
                    {
                        matched_rules.push((*rule).clone());
                    }
                }
            }
        }
    }

    matched_rules
}

pub fn match_rules(tokens: Vec<Token>, rules: RuleSet) -> RuleSet {
    let matched_pos_word_rules = match_pos_word_rules(&tokens, &rules.pos_word_rules);
    let matched_pos_rules = match_pos_rules(&tokens, &rules.pos_rules);
    let matched_word_rules = match_word_rules(&tokens, &rules.word_rules);
    let matched_pos_word_pos_rules = match_pos_word_pos_rules(&tokens, &rules.pos_word_pos_rules);

    RuleSet {
        pos_word_rules: matched_pos_word_rules,
        pos_rules: matched_pos_rules,
        word_rules: matched_word_rules,
        pos_word_pos_rules: matched_pos_word_pos_rules,
    }
}
