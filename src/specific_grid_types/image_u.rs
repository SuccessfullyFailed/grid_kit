#[cfg(test)]
mod tests {
	use std::{ sync::Mutex, thread::sleep, time::Duration };
	use crate::{ Color, Grid };



	#[test]
	fn test_color_creation() {
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

	#[test]
	fn test_color_addition() {
		assert_eq!(Color(0x00000000) + Color(0x00000000), Color(0x00000000)); // Both invisible, return invisible.
		assert_eq!(Color(0x00000000) + Color(0x00112233), Color(0x00000000)); // Addition has color, but no opacity, return original.
		assert_eq!(Color(0xFF112233) + Color(0xFF224466), Color(0xFF224466)); // Addition has full opacity, return addition.
		assert_eq!(Color(0x10000000) + Color(0x80224466), Color(0x87112233)); // Self has no color and a little bit opacity, addition has half opacity, return half of addition with own opacity added.
		assert_eq!(Color(0xFF00FF00) + Color(0x80FF0000), Color(0xFF807E00)); // Self is green, rhs is half opacity red, return red-green combination. Is allowed to deviate due to color handling.
	}


	static CREATE_COLOR_TEST_IMAGE:Mutex<bool> = Mutex::new(true);
	#[test]
	fn create_manual_color_addition_test_image() {
		sleep(Duration::from_millis(10));
		if *CREATE_COLOR_TEST_IMAGE.lock().unwrap() {
			Grid::new(
				(0..0xFF).map(|y|
					(0..0xFF).map(|x| 
						Color::new([0xFF, 0xFF, 0x00, 0x00]) + [x, 0x00, 0xFF, 0x00] + [y, 0x00, 0x00, 0xFF]
					).collect::<Vec<Color>>()
				).flatten().collect::<Vec<Color>>(),
				0xFF,
				0xFF
			)
			.to_bmp("target/color_addition_test_display.bmp").expect("Could not create manual color test image");
		}
	}
	#[test]
	fn disable_manual_color_addition_test_image() {
		*CREATE_COLOR_TEST_IMAGE.lock().unwrap() = false;
	}
}