use std::io;

fn add_commands(mut command_vec: Vec<&str>) -> Vec<&str> {
    command_vec.push("1. Add a new ToDo List.");
    command_vec.push("2. Show my ToDo Lists.");
    command_vec.push("3. Add Item to a ToDo List.");
    command_vec.push("4. Delete item from a ToDo List.");
    command_vec.push("5. Abort");

    return command_vec;
}

fn print_commands(list_of_commands_to_print: Vec<&str>) {
    for command in &list_of_commands_to_print {
        println!("{}", command);
    }
}

fn main() {
    let mut todos: Vec<String> = Vec::new();
    let mut todo = &String::new();

    let mut commands = Vec::new();
    commands = add_commands(commands);
    print_commands(commands);

    // io::stdin().read_line(&mut input.clone()).expect("error");

    // todos.push(input);
}
