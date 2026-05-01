use super::font_table_parsers::*;
use bytes_parser::BytesParser;
use file_ref::FileRef;
use std::error::Error;
use crate::Grid;



type Line = [(f32, f32); 2];



impl Grid<f32> {

	/// Paint a text in a grid.
	pub fn draw_str(text:&str, font:&Font, line_height:usize) -> Grid<f32> {
		font.render_text_grid(text, line_height).map(|v| if v { 1.0 } else { 0.0 })
	}
}
impl Grid<bool> {

	/// Paint a text in a grid.
	pub fn draw_str(str:&str, font:&Font, line_height:usize) -> Grid<bool> {
		Grid::<f32>::draw_str(str, font, line_height).map(|value| value > 0.25)
	}
}



pub struct Font {
	head:FontHeadProps,
	hhea:FontHheaProps,
	hmtx:FontHmtxProps,
	cmap:FontCmapProps,
	glyf:FontGlyfProps
}
impl Font {

	/* CONSTRUCTOR METHODS */

	/// Create a new font reference.
	/// Can take the raw bytes of the TTF file or a path to the file.
	pub fn new<Source:TryInto<Font>>(source:Source) -> Result<Font, Box<dyn Error>> where Source::Error:Into<Box<dyn Error>> {
		match source.try_into() {
			Ok(font) => Ok(font),
			Err(error) => Err(error.into())
		}
	}

	/// Create a new font reference from raw ttf contents.
	fn from_contents(ttf_contents:Vec<u8>) -> Result<Font, Box<dyn Error>> {
		const SFNT_VERSION_TT:u32 = 0x00010000;
		
		// Parse font header.
		let mut parser:BytesParser = BytesParser::new(ttf_contents.clone(), true);
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
			table_parsers.push((tag, table_address, BytesParser::new(ttf_contents[table_address..table_address + table_size].to_vec(), true)));
		}

		// Parse tables in prefered order.
		const TABLE_PARSE_ORDER:&[&str] = &["head", "maxp", "hhea", "hmtx", "loca", "cmap", "glyf"];
		let mut head:Option<FontHeadProps> = None;
		let mut maxp:Option<FontMaxpProps> = None;
		let mut hhea:Option<FontHheaProps> = None;
		let mut hmtx:Option<FontHmtxProps> = None;
		let mut loca:Option<FontLocaProps> = None;
		let mut cmap:Option<FontCmapProps> = None;
		let mut glyf:Option<FontGlyfProps> = None;
		for target_tag in TABLE_PARSE_ORDER {
			for (_, table_address, table_parser) in table_parsers.iter_mut().filter(|(tag, _, _)| tag == target_tag) {
				match *target_tag {
					"head" => {
						if let Ok(parsed_head) = FontHeadProps::new(table_parser) {
							head = Some(parsed_head);
						}
					},

					"maxp" => {
						if let Ok(parsed_maxp) = FontMaxpProps::new(table_parser) {
							maxp = Some(parsed_maxp);
						}
					},

					"hhea" => {
						if let Ok(parsed_hhea) = FontHheaProps::new(table_parser) {
							hhea = Some(parsed_hhea);
						}
					},

					"hmtx" => {
						if let Some(maxp) = &maxp {
							if let Some(hhea) = &hhea {
								if let Ok(parsed_hmtx) = FontHmtxProps::new(table_parser, maxp.glyph_count, hhea.metrics_quantity as usize) {
									hmtx = Some(parsed_hmtx);
								}
							}
						}
					},

					"loca" => {
						if let Some(head) = &head {
							if let Some(maxp) = &maxp {
								if let Ok(parsed_loca) = FontLocaProps::new(table_parser, head.loca_format, maxp.glyph_count) {
									loca = Some(parsed_loca);
								}
							}
						}
					},

					"cmap" => {
						if let Ok(parsed_cmap) = FontCmapProps::new(table_parser, &ttf_contents, *table_address) {
							cmap = Some(parsed_cmap);
						}
					},

					"glyf" => {
						if let Some(loca) = &loca {
							if let Ok(parsed_glyf) = FontGlyfProps::new(&ttf_contents, *table_address, &loca.glyph_offsets) {
								glyf = Some(parsed_glyf);
							}
						}
					},

					_ => {}
				}
			}
		}

		// Return errors for missing data.
		if head.is_none() {
			return Err("Could not parse TrueType font Head.".into());
		}
		if hhea.is_none() {
			return Err("Could not parse TrueType font Hhea.".into());
		}
		if cmap.is_none() {
			return Err("Could not parse TrueType font Cmap.".into());
		}
		if glyf.is_none() {
			return Err("Could not parse TrueType font Glyf.".into());
		}

		// Return the parsed props.
		let metrics_quantity:u16 = hhea.as_ref().unwrap().metrics_quantity;
		Ok(Font {
			head: head.unwrap(),
			hhea: hhea.unwrap(),
			hmtx: hmtx.unwrap_or(FontHmtxProps::zeroed(metrics_quantity as usize)),
			cmap: cmap.unwrap(),
			glyf: glyf.unwrap()
		})
	}



	/* USAGE METHODS */

	/// Create a grid where each pixel is the opacity of the text at that position.
	pub fn render_text_grid(&self, text:&str, line_height:usize) -> Grid<bool> {
		let scale:f32 = line_height as f32 / self.head.units_per_em as f32;
		let ascent:i32 = (self.hhea.ascent as f32 * scale) as i32;
		let descent:i32 = (self.hhea.descent as f32 * scale) as i32;
		let height:usize = (ascent - descent) as usize;

		// Estimate the width and height of the grid.
		let mut total_width:usize = 0;
		let total_height:usize = height + ((text.split('\n').count() - 1) * line_height);
		let mut cursor_x:usize = 0;
		let mut glyph_indices:Vec<(char, usize)> = Vec::new();
		for character in text.chars() {
			if character == '\n' {
				cursor_x = 0;
				total_width += line_height;
				glyph_indices.push((character, 0));
			} else {
				let glyph_index:usize = self.index_for_character(character);
				glyph_indices.push((character, glyph_index));
				cursor_x += (self.hmtx.metrics[glyph_index].advance_width as f32 * scale) as usize;
				if cursor_x > total_width {
					total_width = cursor_x;
				}
			}
		}

		// Draw all glyphs onto the grid.
		let mut grid:Grid<bool> = Grid::new(vec![false; total_width * total_height], total_width, total_height);
		let mut cursor:[i32; 2] = [0, ascent as i32];
		for (character, glyph_index) in &glyph_indices {
			if *character == '\n' {
				cursor = [0, cursor[1] + line_height as i32];
			} else {
				let lines:Vec<Line> = self.build_glyph_lines(*glyph_index, scale);
				self.draw_lines_on_canvas(&lines, &mut grid, cursor[0], cursor[1]);
				cursor[0] += (self.hmtx.metrics[*glyph_index].advance_width as f32 * scale) as i32;
			}
		}

		// Return the created grid.
		grid
	}

	/// Find the index of the given character.
	/// Tries to find it in any available encoders.
	/// Returns 0 when no character is found.
	fn index_for_character(&self, character:char) -> usize {
		for encoder in &self.cmap.encoders {
			if let Some(index) = encoder.glyph_id_for_char(character) {
				return index as usize;
			}
		}
		0
	}

	/// Get the lines necessary to draw the glyph of the given index.
	/// The lines are scaled to the given scale.
	fn build_glyph_lines(&self, glyph_index: usize, scale:f32) -> Vec<Line> {
		let mut lines:Vec<Line> = Vec::new();

		// If this glyph is a simple glyph, simply collect and scale all required lines to the lines list.
		if let Some((_, _, contours)) = self.glyf.contours.iter().find(|(simple_index, _, _)| *simple_index == glyph_index) {
			for contour in contours {
				for contour_point_index in 0..contour.len() {
					let point_a:ContourPoint = contour[contour_point_index];
					let point_b:ContourPoint = contour[(contour_point_index + 1) % contour.len()];
					lines.push([
						(point_a.x as f32 * scale, point_a.y as f32 * scale),
						(point_b.x as f32 * scale, point_b.y as f32 * scale)
					]);
				}
			}
		}

		// If this glyph is a compositve glyph, collect and scale all sub-glyphs to the lines list.
		if let Some((_, sub_glyphs)) = self.glyf.composites.iter().find(|(compisite_index, _)| *compisite_index == glyph_index) {
			for (sub_glyph_index, sub_glyph_transform) in sub_glyphs {
				for line in self.build_glyph_lines(*sub_glyph_index, scale) {
					let point_a:ContourPoint = sub_glyph_transform.apply(&ContourPoint::new(line[0].0 as i16, line[0].1 as i16, true));
					let point_b:ContourPoint = sub_glyph_transform.apply(&ContourPoint::new(line[1].0 as i16, line[1].1 as i16, true));
					lines.push([
						(point_a.x as f32, point_a.y as f32),
						(point_b.x as f32, point_b.y as f32)
					]);
				}
			}
		}

		// Return all collected lines.
		lines
	}

	/// Draw the given lines on the given canvas.
	fn draw_lines_on_canvas(&self, lines:&[Line], canvas:&mut Grid<bool>, offset_x:i32, baseline:i32) {
		let width:i32 = canvas.width as i32;
		let height:i32 = canvas.height as i32;
		for cursor_y in 0..height {
			for cursor_x in 0..width {
				let glyph_x:i32 = cursor_x - offset_x;
				let glyph_y:i32 = baseline - cursor_y;
				if self.point_is_in_shape(glyph_x as f32, glyph_y as f32, lines) {
					canvas[(cursor_x as usize, cursor_y as usize)] = true;
				}
			}
		}
	}
	
	/// Wether or not the given point is in the shape defined by the given lines.
	/// Instead of checking if the coordinate is "on" the line, does the following:
	/// 	Loop over each each line.
	/// 	Count the amount of lines overlap the line from the cursor directly to the left.
	/// 	If the amount of crossed lines is an odd number, we are currently inside the shape.
	fn point_is_in_shape(&self, cursor_x:f32, cursor_y:f32, lines:&[Line]) -> bool {
		let mut line_crossings:usize = 0;
		for line in lines {
			let (a_x, a_y) = line[0];
			let (b_x, b_y) = line[1];
			
			let y_overlaps:bool = (a_y > cursor_y) != (b_y > cursor_y); // Either of the ends is above/below the point and the other is on the opposite side of it.
			
			let cursor_y_offset_from_a:f32 = cursor_y - a_y;
			let line_end_offset_y:f32 = b_y - a_y;
			let y_progress_factor:f32 = cursor_y_offset_from_a / line_end_offset_y;
			let intesection_edge_x:f32 = a_x + (b_x - a_x) * y_progress_factor;
			let to_left_of_edge:bool = cursor_x < intesection_edge_x;

			if y_overlaps && to_left_of_edge {
				line_crossings += 1;
			}
		}
		line_crossings % 2 == 1
	}
}



impl TryInto<Font> for Vec<u8> {
	type Error = Box<dyn Error>;
	fn try_into(self) -> Result<Font, Self::Error> {
		Font::from_contents(self)
	}
}
impl TryInto<Font> for FileRef {
	type Error = Box<dyn Error>;
	fn try_into(self) -> Result<Font, Self::Error> {
		self.read_bytes()?.try_into()
	}
}
impl TryInto<Font> for String {
	type Error = Box<dyn Error>;
	fn try_into(self) -> Result<Font, Self::Error> {
		FileRef::new(&self).try_into()
	}
}
impl TryInto<Font> for &str {
	type Error = Box<dyn Error>;
	fn try_into(self) -> Result<Font, Self::Error> {
		FileRef::new(&self).try_into()
	}
}