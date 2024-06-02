# egui_ratatui : egui widget + ratatui backend (WASM) 

[![Crates.io](https://img.shields.io/crates/v/egui_ratatui.svg)](https://crates.io/crates/egui_ratatui)
[![Documentation](https://docs.rs/egui_ratatui/badge.svg)](https://docs.rs/egui_ratatui/latest/egui_ratatui/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/egui_ratatui.svg)](https://crates.io/crates/egui_ratatui)


See Bevy and Macroquad folders for game engine examples


Web Demo : https://gold-silver-copper.github.io/ (works best on firefox)


See also:
https://ratatui.rs/
https://github.com/emilk/egui


RataguiBackend is the name of the backend/widget in code


`cargo add egui_ratatui`


`git clone https://github.com/gold-silver-copper/egui_ratatui.git`


`cd egui_ratatui`


`cd bevy_example`


`cargo run`


This repo provides the Ratagui Backend, which is also a Widget for egui. So you can have a whole ratatui terminal inside of egui. Also since egui is so WASM compatible, this means we can use this to compile ratatui to WASM !



If you would like more information about compiling to WASM please consult the relevant EGUI backend documentation:

Bevy: https://bevy-cheatbook.github.io/platforms/wasm.html 

Macroquad: https://macroquad.rs/articles/wasm/ 

eframe: https://github.com/emilk/eframe_template 


![](https://github.com/gold-silver-copper/egui_ratatui/blob/main/tyorhun.gif)
![](https://github.com/gold-silver-copper/egui_ratatui/blob/main/screen1.png)
![](https://github.com/gold-silver-copper/egui_ratatui/blob/main/screen2.png)


Previously this repo was called 'ratatatui_egui_wasm' and the crate was called 'ratframe' , I decided to rename it to egui_ratatui for clarity and discoverability , please see the PRE-RENAME branch for the last version of ratframe.
