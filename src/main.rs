use image::GenericImageView;

fn load_image(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let image = image::open(file_path)?;	
		println!("img size: {:?}", image.dimensions());	
		let mut out : Vec<String> = vec![];	
    let mut curr_string : String = "".to_string();
    for x in 0..250 {
        for y in 0..250 {
            curr_string.push(get_ascii_char(&image, 2*x, 2*y, 2).unwrap());
        }        
				out.push(curr_string.clone());
        curr_string = "".to_string();
    }     
        
		println!("length of out vec: {}", out.len());	
    
    for s in out.iter() {
        println!(" {} {} ", s, "");
    }

		Ok(())
}

fn get_magnitude(a : u32, b : u32, c : u32) -> u32 {
    return ((a.pow(2) + b.pow(2) + c.pow(2)) as f32).sqrt() as u32
}

fn get_ascii_char(img: &dyn GenericImageView<Pixel = image::Rgba<u8>>, i: u32, j: u32, size: u32) -> Option<char> {
    let ascii_char = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,^`\'.";	
    let num_char = ascii_char.len();
    let mut vals : Vec<u32> = vec![];
    let max_mag = (3.0*(225_i32).pow(2) as f32).sqrt();
    for a in 0..size {
        for b in 0..size {
            if b + j >= img.dimensions().1 || a + i >= img.dimensions().0 {
                continue;
            } 
            let pixel = img.get_pixel(i + a, j + b).0;
            vals.push(get_magnitude(pixel[0] as u32, pixel[1] as u32, pixel[2] as u32));   
        }
    } 

    let mut avg: u32 = 0;
    for idx in 0..vals.len() {
        avg += vals[idx];
    }
    avg = avg / vals.len() as u32;
    
    avg = (num_char as f32 * avg as f32  / max_mag) as u32;
    if avg > 1 {
        avg = avg - 1;
    }
    if avg >= num_char as u32 - 1 {
         avg = num_char as u32 - 1;
    }
    return ascii_char.chars().nth(avg as usize);
}


fn main() {
    println!("Hello, world!");
		load_image("img/image.jpg");
}
