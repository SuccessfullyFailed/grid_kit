use crate::{ Grid, GridIndexer };



impl<T> Grid<T> where T:Clone {

	/// Append another grid to overwrite part of the current one. Will ignore any pixels out of bounds.
	pub fn append(&mut self, addition:&Grid<T>) {
		self.append_at(addition, 0)
	}

	/// Append another grid to overwrite part of the current one at the given position. Will ignore any pixels out of bounds.
	pub fn append_at<U>(&mut self, addition:&Grid<T>, offset:U) where U:GridIndexer {
		let [start_x, start_y, overlap_width, overlap_height] = self.overlap_between_this_and(addition, offset);
		for row_index in 0..overlap_height {
			let self_row_start_index:usize = (start_y + row_index) * self.width + start_x;
			let addition_row_start_index:usize = row_index * addition.width;
			for cursor_x in 0..overlap_width {
				self[self_row_start_index + cursor_x] = addition[addition_row_start_index + cursor_x].clone();
			}
		}
	}

	/// Calculate the overlap between this grid and another.
	fn overlap_between_this_and<U, V>(&self, grid:&Grid<U>, grid_offset:V) -> [usize; 4] where V:GridIndexer {
		let (start_x, start_y) = grid_offset.to_grid_xy(&self);
		[
			start_x,
			start_y,
			(start_x + grid.width).min(self.width) - start_x,
			(start_y + grid.height).min(self.height) - start_y
		]
	}
}
impl<T> Grid<Grid<T>> where T:Default + Clone {

	/// Flattens a grid of grids into one grid. Acts a lot like a CSS grid.
	pub fn flatten_grid(&self) -> Grid<T> {

		// Calculate the size of rows and columns in the flattened grid.
		let col_widths:Vec<usize> = (0..self.width).map(|x| (0..self.height).map(|y| self[(x, y)].width).max().unwrap_or_default()).collect::<Vec<usize>>();
		let row_heights:Vec<usize> = (0..self.height).map(|y| (0..self.width).map(|x| self[(x, y)].height).max().unwrap_or_default()).collect::<Vec<usize>>();

		// Create a new empty grid.
		let flattened_width:usize = col_widths.iter().sum();
		let flattened_height:usize = row_heights.iter().sum();
		let mut flattened:Grid<T> = Grid::new(vec![T::default(); flattened_width * flattened_height], flattened_width, flattened_height);

		// Append grids to the new grid.
		let mut cursor:[usize; 2] = [0, 0];
		for (y, row_height) in row_heights.iter().enumerate() {
			for (x, col_width) in col_widths.iter().enumerate() {
				flattened.append_at(&self[(x, y)], cursor);
				cursor[0] += col_width;
			}
			cursor[0] = 0;
			cursor[1] += row_height;
		}

		// Return new grid.
		flattened
	}
}