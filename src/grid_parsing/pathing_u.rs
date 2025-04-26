#[cfg(test)]
mod test {
	use crate::Grid;



	#[test]
	fn test_path_finding() {
		let maze:[&str; 5] = [
			" x   xxx x",
			"xxxx     x",
			"x    xx  x",
			"x xxx  xxx",
			"xxx xxxx  "
		];

		let grid:Grid<char> = Grid::new(maze.iter().map(|line| line.chars().collect::<Vec<char>>()).flatten().collect(), 10, 5);
		println!("[grid]\n{grid}\n");
		let path:Vec<[usize; 2]> = grid.find_path([1, 0], [9, 0]).unwrap();
		let mut compare_grid:Grid<char> = Grid::new(vec![' '; grid.width * grid.height], grid.width, grid.height);
		for index in &path {
			compare_grid[*index] = 'x';
		}
		println!("[compare grid]\n{compare_grid}\n");
		
		assert_eq!(path, vec![[1, 0], [1, 1], [0, 1], [0, 2], [0, 3], [0, 4], [1, 4], [2, 4], [2, 3], [3, 3], [4, 3], [4, 4], [5, 4], [6, 4], [7, 4], [7, 3], [8, 3], [9, 3], [9, 2], [9, 1], [9, 0]]);
	}
}