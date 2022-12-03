# Intro #
# Creating Prompt.cpp #
## Definint the struct ##
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
Within this implementation, we're going to add the print, update, and exit message.

Now that we have the general structure of our class, we need to create its constructor.
Constructors in rust are made by creating a public function called new(), which returns type "Self".
We can innitialize our variables here. 

## Working with environment variables in rust ##
The rust standard library includes an enviroment module that allows us manipulate and inspect the enviroment variables of our machine. 
The method "var" allows us to fetch the environment variable key from the current process. 
