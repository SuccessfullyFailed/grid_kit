use std::{ slice::{ Iter, IterMut }, vec::IntoIter };
use crate::Grid;



impl<T> Grid<T> {

	/// Iterate over pixels with their according X and Y coordinate.
	pub fn pixel_iterator(&self) -> PixelIterator<T> {
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
	pub fn pixel_iterator_mut(&mut self) -> PixelIteratorMut<T> {
		PixelIteratorMut {
			grid: self,
			x: 0,
			y: 0,
			index: 0
		}
	}
}
impl<T> IntoIterator for Grid<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
    
	fn into_iter(self) -> Self::IntoIter {
	    self.data.into_iter()
	}
}
impl<'a, T> IntoIterator for &'a Grid<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.data.iter()
	}
}
impl<'a, T> IntoIterator for &'a mut Grid<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.data.iter_mut()
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
impl<'a, T> Iterator for PixelIteratorMut<'a, T> {
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