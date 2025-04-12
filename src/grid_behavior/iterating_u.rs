#[cfg(test)]
mod tests {
	use crate::Grid;



	#[test]
	fn test_grid_iterator() {
		let grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid]\n{grid}\n");

		for (index, item) in grid.into_iter().enumerate() {
			println!("{index}: {item}");
			assert_eq!(index, item);
		}
	}

	#[test]
	fn test_grid_iterator_mut() {
		let mut grid:Grid<usize> = Grid::new((0..9).collect(), 3, 3);
		println!("[grid before mut]\n{grid}\n");

		for entry in &mut grid {
			*entry *= 2;
		}
		println!("[grid after mut]\n{grid}\n");

		assert_eq!(grid.data, vec![0, 2, 4, 6, 8, 10, 12, 14, 16]);
	}

	#[test]
	fn test_grid_iterator_pixel() {
		let grid:Grid<char> = Grid::checkers_board(3, 1, 'x', ' ');
		println!("[grid]\n{grid}\n");

		assert_eq!(grid.pixel_iterator().count(), grid.width * grid.height);
		for (index, (entry_x, entry_y, entry_item)) in grid.pixel_iterator().enumerate() {
			let expected:char = if index % 2 == 0 { 'x' } else { ' ' };
			println!("{}: ({}, {}, {})\t\tshould be {}", index, entry_x, entry_y, entry_item, expected);

			assert_eq!(entry_x, index % grid.width);
			assert_eq!(entry_y, index / grid.width);
			assert_eq!(*entry_item, expected);
			assert_eq!(grid[(entry_x, entry_y)], expected);
		}
	}

	#[test]
	fn test_grid_iterator_pixel_mut() {
		let mut grid:Grid<char> = Grid::checkers_board(3, 1, 'x', ' ');
		println!("[grid before mut]\n{grid}\n");

		for (entry_x, entry_y, entry_item) in grid.pixel_iterator_mut() {
			if entry_x < 2 && entry_y < 2 {
				*entry_item = 'o';
			}
		}
		println!("[grid after mut]\n{grid}\n");

		assert_eq!(grid.data(), &['o', 'o', 'x', 'o', 'o', ' ', 'x', ' ', 'x']);
	}
}