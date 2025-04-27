use super::{masks::Maskable, GridMask};
use crate::Grid;



pub type CompareEqualMethod<T> = &'static dyn Fn(&T, &T) -> bool;



pub struct SimilaritySettings<T> where T:'static {
	comparing_method:CompareEqualMethod<T>,
	minimum_similarity:Option<f32>, // Setting this setting will modify similarity finding methods to return 0.0 when over the threshold or 1.0 when under it.
	mask:Option<GridMask>
}
impl<T> SimilaritySettings<T> {

	/// Create a new settings set.
	pub fn new(comparing_method:CompareEqualMethod<T>) -> SimilaritySettings<T> {
		SimilaritySettings {
			comparing_method,
			minimum_similarity: None,
			mask: None
		}
	}

	/// Return self with a minimum similarity. Setting this setting will modify similarity finding methods to return 0.0 when over the threshold or 1.0 when under it.
	pub fn with_minimum_similarity(mut self, minimum_similarity:f32) -> Self {
		self.minimum_similarity = Some(minimum_similarity);
		self
	}

	/// Return self with a mask.
	pub fn with_mask<U>(mut self, mask:U) -> Self where U:Maskable {
		self.mask = Some(mask.as_mask());
		self
	}

}
impl<T> Default for SimilaritySettings<T> where T:PartialEq {
	fn default() -> Self {
		SimilaritySettings {
			comparing_method: &|a, b| a == b,
			minimum_similarity: None,
			mask: None
		}
	}
}



impl<T> Grid<T> {

	/* SIMILARITY USAGE  METODS */

	/// Get the factor of the similarity between this grid and another. If minimum_similarity is set, will return 1.0 if similarity exceeds threshold, otherwise returns 0.0. If grids aren't the same size, prints warning and returns default value.
	pub fn similarity_to(&self, other:&Grid<T>, settings:&SimilaritySettings<T>) -> f32 {
		if let Some(invalidation_value) = self.validate_settings(other, &settings) {
			return invalidation_value;
		}

		// Call less ambiguous sub-functions to find the similarity depending on settings.
		let similarity:f32 = if let Some(mask) = &settings.mask {
			self.similarity_processor_masked(other, settings.comparing_method, settings.minimum_similarity, mask)
		} else {
			self.similarity_processor_default(other, settings.comparing_method, settings.minimum_similarity)
		};

		// Return similarity.
		match settings.minimum_similarity {
			Some(min) => if similarity >= min { 1.0 } else { 0.0 },
			None => similarity
		}
	}

	// If grids are not the same size, show a warning and return the default value.
	fn validate_settings(&self, other:&Grid<T>, settings:&SimilaritySettings<T>) -> Option<f32> {
		const DEFAULT_VALUE:f32 = 0.0;

		if self.data.len() != other.data.len() {
			eprintln!("Trying to get similarity between grids with buffers of sizes {} and {}. Can only get similarity between grids of the same size. Returning {} similarity.", self.data.len(), other.data.len(), DEFAULT_VALUE);
			Some(DEFAULT_VALUE)
		} else if settings.mask.is_some() && self.data.len() != settings.mask.as_ref().unwrap().grid().data.len() {
			eprintln!("Trying to get similarity between grids of sizes {} with mask of size {}. Can only get similarity between grids and masks of the same size. Returning {} similarity.", self.data.len(), settings.mask.as_ref().unwrap().grid().data.len(), DEFAULT_VALUE);
			Some(DEFAULT_VALUE)
		} else {
			None
		}
	}



	/* SUB-GRID FINDING METHODS */
	
	/// Find a sub-grid in this grid. If no minimum simmilarity is specified, the function will assume a full match is required.
	pub fn find(&self, sub_grid:&Grid<T>, settings:&SimilaritySettings<T>) -> Option<[usize; 2]> {
		self.find_starting_at_offset(sub_grid, settings, [0; 2])
	}
	
	/// Find a sub-grid in this grid, starting at a specific offset. If no minimum simmilarity is specified, the function will assume a full match is required.
	fn find_starting_at_offset(&self, sub_grid:&Grid<T>, settings:&SimilaritySettings<T>, offset:[usize; 2]) -> Option<[usize; 2]> {
		if sub_grid.width == 0 || sub_grid.height == 0 || self.width < sub_grid.width || self.height < sub_grid.height {
			return None;
		}

		// Prepare data to match sub-fields taken from self later.
		let sub_grid_data:Vec<&[T]> = sub_grid.sub_field_data([0, 0, sub_grid.width, sub_grid.height]);

		// Find the scanning bounds.
		let max_allowed_mismatches:usize = settings.minimum_similarity.map(|similarity| (sub_grid.data.len() as f32 * (1.0 - similarity)) as usize).unwrap_or(0);
		let scan_end_x:usize = self.width - sub_grid.width + 1;
		let scan_end_y:usize = self.height - sub_grid.height + 1;
		let mut cursor:[usize; 2] = offset;
		while cursor[1] < scan_end_y {

			// Check if sub-field at position matches sub-grid.
			let mut mismatches:usize = 0;
			let source_field:Vec<&[T]> = self.sub_field_data([cursor[0], cursor[1], sub_grid.width, sub_grid.height]);
			match &settings.mask {

				// Masked compare.
				Some(mask) => {
					let mut cursor:[usize; 2] = [0, 0];
					let max_x:usize = sub_grid_data[0].len();
					let max_y:usize = sub_grid_data.len();
					while cursor[1] < max_y {
						if mask.grid()[cursor] && !(settings.comparing_method)(&sub_grid_data[cursor[1]][cursor[0]],  &source_field[cursor[1]][cursor[0]]) {
							mismatches += 1;
							if mismatches > max_allowed_mismatches {
								break;
							}
						}
						cursor[0] += 1;
						if cursor[0] == max_x {
							cursor = [0, cursor[1] + 1];
						}
					}
				}

				// Unmasked compare.
				None => {
					for (left, right) in sub_grid_data.iter().zip(source_field) {
						mismatches += Self::list_mismatch_counter(*left, right, settings.comparing_method, Some(max_allowed_mismatches - mismatches));
						if mismatches > max_allowed_mismatches {
							break;
						}
					}
				}
			}
			if mismatches <= max_allowed_mismatches {
				return Some(cursor);
			}

			// Incrmeent cursor.
			cursor[0] += 1;
			if cursor[0] == scan_end_x {
				cursor =  [0, cursor[1] + 1];
			}
		}
		
		// Sub-grid not found.
		None
	}
	


	/* SIMILARITY PROCESSOR METHODS */

	/// Get the amount of mismatches between 2 masked datasets.
	fn similarity_processor_masked(&self, other:&Grid<T>, compare_method:CompareEqualMethod<T>, minimum_similarity_factor:Option<f32>, mask:&GridMask) -> f32 {

		// Create masked datasets.
		let self_data:Vec<&[T]> = self.masked_data(&mask);
		let other_data:Vec<&[T]> = other.masked_data(&mask);
		
		// Calculate max allowed mismatches.
		let max_possible_matches:usize = self_data.len();
		let max_allowed_mismatches:Option<usize> = minimum_similarity_factor.map(|similarity| (1.0 - similarity)).map(|difference| (max_possible_matches as f32 * difference) as usize);

		// Find mismatches.
		let mut mismatches:usize = 0;
		for (left, right) in self_data.iter().zip(other_data) {
			mismatches += Self::list_mismatch_counter(left, right, compare_method, max_allowed_mismatches.map(|max| max - mismatches));
			if max_allowed_mismatches.is_some() && mismatches > max_allowed_mismatches.unwrap() {
				break;
			}
		}
		
		// Return similarity.
		(max_possible_matches - mismatches) as f32 / max_possible_matches as f32
	}

	/// Get the amount of mismatches between 2 masked datasets.
	fn similarity_processor_default(&self, other:&Grid<T>, compare_method:CompareEqualMethod<T>, minimum_similarity_factor:Option<f32>) -> f32 {
		let max_possible_matches:usize = self.data.len();
		let max_allowed_mismatches:Option<usize> = minimum_similarity_factor.map(|similarity| (1.0 - similarity)).map(|difference| (max_possible_matches as f32 * difference) as usize);
		let mismatches:usize = Self::list_mismatch_counter(&self.data, &other.data, compare_method, max_allowed_mismatches);
		(max_possible_matches - mismatches) as f32 / max_possible_matches as f32
	}



	/* MISMATCH COUNTER METHODS */

	/// Default similarity processor method. Counts mismatches between grids.
	fn list_mismatch_counter(left_data:&[T], right_data:&[T], compare_method:CompareEqualMethod<T>, max_allowed_mismatches:Option<usize>) -> usize {
		match &max_allowed_mismatches {

			// With max mismatches.
			Some(max_allowed_mismatches) => {
				let mut mismatches:usize = 0;
				for (a, b) in left_data.iter().zip(right_data) {
					if !compare_method(a, b) {
						mismatches += 1;
						if mismatches > *max_allowed_mismatches {
							return mismatches;
						}
					}
				}
				mismatches
			},

			// Without similarity threshold.
			None => left_data.len() - left_data.iter().zip(right_data).filter(|(a, b)| compare_method(a, b)).count() // Subtraction prevents having to flip every result.
		}
	}
}