extern crate ggez;
use ggez::{Context, GameResult, graphics};

use params::{
	SIZE_GRID_PIXELS
};

//Coordinates in the program window
pub struct Coord { x: i32, y: i32, }

impl Coord {
	pub fn new(x: i32, y: i32) -> Self {
		Coord {	x: x, y: y, }
	}

	pub fn from((x, y): (i32, i32)) -> Self {
		Coord {	x: x, y: y, }
	}

	pub fn get_coords(&self) -> (i32, i32) {
		(self.x, self.y)
	}

	//Tell ggez what color to render a Cell
	//https://docs.rs/ggez/0.3.1/ggez/graphics/index.html
	pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		let boarder = 1;
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 0.5].into())?; 
		graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new_i32 (//(x: f32, y: f32, w: f32, h: f32)
		self.x * SIZE_GRID_PIXELS, self.y * SIZE_GRID_PIXELS, SIZE_GRID_PIXELS-boarder, SIZE_GRID_PIXELS-boarder,))?;		
		Ok(())
	}
}

impl Clone for Coord {
	fn clone(&self) -> Coord { Coord::new(self.x, self.y) }
}