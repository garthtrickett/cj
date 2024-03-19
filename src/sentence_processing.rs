// sentence_processing.rs
use crate::ichiran_extraction::Token;

#[derive(Clone, Debug)]
pub struct WordRule {
    jap: String,
}

#[derive(Clone, Debug)]
pub struct PosWordRule {
    pos: Vec<String>,
    jap: String,
}

#[derive(Clone, Debug)]
pub struct PosWordPosRule {
    pos1: Vec<String>,
    jap: String,
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
            jap: "は".to_string(),
        },
        WordRule {
            jap: "これ".to_string(),
        },
        WordRule {
            jap: "それ".to_string(),
        },
        WordRule {
            jap: "あれ".to_string(),
        },
        WordRule {
            jap: "いい".to_string(),
        },
        WordRule {
            jap: "よくない".to_string(),
        },
        WordRule {
            jap: "よかった".to_string(),
        },
        WordRule {
            jap: "よくなかった".to_string(),
        },
        WordRule {
            jap: "いいです".to_string(),
        },
        WordRule {
            jap: "よくないです".to_string(),
        },
        WordRule {
            jap: "よかったです".to_string(),
        },
        WordRule {
            jap: "よくなかったです".to_string(),
        },
        WordRule {
            jap: "か".to_string(),
        },
        WordRule {
            jap: "が".to_string(), // I'm not sure if its possible to figure out wether が is used as but or subject without dependancy parsing or LLM.
        },
        WordRule {
            jap: "よ".to_string(),
        },
        WordRule {
            jap: "ね".to_string(),
        },
        WordRule {
            jap: "を".to_string(),
        },
        WordRule {
            jap: "ここ".to_string(),
        },
        WordRule {
            jap: "そこ".to_string(),
        },
        WordRule {
            jap: "あそこ".to_string(),
        },
        WordRule {
            jap: "でしょう".to_string(),
        },
        WordRule {
            jap: "だろう".to_string(),
        },
        WordRule {
            jap: "だろ".to_string(),
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
            jap: "です".to_string(),
        },
        PosWordRule {
            pos: vec!["n".to_string(), "pn".to_string(), "adj-na".to_string()],
            jap: "だ".to_string(),
        },
        PosWordRule {
            pos: vec!["n".to_string(), "pn".to_string()],
            jap: "も".to_string(),
        },
        PosWordRule {
            pos: vec!["adj-na".to_string()],
            jap: "な".to_string(),
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
        PosRule {
            pos: vec!["adj-i".to_string()], // godan verbs
        },
    ];

    let pos_word_pos_rules = vec![PosWordPosRule {
        pos1: vec!["n".to_string(), "pn".to_string()],
        jap: "の".to_string(),
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
                if rule.jap == word.jap {
                    matched_rules.push((*rule).clone());
                }
            }
        }
    }

    matched_rules
}

fn match_pos_word_rules(tokens: &[Token], pos_word_rules: &Vec<PosWordRule>) -> Vec<PosWordRule> {
    let mut matched_rules = Vec::new();

    for i in 0..tokens.len() {
        let (pos, word) = match &tokens[i] {
            Token::Word(word) => (&word.pos, &word.jap),
            Token::Conjugation(conj) => (&conj.pos, &conj.jap),
            _ => continue,
        };

        if i + 1 < tokens.len() {
            let next_word = match &tokens[i + 1] {
                Token::Word(word) => &word.jap,
                Token::Conjugation(conj) => &conj.jap,
                _ => continue,
            };

            for rule in pos_word_rules {
                if rule.pos.contains(pos) && rule.jap == *next_word {
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
                        && rule.jap == word2.jap
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
