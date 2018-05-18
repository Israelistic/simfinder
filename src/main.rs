mod img_comparer;

fn main() {

    load("examples/example.csv");

    let filename0 = "examples/tri0.png";
    let filename1 = "examples/tri1.png";

    let diff = img_comparer::compare_files(
    	filename0, filename1);
    println!("result: {:?}", diff);
    let j = img_comparer::CompJob::new(String::new(), String::new());
    println!("test: {:?}", img_comparer::execute_job(j));
}



#[derive(Debug)]
struct CompJob {
	filename0: String,
	filename1: String,
}

extern crate csv;
use std::path::Path;
use std::string::String;

fn load(filename: &str) {
	let file = Path::new(filename);

	let reader = csv::Reader::from_path(file);
	if reader.is_ok() {
		let mut reader = reader.unwrap();
		for result in reader.records() {
			let record = result.unwrap();
			println!("{:?}", CompJob{filename0: String::from(&record[0]), filename1: String::from(&record[1])})
		}
	}

}