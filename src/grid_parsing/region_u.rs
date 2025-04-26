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
		println!("[region]\n{}\n", region.grid().map_ref(|value| if *value { 'x' } else { ' ' }));
		
		assert_eq!(&region.grid().data, &[' ', ' ', ' ', ' ', ' ', 	' ', ' ', ' ', ' ', ' ', 	' ', ' ', 'x', ' ', ' ', 	' ', ' ', 'x', ' ', ' ', 	' ', ' ', ' ', ' ', ' '].map(|c| c == 'x'));
	}

	#[test]
	fn test_region_thick_edge_removal() {
		let grid:Grid<char> = Grid::new(vec!['x'; 25 * 25], 25, 25);
		println!("[grid]\n{grid}\n");
		let mut region:GridRegion = grid.region_at_eq([12, 12]);
		region.remove_edge(5);
		println!("[region]\n{}\n", region.grid().map_ref(|value| if *value { 'x' } else { ' ' }));
		
		let validation_grid_data:Vec<bool> = (0..25).map(|y| 
			if y < 5 || y > 19 {
				vec![false; 25]
			} else {
				[vec![false; 5], vec![true; 15], vec![false; 5]].into_iter().flatten().collect()
			}
		).flatten().collect();
		assert_eq!(&region.grid().data, &validation_grid_data);
	}

	#[test]
	fn test_region_thin_edge_addition() {
		let grid:Grid<char> = Grid::new(vec![' ', ' ', 'x', ' ', ' ', 	' ', ' ', 'x', ' ', 'x', 	'x', 'x', 'x', 'x', 'x', 	' ', 'x', 'x', 'x', ' ', 	' ', ' ', 'x', ' ', ' '], 5, 5);
		println!("[grid]\n{grid}\n");
		let mut region:GridRegion = grid.region_at_eq([2, 2]);
		region.add_edge(1);
		println!("[region]\n{}\n", region.grid().map_ref(|value| if *value { 'x' } else { ' ' }));
		
		assert_eq!(&region.grid().data, &[' ', 'x', 'x', 'x', 'x', 	'x', 'x', 'x', 'x', 'x', 	'x', 'x', 'x', 'x', 'x', 	'x', 'x', 'x', 'x', 'x', 	' ', 'x', 'x', 'x', ' '].map(|c| c == 'x'));
	}

	#[test]
	fn test_region_thick_edge_addition() {

		// Prepare grid data.
		let original_grid_data:Vec<bool> = (0..25).map(|y| 
			if y < 5 || y > 19 {
				vec![false; 25]
			} else {
				[vec![false; 5], vec![true; 15], vec![false; 5]].into_iter().flatten().collect()
			}
		).flatten().collect();
		let validation_grid_data:Vec<bool> = (0..25).map(|y| {
			let padding:usize = if y < 5 { 5 - y } else if y > 19 { y - 19 } else { 0 };
			[
				vec![false; padding],
				vec![true; 25 - 2 * padding],
				vec![false; padding]
			]
		}).flatten().flatten().collect();

		// Create and show grids.
		let grid:Grid<char> = Grid::new(original_grid_data.iter().map(|value| if *value { 'x' } else { ' ' }).collect(), 25, 25);
		println!("[grid]\n{grid}\n");
		let mut region:GridRegion = grid.region_at_eq([12, 12]);
		region.add_edge(5);
		println!("[region]\n{}\n", region.grid().map_ref(|value| if *value { 'x' } else { ' ' }));
		println!("[validation]\n{}\n", Grid::new(validation_grid_data.clone(), 25, 25).map_ref(|value| if *value { 'x' } else { ' ' }));

		// Validate result.
		assert_eq!(&region.grid().data, &validation_grid_data);
	}
}