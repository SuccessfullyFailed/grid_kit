use bytes_parser::BytesParser;
use std::error::Error;



pub(crate) struct FontLocaProps {
	pub glyph_offsets:Vec<usize>
}
impl FontLocaProps {
	
	/// Try to create a new Loca properties struct from the given parser.
	/// Expects the parser to be at the start of the Loca table.
	pub fn new(table_parser:&mut BytesParser, loca_format:u16, glyph_count:u16) -> Result<FontLocaProps, Box<dyn Error>> {
		let length:u16 = glyph_count + 1;
		let offset_size:usize = if loca_format == 0 { 2 } else { 4 };
		let glyph_offsets:Vec<usize> = table_parser.take_bytes(length as usize * offset_size)?.chunks(offset_size).map(|offset_bytes |
			if offset_size == 2 {
				u16::from_be_bytes(offset_bytes.try_into().unwrap()) as usize * 2
			} else {
				u32::from_be_bytes(offset_bytes.try_into().unwrap()) as usize
			}
		).collect();
		Ok(FontLocaProps {
			glyph_offsets
		})
	}
}