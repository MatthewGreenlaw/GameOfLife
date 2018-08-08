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

//Import graphics crate
extern crate ggez;
use ggez::{Context, graphics};
use ggez::graphics::{Point2};

//Import global parameters
use coord::{Coord};
use params::{ SIZE_GRID_PIXELS };

//Defines a base UI element that has a header and text
pub struct Frame {
	coord: Coord,
	height: i32,
	width: i32,
	header: String,
	text: String,
}

impl Frame {
	/// Generates a new Frame container.
	///
	/// # Arguments
	/// * 'coord'  - Parent's (x, y) coordintes. Note: Coordinates are based off the center of the object.
	/// * 'height' - Height of this container.
	/// * 'width'  - Width of this container.
	/// * 'indent' - X offset from parent.
	/// * 'offset' - Y offset from parent.
	/// * 'header' - Header text describing children.
	/// * 'text'   - Data to display.
	pub fn new(coord: (i32, i32), height: i32, width: i32, indent: i32, offset: i32, header:String, text: &str) -> Self {
		Frame {
			coord: Coord::from((coord.0 + indent, coord.1 + offset)),
			height: height,
			width: width,
			header: header,
			text: text.to_string(),
		}
	}
	
	/// Updates the text of a Frame with new data.
	///
	/// # Arguments
	/// * 'text'   - New data to display.
	fn update(&mut self, text:&str) {
		self.text = text.to_string();
	}

	/// Draws the header and text onto the program context at the designated coordinates. 
	///
	/// # Arguments
	/// * 'ctx' - [ggez global resources](https://docs.rs/ggez/0.3.1/ggez/struct.Context.html). 
	fn draw(&self, ctx: &mut Context) {
		let topix = |x:i32| { ((x * SIZE_GRID_PIXELS) as f32) };
		let coords = self.coord.get_coords();

		//Build the string to draw
		let mut text = self.header.to_string();
		text.push_str(self.text.as_str());

		//Build the graphical representation of the text using ttf
		let ttf = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("Missing ttf file");
		let text = &graphics::Text::new(ctx, text.as_str(), ttf).expect("Error generating text");

		//Draw the text
		graphics::set_color(ctx, [0.5, 0.5, 0.5, 1.0].into()).expect("Error setting color");
		graphics::draw(ctx, text, graphics::Point2::new(topix(coords.0), topix(coords.1)), 0.0).expect("Error drawing text");	
	}

	/// Determines if this element covers an area of coordinates that contain a target coordinate.
	/// The bounds of this element are determined by its own coordinates, height, and width.
	///
	/// # Arguments
	/// * 'x & y'  - The target coordinate.
	///
	/// # Return
	/// * bool - True if the coordinate is within the bounds of this element, False otherwise.
	fn contains(&mut self, x:i32, y:i32) -> bool {
		let topix = |x:i32| { ((x * SIZE_GRID_PIXELS) as i32) };
		let coords = self.coord.get_coords();
		if x > topix(coords.0) && x < (topix(coords.0 + self.width)) {
			if y > topix(coords.1) && y < (topix(coords.1 + self.height)) {
				return true;
			}
		}

		false
	}

	///Action to perform if user clicks a coordinate in this element.
	pub fn mouse_click(&mut self) {
		println!("Clicked: {}", self.header);
	}
}

//Define a UI element with a header and a vector of generic children.
pub struct UiElem<T> {
	coord: Coord,
	// height: i32,
	// width: i32,
	header:String,
	children: Vec<T>,
}

impl<T> UiElem<T> {
	/// Generates a new UiElem container.
	///
	/// # Arguments
	/// * 'coord'  - This container's (x, y) coordintes. Note: Coordinates are based off the center of the object.
	/// * 'header' - Text describing children.
	/// * 'children'   - Generic children that this container manages.
	pub fn new(coord: (i32, i32), _height: i32, _width: i32, header:String, children: Vec<T>) -> Self {
		UiElem {
			coord: Coord::from(coord),
			// height: height,
			// width: width,
			header: header,
			children: children,
		}
	}
}

///Implementation for UiElem with Frame children
impl UiElem<Frame> {
	/// Updates contained Frames with new text attributes.
	/// struct Game is the client of this function and calls update on all of its elements
	/// if they need them or not. Wrapping the strings in an Option is the way UiElem 
	/// decides if a Frame needs to update. I used this approach primarily in development
	/// because not all UiElems were ready for the complete update chain.
	///
	/// # Arguments
	/// * 'text'   - A vector of strings. The index of a string in the vector corelates to the index of the Frame in the children vector.
	pub fn update(&mut self, text:Option<Vec<String>>) {
		//Varify that there is something to update with
		match text {
			Some(text) => {
				//For each of the children, update it with the corasponding string. Order is important.
				for (i, frame) in self.children.iter_mut().enumerate() {
					frame.update(text[i].as_str());
				}	
			},
			None =>(),
		}
	}

	/// Draws the header and underline, then sends the draw command to children. 
	///
	/// # Arguments
	/// * 'ctx' - [ggez global resources](https://docs.rs/ggez/0.3.1/ggez/struct.Context.html).
	pub fn draw(&mut self, ctx: &mut Context) {
		let topix = |x:i32| { ((x * SIZE_GRID_PIXELS) as f32) };
		let coords = self.coord.get_coords();
		
		//Draw header
		let ttf = &graphics::Font::new(ctx, "/Pacifico.ttf", 24).expect("Missing ttf file");
		let text = &graphics::Text::new(ctx, self.header.as_str(), ttf).expect("Error generating text");
		graphics::set_color(ctx, [0.1, 0.1, 0.1, 1.0].into()).expect("Error setting color");
		graphics::draw(ctx, text, graphics::Point2::new(topix(coords.0 + 1), topix(coords.1)), 0.0).expect("Error drawing header");	

		//Draw underline
		graphics::set_color(ctx, [0.1, 0.1, 0.1, 0.9].into()).expect("Error setting color"); 
		graphics::line(
			ctx,
			&[
				Point2::new(topix(coords.0 + 1), topix(coords.1) + text.height() as f32),
				Point2::new(topix(coords.0 + 1) + text.width() as f32, topix(coords.1) + text.height() as f32),
			],
			4.0
		).expect("Error generating line");

		//Send draw command to children
		for frame in self.children.iter() {
			frame.draw(ctx);
		}
	}

	/// Sends the mouse_click to children if they contain the target coordinate
	///
	/// # Arguments
	/// * 'x & y'  - The target coordinate.
	pub fn mouse_click(&mut self, x:i32, y:i32) {
		for frame in self.children.iter_mut() {
			if frame.contains(x, y) {
				frame.mouse_click();
			}
		}
	}
}