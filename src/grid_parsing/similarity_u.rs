#[cfg(test)]
mod test {
	use crate::Grid;



	#[test]
	fn test_similarity() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let compare_grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 9], 3, 3);
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(1.0 / 9.0 * 8.0, grid.similarity_to(&compare_grid));
	}

	#[test]
	fn test_similarity_custom_function() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let compare_grid:Grid<i32> = Grid::new(vec![0, 2, 3, 1, 5, 6, 7, 8, 10], 3, 3);
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(1.0 / 9.0 * 7.0, grid.similarity_to_using_method(&compare_grid, &|a, b| a.max(b) - a.min(b) < 2));
	}

	#[test]
	fn test_similarity_differently_sized() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let compare_grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7], 3, 3);
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(0.0, grid.similarity_to(&compare_grid));
	}
}