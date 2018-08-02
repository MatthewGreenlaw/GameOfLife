extern crate rand;
extern crate ggez;
use rand::Rng;
use ggez::{event, Context, GameResult, graphics};

//Size of grids in pixels
const SIZE_GRID_PIXELS: i32 = 5;

//Width and height of map in grids
const WIDTH_MAP_GRIDS:i32 = 150;
const HEIGHT_MAP_GRIDS:i32 = 150;

//Define Map of coordinates to display
const AREA_MAP_PIXELS: (i32, i32) = (WIDTH_MAP_GRIDS * SIZE_GRID_PIXELS, HEIGHT_MAP_GRIDS * SIZE_GRID_PIXELS);

//Coordinates for ggez to draw cells
struct Coord { x: i32, y: i32, }

impl Coord {
	pub fn new(x: i32, y: i32) -> Self {
		Coord {	x: x, y: y, }
	}

	//Tell ggez what color to render a Coord
	//https://docs.rs/ggez/0.3.1/ggez/graphics/index.html
	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		let boarder = 1;
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 0.5].into())?; 
		graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new_i32 (
		self.x * SIZE_GRID_PIXELS, self.y * SIZE_GRID_PIXELS, SIZE_GRID_PIXELS-boarder, SIZE_GRID_PIXELS-boarder,))?;		
		Ok(())
	}
}

impl Clone for Coord {
	fn clone(&self) -> Coord { Coord::new(self.x, self.y) }
}

//Define properties of the game
struct World { map: Vec<Vec<Option<Coord>>>, }

impl World {
	//Generate a new world given its population
	pub fn new (num_pop: i32) -> Self {
		let mut range = rand::thread_rng();
		let mut locals: Vec<Vec<Option<Coord>>> = vec![vec![None; WIDTH_MAP_GRIDS as usize]; HEIGHT_MAP_GRIDS as usize];	

		for _i in 0..num_pop {
			let x = range.gen_range::<i32>(0, WIDTH_MAP_GRIDS as i32);
			let y = range.gen_range::<i32>(0, HEIGHT_MAP_GRIDS as i32);
			locals[x as usize][y as usize] = Some(Coord::new(x, y));
		}

		World { map: locals, }
	}

	pub fn clasic_generation(&mut self) {
		//Capture the state of this generation's map
		let generation: Vec<Vec<Option<Coord>>> = self.map.to_vec();

		//Evaluate each cell for living neighbors 
		for (x, row) in generation.iter().enumerate() {
			for (y, coord) in row.iter().enumerate() {
				let live_neighbors = World::num_neighbors(&generation, x as i32, y as i32);
				//Some = populated, check for population collaps
				//None = unpopulated, check for population growth
				match coord {
					Some(_) => { if live_neighbors < 2 || live_neighbors > 3 { self.map[x][y] = None; } },
					None => { if live_neighbors == 3 { self.map[x][y] = Some(Coord::new(x as i32,y as i32)); } },
				}
			}
		}
	}

	fn num_neighbors(generation:&Vec<Vec<Option<Coord>>>, x:i32, y:i32) -> i32 {
		let neighbor = |x:i32, y:i32| {
			match generation[x as usize][y as usize] {
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
		else if x == WIDTH_MAP_GRIDS-1 && y == 0 {
			left(x, y)+bleft(x, y)+bottom(x, y)
		}
		// n n
		// c n
		else if x == 0 && y == HEIGHT_MAP_GRIDS-1 {
			top(x, y)+tright(x, y)+right(x, y)
		}
		// n n
		// n c
		else if x == WIDTH_MAP_GRIDS-1 && y == HEIGHT_MAP_GRIDS-1 {
			tleft(x, y)+top(x, y)+left(x, y)
		}
		// n n
		// c n
		// n n
		else if x == 0 && y > 0 && y < HEIGHT_MAP_GRIDS-1 {
			top(x, y)+tright(x, y)+right(x, y)+bottom(x, y)+bright(x, y)
		}
		// n c n
		// n n n
		else if y == 0 && x > 0 && x < WIDTH_MAP_GRIDS-1 {
			left(x, y)+right(x, y)+bleft(x, y)+bottom(x, y)+bright(x, y)
		}
		// n n
		// n c
		// n n
		else if x == WIDTH_MAP_GRIDS-1 && y > 0 && y < HEIGHT_MAP_GRIDS-1 {
			tleft(x, y)+top(x, y)+left(x, y)+bleft(x, y)+bottom(x, y)
		}
		// n n n
		// n c n
		else if y == HEIGHT_MAP_GRIDS-1 && x > 0 && x < WIDTH_MAP_GRIDS-1 {
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
			for col in row.iter() {
				match col {
					Some(coord) => coord.draw(ctx)?,
					None => (),
				}
			}
		}
		Ok(())
	}
}

impl event::EventHandler for World{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		self.clasic_generation();
		Ok(())//Update for game over scenario?
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		self.draw_all_living(ctx)?;
		graphics::present(ctx);
		Ok(())
	}
}

fn main() {
	//Build game context
    let game = &mut ggez::ContextBuilder::new("Game of Life", "Matthew Greenlaw")//https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html
    	.window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
    	.window_mode(ggez::conf::WindowMode::default().dimensions(AREA_MAP_PIXELS.0 as u32, AREA_MAP_PIXELS.1 as u32))
    	.build().expect("Failed to build game.");
    graphics::set_background_color(game, [1.0, 1.0, 1.0, 1.0].into());
    //Build the world
    let life = &mut World::new(2000);//@todo: Need to be able to adjust this with cmd line input, etc

    //Run the main game loop
    match event::run(game, life){//https://docs.rs/ggez/0.3.0/ggez/event/fn.run.html
    	Ok(_) => println!("Exited..."),
    	Err(error) => println!("Error: {:?}", error),
    }
}