extern crate csv;

use std::path::Path;

mod img_comparer;
use img_comparer::CompJob;

fn main() {

    match load("examples/example.csv") {
    	Ok(jobs) => {
    		println!("Got {:?} jobs!", jobs.len());
    		for j in jobs {

    			match img_comparer::execute_job(&j) {
    				Ok(result) => {
    					println!("A result!: {:?}", result);
    				},
    				Err(_) => {
    					println!("{} could not be completed because one/both of the files do not exist.", j);
    				}
    			}

    		}
    	},
    	Err(msg) => {
    		println!("Got error: {:?}", msg);
    	}
    }

    /*
    let filename0 = "examples/tri0.png";
    let filename1 = "examples/tri1.png";

    let diff = img_comparer::compare_files(
    	filename0, filename1);
    println!("result: {:?}", diff);
    let j = img_comparer::CompJob::new(String::new(), String::new());
    println!("test: {:?}", img_comparer::execute_job(j));
    */
}



// I will leave the csv parsing in this file, since I want to print to screen
// and having multiple files print to screen can be confusing. It is also super small,
// so it should not be too bad. Would need to be moved out if additional methods were needed.
fn load(filename: &str) -> Result<Vec<CompJob>, &str> {
	let file = Path::new(filename);

	let reader = csv::Reader::from_path(file);

	if reader.is_ok() {
		let mut reader = reader.unwrap();
		let mut jobs : Vec<CompJob> = Vec::new();

		for result in reader.records() {
			let record = result.unwrap();
			// println!("{:?}", CompJob{filename0: String::from(&record[0]), filename1: String::from(&record[1])})
			jobs.push(CompJob::new(&record[0], &record[1]));
		}
		return Ok(jobs);
	} else {
		return Err("Could not open reader to file. Does the csv file exist?");
	}

}