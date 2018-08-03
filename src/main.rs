extern crate rand;
extern crate ggez;
use rand::Rng;
use ggez::event::{self, MouseState, MouseButton};
use ggez::{Context, GameResult, graphics};
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
struct World { map: Vec<Vec<Option<Cell>>>, generation:i32, living: i32, dead: i32,}

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

		World { map: locals, generation:0, living: num_pop, dead: 0,}
	}

	pub fn clasic_generation(&mut self) {
		//Capture the state of this generation's map
		let generation: Vec<Vec<Option<Cell>>> = self.map.to_vec();
		let mut born = 0;
		let mut died = 0;

		//Evaluate each cell for living neighbors 
		for (x, row) in generation.iter().enumerate() {
			for (y, cell) in row.iter().enumerate() {
				let live_neighbors = World::num_neighbors(&generation, x as i32, y as i32);
				//Some = populated, check for population collaps
				//None = unpopulated, check for population growth
				match cell {
					Some(_) => { if live_neighbors < 2 || live_neighbors > 3 { self.map[x][y] = None; died +=1} },
					None => { if live_neighbors == 3 { self.map[x][y] = Some(Cell::new(x as i32,y as i32)); born += 1;} },
				}
			}
		}

		self.generation += 1;
		self.living += born - died;
		self.dead += died;
	}

	fn num_neighbors(map:&Vec<Vec<Option<Cell>>>, x:i32, y:i32) -> i32 {
		let neighbor = |x:i32, y:i32| {
			match map[x as usize][y as usize] {
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

	fn update(&mut self,) -> GameResult<(i32, i32, i32)> {
		self.clasic_generation();
		Ok((self.generation, self.living, self.dead))//Update for game over scenario?
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.draw_all_living(ctx)
	}
}

struct Frame {
	coord: Coord,
	height: i32,
	width: i32,
	header: String,
	text: String,
}

impl Frame {
	pub fn new(coord: Coord, height: i32, width: i32, header:String, text: &str, _ctx: &mut Context) -> Self {
		
		Frame {
			coord: coord,
			height: height,
			width: width,
			header: header,
			text: text.to_string(),
		}
	}
	
	//@todo
	fn update(&mut self, text:&str) -> GameResult<()> {
		self.text = text.to_string();
		Ok(())
	}

	fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		let mut text = self.header.to_string();
		text.push_str(self.text.as_str());
		let ttf = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("Missing ttf file");
		let text = &graphics::Text::new(ctx, text.as_str(), ttf).expect("Error generating text");
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 1.0].into())?;
		graphics::draw(ctx, text, graphics::Point2::new((self.coord.x * SIZE_GRID_PIXELS) as f32, (self.coord.y * SIZE_GRID_PIXELS) as f32), 0.0)?;	
		Ok(())
	}

	fn contains(&mut self, x:i32, y:i32) -> bool {
		if x > (self.coord.x * SIZE_GRID_PIXELS) && x < ((self.coord.x + self.width) * SIZE_GRID_PIXELS) {
			if y > (self.coord.y * SIZE_GRID_PIXELS) && y < ((self.coord.y + self.height) * SIZE_GRID_PIXELS) {
				return true;
			}
		}

		false
	}
}

struct UiElem<T> {
	coord: Coord,
	height: i32,
	width: i32,
	header:String,
	contents: Vec<T>,
}

impl<T> UiElem<T> {
	pub fn new(coord: Coord, height: i32, width: i32, header:String, contents: Vec<T>) -> Self {
		UiElem {
			coord: coord,
			height: height,
			width: width,
			header: header,
			contents: contents,
		}
	}
}

impl UiElem<Frame> {
	//@todo
	fn update(&mut self, text:Option<Vec<String>>) -> GameResult<()> {
		match text {
			Some(text) => {
				for (i, frame) in self.contents.iter_mut().enumerate() {
					frame.update(text[i].as_str())?;
				}	
			},
			None =>(),
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		// graphics::set_color(ctx, [0.1, 0.1, 0.1, 0.1].into())?; 
		// graphics::rectangle(ctx, graphics::DrawMode::Line(SIZE_GRID_PIXELS as f32), graphics::Rect::new_i32 (
		// self.coord.x, self.coord.y * SIZE_GRID_PIXELS, 0, (self.height-1)* SIZE_GRID_PIXELS))?;
		
		let ttf = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("Missing ttf file");
		let text = &graphics::Text::new(ctx, self.header.as_str(), ttf).expect("Error generating text");
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 1.0].into())?;
		graphics::draw(ctx, text, graphics::Point2::new((self.coord.x * SIZE_GRID_PIXELS) as f32, (self.coord.y * SIZE_GRID_PIXELS) as f32), 0.0)?;	

		for frame in self.contents.iter() {
			frame.draw(ctx)?;
		}	
		Ok(())
	}

	pub fn mouse_click(&mut self, x:i32, y:i32) {
		for frame in self.contents.iter_mut() {
			if frame.contains(x, y) {
				println!("Clicked {:?}", frame.text);
			}
		}
	}
}

struct Game {
	game: World,
	option: UiElem<Frame>,
	stat: UiElem<Frame>,
	player: UiElem<Frame>,
}

impl Game {
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
		let mut ofset = 0;
		let mut offset = || {ofset += 8; return ofset;};
		Game {
			game: World::new(2000),
			option: UiElem::new(
				//UiElems define where the frames live
				Coord::from(POS_OPTION_GRIDS), 
				HEIGHT_OPTION_GRIDS, 
				WIDTH_OPTION_GRIDS - 5, 
				"Options".to_string(),
				//Each frame defines where thier content lives witin the uielem
				vec![],
			),
			stat: UiElem::new(
				Coord::from(POS_STAT_GRIDS), 
				HEIGHT_STAT_GRIDS, 
				WIDTH_STAT_GRIDS  - 5,
				"Stats".to_string(),
				vec![
					Frame::new(
						Coord::from((POS_STAT_GRIDS.0 + 5, POS_STAT_GRIDS.1 + offset())),
						5, 
						WIDTH_STAT_GRIDS - 5,
						"Generation : ".to_string(),
						"",
						ctx,
					),
					Frame::new(
						Coord::from((POS_STAT_GRIDS.0 + 5, POS_STAT_GRIDS.1 + offset())),
						5, 
						WIDTH_STAT_GRIDS - 5,
						"Living         : ".to_string(),
						"",
						ctx,
					),
					Frame::new(
						Coord::from((POS_STAT_GRIDS.0 + 5, POS_STAT_GRIDS.1 + offset())),
						5, 
						WIDTH_STAT_GRIDS - 5,
						"Fatalities   : ".to_string(),
						"",
						ctx,
					),
				],
			),
			player: UiElem::new(
				Coord::from(POS_PLAYER_GRIDS), 
				HEIGHT_PLAYER_GRIDS, 
				WIDTH_PLAYER_GRIDS - 5,
				"Add life".to_string(), 
				vec![],
			),
		}
	}
}

impl event::EventHandler for Game{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		match self.game.update() {
			Ok(generation) => {
				//let header = self.stat.contents[0].text.to_string();
				self.stat.update(Some(vec![
					generation.0.to_string(),
                    generation.1.to_string(),
                    generation.2.to_string()
                ]))?;
			},
			_ => (),
		}
		self.option.update(None)?;
		
		self.player.update(None)?;
		Ok(())//Update for game-over scenario?
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


	fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32){
	//https://docs.rs/ggez/0.4.3/ggez/event/trait.EventHandler.html#method.mouse_button_up_event
		//println!("Clicked: {}, {}", x, y);
		self.option.mouse_click(x, y);
		self.stat.mouse_click(x, y);
		self.player.mouse_click(x, y);
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