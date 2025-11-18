use crate::Grid;



impl<T> Grid<T> {

	/// Convert the grid to another type.
	pub fn map<U, V>(self, mut conversion_function:V) -> Grid<U> where V:FnMut(T) -> U {
		let width:usize = self.width;
		let height:usize = self.height;
		Grid {
			data: self.into_iter().map(|value| conversion_function(value)).collect(),
			width,
			height
		}
	}

	/// Convert the grid to another type without consuming self.
	pub fn map_ref<U, V>(&self, mut conversion_function:V) -> Grid<U> where V:FnMut(&T) -> U + {
		let width:usize = self.width;
		let height:usize = self.height;
		Grid {
			data: self.iter().map(|value| conversion_function(value)).collect(),
			width,
			height
		}
	}
}