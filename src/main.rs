use std::env;
use todo::ToDo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let todo = ToDo::new().expect("Unable to create todo struct");

    if !args.is_empty() {
        let command = &args[0];

        match &command[..] {
            "list" | "list-all" => todo.list(None),
            "list-done" => todo.list(Some(1)),
            "list-undone" => todo.list(Some(0)),
            "add" => todo.add(&args[1..]),
            "rm" => todo.rm(&args[1..]),
            "rm-all" | "reset" => todo.rm_all(),
            "done" => todo.done_undone(&args[1..], 1),
            "undone" => todo.done_undone(&args[1..], 0),
            "sort" | "sort-asc" => todo.sort(0, None),
            "sort-dsc" => todo.sort(1, None),
            "sort-done" | "sort-done-asc" => todo.sort(0, Some(1)),
            "sort-done-dsc" => todo.sort(1, Some(1)),
            "sort-undone" | "sort-undone-asc" => todo.sort(0, Some(0)),
            "sort-undone-dsc" => todo.sort(1, Some(0)),
            _ => todo.help(),
        }
    } else {
        todo.help();
    }
}
