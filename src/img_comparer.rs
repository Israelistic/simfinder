extern crate image;

use std::cmp::{max, min};
use std::string::String;
use std::error::Error;
use std::boxed::Box;
use self::image::{DynamicImage, GenericImage, Pixel};
use std::fmt;
use std::time::{SystemTime};
use std::path::PathBuf;

/// Comparison Job descriptor
pub struct CompJob {
	filename0: String,
	filename1: String,
	img_path: PathBuf,
}

// Constructor and getters for job structure
impl CompJob {
	/// A constructor for the CompJob Structure
	pub fn new(filename0: &str, filename1: &str, img_path: &str) -> CompJob {
		CompJob{
			filename0: String::from(filename0),
			filename1: String::from(filename1),
			img_path: PathBuf::from(img_path)}
	}

	/// Getter for first filename
	pub fn get_filename0(&self) -> &str {
		self.filename0.trim()
	}

	/// Getter for second filename
	pub fn get_filename1(&self) -> &str {
		self.filename1.trim()
	}

}

// Implements how comp jobs are printed to the screen (in a non-debug way).
impl fmt::Display for CompJob {
	/// Prints the Job as a string to a formatter
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Job for {} and {}", self.filename0.trim(), self.filename1.trim())
    }
}

// Wrapper to execute a comparison via a job
pub fn execute_job(job: &CompJob) -> Result<(f64, u32), Box<Error>> {
	let now = SystemTime::now();

	let path0 = &job.img_path.join(job.get_filename0());
	let path1 = &job.img_path.join(job.get_filename1());

	// Create shadow variables.
	// Necessary since the to_str function requires the temp variables to live longer
	let path0 = path0.to_str().unwrap_or(job.get_filename0());
	let path1 = path1.to_str().unwrap_or(job.get_filename1());

	let result = compare_files(path0, path1)?;
	Ok((result, util::nanos_to_millis(now.elapsed()?.subsec_nanos())))
}


// Defining a custom comparison error type
#[derive(Debug)] // Debug trait is required but not used, so lets let the compiler implement it
pub struct CompError {
}

// Implementing printing to screen behavior for error (needed Trait for error Trait)
impl fmt::Display for CompError {
	/// Writes the Error as a string to a formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description())
    }
}

// Implementing generic error behavior
impl Error for CompError {
	/// Getter for the description of the error
	fn description(&self) -> &str {
        "An error occurred when comparing image files"
    }
}


// Todo: fix comparing images of different sizes
/// Compares two images. Does a avg pixel difference.
pub fn compare(img0: DynamicImage, img1: DynamicImage) -> Result<f64, CompError> {
	let (img0_width, img0_height) = img0.dimensions();
	let (img1_width, img1_height) = img1.dimensions();

	let mut accumulated_diff: f64 = 0.0;

	let width = min(img0_width, img1_width);
	let height = min(img0_height, img1_height);


	for y in 0..height {
		for x in 0..width {
			let px0 = img0.get_pixel(x, y).to_rgb().data;
			let px1 = img1.get_pixel(x, y).to_rgb().data;
			let mut pixel_diff : u32 = 0;
			for i in 0..px0.len() {
				pixel_diff += (max(px0[i], px1[i]) - min(px0[i], px1[i])) as u32;
			}
			accumulated_diff +=  pixel_diff as f64;
		}
	}

	let max_bounds_pixels = max(img0_width, img1_width) * max(img0_height, img1_height);
	let compared_pixels = width * height;
	let not_compared_pixels = max_bounds_pixels - compared_pixels;
	accumulated_diff += not_compared_pixels as f64 * 255.0;

	return Ok(accumulated_diff / max_bounds_pixels as f64);
}

// TODO: handle errors properly
pub fn compare_files(filename0: &str, filename1: &str) -> Result<f64, Box<Error>>{
    let img0 = image::open(filename0)?;
    let img1 = image::open(filename1)?;
    return Ok(compare(img0, img1)?);
}

mod util {
	pub fn nanos_to_millis(nanos: u32) -> u32 {
		// This converts nanos to milliseconds.
		// For some reason, rust Durations only supports
		// seconds and nano seconds.
		nanos / 1_000_000
	}
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

		let very_different = compare_files("examples/blank0.png", "examples/red0.png").unwrap();
		let little_different = compare_files("examples/blank0.png", "examples/tri0.png").unwrap();
		assert!(very_different > little_different, "Comparison values should somewhat make sense.");

		assert!(compare_files("examples/blank0.png", 
			"examples/big.png").is_ok(), "A strictly small and large image should be comparable.");

		assert!(compare_files("examples/wide.png", 
			"examples/tall.png").is_ok(), "Multiple overlapping image sizes should be comparable.");

	}

	#[test]
	fn execute_job_test() {
		let job0 = super::CompJob::new("examples/blank0.png", "examples/blank1.png", "./");
		let job1 = super::CompJob::new("examples/blank0.png", "examples/tri0.png", "./");
		let (result0, _) = super::execute_job(&job0).unwrap();
		let (result1, _) = super::execute_job(&job1).unwrap();
		assert!(result0 == 0.0, "Images are the same.");
		assert!(result1 > 0.0, "Images are not the same.");
	}

	#[test]
	fn nano_to_millis_test() {
		assert_eq!(super::util::nanos_to_millis(1_000_000), 1, 
			"One million nanoseconds is one millisecond.");
	}

}