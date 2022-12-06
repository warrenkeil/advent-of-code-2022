use std::fs::File;
use std::io::Read;
use std::{env};

pub fn say_hello_data() {
    println!("Hello, data!");
}


pub fn get_data_from_filename(filename: &str) -> Vec<String> {
    let src_path = get_src_path();
    let data_path = concat(&src_path, "/data/");
    let path = concat(&data_path, filename);

    println!("Path: {} ", path);
    let data = read_lines(&path);
    return data;
}

pub fn get_vecs_from_data(data: Vec<String>) -> Vec<Vec<String>> {
    // find index of empty string
    let mut vecs: Vec<Vec<String>> = Vec::new();
    let index = &data.iter().position(|r| r == "").unwrap();
    let string_numbers = &data[index-1];
    let mut max_number = 0;

    // iterate over a string and try to parse as int
    for c in string_numbers.chars() {
        if c != ' ' {
            let number = c.to_string().parse::<i32>().unwrap();
            if number > max_number {
                max_number = number;
            }
        }
    }

    let num_vecs = index - 1;

    for i in 0..max_number {
        let mut vec: Vec<String> = Vec::new();
        vecs.push(vec);
    }
 
    let mut vecs: Vec<Vec<String>> = Vec::new();
    for number in 1..max_number {
        // let mut vec: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        // let mut vec = Vec::new();
        vecs.push(vec!());
    }

    // add data to vecs
    for i in (0..index-1).rev() {
        for j in 0..max_number {
            let col_num = ((j*4)+1) as usize;
            // println!("col_num: {}", col_num);
            let char_num = data[i].chars().nth(col_num).unwrap();
            println!("j is: {}", j);
            if char_num != ' ' {
                println!("char_num: {}", char_num);
                vecs[j as usize].push(char_num.to_string());
            }
        }
    }
    println!("vecs: {:?}", vecs);

    println!("break");

    return vecs;
}


fn get_src_path() -> String {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.to_str().unwrap().to_string()
}

fn concat(s1: &str, s2: &str) -> String {
    let mut s = String::new();
    s.push_str(s1);
    s.push_str(s2);
    s
}

fn read_lines(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}


mod test {
    use super::*;

    #[test]
    fn test_get_data_from_filename() {
        let data = get_data_from_filename("test.txt");
        assert_eq!(data[0],"    [D]    ");
        assert_eq!(data[1], "[N] [C]    ");
    }
}