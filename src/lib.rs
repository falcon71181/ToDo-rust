use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use xdg::BaseDirectories;

// TODO: Add backup option (make use of todo/config.ini)
pub struct ToDo {
    pub todo: Vec<String>,
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

        // Read contents of todo.lst
        // BUG: OpenOptions::new() not working here
        let todo_file = File::open(&todo_path).expect("Failed to open todo.lst.");
        let buffer_reader = BufReader::new(&todo_file);
        let mut todo: Vec<String> = vec![];

        for line in buffer_reader.lines() {
            if line.is_ok() {
                todo.push(line.unwrap());
            }
        }

        Ok(Self {
            todo,
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

            // Add \n to every task
            let line: String = format!("{}\n", arg);

            buffer_writter
                .write_all(line.as_bytes())
                .expect("Unable to write task in todo.lst.");
            // TODO: use colored to notify user
        }
    }

    // List all tasks in todo
    pub fn list(&self) {
        // Open todo.lst to read
        // BUG: OpenOptions::new() not working here
        let todo_file = File::open(&self.todo_path).expect("Unable to open todo.lst.");
        // Read buffer
        let buffer_reader = BufReader::new(&todo_file);

        let mut index = 1;
        for line in buffer_reader.lines() {
            if line.is_ok() {
                println!("{}: {}", index, &line.unwrap());
                index += 1;
            }
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
            if !del_line_no.contains(&index) {
                new_list.push(line.unwrap());
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
        // TODO: print removed tasks using colored
        for line in new_list {
            // Add \n to every task
            let line: String = format!("{}\n", line);
            buffer_writter
                .write_all(line.as_bytes())
                .expect("Unable to write to todo.lst.");
        }
    }

    // Remove all tasks from todo.lst
    pub fn rm_all(&self) {
        let mut confirmation = String::new();
        println!("Do you want to remove all tasks from todo ? (y/Y/yes/Yes/YES)");
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
                .write_all("".as_bytes())
                .expect("Unable to write to todo.lst.");
            // TODO: use colored to notify user
            println!("All task are removed from todo.lst.");
        } else {
            exit(1);
        }
    }

    // sort all task asc
    pub fn sort(&self, via: u8) {
        let todo_file = File::open(&self.todo_path).expect("Unable to open todo.lst.");

        // Read Buffer
        let buffer_reader = BufReader::new(&todo_file);
        let mut todo_lst: Vec<String> = vec![];

        for line in buffer_reader.lines() {
            if line.is_ok() {
                todo_lst.push(line.unwrap());
            }
        }
        // sort todo_lst
        if via == 0 {
            todo_lst.sort();
        } else if via == 1 {
            todo_lst.sort_by(|a, b| b.cmp(a));
        }
        let mut index = 1;
        for task in todo_lst {
            println!("{}: {}", index, task);
            index += 1;
        }
    }
}
