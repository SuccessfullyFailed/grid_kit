use crate::{ FontEncoder, FontEncoderFormat4 };
use bytes_parser::BytesParser;
use std::error::Error;



pub(crate) struct FontCmapProps {
	pub _version:u16,
	pub encoders:Vec<Box<dyn FontEncoder + 'static>>
}
impl FontCmapProps {

	/// Try to create a new Cmap properties struct from the given parser.
	/// Expects the parser to be at the start of the Cmap table.
	/// Requires the original file contents and address of the table to parse other tables with an offset to this one.
	pub fn new(table_parser:&mut BytesParser, file_contents:&[u8], table_address:usize) -> Result<FontCmapProps, Box<dyn Error>> {
		let mut encoders:Vec<Box<dyn FontEncoder>> = Vec::new();
		let _version:u16 = table_parser.take()?;
		let encoding_record_quantity:u16 = table_parser.take()?;
		for _encoding_record_index in 0..encoding_record_quantity as usize {
			let platform_id:u16 = table_parser.take()?;
			let encoding_id:u16 = table_parser.take()?;
			let offset:usize = table_parser.take::<u32>()? as usize;

			let mut record_parser:BytesParser = BytesParser::new(file_contents[table_address + offset..].to_vec(), true);
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
		Ok(FontCmapProps {
			_version,
			encoders
		})
	}
}