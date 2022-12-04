use std::collections::HashMap;
use std::{env};
use std::fs::File;
use std::io::Read;

fn main() {
    // let score = get_sum_of_priorities("testData.txt");
    let score = get_sum_of_priorities("data.txt");
    let second_score = get_sum_of_elf_group_priorities("data.txt");
    println!("Score for part 1: {}", score);
    println!("Score for part 2: {}", second_score);
}

fn get_sum_of_elf_group_priorities(filename: &str) -> usize {
    let data = get_data_from_filename(filename);
    let mut score = 0;

    for i in 0..data.len()/3 {
        let ruck_one = &data[i*3];
        let ruck_two = &data[i*3+1];
        let ruck_three = &data[i*3+2];
        let ruck_one_no_dupes = remove_duplicate_chars(ruck_one);
        let ruck_two_no_dupes = remove_duplicate_chars(ruck_two);
        let rust_three_no_dupes = remove_duplicate_chars(ruck_three);
        let combined_2 = concat(&ruck_one_no_dupes, &ruck_two_no_dupes);
        let combined_3 = concat(&combined_2, &rust_three_no_dupes);
        println!("Combined string: {}", combined_3);
        println!("Char of order 3 is: {}", get_char_of_order_three(&combined_3));
        let char_3 = get_char_of_order_three(&combined_3);
        score += get_letter_rank(char_3);
    };
    return score;
}

fn get_char_of_order_three(combined_string: &str) -> char {
    let mut char_hash = HashMap::new();
    for char in combined_string.chars() {
        if char_hash.contains_key(&char) {
            if char_hash.get(&char).unwrap() == &2 {
                return char;
            }
            let count = char_hash.get(&char).unwrap();
            char_hash.insert(char, count + 1);
        } else {
            char_hash.insert(char, 1);
        }
    }
    return '1';
}

fn get_sum_of_priorities(filename: &str) -> usize{
    let data = get_data_from_filename(filename);
    let mut score = 0;

    for line in data {
        let left_string = split_string(&line, true);
        let right_string = split_string(&line, false);
        let left_unique = remove_duplicate_chars(left_string);
        let right_unique = remove_duplicate_chars(right_string);
        let deduped_string = concat(&left_unique, &right_unique);
        let dupe_char = find_first_duplicate_char(&deduped_string);
        score += get_letter_rank(dupe_char);
    };

    return score;
}

fn get_data_from_filename(filename: &str) -> Vec<String> {
    let src_path = get_src_path();
    let data_path = concat(&src_path, "/data/");
    let path = concat(&data_path, filename);

    println!("Path: {} ", path);
    let data = read_lines(&path);
    return data;

}

// function to remove duplicate characters from a string
fn remove_duplicate_chars(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars.into_iter().collect()
}

// function to split string in half
fn split_string(s: &str, is_first: bool) -> &str  {
    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);
    if is_first{
        return left;
    }
    right
}

// function to find duplicate characters in a string
fn find_first_duplicate_char(s: &str) -> char {
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        for j in i+1..chars.len() {
            if chars[i] == chars[j] {
                return chars[i];
            }
        }
    }
    return '1';
}

// function to get rank of letter in the alphabet
fn get_letter_rank(letter: char) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let position = alphabet.chars().position(|c| c == letter).unwrap();
    return position +1;
}

// function to read lines from a file
fn read_lines(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}

// function to get path of src directory
fn get_src_path() -> String {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.to_str().unwrap().to_string()
}

// function for string concatenation 
fn concat(s1: &str, s2: &str) -> String {
    let mut s = String::new();
    s.push_str(s1);
    s.push_str(s2);
    s
}

// create test module
mod tests {
    #[test]
    fn test_find_first_duplicate_char() {
        let s = "abcde";
        let dupe_char = super::find_first_duplicate_char(&s);
        assert_eq!(dupe_char, '1');
    }

    #[test]
    fn test_get_sum_of_priorities() {
        let score = super::get_sum_of_priorities("testData.txt");
        assert_eq!(score, 157);
    }

    #[test]
    fn test_get_sum_of_elf_group_priorities() {
        let score = super::get_sum_of_elf_group_priorities("testData.txt");
        assert_eq!(score, 70);
    }
}