extern crate rand;
extern crate ggez;
use rand::Rng;
use ggez::{event, Context, GameResult, graphics};

//.ceil()
//Size of grids in pixels
const SIZE_GRID_PIXELS: i32 = 5;

//program window
const WIDTH_WINDOW_GRIDS:i32 = 150;
const HEIGHT_WINDOW_GRIDS:i32 = 150;
const AREA_WINDOW_PIXELS: (i32, i32) = (WIDTH_WINDOW_GRIDS * SIZE_GRID_PIXELS, HEIGHT_WINDOW_GRIDS * SIZE_GRID_PIXELS);
//------------------------|-----|
//|game (0,0)             |stats|
//|                       |     |
//|                       |     |
//|-----------------------|-----|
//|Player                 |opts.|
//|-----------------------|-----|
//game_frame
const WIDTH_GAME_GRIDS:i32 = WIDTH_WINDOW_GRIDS - 50;
const HEIGHT_GAME_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 50;
const AREA_GAME_PIXELS: (i32, i32) = (WIDTH_GAME_GRIDS * SIZE_GRID_PIXELS, HEIGHT_GAME_GRIDS * SIZE_GRID_PIXELS);

//stat_frame
const WIDTH_STAT_GRIDS:i32 = WIDTH_WINDOW_GRIDS - WIDTH_GAME_GRIDS;
const HEIGHT_STAT_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 50;
const AREA_STAT_PIXELS: (i32, i32) = (WIDTH_STAT_GRIDS * SIZE_GRID_PIXELS, HEIGHT_STAT_GRIDS * SIZE_GRID_PIXELS);

//player_frame
const WIDTH_PLAYER_GRIDS:i32 = WIDTH_GAME_GRIDS;
const HEIGHT_PLAYER_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - HEIGHT_GAME_GRIDS;
const AREA_PLAYER_PIXELS: (i32, i32) = (WIDTH_PLAYER_GRIDS * SIZE_GRID_PIXELS, HEIGHT_PLAYER_GRIDS * SIZE_GRID_PIXELS);

//option_frame
const WIDTH_OPTION_GRIDS:i32 = WIDTH_WINDOW_GRIDS-WIDTH_PLAYER_GRIDS;
const HEIGHT_OPTION_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - HEIGHT_STAT_GRIDS;
const AREA_OPTION_PIXELS: (i32, i32) = (WIDTH_OPTION_GRIDS * SIZE_GRID_PIXELS, HEIGHT_OPTION_GRIDS * SIZE_GRID_PIXELS);

//Coordinates in the program window
struct Coord { x: i32, y: i32, }

impl Coord {
	pub fn new(x: i32, y: i32) -> Self {
		Coord {	x: x, y: y, }
	}

	pub fn from((x, y): (i32, i32)) -> Self {
		Coord {	x: x, y: y, }
	}
}

impl Clone for Coord {
	fn clone(&self) -> Coord { Coord::new(self.x, self.y) }
}

//Set default coordinates of window elements
const POS_GAME_GRIDS: (i32, i32) = (0, 0);
const POS_STAT_GRIDS: (i32, i32) = (WIDTH_GAME_GRIDS + 1, HEIGHT_GAME_GRIDS);
const POS_PLAYER_GRIDS: (i32, i32) = (0, (HEIGHT_WINDOW_GRIDS - HEIGHT_GAME_GRIDS) + 1);
const POS_OPTION_GRIDS: (i32, i32) = (WIDTH_GAME_GRIDS + 1, HEIGHT_STAT_GRIDS + 1);



//Coordinates for ggez to draw Coords
struct Cell { coord: Coord, }

impl Cell {
	pub fn new(x: i32, y: i32) -> Self {
		Cell {	coord:Coord::new(x, y), }
	}

	//Tell ggez what color to render a Cell
	//https://docs.rs/ggez/0.3.1/ggez/graphics/index.html
	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		let boarder = 1;
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 0.5].into())?; 
		graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new_i32 (
		self.coord.x * SIZE_GRID_PIXELS, self.coord.y * SIZE_GRID_PIXELS, SIZE_GRID_PIXELS-boarder, SIZE_GRID_PIXELS-boarder,))?;		
		Ok(())
	}
}

impl Clone for Cell {
	fn clone(&self) -> Cell { Cell {coord: Coord::new(self.coord.x, self.coord.y,) }}
}

//Define properties of the game
struct World { map: Vec<Vec<Option<Cell>>>, }

impl World {
	//Generate a new world given its population
	pub fn new (num_pop: i32) -> Self {
		let mut range = rand::thread_rng();
		let mut locals: Vec<Vec<Option<Cell>>> = vec![vec![None; WIDTH_GAME_GRIDS as usize]; HEIGHT_GAME_GRIDS as usize];	

		for _i in 0..num_pop {
			let x = range.gen_range::<i32>(0, WIDTH_GAME_GRIDS as i32);
			let y = range.gen_range::<i32>(0, HEIGHT_GAME_GRIDS as i32);
			locals[x as usize][y as usize] = Some(Cell::new(x, y));
		}

		World { map: locals, }
	}

	pub fn clasic_generation(&mut self) {
		//Capture the state of this generation's map
		let generation: Vec<Vec<Option<Cell>>> = self.map.to_vec();

		//Evaluate each cell for living neighbors 
		for (x, row) in generation.iter().enumerate() {
			for (y, cell) in row.iter().enumerate() {
				let live_neighbors = World::num_neighbors(&generation, x as i32, y as i32);
				//Some = populated, check for population collaps
				//None = unpopulated, check for population growth
				match cell {
					Some(_) => { if live_neighbors < 2 || live_neighbors > 3 { self.map[x][y] = None; } },
					None => { if live_neighbors == 3 { self.map[x][y] = Some(Cell::new(x as i32,y as i32)); } },
				}
			}
		}
	}

	fn num_neighbors(generation:&Vec<Vec<Option<Cell>>>, x:i32, y:i32) -> i32 {
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
			for col in row.iter() {
				match col {
					Some(cell) => cell.draw(ctx)?,
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
		//graphics::clear(ctx);
		self.draw_all_living(ctx)?;
		//graphics::present(ctx);
		Ok(())
	}
}

struct Frame {
	pos: Coord,
	height: i32,
	width: i32,
	text: String,
}

impl Frame {
	pub fn new(coord: Coord, height: i32, width: i32, text: &str) -> Self {
		Frame {
			pos: coord,
			height: height,
			width: width,
			text: text.to_string(),
		}
	}
}

struct UI_Elem {
	pos: Coord,
	height: i32,
	width: i32,
	inner: Vec<Frame>,
}

impl UI_Elem {
	pub fn new(coord: Coord, height: i32, width: i32, inner: Vec<Frame>) -> Self {
		UI_Elem {
			pos: coord,
			height: height,
			width: width,
			inner: inner,
		}
	}
}

struct Game_Elem {
	pos: Coord,
	height: i32,
	width: i32,
	instance: World,

}

impl Game_Elem {
	pub fn new(num_pop: i32) -> Self {
		Game_Elem {
			pos: Coord::from(POS_GAME_GRIDS),
			height: HEIGHT_GAME_GRIDS,
			width: WIDTH_GAME_GRIDS,
			instance: World::new(num_pop),
		}
	}
}

struct Game {
	game: Game_Elem,
	option: UI_Elem,
	stat: UI_Elem,
	player: UI_Elem,
}

impl Game {
	pub fn new(num_pop: i32) -> Self {
		Game {
			game: Game_Elem::new(num_pop),
			option: UI_Elem::new(
				Coord::from(POS_OPTION_GRIDS), 
				HEIGHT_OPTION_GRIDS, 
				WIDTH_OPTION_GRIDS, 
				vec![Frame::new(
					Coord::from(POS_OPTION_GRIDS),
					HEIGHT_OPTION_GRIDS, 
					WIDTH_OPTION_GRIDS,
					"Options:",
				)],
			),//(coord: Coord, height: i32, width: i32, inner: Vec<Frame>)
			stat: UI_Elem::new(
				Coord::from(POS_STAT_GRIDS), 
				HEIGHT_STAT_GRIDS, 
				WIDTH_STAT_GRIDS, 
				vec![Frame::new(
					Coord::from(POS_STAT_GRIDS),
					HEIGHT_STAT_GRIDS, 
					WIDTH_STAT_GRIDS,
					"Stats:",
				)],
			),
			player: UI_Elem::new(
				Coord::from(POS_PLAYER_GRIDS), 
				HEIGHT_PLAYER_GRIDS, 
				WIDTH_PLAYER_GRIDS, 
				vec![Frame::new(
					Coord::from(POS_PLAYER_GRIDS),
					HEIGHT_PLAYER_GRIDS, 
					WIDTH_PLAYER_GRIDS,
					"Player:",
				)],
			),
		}
	}

	pub fn basic() -> Self {
		Game {
			game: Game_Elem::new(2500),
			option: UI_Elem::new(Coord::from(POS_OPTION_GRIDS), HEIGHT_OPTION_GRIDS, WIDTH_OPTION_GRIDS, vec![]),//(coord: Coord, height: i32, width: i32, inner: Vec<Frame>)
			stat: UI_Elem::new(Coord::from(POS_STAT_GRIDS), HEIGHT_STAT_GRIDS, WIDTH_STAT_GRIDS, vec![]),
			player: UI_Elem::new(Coord::from(POS_PLAYER_GRIDS), HEIGHT_PLAYER_GRIDS, WIDTH_PLAYER_GRIDS, vec![]),
		}
	}
}

impl event::EventHandler for Game{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.game.instance.update(ctx)?;
		Ok(())//Update for game over scenario?
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		self.game.instance.draw(ctx)?;
		graphics::present(ctx);
		Ok(())
	}
}

fn main() {
	//get user input/args


	//Build game context
    let program = &mut ggez::ContextBuilder::new("Game of Life", "Matthew Greenlaw")//https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html
    	.window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
    	.window_mode(ggez::conf::WindowMode::default().dimensions(AREA_WINDOW_PIXELS.0 as u32, AREA_WINDOW_PIXELS.1 as u32))
    	.build().expect("Failed to build game.");

    graphics::set_background_color(program, [1.0, 1.0, 1.0, 1.0].into());

    //Build the world
    //let life = &mut World::new(2500);//@todo: Need to be able to adjust this with cmd line input, etc

    let game = &mut Game::basic(); 

    //Run the main game loop
    match event::run(program, game){//https://docs.rs/ggez/0.3.0/ggez/event/fn.run.html
    	Ok(_) => println!("Exited..."),
    	Err(error) => println!("Error: {:?}", error),
    }
}