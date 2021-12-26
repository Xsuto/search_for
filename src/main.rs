use std::{env, fs, process};
#[doc(inline)]
pub use std;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use regex;
use regex::Regex;

#[derive(Debug)]
struct FormattedArgs {
    files: String,
    files_as_regex: Regex,
    searched_directory: PathBuf,
    excluded_dirs: Vec<Regex>,
}

fn format_path(path: String) -> String {
    path.replace("~", std::env::home_dir().unwrap().to_str().unwrap())
}

fn get_formatted_args(mut args: env::Args) -> FormattedArgs {
    args.next();
    let files = match args.next() {
        None => {
            eprintln!("Please provide file or wildcard that I will be looking for :P ");
            process::exit(0);
        }
        Some(n) => n
    };
    let searched_directory = match args.next() {
        None => PathBuf::from(format_path(String::from("."))),
        Some(n) => PathBuf::from(format_path(n))
    };
    let mut excluded_dirs = vec![];
    let excluded_dirs_as_string = match args.next() {
        None => String::from(""),
        Some(n) => n
    };

    for excluded_dir in excluded_dirs_as_string.split(",").by_ref().into_iter() {
        if &excluded_dir != &"" {
            excluded_dirs.push(Regex::new(excluded_dir).unwrap());
        }
    }
    FormattedArgs {
        files_as_regex: get_files_name_as_regex(&files),
        files,
        searched_directory,
        excluded_dirs,
    }
}

fn check_for_file(files: &String, files_as_regex: &Regex, searched_directory: &Path, excluded_dirs: &Vec<Regex>) {
    if let Ok(entries) = fs::read_dir(searched_directory) {
        entries.par_bridge().for_each(|entry| {
            if let Ok(entry) = entry {
                let metadata = match entry.metadata() {
                    Ok(n) => n,
                    Err(_) => return
                };
                if metadata.is_dir() && (excluded_dirs.len() == 0 || !excluded_dirs.iter().any(|p| { p.is_match(entry.path().to_str().unwrap()) })) {
                    check_for_file(&files, &files_as_regex, &entry.path(), &excluded_dirs);
                }
                let file_name = entry.file_name().into_string().unwrap();
                if files_as_regex.is_match(&file_name) {
                    println!("{}", entry.path().display().to_string());
                }
            }
        });
    }
}

fn get_files_name_as_regex(file_name: &String) -> Regex {
    let mut temp_regex = String::from("^(");
    for letter in file_name.chars() {
        if letter == '*' {
            temp_regex.push_str("[A-Za-z0-9]*");
        } else if letter == '.' {
            temp_regex.push_str("\\.");
        }
            else if letter == ',' {
                temp_regex.push_str(")?(");
            }
         else {
            temp_regex.push(letter);
        }
    }
    temp_regex.push_str(")?$");
    regex::Regex::new(&temp_regex).expect("Cannot convert file_name to patter")
}

fn setup() {
    let mut cpu_cores = num_cpus::get_physical();
    if cpu_cores >= 4 { cpu_cores /= 1 };
    rayon::ThreadPoolBuilder::new().num_threads(cpu_cores).build_global().unwrap();
}

fn main() {
    setup();
    let args = get_formatted_args(env::args());
    // println!("{:#?}", args);
    check_for_file(&args.files, &args.files_as_regex, &args.searched_directory, &args.excluded_dirs);
}
