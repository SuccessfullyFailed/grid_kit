#[cfg(test)]
mod tests {
	use crate::{ Grid, Mask };



	#[test]
	fn test_create_basic_mask() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("{:?}", grid);
		let mask:Mask = grid.create_mask(|value| value % 3 == 1);
		println!("{:?}", mask);

		assert_eq!(mask.data, vec![false, true, false, false, true, false, false, true, false]);
	}

	#[test]
	fn test_create_value_mask() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("{:?}", grid);
		let mask:Mask = grid.create_value_mask(3);
		println!("{:?}", mask);

		assert_eq!(mask.data, vec![false, false, false, true, false, false, false, false, false]);
	}

	#[test]
	fn test_apply_mask() {
		let mut grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("{:?}\n", grid);
		let mask:Mask = grid.create_mask(|value| *value >= 5);
		println!("{:?}\n", mask);
		grid.apply_mask(&mask);
		println!("{:?}\n", grid);

		assert_eq!(grid.data, vec![0, 0, 0, 0, 0, 5, 6, 7, 8]);
	}

	#[test]
	fn test_create_and_apply_mask() {
		let mut grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("{:?}\n", grid);
		grid.mask(|value| *value >= 5);
		println!("{:?}\n", grid);

		assert_eq!(grid.data, vec![0, 0, 0, 0, 0, 5, 6, 7, 8]);
	}
}