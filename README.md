# snake.rs
A snake game a made in Rust, that can save data between playthroughs. Built using macroquad and quad-storage   

## What is it?
---
It's a replicate of the classic snake game, but created in Rust. 
The game plays exactly like the original, except I made the starting snake one unit long, because I felt like it.   
   
As a reminder, here is how the game works:
- The snake's direction is controlled by the player
- The player can change direction freely except they cannot turn the opposite direction (e.g going right, turning left)
- The apple's location is randomly generated somewhere on the grid. The player's goal is to eat as many apples as possible in a single run
- The snake grows by a unit every time an apple is consumed
- There are two ways in which a player can get eliminated: the snake touches the border of the grid, or its head touches its own body
  
## Controls
---
- Up: W or Up arrow key
- Down: S or Down arrow key
- Left: A or Left arrow key
- Right: D or Right arrow key

## How to Play?
---
- To play on windows, simply download the executable file.
- For other systems, use the rust compiler to generate an OS-compatible file.

- If you don't want to go through the compilation process, I've also added the Web Assembly (.wasm) file to run in the browser, although that might require some more setup.
