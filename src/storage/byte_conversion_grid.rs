use crate::{ Grid, storage::ByteConversion };
use std::error::Error;



type StorageArgType = u32;
const STORAGE_ARG_SIZE:usize = std::mem::size_of::<StorageArgType>();
const STORAGE_ARG_COUNT:usize = 2;
const MIN_BYTES:usize = STORAGE_ARG_SIZE * STORAGE_ARG_COUNT;



impl<T> Grid<T> where T:ByteConversion {

	/// Convert the grid to bytes.
	pub fn to_bytes(&self) -> Vec<u8> {
		let data_size:usize = self.data.len() * if self.data.is_empty() { 0 } else { T::bytes_size(&self.data[0].as_bytes()) };
		let mut bytes:Vec<u8> = Vec::with_capacity(MIN_BYTES + data_size);
		bytes.extend((self.width as StorageArgType).as_bytes());
		bytes.extend((self.height as StorageArgType).as_bytes());
		bytes.extend(self.data.as_bytes());
		bytes
	}

	/// Try to create a grid from bytes.
	pub fn from_bytes(bytes:&[u8]) -> Result<Self, Box<dyn Error>> {

		// Validate initial byte count.
		if bytes.len() < MIN_BYTES {
			return Err(format!("Error creating grid from bytes. Grid from bytes requires at least {} bytes of arguments data. {} bytes provided.", MIN_BYTES, bytes.len()).into());
		}

		// Get args from leading bytes.
		let width:Option<u32> = StorageArgType::from_bytes(&bytes[0..STORAGE_ARG_SIZE]);
		let height:Option<u32> = StorageArgType::from_bytes(&bytes[STORAGE_ARG_SIZE..STORAGE_ARG_SIZE * 2]);
		if width.is_none() || height.is_none() {
			return Err(format!("Error creating grid from bytes. Could not get grid {} from the first bytes.", if width.is_none() { "width" } else { "height" }).into());
		}
		let width:usize = width.unwrap() as usize;
		let height:usize = height.unwrap() as usize;

		// Fetch grid data.
		let data:Option<Vec<T>> = Vec::from_bytes(&bytes[MIN_BYTES..]);
		if data.is_none() {
			return Err("Error creating grid from bytes. Could not get data from provided bytes.".into());
		}

		// Create and return grid.
		Ok(Grid::new(data.unwrap(), width, height))
	}
}