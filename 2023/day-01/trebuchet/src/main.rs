use std::fs;
use std::iter::Iterator;

fn main() {
    let data = read_input_data();
    let mut total = 0u32;
    for line in data.lines() {
        let (first_numeric, last_numeric) = find_first_and_last_numerics(line);
        let line_sum = first_numeric * 10 + last_numeric;
        total += line_sum;
    }
    println!("total: {}", total);
}

fn read_input_data() -> String {
    fs::read_to_string("D:\\SideProjects\\advent-of-code\\2023\\day-01\\trebuchet\\resources\\input.txt").expect("couldn't read input file")
}

fn find_first_and_last_numerics(line: &str) -> (u32, u32) {
    (get_first_number(line.chars(), false), get_first_number(line.chars().rev(), true))
}

fn get_first_number(sequence: impl Iterator<Item = char>, reversed: bool) -> u32 {
    let mut first = None;
    let mut last_three = String::new();
    let mut last_four = String::new();
    let mut last_five = String::new();
    for char in sequence {
        append_sized(char, &mut last_three, 3);
        append_sized(char, &mut last_four, 4);
        append_sized(char, &mut last_five, 5);

        let may_spell_num_three = check_spells_num_three(&last_three, reversed);
        let may_spell_num_four = check_spells_num_four(&last_four, reversed);
        let may_spell_num_five = check_spells_num_five(&last_five, reversed);

        if char.is_ascii_digit() {
            first = Some(char);
            break;
        } else if let Some(digit_as_char) = may_spell_num_five {
            first = Some(digit_as_char);
            break;
        } else if let Some(digit_as_char) = may_spell_num_four {
            first = Some(digit_as_char);
            break;
        } else if let Some(digit_as_char) = may_spell_num_three {
            first = Some(digit_as_char);
            break;
        }
    }
    let first = first.expect("there should have been a digit");

    first.to_digit(10).expect("should have already identified a digit")
}

fn append_sized(char: char, string: &mut String, max_len: usize) {
    if string.chars().count() == max_len {
        string.remove(0);
    }
    string.push(char);
}

fn check_spells_num_three(orig_word: &str, reversed: bool) -> Option<char> {
    let mut word = orig_word;
    let rev_word: String = orig_word.chars().rev().collect();
    if reversed {
        word = rev_word.as_str();
    }
    match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "six" => Some('6'),
        _ => None
    }
}

fn check_spells_num_four(orig_word: &str, reversed: bool) -> Option<char> {
    let mut word = orig_word;
    let rev_word: String = orig_word.chars().rev().collect();
    if reversed {
        word = rev_word.as_str();
    }
    match word {
        "four" => Some('4'),
        "five" => Some('5'),
        "nine" => Some('9'),
        _ => None
    }
}

fn check_spells_num_five(orig_word: &str, reversed: bool) -> Option<char> {
    let mut word = orig_word;
    let rev_word: String = orig_word.chars().rev().collect();
    if reversed {
        word = rev_word.as_str();
    }
    match word {
        "three" => Some('3'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::find_first_and_last_numerics;

    #[test]
    fn finds_different_first_and_last() {
        let result = find_first_and_last_numerics("a5bcd6e");
        assert_eq!(result, (5, 6));
    }
    #[test]
    fn finds_same_first_and_last() {
        let result = find_first_and_last_numerics("a5e");
        assert_eq!(result, (5, 5));
    }
    #[test]
    fn finds_correct_of_many() {
        let result = find_first_and_last_numerics("a524570987fkna09845fanjdofie37409ndfandf8e1");
        assert_eq!(result, (5, 1));
    }
    #[test]
    fn finds_different_spelled() {
        let result = find_first_and_last_numerics("twoabeone");
        assert_eq!(result, (2, 1));
    }
    #[test]
    fn finds_other_different_spelled() {
        let result = find_first_and_last_numerics("aieonfourwmos52349iwsixqdguu");
        assert_eq!(result, (4,6));
    }
    #[test]
    fn finds_same_spelled() {
        let result = find_first_and_last_numerics("kghjythreeacae");
        assert_eq!(result, (3, 3));
    }
    #[test]
    fn finds_overlapping_spelled() {
        let result = find_first_and_last_numerics("kghjysevenineacae");
        assert_eq!(result, (7, 9));
    }
    #[test]
    fn finds_spelled_and_digit() {
        let result = find_first_and_last_numerics("kghjyteightwoneaca4e");
        assert_eq!(result, (8, 4));
    }
    #[test]
    fn finds_digit_and_spelled() {
        let result = find_first_and_last_numerics("kghjyte8ightwoneacafivee");
        assert_eq!(result, (8, 5));
    }
}