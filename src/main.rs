use std::{env, fs, process};
#[doc(inline)]
pub use std;
use std::path::{Path, PathBuf};
use regex;
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct FormattedArgs {
    file: String,
    file_as_regex: Regex,
    dirs: PathBuf,
    exclude_dirs: Vec<String>,
}

fn format_path(path: String) -> String {
    path.replace("~",std::env::home_dir().unwrap().to_str().unwrap())
}

fn get_formatted_args(mut args: env::Args) -> FormattedArgs {
    args.next();
    let file = match args.next() {
        None => {
            eprintln!("Please provide file or wildcard that I will be looking for :P ");
            process::exit(0);
        }
        Some(n) => n
    };
    let dirs = match args.next() {
        None => PathBuf::from(format_path(String::from("."))),
        Some(n) => PathBuf::from(format_path(n))
    };
    let mut exclude_dirs = vec![];
    let exclude_dirs_as_string = match args.next() {
        None => String::from(""),
        Some(n) => n
    };

    for exclude_dir in exclude_dirs_as_string.split(",").by_ref().into_iter() {
        if &exclude_dir != &"" {
            exclude_dirs.push(exclude_dir.to_owned());
        }
    }
    FormattedArgs {
        file_as_regex: get_file_name_as_regex(&file),
        file,
        dirs,
        exclude_dirs,
    }
}

fn check_for_file(file: &String,file_as_regex: &Regex, path: &Path, exclude_dirs: &Vec<String>) {
    if let Ok(entries) = fs::read_dir(path) {
        entries.par_bridge().for_each(|entry| {
            if let Ok(entry) = entry {
                let metadata = match entry.metadata() {
                    Ok(n) => n,
                    Err(_) => return
                };

                if metadata.is_dir() && (exclude_dirs.len() == 0 || !exclude_dirs.iter().any(|p| { entry.path().starts_with(format!("{}/{}", path.display().to_string(), p)) })) {
                    check_for_file(&file, &file_as_regex,&entry.path(), &exclude_dirs);
                }
                let file_name = entry.file_name().into_string().unwrap();
                if file_as_regex.is_match(&file_name) {
                   println!("{}",entry.path().display().to_string());
                }
            }
        });
    }
}
fn get_file_name_as_regex(file_name: &String) -> Regex  {
    let mut temp_regex = String::from("^");
    for letter in file_name.chars() {
        if letter == '*' {
            temp_regex.push_str("[A-Za-z0-9]*");
        }
            else if letter == '.' {
                temp_regex.push_str("\\.")
            }
        else {
            temp_regex.push(letter);
        }
    }
    temp_regex.push('$');
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
    println!("{:#?}", args);
    check_for_file(&args.file,&args.file_as_regex, &args.dirs, &args.exclude_dirs);
}
