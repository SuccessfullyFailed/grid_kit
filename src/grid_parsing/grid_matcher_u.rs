#[cfg(test)]
mod tests {
	use crate::{ Grid, GridMask, GridMatcher };



	const IN_GRID_SIZE:[usize; 2] = [10, 10];
	const AOI_START:[usize; 2] = [1, 2];
	const OUT_GRID_SIZE:[usize; 2] = [5, 6];
	const MASK_END:usize = 8;
	fn test_matcher() -> GridMatcher<usize, u32> {
		let mut grid_matcher:GridMatcher<usize, u32> = {
			GridMatcher::new(|input| (input * 1) as u32)
				.with_area_of_interest([AOI_START[0], AOI_START[1], OUT_GRID_SIZE[0], OUT_GRID_SIZE[1]])
				.with_mask(GridMask::new(Grid::new((0..OUT_GRID_SIZE[0] * OUT_GRID_SIZE[1]).map(|index| index < MASK_END).collect(), OUT_GRID_SIZE[0], OUT_GRID_SIZE[1])))
		};
		for multiplier in 0..8 {
			grid_matcher = grid_matcher.with_named_entry(&format!("test_grid_{multiplier}"), Grid::new((0..IN_GRID_SIZE[0] * IN_GRID_SIZE[1]).map(|index| index * (multiplier + 1)).collect(), IN_GRID_SIZE[0], IN_GRID_SIZE[1]))
		}
		grid_matcher
	}



	#[test]
	fn test_grid_matcher_grid_processing() {
		let grid_matcher:GridMatcher<usize, u32> = test_matcher();
		for (grid_index, (_, grid)) in grid_matcher.named_entries().iter().enumerate() {
			println!("[grid {grid_index}]\n{grid}\n");
			assert_eq!((grid.width, grid.height), (OUT_GRID_SIZE[0], OUT_GRID_SIZE[1]));
			assert_eq!(&grid.data[MASK_END..], &[0; OUT_GRID_SIZE[0] * OUT_GRID_SIZE[1] - MASK_END]);
		}
	}


	#[test]
	fn test_grid_matcher_find_similar() {
		let grid_matcher:GridMatcher<usize, u32> = test_matcher();
		let bad_target_grid:Grid<usize> = Grid::new(vec![0; IN_GRID_SIZE[0] * IN_GRID_SIZE[1]], IN_GRID_SIZE[0], IN_GRID_SIZE[1]);
		println!("[named grid 1]\n{}\n", grid_matcher.named_entries()[0].1);
		println!("[bad target grid]\n{bad_target_grid}\n");
		assert_eq!(grid_matcher.first_similar_to(bad_target_grid.clone(), 0.5), None); // Similarity above 0.0 would mean pixels outside of the mask are compared.
		assert_eq!(grid_matcher.most_similar_to(bad_target_grid.clone()).unwrap().1, 0.0);
		
		let perfect_target_grid_index:usize = 3;
		let perfect_target_grid:Grid<usize> = Grid::new((0..IN_GRID_SIZE[0] * IN_GRID_SIZE[1]).map(|index| index * (perfect_target_grid_index + 1)).collect(), IN_GRID_SIZE[0], IN_GRID_SIZE[1]);
		println!("[named grid {perfect_target_grid_index}]\n{}\n", grid_matcher.named_entries()[perfect_target_grid_index].1);
		println!("[perfect target grid]\n{perfect_target_grid}\n");
		assert_eq!(grid_matcher.first_similar_to(perfect_target_grid.clone(), 0.95), Some(format!("test_grid_{perfect_target_grid_index}").as_str()));
		assert_eq!(grid_matcher.most_similar_to(perfect_target_grid.clone()), Some((format!("test_grid_{perfect_target_grid_index}").as_str(), 1.0)));
		
		let good_target_grid_index:usize = 5;
		let mut good_target_grid:Grid<usize> = Grid::new((0..IN_GRID_SIZE[0] * IN_GRID_SIZE[1]).map(|index| index * (good_target_grid_index + 1)).collect(), IN_GRID_SIZE[0], IN_GRID_SIZE[1]);
		let good_grid_mutations:usize = 2;
		for mutation_index in 0..good_grid_mutations {
			good_target_grid[(AOI_START[0] + mutation_index, AOI_START[1])] = 999_999;
		}
		let expected_similarity:f32 = 1.0 / MASK_END as f32 * (MASK_END - good_grid_mutations) as f32;
		println!("[named grid]\n{}\n", grid_matcher.named_entries()[good_target_grid_index].1);
		println!("[good target grid]\n{good_target_grid}\n");
		assert_eq!(grid_matcher.first_similar_to(good_target_grid.clone(), 0.5), Some(format!("test_grid_{good_target_grid_index}").as_str()));
		assert_eq!(grid_matcher.most_similar_to(good_target_grid.clone()), Some((format!("test_grid_{good_target_grid_index}").as_str(), expected_similarity)));
	}
}