use std::ops::{Index, IndexMut};

pub struct Grid<T> {
	data:Vec<T>,
	width:usize,
	height:usize
}
impl<T> Grid<T> {

	/* CONSTRUCTOR METHODS */

	/// Create a new empty grid.
	pub const fn empty() -> Grid<T> {
		Grid::new(Vec::new(), 0, 0)
	}

	/// Create a new grid with some data.
	pub const fn new(data:Vec<T>, width:usize, height:usize) -> Grid<T> {
		Grid {
			data,
			width,
			height
		}
	}



	/* INDEXER METHODS */

	/// convert an X and Y coordinate to an index.
	pub fn xy_to_index(&self, coordinate:[usize; 2]) -> usize {
		coordinate[1] * self.width + coordinate[0]
	}

	/// Convert an index to an X and Y coordinate.
	pub fn index_to_xy(&self, index:usize) -> [usize; 2] {
		let x:usize = index % self.width;
		[x, (index - x) / self.width]
	}

	/// Wether or not and X and Y coordinate are valid in the grid.
	pub fn xy_is_valid(&self, coordinate:[usize; 2]) -> bool {
		coordinate[0] < self.width && coordinate[1] < self.height
	}



	/* GETTER METHODS */

	/// Get the data of the grid.
	pub fn data(&self) -> &Vec<T> {
		&self.data
	}

	/// Get the data of the grid mutable.
	pub fn data_mut(&mut self) -> &mut Vec<T> {
		&mut self.data
	}

	/// Get the width of the grid.
	pub fn width(&self) -> usize {
		self.width
	}

	/// Get the height of the grid.
	pub fn height(&self) -> usize {
		self.height
	}

	/// Get the length of the grid.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Get wether or not the grid has no data.
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}
impl<T> Default for Grid<T> {
	fn default() -> Self {
		Grid::empty()
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
		&self.data[self.xy_to_index(coordinate)]
	}
}
impl<T> IndexMut<[usize; 2]> for Grid<T> {
	fn index_mut(&mut self, coordinate:[usize; 2]) -> &mut Self::Output {
		let index:usize = self.xy_to_index(coordinate);
		&mut self.data[index]
	}
}