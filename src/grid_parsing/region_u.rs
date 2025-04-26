#[cfg(test)]
mod test {
	use crate::{ Grid, GridRegion };



	#[test]
	fn test_region_at() {
		let grid:Grid<i32> = Grid::new(vec![10, 0, 0, 0, 10, 9, 14, 12, 0], 3, 3);
		println!("[grid]\n{grid}\n");
		let region:GridRegion = grid.region_at([1, 1], |left, right| left.max(right) - left.min(right) < 3);
		println!("[region]\n{}\n", region.grid());

		assert_eq!(region.grid().data, vec![false, false, false, false, true, true, true, true, false]);
	}

	#[test]
	fn test_region_at_eq() {
		let grid:Grid<char> = Grid::new(vec!['x', ' ', ' ', ' ', 'x', 'x', 'x', 'x', ' '], 3, 3);
		println!("[grid]\n{grid}\n");
		let region:GridRegion = grid.region_at_eq([1, 1]);
		println!("[region]\n{}\n", region.grid());
		
		assert_eq!(region.grid().data, vec![false, false, false, false, true, true, true, true, false]);
	}
}