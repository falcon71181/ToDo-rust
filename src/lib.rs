use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use xdg::BaseDirectories;

pub struct ToDo {
    pub todo: Vec<String>,
    pub todo_path: PathBuf,
    pub config_path: PathBuf,
}

impl ToDo {
    pub fn new() -> Result<Self, String> {
        let xdg_dir = BaseDirectories::with_prefix("ToDo").expect("Failed to get XDG directories");

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
        let mut todo_file = File::open(&todo_path).expect("Failed to open todo.lst");
        let mut todo_lst = String::new();
        todo_file
            .read_to_string(&mut todo_lst)
            .expect("Failed to read todo.lst");

        let todo: Vec<String> = todo_lst.lines().map(String::from).collect();
        println!("{:?}", todo);

        Ok(Self {
            todo,
            todo_path,
            config_path,
        })
    }
}
