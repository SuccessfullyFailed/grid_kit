use crate::Grid;



pub struct GridCursor<'a, T> {
	pub(crate) grid:&'a Grid<T>,
	pub(crate) x:usize,
	pub(crate) y:usize
}
impl<'a, T> GridCursor<'a, T> {

	/* CONSTRUCTOR METHODS */

	/// Create a new cursor.
	pub fn new(grid:&'a Grid<T>, x:usize, y:usize) -> GridCursor<'a, T> {
		GridCursor {
			grid,
			x,
			y
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Whether or not the cursor is currently within the bounds of the grid.
	pub fn within_bounds(&self) -> bool {
		self.x < self.grid.width && self.y < self.grid.height
	}



	/* USAGE METHODS */

	/// Move the cursor to a specific location.
	pub fn displace_to(&mut self, x:usize, y:usize) {
		self.x = x;
		self.y = y;
	}

	/// Move the cursor a specific offset.
	pub fn displace(&mut self, offset_x:usize, offset_y:usize) {
		self.x += offset_x;
		self.y += offset_y;
	}

	/// If the cursor is out of bounds, move it to the closest place within bounds.
	pub fn displace_to_bounds(&mut self) {
		if self.x > self.grid.width {
			self.x = self.grid.width.max(1) - 1;
		}
		if self.y > self.grid.height {
			self.y = self.grid.height.max(1) - 1;
		}
	}

	/// Keep displacing the cursor by the given offset as long as the provided filter returns true on the current coordinate.
	/// If the cursor is about to be placed out of bounds, ignores that step.
	/// Does nothing if the cursor is initially out of bounds.
	pub fn displace_while<Filter:Fn(&T) -> bool>(&mut self, offset_x:isize, offset_y:isize, filter:Filter) {
		if self.within_bounds() {
			let grid_size:[isize; 2] = [self.grid.width as isize, self.grid.height as isize];
			let mut cursor:[isize; 2] = [self.x as isize, self.y as isize];
			while filter(&self.grid[(cursor[0] as usize, cursor[1] as usize)]) {
				cursor[0] += offset_x;
				cursor[1] += offset_y;
				if cursor[0] < 0 || cursor[1] < 0 || cursor[0] >= grid_size[0] || cursor[1] >= grid_size[1] {
					break;
				} else {
					self.x = cursor[0] as usize;
					self.y = cursor[1] as usize;
				}
			}
		}
	}
}