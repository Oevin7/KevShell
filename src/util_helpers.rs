use std::{env, io};
use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::io::Write;
use std::path::PathBuf;

fn get_filenames(dir_entry : &DirEntry) -> Result<String, OsString> {
    //Creates a file name variable, to get all the file names and print them cleanly.
    //If a file name has an invalid UTF-8 character, it gets printed as an OsString.
    let filenames = dir_entry.file_name().into_string()?;
    Ok(filenames)
}

pub fn default_list(directory: PathBuf) {
    //Gets all the paths from the current directory and again returns if it fails.
    let paths = match read_dir(&directory) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Could not access the current directory: {e:?}");
            return;
        }
    };

    //Loops through the files in the path and then prints their names, returning if it fails
    for files in paths {
        let dir_files = match files {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Could not read the files in the directory: {e:?}");
                return;
            }
        };

        let filenames = match get_filenames(&dir_files) {
            Ok(name) => name,
            Err(os_str_name) => {
                eprintln!("This file contains invalid UTF-8 characters: {os_str_name:?}");
                continue;
            }
        };

        //Skips hidden files.
        if filenames.starts_with('.') {
            continue;
        } else {
            //Prints to and flushes stdout
            print!("{}  ", filenames);
            io::stdout().flush().unwrap();
        }
    }
}

pub fn list_with_flags(directroy : PathBuf, flags : Vec<String>) {
    //Gets all the paths from the current directory and again returns if it fails.
    let paths = match read_dir(&directroy) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Could not access the current directory: {e:?}");
            return;
        }
    };

    //Loops through the files in the path and then prints their names, returning if it fails
    for files in paths {
        let dir_files = match files {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Could not read the files in the directory: {e:?}");
                return;
            }
        };

        let filenames = match get_filenames(&dir_files) {
            Ok(name) => name,
            Err(os_str_name) => {
                eprintln!("This file contains invalid UTF-8 characters: {os_str_name:?}");
                continue;
            }
        };

        let metadata = match dir_files.metadata() {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("Could not get file metadata: {e:?}");
                continue;
            }
        };

        flags.iter().map(|flag| {
            match flag.as_str() {
                "-a" | "--all" => {
                    //Prints to and flushes stdout
                    print!("{}  ", filenames);
                    io::stdout().flush().unwrap();
                }
                _ => {
                    eprintln!("Not valid bruv");
                    return;
                },
            }
        }).next();

    }
}