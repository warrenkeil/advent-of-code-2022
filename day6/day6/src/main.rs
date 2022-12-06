mod util;
mod algo;

fn main() {
    let data = util::data::get_data_from_filename("data.txt");
    let part_one_score = algo::scorer::score(&data, 4);
    println!("Score part one: {}", part_one_score);

    let part_two_score = algo::scorer::score(&data, 14);
    println!("Score part two: {}", part_two_score);
}


mod test {
    use super::*;

    #[test]
    fn test1_returns_5() {
        let data = util::data::get_data_from_filename("test1.txt");
        let result = algo::scorer::score(&data, 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2_returns_6() {
        let data = util::data::get_data_from_filename("test2.txt");
        let result = algo::scorer::score(&data, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn test3_returns_10() {
        let data = util::data::get_data_from_filename("test3.txt");
        let result = algo::scorer::score(&data, 4);
        assert_eq!(result, 10);
    }

    #[test]
    fn test4_returns_11() {
        let data = util::data::get_data_from_filename("test4.txt");
        let result = algo::scorer::score(&data, 4);
        assert_eq!(result, 11);
    }

    #[test]
    fn test5_returns_19() {
        let data = util::data::get_data_from_filename("test5.txt");
        let result = algo::scorer::score(&data, 14);
        assert_eq!(result, 19);
    }

    #[test]
    fn test6_returns_23() {
        let data = util::data::get_data_from_filename("test6.txt");
        let result = algo::scorer::score(&data, 14);
        assert_eq!(result, 23);
    }

    #[test]
    fn test7_returns_23() {
        let data = util::data::get_data_from_filename("test7.txt");
        let result = algo::scorer::score(&data, 14);
        assert_eq!(result, 23);
    }

    #[test]
    fn test8_returns_29() {
        let data = util::data::get_data_from_filename("test8.txt");
        let result = algo::scorer::score(&data, 14);
        assert_eq!(result, 29);
    }

    #[test]
    fn test9_returns_26() {
        let data = util::data::get_data_from_filename("test9.txt");
        let result = algo::scorer::score(&data, 14);
        assert_eq!(result, 26);
    }

}