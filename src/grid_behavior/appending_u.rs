#[cfg(test)]
mod tests {
	use crate::Grid;


	#[test]
	fn test_append() {
		let mut original_grid:Grid<i32> = Grid::new(vec![0; 25], 5, 5);
		println!("[original grid]\n{original_grid}\n");
		let appending_grid:Grid<i32> = Grid::new(vec![2; 4], 2, 2);
		println!("[appending grid]\n{appending_grid}\n");
		original_grid.append(&appending_grid);
		println!("[modified grid]\n{original_grid}\n");

		assert_eq!([original_grid.width, original_grid.height], [5, 5]);
		assert_eq!(original_grid.data(), &[2, 2, 0, 0, 0,	2, 2, 0, 0, 0,	0, 0, 0, 0, 0,	0, 0, 0, 0, 0,	0, 0, 0, 0, 0]);
	}

	#[test]
	fn test_append_at() {
		let mut original_grid:Grid<i32> = Grid::new(vec![0; 25], 5, 5);
		println!("[original grid]\n{original_grid}\n");
		let appending_grid:Grid<i32> = Grid::new(vec![2; 4], 2, 2);
		println!("[appending grid]\n{appending_grid}\n");
		original_grid.append_at(&appending_grid, (1, 2));
		println!("[modified grid]\n{original_grid}\n");

		assert_eq!([original_grid.width, original_grid.height], [5, 5]);
		assert_eq!(original_grid.data(), &[0, 0, 0, 0, 0,	0, 0, 0, 0, 0,	0, 2, 2, 0, 0,	0, 2, 2, 0, 0,	0, 0, 0, 0, 0]);
	}

	#[test]
	fn test_append_at_overflow() {
		let mut original_grid:Grid<i32> = Grid::new(vec![0; 25], 5, 5);
		println!("[original grid]\n{original_grid}\n");
		let appending_grid:Grid<i32> = Grid::new(vec![2; 4], 2, 2);
		println!("[appending grid]\n{appending_grid}\n");
		original_grid.append_at(&appending_grid, (4, 4));
		println!("[modified grid]\n{original_grid}\n");

		assert_eq!([original_grid.width, original_grid.height], [5, 5]);
		assert_eq!(original_grid.data(), &[0, 0, 0, 0, 0,	0, 0, 0, 0, 0,	0, 0, 0, 0, 0,	0, 0, 0, 0, 0,	0, 0, 0, 0, 2]);
	}

	#[test]
	fn test_rasterize() {
		let original_grid:Grid<Grid<i32>> = Grid::new((0..9).map(|index| Grid::new(vec![index; 4], 2, 2)).collect(), 3, 3);
		println!("[original grid]\n{original_grid}\n");
		let flattened_grid:Grid<i32> = original_grid.flatten_grid();
		println!("[flattened grid]\n{flattened_grid}\n");
		
		assert_eq!(flattened_grid.width, 6);
		assert_eq!(flattened_grid.height, 6);
		assert_eq!(
			flattened_grid.data_2d(),
			[
				[0, 0, 1, 1, 2, 2],
				[0, 0, 1, 1, 2, 2],
				[3, 3, 4, 4, 5, 5],
				[3, 3, 4, 4, 5, 5],
				[6, 6, 7, 7, 8, 8],
				[6, 6, 7, 7, 8, 8]
			]
		);
	}

	#[test]
	fn test_rasterize_mismatched_sizes() {
		let original_grid:Grid<Grid<i32>> = Grid::new(
			vec![
				Grid::new(vec![1; 9], 3, 3),
				Grid::new(vec![2; 6], 3, 2),
				Grid::new(vec![3; 2], 2, 1),
				Grid::new(vec![4; 9], 3, 3)
			],
			2,
			2
		);
		println!("[original grid]\n{original_grid}\n");
		let flattened_grid:Grid<i32> = original_grid.flatten_grid();
		println!("[flattened grid]\n{flattened_grid}\n");
		
		assert_eq!(flattened_grid.width, 6);
		assert_eq!(flattened_grid.height, 6);
		assert_eq!(
			flattened_grid.data_2d(),
			[
				[1, 1, 1, 2, 2, 2],
				[1, 1, 1, 2, 2, 2],
				[1, 1, 1, 0, 0, 0],
				[3, 3, 0, 4, 4, 4],
				[0, 0, 0, 4, 4, 4],
				[0, 0, 0, 4, 4, 4]
			]
		);
	}
}