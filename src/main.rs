extern crate image;

mod img_comparer;

fn main() {

    let img0 = image::open("example-imgs/tri0.png").unwrap();
    let img1 = image::open("example-imgs/tri1.png").unwrap();

    let diff = img_comparer::compare(img0, img1);

    println!("result: {:?}", diff);



}


