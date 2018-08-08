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

//Defines program wide parameters
//for displaying in ggez
//|-----------------------|-----|
//|game                   |stats|
//|                       |-----|
//|                       |opts.|
//|                       |-----|
//|                       |Adv. |
//|-----------------------|-----|

//Hight of a line of ui text
pub const WIDTH_UI_INDENT:i32 = 5;
pub const HEIGHT_UI_LINE:i32 = 7;

//Size of grids in pixels
pub const SIZE_GRID_PIXELS: i32 = 5;

//program window
pub const WIDTH_WINDOW_GRIDS:i32 = 200;
pub const HEIGHT_WINDOW_GRIDS:i32 = 150;
pub const AREA_WINDOW_PIXELS: (i32, i32) = (WIDTH_WINDOW_GRIDS * SIZE_GRID_PIXELS, HEIGHT_WINDOW_GRIDS * SIZE_GRID_PIXELS);

//game_frame
pub const WIDTH_GAME_GRIDS:i32 = WIDTH_WINDOW_GRIDS - 50;
pub const HEIGHT_GAME_GRIDS:i32 = HEIGHT_WINDOW_GRIDS;
//const AREA_GAME_PIXELS: (i32, i32) = (WIDTH_GAME_GRIDS * SIZE_GRID_PIXELS, HEIGHT_GAME_GRIDS * SIZE_GRID_PIXELS);

//stat_frame
pub const WIDTH_STAT_GRIDS:i32 = WIDTH_WINDOW_GRIDS - WIDTH_GAME_GRIDS;
pub const HEIGHT_STAT_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 100;
//const AREA_STAT_PIXELS: (i32, i32) = (WIDTH_STAT_GRIDS * SIZE_GRID_PIXELS, HEIGHT_STAT_GRIDS * SIZE_GRID_PIXELS);

//advanced settings frame
pub const WIDTH_ADVANCED_GRIDS:i32 = WIDTH_STAT_GRIDS;
pub const HEIGHT_ADVANCED_GRIDS:i32 = HEIGHT_STAT_GRIDS;
//const AREA_ADVANCED_PIXELS: (i32, i32) = (WIDTH_ADVANCED_GRIDS * SIZE_GRID_PIXELS, HEIGHT_ADVANCED_GRIDS * SIZE_GRID_PIXELS);

//option_frame
pub const WIDTH_OPTION_GRIDS:i32 = WIDTH_STAT_GRIDS;
pub const HEIGHT_OPTION_GRIDS:i32 = HEIGHT_STAT_GRIDS;
//const AREA_OPTION_PIXELS: (i32, i32) = (WIDTH_OPTION_GRIDS * SIZE_GRID_PIXELS, HEIGHT_OPTION_GRIDS * SIZE_GRID_PIXELS);

//Set default coordinates of window elements
pub const POS_GAME_GRIDS: (i32, i32) = (0,0);
pub const POS_STAT_GRIDS: (i32, i32) = (POS_GAME_GRIDS.0 + (WIDTH_GAME_GRIDS), POS_GAME_GRIDS.1);
pub const POS_OPTION_GRIDS: (i32, i32) = (POS_STAT_GRIDS.0, POS_STAT_GRIDS.1 + HEIGHT_STAT_GRIDS);
pub const POS_ADVANCED_GRIDS: (i32, i32) = (POS_STAT_GRIDS.0, POS_OPTION_GRIDS.1 + HEIGHT_OPTION_GRIDS);