mod algo;
mod util;

fn main() {

    let data = util::data::get_data_from_filename("test.txt");
    let data_vecs = util::data::get_vecs_from_data(data);
    println!("Data: {:?}", data_vecs);
}


