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


extern crate ggez;

use ggez::event::{self, MouseButton};
use ggez::{Context, GameResult, graphics};
use std::env;
use std::path;

//Import global parameters
mod params;
use params::{
	WIDTH_INDENT, HEIGHT_UI_LINE, AREA_WINDOW_PIXELS,
	WIDTH_STAT_GRIDS, HEIGHT_STAT_GRIDS, POS_STAT_GRIDS,
	WIDTH_PLAYER_GRIDS, HEIGHT_PLAYER_GRIDS, POS_PLAYER_GRIDS, 
	WIDTH_OPTION_GRIDS, HEIGHT_OPTION_GRIDS, POS_OPTION_GRIDS
};

//Import coordinate managment
mod gol;
use gol::{World};

mod coord;
mod ui;
use ui::{UiElem, Frame};

struct Game {
	game: World,
	option: UiElem<Frame>,
	stat: UiElem<Frame>,
	player: UiElem<Frame>,
}

impl Game {
	pub fn classic(ctx: &mut Context) -> Self {
		let x_offset = |x| { x * WIDTH_INDENT };
		let y_offset = |x| { x * HEIGHT_UI_LINE };

		let statelem = UiElem::new(
			POS_STAT_GRIDS, 
			HEIGHT_STAT_GRIDS, 
			WIDTH_STAT_GRIDS  - 5,
			"Stats".to_string(),
			vec![
				Frame::new(
					POS_STAT_GRIDS,
					5, 
					WIDTH_STAT_GRIDS - 5,
					x_offset(1),
					y_offset(1), 
					"Generation : ".to_string(),
					"",
					ctx,
				),
				Frame::new(
					POS_STAT_GRIDS,
					5, 
					WIDTH_STAT_GRIDS - 5,
					x_offset(1),
					y_offset(2), 
					"Living         : ".to_string(),
					"",
					ctx,
				),
				Frame::new(
					POS_STAT_GRIDS,
					5, 
					WIDTH_STAT_GRIDS - 5,
					x_offset(1),
					y_offset(3), 
					"Fatalities   : ".to_string(),
					"",
					ctx,
				),
			],
		);

		let optionelem = UiElem::new(
			//UiElems define where the frames live
			POS_OPTION_GRIDS, 
			HEIGHT_OPTION_GRIDS, 
			WIDTH_OPTION_GRIDS - 5, 
			"Options".to_string(),
			//Each frame defines where thier content lives witin the uielem
			vec![
				Frame::new(
					POS_OPTION_GRIDS,
					5, 
					WIDTH_STAT_GRIDS - 5,
					x_offset(1),
					y_offset(1), 
					"Pause".to_string(),
					"",
					ctx,
				),
				Frame::new(
					POS_OPTION_GRIDS,
					5, 
					WIDTH_STAT_GRIDS - 5,
					x_offset(1),
					y_offset(2), 
					"Restart".to_string(),
					"",
					ctx,
				),
				Frame::new(
					POS_OPTION_GRIDS,
					5, 
					WIDTH_OPTION_GRIDS - 5,
					x_offset(1),
					y_offset(3), 
					"Advanced Options".to_string(),
					"",
					ctx,
				),
			],
		);

		let playerelem = UiElem::new(
			POS_PLAYER_GRIDS, 
			HEIGHT_PLAYER_GRIDS, 
			WIDTH_PLAYER_GRIDS - 5,
			"Add life".to_string(), 
			vec![
			],
		);

		Game {
			game: World::new(2000),
			option: optionelem,
			stat: statelem,
			player: playerelem,
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