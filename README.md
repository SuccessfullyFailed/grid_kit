# GridKit 🗺️

A Rust library for creating, manipulating, and analyzing **grids**.  
GridKit provides flexible tools for math operations, iteration, region finding, pathfinding, sub-grid matching, type conversions, and even reading/writing grids as PNG images.

---

## ✨ Features

- **Grid creation & modification**  
  Build and transform 2D grids of any type.

- **Iteration & math utilities**  
  Iterate efficiently, map values, and apply transformations.

- **Region & sub-grid detection**  
  Find connected areas, extract sub-grids, or compare grids.

- **Pathfinding**  
  Run weighted/unweighted pathfinding across the grid.

- **Optional features**  
  - `byte_conversion`: Convert grids to and from raw bytes.  
  - `image_conversion`: Read & write PNGs as grids.
  - `screen_capture`: Allows capturing (part of) the screen on windows.

---

## 📦 Installation

Add **GridKit** to your `Cargo.toml`:

```toml
[dependencies]
gridkit = { git="https://github.com/SuccessfullyFailed/grid_kit" }
```

---

## 🚀 Example

Here’s a small example that:

1. Loads a map image as a grid.  
2. Detects walkable areas by comparing color differences.  
3. Generates a weighted map that favors the center of roads.  
4. Finds a path through the walkable region.  

```rust
use gridkit::{ Grid, GridRegion, GridMask };

const START_POSITION:[usize; 2] = [400, 630];
const TARGET_POSITION:[usize; 2] = [640, 100];

fn main() {
	let original_map:Grid<u32> = Grid::from_png("README_img/1 map_original.png").unwrap();

	// 1. Find walkable area
	let mut walkable_map:GridRegion = original_map.region_at(START_POSITION, |from_color, to_color| {
		let from_rgb:Vec<u8> = from_color.to_be_bytes()[1..].to_vec();
		let to_rgb:Vec<u8> = to_color.to_be_bytes()[1..].to_vec();
		from_rgb.iter().zip(&to_rgb)
			.map(|(a, b)| (a.max(b) - a.min(b)) as u16)
			.sum::<u16>() < 0x50
	});
	walkable_map.remove_edge(5);
	walkable_map.grid().to_png("README_img/2 walkable_map.png").unwrap();

	// 2. Weight the map by distance to edges
	let mask:GridMask = GridMask::new(walkable_map.grid().clone());
	let edge_map:Grid<usize> = walkable_map.to_edge_distance_map();
	let max_dist:u8 = *edge_map.iter().max().unwrap() as u8;
	let weighted_map:Grid<u8> = edge_map.map(move |d| max_dist + 1 - d as u8).masked(&mask);
	weighted_map.to_png("README_img/3 walkable_map_weighed.png").unwrap();

	// 3. Pathfinding
	const COST:fn((usize, &u8), (usize, &u8)) -> Option<usize> = |(_from_index, _from), (_to_index, to)| if *to == 0 { None } else { Some(*to as usize) };
	let path:Vec<[usize]> = weighted_map.find_path_weighed(START_POSITION, TARGET_POSITION, COST).unwrap();

	let mut path_map:Grid<u32> = original_map;
	for pos in path {
		path_map[pos] = 0xFF00FF00; // draw green path
	}
	path_map.to_png("README_img/4 path_map.png").unwrap();
}
```

---

## 🖼️ Demo Output

| Original map | Walkable region | Weighted walkable map | Pathfinding result |
|--------------|-----------------|------------------------|--------------------|
| ![](README_img/1%20map_original.png) | ![](README_img/2%20walkable_map.png) | ![](README_img/3%20walkable_map_weighed.png) | ![](README_img/4%20path_map.png) |

*(The map images included here are provided **only as sample input/output** to demonstrate GridKit’s functionality. They are not part of the library itself and are not affiliated with or endorsed by the creators of Titanfall.)*

---

## 📝 License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)  
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

---