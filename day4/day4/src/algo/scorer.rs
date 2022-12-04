pub fn get_overlapping_pairs(data: &Vec<String>) -> usize {
    let mut count = 0;
    for line in data {
        let first_lower_bound = line.split(",").nth(0).unwrap()
            .split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let first_upper_bound = line.split(",").nth(0).unwrap()
            .split("-").nth(1).unwrap().parse::<usize>().unwrap();
        let second_lower_bound = line.split(",").nth(1).unwrap()
            .split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let second_upper_bound = line.split(",").nth(1).unwrap()
            .split("-").nth(1).unwrap().parse::<usize>().unwrap();

        if first_lower_bound <= second_lower_bound && first_upper_bound >= second_upper_bound {
            count += 1;
        }else if second_lower_bound <= first_lower_bound && second_upper_bound >= first_upper_bound {
            count += 1;
        }
            
    }
    count
}

pub fn get_any_overlapping_points(data: &Vec<String>) -> usize {
    let mut count = 0;
    for line in data {
        let first_lower_bound = line.split(",").nth(0).unwrap()
            .split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let first_upper_bound = line.split(",").nth(0).unwrap()
            .split("-").nth(1).unwrap().parse::<usize>().unwrap();
        let second_lower_bound = line.split(",").nth(1).unwrap()
            .split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let second_upper_bound = line.split(",").nth(1).unwrap()
            .split("-").nth(1).unwrap().parse::<usize>().unwrap();

        if second_lower_bound <= first_upper_bound && second_lower_bound >= first_lower_bound {
            count += 1;
        } 
        else if second_upper_bound >= first_lower_bound && second_upper_bound <= first_upper_bound {
            count += 1;
        }
        else if first_lower_bound <= second_lower_bound && first_upper_bound >= second_upper_bound {
            count += 1;
        }else if second_lower_bound <= first_lower_bound && second_upper_bound >= first_upper_bound {
            count += 1;
        }
        else{
            println!("No overlap: {} {}", line, count);
        }
    }
    count
}

// create test module
mod test {
    use super::get_overlapping_pairs;

    #[test]
    fn test_get_overlapping_pairs() {
        let data = vec!["2-4,6-8".to_string(), "1-3,1-7".to_string(), "1-4,2-3".to_string()];
        let count = get_overlapping_pairs(&data);
        assert_eq!(count, 2);
    }
}