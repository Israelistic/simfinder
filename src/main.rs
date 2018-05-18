mod img_comparer;
mod io;

fn main() {

	let input_filename = "examples/example.csv";
	let output_filename = "test.csv";


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

