#[cfg(test)]
mod tests {
	use crate::{Grid, GridCursor};



	fn sample_grid() -> Grid<u8> {
		Grid::new(
			vec![
				1, 2, 3,
				4, 5, 6,
				7, 8, 9
			],
			3,
			3
		)
	}

	#[test]
	fn new_cursor_has_correct_position() {
		let grid:Grid<u8> = sample_grid();
		let cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 1, 2);

		assert_eq!(cursor.x, 1);
		assert_eq!(cursor.y, 2);
	}

	#[test]
	fn within_bounds_returns_true_for_valid_position() {
		let grid:Grid<u8> = sample_grid();
		let cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 2, 2);

		assert!(cursor.within_bounds());
	}

	#[test]
	fn within_bounds_returns_false_for_invalid_position() {
		let grid:Grid<u8> = sample_grid();

		let cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 3, 0);
		assert!(!cursor.within_bounds());

		let cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 0, 3);
		assert!(!cursor.within_bounds());
	}

	#[test]
	fn displace_to_moves_cursor() {
		let grid:Grid<u8> = sample_grid();
		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 0, 0);

		cursor.displace_to(2, 1);

		assert_eq!(cursor.x, 2);
		assert_eq!(cursor.y, 1);
	}

	#[test]
	fn displace_applies_offset() {
		let grid:Grid<u8> = sample_grid();
		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 1, 1);

		cursor.displace(1, 1);

		assert_eq!(cursor.x, 2);
		assert_eq!(cursor.y, 2);
	}

	#[test]
	fn displace_to_bounds_clamps_out_of_bounds_position() {
		let grid:Grid<u8> = sample_grid();
		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 10, 10);

		cursor.displace_to_bounds();

		assert_eq!(cursor.x, 2);
		assert_eq!(cursor.y, 2);
	}

	#[test]
	fn displace_to_bounds_does_not_modify_valid_position() {
		let grid:Grid<u8> = sample_grid();
		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 1, 1);

		cursor.displace_to_bounds();

		assert_eq!(cursor.x, 1);
		assert_eq!(cursor.y, 1);
	}

	#[test]
	fn displace_while_moves_while_filter_matches() {
		let grid:Grid<u8> = Grid::new(vec![1, 1, 1, 0], 4, 1);

		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 0, 0);

		cursor.displace_while(1, 0, |value| *value == 1);

		// Stops after stepping onto the first non-matching value.
		assert_eq!(cursor.x, 3);
		assert_eq!(cursor.y, 0);
	}

	#[test]
	fn displace_while_does_nothing_if_initially_out_of_bounds() {
		let grid:Grid<u8> = sample_grid();
		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 10, 10);

		cursor.displace_while(1, 0, |_| true);

		assert_eq!(cursor.x, 10);
		assert_eq!(cursor.y, 10);
	}

	#[test]
	fn displace_while_stops_at_negative_boundary() {
		let grid:Grid<u8> = Grid::new(vec![1, 1, 1], 3, 1);

		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 1, 0);

		cursor.displace_while(-1, 0, |value| *value == 1);

		// Final valid in-bounds coordinate before break.
		assert_eq!(cursor.x, 0);
		assert_eq!(cursor.y, 0);
	}

	#[test]
	fn displace_while_can_move_vertically() {
		let grid:Grid<u8> = Grid::new(vec![1, 1, 1, 1], 1, 4);

		let mut cursor:GridCursor<'_, u8> = GridCursor::new(&grid, 0, 0);

		cursor.displace_while(0, 1, |value| *value == 1);

		assert_eq!(cursor.x, 0);
		assert_eq!(cursor.y, 3);
	}
}