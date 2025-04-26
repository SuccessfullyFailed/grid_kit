use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::{ Grid, GridIndexer };



pub struct GridRegion {
	grid:Grid<bool>,
	bounds:[usize; 4]
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

		// Update StartX and EndX.
		let mut start_x:usize = self.grid.height;
		let mut end_x:usize = 0;
		let y_indexes:Vec<usize> = (0..self.grid.height).map(|y| y * self.grid.width).collect();
		for x in 0..self.grid.width {
			if y_indexes.iter().map(|y_index| self[y_index + x]).any(|value| value) {
				if start_x > x {
					start_x = x;
				}
				if end_x < x {
					end_x = x;
				}
			}
		}

		// Update StartY and EndY.
		let mut start_y:usize = self.grid.height;
		let mut end_y:usize = 0;
		for y in 0..self.grid.height {
			let start_index:usize = y * self.grid.width;
			let end_index:usize = (start_index + self.grid.width - 1).max(start_index);
			if self.grid.data[start_index..end_index].iter().any(|value| *value) {
				if start_y > y {
					start_y = y;
				}
				if end_y < y {
					end_y = y;
				}
			}
		}

		// Set new bounds.
		end_x += 1;
		end_y += 1;
		start_x = start_x.min(end_x);
		start_y = start_y.min(end_y);
		self.bounds = [start_x, start_y, end_x - start_x, end_y - start_y]
	}

	/// Remove a specific width of edges.
	pub fn remove_edge(&mut self, removal_width:usize) {
		
		// Remove one layer for each removal width.
		for _ in 0..removal_width {
			let mut edge_indexes:Vec<usize> = Vec::new();
		
			// Find all changes in positive and negative from left to right.
			if self.bounds[2] > 0 {
				for y in self.bounds[1]..self.bounds[1] + self.bounds[3] {
					let start_index:usize = y * self.grid.width + self.bounds[0];
					let end_index:usize = start_index + self.bounds[2];
					let mut previous_value:bool = false;
					for cursor in start_index..end_index {
						if self[cursor] != previous_value {
							edge_indexes.push(if previous_value { cursor - 1 } else { cursor });
							previous_value = self[cursor];
						}
					}
					if previous_value {
						edge_indexes.push(end_index - 1);
					}
				}
			}
		
			// Find all changes in positive and negative from left to right.
			if self.bounds[3] > 0 {
				for x in self.bounds[0]..self.bounds[0] + self.bounds[2] {
					let start_index:usize = self.bounds[1] * self.grid.width + x;
					let end_index:usize = (self.bounds[1] + self.bounds[3]) * self.grid.width + x;//start_index + self.bounds[3] * self.grid.width;
					let mut previous_value:bool = false;
					for cursor in (start_index..end_index).step_by(self.grid.width) {
						if self[cursor] != previous_value {
							edge_indexes.push(if previous_value { cursor - self.grid.width } else { cursor });
							previous_value = self[cursor];
						}
					}
					if previous_value {
						edge_indexes.push(end_index - self.grid.width);
					}
				}
			}

			// Remove found edge coordinates.
			for edge_index in edge_indexes {
				self[edge_index] = false;
			}
		}

		// Update bounds.
		self.update_bounds();
	}



	/* PROPERTY GETTER METHODS */

	/// Get the grid of the region.
	pub fn grid(&self) -> &Grid<bool> {
		&self.grid
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



impl<T> Grid<T> where T:PartialEq + Display {

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
				for neighbor_index in Self::index_neighbors(current_index, self.width, max_x, max_y) {
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
				for neighbor_index in Self::index_neighbors(current_index, self.width, max_x, max_y) {
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

	/// Get the available neighbors for a specific index.
	fn index_neighbors(position_index:usize, width:usize, max_x:usize, max_y:usize) -> Vec<usize> {
		[
			if position_index > 0 { Some(position_index - 1) } else { None }, // Left
			if position_index > width { Some(position_index - width) } else { None }, // Top
			if position_index % width != max_x { Some(position_index + 1) } else { None }, // Right
			if position_index < max_y { Some(position_index + width) } else { None } // Bottom
		].into_iter().flatten().collect()
	}
}