use image::{ io::Reader, ImageBuffer, Rgba, RgbaImage };
use crate::{ Grid, Color, ImageConversion };
use std::{ error::Error, path::Path };



impl<T> Grid<T> {

	/// Read from png file.
	pub fn from_png(path:&str) -> Result<Grid<T>, Box<dyn Error>> where T:From<Color> {
		if Path::new(path).exists() {
			let read_image:RgbaImage = Reader::open(path)?.decode()?.to_rgba8();
			let colors:Vec<Color> = read_image.pixels().map(|rgba| Color(u32::from_be_bytes([rgba[3], rgba[0], rgba[1], rgba[2]]))).collect::<Vec<Color>>();
			Ok(Grid::new(colors.into_iter().map(|color| T::from(color)).collect(), read_image.width() as usize, read_image.height() as usize))
		} else {
			Err(format!("Could not read png data from file '{path}'").into())
		}
	}

	/// Store the grid as a PNG.
	pub fn to_png(&self, file_path:&str) -> Result<(), Box<dyn Error>> where Color:for<'a> From<&'a T> {		
		let mut img:ImageBuffer<Rgba<_>, Vec<_>> = ImageBuffer::new(self.width as u32, self.height as u32);
		for (x, y, pixel) in img.enumerate_pixels_mut() {
			let mut color:[u8; 4] = <[u8; 4]>::from(Color::from(&self[(x as usize, y as usize)]));
			color.rotate_left(1);
			*pixel = Rgba(color);
		}
		img.save(file_path)?;
		Ok(())
	}
}


pub struct PngConversion;
impl ImageConversion for PngConversion {

	/// The file extension required for conversion.
	fn file_extension() -> &'static str {
		"png"
	}

	/// Read an image from a file.
	fn image_from_file<T:From<Color>>(path:&str) -> Result<Grid<T>, Box<dyn Error>> {
		Grid::from_png(path)
	}

	/// Write an image to a file.
	fn image_to_file<T>(image:&Grid<T>, path:&str) -> Result<(), Box<dyn Error>> where Color:for<'a> From<&'a T> {
		image.to_png(path)
	}
}