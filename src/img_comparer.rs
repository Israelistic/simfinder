extern crate image;

use std::cmp::{max, min};
use std::string::String;
use std::error::Error;
use std::boxed::Box;
use self::image::{DynamicImage, GenericImage, Pixel};
use std::fmt;

// Information to start a comparison job
pub struct CompJob {
	filename0: String,
	filename1: String,
}

// Constructor for job structure
impl CompJob {
	pub fn new(filename0: String, filename1: String) -> CompJob {
		return CompJob{filename0, filename1};
	}
	pub fn new(filename0: &str, filename1: &str) -> CompJob {
		return CompJob{filename0: String::from(filename0), filename1: String::from(filename1)};
	}
}

// Wrapper to execute a comparison via a job
pub fn execute_job(job: CompJob) -> Result<f64, Box<Error>> {
	return compare_files(job.filename0.trim(), job.filename1.trim());
}

// Defining a custom comparison error type
#[derive(Debug)]
pub struct CompError {
}

// Implementing printing to screen behavior for error
impl fmt::Display for CompError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description())
    }
}

// Implementing generic error behavior
impl Error for CompError {
	fn description(&self) -> &str {
        "An error occurred when comparing image files"
    }
}

// Todo: fix comparing images of different sizes
pub fn compare(img0: DynamicImage, img1: DynamicImage) -> Result<f64, CompError> {
	let (img0_width, img0_height) = img0.dimensions();
	let (img1_width, img1_height) = img1.dimensions();

	let mut accumulated_diff: f64 = 0.0;

	for y in 0..max(img0_height, img1_height) {
		for x in 0..max(img0_width, img1_width) {
			let px0 = img0.get_pixel(x, y).to_rgb().data;
			let px1 = img1.get_pixel(x, y).to_rgb().data;
			let mut pixel_diff : u32 = 0;
			for i in 0..px0.len() {
				pixel_diff += (max(px0[i], px1[i]) - min(px0[i], px1[i])) as u32;
			}
			accumulated_diff +=  pixel_diff as f64 * pixel_diff as f64;
		}
	}

	return Ok(accumulated_diff.sqrt());
}

// TODO: handle errors properly
pub fn compare_files(filename0: &str, filename1: &str) -> Result<f64, Box<Error>>{
    let img0 = image::open(filename0)?;
    let img1 = image::open(filename1)?;
    return Ok(compare(img0, img1)?);
}

// Unit tests
#[cfg(test)]
mod tests {
	use super::compare_files;

	#[test]
	fn compare_files_test() {
		assert!(compare_files("examples/blank0.png", 
			"examples/tri0.png").unwrap() > 0.0, "Images are not the same.");
		assert!(compare_files("examples/tri0.png", 
			"examples/tri1.png").unwrap() == 0.0, "Images are the same.");
	}

	#[test]
	fn execute_job_test() {
		let job0 = super::CompJob::new("examples/blank0.png", "examples/blank1.png");
		let job1 = super::CompJob::new("examples/blank0.png", "examples/tri0.png");
		assert!(execute_job(job0).unwrap() == 0.0, "Images are the same.");
		assert!(execute_job(job1).unwrap() > 0.0, "Images are not the same.");
	}

}