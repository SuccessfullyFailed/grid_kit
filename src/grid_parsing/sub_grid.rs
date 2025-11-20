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
}
impl<T> Grid<T> where T:Clone {

	/// Take a sub-section of this grid.
	pub fn take(&self, bounds:[usize; 4]) -> Grid<T> {
		let mut sub_data:Vec<T> = Vec::new();
		for y in bounds[1]..bounds[1] + bounds[3] {
			for x in bounds[0]..bounds[0] + bounds[2] {
				sub_data.push(self.data[y * self.width + x].clone());
			}
		}
		Grid::new(sub_data, bounds[2], bounds[3])
	}
}