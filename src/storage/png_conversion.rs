use image::{ io::Reader, ImageBuffer, Rgba, RgbaImage };
use std::{ error::Error, path::Path };
use crate::Grid;



impl<T> Grid<T> where T:ColorConvertable {

	/// Read from png file.
	pub fn from_png(path:&str) -> Result<Grid<T>, Box<dyn Error>> {
		if Path::new(path).exists() {
			let read_image:RgbaImage = Reader::open(path)?.decode()?.to_rgba8();
			let colors:Vec<u32> = read_image.pixels().map(|rgba| u32::from_be_bytes([rgba[3], rgba[0], rgba[1], rgba[2]])).collect::<Vec<u32>>();
			Ok(Grid::new(colors.into_iter().map(|color| T::from_color(color)).collect(), read_image.width() as usize, read_image.height() as usize))
		} else {
			Err(format!("Could not read png data from file '{path}'").into())
		}
	}

	/// Store the grid as a PNG.
	pub fn to_png(&self, file_path:&str) -> Result<(), Box<dyn Error>> {		
		let mut img:ImageBuffer<Rgba<_>, Vec<_>> = ImageBuffer::new(self.width as u32, self.height as u32);
		for (x, y, pixel) in img.enumerate_pixels_mut() {
			let mut color = self[(x as usize, y as usize)].to_color().to_be_bytes();
			color.rotate_left(1);
			*pixel = Rgba(color);
		}
		img.save(file_path)?;
		Ok(())
	}
}



pub trait ColorConvertable {

	/// Convert the value to a 0xAARRGGBB color.
	fn to_color(&self) -> u32;

	/// Convert from color to self.
	fn from_color(color:u32) -> Self;
}
impl ColorConvertable for u32 {

	/// Convert the value to a 0xAARRGGBB color.
	fn to_color(&self) -> u32 {
		*self
	}

	/// Convert from color to self.
	fn from_color(color:u32) -> Self {
		color
	}
}
impl ColorConvertable for u8 {
	
	/// Convert the value to a 0xAARRGGBB color.
	fn to_color(&self) -> u32 {
		u32::from_be_bytes([0xFF, *self, *self, *self])
	}

	/// Convert from color to self.
	fn from_color(color:u32) -> Self {
		(color & 0xFF) as u8
	}
}
impl ColorConvertable for bool {
	
	/// Convert the value to a 0xAARRGGBB color.
	fn to_color(&self) -> u32 {
		if *self { 0xFF00FF00 } else { 0x00000000 }
	}

	/// Convert from color to self.
	fn from_color(color:u32) -> Self {
		color != 0x000000
	}
}
impl ColorConvertable for [u8; 4] {
	
	/// Convert the value to a 0xAARRGGBB color.
	fn to_color(&self) -> u32 {
		u32::from_be_bytes(*self)
	}

	/// Convert from color to self.
	fn from_color(color:u32) -> Self {
		color.to_be_bytes()
	}
}