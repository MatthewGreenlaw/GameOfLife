extern crate rand;
extern crate ggez;
use rand::Rng;
use ggez::{event, Context, GameResult, graphics};
use std::cmp::Ordering;
use std::time::{Duration, Instant};
//Number of pixels per coordinate
//const NUM_PIXEL:u32 = 10;

//Maximum width and depth of the world
const MAX_WIDTH:i32 = 50;
const MAX_DEPTH:i32 = 50;

//Define Map of coordinates to display
const PIXELS: (i32, i32) = (10, 10); //Pixels in a coordinate
//const COORDS: (u32, u32) = (100, 100); //Coordinates in a map

//@todo: Figure out why there is an infinite loop if I don't hard code these
const MAP: (i32, i32) = (MAX_WIDTH * PIXELS.0, MAX_DEPTH * PIXELS.1); //Pixels in the map

const UPDATES_SEC: f32 = 8.0;
const MULLIS_UPDATE: u64 = (1.0 / UPDATES_SEC * 1000.0) as u64;

//Define a coordinate on the map
//representing a cell
#[derive(Clone)]
struct Coordinate {
	x: i32,
	y: i32,
	cell: Location,
}

impl Coordinate {
	pub fn new(x:i32, y:i32) -> Coordinate {
		Coordinate{
			x: x,
			y: y,
			cell: Location::new(),
		}
	}

	//Tell ggez what color to render a location
	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		if self.cell.life { graphics::set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?; }
		else if self.cell.altered { graphics::set_color(ctx, [0.0, 1.0, 0.0, 0.0].into())?; }

		graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new_i32 (
			self.x as i32 * PIXELS.0 as i32, 
			self.y as i32 * PIXELS.1 as i32,
			PIXELS.0 as i32,
			PIXELS.1 as i32,))?;
		Ok(())
	}
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Coordinate) -> bool {
		self.x == other.x &&
		self.y == other.y
	}
}

impl Ord for Coordinate {
	fn cmp(&self, other: &Coordinate) -> Ordering {
		let y_ordering = self.y.cmp(&other.y);

		match self.x.cmp(&other.x) {
			Ordering::Less => Ordering::Less,
			Ordering::Equal => y_ordering,
			Ordering::Greater => Ordering::Greater,
		}
	}
}

impl PartialOrd for Coordinate {
	fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {

		let y_ordering = self.y.partial_cmp(&other.y);

		match self.x.partial_cmp(&other.x) {
			Some(Ordering::Less) => Some(Ordering::Less),
			Some(Ordering::Equal) => y_ordering,
			Some(Ordering::Greater) => Some(Ordering::Greater),
			_ => None,
		}
	}
}

//Converts a Coordinate into something ggez can understand
//Allows us to use .into() on coordinates to type convert them to Rect
//https://docs.rs/ggez/0.3.0/ggez/graphics/struct.Rect.html
//https://doc.rust-lang.org/nightly/core/convert/trait.From.html
impl From<Coordinate> for graphics::Rect {
	fn from(coord: Coordinate) -> Self {
		graphics::Rect::new_i32 (
			coord.x as i32 * PIXELS.0 as i32, 
			coord.y as i32 * PIXELS.1 as i32,
			PIXELS.0 as i32,
			PIXELS.1 as i32,)
	}
}

//Define a location at a coordinate
#[derive(Clone)]
struct Location {
	life: bool,
	altered: bool,
	living_neighbors: u64
}

impl Location {
	pub fn new() -> Location {
		Location {
			life: false,
			altered: true,
			living_neighbors: 0,
		}
	}

	pub fn update(&mut self, health:bool) {
		self.life = health;
		self.altered = true;
	}

	pub fn unaltered(&mut self,) {
		self.altered = false;
	}
}

//Define properties of the game
struct World <'a>{
	cells: &'a mut Vec<Coordinate>,
	time_updated: Instant,
}

impl<'a> World <'a>{
	//Generate a new world given its population
	pub fn new (map: &mut Vec<Coordinate>) -> World {
		// let mut locals: Vec<Coordinate> = Vec::new();
		// let mut range = rand::thread_rng();

		// //Build coordinates on the map
		// for x in 0..MAX_WIDTH {
		// 	for y in 0..MAX_DEPTH { locals.push(Coordinate::new(x, y)); }
		// }

		// //Update random coordinates with life
		// for _i in 0..num_pop {
		// 	locals[range.gen_range::<usize>(0, (MAX_DEPTH * MAX_WIDTH) as usize)].cell.update(true);
		// }

		World {
			cells: map,
			time_updated: Instant::now(),
		}
	}

	pub fn update_neighbors(&mut self, cells:&mut Vec<Coordinate>) {
		//Setup abstractions for dealing with coordinates
		let life_at  = |a, b| match self.cells.get(((a + (b * MAX_WIDTH))) as usize) {
				Some(coord) => coord.cell.life,
				None => panic!("Out of Bounds at: {:?}, {:?}, index of {:?}", a, b, ((a + (b * MAX_WIDTH)))),
			};
		let tleft  = |a, b| life_at(a-1, b-1);
		let top    = |a, b| life_at(a  , b-1);
		let tright = |a, b| life_at(a+1, b-1);
		let left   = |a, b| life_at(a-1, b  );
		let right  = |a, b| life_at(a+1, b  );
		let bleft  = |a, b| life_at(a-1, b+1);
		let bottom = |a, b| life_at(a  , b+1);
		let bright = |a, b| life_at(a+1, b+1);
		
		for coord in self.cells.iter_mut(){
			let x = coord.x;
			let y = coord.y;
			let mut num_life = 0;

			// c n
			// n n
			if x == 0 && y == 0 {
				if right(x, y) { num_life += 1; }
				if bottom(x, y) { num_life += 1; }
				if bright(x, y) { num_life += 1; }
			}
			// n c
			// n n
			else if x == MAX_WIDTH-1 && y == 0 {
				if left(x, y) { num_life += 1; }
				if bleft(x, y) { num_life += 1; }
				if bottom(x, y) { num_life += 1; }
			}
			// n n
			// c n
			else if x == 0 && y == MAX_DEPTH-1 {
				if top(x, y) { num_life += 1; }
				if tright(x, y) { num_life += 1; }
				if right(x, y) { num_life += 1; }
			}
			// n n
			// n c
			else if x == MAX_WIDTH-1 && y == MAX_DEPTH-1 {
				if tleft(x, y) { num_life += 1; }
				if top(x, y) { num_life += 1; }
				if left(x, y) { num_life += 1; }
			}
			// n n
			// c n
			// n n
			else if x == 0 && y > 0 && y < MAX_DEPTH-1 {
				if top(x, y) { num_life += 1; }
				if tright(x, y) { num_life += 1; }
				if right(x, y) { num_life += 1; }
				if bottom(x, y) { num_life += 1; }
				if bright(x, y) { num_life += 1; }
			}
			// n c n
			// n n n
			else if y == 0 && x > 0 && x < MAX_WIDTH-1 {
				if left(x, y) { num_life += 1; }
				if right(x, y) { num_life += 1; }
				if bleft(x, y) { num_life += 1; }
				if bottom(x, y) { num_life += 1; }
				if bright(x, y) { num_life += 1; }
			}
			// n n
			// n c
			// n n
			else if x == MAX_WIDTH-1 && y > 0 && y < MAX_DEPTH-1 {
				if tleft(x, y) { num_life += 1; }
				if top(x, y) { num_life += 1; }
				if left(x, y) { num_life += 1; }
				if bleft(x, y) { num_life += 1; }
				if bottom(x, y) { num_life += 1; }
			}
			// n n n
			// n c n
			else if y == MAX_DEPTH-1 && x > 0 && x < MAX_WIDTH-1 {
				if tleft(x, y) { num_life += 1; }
				if top(x, y) { num_life += 1; }
				if tright(x, y) { num_life += 1; }
				if left(x, y) { num_life += 1; }
				if right(x, y) { num_life += 1; }
			}
			// n n n
			// n c n
			// n n n
			else {
				if tleft(x, y) { num_life += 1; }
				if top(x, y) { num_life += 1; }
				if tright(x, y) { num_life += 1; }
				if left(x, y) { num_life += 1; }
				if right(x, y) { num_life += 1; }
				if bleft(x, y) { num_life += 1; }
				if bottom(x, y) { num_life += 1; }
				if bright(x, y) { num_life += 1; }

			}
			coord.cell.living_neighbors = num_life;
		}
	}
}

impl<'a> event::EventHandler for World <'a>{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		let mut live_neighbors:u64;
		if Instant::now() - self.time_updated >= Duration::from_millis(MULLIS_UPDATE) {
			self.update_neighbors(self.cells);
			for coord in self.cells.iter_mut() {
				live_neighbors = coord.cell.living_neighbors;		

				//For all living cells		
				if coord.cell.life {
					//Dies
					if live_neighbors < 2 || live_neighbors > 3 {
						coord.cell.update(false);
					}
					//Sustains life
					else {
						coord.cell.unaltered();
					}
				}
				//For all dead cells
				else {
					//Life is born
					if live_neighbors == 3 {
						coord.cell.update(true);
					}
				}
			}
			self.time_updated = Instant::now();
		}
		
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		//Draw civs if they exist
			graphics::clear(ctx);

			//Draw each civ on the map
			for local in self.cells.iter() {//@todo: Maybe this is the cause of the slowdown. Only draw updated civs
				if local.cell.altered {//This solves the slowdown, maybe if civ.changed
					local.draw(ctx)?;
				}
			}

			graphics::present(ctx);
		
		Ok(())
	}
}

fn main() {
	let mut world_map: Vec<Coordinate> = build_world(MAX_WIDTH * 2);

	//Build game context
	//https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html
    let game = &mut ggez::ContextBuilder::new("Game of Life", "Matthew Greenlaw")
    	.window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
    	.window_mode(ggez::conf::WindowMode::default().dimensions(MAP.0 as u32, MAP.1 as u32))
    	.build().expect("Failed to build game.");

    //Build the world
    let life = &mut World::new(& mut world_map);//@todo: Need to be able to adjust this with cmd line input, etc

    //Run the main game loop
    //https://docs.rs/ggez/0.3.0/ggez/event/fn.run.html
    match event::run(game, life){
    	Ok(_) => println!("Exited..."),
    	Err(error) => println!("Error: {:?}", error),
    }
}

fn build_world<'a>(num_pop:i32) -> Vec<Coordinate>{
	let mut locals: Vec<Coordinate> = Vec::new();
	let mut range = rand::thread_rng();

	//Build coordinates on the map
	for x in 0..MAX_WIDTH {
		for y in 0..MAX_DEPTH { locals.push(Coordinate::new(x, y)); }
	}

	//Update random coordinates with life
	for _i in 0..num_pop {
		locals[range.gen_range::<usize>(0, (MAX_DEPTH * MAX_WIDTH) as usize)].cell.update(true);
	}

	locals
}