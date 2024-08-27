use std::{env, io};
use crate::util_helpers::{default_list, list_with_flags};

pub fn print(argument : String) {
    println!("{argument}");
}

///Similar to the List command for UNIX/Linux
pub fn list(argument : Vec<String>, flags : Vec<String>) {
    //If list is run without any arguments, it will print the working directory
    if argument.is_empty() && flags.is_empty() {

        //Gets the current directory, and returns if it fails.
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Could not access the directory: {e:?}");
                return;
            }
        };

        default_list(current_dir)
    } else if argument.is_empty() {
        //Gets the current directory, and returns if it fails.
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Could not access the directory: {e:?}");
                return;
            }
        };

        list_with_flags(current_dir, flags);
    }

    print!("\n");
}