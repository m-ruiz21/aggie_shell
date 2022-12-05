use crossterm::{terminal, terminal::ClearType, cursor};
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;
use std::path::PathBuf;

mod prompt;
use crate::prompt::Prompt;

fn main() 
{
    crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
    crossterm::execute!(stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
    let mut prompt = Prompt::new();
    let mut prev_path : String = prompt.path.clone().into_os_string().into_string().unwrap();
    loop {
        prompt.update();
        prompt.print();
        stdout().flush().expect("flush failed!");
        
        
        let mut input = String::new();    
        stdin().read_line(&mut input).expect("Did not enter a valid string");
        
        let mut args = input.trim().split_whitespace();
        let cmd = args.next().unwrap();
        
        match cmd 
        {
            "exit" => {
                prompt.exitMessage();
                break;
            },
            "clear" => 
            {  
                crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
                crossterm::execute!(stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
            },
            "cd" => {
                prev_path = prompt.path.clone().into_os_string().into_string().unwrap();
                let new_dir = args.peekable().peek().map_or(prev_path.clone(), |x| String::from(*x));
                let new_path = Path::new(&new_dir);
                env::set_current_dir(&new_path).expect("invalid directory");
            },
            _ => {
                let child = Command::new(cmd)
                .args(args)
                .spawn();

                match child
                {
                    Ok(mut child) => { child.wait(); },
                    Err(error) => { eprintln!("{}", error); }                    
                };
            }
        }
    }
}
