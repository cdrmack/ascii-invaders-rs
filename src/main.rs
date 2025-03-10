use std::io;
use std::{error::Error, time};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    // setup terminal
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    println!("sup");
    std::thread::sleep(time::Duration::from_secs(1));

    // restore terminal
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
