extern crate csv;

use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::boxed::Box;
use std::io::Write;

use super::img_comparer::CompJob;

/// Loads a csv file into a vector of jobs.
pub fn load<'a>(filename: &str, img_dir: &str) -> Result<Vec<CompJob>, &'a str> {
	let file = PathBuf::from(filename);
	let reader = csv::Reader::from_path(file);

	if reader.is_ok() { // Make sure the reader can read the file.
		let mut reader = reader.unwrap();
		let mut jobs : Vec<CompJob> = Vec::new();

		for result in reader.records() { // Iterate over the records in the result
			if let Ok(record) = result {
				jobs.push(CompJob::new(&record[0], &record[1], img_dir));
			}
		}
		return Ok(jobs);
	} else { // 
		Err("Could not read input file. Does the csv file exist?")
	}
}

/// Writes the headers of the output csv.
pub fn write_headers(mut f: &File) -> Result<(), Box<Error>> {
	writeln!(&mut f, "image1, image2, similar, elapsed,")?;
	Ok(())
}

/// Writes the row in the csv for a comparison result.
pub fn write_results(mut f: &File, filename0: &str, filename1: &str, similarity: f64, elapsed: u32) -> Result<(), Box<Error>> {
	writeln!(&mut f, "{}, {}, {:.2}, {},", filename0, filename1, similarity, elapsed)?;
	Ok(())
}

#[cfg(test)]
mod tests {

	use std::path::PathBuf;
	use std::fs::{File, remove_file};
	use std::io::prelude::*;


	#[test]
	fn load_test() {
		// This test also contains files that do not exist to also test for
		// application robustness
		let result = super::load("examples/example.csv", "./");
		assert!(result.is_ok(), "The file should be loaded properly.");
		let result = result.unwrap();
		assert_eq!(result.len(), 3, "Three rows should be loaded.");

		let result = super::load("examples/empty.csv", "./");
		assert!(result.is_ok(), "The empty file should be loaded properly.");
		let result = result.unwrap();
		assert_eq!(result.len(), 0, "No rows rows should be loaded for an empty file.");

		// Just to make sure these files do not raise an error: the data will be wrong tho
		let result = super::load("examples/one-too-many.csv", "./");
		assert!(result.is_ok(), "The badly formated file should be loaded properly");
		let result = super::load("examples/one-too-few.csv", "./");
		assert!(result.is_ok(), "The badly formated file should be loaded properly");
	}

	#[test]
	fn write_test() {
		let filename = PathBuf::from("examples/example-output.csv");
		let filename = filename.to_str().expect("Path should evaluate properly.");

		let mut file = File::create(filename).expect("Unable to open the file");

		assert!(super::write_headers(&mut file).is_ok(), "File headers should be written properly.");

		assert!(super::write_results(&mut file, "test1.png", "test2.png", 0.111, 111).is_ok(),
		 "Results should be written to the output file.");

		let mut file = File::open(filename).expect("Should be able to open file just written to.");

    	let mut result = String::new();
    	file.read_to_string(&mut result).expect("Unable to read the file");
    	assert_eq!(result,
    		"image1, image2, similar, elapsed,\ntest1.png, test2.png, 0.11, 111,\n", "The output should be exactly as expected.");

    	assert!(remove_file(filename).is_ok(), "Test output was deleted correctly");

	}

}