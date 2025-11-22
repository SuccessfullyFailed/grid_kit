use crate::{ Grid, GridMask };



pub struct GridMatcher<SourceType:PartialEq + Default, TargetType:PartialEq> {
	filter:Box<dyn Fn(SourceType) -> TargetType + Send  + Sync + 'static>,
	area_of_interest:Option<[usize; 4]>,
	mask:Option<GridMask>,
	named_entries:Vec<(String, Grid<TargetType>)>
}
impl<SourceType:PartialEq + Default, TargetType:PartialEq> GridMatcher<SourceType, TargetType> {

	/* CONSTRUCTOR METHODS */

	/// Create a new grid-matcher.
	pub fn new<Filter:Fn(SourceType) -> TargetType + Send  + Sync + 'static>(filter:Filter) -> GridMatcher<SourceType, TargetType> {
		GridMatcher {
			filter: Box::new(filter),
			area_of_interest: None,
			mask: None,
			named_entries: Vec::new()
		}
	}

	/// Return self with an area of interest.
	pub fn with_area_of_interest(mut self, area_of_interest:[usize; 4]) -> Self {
		self.area_of_interest = Some(area_of_interest);
		self
	}

	/// Return self with a mask.
	pub fn with_mask(mut self, mask:GridMask) -> Self {
		self.mask = Some(mask);
		self
	}

	/// Return self with an additional named entry.
	pub fn with_named_entry(mut self, name:&str, grid:Grid<SourceType>) -> Self {
		self.named_entries.push((name.to_string(), self.process_grid(grid)));
		self
	}



	/* PROPERTY GETTER METHODS */

	/// Get the stored named entries.
	#[cfg(test)]
	pub(crate) fn named_entries(&self) -> &[(String, Grid<TargetType>)] {
		&self.named_entries
	}



	/* USAGE METHODS */

	/// From a grid of the input-type, returns a grid of the output type, fully processed from all modifiers in own properties.
	fn process_grid(&self, mut grid:Grid<SourceType>) -> Grid<TargetType> {
		if let Some(aoi) = self.area_of_interest {
			if grid.width != aoi[2] || grid.height != aoi[3] { // Make sure grid was not trimmed to AOI already.
				grid = grid.take(aoi);
			}
		}
		if let Some(mask) = &self.mask {
			grid.apply_mask(mask);
		}
		grid.map(&self.filter)
	}

	/// Find the name and similarity factor of the stored entry that is most similar to the given grid.
	pub fn most_similar_to(&self, grid:Grid<SourceType>) -> Option<(&str, f32)> {
		let grid:Grid<TargetType> = self.process_grid(grid);
		let mut most_similar:Option<(&str, f32)> = None;
		for (name, entry) in &self.named_entries {
			let similarity:f32 = match &self.mask {
				Some(mask) => grid.similarity_to_masked(entry, mask),
				None => grid.similarity_to(entry)
			};
			if most_similar.is_none() || most_similar.unwrap().1 < similarity {
				most_similar = Some((name, similarity));
			}
		}
		most_similar
	}

	/// Find the name of the first stored entry that is at least as similar to the given grid as the given factor.
	pub fn first_similar_to(&self, grid:Grid<SourceType>, similarity_threshold:f32) -> Option<&str> {
		let grid:Grid<TargetType> = self.process_grid(grid);
		for (name, entry) in &self.named_entries {
			let is_similar:bool = match &self.mask {
				Some(mask) => grid.similar_to_masked(entry, similarity_threshold, mask),
				None => grid.similar_to(entry, similarity_threshold)
			};
			if is_similar {
				return Some(name);
			}
		}
		None
	}
}



#[cfg(feature="file_storage")]
#[cfg(feature="png_conversion")]
use file_ref::FileRef;
#[cfg(feature="file_storage")]
#[cfg(feature="png_conversion")]
use crate::{ ColorConvertible, GridByteConvertible };
#[cfg(feature="file_storage")]
#[cfg(feature="png_conversion")]
use std::error::Error;
#[cfg(feature="file_storage")]
#[cfg(feature="png_conversion")]
pub struct CachedGridMatcher<SourceType:PartialEq + Default, TargetType:PartialEq + GridByteConvertible> {
	source_dir:FileRef,
	cache_dir:FileRef,
	grid_matcher:GridMatcher<SourceType, TargetType>,
}
#[cfg(feature="file_storage")]
#[cfg(feature="png_conversion")]
impl<SourceType:PartialEq + Default + ColorConvertible, TargetType:PartialEq + GridByteConvertible + Default + ColorConvertible> CachedGridMatcher<SourceType, TargetType> {
	const CACHE_DIR_NAME:&str = "_grid_matcher_cache";
	const SOURCE_FILE_EXTENSION:&str = "png";
	const CACHE_FILE_EXTENSION:&str = "gmc";
	const DEBUG_FILE_EXTENSION:&str = "png";



	/* CONSTRUCTOR METHODS */

	/// Create a new cached grid-matcher.
	pub fn new(source_dir:&str, force_update_all:bool, grid_matcher:GridMatcher<SourceType, TargetType>) -> Result<CachedGridMatcher<SourceType, TargetType>, Box<dyn Error>> {
		let source_dir:FileRef = FileRef::new(source_dir);
		let mut cached_matcher:CachedGridMatcher<SourceType, TargetType> = CachedGridMatcher {
			source_dir: source_dir.clone(),
			cache_dir: source_dir + "/" + Self::CACHE_DIR_NAME,
			grid_matcher
		};

		// Remove existing cache if force-updating or cache is outdated.
		if force_update_all && cached_matcher.cache_dir.exists() {
			cached_matcher.cache_dir.delete()?;
		}

		// Make sure all required dirs exist.
		for dir in [&cached_matcher.source_dir, &cached_matcher.cache_dir] {
			if !dir.exists() {
				dir.create()?;
			}
		}

		// Read existing entries from cache files.
		for cache_file in cached_matcher.cache_files() {
			match cached_matcher.entry_from_cache_file(&cache_file) {
				Ok(named_entry) => cached_matcher.grid_matcher.named_entries.push(named_entry),
				Err(_) => cache_file.delete()? // Recreate and add it later.
			}
		}

		// If any source files don't have cache files, create them and add them to the entries list.
		for source_file in cached_matcher.source_files() {
			let cache_file:FileRef = cached_matcher.cache_for_source(&source_file);
			if !cache_file.exists() {
				cached_matcher.create_cache_file_for(&source_file)?;
				let named_entry:(String, Grid<TargetType>) = cached_matcher.entry_from_cache_file(&cache_file)?;
				cached_matcher.grid_matcher.named_entries.push(named_entry);
			}
		}

		// Return full dir set.
		Ok(cached_matcher)
	}



	/* USAGE METHODS */

	/// Find the name and similarity factor of the stored entry that is most similar to the given grid.
	pub fn most_similar_to(&self, grid:Grid<SourceType>) -> Option<(&str, f32)> {
		self.grid_matcher.most_similar_to(grid)
	}

	/// Find the name of the first stored entry that is at least as similar to the given grid as the given factor.
	pub fn first_similar_to(&self, grid:Grid<SourceType>, similarity_threshold:f32) -> Option<&str> {
		self.grid_matcher.first_similar_to(grid, similarity_threshold)
	}



	/* FILE METHODS */

	/// Get all source files.
	fn source_files(&self) -> Vec<FileRef> {
		self.source_dir.scanner().include_files().filter(|file| file.extension() == Some(Self::SOURCE_FILE_EXTENSION)).collect()
	}

	/// Get all source files.
	fn cache_files(&self) -> Vec<FileRef> {
		self.cache_dir.scanner().include_files().filter(|file| file.extension() == Some(Self::CACHE_FILE_EXTENSION)).collect()
	}

	/// Get the cache path for a source file.
	fn cache_for_source(&self, source:&FileRef) -> FileRef {
		self.cache_dir.clone() + "/" + source.name() + "." + Self::CACHE_FILE_EXTENSION
	}

	/// Get the debug path for a source file.
	fn debug_for_source(&self, source:&FileRef) -> FileRef {
		self.cache_dir.clone() + "/" + source.name() + "_debug." + Self::DEBUG_FILE_EXTENSION
	}



	/* CACHE METHODS */

	/// Get an entry from a cache file.
	fn entry_from_cache_file(&mut self, cache_file:&FileRef) -> Result<(String, Grid<TargetType>), Box<dyn Error>> {
		Ok((cache_file.file_name_no_extension().to_string(), Grid::read_from_file(cache_file.path())?))
	}

	/// Create the cache file for a specific source file.
	fn create_cache_file_for(&self, source:&FileRef) -> Result<(), Box<dyn Error>> {

		// Create cache.
		let image:Grid<SourceType> = Grid::from_png(source.path())?;
		let aoi:Grid<SourceType> = match &self.grid_matcher.area_of_interest {
			Some(aoi) => image.take(*aoi),
			None => image
		};
		let mut filtered_aoi:Grid<TargetType> = aoi.map(|value| (self.grid_matcher.filter)(value));
		if let Some(mask) = &self.grid_matcher.mask {
			filtered_aoi.apply_mask(mask);
		}

		// Store cache and debug image.
		let cache_file:FileRef = self.cache_for_source(&source);
		let debug_file:FileRef = self.debug_for_source(&source);
		filtered_aoi.save_to_file(cache_file.path())?;
		filtered_aoi.to_png(debug_file.path())?;

		// Return success.
		Ok(())
	}
}