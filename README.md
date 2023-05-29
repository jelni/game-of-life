# Conway's Game of Life

I haven't programmed a Game of Life before, so here's my attempt:

[Conway's Game of Life](https://wikipedia.org/wiki/Conway%27s_Game_of_Life) Rust
implementation using a
[point quadtree](https://wikipedia.org/wiki/Quadtree#Point_quadtree) data
structure.

The use of a quadtree allows the board to be infinite.

![Screenshot of the program showing a Gosper's glider gun](https://github.com/jelni/game-of-life/assets/25802745/b2b9f12f-f097-4e7e-a405-3e230640680c)

## Interactions

-   Right mouse button – add cells
-   Left mouse button – remove cells
-   Mouse wheel – change the scale
-   <kbd>Space</kbd> – pause or resume
-   <kbd>N</kbd> – go forward one step
-   <kbd>Ctrl</kbd>+<kbd>Z</kbd> – go back to the last resume state

## Thoughts on the [`macroquad`](https://github.com/not-fl3/macroquad) crate

This crate is easy to use and renders the screen using a GPU shader, but I don't
like how there's an implicit global window instance and that every value (window
size, mouse position, scroll distance) is a float. It also doesn't support arrow
keys. I didn't find a crate I liked more.
