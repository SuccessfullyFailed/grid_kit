use crate::Grid;



impl<T> Grid<T> {

	/// Create a sub-grid. Bounds are XYWH.
	pub fn sub_grid(&self, bounds:[usize; 4]) -> Grid<&T> {
		let end_x:usize = (bounds[0] + bounds[2]).min(self.width).max(bounds[0]);
		let end_y:usize = (bounds[1] + bounds[3]).min(self.height).max(bounds[1]);
		
		let mut sub_data:Vec<&T> = Vec::new();
		for y in bounds[1]..end_y {
			for x in bounds[0]..end_x {
				sub_data.push(&self.data[y * self.width + x]);
			}
		}

		Grid::new(sub_data, end_x - bounds[0], end_y - bounds[1])
	}

	/// Create a sub-grid of the same size as the original grid.
	pub fn full_sub_grid(&self) -> Grid<&T> {
		self.sub_grid([0, 0, self.width, self.height])
	}

	/// Take a specific sub-field of self.
	pub fn take(mut self, bounds:[usize; 4]) -> Grid<T> {
		assert!(bounds[0] + bounds[2] <= self.width, "Trying to take width from image that is bigger than image width.");
		assert!(bounds[1] + bounds[3] <= self.height, "Trying to take height from image that is bigger than image height.");
		
		let mut new_data:Vec<Vec<T>> = Vec::with_capacity(bounds[3]);
		for row_index in (bounds[1]..bounds[1] + bounds[3]).rev() {
			let data_start:usize = row_index * self.width + bounds[0];
			new_data.push(self.data.drain(data_start..data_start + bounds[2]).collect());
		}

		Grid::new(
			new_data.into_iter().rev().flatten().collect(),
			bounds[2],
			bounds[3]
		)
	}
}