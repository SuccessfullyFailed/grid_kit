#[cfg(test)]
mod test {
	use crate::{ Grid, RegionMask };



	#[test]
	fn test_region_of_equals_at() {
		let grid:Grid<char> = Grid::new(vec!['x', ' ', ' ', ' ', 'x', 'x', 'x', 'x', ' '], 3, 3);
		println!("[grid]\n{grid}\n");
		let region:RegionMask = grid.region_of_equals_at([1, 1]);
		println!("[region]\n{region}\n");
		
		assert_eq!(region.data, vec![false, false, false, false, true, true, true, true, false]);
	}

	#[test]
	fn test_regionat() {
		let grid:Grid<i32> = Grid::new(vec![10, 0, 0, 0, 10, 9, 14, 12, 0], 3, 3);
		println!("[grid]\n{grid}\n");
		let region:RegionMask = grid.region_at([1, 1], |left, right| left.max(right) - left.min(right) < 3);
		println!("[region]\n{region}\n");

		assert_eq!(region.data, vec![false, false, false, false, true, true, true, true, false]);
	}
}