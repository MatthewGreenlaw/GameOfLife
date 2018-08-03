extern crate rand;
extern crate ggez;
use rand::Rng;
use ggez::{event, Context, GameResult, graphics};
use std::env;
use std::path;

//.ceil()
//Size of grids in pixels
const SIZE_GRID_PIXELS: i32 = 5;

//program window
const WIDTH_WINDOW_GRIDS:i32 = 150;
const HEIGHT_WINDOW_GRIDS:i32 = 150;
const AREA_WINDOW_PIXELS: (i32, i32) = (WIDTH_WINDOW_GRIDS * SIZE_GRID_PIXELS, HEIGHT_WINDOW_GRIDS * SIZE_GRID_PIXELS);
//|-----------------------|-----|
//|game                   |stats|
//|                       |     |
//|                       |     |
//|-----------------------|-----|
//|Player                 |opts.|
//|-----------------------|-----|
//game_frame
const WIDTH_GAME_GRIDS:i32 = WIDTH_WINDOW_GRIDS - 50;
const HEIGHT_GAME_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 50;
//const AREA_GAME_PIXELS: (i32, i32) = (WIDTH_GAME_GRIDS * SIZE_GRID_PIXELS, HEIGHT_GAME_GRIDS * SIZE_GRID_PIXELS);

//stat_frame
const WIDTH_STAT_GRIDS:i32 = WIDTH_WINDOW_GRIDS - WIDTH_GAME_GRIDS;
const HEIGHT_STAT_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 50;
//const AREA_STAT_PIXELS: (i32, i32) = (WIDTH_STAT_GRIDS * SIZE_GRID_PIXELS, HEIGHT_STAT_GRIDS * SIZE_GRID_PIXELS);

//player_frame
const WIDTH_PLAYER_GRIDS:i32 = WIDTH_GAME_GRIDS;
const HEIGHT_PLAYER_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - HEIGHT_GAME_GRIDS;
//const AREA_PLAYER_PIXELS: (i32, i32) = (WIDTH_PLAYER_GRIDS * SIZE_GRID_PIXELS, HEIGHT_PLAYER_GRIDS * SIZE_GRID_PIXELS);

//option_frame
const WIDTH_OPTION_GRIDS:i32 = WIDTH_WINDOW_GRIDS-WIDTH_PLAYER_GRIDS;
const HEIGHT_OPTION_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - HEIGHT_STAT_GRIDS;
//const AREA_OPTION_PIXELS: (i32, i32) = (WIDTH_OPTION_GRIDS * SIZE_GRID_PIXELS, HEIGHT_OPTION_GRIDS * SIZE_GRID_PIXELS);

//Set default coordinates of window elements
//Position is the center of the element
const POS_GAME_GRIDS: (i32, i32) = (0,0);
const POS_STAT_GRIDS: (i32, i32) = (POS_GAME_GRIDS.0 + (WIDTH_GAME_GRIDS), POS_GAME_GRIDS.1);
const POS_PLAYER_GRIDS: (i32, i32) = (POS_GAME_GRIDS.0, POS_GAME_GRIDS.1 + (HEIGHT_GAME_GRIDS));
const POS_OPTION_GRIDS: (i32, i32) = (POS_STAT_GRIDS.0, POS_PLAYER_GRIDS.1);

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
		graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new_i32 (//(x: f32, y: f32, w: f32, h: f32)
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

	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		self.clasic_generation();
		Ok(())//Update for game over scenario?
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.draw_all_living(ctx)
	}
}

struct Frame {
	text: graphics::Text,
}

impl Frame {
	pub fn new(text: &str, ctx: &mut Context) -> Self {
		let input = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("missing asset");
		Frame {

			text: graphics::Text::new(ctx, text, input).expect("missing asset"),
		}
	}
	
	//@todo
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 1.0].into())?;
		graphics::draw(ctx, &self.text, graphics::Point2::new((self.coord.x * SIZE_GRID_PIXELS) as f32, (self.coord.y * SIZE_GRID_PIXELS) as f32), 0.0)?;	
		Ok(())
	}
}

struct Element<'a, T> {
	ctx: &'a Context,
	coord: Coord,
	height: i32,
	width: i32,
	elem: T,
}

impl<'a, T> Element<'a, T> {
	pub fn new(coord: Coord, height: i32, width: i32, elem: T, ctx: &'a mut Context)-> Self{
		Element {
			ctx: ctx,
			coord: coord,
			height: height,
			width: width,
			elem: elem,
		}
	}
}

struct UiElem<'a> {
	inner: Vec<Element<'a, Frame>>,
}

impl<'a> UiElem<'a> {
	pub fn new(inner: Vec<Element<'a, Frame>>) -> Self {
		UiElem {
			inner: inner,
		}
	}
	
	//@todo
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		// graphics::set_color(ctx, [0.5, 0.5, 0.5, 0.5].into())?; 
		// graphics::rectangle(ctx, graphics::DrawMode::Line(SIZE_GRID_PIXELS as f32), graphics::Rect::new_i32 (
		// self.coord.x * SIZE_GRID_PIXELS, self.coord.y * SIZE_GRID_PIXELS, self.width* SIZE_GRID_PIXELS, self.height* SIZE_GRID_PIXELS))?;
		
		for frame in self.inner.iter() {
			//frame.draw(ctx)?;
		}	
		Ok(())
	}
}

struct Game<'a, A, B> {
	game: Element<'a, A>,
	option: Element<'a, B>,
	stat: Element<'a, B>,
	player: Element<'a, B>,
}

impl<'a> Game<'a, World, UiElem<'a>> {
	//@todo
	// pub fn new(num_pop: i32) -> Self {
	// 	Game {
	// 		game: GameElem::new(num_pop),
	// 		option: UiElem::new(
	// 			Coord::from(POS_OPTION_GRIDS), 
	// 			HEIGHT_OPTION_GRIDS, 
	// 			WIDTH_OPTION_GRIDS, 
	// 			vec![Frame::new(
	// 				Coord::from(POS_OPTION_GRIDS),
	// 				HEIGHT_OPTION_GRIDS, 
	// 				WIDTH_OPTION_GRIDS,
	// 				"Options:",
	// 			)],
	// 		),//(coord: Coord, height: i32, width: i32, inner: Vec<Frame>)
	// 		stat: UiElem::new(
	// 			Coord::from(POS_STAT_GRIDS), 
	// 			HEIGHT_STAT_GRIDS, 
	// 			WIDTH_STAT_GRIDS, 
	// 			vec![Frame::new(
	// 				Coord::from(POS_STAT_GRIDS),
	// 				HEIGHT_STAT_GRIDS, 
	// 				WIDTH_STAT_GRIDS,
	// 				"Stats:",
	// 			)],
	// 		),
	// 		player: UiElem::new(
	// 			Coord::from(POS_PLAYER_GRIDS), 
	// 			HEIGHT_PLAYER_GRIDS, 
	// 			WIDTH_PLAYER_GRIDS, 
	// 			vec![Frame::new(
	// 				Coord::from(POS_PLAYER_GRIDS),
	// 				HEIGHT_PLAYER_GRIDS, 
	// 				WIDTH_PLAYER_GRIDS,
	// 				"Player:",
	// 			)],
	// 		),
	// 	}
	// }

	pub fn classic(ctx: &mut Context) -> Self {
		Game {
			game: Element::new(
				Coord::from(POS_GAME_GRIDS), 
				HEIGHT_GAME_GRIDS,
				WIDTH_GAME_GRIDS, 
				World::new(2000), 
				ctx),
			option: Element::new(
				Coord::from(POS_OPTION_GRIDS), 
				HEIGHT_OPTION_GRIDS, 
				WIDTH_OPTION_GRIDS,
				UiElem::new(
					vec![
						Element::new(
							Frame::new("Options:", ctx),
							ctx,
						),
						Element::new(
							Coord::from((POS_OPTION_GRIDS.0+1, POS_OPTION_GRIDS.1+1+5)),
							HEIGHT_OPTION_GRIDS/4, 
							WIDTH_OPTION_GRIDS,
							"Restart",
							ctx,
						),
						Element::new(
							Coord::from((POS_OPTION_GRIDS.0+1, POS_OPTION_GRIDS.1+1+5+5)),
							HEIGHT_OPTION_GRIDS/4, 
							WIDTH_OPTION_GRIDS,
							"Pause",
							ctx,
						),
						Element::new(
							Coord::from((POS_OPTION_GRIDS.0+1, POS_OPTION_GRIDS.1+1+5+5+5)),
							HEIGHT_OPTION_GRIDS/4, 
							WIDTH_OPTION_GRIDS,
							"Exit",
							ctx,
						),
					],
				),
			),
			stat: UiElem::new(
				Coord::from(POS_STAT_GRIDS), 
				HEIGHT_STAT_GRIDS, 
				WIDTH_STAT_GRIDS, 
				vec![
					Element::new(
						Coord::from((POS_STAT_GRIDS.0+1, POS_STAT_GRIDS.1+1)),
						HEIGHT_STAT_GRIDS/4, 
						WIDTH_STAT_GRIDS,
						"Stats",
						ctx,
					),
					Element::new(
						Coord::from((POS_STAT_GRIDS.0+1, POS_STAT_GRIDS.1+1+5)),
						HEIGHT_STAT_GRIDS/4, 
						WIDTH_STAT_GRIDS,
						"Generations",
						ctx,
					),
					Element::new(
						Coord::from((POS_STAT_GRIDS.0+1, POS_STAT_GRIDS.1+1+5+5)),
						HEIGHT_STAT_GRIDS/4, 
						WIDTH_STAT_GRIDS,
						"Living",
						ctx,
					),
					Element::new(
						Coord::from((POS_STAT_GRIDS.0+1, POS_STAT_GRIDS.1+1+5+5+5)),
						HEIGHT_STAT_GRIDS/4, 
						WIDTH_STAT_GRIDS,
						"Fatalities",
						ctx,
					),
				],
			),
			player: UiElem::new(
				Coord::from(POS_PLAYER_GRIDS), 
				HEIGHT_PLAYER_GRIDS, 
				WIDTH_PLAYER_GRIDS, 
				vec![Frame::new(
					Coord::from((POS_PLAYER_GRIDS.0+1, POS_PLAYER_GRIDS.1+1)),
					HEIGHT_PLAYER_GRIDS, 
					WIDTH_PLAYER_GRIDS,
					"Add life:",
					ctx,
				)],
			),
		}
	}
}

impl<'a, A, B> event::EventHandler for Game<'a, A, B>{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.game.update(ctx)?;
		self.option.update(ctx)?;
		self.stat.update(ctx)?;
		self.player.update(ctx)?;
		Ok(())//Update for game over scenario?
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		self.game.draw(ctx)?;
		self.option.draw(ctx)?;
		self.stat.draw(ctx)?;
		self.player.draw(ctx)?;
		graphics::present(ctx);
		Ok(())
	}
}

fn main() {
	//@todo
	//get user input/args

	//Check cargo manifest directory for external .ttf files
    if let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path_buffer = path::PathBuf::from(cargo_manifest_dir);
        path_buffer.push("ttf");

        //Build program
        let program = &mut ggez::ContextBuilder::new("Game of Life", "Matthew Greenlaw")
        //https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html
    	.window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
    	.window_mode(ggez::conf::WindowMode::default().dimensions(AREA_WINDOW_PIXELS.0 as u32, AREA_WINDOW_PIXELS.1 as u32))
    	.add_resource_path(path_buffer)
    	.build().expect("Failed to build game.");

    	graphics::set_background_color(program, [1.0, 1.0, 1.0, 1.0].into());

    	//Build the game
	    let game = &mut Game::classic(program); 

	    //Run the main game loop
	    match event::run(program, game){
	    //https://docs.rs/ggez/0.3.0/ggez/event/fn.run.html
	    	Ok(_) => println!("Exited..."),
	    	Err(error) => println!("Error: {:?}", error),
	    }
    }
    else {
        panic!("Usage: cargo run");
    }
}