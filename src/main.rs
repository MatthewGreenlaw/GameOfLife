extern crate rand;
use rand::Rng;

extern crate ggez;
use ggez::{event, Context, GameResult, graphics};

//Number of pixels per coordinate
//const NUM_PIXEL:u32 = 10;

//Maximum width and depth of the world
const MAX_WIDTH:i32 = 100;
const MAX_DEPTH:i32 = 100;

//Define Map of coordinates to display
const PIXELS: (i32, i32) = (10, 10); //Pixels in a coordinate
//const COORDS: (u32, u32) = (100, 100); //Coordinates in a map

//@todo: Figure out why there is an infinite loop if I don't hard code these
const MAP: (i32, i32) = (1000, 1000); //Pixels in the map

//Define a coordinate on the map
#[derive(Clone, Copy)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	pub fn new(x:i32, y:i32) -> Coordinate {
		Coordinate{
			x: x,
			y: y,
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
struct Location {
	coord: Coordinate,
	life: bool,
	neighbors: Vec<Coordinate>,
}

impl Location {
	pub fn new(x:i32, y:i32) -> Location {
		Location {
			coord: Coordinate::new(x, y),
			life: true,
			neighbors: set_neighbors(Coordinate::new(x, y)),
		}
	}

	fn update(&mut self){
		if self.neighbors.is_empty() {
			self.life = false;
		}
		else {
			//Figure out how to kill and breed
		}
	}

	//Tell ggez how what color to render a location
	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		if self.life { graphics::set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?; }
		else { graphics::set_color(ctx, [0.0, 1.0, 0.0, 0.0].into())?; }

		graphics::rectangle(ctx, graphics::DrawMode::Fill, self.coord.into())?;
		Ok(())
	}
}

fn set_neighbors(coord: Coordinate) -> Vec<Coordinate> {
	// (0, 0) (n, 0)
	// (0, n) (n, n)
	let x = coord.x;
	let y = coord.y;
	let tleft =  Coordinate::new(x - 1, y - 1);
	let top =    Coordinate::new(x    , y - 1);
	let tright = Coordinate::new(x + 1, y - 1);
	let left =   Coordinate::new(x - 1, y    );
	let right =  Coordinate::new(x + 1, y    );
	let bleft =  Coordinate::new(x - 1, y + 1);
	let bottom = Coordinate::new(x    , y + 1);
	let bright = Coordinate::new(x + 1, y + 1);
	//Check Corners
	if x == 0 || y == 0 {
		// c n
		// n n
		if x == 0 && y == 0 {
			vec![right, bottom, bright]
		}
		// n c
		// n n
		else if x == MAX_WIDTH && y == 0 {
			vec![left, bleft, bottom]
		}
		// n n
		// c n
		else if x == 0 && y == MAX_DEPTH {
			vec![top, tright, right]
		}
		// n n
		// n c
		else {
			vec![tleft, top, left]
		}
	}
	// n n n
	// n c n
	// n n n
	else {
		vec![tleft, top, tright, left, right, bleft, bottom, bright]
	}
}

//Define properties of the game
struct World {
	population: i32,
	civilizations: Vec<Location>,
}

impl World {
	//Generate a new world given its population
	pub fn new (num_pop: i32) -> World {
		let mut civs: Vec<Location> = Vec::new();
		let mut range = rand::thread_rng();

		for _i in 0..num_pop {
			civs.push(Location::new(
				range.gen_range::<i32>(0, MAX_WIDTH), 
				range.gen_range::<i32>(0, MAX_WIDTH)
			));
		}

		World {
			population: num_pop,
			civilizations: civs,
		}
	}
}


impl event::EventHandler for World{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		if self.population > 0 {
			//Update each civilization
			// for civ in self.civilizations.iter() {
			// 	civ.update();
			// }
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		//Draw civs if they exist
		if self.population > 0 {
			graphics::clear(ctx);

			//Draw each civ on the map
			for civ in self.civilizations.iter() {
				civ.draw(ctx)?;
			}

			graphics::present(ctx);
		}
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
