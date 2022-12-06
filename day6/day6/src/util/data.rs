use std::fs::File;
use std::io::Read;
use std::{env};

pub fn get_data_from_filename(filename: &str) -> String {
    let src_path = get_src_path();
    let data_path = concat(&src_path, "/data/");
    let path = concat(&data_path, filename);

    println!("Path: {} ", path);
    let data = read_lines(&path);

    // get first element of vector
    let data = data[0].clone();
    return data;

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
    fn test_get_data_from_test1() {
        let data = get_data_from_filename("test1.txt");
        assert_eq!(data, "bvwbjplbgvbhsrlpgdmjqwftvncz");
    }

    #[test]
    fn test_get_data_from_test2() {
        let data = get_data_from_filename("test2.txt");
        assert_eq!(data, "nppdvjthqldpwncqszvftbrmjlhg");
    }

    #[test]
    fn test_get_data_from_test3() {
        let data = get_data_from_filename("test3.txt");
        assert_eq!(data, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    }

    #[test]
    fn test_get_data_from_test4() {
        let data = get_data_from_filename("test4.txt");
        assert_eq!(data, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    }



}

