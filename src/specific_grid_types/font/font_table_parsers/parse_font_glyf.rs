use bytes_parser::BytesParser;
use std::error::Error;



#[derive(Clone, Copy)]
pub(crate) struct ContourPoint {
	pub x:i16,
	pub y:i16,
	pub is_on_curve:bool
}
impl ContourPoint {

	/// Create a new contour point.
	pub fn new(x:i16, y:i16, is_on_curve:bool) -> ContourPoint {
		ContourPoint {
			x,
			y,
			is_on_curve
		}
	}
}



#[derive(Clone)]
pub(crate) struct Transform {
	translate_x:f32,
	translate_y:f32,
	scale_x:f32,
	scale_y:f32,
	rotation_x_to_y:f32,
	rotation_y_to_x:f32
}
impl Transform {

	/// Create a new transform set.
	pub fn new(scale_x:f32, rotation_x_to_y:f32, rotation_y_to_x:f32, scale_y:f32, translate_x:f32, translate_y:f32) -> Transform {
		Transform {
			scale_x,
			rotation_x_to_y,
			rotation_y_to_x,
			scale_y,
			translate_x,
			translate_y
		}
	}

	/// Apply the transform to the given contour point.
	pub fn apply(&self, contour_point:&ContourPoint) -> ContourPoint {
		ContourPoint {
			x: (self.scale_x * contour_point.x as f32 + self.rotation_y_to_x * contour_point.y as f32 + self.translate_x).round() as i16,
			y: (self.rotation_x_to_y * contour_point.x as f32 + self.scale_y * contour_point.y as f32 + self.translate_y).round() as i16,
			is_on_curve: contour_point.is_on_curve,
		}
	}
}
impl Default for Transform {
	fn default() -> Self {
		Transform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
	}
}



pub(crate) struct FontGlyfProps {
	pub contours:Vec<(usize, [i16; 4], Vec<Vec<ContourPoint>>)>,
	pub composites:Vec<(usize, Vec<(usize, Transform)>)>
}
impl FontGlyfProps {
	
	/// Try to create a new Glyf properties struct from the raw file contents and address.
	/// Requires the original file contents and address of the table to parse other tables with an offset to this one.
	pub fn new(file_contents:&[u8], table_address:usize, glyph_offsets:&[usize]) -> Result<FontGlyfProps, Box<dyn Error>> {
		const HALF_I16_MAX_F32:f32 = 16384.0; 
		
		let mut contours:Vec<(usize, [i16; 4], Vec<Vec<ContourPoint>>)> = Vec::new();
		let mut composites:Vec<(usize, Vec<(usize, Transform)>)> = Vec::new();
		for glyph_index in 0..glyph_offsets.len() {
			if glyph_index < glyph_offsets.len() - 1 && glyph_offsets[glyph_index] == glyph_offsets[glyph_index + 1] {
				continue;
			}

			let glyph_offset:usize = glyph_offsets[glyph_index];
			let mut glyph_parser:BytesParser = BytesParser::new(file_contents[table_address + glyph_offset..].to_vec(), true);
			let contour_quantity:i16 = glyph_parser.take()?;
			let x_min:i16 = glyph_parser.take()?;
			let y_min:i16 = glyph_parser.take()?;
			let x_max:i16 = glyph_parser.take()?;
			let y_max:i16 = glyph_parser.take()?;

			// Skip empty glyphs.
			if contour_quantity == 0 {
				continue;
			}
			
			// Parse composite glyph.
			if contour_quantity < 0 {
				let mut components:Vec<(usize, Transform)> = Vec::new();
				let mut have_instructions:bool = false;
				let mut more_components:bool = true;
				while more_components {
					let flags:u16 = glyph_parser.take()?;
					let glyph_index:u16 = glyph_parser.take()?;
					let args_are_i16:bool = flags & 0x01 == 0x01;
					let args_are_xy_value:bool = flags & 0x02 == 0x02;
					let has_scale:bool = flags & 0x08 == 0x08;
					let has_x_and_y_scale:bool = flags & 0x40 == 0x40;
					let has_two_by_two:bool = flags & 0x80 == 0x80;
					have_instructions |= flags & 0x0100 != 0;
					more_components = flags & 0x20 == 0x20;

					// Read arguments.
					let (translate_x, translate_y) = {
						if args_are_xy_value {
							if args_are_i16 {
								(glyph_parser.take::<i16>()? as f32, glyph_parser.take::<i16>()? as f32)
							} else {
								(glyph_parser.take::<i8>()? as f32, glyph_parser.take::<i8>()? as f32)
							}
						} else {
							// Matched point numbers, ignore for now.
							glyph_parser.skip(if args_are_i16 { 4 } else { 2 });
							(0.0, 0.0)
						}
					};

					// Read transform.
					let transform:Transform = {
						if has_two_by_two {
							Transform::new(glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32,
								glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32,
								glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32,
								glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32,
								translate_x,
								translate_y
							)
						} else if has_x_and_y_scale {
							Transform::new(
								glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32,
								0.0,
								0.0,
								glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32,
								translate_x,
								translate_y
							)
						} else if has_scale {
							let scale:f32 = glyph_parser.take::<i16>()? as f32 / HALF_I16_MAX_F32;
							Transform::new(
								scale,
								0.0,
								0.0,
								scale,
								translate_x,
								translate_y
							)
						} else {
							Transform {
								translate_x,
								translate_y,
								..Transform::default()
							}
						}
					};
					components.push((glyph_index as usize, transform));
				}

				// Ignore other instructions for now.
				if have_instructions {
					let instruction_len:u16 = glyph_parser.take()?;
					glyph_parser.skip(instruction_len as usize);
				}

				composites.push((glyph_index, components));
			}

			// Parse non-composite glyph.
			else {

				// Parse instruction data.
				let contour_end_point_indices:Vec<u16> = glyph_parser.take_many(contour_quantity as usize)?;
				let instruction_byte_count:u16 = glyph_parser.take()?;
				glyph_parser.skip(instruction_byte_count as usize);

				// Get all the flags of this specific point.
				let glyph_point_quantity:u16 = contour_end_point_indices.last().unwrap() + 1;
				let mut glyph_point_flags:Vec<u8> = Vec::with_capacity(glyph_point_quantity as usize);
				while glyph_point_flags.len() < glyph_point_quantity as usize {
					let flag:u8 = glyph_parser.take()?;
					glyph_point_flags.push(flag);

					// If bit 3 is set, repeat the flag.
					if flag & 0x08 != 0 {
						let repeat_count:u8 = glyph_parser.take()?;
						for _ in 0..repeat_count {
							glyph_point_flags.push(flag);
						}
					}
				}

				// Get the X coordinates of all points.
				let mut x_coordinates:Vec<i16> = Vec::with_capacity(glyph_point_quantity as usize);
				let mut current_x:i16 = 0;
				for flag in &glyph_point_flags {
					current_x += {
						if flag & 0x02 != 0 { (glyph_parser.take::<u8>()? as i16) * (if flag & 0x10 != 0 { 1 } else { -1 }) } // 1-byte vector.
						else if flag & 0x10 != 0 { 0 } // Same x as previous output.
						else { glyph_parser.take()? } // 2-byte delta.
					};
					x_coordinates.push(current_x);
				}

				// Get the Y coordinates of all points.
				let mut y_coordinates:Vec<i16> = Vec::with_capacity(glyph_point_quantity as usize);
				let mut current_y:i16 = 0;
				for flag in &glyph_point_flags {
					current_y += {
						if flag & 0x04 != 0 { (glyph_parser.take::<u8>()? as i16) * (if flag & 0x20 != 0 { 1 } else { -1 }) } // 1-byte vector.
						else if flag & 0x20 != 0 { 0 } // Same y as previous output.
						else { glyph_parser.take()? } // 2-byte delta.
					};
					y_coordinates.push(current_y);
				}

				// Combine X and Y coordinates into contour points.
				let contour_points:Vec<ContourPoint> = {
					(0..glyph_point_quantity as usize).map(|point_index| 
						ContourPoint {
							x: x_coordinates[point_index],
							y: y_coordinates[point_index],
							is_on_curve: glyph_point_flags[point_index] & 0x01 != 0
						}
					).collect()
				};

				// Build contours list from contour points.
				let mut glyph_contours:Vec<Vec<ContourPoint>> = Vec::new();
				let mut start:usize = 0;
				for end in contour_end_point_indices {
					glyph_contours.push(contour_points[start..=end as usize].to_vec());
					start = end as usize + 1;
				}
				contours.push((glyph_index, [x_min, y_min, (x_max - x_min).max(0), (y_max - y_min).max(0)], glyph_contours));
			}
		}

		Ok(FontGlyfProps {
			contours,
			composites
		})
	}
}