mod maze;
mod utils;
mod node;

extern crate sdl2;

use crate::maze::*;


fn main() -> Result<(), String> {
    let mut m = Maze::new(10, 10)?;
    m.draw()?;
    m.main_loop()?;
    Ok(())
}

