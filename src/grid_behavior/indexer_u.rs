#[cfg(test)]
mod tests {
	use crate::Grid;


	#[test]
	fn test_grid_indexers() {
		let grid:Grid<[usize; 2]> = Grid::new_2d(
			(0..10).map(|y| (0..10).map(|x| [x, y]).collect::<Vec<[usize; 2]>>()).collect(),
			10,
			10
		);
		println!("{:?}", grid);

		for y in 0..10 {
			for x in 0..10 {
				let index:usize = grid.xy_to_index(x, y);
				println!("{x}, {y} ({index})");

				assert_eq!(grid.xy_is_valid(x, y), true);
				assert_eq!(grid[(x, y)], [x, y]);
				assert_eq!(grid[index], [x, y]);
			}
		}

		assert_eq!(grid.xy_is_valid(10, 0), false);
		assert_eq!(grid.xy_is_valid(0, 10), false);
		assert_eq!(grid.xy_is_valid(10, 10), false);
		assert_eq!(grid.xy_is_valid(10, 100), false);
	}
}