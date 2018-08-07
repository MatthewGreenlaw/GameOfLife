extern crate rand;
use self::rand::Rng;

extern crate ggez;
use ggez::{Context, GameResult};

use coord::{Coord};
use params::{
	WIDTH_GAME_GRIDS, HEIGHT_GAME_GRIDS
};

//Coordinates for ggez to draw Coords
struct Cell { coord: Coord, }

impl Cell {
	pub fn new(x: i32, y: i32) -> Self {
		Cell {	coord:Coord::new(x, y), }
	}

	//Tell ggez what color to render a Cell
	//https://docs.rs/ggez/0.3.1/ggez/graphics/index.html
	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		self.coord.draw(ctx)
	}
}

impl Clone for Cell {
	fn clone(&self) -> Cell { Cell {coord: self.coord.clone() }}
}

//Define properties of the game
pub struct World { map: Vec<Vec<Option<Cell>>>, generation:i32, living: i32, dead: i32,}

impl World {
	//Generate a new world given its population
	pub fn new (num_pop: i32) -> Self {
		let mut range = rand::thread_rng();
		let mut locals: Vec<Vec<Option<Cell>>> = vec![vec![None; WIDTH_GAME_GRIDS as usize]; HEIGHT_GAME_GRIDS as usize];	

		for _i in 0..num_pop {
			let x = range.gen_range::<i32>(0, WIDTH_GAME_GRIDS as i32);
			let y = range.gen_range::<i32>(0, HEIGHT_GAME_GRIDS as i32);
			locals[y as usize][x as usize] = Some(Cell::new(x, y));
		}

		World { map: locals, generation:0, living: num_pop, dead: 0,}
	}

	pub fn clasic_generation(&mut self) {
		//Capture the state of this generation's map
		let generation: Vec<Vec<Option<Cell>>> = self.map.to_vec();
		let mut born = 0;
		let mut died = 0;

		//Evaluate each cell for living neighbors 
		for (y, row) in generation.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				let live_neighbors = World::num_neighbors(&generation, x as i32, y as i32);
				//Some = populated, check for population collaps
				//None = unpopulated, check for population growth
				match cell {
					Some(_) => { if live_neighbors < 2 || live_neighbors > 3 { self.map[y][x] = None; died +=1} },
					None => { if live_neighbors == 3 { self.map[y][x] = Some(Cell::new(x as i32,y as i32)); born += 1;} },
				}
			}
		}

		self.generation += 1;
		self.living += born - died;
		self.dead += died;
	}

	fn num_neighbors(map:&Vec<Vec<Option<Cell>>>, x:i32, y:i32) -> i32 {
		let neighbor = |x:i32, y:i32| {
			match map[y as usize][x as usize] {
				Some(_) => 1, //Living neighbor
				None    => 0, //Dead neighbor
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

		// c n
		// n n
		if x == 0 && y == 0 {
			right(x, y) + bottom(x, y) + bright(x, y)
		}
		// n c
		// n n
		else if x == WIDTH_GAME_GRIDS-1 && y == 0 {
			left(x, y)+bleft(x, y)+bottom(x, y)
		}
		// n n
		// c n
		else if x == 0 && y == HEIGHT_GAME_GRIDS-1 {
			top(x, y)+tright(x, y)+right(x, y)
		}
		// n n
		// n c
		else if x == WIDTH_GAME_GRIDS-1 && y == HEIGHT_GAME_GRIDS-1 {
			tleft(x, y)+top(x, y)+left(x, y)
		}
		// n n
		// c n
		// n n
		else if x == 0 && y > 0 && y < HEIGHT_GAME_GRIDS-1 {
			top(x, y)+tright(x, y)+right(x, y)+bottom(x, y)+bright(x, y)
		}
		// n c n
		// n n n
		else if y == 0 && x > 0 && x < WIDTH_GAME_GRIDS-1 {
			left(x, y)+right(x, y)+bleft(x, y)+bottom(x, y)+bright(x, y)
		}
		// n n
		// n c
		// n n
		else if x == WIDTH_GAME_GRIDS-1 && y > 0 && y < HEIGHT_GAME_GRIDS-1 {
			tleft(x, y)+top(x, y)+left(x, y)+bleft(x, y)+bottom(x, y)
		}
		// n n n
		// n c n
		else if y == HEIGHT_GAME_GRIDS-1 && x > 0 && x < WIDTH_GAME_GRIDS-1 {
			tleft(x, y)+top(x, y)+tright(x, y)+left(x, y)+right(x, y)
		}
		// n n n
		// n c n
		// n n n
		else {
			tleft(x, y)+top(x, y)+tright(x, y)+left(x, y)+right(x, y)+bleft(x, y)+bottom(x, y)+bright(x, y)
		}
	}

	pub fn draw_all_living(&mut self, ctx: &mut Context) -> GameResult<()> {
		for row in self.map.iter() {
			for pos in row.iter() {
				match pos {
					Some(cell) => cell.draw(ctx)?,
					None => (),
				}
			}
		}
		Ok(())
	}

	pub fn update(&mut self,) -> GameResult<(i32, i32, i32)> {
		self.clasic_generation();
		Ok((self.generation, self.living, self.dead))//Update for game over scenario?
	}

	pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.draw_all_living(ctx)
	}
}