use bytes_parser::BytesParser;
use file_ref::FileRef;
use std::error::Error;
use crate::{ Grid, FontEncoder, FontEncoderFormat4 };




#[derive(Clone, Copy)]
struct Line {
	start_x:f32,
	start_y:f32,
	end_x:f32,
	end_y:f32
}



#[derive(Clone, Copy)]
struct ContourPoint {
	x:i16,
	y:i16,
	on_curve:bool
}



pub struct Font {
	units_per_em:u16,
	encoders:Vec<Box<dyn FontEncoder>>,
	contours:Vec<(usize, Vec<Vec<ContourPoint>>)>
}
impl Font {

	/* CONSTRUCTOR METHODS */

	/// Read a new font from a file.
	pub fn new(ttf_file_path:&str) -> Result<Font, Box<dyn Error>> {
		const SFNT_VERSION_TT:u32 = 0x00010000;

		// Read file and create parser.
		let file_contents:Vec<u8> = FileRef::new(ttf_file_path).read_bytes()?;
		let mut parser:BytesParser = BytesParser::new(file_contents.clone(), true);

		// Parse font header.
		let sfnt_version:u32 = parser.take()?;
		if sfnt_version != SFNT_VERSION_TT {
			return Err("Passed file does not contains TrueType sfnt version.".into());
		}
		let table_quantity:u16 = parser.take()?;
		let _search_range:u16 = parser.take()?;
		let _entry_selector:u16 = parser.take()?;
		let _range_shift:u16 = parser.take()?;

		// Create table parsers.
		let mut table_parsers:Vec<(String, usize, BytesParser)> = Vec::new();
		for _table_index in 0..table_quantity {
			let tag:String = parser.take::<[u8; 4]>()?.map(|byte| byte as char).iter().collect::<String>();
			let _checksum:u32 = parser.take()?;
			let table_address:usize = parser.take::<u32>()? as usize;
			let table_size:usize = parser.take::<u32>()? as usize;
			table_parsers.push((tag, table_address, BytesParser::new(file_contents[table_address..table_address + table_size].to_vec(), true)));
		}

		// Parse tables in prefered order.
		const TABLE_PARSE_ORDER:&[&str] = &["head", "maxp", "loca", "cmap", "glyf"];
		let mut loca_format:u16 = 0;
		let mut glyph_count:u16 = 0;
		let mut units_per_em:u16 = 0;
		let mut glyph_offsets:Vec<usize> = Vec::new();
		let mut encoders:Vec<Box<dyn FontEncoder>> = Vec::new();
		let mut contours:Vec<(usize, Vec<Vec<ContourPoint>>)> = Vec::new();
		for target_tag in TABLE_PARSE_ORDER {
			for (_, table_address, table_parser) in table_parsers.iter_mut().filter(|(tag, _, _)| tag == target_tag) {
				match *target_tag {
					"head" => {
						table_parser.skip(0x12);
						units_per_em = table_parser.take()?;
						table_parser.skip(0x10);
						loca_format = table_parser.take()?;
					},
					"maxp" => {
						table_parser.skip(0x4);
						glyph_count = table_parser.take()?;
					},
					"loca" => {
						let length:u16 = glyph_count + 1;
						let offset_size:usize = if loca_format == 0 { 2 } else { 4 };
						glyph_offsets = table_parser.take_bytes(length as usize * offset_size)?.chunks(offset_size).map(|offset_bytes |
							if offset_size == 2 {
								u16::from_be_bytes(offset_bytes.try_into().unwrap()) as usize * 2
							} else {
								u32::from_be_bytes(offset_bytes.try_into().unwrap()) as usize
							}
						).collect();
					},
					"cmap" => {
						let _version:u16 = table_parser.take()?;
						let encoding_record_quantity:u16 = table_parser.take()?;
						for _encoding_record_index in 0..encoding_record_quantity as usize {
							let platform_id:u16 = table_parser.take()?;
							let encoding_id:u16 = table_parser.take()?;
							let offset:usize = table_parser.take::<u32>()? as usize;

							let mut record_parser:BytesParser = BytesParser::new(file_contents[*table_address + offset..].to_vec(), true);
							let format:u16 = record_parser.take()?;

							// Windows BITmap.
							if platform_id == 3 && encoding_id == 1 && format == 4 {
								let length:u16 = record_parser.take()?;
								let _language:u16 = record_parser.take()?;
								let segment_quantity:usize = record_parser.take::<u16>()? as usize / 2;
								let _search_range:u16 = record_parser.take()?;
								let _entry_selector:u16 = record_parser.take()?;
								let _range_shift:u16 = record_parser.take()?;

								let end_codes:Vec<u16> = record_parser.take_many(segment_quantity)?;
								let _reserved_pad:u16 = record_parser.take()?;
								let start_codes:Vec<u16> = record_parser.take_many(segment_quantity)?;
								let id_deltas:Vec<i16> = record_parser.take_many(segment_quantity)?;
								let id_range_offsets:Vec<u16> = record_parser.take_many(segment_quantity)?;
								let glyph_id_array:Vec<u16> = record_parser.take_many((length as usize - record_parser.cursor()) / 2)?;

								encoders.push(Box::new(
									FontEncoderFormat4::new(start_codes, end_codes, id_deltas, id_range_offsets, glyph_id_array)
								));
							}
						}
					},
					"glyf" => {
						for glyph_index in 0..glyph_offsets.len() {
							if glyph_index < glyph_offsets.len() - 1 && glyph_offsets[glyph_index] == glyph_offsets[glyph_index + 1] {
								continue;
							}
							let glyph_offset:usize = glyph_offsets[glyph_index];
							let mut glyph_parser:BytesParser = BytesParser::new(file_contents[*table_address + glyph_offset..].to_vec(), true);
							let contour_quantity:i16 = glyph_parser.take()?;
							let _x_min:i16 = glyph_parser.take()?;
							let _y_min:i16 = glyph_parser.take()?;
							let _x_max:i16 = glyph_parser.take()?;
							let _y_max:i16 = glyph_parser.take()?;

							// Skip empty glyphs.
							if contour_quantity == 0 {
								continue;
							}
							// Skip composite glyphs for now.
							if contour_quantity < 0 {
								continue;
							}

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

									// 1-byte vector.
									if flag & 0x02 != 0 {
										(glyph_parser.take::<u8>()? as i16) * (if flag & 0x10 != 0 { 1 } else { -1 })
									}
									
									// Same x as previous output.
									else if flag & 0x10 != 0 {
										0
									}
									
									// 2-byte delta.
									else {
										glyph_parser.take()?
									}
								};
								x_coordinates.push(current_x);
							}

							// Get the Y coordinates of all points.
							let mut y_coordinates:Vec<i16> = Vec::with_capacity(glyph_point_quantity as usize);
							let mut current_y:i16 = 0;
							for flag in &glyph_point_flags {
								current_y += {

									// 1-byte vector.
									if flag & 0x04 != 0 {
										(glyph_parser.take::<u8>()? as i16) * (if flag & 0x20 != 0 { 1 } else { -1 })
									}
									
									// Same y as previous output.
									else if flag & 0x20 != 0 {
										0
									}
									
									// 2-byte delta.
									else {
										glyph_parser.take()?
									}
								};
								y_coordinates.push(current_y);
							}

							// Combine X and Y coordinates into contour points.
							let contour_points:Vec<ContourPoint> = {
								(0..glyph_point_quantity as usize).map(|point_index| 
									ContourPoint {
										x: x_coordinates[point_index],
										y: y_coordinates[point_index],
										on_curve: glyph_point_flags[point_index] & 0x01 != 0
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
							contours.push((glyph_index, glyph_contours));
						}
					}
					_ => {}
				}
			}
		}

		// Return font.
		Ok(
			Font {
				units_per_em,
				encoders,
				contours
			}
		)
	}



	/* USAGE METHODS */

	/// Render a string to a list of characters as a 2D list of pixels.
	pub fn draw_str(&self, str:&str, line_height:usize) -> Grid<Grid<bool>> {

		// Split string into lines.
		let lines:Vec<Vec<char>> = str.split('\n').map(|line| line.chars().collect::<Vec<char>>()).collect();
		let max_line_size:usize = lines.iter().map(|line| line.len()).max().unwrap_or_default();
		let line_count:usize = lines.len();

		// Create and return a grid containing a Grid<bool> for each character.
		Grid::new(
			(0..line_count).map(|y| 
				(0..max_line_size).map(|x| 
					if x < lines[y].len() {
						self.draw_char(lines[y][x], line_height)
					} else {
						Grid::new(Vec::new(), 0, 0)
					}
				).collect::<Vec<Grid<bool>>>()
			).flatten().collect(),
			max_line_size,
			line_count
		)
	}

	/// Render a character to a grid of pixels.
	pub fn draw_char(&self, character:char, line_height:usize) -> Grid<bool> {

		// Get contours for character.
		let glyph_id:usize = self.encoders.iter().find_map(|e| e.glyph_id_for_char(character)).unwrap_or(0) as usize;
		let contours:&Vec<Vec<ContourPoint>> = match self.contours.iter().find(|(id, _)| *id == glyph_id) {
			Some((_, contours)) => contours,
			None => return Grid::new(vec![false; line_height * line_height], line_height, line_height)
		};

		// Create a list of lines from the contours.
		let scale:f32 = line_height as f32 / self.units_per_em as f32;
		let mut lines:Vec<Line> = Vec::new();
		for contour in contours {
			Self::contour_points_to_edges(contour, scale, &mut lines);
		}

		// Fill the outlines, creating a character grid.
		Self::outlines_to_pixels(line_height, &lines)
	}



	/* HELPER METHODS */

	/// Turn a list of contour points into a list of lines, creating edges to the vertices of the character.
	fn contour_points_to_edges(contour:&[ContourPoint], scale:f32, output_list:&mut Vec<Line>) {
		let mut contour_points:Vec<ContourPoint> = contour.to_vec();
		contour_points.push(contour[0]); // Wrap back to beginning, filling the outline.

		// Loop through sets of points, drawing a line between each set of points.
		let mut contour_point_index:usize = 0;
		while contour_point_index + 1 < contour_points.len() {
			let start_point:ContourPoint = contour_points[contour_point_index];
			let end_point:ContourPoint = contour_points[contour_point_index + 1];

			// If both of the points are along a curve, simply draw a line.
			if start_point.on_curve && end_point.on_curve {
				output_list.push(Line {
					start_x: start_point.x as f32 * scale,
					start_y: start_point.y as f32 * scale,
					end_x: end_point.x as f32 * scale,
					end_y: end_point.y as f32 * scale
				});
				contour_point_index += 1;
			}
			
			// If not both of the points are along a curve, draw a list of small lines following a bezier curve.
			else {
				let control_point:ContourPoint = end_point;
				let end_point:ContourPoint = contour_points[contour_point_index + 2];
				output_list.extend(Self::bezier_to_lines(start_point, control_point, end_point, scale));
				contour_point_index += 2;
			}
		}
	}

	/// Splits a quadratic bezier curve into a list of small lines.
	fn bezier_to_lines(start_point:ContourPoint, control_point:ContourPoint, end_point:ContourPoint, scale:f32) -> Vec<Line> {
		let mut lines:Vec<Line> = Vec::new();
		let segment_count:usize = 16;

		// Loop through segment points, skipping the first.
		let mut previous_x:f32 = start_point.x as f32;
		let mut previous_y:f32 = start_point.y as f32;
		for segment_index in 1..=segment_count {
			let progress_factor:f32 = segment_index as f32 / segment_count as f32;
			let negative_progress_factor:f32 = 1.0 - progress_factor;

			// Get current coordinates in the quadratic bezier.
			let current_x:f32 = negative_progress_factor * negative_progress_factor * start_point.x as f32 + 2.0 * negative_progress_factor * progress_factor * control_point.x as f32 + progress_factor * progress_factor * end_point.x as f32;
			let current_y:f32 = negative_progress_factor * negative_progress_factor * start_point.y as f32 + 2.0 * negative_progress_factor * progress_factor * control_point.y as f32 + progress_factor * progress_factor * end_point.y as f32;

			// Create a line from the previous position to this one.
			lines.push(Line {
				start_x: previous_x * scale,
				start_y: previous_y * scale,
				end_x: current_x * scale,
				end_y: current_y * scale
			});

			// Move on to the next segment.
			previous_x = current_x;
			previous_y = current_y;
		}

		// Return list of lines.
		lines
	}

	/// Create a pixel-grid from a list of lines.
	fn outlines_to_pixels(line_height:usize, outline_lines:&[Line]) -> Grid<bool> {
		let mut pixel_grid:Grid<bool> = Grid::new(vec![false; line_height * line_height], line_height, line_height);

		// Loop through rows in the bitmap.
		for pixel_row in 0..line_height {
			let scanline_y:f32 = pixel_row as f32 + 0.5; // The center of the pixel-row.

			// For each outline that intersects the scanline, find the X coordinate of the intersection.
			let mut intersection_x_positions:Vec<f32> = Vec::new();
			for line in outline_lines {
				let line_start_y:f32 = line.start_y;
				let line_end_y:f32   = line.end_y;

				// Skip outlines that do not cross the scanline.
				if !((line_start_y <= scanline_y && line_end_y > scanline_y) || (line_end_y   <= scanline_y && line_start_y > scanline_y)) {
					continue;
				}

				// Get the X coordinate where the line intersect the scanline and add it to the intersection positions.
				let line_intersection_factor:f32 = (scanline_y - line_start_y) / (line_end_y - line_start_y);
				let intersection_x:f32 = line.start_x + line_intersection_factor * (line.end_x - line.start_x);
				intersection_x_positions.push(intersection_x);
			}

			// Sort intersections from left to right.
			intersection_x_positions.sort_by(|a, b| a.partial_cmp(b).unwrap());

			// Fill pixels between each set of two intersections.
			for intersection_pair in intersection_x_positions.chunks(2) {
				if intersection_pair.len() != 2 {
					continue;
				}
				let fill_start_x:isize = intersection_pair[0].floor() as isize;
				let fill_end_x:isize   = intersection_pair[1].ceil()  as isize;
				for pixel_x in fill_start_x..fill_end_x {
					if pixel_x >= 0 && pixel_x < line_height as isize {
						let pixel_y:usize = line_height - pixel_row - 1;
						pixel_grid[[pixel_x as usize, pixel_y]] = true;
					}
				}
			}
		}

		// Return filled pixel grid.
		pixel_grid
	}
}



impl Grid<bool> {

	/// Paint a character in a grid.
	pub fn draw_char(character:char, font:&Font, line_height:usize) -> Grid<bool> {
		font.draw_char(character, line_height)
	}

	/// Paint a text in a grid.
	pub fn draw_str(str:&str, font:&Font, line_height:usize) -> Grid<Grid<bool>> {
		font.draw_str(str, line_height)
	}
}