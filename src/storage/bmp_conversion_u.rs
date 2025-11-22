



#[cfg(test)]
mod tests {
	use crate::{ Grid, Color };



	#[test]
	fn test_grid_bmp_conversion_read_matches_write() {

		// Create sample grid.
		const SIZE:u8 = 0xF;
		let original_grid:Grid<Color> = Grid::new(
			(0..SIZE).map(|y| 
				(0..SIZE).map(|x|
					Color::new(u32::from_be_bytes([0xFF, (0xFF * x as u16 / SIZE as u16) as u8, 0, (0xFF * y as u16 / SIZE as u16) as u8]))
				).collect::<Vec<Color>>()
			).flatten().collect::<Vec<Color>>(),
			SIZE as usize,
			SIZE as usize
		);
		println!("[original grid]\n{original_grid}\n");

		// Convert grid to BMP bytes, then read it again.
		let grid_as_bytes:Vec<u8> = original_grid.to_bmp_bytes();
		let validation_grid:Grid<Color> = Grid::from_bmp_bytes(grid_as_bytes).unwrap();
		println!("[validation grid]\n{validation_grid}\n");

		// Compare grids.
		assert_eq!(original_grid, validation_grid);
	}
}