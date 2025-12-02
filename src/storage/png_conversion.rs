use image::{ io::Reader, ImageBuffer, Rgba, RgbaImage };
use std::{ error::Error, path::Path };
use crate::{ Grid, ColorConvertible, Color, ImageConversion };



impl<T> Grid<T> where T:ColorConvertible {

	/// Read from png file.
	pub fn from_png(path:&str) -> Result<Grid<T>, Box<dyn Error>> {
		if Path::new(path).exists() {
			let read_image:RgbaImage = Reader::open(path)?.decode()?.to_rgba8();
			let colors:Vec<Color> = read_image.pixels().map(|rgba| Color(u32::from_be_bytes([rgba[3], rgba[0], rgba[1], rgba[2]]))).collect::<Vec<Color>>();
			Ok(Grid::new(colors.into_iter().map(|color| T::from_color(color)).collect(), read_image.width() as usize, read_image.height() as usize))
		} else {
			Err(format!("Could not read png data from file '{path}'").into())
		}
	}

	/// Store the grid as a PNG.
	pub fn to_png(&self, file_path:&str) -> Result<(), Box<dyn Error>> {		
		let mut img:ImageBuffer<Rgba<_>, Vec<_>> = ImageBuffer::new(self.width as u32, self.height as u32);
		for (x, y, pixel) in img.enumerate_pixels_mut() {
			let mut color = self[(x as usize, y as usize)].to_color().0.to_be_bytes();
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
	fn image_from_file<T:ColorConvertible>(path:&str) -> Result<Grid<T>, Box<dyn Error>> {
		Grid::from_png(path)
	}

	/// Write an image to a file.
	fn image_to_file<T:ColorConvertible>(image:Grid<T>, path:&str) -> Result<(), Box<dyn Error>> {
		image.to_png(path)
	}
}