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
}