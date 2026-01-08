use std::{ error::Error, fmt::{ Debug, Display }, ops::{Add, AddAssign} };
use crate::Grid;



const BYTE_MAX_F32:f32 = 0xFF as f32;



pub type Image = Grid<Color>;



#[derive(Clone, Copy, PartialEq, Default)]
pub struct Color(pub u32); // 0xAARRGGBB.
impl Color {

	/* CONSTRUCTOR METHODS */

	/// Create a new 0xAARRGGBB color.
	pub fn new<T>(source:T) -> Color where T:ColorConvertible + Send + Sync + 'static {
		source.to_color()
	}



	/* PROPERTY GETTER METHODS */

	/// Create reference to part of the value of self.
	fn value_pointer<U>(&self, offset:usize) -> &U {
		let ptr:*const U = unsafe { (&self.0 as *const u32 as *const U).add(offset) };
		unsafe { &*ptr }
	}

	/// Create reference to part of the value of self.
	fn value_pointer_mut<U>(&mut self, offset:usize) -> &mut U {
		let ptr:*mut U = unsafe { (&mut self.0 as *mut u32 as *mut U).add(offset) };
		unsafe { &mut *ptr }
	}

	/// Get the opacity of the color.
	pub fn a(&self) -> &u8 {
		self.value_pointer(3)
	}

	/// Get the opacity of the color.
	pub fn r(&self) -> &u8 {
		self.value_pointer(2)
	}

	/// Get the opacity of the color.
	pub fn g(&self) -> &u8 {
		self.value_pointer(1)
	}

	/// Get the opacity of the color.
	pub fn b(&self) -> &u8 {
		self.value_pointer(0)
	}

	/// Get the opacity of the color.
	pub fn a_mut(&mut self) -> &mut u8 {
		self.value_pointer_mut(3)
	}

	/// Get the opacity of the color.
	pub fn r_mut(&mut self) -> &mut u8 {
		self.value_pointer_mut(2)
	}

	/// Get the opacity of the color.
	pub fn g_mut(&mut self) -> &mut u8 {
		self.value_pointer_mut(1)
	}

	/// Get the opacity of the color.
	pub fn b_mut(&mut self) -> &mut u8 {
		self.value_pointer_mut(0)
	}

	/// Get the shade of the color.
	pub fn shade(&self) -> u8 {
		(self.0.to_be_bytes()[1..].iter().map(|val| *val as u16).sum::<u16>() / 3) as u8
	}
}
impl Display for Color {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:#010x}", self.0)
	}
}
impl Debug for Color {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:#010x}", self.0)
	}
}
impl<T:ColorConvertible> Add<T> for Color {
	type Output = Color;

	fn add(self, rhs:T) -> Self::Output {
		let rhs:Color  = rhs.to_color();

		// Edge cases that require no merging.
		let rhs_opacity:u32 = rhs.0 >> 24;
		if rhs_opacity == 0xFF {
			return rhs;
		}
		if rhs_opacity == 0 {
			return self;
		}
		let self_opacity:u32 = self.0 >> 24;
		if self_opacity == 0 {
			return rhs;
		}

		// Calculate combined opacity factor.
		let self_opacity:f32 = self_opacity as f32 / BYTE_MAX_F32;
		let rhs_opacity:f32 = rhs_opacity as f32 / BYTE_MAX_F32;
		let rhs_opacity_neg:f32 = 1.0 - rhs_opacity;
		let bg_alpha_factor:f32 = self_opacity * rhs_opacity_neg;

		// Combine colors into new color using porter-duff "over".
		let self_rgb:Vec<f32> = self.0.to_be_bytes()[1..].into_iter().map(|value| *value as f32 / BYTE_MAX_F32).collect();
		let rhs_rgb:Vec<f32>= rhs.0.to_be_bytes()[1..].into_iter().map(|value| *value as f32 / BYTE_MAX_F32).collect();
		Color::new(
		[
				rhs_opacity + bg_alpha_factor,
				rhs_rgb[0] * rhs_opacity + self_rgb[0] * bg_alpha_factor,
				rhs_rgb[1] * rhs_opacity + self_rgb[1] * bg_alpha_factor,
				rhs_rgb[2] * rhs_opacity + self_rgb[2] * bg_alpha_factor
			].map(|value| (value * BYTE_MAX_F32) as u8)
		)
	}
}
impl<T:ColorConvertible> AddAssign<T> for Color {
	fn add_assign(&mut self, rhs:T) {
		*self = *self + rhs;
	}
}



pub trait ColorConvertible:Send + Sync + 'static {

	/// Convert the value to a 0xAARRGGBB color.
	fn to_color(&self) -> Color;

	/// Convert from color to self.
	fn from_color(color:Color) -> Self;
}
impl ColorConvertible for Color {
	fn to_color(&self) -> Color {
		*self
	}
	fn from_color(color:Color) -> Self {
		color
	}
}
impl ColorConvertible for u32 {
	fn to_color(&self) -> Color {
		Color(*self)
	}
	fn from_color(color:Color) -> Self {
		color.0
	}
}
impl ColorConvertible for u8 {
	fn to_color(&self) -> Color {
		Color(u32::from_be_bytes([0xFF, *self, *self, *self]))
	}
	fn from_color(color:Color) -> Self {
		color.shade()
	}
}
impl ColorConvertible for bool {
	fn to_color(&self) -> Color {
		Color(if *self { 0xFF00FF00 } else { 0x00000000 })
	}
	fn from_color(color:Color) -> Self {
		color.0 != 0x000000
	}
}
impl ColorConvertible for [u8; 4] {
	fn to_color(&self) -> Color {
		Color(u32::from_be_bytes(*self))
	}
	fn from_color(color:Color) -> Self {
		color.0.to_be_bytes()
	}
}



pub trait ImageConversion {

	/// The file extension required for conversion.
	fn file_extension() -> &'static str;

	/// Read an image from a file.
	fn image_from_file<T:ColorConvertible>(path:&str) -> Result<Grid<T>, Box<dyn Error>>;

	/// Write an image to a file.
	fn image_to_file<T:ColorConvertible>(image:Grid<T>, path:&str) -> Result<(), Box<dyn Error>>;
}