pub trait ByteConversion:Clone {

	/// Represent the datatype as bytes.
	fn as_bytes(&self) -> Vec<u8>;

	/// Create the datatype from bytes.
	fn from_bytes(bytes:&[u8]) -> Option<Self>;

	/// Create the value from a mutable list of bytes. Consumes the bytes used for the value.
	fn from_consume_bytes(bytes:&mut Vec<u8>) -> Option<Self>;
	
	/// Get the size in bytes of the type.
	fn bytes_size(source_bytes:&[u8]) -> usize;
}



/* NUMERIC IMPLEMENTATIONS */

macro_rules! implement_byte_conversion_for_numberic {
	($type:ty, $type_size:expr) => {
		impl ByteConversion for $type {

			/// Represent the datatype as bytes.
			fn as_bytes(&self) -> Vec<u8> {
				self.to_be_bytes().to_vec()
			}
			
			/// Create the datatype from bytes.
			fn from_bytes(bytes:&[u8]) -> Option<Self> {
				if let Ok(bytes) = bytes.try_into() {
					Some(<$type>::from_be_bytes(bytes))
				} else {
					None
				}
			}

			/// Create the value from a mutable list of bytes. Consumes the bytes used for the value.
			fn from_consume_bytes(bytes:&mut Vec<u8>) -> Option<Self> {
				if bytes.len() >= $type_size {
					Self::from_bytes(&bytes.drain(0..$type_size).collect::<Vec<u8>>())
				} else {
					None
				}
			}
			
			/// Get the size in bytes of the type.
			fn bytes_size(_source_bytes:&[u8]) -> usize {
				$type_size
			}
		}
	};
}
implement_byte_conversion_for_numberic!(u8, 1);
implement_byte_conversion_for_numberic!(u16, 2);
implement_byte_conversion_for_numberic!(u32, 4);
implement_byte_conversion_for_numberic!(u64, 8);
implement_byte_conversion_for_numberic!(u128, 16);
implement_byte_conversion_for_numberic!(i8, 1);
implement_byte_conversion_for_numberic!(i16, 2);
implement_byte_conversion_for_numberic!(i32, 4);
implement_byte_conversion_for_numberic!(i64, 8);
implement_byte_conversion_for_numberic!(i128, 16);
implement_byte_conversion_for_numberic!(f32, 4);
implement_byte_conversion_for_numberic!(f64, 8);



/* MISCELLANEOUS IMPLEMENTATIONS */

impl ByteConversion for bool {
	
	/// Represent the datatype as bytes.
	fn as_bytes(&self) -> Vec<u8> {
		vec![if *self { 1 } else { 0 }]
	}

	/// Create the datatype from bytes.
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		if bytes.len() == 1 {
			Some(bytes[0] != 0)
		} else {
			None
		}
	}

	/// Create the value from a mutable list of bytes. Consumes the bytes used for the value.
	fn from_consume_bytes(bytes:&mut Vec<u8>) -> Option<Self> {
		if !bytes.is_empty() {
			Some(bytes.remove(0) != 0)
		} else {
			None
		}
	}

	/// Get the size in bytes of the type.
	fn bytes_size(_source_bytes:&[u8]) -> usize {
		1
	}
}
impl ByteConversion for String {

	/// Represent the datatype as bytes.
	fn as_bytes(&self) -> Vec<u8> {
		self.chars().map(|c| c as u8).collect::<Vec<u8>>().as_bytes()
	}

	/// Create the datatype from bytes.
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		Vec::<u8>::from_bytes(bytes).map(|chars| chars.into_iter().map(|char| char as char).collect::<String>())
	}

	/// Create the value from a mutable list of bytes. Consumes the bytes used for the value.
	fn from_consume_bytes(bytes:&mut Vec<u8>) -> Option<Self> {
		Vec::<u8>::from_consume_bytes(bytes).map(|chars| chars.into_iter().map(|char| char as char).collect::<String>())
	}

	/// Get the size in bytes of the type.
	fn bytes_size(source_bytes:&[u8]) -> usize {
		Vec::<u8>::bytes_size(source_bytes)
	}
}



/* LIST IMPLEMENTATIONS */

impl<T, const LENGTH:usize> ByteConversion for [T; LENGTH] where T:ByteConversion {

	/// Represent the datatype as bytes.
	fn as_bytes(&self) -> Vec<u8> {
		self.to_vec().as_bytes()
	}

	/// Create the datatype from bytes.
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		if let Some(list) = Vec::<T>::from_bytes(bytes) {
			if list.len() == LENGTH {
				if let Ok(array) = list.try_into() {
					return Some(array);
				}
			}
		}
		None
	}

	/// Create the value from a mutable list of bytes. Consumes the bytes used for the value.
	fn from_consume_bytes(bytes:&mut Vec<u8>) -> Option<Self> {
		if let Some(list) = Vec::<T>::from_bytes(bytes) {
			if list.len() == LENGTH {
				if let Ok(array) = list.try_into() {
					bytes.drain(..Self::bytes_size(bytes));
					return Some(array);
				}
			}
		}
		None
	}

	/// Get the size in bytes of the type.
	fn bytes_size(source_bytes:&[u8]) -> usize {
		Vec::<T>::bytes_size(source_bytes)
	}
}
impl<T> ByteConversion for Vec<T> where T:ByteConversion {

	/// Represent the datatype as bytes.
	fn as_bytes(&self) -> Vec<u8> {
		let mut bytes:Vec<u8> = Vec::with_capacity(4 + self.len());
		bytes.extend((self.len() as u32).as_bytes());
		bytes.extend(self.iter().map(|value| value.as_bytes()).flatten().collect::<Vec<u8>>());
		bytes
	}

	/// Create the datatype from bytes.
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		if bytes.len() >= 4 {
			if let Some(entry_count) = u32::from_bytes(&bytes[..4]).map(|c| c as usize) {
				let mut values:Vec<T> = Vec::new();
				let mut cursor:usize = 4;
				let mut t_size:usize = T::bytes_size(&bytes[cursor..]);
				let mut found_entries:usize = 0;
				while found_entries < entry_count && bytes.len() - cursor >= t_size {
					if let Some(value) = T::from_bytes(&bytes[cursor..cursor + t_size]) {
						values.push(value);
					} else {
						return None;
					}
					cursor += t_size;
					found_entries += 1;
					t_size = T::bytes_size(&bytes[cursor..]);
				}
				if values.len() == entry_count && cursor >= bytes.len() {
					return Some(values);
				}
			}
		}
		None
	}

	/// Create the value from a mutable list of bytes. Consumes the bytes used for the value.
	fn from_consume_bytes(bytes:&mut Vec<u8>) -> Option<Self> {
		if bytes.len() >= 4 {
			if let Some(entry_count) = u32::from_bytes(&bytes[..4]).map(|c| c as usize) {
				let mut values:Vec<T> = Vec::new();
				let mut cursor:usize = 4;
				let mut t_size:usize = T::bytes_size(&bytes[cursor..]);
				let mut found_entries:usize = 0;
				while found_entries < entry_count && bytes.len() - cursor >= t_size {
					if let Some(value) = T::from_bytes(&bytes[cursor..cursor + t_size]) {
						values.push(value);
					} else {
						return None;
					}
					cursor += t_size;
					found_entries += 1;
					t_size = T::bytes_size(&bytes[cursor..]);
				}
				if values.len() == entry_count && cursor >= bytes.len() {
					bytes.drain(..cursor);
					return Some(values);
				}
			}
		}
		None
	}

	/// Get the size in bytes of the type.
	fn bytes_size(source_bytes:&[u8]) -> usize {
		if source_bytes.len() > 4 {
			if let Some(entry_count) = u32::from_bytes(&source_bytes[..4]).map(|c| c as usize) {
				let mut cursor:usize = 4;
				let mut t_size:usize = T::bytes_size(&source_bytes[cursor..]);
				let mut found_entries:usize = 0;
				while found_entries < entry_count && source_bytes.len() - cursor >= t_size {
					cursor += t_size;
					found_entries += 1;
					t_size = T::bytes_size(&source_bytes[cursor..]);
				}
				if found_entries == entry_count {
					return cursor;
				}
			}
		}
		0
	}
}