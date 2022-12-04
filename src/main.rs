use crossterm::{terminal, terminal::ClearType, cursor};
use std::io;

mod prompt;
use crate::prompt::Prompt;

fn main() 
{
    crossterm::execute!(io::stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
    crossterm::execute!(io::stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
    let prompt = Prompt::new();
    prompt.print();
}
