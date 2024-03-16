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

pub fn extract_pos_tags(lines: Vec<&str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut pos_tags = Vec::new();
    let mut conjugation_types = Vec::new();
    let re = Regex::new(r"\[([a-z0-9,-]+)\]")?;
    let re_conj = Regex::new(r"\[ Conjugation: \[.*?\] (.*)")?;

    for i in 0..lines.len() {
        println!("{:?}", lines[i]);
        if lines[i].starts_with("* ") {
            // Check the next line for the part of speech tag
            if i + 1 < lines.len() {
                if lines[i + 1].starts_with("1.") {
                    let match_ = re.captures(lines[i + 1]);
                    if let Some(match_) = match_ {
                        pos_tags.push(match_[1].to_string());
                    }
                }
                // This deals with the conjugation case that spits out '\n \n'
                else if lines[i + 1].is_empty() {
                    let match_ = re.captures(lines[i + 2]);
                    if let Some(match_) = match_ {
                        pos_tags.push(match_[1].to_string());
                    }
                }
            }
        } else if lines[i].starts_with("[ Conjugation") {
            let match_ = re_conj.captures(lines[i]);
            if let Some(match_) = match_ {
                conjugation_types.push(match_[1].to_string());
            }
        }
    }

    println!("POS tags: {:?}", pos_tags); // Print the final list of POS tags
    println!("Conjugation types: {:?}", conjugation_types); // Print the final list of conjugation types
    Ok(pos_tags)
}

pub fn find_grammar_rules(
    kanji_with_furigana_array: Vec<String>,
    parts_of_speech_array: Vec<String>,
    rules: Vec<Vec<String>>,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut rule_matches = vec![];

    for rule in rules {
        if rule.len() == 1 {
            let matches = match_single_element_rule(
                &kanji_with_furigana_array,
                &parts_of_speech_array,
                &rule,
            )?;
            rule_matches.extend(matches.clone());
            println!("Single element rule matches: {:?}", matches);
        } else {
            let matches = match_multi_element_rule(
                &kanji_with_furigana_array,
                &parts_of_speech_array,
                &rule,
            )?;
            rule_matches.extend(matches.clone());
            println!("Multi element rule matches: {:?}", matches);
        }
    }

    println!("Total rule matches: {:?}", rule_matches);

    Ok(rule_matches)
}

fn match_single_element_rule(
    kanji_with_furigana_array: &[String],
    parts_of_speech_array: &[String],
    rule: &[String],
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut matches = vec![];

    for i in 0..kanji_with_furigana_array.len() {
        if kanji_with_furigana_array[i] == rule[0] || parts_of_speech_array[i] == rule[0] {
            matches.push(rule.to_vec());
        }
    }

    println!("Single element rule: {:?}, Matches: {:?}", rule, matches);

    Ok(matches)
}

fn match_multi_element_rule(
    kanji_with_furigana_array: &[String],
    parts_of_speech_array: &[String],
    rule: &[String],
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut rule_matches = vec![];

    println!("{:?}", parts_of_speech_array);

    if rule.len() == 2 {
        let rule_start: Vec<&str> = rule[0].split(',').collect(); // error happens here -> Message:  index out of bounds: the len is 1 but the index is 1
        let rule_end = &rule[1];

        for i in 0..(kanji_with_furigana_array.len() - 1) {
            println!(
                "Index: {}, Array Length: {}",
                i,
                parts_of_speech_array.len()
            );

            let pos_tags_start: Vec<&str> = parts_of_speech_array[i].split(',').collect();

            if kanji_with_furigana_array[i + 1] == *rule_end
                && pos_tags_start.iter().any(|&x| rule_start.contains(&x))
            {
                rule_matches.push(rule.to_vec());
            }
        }
    } else {
        let rule_start: Vec<&str> = rule[0].split(',').collect();
        let rule_end: Vec<&str> = rule[rule.len() - 1].split(',').collect();
        let rule_middle = &rule[1];

        for i in 1..(kanji_with_furigana_array.len() - 1) {
            if kanji_with_furigana_array[i] == *rule_middle {
                let pos_tags_start: Vec<&str> = parts_of_speech_array[i - 1].split(',').collect();
                let pos_tags_end: Vec<&str> = parts_of_speech_array[i + 1].split(',').collect();

                if pos_tags_start.iter().any(|&x| rule_start.contains(&x))
                    && pos_tags_end.iter().any(|&x| rule_end.contains(&x))
                {
                    rule_matches.push(rule.to_vec());
                }
            }
        }
    }

    println!(
        "Multi element rule: {:?}, Matches: {:?}",
        rule, rule_matches
    );

    Ok(rule_matches)
}
