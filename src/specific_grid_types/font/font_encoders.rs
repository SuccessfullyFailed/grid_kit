pub trait FontEncoder {
	fn glyph_id_for_char(&self, character:char) -> Option<u16>;
}



pub struct FontEncoderFormat4 {
	start_codes:Vec<u16>,
	end_codes:Vec<u16>,
	id_deltas:Vec<i16>,
	id_range_offsets:Vec<u16>,
	glyph_id_array:Vec<u16>
}
impl FontEncoderFormat4 {

	/// Create a new encoder.
	pub fn new(start_codes:Vec<u16>, end_codes:Vec<u16>, id_deltas:Vec<i16>, id_range_offsets:Vec<u16>, glyph_id_array:Vec<u16>) -> FontEncoderFormat4 {
		FontEncoderFormat4 { start_codes, end_codes, id_deltas, id_range_offsets, glyph_id_array }
	}
}
impl FontEncoder for FontEncoderFormat4 {

	/// Try to get a glyph ID for the given character.
	fn glyph_id_for_char(&self, character:char) -> Option<u16> {
		let character_code:u16 = character as u16;

		for code_index in 0..self.start_codes.len() {
			if character_code < self.start_codes[code_index] || character_code > self.end_codes[code_index] {
				continue;
			}

			let range_offset:u16 = self.id_range_offsets[code_index];
			if range_offset == 0 {
				return Some(((character_code as i32 + self.id_deltas[code_index] as i32) & 0xFFFF) as u16);
			}

			let glyph_id_index:usize = (range_offset as usize / 2) + (character_code - self.start_codes[code_index]) as usize - (self.id_range_offsets.len() - code_index);
			let gid:&u16 = self.glyph_id_array.get(glyph_id_index)?;
			if *gid == 0 {
				return None;
			}

			return Some(((*gid as i32 + self.id_deltas[code_index] as i32 ) & 0xFFFF) as u16);
		}

		None
	}
}