# Intro #
# Creating Prompt.cpp #
## Defining the struct ##
Defining a struct in rust is similar to C++, except that in rust, structs are private by default, so we need to declare it as public using the "pub" keyword.
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
Constructors in rust are made by creating a public function called new(), which returns type "Self".
We can innitialize our variables here. 
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
## Working with environment variables in rust ##
The rust standard library includes an enviroment module that allows us manipulate and inspect the enviroment variables of our machine. We will use this module to get our user and path.
The enviornment module comes with a built in current_dir() function, which allows us to get the directory as PathBuf object (similar to a String).
For the user, however, we need to use the "var" function. The environment variables in rust are stored as key value pairs. So the var function takes it a key (the name of the environemnt variable), and returns to us the value of our environment variable.  

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

## Time in Rust ##
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
## Cleaning up our constructor: dealing with Result<> objects ## 
If you try to compile this now, you'll probably get a warning that we're not doing anything with the std::result our functions are returning. An std::result is a type used for discovering and propogating erros. If your function returns a result, you can utalize the ? operator to handle and propogate the result. For example:
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
