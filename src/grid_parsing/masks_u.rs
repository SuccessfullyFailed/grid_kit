#[cfg(test)]
mod tests {
	use crate::{ Grid, GridMask };



	#[test]
	fn test_create_basic_mask() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid]\n{grid}\n");
		let mask:GridMask = grid.create_mask(|value| value % 3 == 1);
		let mask_grid:&Grid<bool> = mask.grid();
		println!("[mask]\n{mask_grid}\n");

		assert_eq!(mask_grid.data, vec![false, true, false, false, true, false, false, true, false]);
	}

	#[test]
	fn test_create_value_mask() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid]\n{grid}\n");
		let mask:GridMask = grid.create_value_mask(3);
		let mask_grid:&Grid<bool> = mask.grid();
		println!("[mask]\n{mask_grid}\n");

		assert_eq!(mask_grid.data, vec![false, false, false, true, false, false, false, false, false]);
	}

	#[test]
	fn test_apply_mask() {
		let mut grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid before mask]\n{grid}\n");
		let mask:GridMask = grid.create_mask(|value| *value >= 5);
		let mask_grid:&Grid<bool> = mask.grid();
		println!("[mask]\n{mask_grid}\n");
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

	#[test]
	fn test_mask_ranges() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid]\n{grid}\n");
		let mask:GridMask = grid.create_mask(|value| value % 3 == 1);
		let mask_grid:&Grid<bool> = mask.grid();
		println!("[mask]\n{mask_grid}\n");

		assert_eq!(mask_grid.data, vec![false, true, false, false, true, false, false, true, false]);
		assert_eq!(mask.positive_ranges(), &[1..2, 4..5, 7..8]);
		assert_eq!(mask.negative_ranges(), &[0..1, 2..4, 5..7, 8..9]);
	}
}