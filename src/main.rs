mod img_comparer;

fn main() {

    let filename0 = "example-imgs/tri0.png";
    let filename1 = "example-imgs/tri1.png";

    let diff = img_comparer::compare_files(
    	filename0, filename1);
    println!("result: {:?}", diff);

}


