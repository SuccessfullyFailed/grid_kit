pub struct Grid<T> {
	pub(crate) data:Vec<T>,
	pub(crate) width:usize,
	pub(crate) height:usize
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

	/// Create a new grid with some data from a two-dimensional array.
	pub fn new_2d(data:Vec<Vec<T>>, width:usize, height:usize) -> Grid<T> {
		Grid::new(data.into_iter().flatten().collect(), width, height)
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