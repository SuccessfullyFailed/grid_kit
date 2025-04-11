#[cfg(test)]
mod tests {
	use crate::Grid;


	#[test]
	fn test_conversion() {
		let int_grid:Grid<i32> = Grid::new((0..10).collect(), 10, 10);
		println!("{:?}", int_grid);
		let float_grid:Grid<f32> = int_grid.clone().convert(|int| int as f32 / 2.0);
		println!("{:?}", float_grid);

		assert_eq!([int_grid.width, int_grid.height], [float_grid.width, float_grid.height]);
		assert_eq!(int_grid.data(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
		assert_eq!(float_grid.data(), &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5]);
		assert_eq!(int_grid, float_grid.convert(|float| (float * 2.0) as i32));
	}
}