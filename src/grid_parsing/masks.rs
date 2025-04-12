use crate::Grid;


pub type Mask = Grid<bool>;



impl<T> Grid<T> where T:PartialEq + 'static {

	/// Create a mask based on which values pass the given function.
	pub fn create_mask<U>(&self, comparing_function:U) -> Mask where U:Fn(&T) -> bool + 'static {
		self.map_ref(comparing_function)
	}

	/// Create a mask based on pixels that match the specific value.
	pub fn create_value_mask(&self, value:T) -> Mask {
		self.map_ref(move |field_value| field_value == &value)
	}
}
impl<T> Grid<T> where T:Default {
	
	/// Apply a mask to self that sets all mismatches to default value.
	pub fn apply_mask(&mut self, mask:&Mask) {
		for (index, value) in self.data.iter_mut().enumerate() {
			if !mask[index] {
				*value = T::default();
			}
		}
	}
}
impl<T> Grid<T> where T:Default + PartialEq {
	
	/// Create and apply a mask in one step.
	pub fn mask<U>(&mut self, comparing_function:U) where U:Fn(&T) -> bool + 'static {
		for value in &mut self.data {
			if !comparing_function(&value) {
				*value = T::default();
			}
		}
	}
}