extern crate image;

use image::{DynamicImage, GenericImage, Pixel};
use std::cmp::{max, min};

// Todo: fix comparing images of different sizes
pub fn compare(img0: DynamicImage, img1: DynamicImage) -> f64 {
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

	return accumulated_diff.sqrt();

}