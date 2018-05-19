extern crate clap;
extern crate rayon;

mod img_comparer;
mod io;

use clap::{Arg, App};
use std::sync::mpsc::channel;

// Default threads to use is 3. I remember reading somewhere this is a safe 
// number to assume because it is the least number of worker threads allowed 
// to be run concurrently on the same machine. More threads MIGHT have diminishing
// returns depending on OS configuration. ITS BEST TO RUN SOME TESTS AND SEE RUNTIME.
const DEFAULT_THREADPOOL_COUNT: usize = 3;

fn main() {
	// Parse arguments.
	let matches = App::new("simvar")
		.version("1.0")
		.author("Frank V. <fvumbaca@outlook.com>")
		.about("Compares similarity of images.")
		.arg(Arg::with_name("IMG_DIR")
			.short("i")
			.long("img-dir")
			.value_name("IMG_DIR")
			.help("Root folder for images referenced in input file.")
			.takes_value(true))
		.arg(Arg::with_name("THREAD_COUNT")
			.short("t")
			.long("imgs")
			.value_name("THREAD_COUNT")
			.help("Number of threads in thread pool.")
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

	// Passed arguments after parsing and applying defaults and type enforcements
	let input_filename = matches.value_of("INPUT_CSV").unwrap(); // Is required so safe for unwraping
	let output_filename = matches.value_of("OUTPUT_CSV").unwrap(); // Also required
	let img_dir = matches.value_of("IMG_DIR").unwrap_or("./");
	let thread_count : usize = matches.value_of("THREAD_COUNT").unwrap_or(&DEFAULT_THREADPOOL_COUNT.to_string())
		.parse().unwrap_or_else(|_| {
			println!("Invalid number of threads!");
			std::process::exit(1);
		});

    match io::load(input_filename, img_dir) {
    	Ok(jobs) => { // No errors when loading the file

    		// Make sure we can write to the output, and might as well write the headers too
		    let mut f = std::fs::File::create(output_filename).unwrap();
		    if io::write_headers(&mut f).is_err() {
		    	println!("Could not create file {}.", output_filename);
		    	return;
		    }

		    // Spin up the thread pool
		    let pool = rayon::ThreadPoolBuilder::new().num_threads(thread_count).build().unwrap();
		    let (tx, rx) = channel();

		    let num_jobs = jobs.len();

		    // Iterate over the jobs and run each.
    		for j in jobs {
				let tx = tx.clone();
    			pool.install(move || {
    				let result = img_comparer::execute_job(&j);
    				// Re-wrapping Result to a Option enum for thread safety
    				match result {
    					Ok((similarity, elapsed)) => {
		    				tx.send((Some((similarity, elapsed)), j)).unwrap();
    					},
    					Err(_) => {
    						tx.send((None, j)).unwrap();
    					}
    				}
    			});
    		}

    		for result in rx.iter().take(num_jobs) { // will wait for all the jobs to complete, but handle as soon as a result is received
    			let (result, j) = result;
    			match result {
    				Some((similarity, elapsed)) => {
						let ok = io::write_results(&mut f, j.get_filename0(), j.get_filename1(), similarity, elapsed);
						if ok.is_err() {
							println!("There was a problem writing a result to {}", output_filename);
						}
    				},
    				None => {
    					println!("{} could not be completed because one/both of the files do not exist.", j);
    				}
    			}
    		}

    	},

    	Err(_) => { // An error occurred when loading the input file
    		println!("Could not open file {}", input_filename);
    	}
    }

}

