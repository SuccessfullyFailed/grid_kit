#[cfg(test)]
mod tests {
	use crate::Grid;



	#[test]
	fn test_grid_properties() {
		let grid:Grid<char> = Grid::new(vec!['x', 'x', 'x', ' ', ' ', ' ', '1', '2', '3'], 3, 3);
		println!("{}", grid.to_string());
		
		assert_eq!([grid.width, grid.height], [3; 2]);
		assert_eq!([grid.width(), grid.height()], [3; 2]);
		assert_eq!(grid.data(), &['x', 'x', 'x', ' ', ' ', ' ', '1', '2', '3']);
		assert_eq!(&grid.data_2d(), &[['x', 'x', 'x'], [' ', ' ', ' '], ['1', '2', '3']]);
		assert_eq!(grid.len(), 9);
		assert_eq!(grid.is_empty(), false);
	}

	#[test]
	fn test_checkers_board() {
		let grid:Grid<char> = Grid::<char>::checkers_board(9, 3, 'x', ' ');
		println!("{}", grid.to_string());

		assert_eq!([grid.width, grid.height], [9 * 3; 2]);
		assert_eq!(grid.data_2d()[0], ['x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x']);
		assert_eq!(grid.data_2d()[1], ['x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x']);
		assert_eq!(grid.data_2d()[2], ['x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x']);
		assert_eq!(grid.data_2d()[3], [' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ']);
		assert_eq!(grid.data_2d()[4], [' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ']);
		assert_eq!(grid.data_2d()[5], [' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ']);
		assert_eq!(grid.data_2d()[6], ['x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x']);
		assert_eq!(grid.data_2d()[7], ['x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x']);
		assert_eq!(grid.data_2d()[8], ['x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x', ' ', ' ', ' ', 'x', 'x', 'x']);
	}
}