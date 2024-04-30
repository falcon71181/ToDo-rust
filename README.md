# ⚡ToDo_Rust⚡
<p align="center">
  <img src="https://skillicons.dev/icons?i=rust" />
  <br/>
</p>
<br/><br/>

---

## Overview

This is a simple command-line interface (CLI) todo program written in Rust.

## Installation

To install this program, you'll need Rust installed on your system. If you don't have it installed, you can get it from [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can build the program using Cargo:

```bash
cargo build --release
```

After successful compilation, you can copy the executable to a directory in your PATH, for example:

```bash
cp target/release/todo-rust /usr/local/bin/todo
```

## Usage

The following commands are available:

- `list` or `list-all`: Lists all the tasks.
- `list-done`: Lists only the completed tasks.
- `list-undone`: Lists only the tasks that are not completed.
- `add <task>`: Adds a new task.
- `rm <task_index>`: Removes the task at the specified index.
- `rm-all`: Removes all tasks.
- `done <task_index>`: Marks the task at the specified index as done.
- `undone <task_index>`: Marks the task at the specified index as undone.
- `sort` or `sort-asc`: Sorts the tasks in ascending order.
- `sort-dsc`: Sorts the tasks in descending order.
- `sort-done` or `sort-done-asc`: Sorts the completed tasks in ascending order.
- `sort-done-dsc`: Sorts the completed tasks in descending order.
- `sort-undone` or `sort-undone-asc`: Sorts the tasks that are not completed in ascending order.
- `sort-undone-dsc`: Sorts the tasks that are not completed in descending order.

## Future Improvements

- **Backup Option**: Implement a backup option, utilizing a configuration file (`todo/config.ini`).
- **Date and Time**: Add functionality to include date and time for tasks.
- **Sort by Date-Time**: Implement sorting functionality based on date and time.
- **Macros for File Operations**: Utilize macros for file operations, such as opening files with read or write permissions.
- **Config Options**: Add configuration options to `todo/config`.
- **Separate Function**: Create a separate function for managing configuration options.
- **Use Colored Output**: Utilize colored output for better notifications.
  #### In the future, additional features such as support handling more command-line arguments may be added.

---
