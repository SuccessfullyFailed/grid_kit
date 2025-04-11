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

	/// Wether or not the given index is valid in the grid.
	pub fn index_is_valid(&self, index:usize) -> bool {
		index < self.len()
	}

	/// Wether or not the given and X and Y coordinate are valid in the grid.
	pub fn xy_is_valid(&self, x:usize, y:usize) -> bool {
		x < self.width && y < self.height
	}
}
impl<T, U> Index<U> for Grid<T> where U:GridIndexer {
	type Output = T;

	fn index(&self, indexer:U) -> &Self::Output {
		&self.data[indexer.to_grid_index(self)]
	}
}
impl<T, U> IndexMut<U> for Grid<T> where U:GridIndexer {
	fn index_mut(&mut self, indexer:U) -> &mut Self::Output {
		let index:usize = indexer.to_grid_index(self);
		&mut self.data[index]
	}
}


pub trait GridIndexer {

	/// Convert the indexer to an actual index.
	fn to_grid_index<T>(&self, grid:&Grid<T>) -> usize;

	/// Convert the index to a X and Y coordinate on the grid.
	fn to_grid_xy<T>(&self, grid:&Grid<T>) -> (usize, usize) {
		let index:usize = self.to_grid_index(grid);
		(index % grid.width, index / grid.width)
	}
}
impl GridIndexer for usize {
	fn to_grid_index<T>(&self, _grid:&Grid<T>) -> usize {
		*self
	}
}
impl GridIndexer for [usize; 2] {
	fn to_grid_index<T>(&self, grid:&Grid<T>) -> usize {
		self[1] * grid.width + self[0]
	}
}
impl GridIndexer for (usize, usize) {
	fn to_grid_index<T>(&self, grid:&Grid<T>) -> usize {
		self.1 * grid.width + self.0
	}
}