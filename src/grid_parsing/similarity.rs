use crate::Grid;



pub type CompareEqualMethod<T> = &'static dyn Fn(&T, &T) -> bool;



impl<T> Grid<T> {

	// If grids are not the same size, show a warning and return the default value.
	fn validate_equal_size(&self, other:&Grid<T>) -> Option<f32> {
		const DEFAULT_VALUE:f32 = 0.0;

		if self.data.len() != other.data.len() {
			eprintln!("Trying to get similarity between grids with buffers of sizes {} and {}. Can only get similarity between grids of the same size. Returnin 0.0 similarity.", self.data.len(), other.data.len());
			Some(DEFAULT_VALUE)
		} else {
			None
		}
	}

	/// Get the factor of the similarity between this grid and another. If grids aren't the same size, prints warning and returns default value.
	pub fn similarity_to_using_method(&self, other:&Grid<T>, comparing_method:CompareEqualMethod<T>) -> f32 {
		if let Some(invalidation_value) = self.validate_equal_size(other) {
			return invalidation_value;
		}
		1.0 / (self.width * self.height) as f32 * self.data.iter().zip(&other.data).filter(|(a, b)| comparing_method(a, b)).count() as f32
	}

	/// Wether or not the similarity between this grid and another is greater than the given amount. If grids aren't the same size, prints warning and returns default value.
	pub fn similarity_to_greater_than_using_method(&self, other:&Grid<T>, minimum_similarity:f32, comparing_method:CompareEqualMethod<T>) -> bool {
		if let Some(invalidation_value) = self.validate_equal_size(other) {
			return invalidation_value > minimum_similarity;
		}
		let maximum_allowed_mismatches:usize = ((self.width * self.height) as f32 * (1.0 - minimum_similarity)).round() as usize;
		let mut mismatches:usize = 0;
		for (a, b) in self.data.iter().zip(&other.data) {
			if !comparing_method(a, b) {
				mismatches += 1;
				if mismatches > maximum_allowed_mismatches {
					return false;
				}
			}
		}
		true
	}
}
impl<T> Grid<T> where T:PartialEq + 'static {

	/// Get the factor of the similarity between this grid and another. If grids aren't the same size, prints warning and returns default value.
	pub fn similarity_to(&self, other:&Grid<T>) -> f32 {
		self.similarity_to_using_method(other, &|a, b| a == b)
	}

	/// Wether or not the similarity between this grid and another is greater than the given amount. If grids aren't the same size, prints warning and returns default value.
	pub fn similarity_to_greater_than(&self, other:&Grid<T>, minimum_similarity:f32) -> bool {
		self.similarity_to_greater_than_using_method(other, minimum_similarity, &|a, b| a == b)
	}
}