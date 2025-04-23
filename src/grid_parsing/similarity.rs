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

	/// Get the factor of the similarity between this grid and another. If minimum_similarity is set, will return 1.0 if similarity exceeds threshold, otherwise returns 0.0. If grids aren't the same size, prints warning and returns default value.
	pub fn similarity_to(&self, other:&Grid<T>, settings:&SimilaritySettings<T>) -> f32 {
		if let Some(invalidation_value) = self.validate_settings(other, &settings) {
			return invalidation_value;
		}

		// Parse data based on masked/unmasked.
		let (self_data, other_data) = match &settings.mask {
			Some(mask) => (self.masked_data(&mask), other.masked_data(&mask)),
			None => (self.data.iter().collect(), other.data.iter().collect())
		};

		// Count mismatches in data.
		let max_matches:usize = self_data.len();
		let mismatches:usize = match &settings.minimum_similarity {

			// With similarity threshold.
			Some(minimum_similarity) => {
				let maximum_allowed_mismatches:usize = (max_matches as f32 * (1.0 - minimum_similarity)).round() as usize;
				let mut mismatches:usize = 0;
				for (a, b) in self.data.iter().zip(&other.data) {
					if !(settings.comparing_method)(a, b) {
						mismatches += 1;
						if mismatches > maximum_allowed_mismatches {
							break;
						}
					}
				}
				if mismatches > maximum_allowed_mismatches { max_matches } else { 0 }
			},

			// Without similarity threshold.
			None => max_matches - self_data.iter().zip(&other_data).filter(|(a, b)| (settings.comparing_method)(a, b)).count() // Subtraction prevents having to flip every result.
		};
		
		// Return the matching factor.
		let matches:usize = max_matches - mismatches;
		matches as f32 / max_matches as f32
	}
}