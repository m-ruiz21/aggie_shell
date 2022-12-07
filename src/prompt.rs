use std::env;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use colored::*;

pub struct Prompt
{
    user: String,
    user_time: DateTime<Local>,
    pub path: PathBuf,
}

impl Prompt
{
    pub fn new() -> Self
    {
        Self 
        {
            user : env::var("USER").expect("$USER variable not declared"),    
            user_time : Local::now(),
            path : env::current_dir().expect("Failed to get current directory"), 
        }
    }
    
    pub fn print(&self)
    {
        let formatted_time = format!("{}", self.user_time.format("%d/%m %T"));
        let formatted_path = format!("{}", self.path.display());
        print!("{0} {1}:{2}$ ", formatted_time.green(), self.user.green(), formatted_path.blue()); 
    }   

    pub fn update(&mut self)
    {
        // update time
        self.user_time = Local::now();
        
        // update path 
        self.path = env::current_dir().expect("Failed to get current directory"); 
    }

    pub fn exit_message(&self)
    {
        println!("{}", "Now exiting shell...\nGoodbye".red());
    }
}
