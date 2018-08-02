# Project Introduction
This repository contains my final project for CS 410: Rust Programming at Portland State University. The project is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using [Rust](https://www.rust-lang.org/en-US/) and the [ggez](https://github.com/ggez/ggez) 2D graphics framework. 

# Stages of Development
1. [x] [Benchmark 1](https://github.com/MatthewGreenlaw/GameOfLife/releases/tag/Benchmark-1): Basic implementation
   1. [x] Create basic game elements
      * [x] Cells
      * [x] Map
   2. [x] Setup ggez to work with game elements
      * [x] [EventHandler](//https://docs.rs/ggez/0.3.1/ggez/event/trait.EventHandler.html)
   3. [x] Implement the clasic Game of Life [rules](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules)

2. [ ] Benchmark 2: Add UI (In-Progress)
   1. [ ] Create basic UI elements
      * [ ] Stats side-bar
      * [ ] Insertion frame
      * [ ] Options frame
   2. [ ] Setup ggez to work with UI elements

# Installation Instructions
## Windows 10 installation
### Install Visual Studio build tools
  * Download and install `https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15#`
    * Make sure that `windows 10 SDK` is checked during installation settings

### Install SDL2 libraries
  * Download and unzip: `http://www.libsdl.org/release/SDL2-devel-2.0.8-VC.zip`
  * Move all .dll files:
    * from: `C:\{path to unzipped folder}\SDL2-2.0.8\lib\x86`
    * to: `C:\{path to rustup installation}\.multirust\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
    * to: `C:\{path to VS installation}Microsoft Visual Studio\2017\BuildTools\VC\Tools\MSVC\14.14.26428\lib\x64`

   * Move SDL2.dll 
    * from: `C:\{path to unzipped folder}\SDL2-2.0.8\lib\x86`
    * to: project root next to Cargo.toml