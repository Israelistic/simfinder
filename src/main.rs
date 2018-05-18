mod img_comparer;
mod io;

fn main() {

    let mut f = std::fs::File::create("test.csv").unwrap();
    io::write_headers(&mut f);

    match io::load("examples/example.csv") {
    	Ok(jobs) => {


    		for j in jobs {
    			match img_comparer::execute_job(&j) {
    				Ok(result) => {
    					// println!("A result!: {:?}", result);
    					let (similarity, elapsed) = result;
    					io::write_results(&mut f, j.get_filename0(), j.get_filename1(), similarity, elapsed);
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

}

