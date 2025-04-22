use crate::{ Grid, GridIndexer };



pub type RegionMask = Grid<bool>;



impl<T> Grid<T> where T:PartialEq {
	
	/// Starting at the selected pixel, create a list of all attached pixels that are the same.
	pub fn region_at<U, V>(&self, start:U, comparing_function:V) -> RegionMask where U:GridIndexer, V:Fn(&T, &T) -> bool {
		let start_index:usize = start.to_grid_index(self);
		let x_last_index:usize = self.len() - 1;
		let y_last_index:usize = self.len() - self.width;
		
		let mut queue:Vec<(usize, &T)> = vec![(start_index, &self[start_index])]; // Items are not removed from queue, so it can grow to the maximum size of grid.len() * 4, but guarantees no pixel is parsed the same twice. This also prevents any posibility infinite loops.
		let mut queue_cursor:usize = 0;
		let mut region_mask:Grid<bool> = Grid::new(vec![false; self.len()], self.width, self.height);
		
		while queue_cursor < queue.len() {
			let (current_index, source_value) = queue[queue_cursor];

			// Skip done and invalid cursors.
			if region_mask[current_index] || !self.index_is_valid(current_index) {
				continue;
			}

			// Set to positive in mask grid.
			if comparing_function(&source_value, &self[current_index]) {
				region_mask[current_index] = true;

				// Add neighbors to queue.
				let potential_neigbors:[Option<usize>; 4] = [
					if current_index > 0 { Some(current_index - 1) } else { None },
					if current_index > self.width { Some(current_index - self.width) } else { None },
					if current_index < x_last_index { Some(current_index + 1) } else { None },
					if current_index < y_last_index { Some(current_index + self.width) } else { None }
				];
				for neighbor_index in potential_neigbors.into_iter().flatten() {
					let new_queue_entry:(usize, &T) = (neighbor_index, &self[current_index]);
					if !region_mask[neighbor_index] && !queue.contains(&new_queue_entry) {
						queue.push(new_queue_entry);
					}
				}
			}

			queue_cursor += 1;
		}
		
		region_mask
	}

	/// Starting at the selected pixel, create a list of all attached pixels that are the same.
	pub fn region_of_equals_at<U>(&self, start:U) -> RegionMask where U:GridIndexer {
		// This function is very similar to region_at, but does not require keeping track of values, so keeping the code separate speeds up the process.

		let start_index:usize = start.to_grid_index(self);
		let start_value:&T = &self[start_index];
		let x_last_index:usize = self.len() - 1;
		let y_last_index:usize = self.len() - self.width;
		
		let mut queue:Vec<usize> = vec![start_index];
		let mut checked_indexes:Vec<usize> = Vec::with_capacity(self.len()); // More memory and less re-allocation than Vec::new().
		let mut region_mask:Grid<bool> = Grid::new(vec![false; self.len()], self.width, self.height);

		while !queue.is_empty() {
			let index:usize = queue.pop().unwrap();

			// Skip done and invalid cursors.
			if checked_indexes.contains(&index) || !self.index_is_valid(index) {
				continue;
			}
			checked_indexes.push(index);

			// Set to positive in mask grid.
			if &self[index] == start_value {
				region_mask[index] = true;

				// Add neighbors to queue.
				let potential_neigbors:[Option<usize>; 4] = [
					if index > 0 { Some(index - 0) } else { None },
					if index > self.width { Some(index - self.width) } else { None },
					if index < x_last_index { Some(index + 1) } else { None },
					if index < y_last_index { Some(index + self.width) } else { None }
				];
				for neighbor in potential_neigbors.into_iter().flatten() {
					if !checked_indexes.contains(&neighbor) && !queue.contains(&neighbor) {
						queue.push(neighbor);
					}
				}
			}
		}

		region_mask
	}
}