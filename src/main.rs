extern crate clap;

mod img_comparer;
mod io;

use clap::{Arg, App};

fn main() {
	let matches = App::new("simvar")
		.version("1.0")
		.author("Frank V. <fvumbaca@outlook.com>")
		.about("Compares similarity of images.")
		.arg(Arg::with_name("IMG_PATH")
			.short("i")
			.long("imgs")
			.value_name("IMG_PATH")
			.help("Root folder for images referenced in input file.")
			.takes_value(true))
		.arg(Arg::with_name("INPUT_CSV")
			.help("Filename of the input csv listing comparisons to preform.")
			.required(true)
			.index(1))
		.arg(Arg::with_name("OUTPUT_CSV")
			.help("Filename for the results.")
			.required(true)
			.index(2))
		.get_matches();

	let input_filename = matches.value_of("INPUT_CSV").unwrap(); // Is required so safe for unwraping
	let output_filename = matches.value_of("OUTPUT_CSV").unwrap(); // Also required
	let img_path = matches.value_of("IMG_PATH").unwrap_or("./");


    match io::load(input_filename) {
    	Ok(jobs) => {

    		// Make sure we can write to the output, and might as well write the headers too
		    let mut f = std::fs::File::create(output_filename).unwrap();
		    if io::write_headers(&mut f).is_err() {
		    	println!("Could not create file {}.", output_filename);
		    	return;
		    }

		    // Iterate over the jobs and run each.
    		for j in jobs {
    			match img_comparer::execute_job(&j) {
    				Ok(result) => {
    					let (similarity, elapsed) = result;
    					let ok = io::write_results(&mut f, j.get_filename0(), j.get_filename1(), similarity, elapsed);
    					if ok.is_err() {
    						println!("There was a problem writing a result to {}", output_filename);
    					}
    				},
    				Err(_) => {
    					println!("{} could not be completed because one/both of the files do not exist.", j);
    				}
    			}
    		}

    	},
    	Err(_) => {
    		println!("Could not open file {}", input_filename);
    	}
    }

}

