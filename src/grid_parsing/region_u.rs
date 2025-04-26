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

	#[test]
	fn test_region_thin_edge_removal() {
		let grid:Grid<char> = Grid::new(vec![' ', ' ', 'x', ' ', ' ', 	' ', ' ', 'x', ' ', 'x', 	'x', 'x', 'x', 'x', 'x', 	' ', 'x', 'x', 'x', ' ', 	' ', ' ', 'x', ' ', ' '], 5, 5);
		println!("[grid]\n{grid}\n");
		let mut region:GridRegion = grid.region_at_eq([2, 2]);
		region.remove_edge(1);
		println!("[region]\n{}\n", region.grid());
		
		assert_eq!(&region.grid().data, &[' ', ' ', ' ', ' ', ' ', 	' ', ' ', ' ', ' ', ' ', 	' ', ' ', 'x', ' ', ' ', 	' ', ' ', 'x', ' ', ' ', 	' ', ' ', ' ', ' ', ' '].map(|c| c == 'x'));
	}

	#[test]
	fn test_region_thick_edge_removal() {
		let grid:Grid<char> = Grid::new(vec!['x'; 25 * 25], 25, 25);
		println!("[grid]\n{grid}\n");
		let mut region:GridRegion = grid.region_at_eq([12, 12]);
		region.remove_edge(5);
		println!("[region]\n{}\n", region.grid());
		
		let top_rows:Vec<bool> = vec![false; 25];
		let center_rows:Vec<bool> = [vec![false; 5], vec![true; 15], vec![false; 5]].into_iter().flatten().collect();
		let bottom_rows:Vec<bool> = vec![false; 25];
		let validation_data:Vec<bool> = [vec![top_rows; 5], vec![center_rows; 15], vec![bottom_rows; 5]].into_iter().flatten().flatten().collect();

		assert_eq!(&region.grid().data, &validation_data);
	}
}