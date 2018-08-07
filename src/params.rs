//Hight of a line of ui text
pub const WIDTH_INDENT:i32 = 5;
pub const HEIGHT_UI_LINE:i32 = 8;

//Size of grids in pixels
pub const SIZE_GRID_PIXELS: i32 = 5;

//program window
pub const WIDTH_WINDOW_GRIDS:i32 = 150;
pub const HEIGHT_WINDOW_GRIDS:i32 = 150;
pub const AREA_WINDOW_PIXELS: (i32, i32) = (WIDTH_WINDOW_GRIDS * SIZE_GRID_PIXELS, HEIGHT_WINDOW_GRIDS * SIZE_GRID_PIXELS);
//|-----------------------|-----|
//|game                   |stats|
//|                       |     |
//|                       |     |
//|-----------------------|-----|
//|Player                 |opts.|
//|-----------------------|-----|
//game_frame
pub const WIDTH_GAME_GRIDS:i32 = WIDTH_WINDOW_GRIDS - 50;
pub const HEIGHT_GAME_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 50;
//const AREA_GAME_PIXELS: (i32, i32) = (WIDTH_GAME_GRIDS * SIZE_GRID_PIXELS, HEIGHT_GAME_GRIDS * SIZE_GRID_PIXELS);

//stat_frame
pub const WIDTH_STAT_GRIDS:i32 = WIDTH_WINDOW_GRIDS - WIDTH_GAME_GRIDS;
pub const HEIGHT_STAT_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - 50;
//const AREA_STAT_PIXELS: (i32, i32) = (WIDTH_STAT_GRIDS * SIZE_GRID_PIXELS, HEIGHT_STAT_GRIDS * SIZE_GRID_PIXELS);

//player_frame
pub const WIDTH_PLAYER_GRIDS:i32 = WIDTH_GAME_GRIDS;
pub const HEIGHT_PLAYER_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - HEIGHT_GAME_GRIDS;
//const AREA_PLAYER_PIXELS: (i32, i32) = (WIDTH_PLAYER_GRIDS * SIZE_GRID_PIXELS, HEIGHT_PLAYER_GRIDS * SIZE_GRID_PIXELS);

//option_frame
pub const WIDTH_OPTION_GRIDS:i32 = WIDTH_WINDOW_GRIDS-WIDTH_PLAYER_GRIDS;
pub const HEIGHT_OPTION_GRIDS:i32 = HEIGHT_WINDOW_GRIDS - HEIGHT_STAT_GRIDS;
//const AREA_OPTION_PIXELS: (i32, i32) = (WIDTH_OPTION_GRIDS * SIZE_GRID_PIXELS, HEIGHT_OPTION_GRIDS * SIZE_GRID_PIXELS);

//Set default coordinates of window elements
//Position is the center of the element
pub const POS_GAME_GRIDS: (i32, i32) = (0,0);
pub const POS_STAT_GRIDS: (i32, i32) = (POS_GAME_GRIDS.0 + (WIDTH_GAME_GRIDS), POS_GAME_GRIDS.1);
pub const POS_PLAYER_GRIDS: (i32, i32) = (POS_GAME_GRIDS.0, POS_GAME_GRIDS.1 + (HEIGHT_GAME_GRIDS));
pub const POS_OPTION_GRIDS: (i32, i32) = (POS_STAT_GRIDS.0, POS_PLAYER_GRIDS.1);