// Copyright 2016 Matthew Greenlaw.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//Grab env and path for use with external files
use std::env;
use std::path;

//Import graphics crate
extern crate ggez;
use ggez::event::{self, MouseButton};
use ggez::{Context, GameResult, graphics};

//Import global parameters
mod params;
use params::{
	WIDTH_UI_INDENT, HEIGHT_UI_LINE, AREA_WINDOW_PIXELS,
	WIDTH_STAT_GRIDS, HEIGHT_STAT_GRIDS, POS_STAT_GRIDS,
	WIDTH_ADVANCED_GRIDS, HEIGHT_ADVANCED_GRIDS, POS_ADVANCED_GRIDS, 
	WIDTH_OPTION_GRIDS, HEIGHT_OPTION_GRIDS, POS_OPTION_GRIDS
};

//Import game of life managment
mod gol;
use gol::{World};

//Import user interface managment
mod coord;
mod ui;
use ui::{UiElem, Frame};

///Define game elements
struct Game {
	game: World,
	option: UiElem<Frame>,
	stat: UiElem<Frame>,
	advanced: UiElem<Frame>,
	paused: bool,
}

//Implement game functions
impl Game {

	/// Creates a game with classic Game of Life rules
	pub fn classic() -> Self {
		let x_offset = |x| { x * WIDTH_UI_INDENT };
		let y_offset = |x| { x * HEIGHT_UI_LINE };

		//UiElems define where the frames live in the window
		let statelem = UiElem::new(
			POS_STAT_GRIDS, 
			HEIGHT_STAT_GRIDS, 
			WIDTH_STAT_GRIDS  - x_offset(1),
			"Stats".to_string(),
			vec![
				//Each frame defines where thier content lives witin the uielem
				Frame::new(
					POS_STAT_GRIDS,
					y_offset(1), 
					WIDTH_STAT_GRIDS - x_offset(1),
					x_offset(1),
					y_offset(1), 
					"Generation : ".to_string(),
					"",
				),
				Frame::new(
					POS_STAT_GRIDS,
					y_offset(1), 
					WIDTH_STAT_GRIDS - x_offset(1),
					x_offset(1),
					y_offset(2), 
					"Living         : ".to_string(),
					"",
				),
				Frame::new(
					POS_STAT_GRIDS,
					y_offset(1), 
					WIDTH_STAT_GRIDS - x_offset(1),
					x_offset(1),
					y_offset(3), 
					"Fatalities   : ".to_string(),
					"",
				),
			],
		);

		let optionelem = UiElem::new(
			POS_OPTION_GRIDS, 
			HEIGHT_OPTION_GRIDS, 
			WIDTH_OPTION_GRIDS - x_offset(1), 
			"Options".to_string(),
			vec![
				Frame::new(
					POS_OPTION_GRIDS,
					y_offset(1), 
					WIDTH_STAT_GRIDS - x_offset(1),
					x_offset(1),
					y_offset(1), 
					"Pause".to_string(),
					"",
				),
				Frame::new(
					POS_OPTION_GRIDS,
					y_offset(1), 
					WIDTH_STAT_GRIDS - x_offset(1),
					x_offset(1),
					y_offset(2), 
					"Restart".to_string(),
					"",
				),
			],
		);

		let advancedelem = UiElem::new(
			POS_ADVANCED_GRIDS, 
			HEIGHT_ADVANCED_GRIDS, 
			WIDTH_ADVANCED_GRIDS - x_offset(1),
			"Advanced Options".to_string(), 
			vec![
			],
		);

		Game {
			game: World::new(2000),
			option: optionelem,
			stat: statelem,
			advanced: advancedelem,
			paused: false,
		}
	}
}


impl event::EventHandler for Game{
//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html
//Must override at least update() and draw() methods

	/// Mandatory override of [EventHandler::update()](https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html#tymethod.update)
	///
	/// Updates all game elements
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

		if !self.paused {
			let generation = self.game.update();
			self.stat.update(Some(
						vec![
							generation.0.to_string(),
							generation.1.to_string(),
							generation.2.to_string()
						]
					));
		}
		self.option.update(None);
		self.advanced.update(None);
		Ok(())//Update for game-over scenario?
	}

	/// Mandatory override of [EventHandler::draw()](https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html#tymethod.draw)
	///
	/// Draws all game elements
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		self.game.draw(ctx);
		self.option.draw(ctx);
		self.stat.draw(ctx);
		self.advanced.draw(ctx);
		graphics::present(ctx);
		Ok(())
	}

	/// Optional override of [EventHandler::mouse_button_up_event()](https://docs.rs/ggez/0.4.3/ggez/event/trait.EventHandler.html#method.mouse_button_up_event)
	///
	/// Pass mouse click data to game elements
	fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: i32, y: i32){
	//https://docs.rs/ggez/0.4.3/ggez/event/trait.EventHandler.html#method.mouse_button_up_event
		match self.option.mouse_click(x, y) {
			Some(string) => {
				match string.as_str() {
					"Pause" => { self.paused = true; self.draw(ctx).expect("Error drawing game"); },
					"Start" => { self.paused = false; self.draw(ctx).expect("Error drawing game"); },
					"Restart" => { 
						self.game = World::new(2000);
						self.stat.update(Some(vec!["0".to_string(), "2000".to_string(), "0".to_string()]));
						self.draw(ctx).expect("Error drawing game"); 
					},
					&_ => {},
				}
			}, 
			None => match self.stat.mouse_click(x, y) {
				Some(_string) => {},
				None => match self.advanced.mouse_click(x, y) {
					Some(_string) => {},
					None => (),
				}
			}
		}
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
		//https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html
		let window = &mut ggez::ContextBuilder::new("Game of Life    Copyright 2016 Matthew Greenlaw", "Matthew Greenlaw")
		.window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
		.window_mode(ggez::conf::WindowMode::default().dimensions(AREA_WINDOW_PIXELS.0 as u32, AREA_WINDOW_PIXELS.1 as u32))
		.add_resource_path(path_buffer)
		.build().expect("Failed to build game.");

		graphics::set_background_color(window, [1.0, 1.0, 1.0, 1.0].into());

		//Build the game
		let game = &mut Game::classic(); 

		//Run the main game loop
		match event::run(window, game){
		//https://docs.rs/ggez/0.3.0/ggez/event/fn.run.html
			Ok(_) => println!("Copyright 2016 Matthew Greenlaw. Download from: https://github.com/MatthewGreenlaw/GameOfLife"),
			Err(error) => println!("Error running game: {:?}", error),
		}
	}
	else {
		panic!("Usage: cargo run");
	}
}