use bytes_parser::BytesParser;
use std::error::Error;



pub(crate) struct FontHeadProps {
	pub units_per_em:u16,
	pub loca_format:u16
}
impl FontHeadProps {
	
	/// Try to create a new Head properties struct from the given parser.
	/// Expects the parser to be at the start of the Head table.
	pub fn new(table_parser:&mut BytesParser) -> Result<FontHeadProps, Box<dyn Error>> {
		table_parser.skip(0x12);
		let units_per_em:u16 = table_parser.take()?;
		table_parser.skip(0x1E);
		let loca_format:u16 = table_parser.take()?;
		Ok(FontHeadProps {
			units_per_em,
			loca_format
		})
	}
}