#[cfg(test)]
mod test {
	use crate::{ Grid, GridMask};



	#[test]
	fn test_similarity() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let compare_grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 9], 3, 3);
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(1.0 / 9.0 * 8.0, grid.similarity_to(&compare_grid));
	}

	#[test]
	fn test_similarity_greater_than() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let compare_grid:Grid<i32> = Grid::new(vec![0, 1, 2, 4, 4, 5, 6, 7, 8], 3, 3);
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(grid.similar_to(&compare_grid, 1.0), false);
		assert_eq!(grid.similar_to(&compare_grid, 1.0 / 9.0 * 8.0), true);
	}

	#[test]
	fn test_similarity_differently_sized() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let compare_grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7], 3, 3);
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(grid.similarity_to(&compare_grid), 0.0);
	}

	#[test]
	fn test_find_small() {
		let grid:Grid<i32> = Grid::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3);
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<i32> = Grid::new(vec![3, 4, 6, 7], 2, 2);
		println!("[sub grid]\n{sub_grid}\n");
		
		assert_eq!(grid.find(&sub_grid, 1.0), Some([0, 1]));
	}

	#[test]
	fn test_find_large() {
		const TILE_SIZE:usize = 3;
		const FILL_TARGET:[usize; 2] = [8, 3];

		let mut grid:Grid<char> = Grid::checkers_board(10, TILE_SIZE, 'x', ' ');
		for y in FILL_TARGET[1] * TILE_SIZE..(FILL_TARGET[1] + 1) * TILE_SIZE {
			for x in FILL_TARGET[0] * TILE_SIZE..(FILL_TARGET[0] + 1) * TILE_SIZE  {
				grid[(x, y)] = 'x';
			}
		}
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<char> = Grid::new(vec!['x'; 3 * TILE_SIZE * TILE_SIZE], 3 * TILE_SIZE, TILE_SIZE);
		println!("[sub grid]\n{sub_grid}\n");
		
		assert_eq!(grid.find(&sub_grid, 1.0), Some([(FILL_TARGET[0] - 1) * TILE_SIZE, FILL_TARGET[1] * TILE_SIZE]));
	}

	#[test]
	fn test_find_large_margined() {
		const TILE_SIZE:usize = 3;
		const FILL_TARGET:[usize; 2] = [8, 3];

		let mut grid:Grid<char> = Grid::checkers_board(10, TILE_SIZE, 'x', ' ');
		for y in FILL_TARGET[1] * TILE_SIZE..(FILL_TARGET[1] + 1) * TILE_SIZE {
			for x in FILL_TARGET[0] * TILE_SIZE..(FILL_TARGET[0] + 1) * TILE_SIZE  {
				grid[(x, y)] = 'x';
				println!("{x}, {y}");
			}
		}
		grid[(FILL_TARGET[0] * TILE_SIZE + 2, FILL_TARGET[1] * TILE_SIZE)] = 'n';
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<char> = Grid::new(vec!['x'; 3 * TILE_SIZE * TILE_SIZE], 3 * TILE_SIZE, TILE_SIZE);
		println!("[sub grid]\n{sub_grid}\n");

		assert_eq!(grid.find(&sub_grid, 0.9), Some([(FILL_TARGET[0] - 1) * TILE_SIZE, FILL_TARGET[1] * TILE_SIZE]));
	}

	#[test]
	fn test_find_large_masked() {
		const TILE_SIZE:usize = 3;
		const FILL_TARGET:[usize; 2] = [8, 3];

		let mut grid:Grid<char> = Grid::checkers_board(10, TILE_SIZE, 'x', ' ');
		for y in FILL_TARGET[1] * TILE_SIZE..(FILL_TARGET[1] + 1) * TILE_SIZE {
			for x in FILL_TARGET[0] * TILE_SIZE..(FILL_TARGET[0] + 1) * TILE_SIZE  {
				grid[(x, y)] = 'x';
			}
		}
		println!("[grid]\n{grid}\n");
		let sub_grid:Grid<char> = Grid::new(vec!['x'; 3 * TILE_SIZE * (TILE_SIZE + 1)], 3 * TILE_SIZE, TILE_SIZE + 1);
		println!("[sub grid]\n{sub_grid}\n");
		let mask:GridMask = GridMask::new(Grid::new(vec![vec![true; 3 * TILE_SIZE * TILE_SIZE], vec![false; 3 * TILE_SIZE]].into_iter().flatten().collect(), 3 * TILE_SIZE, TILE_SIZE + 1));
		println!("[mask grid]\n{}\n", mask.grid().map_ref(|val| if *val { 'x' } else { ' ' }));

		assert_eq!(grid.find_masked(&sub_grid, &mask, 1.0), Some([(FILL_TARGET[0] - 1) * TILE_SIZE, FILL_TARGET[1] * TILE_SIZE]));
	}
}