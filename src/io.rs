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
	#[test]
	fn load_test() {
		let result = super::load("examples/example.csv", "./").unwrap();
		assert_eq!(result.len(), 3, "Three rows should be loaded.");
	}
}