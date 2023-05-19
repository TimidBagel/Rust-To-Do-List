use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{stdin, BufReader, Result, Write},
};

// Represents a task with its name, description, due date, and completion status.
#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    desc: String,
    due_date: String,
    done: bool,
}

// Main program procedure
fn main() {
    // Initializes vector of tasks, and copies data from `tasks.json` if file exists.
    let mut tasks: Vec<Task> = match read_tasks() {
        Ok(tasks) => {
            println!("loaded tasks from `tasks.json`");
            tasks
        }
        Err(_) => {
            println!("`tasks.json` is empty, no tasks loaded.");
            vec![]
        }
    };

    // Runtime loop
    loop {
        println!(
            "\nWhat would you like to? (eg: '1')\n1. View tasks\n2. Add a task\n3. Complete task\n4. Delete task"
        );
        let resp = read_line();

        match resp.as_str() {
            "1" => view_tasks(&mut tasks),

            "2" => {
                let new_task = create_task();
                add_task(&mut tasks, new_task);
            }

            "3" => {
                view_tasks(&tasks);

                println!("\nSelect a task to mark as complete:");

                if let Some(index) = read_index_input(&tasks) {
                    complete_task(&mut tasks, index);
                } else {
                    continue;
                }
            }

            "4" => {
                view_tasks(&tasks);

                println!("\nSelect a task to delete:");

                if let Some(index) = read_index_input(&tasks) {
                    delete_task(&mut tasks, index);
                } else {
                    continue;
                }
            }

            // If other input, save task vector to `tasks.json` and exit program.
            _ => {
                println!("\nRemoving completed tasks...");
                remove_complete_tasks(&mut tasks);
                println!("Completed tasks removed");

                println!("\nSerializing data...");
                let serialized_tasks = serde_json::to_string(&tasks).expect("Serialization failed");
                println!("Data serialized");

                println!("\nCreating file...");
                let mut file = File::create("tasks.json").expect("File creation failed");
                println!("File created");

                println!("\nSaving work...");
                file.write_all(serialized_tasks.as_bytes())
                    .expect("Writing to file failed");
                println!("Work saved");

                println!("\nExiting successfully");
                break;
            }
        }
    }
}

// Removes all completed tasks from tasks vector.
fn remove_complete_tasks(tasks: &mut Vec<Task>){
    tasks.retain(|task| !task.done);
}

// Reads tasks from the "tasks.json" file and returns them as a vector.
fn read_tasks() -> Result<Vec<Task>> {
    let file = File::open("tasks.json")?;
    let reader = BufReader::new(file);
    let tasks: Vec<Task> = serde_json::from_reader(reader)?;
    Ok(tasks)
}

// Adds a new task to the vector of tasks.
fn add_task(tasks: &mut Vec<Task>, new_task: Task) {
    tasks.push(new_task);
}

// Creates a new task by prompting the user for its name, description, and due date.
fn create_task() -> Task {
    println!("\nEnter a name for 'new_task':");
    let name: String = read_line();

    println!("\nEnter a short description for '{name}':");
    let desc: String = read_line();

    println!("\nEnter a due date for '{name}':");
    let due_date: String = read_line();

    let done: bool = false;

    Task {
        name,
        desc,
        due_date,
        done,
    }
}
// Reads user input for the task index and returns it as an `Option<usize>`.
// If the input is invalid or out of range, it returns `None`.
fn read_index_input(tasks: &[Task]) -> Option<usize> {
    let index: usize = match read_line().parse::<usize>() {
        Ok(num) => num - 1,
        Err(_) => {
            println!("\nInput must be a valid index!");
            return None;
        }
    };

    if index >= tasks.len() {
        println!("\nInvalid task index!");
        return None;
    } else {
        return Some(index);
    }
}

// Reads a line of input from the user.
fn read_line() -> String {
    let mut input: String = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }

    input
}

// Displays the list of tasks to the user.
fn view_tasks(tasks: &Vec<Task>) {
    println!(); // newline

    for (i, task) in tasks.iter().enumerate() {
        println!(
            "\t{}. {} : {} : Done - {}\n\t{}\n",
            i + 1,
            task.name,
            task.due_date,
            task.done,
            task.desc
        );
    }
}

// Marks a task as complete at the specified index.
fn complete_task(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.done = true;
    } else {
        println!("\nInvalid task index!");
    }
}

// Deletes a task at the specified index.
fn delete_task(tasks: &mut Vec<Task>, index: usize) {
    if let Some(_) = tasks.get(index) {
        tasks.remove(index);
    } else {
        println!("\nInvalid task index!");
    }
}
