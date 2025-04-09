use std::{ slice::{ Iter, IterMut }, vec::IntoIter };
use crate::Grid;



impl<T> Grid<T> {

	/// Iterate over the data in the grid.
	pub fn iter(&self) -> Iter<'_, T> {
		self.data.iter()
	}

	/// Iterate over the mutable data in the grid.
	pub fn iter_mut(&mut self) -> IterMut<'_, T> {
		self.data.iter_mut()
	}

	/// Consume self into an iterater over the data in the grid.
	pub fn into_iter(self) -> IntoIter<T> {
		self.data.into_iter()
	}

	/// Iterate over pixels with their according X and Y coordinate.
	pub fn pixel_iterator<'a>(&'a self) -> PixelIterator<'a, T> {
		PixelIterator {
			grid: self,
			grid_len: self.data.len(),
			x: 0,
			y: 0,
			index: 0
		}
	}
}
impl<T> Grid<T> where Grid<T>:Sized {

	/// Iterate over mutable pixels with their according X and Y coordinate.
	pub fn pixel_iterator_mut<'a>(&'a mut self) -> PixelIteratorMut<'a, T> {
		PixelIteratorMut {
			grid: self,
			x: 0,
			y: 0,
			index: 0
		}
	}
}



pub struct PixelIterator<'a, T> {
	grid:&'a Grid<T>,
	grid_len:usize,
	x:usize,
	y:usize,
	index:usize
}
impl<'a, T> Iterator for PixelIterator<'a, T> {
	type Item = (usize, usize, &'a T);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.grid_len {
			let value:Self::Item = (self.x, self.y, &self.grid.data[self.index]);
			self.index += 1;
			self.x += 1;
			if self.x == self.grid.width {
				self.x = 0;
				self.y += 1;
			}
			Some(value)
		} else {
			None
		}
	}
}



pub struct PixelIteratorMut<'a, T> {
	grid:&'a mut Grid<T>,
	x:usize,
	y:usize,
	index:usize
}
impl<'a, 'b, T> Iterator for PixelIteratorMut<'a, T> {
	type Item = (usize, usize, &'a mut T);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.grid.len() {
			let value:Self::Item = (self.x, self.y, unsafe { &mut *(self.grid.data.as_mut_ptr()).add(self.index) });
			self.index += 1;
			self.x += 1;
			if self.x == self.grid.width {
				self.x = 0;
				self.y += 1;
			}
			Some(value)
		} else {
			None
		}
	}
}