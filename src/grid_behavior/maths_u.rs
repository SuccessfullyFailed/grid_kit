#[cfg(test)]
mod tests {
	use crate::Grid;



	fn test_maths_modification(operation:&'static dyn Fn(Grid<i32>, Grid<i32>) -> Grid<i32>, assign_operation:&'static dyn Fn(&mut Grid<i32>, Grid<i32>), validation_operation:&'static dyn Fn(i32, i32) -> i32, operation_print:&str) {
		let grid:Grid<i32> = Grid::new((1..10).map(|value| value * 10).collect(), 3, 3);
		let modifier:Grid<i32> = Grid::new((1..10).map(|value| value * 2).collect(), 3, 3);
		let result:Grid<i32> = operation(grid.clone(), modifier.clone());
		let mut result_assign:Grid<i32> = grid.clone();
		assign_operation(&mut result_assign, modifier.clone());
		println!("[grid]\n{grid}\n");
		println!("{operation_print}\n");
		println!("[modifier]\n{modifier}\n");
		println!("=>\n");
		println!("[result]\n{result}\n");
		println!("[result assign]\n{result_assign}\n");

		for (index, (item, item_assign)) in result.into_iter().zip(result_assign).enumerate() {
			let expected:i32 =  validation_operation((index as i32 + 1) * 10, (index as i32 + 1) * 2);
			println!("{index}:\t{item} && {item_assign}\t\t(should be {expected})");
			assert_eq!(item, expected);
			assert_eq!(item_assign, expected);
		}
	}



	#[test]
	fn test_add() {
		test_maths_modification(
			&|left, right| left + right,
			&|left, right| *left += right,
			&|left, right| left + right,
			"+"
		);
	}

	#[test]
	fn test_sub() {
		test_maths_modification(
			&|left, right| left - right,
			&|left, right| *left -= right,
			&|left, right| left - right,
			"-"
		);
	}

	#[test]
	fn test_mult() {
		test_maths_modification(
			&|left, right| left * right,
			&|left, right| *left *= right,
			&|left, right| left * right,
			"*"
		);
	}

	#[test]
	fn test_div() {
		test_maths_modification(
			&|left, right| left / right,
			&|left, right| *left /= right,
			&|left, right| left / right,
			"/"
		);
	}

	

	#[test]
	fn test_bitand() {
		test_maths_modification(
			&|left, right| left & right,
			&|left, right| *left &= right,
			&|left, right| left & right,
			"&"
		);
	}

	#[test]
	fn test_bitor() {
		test_maths_modification(
			&|left, right| left | right,
			&|left, right| *left |= right,
			&|left, right| left | right,
			"|"
		);
	}

	#[test]
	fn test_bitxor() {
		test_maths_modification(
			&|left, right| left ^ right,
			&|left, right| *left ^= right,
			&|left, right| left ^ right,
			"^"
		);
	}

	#[test]
	fn test_shl() {
		test_maths_modification(
			&|left, right| left << right,
			&|left, right| *left <<= right,
			&|left, right| left << right,
			"<<"
		);
	}

	#[test]
	fn test_shr() {
		test_maths_modification(
			&|left, right| left >> right,
			&|left, right| *left >>= right,
			&|left, right| left >> right,
			">>"
		);
	}
}