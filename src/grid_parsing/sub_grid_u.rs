#[cfg(test)]
mod tests {
	use crate::Grid;



	#[test]
	fn test_sub_grid_size_and_data() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<&i32> = grid.sub_grid([0, 0, 2, 2]);
		println!("[compare grid]\n{sub_grid}\n");

		assert_eq!(sub_grid.width, 2);
		assert_eq!(sub_grid.height, 2);
		assert_eq!(sub_grid.data, vec![&0, &1, &3, &4]);
	}

	#[test]
	fn test_full_sub_grid_size_and_data() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<&i32> = grid.full_sub_grid();
		println!("[compare grid]\n{sub_grid}\n");

		assert_eq!(sub_grid.width, 3);
		assert_eq!(sub_grid.height, 3);
		assert_eq!(sub_grid.data, grid.data.iter().collect::<Vec<&i32>>());
	}

	#[test]
	fn test_cloned_sub_grid_size_and_data() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<i32> = grid.take([1, 1, 2, 2]);
		println!("[compare grid]\n{sub_grid}\n");

		assert_eq!(sub_grid.width, 2);
		assert_eq!(sub_grid.height, 2);
		assert_eq!(sub_grid.data, vec![4, 5, 7, 8]);
	}
}