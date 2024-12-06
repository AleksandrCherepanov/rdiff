use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use crate::cli::Cli;

pub struct Flags {
    pub compared: bool,
    pub postfix: bool,
    pub delete: bool,
}

impl Flags {
    pub fn from_cli(cli: &Cli) -> Flags {
        Flags {
            compared: cli.compared,
            postfix: cli.postfix,
            delete: cli.delete,
        }
    }
}

pub fn parse_path(path: &str) -> PathBuf {
    let mut dir = PathBuf::new();
    dir.push(path);

    if !dir.is_dir() {
        println!("{} is not a directory", dir.display());
        exit(1);
    }

    dir.canonicalize().unwrap()
}

pub fn dir_compare(
    src: &HashMap<String, bool>,
    dst: &HashMap<String, bool>,
    flags: &Flags
) -> (usize, Vec<String>) {
    let mut diff = 0;
    let mut compared: Vec<String> = vec![];

    for (dst_path, _) in dst {
        let file_name = dst_path.split('/').last().unwrap();
        if !src.contains_key(file_name) {
            diff += 1;
        } else {
            compared.push(dst_path.to_string());
            if flags.delete {
                fs::remove_file(dst_path).unwrap();
                continue;
            }

            if flags.postfix {
                let new_name = dst_path.clone() + "_deleted";
                fs::rename(dst_path, new_name).unwrap();
                continue;
            }
        }
    }

    (diff, compared)
}

pub struct Traverser<'a> {
    exceptions: &'a Vec<&'a str>,
    use_file_path: bool,
}

impl<'a> Traverser<'a> {
    pub fn new(exceptions: &'a Vec<&str>, use_file_path: bool) -> Traverser<'a> {
        Traverser {
            exceptions,
            use_file_path,
        }
    }

    pub fn dir_traverse(&self, path: &str, result: &mut HashMap<String, bool>) -> usize {
        for e in self.exceptions {
            if path.contains(e) {
                return 0;
            }
        }

        let mut count = 0;
        let dir_entries = fs::read_dir(path).unwrap();
        for entry in dir_entries {
            let unwrapped = &entry.unwrap();
            if unwrapped.metadata().unwrap().is_dir() {
                count += self.dir_traverse(unwrapped.path().to_str().unwrap(), result);
            } else {
                if self.use_file_path {
                    result.insert(unwrapped.path().to_str().unwrap().to_string(), false);
                } else {
                    result.insert(unwrapped.file_name().to_str().unwrap().to_string(), false);
                }
                count += 1;
            }
        }

        count
    }
}
