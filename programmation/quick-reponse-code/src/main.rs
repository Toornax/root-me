use bardecoder;
use bardecoder::prepare::BlockedMean;
// use quirc;

use image::{ ImageBuffer, ImageFormat };

fn main() {
    let mut img_read = image::open("qrcode_rootme.png").unwrap().into_rgba8();

	let mut img_postionner = ImageBuffer::from_fn(64, 64, |x, y| {
		if x <= 9 || y <= 9 || x >= 55 || y >= 55 {			
			image::Rgba::<u8>([0, 0, 0, 255])
		} else if x >= 18 && x <= 46 && y >= 18 && y <= 46  {		
			image::Rgba::<u8>([0, 0, 0, 255])			
		} else {			
			image::Rgba::<u8>([255, 255, 255, 255])
		}
	});
	image::imageops::overlay(&mut img_read, &img_postionner, 18, 18);
	image::imageops::overlay(&mut img_read, &img_postionner, 18, 218);
	image::imageops::overlay(&mut img_read, &img_postionner, 218, 18);

	img_read.save_with_format("test.png", ImageFormat::Png);

    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(&img_read);
    for result in results {
        println!("{}", result.unwrap());
    }
}