# Project Introduction
This repository contains my final project for CS 410: Rust Programming at Portland State University. The project is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using [Rust](https://www.rust-lang.org/en-US/) and the [ggez](https://github.com/ggez/ggez) 2D graphics framework. 

## Note on compatability
This program was written and compiled in Windows using rustc 1.28.0-nightly (e3bf634e0 2018-06-28). Compiling the same code on other versions of the rust compiler may produce errors. Example: rustc 1.25.0 throws "non-reference patter used to match reference" errors from code in gol.rs on lines 79, 192, and 193 which can be solved by updating the following lines of gol.rs:
* Line 79: dereference cell - `*cell`
* Line 192: dereference life - `*life`
* Line 193: Reference cell - 'Some(ref cell)'

# Stages of Development
1. [x] [Stage 1](https://github.com/MatthewGreenlaw/GameOfLife/releases/tag/Benchmark-1): Basic implementation
   1. [x] Create basic game elements
      * [x] Cells
      * [x] Map
   2. [x] Setup ggez to work with game elements
      * [x] [EventHandler](https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html)
      * [x] [ContextBuilder](https://docs.rs/ggez/0.4.1/ggez/struct.ContextBuilder.html)
      * [x] [run](https://docs.rs/ggez/0.3.1/ggez/event/fn.run.html)
   3. [x] Implement the clasic Game of Life [rules](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules)

2. [x] [Stage 2](https://github.com/MatthewGreenlaw/GameOfLife/releases/tag/Stage-2): Add UI
   1. [x] Create basic UI elements
      * [x] Import and display ttf
      * [x] Game Map element
      * [x] Stats element
      * [x] Options element
      * [x] Advanced options element
      
   2. [x] UI and Mouse interaction
      * [x] User interaction
         * [x] Capture mouse clicks
         * [x] Identify which area was clicked
      * [x] Update stat area with game statistics
      * [x] Option menu
         * [x] Implement user interaction for menu
         * [x] Update options upon user interaction

2. [ ] Stage 3: Text input and Advanced settings
   1. [ ] Capture keyboard clicks     
   2. [ ] Advanced option menu
      * [ ] Implement user interaction and text input for advanced menu
      * [ ] Update options upon user interaction

# Installation Instructions

## Linux
### Install [rust](https://www.rust-lang.org/en-US/install.html)

### Install [SDL2 libraries](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements)
  * Run 'sudo apt-get install libsdl2-dev'
  * Add the following to Cargo.toml: 
  ```
  [dependencies.gl]
  git = "https://github.com/bjz/gl-rs"
  ```
  * [Additional updates](https://github.com/ggez/ggez/issues/194) to graphics drivers may be needed.

###
## Windows 10 installation
### Install [Rustup](https://win.rustup.rs/)

### Install Visual Studio build tools
  * Download and install `https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15#`
    * Make sure that `windows 10 SDK` is checked during installation settings

### [Install SDL2 libraries](https://github.com/Rust-SDL2/rust-sdl2#windows-with-build-script)
  * Download and unzip: `http://www.libsdl.org/release/SDL2-devel-2.0.8-VC.zip`
  * Move all .dll files:
    * from: `C:\{path to unzipped folder}\SDL2-2.0.8\lib\x86`
    * to: `C:\{path to rustup installation}\.multirust\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
    * to: `C:\{path to VS installation}Microsoft Visual Studio\2017\BuildTools\VC\Tools\MSVC\14.14.26428\lib\x64`

   * Move SDL2.dll 
    * from: `C:\{path to unzipped folder}\SDL2-2.0.8\lib\x86`
    * to: project root next to Cargo.toml