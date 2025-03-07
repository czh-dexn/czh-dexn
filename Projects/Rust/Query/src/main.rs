use std::io::{self, Read, Write};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, ScrollUp, SetSize, size}
};

fn main() -> io::Result<()> {
    let (cols, rows): (u16, u16) = size()?;
    // Resize terminal and scroll up.
    execute!(
        io::stdout(),
        SetSize(20, 20),
        ScrollUp(10)
    )?;

    execute!(io::stdout(), SetSize(cols, rows))?;
    Ok(())
}
