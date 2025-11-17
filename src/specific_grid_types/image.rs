use std::fmt::{ Debug, Display };
use crate::Grid;



type Image = Grid<Color>;
impl Image {

	/// Capture a specific part of the screen. Bounds are XYWH.
	#[cfg(feature="screen_capture")]
	pub fn screen_capture(bounds:[i32; 4]) -> Result<Image, Box<dyn std::error::Error>> {
		use winapi::{ ctypes::c_void, shared::{ minwindef::DWORD, windef::{ HBITMAP__, HDC__ } }, um::{ wingdi, winuser } };
		use std::{ mem, ptr::null_mut };

		unsafe {
		
			// Create a device context.
			let dc:*mut HDC__ = winuser::GetDC(null_mut());
			if dc.is_null() {
				return Err("Could not create device context".into());
			}

			// Create compatible device context.
			let hdc:*mut HDC__ = wingdi::CreateCompatibleDC(dc);
			if hdc.is_null() {
				winuser::ReleaseDC(null_mut(), dc);
				return Err("Could not create compatible device context.".into())
			}

			// Create a compatible bitmap.
			let hbitmap:*mut HBITMAP__ = wingdi::CreateCompatibleBitmap(dc, bounds[2], bounds[3]);
			if hbitmap.is_null() {
				wingdi::DeleteDC(hdc);
				winuser::ReleaseDC(null_mut(), dc);
				return Err("Could not create compatible bitmap.".into())
			}
			
			// Select the bitmap into the DC.
			let hold:*mut c_void = wingdi::SelectObject(hdc, hbitmap as *mut _);
			if hold.is_null() {
				wingdi::DeleteObject(hbitmap as *mut _);
				wingdi::DeleteDC(hdc);
				winuser::ReleaseDC(null_mut(), dc);
				return Err("Could not select the bitmap in the device context.".into())
			}
			
			// BitBlt to capture the screen content.
			let result:i32 = wingdi::BitBlt(hdc, -bounds[0], -bounds[1], bounds[0] + bounds[2], bounds[1] + bounds[3], dc, 0, 0, 0x00CC0020);
			if result == 0 {
				wingdi::DeleteDC(hdc);
				winuser::ReleaseDC(null_mut(), dc);
				return Err("Image from screen result is 0.".into());
			}
			
			// Get the pixel data using GetDIBits.
			let mut bitmap_info:wingdi::BITMAPINFO = mem::zeroed();
			bitmap_info.bmiHeader.biSize = mem::size_of::<wingdi::BITMAPINFOHEADER>() as DWORD;
			bitmap_info.bmiHeader.biWidth = bounds[2];
			bitmap_info.bmiHeader.biHeight = -bounds[3];
			bitmap_info.bmiHeader.biPlanes = 1;
			bitmap_info.bmiHeader.biBitCount = 32;
			bitmap_info.bmiHeader.biCompression = wingdi::BI_RGB;
			
			// Create a list of bits.
			let mut bits:Vec<u8> = vec![0; (bounds[2] * bounds[3] * 4) as usize];
			bits.resize((bounds[2] * bounds[3] * 4) as usize, 0u8);
			let result:i32 = wingdi::GetDIBits(hdc, hbitmap, 0, bounds[3] as u32, bits.as_mut_ptr() as *mut c_void, &mut bitmap_info, wingdi::DIB_RGB_COLORS);
			if result == 0 {
				wingdi::DeleteDC(hdc);
				winuser::ReleaseDC(null_mut(), dc);
				return Err("GetDIBits failed.".into());
			}
			
			// Convert the raw pixel data to the desired format.
			let mut pixels:Vec<Color> = vec![Color(0x00000000); (bounds[2] * bounds[3]) as usize];
			for pixel_index in 0..(bounds[2] * bounds[3]) as usize {
				pixels[pixel_index] = [0xFF, bits[pixel_index * 4 + 2], bits[pixel_index * 4 + 1], bits[pixel_index * 4]].to_color();
			}
			
			// Cleanup.
			wingdi::SelectObject(hdc, hold);
			wingdi::DeleteObject(hbitmap as *mut _);
			wingdi::DeleteDC(hdc);
			winuser::ReleaseDC(null_mut(), dc);
			
			// Return image-buffer.
			Ok(Image::new(pixels, bounds[2] as usize, bounds[3] as usize))
		}
	}
}
impl<T> Grid<T> where T:ColorConvertible {

	/// Convert the grid to an image.
	pub fn to_image(&self) -> Image {
		self.map_ref(|source| source.to_color())
	}
}



#[derive(Clone, Copy, PartialEq, Default)]
pub struct Color(pub u32);
impl Color {

	/* CONSTRUCTOR METHODS */

	/// Create a new 0xAARRGGBB color.
	pub fn new<T>(source:T) -> Color where T:ColorConvertible {
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
pub trait ColorConvertible {

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