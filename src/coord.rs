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

///Define coordinates
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
}

impl Clone for Coord {
	fn clone(&self) -> Coord { Coord::new(self.x, self.y) }
}