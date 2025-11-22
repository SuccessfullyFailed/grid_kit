use crate::{ Color, ColorConvertible, Grid };
use std::{ error::Error, fs, path::Path };
use bytes_parser::BytesParser;



const BMP_FILE_HEADER_SIZE:u32 = 14;
const BMP_FILE_SIGNATURE:[u8; 2] = [0x42, 0x4D];

const BMP_INFO_HEADER_SIZE:u32 = 40;



impl<T> Grid<T> where T:ColorConvertible {

	/// Read a grid from a BMP file.
	pub fn from_bmp(file_path:&str) -> Result<Grid<T>, Box<dyn Error>> {
		if !Path::new(file_path).exists() {
			return Err("File does not exist.".into());
		}
		Grid::from_bmp_bytes(fs::read(file_path)?)
	}

	/// Store the grid as a BMP file.
	pub fn to_bmp(&self, file_path:&str) -> Result<(), Box<dyn Error>> {
		fs::write(file_path, self.to_bmp_bytes())?;
		Ok(())
	}

	/// Read a grid from a BMP bytes list.
	pub(crate) fn from_bmp_bytes(bytes:Vec<u8>) -> Result<Grid<T>, Box<dyn Error>> {
		let mut parser:BytesParser = BytesParser::new(bytes, false);

		// Parse file header.
		if parser.take::<[u8; 2]>()? != BMP_FILE_SIGNATURE {
			return Err("File does not include BMP file header signature.".into());
		}
		let _full_file_bytes_size:u32 = parser.take()?;
		let _reserved_1:u16 = parser.take()?;
		let _reserved_2:u16 = parser.take()?;
		let pixel_data_offset:u32 = parser.take()?;

		// Parse BMP info header.
		if parser.take::<u32>()? != BMP_INFO_HEADER_SIZE {
			return Err("File contains incorrect BMP info header size.".into());
		}
		let width:u32 = parser.take()?;
		let height:i32 = parser.take()?;
		let top_down:bool = height < 0;
		let height:u32 = height.abs() as u32;
		let _amount_of_planes:u16 = parser.take()?;
		let bits_per_pixel:u16 = parser.take::<u16>()?;
		let bytes_per_pixel:u16 = bits_per_pixel / 8;
		let row_padding:u32 = (4 - ((width * bytes_per_pixel as u32) % 4)) % 4;
		let _compression:u32 = parser.take()?;
		let _image_size:u32 = parser.take()?;
		let _pixels_per_meter_x:u32 = parser.take()?;
		let _pixels_per_meter_y:u32 = parser.take()?;
		let _used_colors:u32 = parser.take()?;
		let _important_colors:u32 = parser.take()?;

		// Parse color data.
		if parser.cursor() as u32 != pixel_data_offset {
			return Err("Image color data was not at expected location.".into());
		}
		let mut colors:Vec<Vec<Color>> = Vec::new();
		match bits_per_pixel {
			32 => {
				for _y in 0..height {
					let mut row:Vec<Color> = Vec::with_capacity((width * bytes_per_pixel as u32) as usize);
					for _x in 0..width {
						let bbggrraa:[u8; 4] = parser.take()?;
						row.push(Color::new(u32::from_be_bytes([bbggrraa[3], bbggrraa[2], bbggrraa[1], bbggrraa[0]])));
					}
					colors.push(row);
					parser.skip(row_padding as usize);
				}
			},
			24 => {
				for _y in 0..height {
					let mut row:Vec<Color> = Vec::with_capacity((width * bytes_per_pixel as u32) as usize);
					for _x in 0..width {
						let bbggrr:[u8; 3] = parser.take()?;
						row.push(Color::new(u32::from_be_bytes([0xFF, bbggrr[2], bbggrr[1], bbggrr[0]])));
					}
					colors.push(row);
					parser.skip(row_padding as usize);
				}
			}
			_ => return Err(format!("Unexpected bits per pixel: {bits_per_pixel}.").into())
		};
		if !top_down {
			colors.reverse();
		}

		// Return the read data as a grid.
		Ok(Grid::new(colors.into_iter().flatten().map(|color| T::from_color(color)).collect(), width as usize, height as usize))
	}

	/// Convert the grid to BMP bytes.
	pub(crate) fn to_bmp_bytes(&self) -> Vec<u8> {

		// Prepare required variables.
		let full_file_bytes_size:u32 = BMP_FILE_HEADER_SIZE + BMP_INFO_HEADER_SIZE + (self.width * self.height * 4) as u32;
		let pixel_data_offset:u32 = BMP_FILE_HEADER_SIZE + BMP_INFO_HEADER_SIZE;
		let amount_of_planes:u16 = 1;
		let bits_per_pixel:u16 = 32;
		let bytes_per_pixel:u16 = bits_per_pixel / 3;
		let compression:u32 = 0;
		let image_data_size:u32 = (self.width + ((4 - self.width % 4)) % 4) as u32 * self.height as u32 * bytes_per_pixel as u32; // The data size of a all row's of data, including header.
		let pixels_per_meter_x:u32 = 1000;
		let pixels_per_meter_y:u32 = 1000;
		let used_colors:u32 = 0; // if bits per pixel is less than 8, the pixel data is a cursor to these colors.
		let important_colors:u32 = 0;

		// Create the file info header.
		let file_info_header:Vec<Vec<u8>> = vec![
			BMP_FILE_SIGNATURE.to_vec(),
			full_file_bytes_size.to_le_bytes().to_vec(),
			vec![0, 0], // Reserved 1.
			vec![0, 0], // Reserved 2.
			pixel_data_offset.to_le_bytes().to_vec()
		];

		// Create the bitmap info header.
		let bitmap_info_header:Vec<Vec<u8>> = vec![
			BMP_INFO_HEADER_SIZE.to_le_bytes().to_vec(),
			(self.width as u32).to_le_bytes().to_vec(),
			(-(self.height as i32)).to_le_bytes().to_vec(), // Positive = bottom-up, negative = top-down
			amount_of_planes.to_le_bytes().to_vec(),
			bits_per_pixel.to_le_bytes().to_vec(),
			compression.to_le_bytes().to_vec(),
			image_data_size.to_le_bytes().to_vec(),
			pixels_per_meter_x.to_le_bytes().to_vec(),
			pixels_per_meter_y.to_le_bytes().to_vec(),
			used_colors.to_le_bytes().to_vec(),
			important_colors.to_le_bytes().to_vec()
		];

		// Create the image data.
		let image_data:Vec<Vec<u8>> = self.data.iter().map(|color| color.to_color()).map(|color| vec![*color.b(), *color.g(), *color.r(), (color.0 >> 24) as u8]).collect();

		// Full list of bytes.
		vec![file_info_header, bitmap_info_header, image_data].into_iter().flatten().flatten().collect::<Vec<u8>>()
	}
}