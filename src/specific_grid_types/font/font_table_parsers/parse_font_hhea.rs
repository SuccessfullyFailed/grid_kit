use bytes_parser::BytesParser;
use std::error::Error;



pub(crate) struct FontHheaProps {
	pub _version:u32,
	pub ascent:i16,
	pub descent:i16,
	pub metrics_quantity:u16
}
impl FontHheaProps {
	
	/// Try to create a new Hhea properties struct from the given parser.
	/// Expects the parser to be at the start of the Hhea table.
	pub fn new(table_parser:&mut BytesParser) -> Result<FontHheaProps, Box<dyn Error>> {
		let _version:u32 = table_parser.take()?;
		let ascent:i16 = table_parser.take()?;
		let descent:i16 = table_parser.take()?;
		table_parser.skip(0x1A);
		let metrics_quantity:u16 = table_parser.take()?;
		Ok(FontHheaProps {
			_version,
			ascent,
			descent,
			metrics_quantity
		})
	}
}