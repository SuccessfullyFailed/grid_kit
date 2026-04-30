use bytes_parser::BytesParser;
use std::error::Error;



pub(crate) struct FontMaxpProps {
	pub glyph_count:u16
}
impl FontMaxpProps {
	
	/// Try to create a new Maxp properties struct from the given parser.
	/// Expects the parser to be at the start of the Maxp table.
	pub fn new(table_parser:&mut BytesParser) -> Result<FontMaxpProps, Box<dyn Error>> {
		table_parser.skip(0x4);
		let glyph_count:u16 = table_parser.take()?;
		Ok(FontMaxpProps {
			glyph_count
		})
	}
}