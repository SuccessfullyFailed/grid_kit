use crate::Grid;



impl<T> Grid<T> {

	/// Convert the grid to another type.
	pub fn convert<U, V>(self, conversion_function:V) -> Grid<U> where V:Fn(T) -> U + 'static {
		let width:usize = self.width;
		let height:usize = self.height;
		Grid {
			data: self.into_iter().map(|value| conversion_function(value)).collect(),
			width,
			height
		}
	}
}