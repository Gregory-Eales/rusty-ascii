use image::GenericImageView;

fn load_image(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let image = image::open(file_path)?;	
		println!("img size: {:?}", image.dimensions());	
		
		for (x, y, pixel) in image.pixels() {
				let rgba = pixel.0;

				println!("pixel at ({}, {}) w color ({}, {}, {})", x, y, rgba[0], rgba[1], rgba[2]);
		}

		Ok(())
}


fn main() {
    println!("Hello, world!");
		
		load_image("img/image.jpg");

    let mut x = 1;

    println!("{}", x);
}
