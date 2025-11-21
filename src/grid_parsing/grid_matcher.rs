use crate::{ Grid, GridMask };



pub struct GridMatcher<SourceType:PartialEq + Default, TargetType:PartialEq> {
	filter:Box<dyn Fn(SourceType) -> TargetType + 'static>,
	area_of_interest:Option<[usize; 4]>,
	mask:Option<GridMask>,
	named_entries:Vec<(String, Grid<TargetType>)>
}
impl<SourceType:PartialEq + Default, TargetType:PartialEq> GridMatcher<SourceType, TargetType> {

	/* CONSTRUCTOR METHODS */

	/// Create a new window-image recognition set.
	pub fn new<Filter:Fn(SourceType) -> TargetType + 'static>(filter:Filter) -> GridMatcher<SourceType, TargetType> {
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