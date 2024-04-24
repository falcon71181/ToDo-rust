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
            "done" => println!("Done command"),
            "sort" => println!("Sort all command"),
            "sort-done" => println!("Sort Done command"),
            "sort-undone" => println!("Sort UnDone command"),
            _ => println!("Help command"),
        }
    }
}
