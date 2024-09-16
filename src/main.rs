use image::GenericImageView;

fn load_image(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let image = image::open(file_path)?;	
		println!("img size: {:?}", image.dimensions());	
		let mut out : Vec<&str> = vec![];	
    let mut curr_pool : Vec<&u32> = vec![];
		for (x, y, pixel) in image.pixels() {
				let rgba = pixel.0;

				//println!("pixel at ({}, {}) w color ({}, {}, {})", x, y, rgba[0], rgba[1], rgba[2]);
				if out.len() < (y / 10 + 1 as u32).try_into().unwrap() {
						out.push("");
				}
        // need to pool pixel values
		    get_ascii_char(&image, x, y, 10);	
  	}
		println!("length of out vec: {}", out.len());	
		// need to create a mapping between pixels -> ascii -> pixels 
		// img -> text -> img 
		// might need some kind of edge detection for character seleciton
		// need to decide pixel pooling to ascii character

		Ok(())
}

fn get_magnitude(a : u32, b : u32, c : u32) -> u32 {
    return ((a.pow(2) + b.pow(2) + c.pow(2)) as f32).sqrt() as u32
}

fn get_ascii_char(img: &dyn GenericImageView<Pixel = image::Rgba<u8>>, i: u32, j: u32, size: u32) {
    let ascii_char = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,^`\'.";	
    let num_char = ascii_char.len();
    // loop through square (size x size) starting from i, j
    // map find magnitude of color vector avg across pixels
    // map intensity (avg mag) to char via ascii str (brightest -> darkest) 
    let mut vals : Vec<u32> = vec![];
    let max_mag = (3.0*(225_i32).pow(2) as f32).sqrt();
    for a in 0..size {
        for b in 0..size {
            if b + j >= img.dimensions().1 || a + i >= img.dimensions().0 {
                continue;
            } 
            let mut pixel = img.get_pixel(i + a, j + b).0;
            vals.push(get_magnitude(pixel[0] as u32, pixel[1] as u32, pixel[2] as u32));   
        }
    } 
}

fn main() {
    println!("Hello, world!");
		load_image("img/image.jpg");
}
