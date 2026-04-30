use bytes_parser::BytesParser;
use std::error::Error;



pub(crate) type FontHmtxMetrics = Vec<FontHmtxMetric>;

#[derive(Clone)]
pub(crate) struct FontHmtxMetric {
	pub advance_width:u16,
	pub _left_side_bearing:i16
}



pub(crate) struct FontHmtxProps {
	pub metrics:FontHmtxMetrics
}
impl FontHmtxProps {
	
	/// Try to create a new Hmtx properties struct from the given parser.
	/// Expects the parser to be at the start of the Hmtx table.
	pub fn new(table_parser:&mut BytesParser, glyph_count:u16, metrics_quantity:usize) -> Result<FontHmtxProps, Box<dyn Error>> {
		let mut metrics:Vec<FontHmtxMetric> = Vec::new(); // Advance width, Left side bearing
		for _metrics_index in 0..metrics_quantity {
			metrics.push(FontHmtxMetric {
				advance_width: table_parser.take()?,
				_left_side_bearing: table_parser.take()?
			});
		}
		while metrics.len() < glyph_count as usize {
			metrics.push(FontHmtxMetric {
				advance_width: metrics.last().map(|prev| prev.advance_width).unwrap_or_default(),
				_left_side_bearing: table_parser.take()?
			});
		}
		Ok(FontHmtxProps {
			metrics
		})
	}
	pub fn zeroed(metrics_quantity:usize) -> FontHmtxProps {
		FontHmtxProps {
			metrics: vec![FontHmtxMetric { advance_width: 0, _left_side_bearing: 0 }; metrics_quantity]
		}
	}
}