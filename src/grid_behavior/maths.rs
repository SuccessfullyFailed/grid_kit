macro_rules! impl_maths {
	($trait:ident, $fn_name:ident, $modification:expr) => {
		impl<T> std::ops::$trait<crate::Grid<T>> for crate::Grid<T> where T:std::ops::$trait<Output=T> + Clone + 'static {
			type Output = crate::Grid<T>;
		
			fn $fn_name(mut self, modification:crate::Grid<T>) -> Self::Output {
				let modification_function:&'static dyn Fn(T, T) -> T = $modification;
				for y in 0..self.height.min(modification.height) {
					for x in 0..self.width.min(modification.width) {
						let left_index:usize = y * self.width + x;
						let right_index:usize = y * modification.width + x;
						self[left_index] = modification_function(self[left_index].clone(), modification[right_index].clone())
					}
				}
				self
			}
		}
	};
}
macro_rules! impl_maths_assign {
	($trait:ident, $fn_name:ident, $modification:expr) => {
		impl<T> std::ops::$trait<crate::Grid<T>> for crate::Grid<T> where T:std::ops::$trait + Clone + 'static {
			fn $fn_name(&mut self, modification:crate::Grid<T>) {
				let modification_function:&'static dyn Fn(&mut T, T) = $modification;
				for y in 0..self.height.min(modification.height) {
					for x in 0..self.width.min(modification.width) {
						let left_index:usize = y * self.width + x;
						let right_index:usize = y * modification.width + x;
						modification_function(&mut self[left_index], modification[right_index].clone())
					}
				}
			}
		}
	};
}



impl_maths!(Add, add, &|left, right| left.clone() + right.clone());
impl_maths!(Sub, sub, &|left, right| left.clone() - right.clone());
impl_maths!(Mul, mul, &|left, right| left.clone() * right.clone());
impl_maths!(Div, div, &|left, right| left.clone() / right.clone());

impl_maths_assign!(AddAssign, add_assign, &|left, right| *left += right);
impl_maths_assign!(SubAssign, sub_assign, &|left, right| *left -= right);
impl_maths_assign!(MulAssign, mul_assign, &|left, right| *left *= right);
impl_maths_assign!(DivAssign, div_assign, &|left, right| *left /= right);