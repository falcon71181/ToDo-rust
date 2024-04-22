use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        let command = &args[0];

        match &command[..] {
            "list" | "list-all" => println!("List all command"),
            "list-done" => println!("List done command"),
            "list-undone" => println!("List undone command"),
            "add" => println!("Add command"),
            "rm" => println!("Remove command"),
            "rm-all" => println!("Remove all command"),
            "done" => println!("Done command"),
            "sort" => println!("Sort all command"),
            "sort-done" => println!("Sort Done command"),
            "sort-undone" => println!("Sort UnDone command"),
            _ => println!("Help command"),
        }
    }
}
