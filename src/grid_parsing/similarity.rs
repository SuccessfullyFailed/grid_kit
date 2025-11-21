/*
	! Warning !
	this file will have duplicate code. As the methods in this file will have lots of nested loops,
	the duplicate code will save lots of small amounts of time, especially in large grids. While this
	is often not considered proper etiquette, it helps keep functions simple and efficient.
*/



use crate::{ Grid, GridMask };
use std::error::Error;



impl<T> Grid<T> where T:PartialEq {

	/* HELPER METHODS */

	/// Validate that this grid is comparable to the other. Returns an error if something is wrong.
	fn validate_comparable_grids(&self, other:&Grid<T>, mask:Option<&GridMask>) -> Result<(), Box<dyn Error>> {
		if (self.width, self.height) != (other.width, other.height) {
			return Err(format!("Cannot get similarity between grids with differing width or height. Trying to compare grid {}x{} to grid {}x{}. Returning default value.", self.width, self.height, other.width, other.height).into());
		}
		if self.data.len() != other.data.len() {
			return Err(format!("Cannot get similarity between grids with differing length of data. Trying to compare {} length grid to {} length grid. Returning default value.", self.data.len(), other.data.len()).into());
		}
		if let Some(mask) = mask {
			if (self.width, self.height) != (mask.width(), mask.height()) {
				return Err(format!("Cannot get similarity between grids with a mask of differing width or height. Trying to compare grid {}x{} to mask {}x{}. Returning default value.", self.width, self.height, mask.width(), mask.height()).into());
			}
		}
		Ok(())
	}

	/// Validate that this grid can be used to find the other in. Returns an error if something is wrong.
	fn validate_findable_grids(&self, other:&Grid<T>, mask:Option<&GridMask>) -> Result<(), Box<dyn Error>> {
		if self.width < other.width || self.height < other.height {
			return Err(format!("Cannot find sub-grid in this grid, the main grid should be larger than the sub-grid. Trying to find grid {}x{} in grid {}x{}", other.width, other.height, self.width, self.height).into());
		}
		if let Some(mask) = mask {
			if (other.width, other.height) != (mask.width(), mask.height()) {
				return Err(format!("Cannot find sub-grid in this grid with a mask of differing width or height than the sub-grid. Trying to apply mask {}x{} to sub-grid {}x{}. Returning default value.", mask.width(), mask.height(), self.width, self.height).into());
			}
		}
		Ok(())
	}



	/* SIMILARITY METHODS */

	/// Compare this grid to another. Returns 'true' if the similarity reaches the given threshold.
	pub fn similar_to(&self, other:&Grid<T>, similarity_threshold_factor:f32) -> bool {

		// Validate grids same size.
		if let Err(error) = self.validate_comparable_grids(other, None) {
			eprintln!("{}", error);
			return false;
		}

		// Edge cases.
		if similarity_threshold_factor <= 0.0 {
			return true;
		}
		if similarity_threshold_factor == 1.0 {
			return self.data == other.data;
		}
		if similarity_threshold_factor > 1.0 {
			return false;
		}

		// Loop through pixels counting mismatches.
		let comparing_pixel_count:usize = self.width * self.height;
		let max_mismatches:usize = ((1.0 - similarity_threshold_factor) * comparing_pixel_count as f32).round() as usize;
		let mut mismatches:usize = 0;
		for (left, right) in self.data.iter().zip(&other.data) {
			if left != right {
				mismatches += 1;
				if mismatches > max_mismatches {
					return false;
				}
			}
		}
		return true;
	}

	/// Compare this grid to another. Returns the factor of similarity where 0.0 is no similarity and 1.0 is a full match.
	pub fn similarity_to(&self, other:&Grid<T>) -> f32 {

		// Validate grids same size.
		if let Err(error) = self.validate_comparable_grids(other, None) {
			eprintln!("{}", error);
			return 0.0;
		}

		// Get similarity.
		let matches:usize = self.data.iter().zip(&other.data).filter(|(left, right)| left == right).count();
		matches as f32 / self.data.len() as f32
	}

	/// Compare this grid to another. Only compare the pixels matching the given mask. Returns 'true' if the similarity reaches the given threshold.
	pub fn similar_to_masked(&self, other:&Grid<T>, similarity_threshold_factor:f32, mask:&GridMask) -> bool {

		// Validate grids same size.
		if let Err(error) = self.validate_comparable_grids(other, Some(mask)) {
			eprintln!("{}", error);
			return false;
		}

		// Edge cases.
		if similarity_threshold_factor <= 0.0 {
			return true;
		}
		if similarity_threshold_factor == 1.0 {
			return mask.positive_ranges().iter().all(|range| range.clone().all(|index| self[index] == other[index]));
		}
		if similarity_threshold_factor > 1.0 {
			return false;
		}

		// Loop through pixels counting mismatches.
		let comparing_pixel_count:usize = mask.positive_ranges().iter().map(|range| range.end - range.start).sum();
		let max_mismatches:usize = ((1.0 - similarity_threshold_factor) * comparing_pixel_count as f32).round() as usize;
		let mut mismatches:usize = 0;
		for index_range in mask.positive_ranges() {
			for index in index_range.clone() {
				if self[index] != other[index] {
					mismatches += 1;
					if mismatches > max_mismatches {
						return false;
					}
				}
			}
		}
		return true;
	}



	/* MASKED SIMILARITY METHODS */

	/// Compare this grid to another. Only compare the pixels matching the given mask. Returns the factor of similarity where 0.0 is no similarity and 1.0 is a full match.
	pub fn similarity_to_masked(&self, other:&Grid<T>, mask:&GridMask) -> f32 {

		// Validate grids same size.
		if let Err(error) = self.validate_comparable_grids(other, Some(mask)) {
			eprintln!("{}", error);
			return 0.0;
		}

		// Get similarity.
		let comparing_pixel_count:usize = mask.positive_ranges().iter().map(|range| range.end - range.start).sum();
		let matches:usize = mask.positive_ranges().iter().map(|range| range.clone().filter(|&index| self[index] == other[index]).count()).sum();
		matches as f32 / comparing_pixel_count as f32
	}



	/* SUB-GRID FINDING METHODS */

	/// Find the given sub-grid in self. Returns the topleft coordinates of the first position where the similarity reaches the given threshold.
	pub fn find(&self, sub_grid:&Grid<T>, similarity_threshold_factor:f32) -> Option<[usize; 2]> {
		let comparing_pixel_count:usize = sub_grid.width * sub_grid.height;
		let max_mismatches:usize = ((1.0 - similarity_threshold_factor) * comparing_pixel_count as f32).round() as usize;
		self.find_starting_at_position(sub_grid, max_mismatches, [0, 0])
	}

	/// Find all instances of the given sub-grid in self. Returns the topleft coordinates of the all positions where the similarity reaches the given threshold.
	pub fn find_all(&self, sub_grid:&Grid<T>, similarity_threshold_factor:f32) -> Vec<[usize; 2]> {
		let comparing_pixel_count:usize = sub_grid.width * sub_grid.height;
		let max_mismatches:usize = ((1.0 - similarity_threshold_factor) * comparing_pixel_count as f32).round() as usize;
		let mut cursor:[usize; 2] = [0, 0];
		let mut results:Vec<[usize; 2]> = Vec::new();
		while let Some(position) = self.find_starting_at_position(sub_grid, max_mismatches, cursor) {
			results.push(position);
			cursor = [position[0] + 1, position[1]];
		}
		results
	}
	
	/// Find the given sub-grid in self. Returns the topleft coordinates of the first position where the similarity reaches the given threshold. Starts at the given position.
	pub fn find_starting_at_position(&self, sub_grid:&Grid<T>, max_allowed_mismatches:usize, position:[usize; 2]) -> Option<[usize; 2]> {

		// Validate grids same size.
		if let Err(error) = self.validate_findable_grids(sub_grid, None) {
			eprintln!("{}", error);
			return None;
		}

		// Loop through all possible top-left positions.
		let self_row_shift:usize = self.width - sub_grid.width;
		let end_x:usize = self.width - sub_grid.width + 1;
		let end_y:usize = self.height - sub_grid.height + 1;
		let mut origin_x:usize = position[0];
		let mut origin_y:usize = position[1];
		while origin_x > end_x {
			origin_x -= self.width;
			origin_y += 1;
		}
		while origin_y < end_y {
			while origin_x < end_x {
				if self.find_at_position(sub_grid, max_allowed_mismatches, [origin_x, origin_y], self_row_shift) {
					return Some([origin_x, origin_y]);
				}
				origin_x += 1;
			}
			origin_x = 0;
			origin_y += 1;
		}

		// Not found.
		None
	}

	/// Check if a sub-grid is at a specific position in self.
	fn find_at_position(&self, sub_grid:&Grid<T>, max_allowed_mismatches:usize, position:[usize; 2], self_row_shift:usize) -> bool {

		let mut mismatches:usize = 0;
		let mut self_index:usize = position[1] * self.width + position[0];
		let mut sub_index:usize = 0;
		for _sub_y in 0..sub_grid.height {
			for _sub_x in 0..sub_grid.width {
				if self[self_index] != sub_grid[sub_index] {
					mismatches += 1;
					if mismatches > max_allowed_mismatches {
						return false;
					}
				}
				self_index += 1;
				sub_index += 1;
			}
			self_index += self_row_shift;
		}
		return true;
	}



	/* MASKED SUB-GRID FINDING METHODS */

	/// Find the given sub-grid in self. Returns the topleft coordinates of the first position where the similarity reaches the given threshold, only matching the positive pixels of the given mask.
	pub fn find_masked(&self, sub_grid:&Grid<T>, mask:&GridMask, similarity_threshold_factor:f32) -> Option<[usize; 2]> {
		let comparing_pixel_count:usize = sub_grid.width * sub_grid.height;
		let max_mismatches:usize = ((1.0 - similarity_threshold_factor) * comparing_pixel_count as f32).round() as usize;
		self.find_starting_at_position_masked(sub_grid, mask, max_mismatches, [0, 0])
	}

	/// Find all instances of the given sub-grid in self. Returns the topleft coordinates of the all positions where the similarity reaches the given threshold, only matching the positive pixels of the given mask.
	pub fn find_all_masked(&self, sub_grid:&Grid<T>, mask:&GridMask, similarity_threshold_factor:f32) -> Vec<[usize; 2]> {
		let comparing_pixel_count:usize = sub_grid.width * sub_grid.height;
		let max_mismatches:usize = ((1.0 - similarity_threshold_factor) * comparing_pixel_count as f32).round() as usize;
		let mut cursor:[usize; 2] = [0, 0];
		let mut results:Vec<[usize; 2]> = Vec::new();
		while let Some(position) = self.find_starting_at_position_masked(sub_grid, mask, max_mismatches, cursor) {
			results.push(position);
			cursor = [position[0] + 1, position[1]];
		}
		results
	}
	
	/// Find the given sub-grid in self. Returns the topleft coordinates of the first position where the similarity reaches the given threshold, only matching the positive pixels of the given mask. Starts at the given position.
	pub fn find_starting_at_position_masked(&self, sub_grid:&Grid<T>, mask:&GridMask, max_allowed_mismatches:usize, position:[usize; 2]) -> Option<[usize; 2]> {

		// Validate grids same size.
		if let Err(error) = self.validate_findable_grids(sub_grid, None) {
			eprintln!("{}", error);
			return None;
		}

		// Initialize variables used in the loop.
		let mask_grid:&Grid<bool> = mask.grid();
		let self_row_shift:usize = self.width - sub_grid.width;
		let end_x:usize = self.width - sub_grid.width + 1;
		let end_y:usize = self.height - sub_grid.height + 1;
		let mut origin_x:usize = position[0];
		let mut origin_y:usize = position[1];
		while origin_x > end_x {
			origin_x -= self.width;
			origin_y += 1;
		}

		// Loop through all possible top-left positions.
		while origin_y < end_y {
			while origin_x < end_x {
				if self.find_at_position_masked(sub_grid, mask_grid, max_allowed_mismatches, [origin_x, origin_y], self_row_shift) {
					return Some([origin_x, origin_y]);
				}
				origin_x += 1;
			}
			origin_x = 0;
			origin_y += 1;
		}

		// Not found.
		None
	}

	/// Check if a sub-grid is at a specific position in self. Only count pixels matching the mask.
	fn find_at_position_masked(&self, sub_grid:&Grid<T>, mask_grid:&Grid<bool>, max_allowed_mismatches:usize, position:[usize; 2], self_row_shift:usize) -> bool {
		let mut mismatches:usize = 0;
		let mut self_index:usize = position[1] * self.width + position[0];
		let mut sub_index:usize = 0;
		for _sub_y in 0..sub_grid.height {
			for _sub_x in 0..sub_grid.width {
				if mask_grid[sub_index] && self[self_index] != sub_grid[sub_index] {
					mismatches += 1;
					if mismatches > max_allowed_mismatches {
						return false;
					}
				}
				self_index += 1;
				sub_index += 1;
			}
			self_index += self_row_shift;
		}
		return true;
	}
}