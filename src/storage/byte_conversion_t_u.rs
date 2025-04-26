#[cfg(test)]
mod tests {
	use crate::storage::ByteConversion;



	/* HELPER MACROS */

	macro_rules! test_int_type_conversion {
		($type:ty, $type_size:expr) => {
			let mut value:$type = 1;
			for bit_index in 0..$type_size * 8 {
				println!("Testing byte conversion for value {value}");
				assert_eq!(value, <$type>::from_bytes(&value.as_bytes()).unwrap());
				assert_eq!(value, <$type>::from_consume_bytes(&mut value.as_bytes()).unwrap());
				value = (value << 1) ^ (if bit_index % 4 == 0 { 1 } else { 0 });
			}
		};
	}



	/* INTEGER TEST METHODS */

	#[test]
	fn test_type_byte_conversion_u8() {
		test_int_type_conversion!(u8, 1);
	}

	#[test]
	fn test_type_byte_conversion_u16() {
		test_int_type_conversion!(u16, 2);
	}

	#[test]
	fn test_type_byte_conversion_u32() {
		test_int_type_conversion!(u32, 4);
	}

	#[test]
	fn test_type_byte_conversion_u64() {
		test_int_type_conversion!(u64, 8);
	}

	#[test]
	fn test_type_byte_conversion_u128() {
		test_int_type_conversion!(u128, 16);
	}

	#[test]
	fn test_type_byte_conversion_i8() {
		test_int_type_conversion!(i8, 1);
	}

	#[test]
	fn test_type_byte_conversion_i16() {
		test_int_type_conversion!(i16, 2);
	}

	#[test]
	fn test_type_byte_conversion_i32() {
		test_int_type_conversion!(i32, 4);
	}

	#[test]
	fn test_type_byte_conversion_i64() {
		test_int_type_conversion!(i64, 8);
	}

	#[test]
	fn test_type_byte_conversion_i128() {
		test_int_type_conversion!(i128, 16);
	}



	/* FLOATING POINT TEST METHODS */

	#[test]
	fn test_type_byte_conversion_f32() {
		const MIN:f32 = 0.0000000001;
		const MAX:f32 = 1000000000.0;

		let mut value:f32 = MIN;
		while value < MAX {
			println!("Testing byte conversion for value {value}");
			assert_eq!(value, f32::from_bytes(&value.as_bytes()).unwrap());
			assert_eq!(value, f32::from_consume_bytes(&mut value.as_bytes()).unwrap());
			value = (value * 10.0) + MIN;
		}
	}

	#[test]
	fn test_type_byte_conversion_f64() {
		const MIN:f64 = 0.0000000001;
		const MAX:f64 = 1000000000.0;

		let mut value:f64 = MIN;
		while value < MAX {
			println!("Testing byte conversion for value {value}");
			assert_eq!(value, f64::from_bytes(&value.as_bytes()).unwrap());
			assert_eq!(value, f64::from_consume_bytes(&mut value.as_bytes()).unwrap());
			value = (value * 10.0) + MIN;
		}
	}



	/* MISCELLANEOUS TEST METHODS */

	#[test]
	fn test_type_byte_conversion_bool() {
		for value in [true, false] {
			println!("Testing byte conversion for value {value}");
			assert_eq!(value, bool::from_bytes(&value.as_bytes()).unwrap());
			assert_eq!(value, bool::from_consume_bytes(&mut value.as_bytes()).unwrap());
		}
	}

	#[test]
	fn test_type_byte_conversion_string() {
		for value in ["test_lowercase", "TEST_UPPERCASE", "TEST_MIXED_CASE", "TEST_SPECIAL_CASE 1234567890!@#$%^&*()-_=+[]{};':\",>\\"] {
			println!("Testing byte conversion for value {value}");
			assert_eq!(value, String::from_bytes(&ByteConversion::as_bytes(&value.to_string())).unwrap());
			assert_eq!(value, String::from_consume_bytes(&mut ByteConversion::as_bytes(&value.to_string())).unwrap());
		}
	}

	#[test]
	fn test_type_byte_conversion_array() {
		let numbers_list:[u16; 25] = (0..25).map(|index| index * 25).collect::<Vec<u16>>().try_into().unwrap();
		println!("Testing byte conversion for value {:?}", numbers_list);
		assert_eq!(numbers_list, <[u16; 25]>::from_bytes(&numbers_list.as_bytes()).unwrap());
		assert_eq!(numbers_list, <[u16; 25]>::from_consume_bytes(&mut numbers_list.as_bytes()).unwrap());
	}

	#[test]
	fn test_type_byte_conversion_list() {
		let numbers_list:Vec<u16> = (0..25).map(|index| index * 25).collect();
		println!("Testing byte conversion for value {:?}", numbers_list);
		assert_eq!(numbers_list, Vec::<u16>::from_bytes(&numbers_list.as_bytes()).unwrap());
		assert_eq!(numbers_list, Vec::<u16>::from_consume_bytes(&mut numbers_list.as_bytes()).unwrap());
	}

	#[test]
	fn test_type_byte_conversion_recursive_list() {
		let numbers_list:Vec<Vec<u16>> = (0..2).map(|y| (0..3).map(|x| y * 10 + x).collect()).collect();
		println!("Testing byte conversion for value {:?}", numbers_list);
		assert_eq!(numbers_list, Vec::<Vec<u16>>::from_bytes(&numbers_list.as_bytes()).unwrap());
		assert_eq!(numbers_list, Vec::<Vec<u16>>::from_consume_bytes(&mut numbers_list.as_bytes()).unwrap());
	}
}