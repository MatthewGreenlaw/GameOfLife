extern crate rand;
extern crate ggez;
use rand::Rng;
use std::cmp::Ordering;
use ggez::{event, Context, GameResult, graphics};

const PIXELS: usize = 10; //Pixels in a coordinate

//Maximum width and depth of the world
const MAX_WIDTH:usize = 50;
const MAX_HEIGHT:usize = 50;

//Define Map of coordinates to display
const MAP: (usize, usize) = (MAX_WIDTH * PIXELS, MAX_HEIGHT * PIXELS); //Pixels in the map

struct Coord {
	x: i32,
	y: i32,
}

impl Coord {
	pub fn new(x: i32, y: i32) -> Coord {
		Coord {
			x: x,
			y: y,
		}
	}
}

impl Eq for Coord {}

impl PartialEq for Coord {
	fn eq(&self, other: &Coord) -> bool {
		self.x == other.x &&
		self.y == other.y
	}
}

impl Ord for Coord {
	fn cmp(&self, other: &Coord) -> Ordering {
		let y_ordering = self.y.cmp(&other.y);

		match self.x.cmp(&other.x) {
			Ordering::Less => Ordering::Less,
			Ordering::Equal => y_ordering,
			Ordering::Greater => Ordering::Greater,
		}
	}
}

impl PartialOrd for Coord {
	fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {

		let y_ordering = self.y.partial_cmp(&other.y);

		match self.x.partial_cmp(&other.x) {
			Some(Ordering::Less) => Some(Ordering::Less),
			Some(Ordering::Equal) => y_ordering,
			Some(Ordering::Greater) => Some(Ordering::Greater),
			_ => None,
		}
	}
}

struct Cell {
	coord: Coord,
	life: bool,
}


impl Cell {
	pub fn new(x: i32, y: i32) -> Cell {
		Cell {
			coord: Coord::new(x, y),
			life: true,
		}
	}

	// pub fn update(&mut self, health: bool) {
	// 	self.life = health;
	// }

	//Tell ggez what color to render a location
	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		if self.life { 
			graphics::set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?; 
			graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new_i32 (
			self.coord.x as i32 * PIXELS as i32, 
			self.coord.y as i32 * PIXELS as i32,
			PIXELS as i32,
			PIXELS as i32,))?;
		}
		
		Ok(())
	}
}

impl Clone for Cell {
	fn clone(&self) -> Cell {
		Cell {
			coord: Coord::new(self.coord.x, self.coord.y),
			life: self.life,
		}
	}
}

impl Eq for Cell {}

impl PartialEq for Cell {
	fn eq(&self, other: &Cell) -> bool {
		self.coord == other.coord
	}
}

impl Ord for Cell {
	fn cmp(&self, other: &Cell) -> Ordering {
		self.coord.cmp(&other.coord)
	}
}

impl PartialOrd for Cell {
	fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
		 self.coord.partial_cmp(&other.coord)
	}
}

//Define properties of the game
struct World {
	locals: Vec<Vec<Option<Cell>>>,
}

impl World {
	//Generate a new world given its population
	pub fn new (num_pop: i32) -> World {
		let mut range = rand::thread_rng();
		let mut locals: Vec<Vec<Option<Cell>>> = vec![vec![None; MAX_WIDTH]; MAX_HEIGHT];	

		for _i in 0..num_pop {
			let x = range.gen_range::<i32>(0, MAX_WIDTH as i32);
			let y = range.gen_range::<i32>(0, MAX_HEIGHT as i32);
			let local = Cell::new(x, y);
			locals[x as usize][y as usize] = Some(local);
		}

		World {
			locals: locals,
		}
	}
}

impl event::EventHandler for World{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		for row in self.locals.iter() {//@todo: Maybe this is the cause of the slowdown. Only draw updated civs
			for col in row.iter() {
				match col {
					Some(local) => local.draw(ctx)?,
					None => (),
				}
			}
		}
		graphics::present(ctx);
		Ok(())
	}
}

fn main() {
	//Build game context
	//https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html
    let game = &mut ggez::ContextBuilder::new("Game of Life", "Matthew Greenlaw")
    	.window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
    	.window_mode(ggez::conf::WindowMode::default().dimensions(MAP.0 as u32, MAP.1 as u32))
    	.build().expect("Failed to build game.");

    //Build the world
    let life = &mut World::new(5);//@todo: Need to be able to adjust this with cmd line input, etc

    //Run the main game loop
    //https://docs.rs/ggez/0.3.0/ggez/event/fn.run.html
    match event::run(game, life){
    	Ok(_) => println!("Exited..."),
    	Err(error) => println!("Error: {:?}", error),
    }
}