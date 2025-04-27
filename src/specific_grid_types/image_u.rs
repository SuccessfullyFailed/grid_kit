#[cfg(test)]
mod tests {
	use crate::Color;



	#[test]
	fn test_color() {
		let mut color:Color = Color(0x01234567);

		assert_eq!(color.0, 0x01234567);
		assert_eq!(color.a(), &0x01);
		assert_eq!(color.r(), &0x23);
		assert_eq!(color.g(), &0x45);
		assert_eq!(color.b(), &0x67);

		*color.a_mut() = 0x89;
		*color.r_mut() = 0xAB;
		*color.g_mut() = 0xCD;
		*color.b_mut() = 0xEF;

		assert_eq!(color.0, 0x89ABCDEF);
		assert_eq!(color.a(), &0x89);
		assert_eq!(color.r(), &0xAB);
		assert_eq!(color.g(), &0xCD);
		assert_eq!(color.b(), &0xEF);
	}
}