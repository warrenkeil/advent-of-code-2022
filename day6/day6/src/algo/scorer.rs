use std::collections::HashSet;

pub fn score(data: &str, num_distinct: usize) -> i32 {

    let chars: Vec<char> = data.chars().collect();
    // iterate over chars
    for i in num_distinct..chars.len() {

        let char_slice = &chars[i-num_distinct..i];

        // create hashset from char_slice
        let char_set: HashSet<char> = char_slice.iter().cloned().collect();

        // get size of hashset
        let char_set_size = char_set.len();
        if char_set_size == num_distinct {
            return i as i32;
        }
    }
    return -1;
}

