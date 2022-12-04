mod util;
mod algo;

fn main() {

    let data = util::data::get_data_from_filename("data.txt");

    let count = algo::scorer::get_overlapping_pairs(&data);
    println!("Part one: {}", count);

    let second_count = algo::scorer::get_any_overlapping_points(&data);
    println!("Part two: {}", second_count);
}


