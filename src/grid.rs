use std::fmt::Debug;



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

	/// Get the data of the grid as rows.
	pub fn data_2d(&self) -> Vec<&[T]> {
		self.data.chunks(self.width).collect()
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
impl<T> Clone for Grid<T> where T:Clone {
	fn clone(&self) -> Self {
		Grid {
			data: self.data.clone(),
			width: self.width,
			height: self.height
		}
	}
}
impl<T> ToString for Grid<T> where T:ToString {
	fn to_string(&self) -> String {
		let values_as_string:Vec<Vec<String>> = self.data_2d().into_iter().map(|row| row.into_iter().map(|value| value.to_string()).collect::<Vec<String>>()).collect();
		let field_size:usize = values_as_string.iter().flatten().map(|value| value.len()).min().unwrap_or_default();
		values_as_string.into_iter().map(|row|
			row.into_iter().map(|value_str| 
				format!("[{}{}]", value_str, " ".repeat(field_size - value_str.len()))
			).collect::<Vec<String>>().join(" ")
		).collect::<Vec<String>>().join("\n")
	}
}
impl<T> Debug for Grid<T> where T:Debug {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let values_as_string:Vec<Vec<String>> = self.data_2d().into_iter().map(|row| row.into_iter().map(|value| format!("{:?}", value)).collect::<Vec<String>>()).collect();
		let field_size:usize = values_as_string.iter().flatten().map(|value| value.len()).min().unwrap_or_default();
		write!(
			f,
			"{}",
			values_as_string.into_iter().map(|row|
				row.into_iter().map(|value_str| 
					format!("[{}{}]", value_str, " ".repeat(field_size - value_str.len()))
				).collect::<Vec<String>>().join(" ")
			).collect::<Vec<String>>().join("\n")
		)
	}
}