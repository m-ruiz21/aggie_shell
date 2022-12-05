use crossterm::{terminal, terminal::ClearType, cursor};
use std::io::{stdin, stdout, Write};
use std::process::{Command, Child};
use std::path::Path;
use std::env;
use std::process::Stdio;
use std::fs::File;

mod prompt;
use crate::prompt::Prompt;

fn main() 
{
    crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
    crossterm::execute!(stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
    let mut prompt = Prompt::new();
    let mut prev_path : String = prompt.path.clone()
                                            .into_os_string()
                                            .into_string()
                                            .unwrap();
    loop {
        prompt.update();
        prompt.print();
        stdout().flush().expect("flush failed!");
        
        
        let mut input = String::new();    
        stdin().read_line(&mut input).expect("Did not enter a valid string");
        
        let mut cmds = input.trim().split(" | ").peekable();
        let mut prev_cmd = None;
        
        while let Some(cmd) = cmds.next()
        {
            let mut args = cmd.trim().split_whitespace();
            let cmd = args.next().unwrap();  
            let args_vec = args.clone().collect::<Vec<&str>>();

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

                    prev_cmd = None;
                },
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let prev_dir = prev_path.clone();                
                    let new_path = match new_dir 
                    {
                        "-" => Path::new(&prev_dir),
                        _ => Path::new(&new_dir), 
                    }; 
                    
                    prev_path = prompt.path.clone().into_os_string().into_string().unwrap();
                    if let Err(error) = env::set_current_dir(&new_path) { eprint!("{}", error); }

                    prev_cmd = None;
                },
                _ => {
                    // set input
                    let stdin = prev_cmd
                                .map_or(
                                    Stdio::inherit(), 
                                    |output: Child| Stdio::from(output.stdout.unwrap())
                                );
                    
                    // set output
                    let output_position = args.position(|x| x == ">");
                    let has_output = (output_position != None) && (output_position.unwrap() < args_vec.len()); 
                    
                    let mut args_it = args_vec.iter();
                    let stdout: Stdio; 
                    if has_output 
                    {
                        let file = File::create(args_vec[output_position.unwrap()+1])
                                        .expect("Failed to create file");
                        stdout = Stdio::from(file);
                        args_it = args_vec[0.. output_position.unwrap()].iter();              
                    }
                    else if cmds.peek().is_some()
                    {
                        stdout = Stdio::piped();
                    }
                    else 
                    {
                        stdout = Stdio::inherit();
                    };
                    
                    let child = Command::new(cmd)
                    .args(args_it)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                    match child
                    {
                        Ok(mut child) => { prev_cmd = Some(child); },
                        Err(error) => { 
                            prev_cmd = None; 
                            eprintln!("{}", error); 
                        }                    
                    };
                }
            }
        }
        if let Some(mut final_cmd) = prev_cmd { final_cmd.wait(); }
    }
}
