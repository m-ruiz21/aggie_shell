# System Programming in Rust By Example #
## Intro ##
Intro to rust, cargo, setting up, and the features of our aggie shell. 
## Creating Prompt.cpp ##
### Defining the struct ###
Defining a struct in Rust is similar to C++, except that in Rust, structs are private by default, so we need to declare it as public using the "pub" keyword.
> Note: we're not going to dive into OOP with Rust, but for now, because of the default private nature of structs, we can think of structs in Rust more like classes in C++. 
```
pub struct Prompt
{
    user: String,
    user_time: DateTime<Local>,
    path: PathBuf,
}
```
Since this struct has a constructor and functions, we're also going to create an "implementation" of the struct. 
Implementations are where we put the functions that make up the class or struct. 
Within this implementation, we're going to add the print(), update(), and exit_message() class methods.

Now that we have the general structure of our class, we need to create its constructor.
Constructors in Rust are made by creating a public function called new(), which returns type "Self".
We can innitialize our variables and their types here.
Note that class members in Rust are private by default. For that reason, we need to prefix all of our public function declerations with the "pub" keyword. 
```
impl Prompt
{
    pub fn new() -> Self
    {
        Self 
        {
            user : // initalize user here,    
            user_time : // initialize user_time path,
            path : // initialize path here, 
        }
    }
}
```
### Working with environment variables in Rust ###
The Rust standard library includes an enviroment module that allows us manipulate and inspect the enviroment variables of our machine. We will use this module to get our user and path.
The enviornment module comes with a built in current_dir() function, which allows us to get the directory as PathBuf object (similar to a String).
For the user, however, we need to use the "var" function. The environment variables in Rust are stored as key value pairs. So the var function takes it a key (the name of the environemnt variable), and returns to us the value of our environment variable.  

Applying this to our constructor:
```
use std::env;
use std::path::PathBuf;

...

impl Prompt
{
    pub fn new() -> Self
    {
        Self 
        {
            user : env::var("USER"),    
            user_time : // initialize ,
            path : env::current_dir(), 
        }
    }
}
```

### Time in Rust ###
To get the current system time, we can use the chrono module. Since this is an external dependency, we have to edit our cargo.toml.   
Under dependencies:
```
chrono = "0.4.23"
```
This way, the next time we run "cargo run", cargo will download version 0.4.23 of chrono.  
Now that we have chrono, we can import DateTime and Local module to get the local datetime.

Finishing up our constructor:
```
impl Prompt
{
    pub fn new() -> Self
    {
        Self 
        {
            user : env::var("USER"),    
            user_time :  Local::now(),
            path : env::current_dir(), 
        }
    }
}
```
### Cleaning up our constructor: dealing with Result<> objects ### 
If you try to compile this now, you'll get a warning that we're not doing anything with the std::result our functions are returning. An std::result is a type used for discovering and propogating erros. If your function returns a result, you can utalize the ? operator to handle and propogate the result. For example:
```
    user = env::var("USER")?;
```
However, in any function that does not return a result, we can simply utalize the expect() method.
This method allows us to handle any possible error and print out our prefered error message.
Applying this to our constructor:
```
pub fn new() -> Self
{
    Self 
    {
        user : env::var("USER").expect("$USER variable not declared"),    
        user_time : Local::now(),
        path : env::current_dir().expect("Failed to get current directory"), 
    }
}
```

### Completing the print() Function ###
What use is a prompt if we can't print it out? In this section, we're going to actually print our custom prompt. The format of our prompt will include all of our class variables in the following order:
```
{user_time (day/month time)} {user}: {path}$
```
Like our terminal, our user time and user name will be colored in green, and our path will be colored in blue. 
We will do the coloring using the colored library. We can import it by pasting the following line in cargo.toml:
```
colored = "2"
```
First things first, we have to set our time in our prefered format. 
Unfortunately, our path and user_time have their respective display / format methods, they do not return a string object.
This is an issue because the colored library only works for string literals. 
So, we will first need to convert our variables into formatted strings before we can color them and print them out.
For this, we can use the built in format!() macro that takes in a format string ( a string literal containing "{}") and the format string parameters (which will replace our "{}" strings), and returns our formatted string.
For our user_time, we can use the built in .format() method to give us our formatted time, and for the path, we can simply call the .display() method.
```
pub fn print(&self)
{
    let formatted_time = format!("{}", self.user_time.format("%d/%m %T"));
    let formatted_path = format!("{}", self.path.display());
}
```
Note that we have to import self for all class methods (similar to Python). 
From here, we just need to print our a formatted string with our colored format parameters:
```
print!("{0} {1}:{2}$ ", formatted_time.green(), self.user.green(), formatted_path.blue()); 
```
That will give us the final method:
```
pub fn print(&self)
{
    let formatted_time = format!("{}", self.user_time.format("%d/%m %T"));
    let formatted_path = format!("{}", self.path.display());
    print!("{0} {1}:{2}$ ", formatted_time.green(), self.user.green(), formatted_path.blue()); 
}   
```

### Completing the update() Function ###
Next, our prompt needs to be able to update as the time / directory changes. 
To do this, we can simply call the same methods we did to initialize our variables in the constructor.
That wil give us the final method:
```
pub fn update(&mut self)
{
    // update time
    self.user_time = Local::now();

    // update path 
    self.path = env::current_dir().expect("Failed to get current directory"); 
}
```

### Completing the exitMessage() function ###
Finally, we want to say goodbye to the user whenever they are exiting the shell.
For this, we can just println! our message in red using the colored library.
```
pub fn exit_message(&self)
{
    println!("{}", "Now exiting shell...\nGoodbye".red());
}
```

## Getting started with main.rs ##
### Importing our Struct ###
Returning back to the main.rs file, we can import our struct into our main.rs by declaring the module and importing it as a crate. 
By declaring it in our main.rs file, we can link our prompt.rs "libray" with our main code.
```
mod prompt;
use crate::prompt::Prompt;
```
### Clearing the Screen and Prompting the User ###
Now that we can finally declare a new Prompt object and use it in our main.rs function, let's clear our screen and print our prompt.
To clear our screen we can use crossterm, a terminal library for Rust.
We can import it by pasting the following to Cargo.toml:
```
crossterm = "0.23"
```
Now, we can use the crossterm execute() method run the terminal::Clear() command in our terminal:
```
crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
```
Running this results in our entire screen clearing our, but now our cursor is left at the very bottom! 
So, we need to move back our cursor to the top before we can print out our prompt.
We can do this by using the execute() method again to call the cursor::MoveTo() method:
```
crossterm::execute!(stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
```
Now that we have our screen cleared and cursor in the right spot, lets make our Prompt object and print:
```
crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
crossterm::execute!(stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
let mut prompt = Prompt::new();
prompt.print();
```
> This is our first encounter with the "mut" keyword. By default, all Rust variables are immutable. It's one of the ways that Rust encourages better and safer code. We can override this thorugh the "mut" keyword. This way, we can change / update our prompt object.

Now that that's done, we can run our code and finally see our beautiful prompt show up!
```
<show image here>
```

### Taking In User Input and Run Basic Commands ##
We can take in user input through the stdin.read_line(&String) method. It writes the stdin to the input string.
So, lets create a new mutable string and give it the user input.
```
let mut input = String::new();    
stdin().read_line(&mut input).expect("Did not enter a valid string");
```
Now that we have the user input, lets trim and run the command using the "Command" object from the process module.
```
let cmd = input.trim();
Command::new(cmd)
        .spawn()
        .expect("Failed to run command");
```
> In C this would be equivalent of forking a child process and running execvp()
Now we're able to run simple commands with no parameters like "ls" or "dir"!

### Accepting Arguments ###
Observe the following command:
```
cat dog.txt
```
If we want to run the "cat" command with the "dog.txt" parameter, we need to split the words using the whitespace, take the first word "cat" as the command, and the rest of the string "dog.txt" as the parameters.    
To split the input, we can use the split_whitespace() method to split up the input. This returns an iterator for a collection of strings in our input. 
So, lets use the next() method to return the first element of the collection, and move the iterator down to the next element.
Now, we have the cmd variable that only holds the command, and the args iterator that now holds the rest of our input string (arguments to our given cmd).
```
let mut args = cmd.trim().split_whitespace();
let cmd = args.next().unwrap(); 
```
> Note the unwrap() method. This is necessary because the .next() method returns an Option() object. The Option type represents an optional value. It either holds Some() if it contains a value, or None() if it does not. In this case, next() would return None() if there was no next argument or a Some() object containing the value of our first element. This is a very common pattern in Rust for dealing with simple errors or functions that have an undetermined output for a certain input range.

Now that we have collected our command and arguments, we only need to add the .args field and we'll be able to handle arguments.
```
Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to run command");
```

### Running multiple commands ###
We rarely use the shell to run just one command, so in this section we'll make sure that the shell runs until we close the program.
In Rust, we can represent an infinite loop through:
```
loop { println!("doing this forever"); }
```
Adding our code into our infinite loop:
```
loop {
    prompt.update();
    prompt.print();
    stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    
    let mut args = input.trim().split_whitespace();
    let cmd = parts.next().unwrap();

    Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap();
}
```
This solution works decently, but since we're creating a child process to run the command, it doesn't wait for our previous process to end before going onto the next iteration. 
Lets make the parent wait on the child process before continuing:
```
let mut child = Command::new(cmd)
    .args(args)
    .spawn()
    .unwrap();

child.wait(); 
``` 
Furthermore, if we have any sort of error running the command, if we try to run a typo for example, our whole shell crashes. It would be better if we were to be able to get the shell to simply gracefully inform the user about the error.
Since the .spawn() method returns a Result<()>, we can check the state of the child process to see if there is any error we need to intercept.
```
let mut child = Command::new(cmd)
    .args(args)
    .spawn();

match child {
    Ok(mut child) => { child.wait(); },
    Err(error) => eprintln!("{}", error),
};
```
We're now left with a working basic shell.

## Handling built in shell commands ##
In this section, we're going to be handling the cd, clear, and exit commands. 
We can handle all the cases using the match keyword. Below, we handle the "exit" and default case for now.
```
match cmd
{
    "exit" => { todo!() };
    _ => {
        let mut child = Command::new(cmd)
            .args(args)
            .spawn();

        match child {
            Ok(mut child) => { child.wait(); },
            Err(error) => eprintln!("{}", error),
        };
    }
```
> Note: The todo!() macro simply indicates temporarily unfinished code, this way we can just the rest of the code without complaints.   

### Exit ###
Exit is straightforward to implmement, we'll simply print out our exit message and break out of the loop.
```
"exit" => {
    prompt.exit_message();
    break;
}
```

### Clear ###
For clear, we simply have to add the case and reuse our clear code from the beginning of our code.
```
"clear" => {  
    crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).expect("Failed to clear terminal");
    crossterm::execute!(stdout(), cursor::MoveTo(0, 0)).expect("Failed to move cursor to top");
},
```
### cd ###
The change of direction command is going to take a bit more thought. Here, wer're going to cover the "cd" (no args), "cd {dir}", and "cd -" cases.

First, lets tackle the "cd" and "cd {dir}" cases. These two cases are the easiest to implement since we don't have to consider any previous states of the system. 
We can look at the next element ( our directory argument in this case ) without moving the iterator through the peek() function.
```
let new_dir = args.peekable().peek().unwrap();
```
> Note that we have to make the iterator "peekable" before we are able to peek().
Now that we can check the new directory, we need to check if there is a new directory at all in the first place.
We could check if new_dir is None(), but Rust offers a simpler solution that does the same thing: map_or().
The map_or() method will return a default value if there is no contained value (peek() == None()), or apply a function to the contained function.
So, we while we could write
```
if args.peekable.peek().is_some()       // if not "None()"
{
    new_dir = * args.peekable().peek().unwrap();
}
else
{
    new_dir = "/";
}
```
we end up writing the equivalent
```
let new_dir = args.peekable().peek().map_or("/", |x| *x);
```

Now that we have our directory, we have to make a new Path() object and use it to set the current directory using the env::set_current_dir() method.
```
let new_path = Path::new(new_dir);
env::set_current_dir(&new_path); 
```
Since env::set_current_dir returns a Result<()> object, we can use the same strategy as we did earlier with the command execution to make sure we alert the user gracefully if they gave us an invalid directory.
```
if let Err(error) = env::set_current_dir(&new_path) { eprint!("{}", error); }
```

Finally, we need to handle the "cd -" case. First, we need to create a prev_path variable to keep track of our previous directory. We can declare it before our loop.
```
let mut prev_path : String = prompt.path.clone()
                                        .into_os_string()
                                        .into_string()
                                        .unwrap();
```
> Note that we're not just copying over the value and type casting it, we're "cloning" it. Because of Rust's borrowing system, we'd be borrowing the ownership of the prompt.path buffer and changing it. This would be equivalent to copying a reference in C++. We only want the value, so we're going to "clone" it so we have a variable exactly like our prompt.path variable that we can work with.   

Within our cd case handler, lets set the new path according to whether our new directory is "-" or not, and lets update the prev_path at the very end.
```
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
},
```
> Note we have to make a new variable, prev_dir, in order to create the new path with the previous directory. This goes back to the borrow/ownership system in Rust. The path constructor requires full ownership of the variable, something we cannot offer it if we give it the prev_path variable. So, again, we clone our prev_path so we can use it to create a new path instance.    

## Handling Output Redirection and Piping ##
To finish up our shell, we're going to add output redirection and piping.

### Output redirection ###
Lets first try and see if we need to redirect our output. Lets create a vector from our iterator and find the position of the '>" character through the .position character.
```
let args_vec = args.clone().collect::<Vec<&str>>();
let output_position = args.position(|x| x == ">");
```
Unfortunately, the .position() method moved our iterator. So, lets create a new iterator to represent our new-found arguments based on the args_vec and our ">" character position / existance.
```
let args_vec = args.clone().collect::<Vec<&str>>();
let output_position = args.position(|x| x == ">");
let has_output = (output_position != None) && (output_position.unwrap() < args_vec.len()); 

let args_it = if has_output { args_vec[0.. output_position.unwrap()].iter() } else { args_vec.iter() };
```
The .position argument returns an Option objet to us, so we can use that option to determine if we have a given output / output file.
Afterwards, we can use the boolean we got to determine if we're going to need an iterator of the entire arguments vector, or just a sub-array of our vector.

Now that we have all of that settled, we can move on to our output redirection.

### Piping ###

## Sources ##
rust documentation  
rust by example  
chrono documentation  
colored documentation    
[Crossterm Documentation](https://docs.rs/crossterm/latest/crossterm/)  
path documentation  
[My Starting Point](https://www.joshmcguigan.com/blog/build-your-own-shell-rust/)  
