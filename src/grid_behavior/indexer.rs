use std::ops::{ Index, IndexMut };
use crate::Grid;



impl<T> Grid<T> {

	/// convert an X and Y coordinate to an index.
	pub fn xy_to_index(&self, x:usize, y:usize) -> usize {
		y * self.width + x
	}

	/// Convert an index to an X and Y coordinate.
	pub fn index_to_xy(&self, index:usize) -> [usize; 2] {
		let x:usize = index % self.width;
		[x, (index - x) / self.width]
	}

	/// Wether or not and X and Y coordinate are valid in the grid.
	pub fn xy_is_valid(&self, x:usize, y:usize) -> bool {
		x < self.width && y < self.height
	}
}
impl<T> Index<usize> for Grid<T> {
	type Output = T;

	fn index(&self, index:usize) -> &Self::Output {
		&self.data[index]
	}
}
impl<T> IndexMut<usize> for Grid<T> {
	fn index_mut(&mut self, index:usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}
impl<T> Index<[usize; 2]> for Grid<T> {
	type Output = T;

	fn index(&self, coordinate:[usize; 2]) -> &Self::Output {
		&self.data[self.xy_to_index(coordinate[0], coordinate[1])]
	}
}
impl<T> IndexMut<[usize; 2]> for Grid<T> {
	fn index_mut(&mut self, coordinate:[usize; 2]) -> &mut Self::Output {
		let index:usize = self.xy_to_index(coordinate[0], coordinate[1]);
		&mut self.data[index]
	}
}
impl<T> Index<(usize, usize)> for Grid<T> {
	type Output = T;

	fn index(&self, coordinate:(usize, usize)) -> &Self::Output {
		&self.data[self.xy_to_index(coordinate.0, coordinate.1)]
	}
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
	fn index_mut(&mut self, coordinate:(usize, usize)) -> &mut Self::Output {
		let index:usize = self.xy_to_index(coordinate.0, coordinate.1);
		&mut self.data[index]
	}
}