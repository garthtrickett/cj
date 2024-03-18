// ichiran.rs
// use regex::Regex;
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

// pub fn ichiran_output_to_bracket_furigana(
//     lines: Vec<&str>,
// ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let new_list = ichiran_output_to_kanji_hirigana_array(lines)?;
//     let result_list = process_kanji_hirigana_into_kanji_with_furigana(new_list)?;
//     Ok(result_list)
// }

// fn ichiran_output_to_kanji_hirigana_array(lines: Vec<&str>) -> Result<Vec<String>, Error> {
//     let star_lines: Vec<&str> = lines
//         .iter()
//         .filter(|line| line.starts_with('*'))
//         .copied()
//         .collect();

//     let star_lines = remove_compound_words(star_lines);

//     let re = Regex::new(r"(【[^】]*】)").unwrap();
//     let star_lines: Vec<String> = star_lines
//         .iter()
//         .map(|s| {
//             re.replace_all(s, |caps: &regex::Captures| caps[0].replace(' ', ""))
//                 .to_string()
//         }) // Convert Cow<str> to String
//         .collect();

//     let mut new_list: Vec<String> = Vec::new();
//     for string in &star_lines {
//         let split_string: Vec<&str> = string.split(' ').collect(); // Now in wider scope

//         if string.contains('【') {
//             let index = split_string
//                 .iter()
//                 .position(|word| word.contains('【'))
//                 .unwrap();
//             new_list.push(format!(
//                 "{} {}",
//                 split_string[index - 1],
//                 split_string[index]
//             ));
//         } else {
//             new_list.push(split_string.last().unwrap().to_string());
//         }
//     }

//     new_list = new_list
//         .iter()
//         .map(|item| item.replace('【', "[").replace('】', "]"))
//         .collect();

//     Ok(new_list)
// }

// fn process_kanji_hirigana_into_kanji_with_furigana(
//     new_list: Vec<String>,
// ) -> Result<Vec<String>, Error> {
//     let result_list: Vec<String> = new_list
//         .into_iter()
//         .map(|item| add_furigana(&item))
//         .collect();
//     Ok(result_list)
// }

// fn remove_compound_words(strings: Vec<&str>) -> Vec<&str> {
//     let mut result: Vec<&str> = Vec::new();
//     for s in strings {
//         if s.contains("Compound word") {
//             let index = s.find("Compound word").unwrap();
//             result.push(s[..index].trim_end());
//         } else {
//             result.push(s);
//         }
//     }
//     result
// }

// fn add_furigana(s: &str) -> String {
//     if !s.contains('[') || !s.contains(']') {
//         return s.to_string();
//     }

//     let parts: Vec<&str> = s.split('[').collect();
//     let outside = parts[0].trim().to_string();
//     let mut inside = parts[1].split(']').next().unwrap().trim().to_string();

//     let jap_comma_after_brackets = inside.contains('、');
//     if jap_comma_after_brackets {
//         inside = inside.replace('、', "");
//     }

//     let outside_chars: Vec<char> = outside.chars().collect();
//     let inside_chars: Vec<char> = inside.chars().collect();

//     let n = std::cmp::min(outside_chars.len(), inside_chars.len());
//     let mut common_start = 0;
//     let mut common_end = 0;

//     for i in 0..n {
//         if outside_chars[i] == inside_chars[i] {
//             common_start += 1;
//         } else {
//             break;
//         }
//     }

//     for i in 0..n {
//         if outside_chars[outside_chars.len() - i - 1] == inside_chars[inside_chars.len() - i - 1] {
//             common_end += 1;
//         } else {
//             break;
//         }
//     }

//     let output = if common_start == 0 && common_end == 0 {
//         format!(" {}[{}]", outside, inside)
//     } else if common_start > 0 {
//         format!(
//             " {}[{}]",
//             outside,
//             inside_chars[common_start..].iter().collect::<String>()
//         )
//     } else {
//         format!(
//             " {}[{}]{}",
//             outside_chars[..outside_chars.len() - common_end]
//                 .iter()
//                 .collect::<String>(),
//             inside_chars[..inside_chars.len() - common_end]
//                 .iter()
//                 .collect::<String>(),
//             outside_chars[outside_chars.len() - common_end..]
//                 .iter()
//                 .collect::<String>()
//         )
//     };

//     if jap_comma_after_brackets {
//         format!("{},", output)
//     } else {
//         output
//     }
// }
