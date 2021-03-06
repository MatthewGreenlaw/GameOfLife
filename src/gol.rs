// Copyright 2016 Matthew Greenlaw.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// A basic Game of Life implementation for the ggez 2d gaming environment. 

extern crate rand;
use self::rand::Rng;

extern crate ggez;
use ggez::{Context, graphics};

use params::{
	SIZE_GRID_PIXELS,
	WIDTH_GAME_GRIDS, HEIGHT_GAME_GRIDS
};

/// Defines the map and meta data
pub struct World {
	/// A 2D vector of options representing cells on the map:
	/// * A cell is alive if it is true
	/// * A cell is dead if it is false
	map: Vec<Vec<bool>>,
	///Tracks the number of times the world updates 
	generation:i32,
	///Tracks living cells
	living: i32, 
	///Tracks total of living cells that died
	dead: i32,
}

impl World {
	/// Generates a new world given its population
	///
	/// # Arguments
	///
	/// * 'num_pop' - The initial population. Each population is located randomly.
	pub fn new (num_pop: i32) -> Self {
		let mut range = rand::thread_rng();
		let mut locals: Vec<Vec<bool>> = vec![vec![false; WIDTH_GAME_GRIDS as usize]; HEIGHT_GAME_GRIDS as usize];  

		for _i in 0..num_pop {
			let x = range.gen_range::<i32>(0, WIDTH_GAME_GRIDS as i32);
			let y = range.gen_range::<i32>(0, HEIGHT_GAME_GRIDS as i32);
			locals[y as usize][x as usize] = true;
		}

		World { map: locals, generation:0, living: num_pop, dead: 0,}
	}

	/// Used only in testing. 
	/// Generates a new world given values for each data member
	///
	/// # Arguments
	///
	/// * 'map' - The map to use
	/// * 'generation' - The desired starting generation
	/// * 'living' - The desired count of living cells
	/// * 'dead' - The desired count of dead cells
	#[allow(dead_code)]
	pub fn set_all (map: Vec<Vec<bool>>, generation:i32, living: i32, dead: i32) -> Self {
		World { 
			map: map, 
			generation: generation, 
			living: living, 
			dead: dead,
		}
	}

	/// The classic rules of a Game of Life generation
	/// * Make a static copy of the map to evaluate while updating the working map
	/// * Gather the number of neighbors a cell has
	/// * Kill a living cell if there are too many/few neighbors
	/// * Spawn life in a dead cell if it has exactly 3 living neighbors
	pub fn clasic_generation(&mut self) {
		//Capture the state of this generation's map
		let generation: Vec<Vec<bool>> = self.map.to_vec();
		let mut born = 0;
		let mut died = 0;

		//Evaluate each cell for living neighbors 
		for (y, row) in generation.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				let live_neighbors = World::num_neighbors(&generation, x as i32, y as i32);
				//Some = populated, check for population collaps
				//None = unpopulated, check for population growth
				match cell {
					true => { if live_neighbors < 2 || live_neighbors > 3 { self.map[y][x] = false; died +=1} },
					false => { if live_neighbors == 3 { self.map[y][x] = true; born += 1;} },
				}
			}
		}

		self.generation += 1;
		self.living += born - died;
		self.dead += died;
	}

	/// Determines the number of living neighbors around a target in a map of coordinates
	///
	/// # Arguments
	/// * 'map' - The map of coordinates.
	/// * 'x & y' - The target coordinate.
	///
	/// # Return
	/// * i32 - The number of living neighbors around the target.
	pub fn num_neighbors(map:&Vec<Vec<bool>>, x:i32, y:i32) -> i32 {
		let neighbor = |x:i32, y:i32| {
			match map[y as usize][x as usize] {
				true => 1, //Living neighbor
				false => 0, //Dead neighbor
			}
		};

		//Only check indexes that need to be checked
		let tleft  = |a:i32, b:i32| neighbor(a-1, b-1);
		let top    = |a:i32, b:i32| neighbor(a  , b-1);
		let tright = |a:i32, b:i32| neighbor(a+1, b-1);
		let left   = |a:i32, b:i32| neighbor(a-1, b  );
		let right  = |a:i32, b:i32| neighbor(a+1, b  );
		let bleft  = |a:i32, b:i32| neighbor(a-1, b+1);
		let bottom = |a:i32, b:i32| neighbor(a  , b+1);
		let bright = |a:i32, b:i32| neighbor(a+1, b+1);   

		// t n
		// n n
		if x == 0 && y == 0 {
			right(x, y) + bottom(x, y) + bright(x, y)
		}
		// n t
		// n n
		else if x == WIDTH_GAME_GRIDS-1 && y == 0 {
			left(x, y)+bleft(x, y)+bottom(x, y)
		}
		// n n
		// t n
		else if x == 0 && y == HEIGHT_GAME_GRIDS-1 {
			top(x, y)+tright(x, y)+right(x, y)
		}
		// n n
		// n t
		else if x == WIDTH_GAME_GRIDS-1 && y == HEIGHT_GAME_GRIDS-1 {
			tleft(x, y)+top(x, y)+left(x, y)
		}
		// n n
		// t n
		// n n
		else if x == 0 && y > 0 && y < HEIGHT_GAME_GRIDS-1 {
			top(x, y)+tright(x, y)+right(x, y)+bottom(x, y)+bright(x, y)
		}
		// n t n
		// n n n
		else if y == 0 && x > 0 && x < WIDTH_GAME_GRIDS-1 {
			left(x, y)+right(x, y)+bleft(x, y)+bottom(x, y)+bright(x, y)
		}
		// n n
		// n t
		// n n
		else if x == WIDTH_GAME_GRIDS-1 && y > 0 && y < HEIGHT_GAME_GRIDS-1 {
			tleft(x, y)+top(x, y)+left(x, y)+bleft(x, y)+bottom(x, y)
		}
		// n n n
		// n t n
		else if y == HEIGHT_GAME_GRIDS-1 && x > 0 && x < WIDTH_GAME_GRIDS-1 {
			tleft(x, y)+top(x, y)+tright(x, y)+left(x, y)+right(x, y)
		}
		// n n n
		// n t n
		// n n n
		else {
			tleft(x, y)+top(x, y)+tright(x, y)+left(x, y)+right(x, y)+bleft(x, y)+bottom(x, y)+bright(x, y)
		}
	}

	/// Updates cells in the map and returns a tuple of world meta data.
	/// Required function for ggez
	///
	/// # Return
	/// * (i32, i32, i32) - Tripplet of:
	///   * i32 - Current generation
	///   * i32 - Current number of living cells
	///   * i32 - Current total of living cells that died
	pub fn update(&mut self) -> (i32, i32, i32) {
		self.clasic_generation();
		(self.generation, self.living, self.dead)
	}

	/// Passes draw command to living cells. Used to [draw in ggez](https://docs.rs/ggez/0.4.1/ggez/graphics/fn.draw.html).
	///
	/// # Arguments
	/// * 'ctx' - [ggez global resources](https://docs.rs/ggez/0.3.1/ggez/struct.Context.html). 
	pub fn draw(&mut self, ctx: &mut Context) {
		let topix = |x:i32| { ((x * SIZE_GRID_PIXELS) as i32) };
		let boarder = 1;

		//Loop over cells in the map
		for (y, row) in self.map.iter().enumerate() {
			for (x, life) in row.iter().enumerate() {
				//Draw living cells
				match life {
					true => {
						//set_color(r: f32, g: f32, b: f32 a: f32)
						graphics::set_color(ctx, [0.5, 0.5, 0.5, 0.9].into()).expect("Error setting color"); 
						graphics::rectangle(ctx, 
							graphics::DrawMode::Fill,
							//Rect(x: f32, y: f32, w: f32, h: f32)
							graphics::Rect::new_i32 (
								topix(x as i32), 
								topix(y as i32), 
								topix(1)-boarder, 
								topix(1)-boarder,
							)
						).expect("Error drawing Rect");
					},
					false => (),
				}
			}
		}
	}
}

#[test]
fn test_gol_update_and_classic_generation() {
	//update calls classic generation and uses the values it sets.

	//update -> (generation, living, dead)
	//update causes a generation, and since the map only has one life it should kill it, 
	//leaving no living and one dead
	let mut world = World::new(1);
	assert_eq!((1,0,1), world.update());

	//Three live cells in a row causes a blinker structure which kills two cells each generation and creates two living cells.
	//For each update, generation should go up by one, living should stay the same, and dead should go up by two.
	let mut locals: Vec<Vec<bool>> = vec![vec![false; WIDTH_GAME_GRIDS as usize]; HEIGHT_GAME_GRIDS as usize];
	locals [1][0] = true;
	locals [1][1] = true;
	locals [1][2] = true;
	world = World::set_all(locals, 0, 3, 0);
	assert_eq!((1,3,2), world.update());
	assert_eq!((2,3,4), world.update());
}

#[test]
fn test_gol_num_neighbors() {
	let mut locals: Vec<Vec<bool>> = vec![vec![false; WIDTH_GAME_GRIDS as usize]; HEIGHT_GAME_GRIDS as usize];
	for y in 0..HEIGHT_GAME_GRIDS {
		for x in 0..WIDTH_GAME_GRIDS {
			
			locals [y as usize][x as usize] = true;

			if y == 0 {
				//top left corner
				if x == 0 {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y+1), "Should have one neighbor.");
				}
				//top right corner
				else if x == WIDTH_GAME_GRIDS-1 {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y+1), "Should have one neighbor.");
				}
				//top row
				else {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y+1), "Should have one neighbor.");
				}
			}
			else if y == HEIGHT_GAME_GRIDS-1 {
				//bottom left corner
				if x == 0 {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y), "Should have one neighbor.");
				}
				//bottom right corner
				else if x == WIDTH_GAME_GRIDS-1 {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y), "Should have one neighbor.");
				}
				//bottom row
				else {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y), "Should have one neighbor.");
				}
			}
			else {
				//left column
				if x == 0 {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y+1), "Should have one neighbor.");
				}
				//right column
				else if x == WIDTH_GAME_GRIDS-1 {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y+1), "Should have one neighbor.");
				}
				//Anywhere in the middle
				else {
					assert_eq!(0, World::num_neighbors(&locals, x, y), "Should have no neighbors.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y-1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x-1, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x, y+1), "Should have one neighbor.");
					assert_eq!(1, World::num_neighbors(&locals, x+1, y+1), "Should have one neighbor.");
				}
			}

			locals [y as usize][x as usize] = false;
		}
	}
} 