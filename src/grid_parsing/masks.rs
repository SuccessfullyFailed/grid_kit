use std::ops::Range;
use crate::Grid;



#[derive(Clone, PartialEq)]
pub struct GridMask {
	source_grid:Grid<bool>,
	positive_ranges:Vec<Range<usize>>,
	negative_ranges:Vec<Range<usize>>
}
impl GridMask {

	/* CONSTRUCTOR METHODS */

	/// Create a new mask.
	pub fn new(source_grid:Grid<bool>) -> GridMask {

		// Find ranges.
		let grid_size:usize = source_grid.width * source_grid.height;
		let mut positive_ranges:Vec<Range<usize>> = Vec::with_capacity(grid_size);
		let mut negative_ranges:Vec<Range<usize>> = Vec::with_capacity(grid_size);
		if !source_grid.data.is_empty() {
			let mut range_start:usize = 0;
			let mut range_start_value:&bool = &source_grid.data[0];
			for (cursor, cursor_value) in source_grid.data().iter().enumerate().skip(1) {
				if cursor_value != range_start_value {
					if *range_start_value {
						positive_ranges.push(range_start..cursor);
					} else {
						negative_ranges.push(range_start..cursor);
					}
					range_start = cursor;
					range_start_value = cursor_value;
				}
			}
			if *range_start_value {
				positive_ranges.push(range_start..source_grid.data.len());
			} else {
				negative_ranges.push(range_start..source_grid.data.len());
			}
		}

		// Create grid.
		GridMask {
			source_grid,
			positive_ranges,
			negative_ranges
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the source grid.
	pub fn grid(&self) -> &Grid<bool> {
		&self.source_grid
	}

	/// Get the positive ranges.
	pub fn positive_ranges(&self) -> &[Range<usize>] {
		&self.positive_ranges
	}

	/// Get the negative ranges.
	pub fn negative_ranges(&self) -> &[Range<usize>] {
		&self.negative_ranges
	}

	/// Get the width.
	pub fn width(&self) -> usize {
		self.source_grid.width
	}

	/// Get the height.
	pub fn height(&self) -> usize {
		self.source_grid.height
	}
}



pub trait Maskable {
	fn as_mask(self) -> GridMask;
}
impl Maskable for GridMask {
	fn as_mask(self) -> GridMask {
		self
	}
}
impl Maskable for Grid<bool> {
	fn as_mask(self) -> GridMask {
		GridMask::new(self)
	}
}
impl<T, U> Maskable for (Grid<T>, U) where T:PartialEq + 'static, U:Fn(&T) -> bool + 'static {
	fn as_mask(self) -> GridMask {
		self.0.create_mask(self.1)
	}
}



impl<T> Grid<T> {

	/// Return the data of self filtered by the mask.
	pub(crate) fn masked_data(&self, mask:&GridMask) -> Vec<&[T]> {
		mask.positive_ranges.iter().map(|range| &self.data[range.clone()]).collect()
	}
}
impl<T> Grid<T> where T:PartialEq + 'static {

	/// Create a mask based on which values pass the given function.
	pub fn create_mask<U>(&self, comparing_function:U) -> GridMask where U:Fn(&T) -> bool + 'static {
		GridMask::new(self.map_ref(comparing_function))
	}

	/// Create a mask based on pixels that match the specific value.
	pub fn create_value_mask(&self, value:T) -> GridMask {
		self.create_mask(move |a| *a == value)
	}
}
impl<T> Grid<T> where T:Default {
	
	/// Apply a mask to self that sets all mismatches to default value.
	pub fn apply_mask(&mut self, mask:&GridMask) {
		assert_eq!([self.width, self.height], [mask.source_grid.width, mask.source_grid.height], "Mask application requires grid and mask to be the same size");
		for range in &mask.negative_ranges {
			for index in range.clone() {
				self[index] = T::default();
			}
		}
	}
}
impl<T> Grid<T> where T:Default + Clone {
	
	/// Return a version of self masked by the given mask.
	pub fn masked(&self, mask:&GridMask) -> Self {
		let mut clone:Grid<T> = self.clone();
		clone.apply_mask(mask);
		clone
	}
}
impl<T> Grid<T> where T:Default + PartialEq + 'static {
	
	/// Create and apply a mask in one step.
	pub fn mask<U>(&mut self, comparing_function:U) where U:Fn(&T) -> bool + 'static {
		for value in &mut self.data {
			if !comparing_function(&value) {
				*value = T::default();
			}
		}
	}
	
	/// Create and apply a mask in one step.
	pub fn value_mask(&mut self, value:T) {
		self.mask(move |a| *a == value)
	}
}