use ansi_term::Colour::{Blue, Cyan, Green, Purple, Red, Yellow};
use ansi_term::Style;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use xdg::BaseDirectories;

// TODO: Add backup option (make use of todo/config.ini)
pub struct ToDo {
    pub todo_path: PathBuf,
    pub config_path: PathBuf,
}

// TODO: make add to accept line-type tasks instead of one word
// TODO: Add date and time
// TODO: Add sort via date-time
// TODO: make use of macros for open file with read or write permissions

impl ToDo {
    // TODO: make todo/config.ini usable
    // TODO: Add config options to todo/config
    pub fn new() -> Result<Self, String> {
        let xdg_dir = BaseDirectories::with_prefix("ToDo").expect("Failed to get XDG directories.");

        let config_path = xdg_dir
            .place_config_file("config.ini")
            .expect("Unable to create Config file.");

        // TODO: create a separate function to do this work
        if !Path::new(&config_path.as_path()).exists() {
            File::create(&config_path).expect("Failed to create Config file.");
        }

        let todo_path = xdg_dir
            .place_config_file("todo.lst")
            .expect("Unable to create ToDo lst file.");

        // TODO: create a separate function to do this work
        if !Path::new(&todo_path.as_path()).exists() {
            File::create(&todo_path).expect("Failed to create ToDo lst file.");
        }

        Ok(Self {
            todo_path,
            config_path,
        })
    }

    // Add new task in todo
    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("Add option needs atleast 1 argument.");
            exit(1);
        }

        // Write contents in todo.lst
        let todo_file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Unable to open todo.lst.");
        let mut buffer_writter = BufWriter::new(&todo_file);

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            // Remove one or more spaces and trim task
            let re_multiple_spaces = Regex::new(r"\s+").unwrap();
            let formated_task: String =
                re_multiple_spaces.replace_all(&arg.trim(), "_").to_string();
            // Add \n to every task
            let line: String = format!("{} 0\n", &formated_task);

            buffer_writter
                .write_all(&line.as_bytes())
                .expect("Unable to write task in todo.lst.");

            println!("{}: {}", Purple.bold().paint("Added"), arg.trim());
        }
    }

    // List all tasks in todo
    pub fn list(&self, via_status: Option<u8>) {
        // Open todo.lst to read
        // BUG: OpenOptions::new() not working here
        let todo_file = File::open(&self.todo_path).expect("Unable to open todo.lst.");
        // Read buffer
        let buffer_reader = BufReader::new(&todo_file);

        let mut index = 1;
        for line in buffer_reader.lines() {
            if let Ok(line) = line {
                let task_details: Vec<&str> = line.split_whitespace().collect();
                // TODO: make function return Result<boolean> and make task_status boolean
                let task_status: u8 = task_details[1].parse::<u8>().unwrap_or(0);

                match via_status {
                    Some(via) => {
                        if via == 1 && task_status == 1 {
                            println!(
                                "{}: {}",
                                Yellow.bold().italic().paint(&index.to_string()),
                                Style::new().paint(task_details[0])
                            );
                        } else if via == 0 && task_status == 0 {
                            println!(
                                "{}: {}",
                                Blue.bold().italic().paint(&index.to_string()),
                                Style::new().paint(task_details[0])
                            );
                        }
                    }
                    None => {
                        if task_status == 1 {
                            println!(
                                "{}: {}",
                                Yellow.bold().italic().paint(&index.to_string()),
                                Style::new().strikethrough().italic().paint(task_details[0])
                            );
                        } else {
                            println!(
                                "{}: {}",
                                Blue.bold().italic().paint(&index.to_string()),
                                &task_details[0].to_string().replace("_", " ")
                            );
                        }
                    }
                }
                index += 1;
            }
        }
    }

    // Completed a task from todo.lst
    // NOTE: 1 - done
    // NOTE: 0 - undone
    pub fn done_undone(&self, args: &[String], status_todo: u8) {
        if args.is_empty() {
            eprintln!("done option needs atleast 1 argument.");
            exit(1);
        }
        let done_line_no: Vec<u64> = args[..].iter().map(|z| z.parse::<u64>().unwrap()).collect();

        // Open todo.lst to read
        // BUG: OpenOptions::new() not working here
        let mut todo_file = File::open(&self.todo_path).expect("Unable to open todo.lst.");
        // Read Buffer
        let buffer_reader = BufReader::new(&todo_file);

        let mut new_list: Vec<String> = Vec::new();

        let mut index: u64 = 1;
        for line in buffer_reader.lines() {
            let line = line.unwrap();
            if done_line_no.contains(&index) {
                let mut task_details: Vec<&str> = line.split_whitespace().collect();

                // Update the taak status
                task_details[1] = if status_todo == 1 { "1" } else { "0" };

                let updated_line: String = format!(
                    "{} {}",
                    task_details[0].to_string(),
                    task_details[1].to_string()
                );

                match status_todo {
                    1 => println!(
                        "{}: {}  : {}",
                        Purple.bold().italic().paint(&index.to_string()),
                        &task_details[0],
                        Green.bold().paint("Completed ")
                    ),
                    0 => println!(
                        "{}: {}  : {}",
                        Purple.bold().italic().paint(&index.to_string()),
                        &task_details[0],
                        Cyan.bold().paint("UnDone 󰚭")
                    ),
                    _ => println!("{}", Red.paint("Configuration is wrong.")),
                }

                new_list.push(updated_line);
            } else {
                new_list.push(line);
            }
            index += 1;
        }

        // rewritting new_list to todo.lst
        todo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Unable to open todo.lst.");
        // Write Buffer
        let mut buffer_writter = BufWriter::new(&todo_file);
        for line in new_list {
            // Add \n to every task
            let line: String = format!("{}\n", line);
            buffer_writter
                .write_all(&line.as_bytes())
                .expect("Unable to write to todo.lst.");
        }
    }

    // Remove a task from todo.lst
    pub fn rm(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("rm option needs atleast 1 argument.");
            exit(1);
        }

        let mut del_line_no: Vec<u64> =
            args[..].iter().map(|z| z.parse::<u64>().unwrap()).collect();
        del_line_no.sort();

        // Open todo.lst to read
        // BUG: OpenOptions::new() not working here
        let mut todo_file = File::open(&self.todo_path).expect("Unable to open todo.lst.");
        // Read Buffer
        let buffer_reader = BufReader::new(&todo_file);

        let mut new_list: Vec<String> = vec![];
        let mut index: u64 = 1;
        for line in buffer_reader.lines() {
            let task_details = line.unwrap_or("".to_string());
            if !del_line_no.contains(&index) {
                new_list.push(task_details);
            } else {
                let task = task_details.split_whitespace().nth(0).to_owned();
                println!(
                    "{} {}: {}",
                    Red.bold().paint("Removed"),
                    Purple.bold().italic().paint(&index.to_string()),
                    &task.unwrap_or("")
                );
            }
            index += 1;
        }

        // rewritting new_list to todo.lst
        todo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Unable to open todo.lst.");
        // Write Buffer
        let mut buffer_writter = BufWriter::new(&todo_file);
        for line in new_list {
            // Add \n to every task
            let line: String = format!("{}\n", line);
            buffer_writter
                .write_all(&line.as_bytes())
                .expect("Unable to write to todo.lst.");
        }
    }

    // Remove all tasks from todo.lst
    pub fn rm_all(&self) {
        let mut confirmation = String::new();
        println!(
            "{}: Do you want to remove all tasks from todo ? {}",
            Red.bold().paint("WARNING"),
            Blue.bold().paint("(y/Y/yes/Yes/YES)")
        );
        io::stdin()
            .read_line(&mut confirmation)
            .expect("Unable to take confirmation.");

        let confirm: Vec<String> = vec!["y".to_string(), "yes".to_string()];

        // Convert input to lowercase and remove whitespace
        let confirmation = confirmation.trim().to_lowercase();

        if confirm.iter().any(|z| z == &confirmation) {
            let todo_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&self.todo_path)
                .expect("Unable to open todo.lst.");

            let mut buffer_writer = BufWriter::new(&todo_file);
            buffer_writer
                .write_all(&"".as_bytes())
                .expect("Unable to write to todo.lst.");
            println!(
                "{}",
                Cyan.bold().paint("All task are removed from todo.lst.")
            );
        } else {
            exit(1);
        }
    }

    // sort all task asc
    // NOTE: via 0 - asc
    // NOTE: via 1 - dsc
    // NOTE: via_status 0 - undone
    // NOTE: via_status 1 - done
    pub fn sort(&self, via: u8, via_status: Option<u8>) {
        let todo_file = File::open(&self.todo_path).expect("Unable to open todo.lst.");

        // Read Buffer
        let buffer_reader = BufReader::new(&todo_file);
        let mut todo_lst: Vec<String> = vec![];

        let mut index = 1;
        for line in buffer_reader.lines() {
            if line.is_ok() {
                let formated_line = format!("{} {}", index, line.unwrap());
                todo_lst.push(formated_line);
            }
            index += 1;
        }
        // sort todo_lst
        match via {
            0u8 => {
                sort_by_key(&mut todo_lst, 1);
                display_sorted(&todo_lst, via_status);
            }
            1u8 => {
                sort_by_key(&mut todo_lst, 1);
                todo_lst.reverse();
                display_sorted(&todo_lst, via_status);
            }
            _ => println!("Configuration is incorrect."),
        }
    }

    pub fn help(&self) {
        println!("{}", USAGE_HELP);
    }
}

// Helper functions

// to display sorted List
// NOTE: via_status means done or undone | Some(0) - undone, Some(1) - done
fn display_sorted(todo_lst: &Vec<String>, via_status: Option<u8>) -> () {
    for task in todo_lst {
        // TODO: make function return Result<boolean> and make task_status boolean
        let task_details: Vec<&str> = task.split_whitespace().collect();
        let task_status: u8 = task_details[2].parse::<u8>().unwrap_or(0);
        match via_status {
            Some(via) => {
                if via == 1 && task_status == 1 {
                    println!(
                        "{}: {}",
                        task_details[0],
                        Style::new().paint(task_details[1])
                    );
                } else if via == 0 && task_status == 0 {
                    println!(
                        "{}: {}",
                        task_details[0],
                        Style::new().paint(task_details[1])
                    );
                }
            }
            None => {
                if task_status == 1 {
                    println!(
                        "{}: {}",
                        task_details[0],
                        Style::new().strikethrough().paint(task_details[1])
                    )
                } else if task_status == 0 {
                    println!(
                        "{}: {}",
                        task_details[0],
                        Style::new().paint(task_details[1])
                    )
                }
            }
        }
    }
}

// to sort by specific key
fn sort_by_key(todo_lst: &mut Vec<String>, key: usize) -> () {
    todo_lst.sort_by_key(|line| line.split_whitespace().nth(key).unwrap_or("").to_owned());
}

// Help Usage
const USAGE_HELP: &str = "Usage: todo [OPTIONS] [ARGUMENTS]
Todo is a blazingly fast CLI program written in Rust.

    - add [TASK/s]: Adds new task/s.
        Example: todo add 'do at least 10 dynamic programming questions.'
        Example: todo add task1 task2 task3
        Example: todo add 'mine task1', 'mine task2', 'mine task3'
    
    - list | list-all: Lists all tasks.
        Example: todo list
        Example: todo list-all
    
    - list-done: Lists all completed tasks.
        Example: todo list-done
    
    - list-undone: Lists all pending tasks.
        Example: todo list-undone
    
    - done [INDEX KEY]: Marks task as completed.
        Example: todo done 5 6
    
    - undone [INDEX KEY]: Marks task as pending.
        Example: todo undone 5 6
    
    - rm [INDEX KEY]: Removes a task.
        Example: todo rm 2 1 3
    
    - rm-all | reset: Removes all tasks.
        Example: todo rm-all
        Example: todo reset
    
    - sort: Sorts all tasks (default - ascending order).
        Example: todo sort
    
    - sort-asc: Sorts all tasks in ascending order.
        Example: todo sort-asc
    
    - sort-dsc: Sorts all tasks in descending order.
        Example: todo sort-dsc
    
    - sort-done: Sorts all completed tasks (default - ascending order).
        Example: todo sort-done
    
    - sort-done-asc: Sorts all completed tasks in ascending order.
        Example: todo sort-done-asc
    
    - sort-done-dsc: Sorts all completed tasks in descending order.
        Example: todo sort-done-dsc
    
    - sort-undone: Sorts all pending tasks (default - ascending order).
        Example: todo sort-undone
    
    - sort-undone-asc: Sorts all pending tasks in ascending order.
        Example: todo sort-undone-asc
    
    - sort-undone-dsc: Sorts all pending tasks in descending order.
        Example: todo sort-undone-dsc
    
Report any bugs or issues at: https://github.com/falcon71181/ToDo-rust";
