#[cfg(test)]
mod test {
	use file_ref::TempFile;
	use std::ops::Range;
	use crate::Grid;



	const TEST_GRID_SIZE:[usize; 2] = [16, 8];
	const TEST_DATA_SIZE:usize = TEST_GRID_SIZE[0] * TEST_GRID_SIZE[1];



	/* HELPER MACROS */

	macro_rules! test_grid {
		($type:ty, $grid:expr) => {
			let original_grid:Grid<$type> = $grid;
			println!("[original grid]\n{original_grid}\n");
			let file:TempFile = TempFile::new(Some("png"));
			original_grid.to_png(file.path()).unwrap();
			let validation_grid:Grid<$type> = Grid::from_png(file.path()).unwrap();
			println!("[validation grid]\n{validation_grid}\n");
			let colored_validation_grid:Grid<u32> = Grid::from_png(file.path()).unwrap();
			println!("[colored validation grid]\n{colored_validation_grid}\n");
	
			assert_eq!(original_grid, validation_grid);
		};
	}



	/* TEST METHODS */

	#[test]
	fn test_t_conversion_u32() {
		const RANGE:Range<u32> = u32::MIN..u32::MAX;

		test_grid!(u32, Grid::new((0..TEST_DATA_SIZE).map(|index| ((index as f64 / TEST_DATA_SIZE as f64 * (RANGE.end - RANGE.start) as f64) + RANGE.start as f64) as u32).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}

	#[test]
	fn test_t_conversion_u8() {
		const RANGE:Range<u8> = u8::MIN..u8::MAX;

		test_grid!(u8, Grid::new((0..TEST_DATA_SIZE).map(|index| ((index as f64 / TEST_DATA_SIZE as f64 * (RANGE.end - RANGE.start) as f64) + RANGE.start as f64) as u8).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}

	#[test]
	fn test_t_conversion_bool() {
		test_grid!(bool, Grid::new((0..TEST_DATA_SIZE).map(|index| index % 6 == 0).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}

	#[test]
	fn test_t_conversion_4bytes() {
		const RANGE:Range<u32> = u32::MIN..u32::MAX;

		let original_grid:Grid<[u8; 4]> = Grid::new((0..TEST_DATA_SIZE).map(|index| ((index as f64 / TEST_DATA_SIZE as f64 * (RANGE.end - RANGE.start) as f64) + RANGE.start as f64) as u32).map(|color| color.to_be_bytes()).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]);
		println!("[original grid]\n{}\n", original_grid.map_ref(|value| format!("{:?}", value)));
		let file:TempFile = TempFile::new(Some("png"));
		original_grid.to_png(file.path()).unwrap();
		let validation_grid:Grid<[u8; 4]> = Grid::from_png(file.path()).unwrap();
		println!("[validation grid]\n{}\n", validation_grid.map_ref(|value| format!("{:?}", value)));
		let colored_validation_grid:Grid<u32> = Grid::from_png(file.path()).unwrap();
		println!("[colored validation grid]\n{colored_validation_grid}\n");

		assert_eq!(original_grid, validation_grid);
	}
}