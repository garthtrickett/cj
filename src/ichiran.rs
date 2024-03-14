// ichiran.rs
use std::io::Error;
use std::process::Command;
use unicode_segmentation::UnicodeSegmentation;

pub fn run_docker_command(input: &str) -> Result<String, Error> {
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

pub fn ichiran_output_to_bracket_furigana(ichiran_output: &str) -> Result<Vec<String>, Error> {
    let new_list = ichiran_output_to_kanji_hirigana_array(ichiran_output)?;
    let result_list = process_kanji_hirigana_into_kanji_with_furigana(new_list)?;
    Ok(result_list)
}

fn ichiran_output_to_kanji_hirigana_array(result: &str) -> Result<Vec<String>, Error> {
    let lines: Vec<&str> = result.lines().collect();
    let star_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with('*'))
        .copied()
        .collect();

    let star_lines = remove_compound_words(star_lines);

    // If weird stuff happens on ocassion the euivalent of this python code needs may need to go in
    // star_lines = [re.sub(r'(?<=【)[^】]*', lambda x: x.group().replace(' ', ''), s) for s in star_lines]

    let mut new_list: Vec<String> = Vec::new();
    for string in star_lines {
        let split_string: Vec<&str> = string.split(' ').collect(); // Now in wider scope

        if string.contains('【') {
            let split_string: Vec<&str> = string.split(' ').collect();
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

    let jap_comma_after_brackets = if inside.contains('、') {
        inside = inside.replace('、', "");
        true
    } else {
        false
    };

    let outside_graphemes =
        UnicodeSegmentation::graphemes(&outside[..], true).collect::<Vec<&str>>();
    let inside_graphemes = UnicodeSegmentation::graphemes(&inside[..], true).collect::<Vec<&str>>();

    let n = std::cmp::min(outside_graphemes.len(), inside_graphemes.len());
    let mut common_start = 0;
    let mut common_end = 0;

    for i in 0..n {
        if outside_graphemes[i] == inside_graphemes[i] {
            common_start += 1;
        } else {
            break;
        }
    }

    for i in 0..n {
        if outside_graphemes[n - i - 1] == inside_graphemes[n - i - 1] {
            common_end += 1;
        } else {
            break;
        }
    }

    let output = if common_start == 0 && common_end == 0 {
        format!(" {}", s.replace(' ', ""))
    } else if common_start > 0 {
        format!(
            " {}[{}]",
            outside_graphemes[..common_start].concat(),
            inside_graphemes[common_start..].concat()
        )
    } else {
        format!(
            " {}[{}]{}",
            outside_graphemes[..outside_graphemes.len() - common_end].concat(),
            inside_graphemes[..inside_graphemes.len() - common_end].concat(),
            outside_graphemes[outside_graphemes.len() - common_end..].concat()
        )
    };

    if jap_comma_after_brackets {
        format!("{},", output)
    } else {
        output
    }
}
