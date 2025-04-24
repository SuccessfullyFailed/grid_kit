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
			let file:TempFile = TempFile::new(Some("dat"));
			original_grid.save_to_file(file.path()).unwrap();
			let validation_grid:Grid<$type> = Grid::read_from_file(file.path()).unwrap();
			println!("[validation grid]\n{original_grid}\n");
	
			assert_eq!(original_grid, validation_grid);
		};
	}

	macro_rules! test_int_grid {
		($type:ty, $type_size:expr) => {
			const MAX_SHIFT:usize = $type_size * 8;

			test_grid!($type, Grid::new((0..TEST_DATA_SIZE).map(|index| 1 << (index as f32 / TEST_DATA_SIZE as f32 * MAX_SHIFT as f32) as usize).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
		};
	}



	/* INTEGER TEST METHODS */

	#[test]
	fn test_t_conversion_u8() {
		test_int_grid!(u8, 1);
	}

	#[test]
	fn test_t_conversion_u16() {
		test_int_grid!(u16, 2);
	}

	#[test]
	fn test_t_conversion_u32() {
		test_int_grid!(u32, 4);
	}

	#[test]
	fn test_t_conversion_u64() {
		test_int_grid!(u64, 8);
	}

	#[test]
	fn test_t_conversion_u128() {
		test_int_grid!(u128, 16);
	}

	#[test]
	fn test_t_conversion_i8() {
		test_int_grid!(i8, 1);
	}

	#[test]
	fn test_t_conversion_i16() {
		test_int_grid!(i16, 2);
	}

	#[test]
	fn test_t_conversion_i32() {
		test_int_grid!(i32, 4);
	}

	#[test]
	fn test_t_conversion_i64() {
		test_int_grid!(i64, 8);
	}

	#[test]
	fn test_t_conversion_i128() {
		test_int_grid!(i128, 16);
	}



	/* FLOATING POINT TEST METHODS */

	#[test]
	fn test_t_conversion_f32() {
		const RANGE:Range<f32> = -1000000.0..1000000.0;

		test_grid!(f32, Grid::new((0..TEST_DATA_SIZE).map(|index| (index as f32 / TEST_DATA_SIZE as f32 * (RANGE.end - RANGE.start)) + RANGE.start).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}

	#[test]
	fn test_t_conversion_f64() {
		const RANGE:Range<f64> = -10000000000.0..10000000000.0;
		
		test_grid!(f64, Grid::new((0..TEST_DATA_SIZE).map(|index| (index as f64 / TEST_DATA_SIZE as f64 * (RANGE.end - RANGE.start)) + RANGE.start).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}



	/* MISCELLANEOUS TEST METHODS */

	#[test]
	fn test_t_conversion_bool() {
		test_grid!(bool, Grid::new((0..TEST_DATA_SIZE).map(|index| index % 3 == 0 && index % 8 != 0).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}

	#[test]
	fn test_t_conversion_string() {
		const EXAMPLE_STR:&str = "QWERTYUIOPASDFGHJKLZXCVBNM";

		test_grid!(String, Grid::new((0..TEST_DATA_SIZE).map(|index| EXAMPLE_STR.split_at((index as f32 / TEST_DATA_SIZE as f32 * 22.0) as usize).1.to_string()).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]));
	}

	#[test]
	fn test_t_conversion_array() {
		const MAX_SHIFT:usize = 16;
		const ARRAY_SIZE:usize = 3;

		let original_grid:Grid<[u16; ARRAY_SIZE]> = Grid::new((0..TEST_DATA_SIZE).map(|index| [1 << (index as f32 / TEST_DATA_SIZE as f32 * MAX_SHIFT as f32) as u16, 900, 20]).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]);
		println!("[original grid]\n{}\n", original_grid.map_ref(|value| format!("{:?}", value)));
		let bytes:Vec<u8> = original_grid.to_bytes();
		let validation_grid:Grid<[u16; ARRAY_SIZE]> = Grid::from_bytes(&bytes).unwrap();
		println!("[validation grid]\n{}\n", validation_grid.map_ref(|value| format!("{:?}", value)));

		assert_eq!(original_grid, validation_grid);
	}

	#[test]
	fn test_t_conversion_list() {
		const MAX_SHIFT:usize = 16;
		
		let original_grid:Grid<Vec<u16>> = Grid::new((0..TEST_DATA_SIZE).map(|index| vec![1 << (index as f32 / TEST_DATA_SIZE as f32 * MAX_SHIFT as f32) as u16, 900, 20]).collect(), TEST_GRID_SIZE[0], TEST_GRID_SIZE[1]);
		println!("[original grid]\n{}\n", original_grid.map_ref(|value| format!("{:?}", value)));
		let bytes:Vec<u8> = original_grid.to_bytes();
		let validation_grid:Grid<Vec<u16>> = Grid::from_bytes(&bytes).unwrap();
		println!("[validation grid]\n{}\n", validation_grid.map_ref(|value| format!("{:?}", value)));

		assert_eq!(original_grid, validation_grid);
	}
}