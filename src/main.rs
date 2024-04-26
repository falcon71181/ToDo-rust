use std::env;
use todo::ToDo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let todo = ToDo::new().expect("Unable to create todo struct");

    if !args.is_empty() {
        let command = &args[0];

        match &command[..] {
            "list" | "list-all" => todo.list(),
            "list-done" => println!("List done command"),
            "list-undone" => println!("List undone command"),
            "add" => todo.add(&args[1..]),
            "rm" => todo.rm(&args[1..]),
            "rm-all" => todo.rm_all(),
            "done" => todo.done_undone(&args[1..], 1),
            "undone" => todo.done_undone(&args[1..], 0),
            "sort" | "sort-asc" => todo.sort(0),
            "sort-dsc" => todo.sort(1),
            "sort-done" | "sort-done-asc" => println!("Sort Done command"),
            "sort-done-dsc" => println!("Sort Done command dsc"),
            "sort-undone" | "sort-undone-asc" => println!("Sort UnDone command"),
            "sort-undone-dsc" => println!("Sort UnDone command dsc"),
            _ => println!("Help command"),
        }
    }
}
