use std::ops::{ Index, IndexMut, Range };
use crate::{ Grid, GridIndexer };



struct EdgeIndex {
	positive_index:usize,
	negative_index:Option<usize>
}



pub struct GridRegion {
	pub(crate) grid:Grid<bool>,
	pub(crate) bounds:[usize; 4]
}
impl GridRegion {

	/* PROPERTY GETTERS */
	
	/// Create a new GridRegion.
	pub fn new(grid:Grid<bool>) -> GridRegion {
		let mut region:GridRegion = GridRegion { grid, bounds: [0; 4] };
		region.update_bounds();
		region
	}
	


	/* MODIFICATION METHODS */

	/// Update the bounds of the region.
	fn update_bounds(&mut self) {
		self.bounds = [0; 4];

		let mut start_x:usize = self.grid.width;
		let mut start_y:usize = self.grid.height;
		let mut end_x:usize = 0;
		let mut end_y:usize = 0;

		let y_indexes:Vec<usize> = (0..self.grid.height).map(|y| y * self.grid.width).collect();

		// Find x start and end.
		for y in 0..self.grid.height {
			let y_index:usize = y_indexes[y];
			if let Some(x) = self.grid.data[y_index..y_index + start_x].iter().position(|node| *node) {
				start_x = x;
				if x > end_x { end_x = x; }
				if y < start_y { start_y = y; }
				if y > end_y { end_y = y; }
			}
			if let Some(flipped_x) = self.grid.data[y_index + end_x..y_index + self.grid.width].iter().rev().position(|node| *node) {
				let x:usize = self.grid.width - flipped_x - 1;
				end_x = x;
				if x < start_x { start_x = x; }
				if y < start_y { start_y = y; }
				if y > end_y { end_y = y; }
			}
		}

		// Find y start and end.
		for x in 0..self.grid.width {
			if let Some(y) = y_indexes[..start_y].iter().position(|y_index| self.grid[y_index + x]) {
				start_y = y;
				if x < start_x { start_x = x; }
				if x > end_x { end_x = x; }
				if y > end_y { end_y = y; }
			}
			if let Some(flipped_y) = y_indexes[end_y..].iter().rev().position(|y_index| self.grid[y_index + x]) {
				let y:usize = self.grid.height - flipped_y - 1;
				end_y = y;
				if x < start_x { start_x = x; }
				if x > end_x { end_x = x; }
				if y < start_y { start_y = y; }
			}
		}

		// Set new bounds.
		end_x += 1;
		end_y += 1;
		start_x = start_x.min(end_x);
		start_y = start_y.min(end_y);
		self.bounds = [start_x, start_y, end_x - start_x, end_y - start_y]
	}

	/// Add a specific width of edges.
	pub fn add_edge(&mut self, addition_width:usize) {
		for _ in 0..addition_width {
			for edge_index in self.find_edges().into_iter().map(|edge| edge.negative_index).flatten() {
				self[edge_index] = true;
			}
			self.update_bounds();
		}
	}

	/// Remove a specific width of edges.
	pub fn remove_edge(&mut self, removal_width:usize) {
		for _ in 0..removal_width {
			for edge_index in self.find_edges().into_iter().map(|edge| edge.positive_index) {
				self[edge_index] = false;
			}
		}
		self.update_bounds();
	}

	/// Map the region to values indicating their distance to the nearest edge.
	pub fn to_edge_distance_map(mut self) -> Grid<usize> {
		let mut edge_map:Grid<usize> = Grid::new(vec![0; self.grid.width * self.grid.height], self.grid.width, self.grid.height);
		let mut distance:usize = 1;
		let mut any_changes:bool = true;
		while any_changes {
			any_changes = false;
			for edge_index in self.find_edges().into_iter().map(|edge| edge.positive_index) {
				self[edge_index] = false;
				edge_map[edge_index] = distance;
				any_changes = true;
			}
			distance += 1;
		}
		edge_map
	}

	/// Find the edges of the region.
	fn find_edges(&self) -> Vec<EdgeIndex> {
		let mut edges:Vec<EdgeIndex> = Vec::with_capacity(self.grid.width * self.grid.height);
		if self.bounds[2] == 0 || self.bounds[3] == 0 {
			return edges;
		}

		// Figure out sub-bounds to find edges.
		let x_start:usize = self.bounds[0] - if self.bounds[0] > 0 { 1 } else { 0 };
		let x_end:usize = self.bounds[0] + self.bounds[2] + if self.bounds[0] + self.bounds[2] < self.grid.width { 1 } else { 0 };
		let y_start:usize = self.bounds[1] - if self.bounds[1] > 0 { 1 } else { 0 };
		let y_end:usize = self.bounds[1] + self.bounds[3] + if self.bounds[1] + self.bounds[3] < self.grid.width { 1 } else { 0 };

		// Find edges in center rows and columns.
		let mut last_value_y:Vec<bool> = vec![false; x_end];
		for y in y_start..y_end {
			let y_start_index:usize = y * self.grid.width;
			let mut last_value_x:bool = false;
			for x in x_start..x_end {
				let index:usize = y_start_index + x;
				let value:bool = self[index];
				if value != last_value_x {
					edges.push(EdgeIndex {
						positive_index: if value { index } else { index - 1 },
						negative_index: if x == 0 { None } else { Some(if value { index - 1 } else { index }) }
					});
					last_value_x = value;
				}
				if value != last_value_y[x] {
					edges.push(EdgeIndex {
						positive_index: if value { index } else { index - self.grid.width },
						negative_index: if y == 0 { None } else { Some(if value { index - self.grid.width } else { index }) }
					});
					last_value_y[x] = value;
				}
			}
		}

		// Find edges in last row and column.
		if self.bounds[0] + self.bounds[2] == self.grid.width {
			let x:usize = self.grid.width - 1;
			for y in self.bounds[1]..self.bounds[1] + self.bounds[3] {
				let index:usize = y * self.grid.width + x;
				if self[index] {
					edges.push(EdgeIndex { positive_index: index, negative_index: None });
				}
			}
		}
		if self.bounds[1] + self.bounds[3] == self.grid.height {
			let y_index:usize = (self.grid.height - 1) * self.grid.width;
			for x in self.bounds[0]..self.bounds[0] + self.bounds[2] {
				let index:usize = y_index + x;
				if self[index] {
					edges.push(EdgeIndex { positive_index: index, negative_index: None });
				}
			}
		}

		// Return edges.
		edges
	}



	/* PROPERTY GETTER METHODS */

	/// Get the grid of the region.
	pub fn grid(&self) -> &Grid<bool> {
		&self.grid
	}

	/// Get the positive indexes of the region.
	pub fn indexes(&self) -> Vec<usize> {
		let mut indexes:Vec<usize> = Vec::with_capacity(self.bounds[2] * self.bounds[3]);
		let mut y_index:usize = self.bounds[1] * self.grid.width;
		for _y in self.bounds[1]..self.bounds[1] + self.bounds[3] {
			for x in self.bounds[0]..self.bounds[0] + self.bounds[2] {
				let index:usize = y_index + x;
				if self[index] {
					indexes.push(index);
				}
			}
			y_index += self.grid.width;
		}
		indexes
	}

	/// Get a sub-grid of the bounds.
	pub fn bounds_sub_grid(&self) -> Grid<&bool> {
		self.grid.sub_grid(self.bounds)
	}
}
impl<U> Index<U> for GridRegion where U:GridIndexer {
	type Output = bool;

	fn index(&self, index:U) -> &Self::Output {
		&self.grid[index]
	}
}
impl<U> IndexMut<U> for GridRegion where U:GridIndexer {
	fn index_mut(&mut self, index:U) -> &mut Self::Output {
		&mut self.grid[index]
	}
}
impl<U> Index<Range<U>> for GridRegion where U:GridIndexer {
	type Output = [bool];

	fn index(&self, index:Range<U>) -> &Self::Output {
		&self.grid[index]
	}
}
impl<U> IndexMut<Range<U>> for GridRegion where U:GridIndexer {
	fn index_mut(&mut self, index:Range<U>) -> &mut Self::Output {
		&mut self.grid[index]
	}
}



impl<T> Grid<T> where T:PartialEq {

	/// Starting at the selected pixel, create a list of all attached pixels that match the comparing function. In the comparing function, the first value is the value of the neighbor that added this node to the queue. The second value is the value of the current node.
	pub fn region_at<U, V>(&self, start:U, comparing_function:V) -> GridRegion where U:GridIndexer, V:Fn(&T, &T) -> bool {

		// Prepare important indexes.
		let start_index:usize = start.to_grid_index(self);
		let max_x:usize = self.width - 1;
		let max_y:usize = self.len() - self.width;

		// Create region tracking grid.
		let mut region_grid:Grid<bool> = Grid::new(vec![false; self.width * self.height], self.width, self.height);
		let mut checked_values_grid:Grid<Vec<&T>> = Grid::new(vec![Vec::new(); self.width * self.height], self.width, self.height);

		// Keep checking positions in the queue.
		let mut queue:Vec<(usize, &T)> = Vec::with_capacity(self.width * self.height * 4); // Has a lot of space, likely too much. Stops it from moving around in memory when growing.
		queue.push((start_index, &self[start_index]));
		let mut queue_cursor:usize = 0; // Keep a cursor to prevent moving the entire queue through memory on resizing.
		while queue_cursor < queue.len() {
			let (current_index, source_value) = queue[queue_cursor];

			// Skip finished and invalid cursors.
			if region_grid[current_index] || !self.index_is_valid(current_index) {
				queue_cursor += 1;
				continue;
			}

			// Set to positive in mask grid.
			if comparing_function(&source_value, &self[current_index]) || queue_cursor == 0 {
				region_grid[current_index] = true;

				// Add neighbors to queue.
				for neighbor_index in self._index_neighbors(current_index, max_x, max_y) {
					if !region_grid[neighbor_index] && !checked_values_grid[neighbor_index].contains(&source_value) {
						queue.push((neighbor_index, &self[current_index]));
						checked_values_grid[neighbor_index].push(source_value);
					}
				}
			}

			queue_cursor += 1;
		}

		// Return region.
		GridRegion::new(region_grid)
	}

	/// Starting at the selected pixel, create a list of all attached pixels that are the same. This function is very similar to the `region_at` function. Because this function does not need to check nodes for multiple different values, it is more efficient.
	pub fn region_at_eq<U>(&self, start:U) -> GridRegion where U:GridIndexer {
		
		// Prepare important indexes.
		let start_index:usize = start.to_grid_index(self);
		let max_x:usize = self.width - 1;
		let max_y:usize = self.len() - self.width;

		// Create region tracking grid.
		let mut region_grid:Grid<bool> = Grid::new(vec![false; self.width * self.height], self.width, self.height);
		let target_value:&T = &self[start_index];

		// Keep checking positions in the queue.
		let mut queue:Vec<usize> = Vec::with_capacity(self.width * self.height); // Has a lot of space, likely too much. Stops it from moving around in memory when growing.
		queue.push(start_index);
		let mut queue_cursor:usize = 0; // Keep a cursor to prevent moving the entire queue through memory on resizing.
		while queue_cursor < queue.len() {
			let current_index = queue[queue_cursor];

			// Skip finished and invalid cursors.
			if region_grid[current_index] || !self.index_is_valid(current_index) {
				queue_cursor += 1;
				continue;
			}

			// Set to positive in mask grid.
			if &self[current_index] == target_value {
				region_grid[current_index] = true;

				// Add neighbors to queue.
				for neighbor_index in self._index_neighbors(current_index, max_x, max_y) {
					if !region_grid[neighbor_index] && !queue.contains(&neighbor_index) {
						queue.push(neighbor_index);
					}
				}
			}

			queue_cursor += 1;
		}

		// Return region.
		GridRegion::new(region_grid)
	}
}