extern crate csv;

use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::boxed::Box;
use std::io::Write;

use super::img_comparer::CompJob;

// Load csv file into a vector of jobs
pub fn load<'a>(filename: &str, img_dir: &str) -> Result<Vec<CompJob>, &'a str> {
	let file = PathBuf::from(filename);

	let reader = csv::Reader::from_path(file);

	if reader.is_ok() {
		let mut reader = reader.unwrap();
		let mut jobs : Vec<CompJob> = Vec::new();

		for result in reader.records() {
			let record = result.unwrap();
			jobs.push(CompJob::new(&record[0], &record[1], img_dir));
		}
		return Ok(jobs);
	} else {
		Err("Could not open reader to file. Does the csv file exist?")
	}
}

pub fn write_headers(mut f: &File) -> Result<(), Box<Error>> {
	writeln!(&mut f, "image1, image2, similar, elapsed,")?;
	Ok(())
}

pub fn write_results(mut f: &File, filename0: &str, filename1: &str, similarity: f64, elapsed: u32) -> Result<(), Box<Error>> {
	writeln!(&mut f, "{}, {}, {:.2}, {},", filename0, filename1, similarity, elapsed)?;
	Ok(())
}


#[cfg(test)]
mod tests {
	#[test]
	fn load_test() {
		let result = super::load("examples/example.csv", "./").unwrap();
		assert_eq!(result.len(), 3, "Three rows should be loaded.");
	}
}