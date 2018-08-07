extern crate ggez;
use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Point2};


use coord::{Coord};
use params::{ SIZE_GRID_PIXELS };

pub struct Frame {
	coord: Coord,
	height: i32,
	width: i32,
	header: String,
	text: String,
}

impl Frame {
	pub fn new(coord: (i32, i32), height: i32, width: i32, indent: i32, offset: i32, header:String, text: &str, _ctx: &mut Context) -> Self {
		
		Frame {
			coord: Coord::from((coord.0 + indent, coord.1 + offset)),
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
		let coords = self.coord.get_coords();
		let mut text = self.header.to_string();
		text.push_str(self.text.as_str());
		let ttf = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("Missing ttf file");
		let text = &graphics::Text::new(ctx, text.as_str(), ttf).expect("Error generating text");
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 1.0].into())?;
		graphics::draw(ctx, text, graphics::Point2::new((coords.0 * SIZE_GRID_PIXELS) as f32, (coords.1 * SIZE_GRID_PIXELS) as f32), 0.0)?;	
		Ok(())
	}

	fn contains(&mut self, x:i32, y:i32) -> bool {
		let coords = self.coord.get_coords();
		if x > (coords.0 * SIZE_GRID_PIXELS) && x < ((coords.0 + self.width) * SIZE_GRID_PIXELS) {
			if y > (coords.1 * SIZE_GRID_PIXELS) && y < ((coords.1 + self.height) * SIZE_GRID_PIXELS) {
				return true;
			}
		}

		false
	}

	pub fn mouse_click(&mut self) {
		println!("Clicked: {}", self.header);
	}
}

pub struct UiElem<T> {
	coord: Coord,
	height: i32,
	width: i32,
	header:String,
	contents: Vec<T>,
}

impl<T> UiElem<T> {
	pub fn new(coord: (i32, i32), height: i32, width: i32, header:String, contents: Vec<T>) -> Self {
		UiElem {
			coord: Coord::from(coord),
			height: height,
			width: width,
			header: header,
			contents: contents,
		}
	}
}

impl UiElem<Frame> {
	//@todo
	pub fn update(&mut self, text:Option<Vec<String>>) -> GameResult<()> {
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

	pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		let topix = |x:i32| { ((x * SIZE_GRID_PIXELS) as f32) };
		let coords = self.coord.get_coords();

		//graphics::draw(ctx, text, graphics::Point2::new((coords.0 * SIZE_GRID_PIXELS) as f32, (coords.1 * SIZE_GRID_PIXELS) as f32), 0.0)?;	
		
		//Draw header
		let ttf = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("Missing ttf file");
		let text = &graphics::Text::new(ctx, self.header.as_str(), ttf).expect("Error generating text");
		graphics::set_color(ctx, [0.1, 0.1, 0.1, 1.0].into())?;
		graphics::draw(ctx, text, graphics::Point2::new(topix(coords.0 + 1), topix(coords.1)), 0.0)?;	

		//Draw underline
		graphics::set_color(ctx, [0.1, 0.1, 0.1, 0.9].into())?; 
		graphics::line(
			ctx,
			&[
				Point2::new(topix(coords.0 + 1), topix(coords.1) + text.height() as f32),
				Point2::new(topix(coords.0 + 1) + text.width() as f32, topix(coords.1) + text.height() as f32),
			],
			4.0
		)?;

		for frame in self.contents.iter() {
			frame.draw(ctx)?;
		}	
		Ok(())
	}

	pub fn mouse_click(&mut self, x:i32, y:i32) {
		for frame in self.contents.iter_mut() {
			if frame.contains(x, y) {
				frame.mouse_click();
			}
		}
	}
}