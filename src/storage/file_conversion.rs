use crate::{ Grid, storage::GridByteConvertible };
use file_ref::FileRef;
use std::error::Error;



impl<T> Grid<T> where T:GridByteConvertible {

	/// Save the grid to a file.
	pub fn save_to_file(&self, file_path:&str) -> Result<(), Box<dyn Error>> {
		FileRef::new(file_path).write_bytes(&self.to_bytes())
	}

	/// Create a grid by reading a file.
	pub fn read_from_file(file_path:&str) -> Result<Grid<T>, Box<dyn Error>> {
		let file:FileRef = FileRef::new(file_path);
		if !file.exists() {
			return Err(format!("Could not read grid from file '{file_path}', file does not exist.").into());
		}
		let file_bytes:Vec<u8> = file.read_bytes()?;
		Grid::<T>::from_bytes(&file_bytes)
	}
}