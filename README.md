# Remaking Cavestory in Rust
Inspired by the youtube series about remaking the game Cavestory in C++ (https://github.com/Limeoats/cavestory-development), I started working on remaking it in Rust. Just to learn Rust. Soon I ran into trouble with the object oriented approch, especially with the SDL canvas and -textures. They don't really like the borrow checker :).
After some investigation I found this (https://sunjay.dev/learn-game-dev/intro.html) tutorial from a game in Rust with the Entity - Component - Systems approach, with the Specs library. So now it is a combination of the 2 tutorials.

Progress:
- [x] Create a SDL window
- [x] Draw the player 
- [x] Have it move with the keyboard
- [x] Animate the player
- [x] Read the Level map
- [x] Render the background
- [x] Collision detection
- [ ] Slopes
- [ ] Enemies
- [ ] HUD
