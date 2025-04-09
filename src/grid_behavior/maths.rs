macro_rules! impl_maths {
	($trait:ident, $fn_name:ident, $modification:expr) => {
		impl<T> std::ops::$trait<crate::Grid<T>> for crate::Grid<T> where T:std::ops::$trait<Output=T> + Clone + 'static {
			type Output = crate::Grid<T>;
		
			fn $fn_name(mut self, addition:crate::Grid<T>) -> Self::Output {
				let modification_function:&'static dyn Fn(T, T) -> T = $modification;
				for y in 0..self.height.min(addition.height) {
					for x in 0..self.width.min(addition.width) {
						let left_index:usize = y * self.width + x;
						let right_index:usize = y * addition.width + x;
						self[left_index] = modification_function(self[left_index].clone(), addition[right_index].clone())
					}
				}
				self
			}
		}
	};
}
impl_maths!(Add, add, &|left, right| left.clone() + right.clone());
impl_maths!(Sub, sub, &|left, right| left.clone() - right.clone());
impl_maths!(Mul, mul, &|left, right| left.clone() * right.clone());
impl_maths!(Div, div, &|left, right| left.clone() / right.clone());