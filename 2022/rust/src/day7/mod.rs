use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::utils;

type FileSystemItemHandle = Rc<RefCell<FileSystemItem>>;

#[derive(Clone)]
struct FileSystemItem {
    name: String,
    size: i32,
    children: HashMap<String, FileSystemItemHandle>,
    parent: Option<FileSystemItemHandle>,
}

impl FileSystemItem {
    fn is_directory(&self) -> bool {
        self.name != "/" && self.size == 0
    }
}

impl fmt::Debug for FileSystemItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileSystemItem")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug)]
enum Command {
    GoIntoDirectory(String),
    GoUpDirectory,
    List,
}

pub fn part_1() {
    let file_system = build_file_system();
    let directories = get_directories(file_system);
    let sum = directories
        .iter()
        .map(get_size_of_contents)
        .filter(|&s| s <= 100_000)
        .sum::<i32>();
    println!("Day 7 part 1 solution: {}", sum);
}

pub fn part_2() {
    let file_system = build_file_system();
    let total_size: i32 = get_size_of_contents(&file_system);

    let directories = get_directories(file_system);
    let mut sizes: Vec<i32> = directories.iter().map(get_size_of_contents).collect();
    sizes.sort();

    let target = 30_000_000;
    let delta = 70_000_000 - total_size;

    let smallest_to_delete = sizes
        .iter()
        .find(|&s| s + delta >= target)
        .expect("No solution");
    println!("Day 7 part 2 solution: {}", smallest_to_delete);
}

fn build_file_system() -> FileSystemItem {
    let file_system = Rc::new(RefCell::new(FileSystemItem {
        name: String::from("/"),
        size: 0,
        children: HashMap::new(),
        parent: None,
    }));
    let mut current_directory = file_system.clone();

    if let Ok(lines) = utils::read_lines("./data/day7.txt") {
        for line in lines.flatten() {
            if is_command(&line) {
                match parse_command(&line) {
                    Command::GoIntoDirectory(directory_name) => {
                        if directory_name == "/" {
                            continue;
                        }
                        let next_current_directory = current_directory
                            .borrow_mut()
                            .children
                            .entry(directory_name.to_owned())
                            .or_insert(Rc::new(RefCell::new(FileSystemItem {
                                name: directory_name,
                                size: 0,
                                children: HashMap::new(),
                                parent: Some(current_directory.clone()),
                            })))
                            .clone();
                        current_directory = next_current_directory;
                    }
                    Command::GoUpDirectory => {
                        let next_current_directory = current_directory.borrow().parent.clone();
                        current_directory = next_current_directory
                            .expect("Cannot go up a directory if already at the root");
                    }
                    Command::List => {
                        continue;
                    }
                }
            } else if is_directory(&line) {
                let directory_data = line.split(' ').collect::<Vec<&str>>();
                if directory_data.len() != 2 {
                    panic!("Invalid directory data: {}", line);
                }
                let directory_name = directory_data[1];
                current_directory
                    .borrow_mut()
                    .children
                    .entry(directory_name.to_string())
                    .or_insert(Rc::new(RefCell::new(FileSystemItem {
                        name: directory_name.to_string(),
                        size: 0,
                        children: HashMap::new(),
                        parent: Some(current_directory.clone()),
                    })));
            } else {
                let file_data = line.split(' ').collect::<Vec<&str>>();
                if file_data.len() != 2 {
                    panic!("Invalid file data: {}", line);
                }
                let file_size = file_data[0].parse::<i32>().expect("Invalid file size");
                let file_name = file_data[1];
                current_directory
                    .borrow_mut()
                    .children
                    .entry(file_name.to_string())
                    .or_insert(Rc::new(RefCell::new(FileSystemItem {
                        name: file_name.to_string(),
                        size: file_size,
                        children: HashMap::new(),
                        parent: Some(current_directory.clone()),
                    })));
            }
        }
    }

    let content = file_system.borrow();
    content.clone()
}

fn get_directories(file_system_item: FileSystemItem) -> Vec<FileSystemItem> {
    fn get_directories_aux(
        file_system_item: FileSystemItem,
        directories: &mut Vec<FileSystemItem>,
    ) -> Vec<FileSystemItem> {
        if file_system_item.is_directory() {
            directories.push(file_system_item.clone());
        }
        for child in file_system_item.children.values() {
            get_directories_aux(child.borrow().clone(), directories);
        }
        directories.to_vec()
    }

    let mut out: Vec<FileSystemItem> = vec![];
    get_directories_aux(file_system_item.clone(), &mut out)
}

fn get_size_of_contents(file_system_item: &FileSystemItem) -> i32 {
    let mut size = file_system_item.size;
    for child in file_system_item.children.values() {
        size += get_size_of_contents(&child.borrow().clone());
    }
    size
}

fn is_command(line: &str) -> bool {
    line.starts_with('$')
}

fn is_directory(line: &str) -> bool {
    line.starts_with("dir")
}

fn parse_command(line: &str) -> Command {
    let command = &line[2..];
    if command.starts_with("cd") {
        let directory_name = &command[3..];
        if directory_name == ".." {
            Command::GoUpDirectory
        } else {
            Command::GoIntoDirectory(String::from(directory_name))
        }
    } else if command == "ls" {
        Command::List
    } else {
        panic!("Unknown command: {}", command);
    }
}
