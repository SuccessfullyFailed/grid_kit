use std::{ error::Error, ops::Add };
use crate::{ Grid, GridIndexer };
use super::GridRegion;



impl GridRegion {

	// Find a path from one index to another. Will only move over positive pixels in the region.
	pub fn find_path<U, V>(&self, start:U, end:V) -> Result<Vec<[usize; 2]>, Box<dyn Error>> where U:GridIndexer, V:GridIndexer {

		// Find and validate start and end.
		let start_index:usize = start.to_grid_index(self.grid());
		let end_index:usize = end.to_grid_index(self.grid());
		if !self[start_index] {
			return Err("Start coordinate falls outside of the region.".into());
		}
		if !self[end_index] {
			return Err("Start coordinate falls outside of the region.".into());
		}

		// Modify arguments to sub-grid.
		let start_coord:[usize; 2] = self.grid.index_to_xy(start_index);
		let end_coord:[usize; 2] = self.grid.index_to_xy(end_index);
		let bounds_grid:Grid<&bool> = self.bounds_sub_grid();
		let start_index:usize = bounds_grid.xy_to_index(start_coord[0] - self.bounds[0], start_coord[1] - self.bounds[1]);
		let end_index:usize = bounds_grid.xy_to_index(end_coord[0] - self.bounds[0], end_coord[1] - self.bounds[1]);
		let max_x:usize = bounds_grid.width - 1;
		let max_y:usize = bounds_grid.len() - bounds_grid.width;

		// Keep checking positions in the queue.
		let mut search_grid:Grid<Option<usize>> = Grid::new(vec![None; bounds_grid.width * bounds_grid.height], bounds_grid.width, bounds_grid.height); // For each node, keeps the amount of steps to the start coordinate.
		let mut queue:Vec<(usize, usize)> = Vec::with_capacity(bounds_grid.width * bounds_grid.height); // Has a lot of space, likely too much. Stops it from moving around in memory when growing.
		queue.push((start_index, start_index));
		let mut queue_cursor:usize = 0; // Keep a cursor to prevent moving the entire queue through memory on resizing.
		while queue_cursor < queue.len() {
			let (current_index, previous_index) = queue[queue_cursor];

			// If end found, recreate and return path.
			if current_index == end_index {
				search_grid[current_index] = Some(previous_index);
				let mut path_indexes:Vec<usize> = Vec::new();
				let mut backtrack_cursor:usize = current_index;
				while let Some(previous_index) = &search_grid[backtrack_cursor] {
					path_indexes.push(backtrack_cursor);
					if previous_index == &backtrack_cursor {
						break;
					}
					backtrack_cursor = *previous_index;
				}
				return Ok(path_indexes.into_iter().rev().map(|index| bounds_grid.index_to_xy(index)).map(|[x, y]| [x + self.bounds[0], y + self.bounds[1]]).collect());
			}

			// Skip finished and invalid cursors.
			if search_grid[current_index].is_some() || !bounds_grid.index_is_valid(current_index) {
				queue_cursor += 1;
				continue;
			}
			search_grid[current_index] = Some(previous_index);

			// Add neighbors to queue.
			for neighbor_index in bounds_grid._index_neighbors(current_index, max_x, max_y) {
				if *bounds_grid[neighbor_index] && search_grid[neighbor_index].is_none() && !queue.iter().any(|(index, _)| index == &neighbor_index) {
					queue.push((neighbor_index, current_index));
				}
			}

			queue_cursor += 1;
		}

		// No path was found.
		Err("Could not find path.".into())
	}
}
impl<T> Grid<T> where T:PartialEq {

	// Find a path from one index to another. Will only move over pixels that are equal to the starting pixel.
	pub fn find_path<U, V>(&self, start:U, end:V) -> Result<Vec<[usize; 2]>, Box<dyn Error>> where U:GridIndexer, V:GridIndexer {
		self.region_at_eq(start.to_grid_index(self)).find_path(start, end)
	}
}
impl<T> Grid<T> {

	// Find a path from one index to another. Will only move over pixels that are equal to the starting pixel.
	pub fn find_path_weighed<U, V, W, X>(&self, start:U, end:V, weight_function:W) -> Result<Vec<[usize; 2]>, Box<dyn Error>> where U:GridIndexer, V:GridIndexer, W:Fn((usize, &T), (usize, &T)) -> Option<X>, X:Ord + Add<Output=X> + Clone + Default {

		// Find and validate start and end.
		let start_index:usize = start.to_grid_index(self);
		let end_index:usize = end.to_grid_index(self);
		if start_index >= self.len() {
			return Err("Start coordinate falls outside of the grid.".into());
		}
		if end_index >= self.len() {
			return Err("Start coordinate falls outside of the grid.".into());
		}
		let max_x:usize = self.width - 1;
		let max_y:usize = self.len() - self.width;

		// Keep checking positions in the queue.
		let mut origin_grid:Grid<Option<(usize, X)>> = Grid::new(vec![None; self.width * self.height], self.width, self.height); // For each node, keep the origin and value of total weight to get here.
		origin_grid[start_index] = Some((start_index, X::default()));
		let mut queue:Vec<(usize, X)> = Vec::with_capacity(self.width * self.height); // Has a lot of space, likely too much. Stops it from moving around in memory when growing.
		queue.push((start_index, X::default()));
		let mut queue_cursor:usize = 0; // Keep a cursor to prevent moving the entire queue through memory on resizing.
		while queue_cursor < queue.len() {
			let (current_index, current_weight) = &queue[queue_cursor];
			let current_index:usize = *current_index;

			// If end found, backtrack and return path.
			if current_index == end_index {
				let mut path_indexes:Vec<usize> = vec![current_index];
				let mut backtrack_cursor:usize = current_index;
				while let Some((previous_index, _)) = &origin_grid[backtrack_cursor] {
					if path_indexes.contains(previous_index) {
						return Err("Repeating loop in backtracking path.".into());
					}
					path_indexes.push(*previous_index);
					if previous_index == &start_index {
						break;
					}
					backtrack_cursor = *previous_index;
				}
				return Ok(path_indexes.into_iter().rev().map(|index| self.index_to_xy(index)).collect());
			}

			// Add neighbors to queue.
			let current_value:&T = &self[current_index];
			let mut queue_additions:Vec<(usize, X)> = Vec::with_capacity(4);
			for neighbor_index in self._index_neighbors(current_index, max_x, max_y) {
				if self.index_is_valid(neighbor_index) {
					let neighbor_value:&T = &self[neighbor_index];
					if let Some(weight_addition_to_neighbor) = weight_function((current_index, current_value), (neighbor_index, neighbor_value)) {
						let value_to_neighbor:X = current_weight.clone() + weight_addition_to_neighbor;
						let current_neighbor_origin:&Option<(usize, X)> = &origin_grid[neighbor_index];
						if current_neighbor_origin.is_none() || (current_neighbor_origin.as_ref().unwrap().0 != current_index && value_to_neighbor < current_neighbor_origin.as_ref().unwrap().1) {
							origin_grid[neighbor_index] = Some((current_index, value_to_neighbor.clone()));
							queue_additions.push((neighbor_index, value_to_neighbor));
						}
					}
				}
			}
			queue.extend(queue_additions);

			// Move cursor to next item.
			queue_cursor += 1;
		}

		// No path was found.
		Err("Could not find path.".into())
	}
}