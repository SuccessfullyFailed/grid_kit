#[cfg(test)]
mod tests {
	use crate::{ Grid, Mask };



	#[test]
	fn test_create_basic_mask() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid]\n{grid}\n");
		let mask:Mask = grid.create_mask(|value| value % 3 == 1);
		println!("[mask]\n{mask}\n");

		assert_eq!(mask.data, vec![false, true, false, false, true, false, false, true, false]);
	}

	#[test]
	fn test_create_value_mask() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid]\n{grid}\n");
		let mask:Mask = grid.create_value_mask(3);
		println!("[mask]\n{mask}\n");

		assert_eq!(mask.data, vec![false, false, false, true, false, false, false, false, false]);
	}

	#[test]
	fn test_apply_mask() {
		let mut grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid before mask]\n{grid}\n");
		let mask:Mask = grid.create_mask(|value| *value >= 5);
		println!("[mask]\n{mask}\n");
		grid.apply_mask(&mask);
		println!("[grid after mask]\n{grid}\n");

		assert_eq!(grid.data, vec![0, 0, 0, 0, 0, 5, 6, 7, 8]);
	}

	#[test]
	fn test_create_and_apply_mask() {
		let mut grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid before mask]\n{grid}\n");
		grid.mask(|value| *value >= 5);
		println!("[grid after mask]\n{grid}\n");

		assert_eq!(grid.data, vec![0, 0, 0, 0, 0, 5, 6, 7, 8]);
	}
}