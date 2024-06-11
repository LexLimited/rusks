const HELP_STRING: &str =
"
    ========== RUSKS HELP ==========

    List of commands:

    1)  help - displays help
        syntax: rusks help

    2)  new - add a new task
        syntax: rusks new <task_name> <option_list>

    3)  remove - remove a task
        syntax: rusks remove <task_id>
                rusks remove --name <task_name>

    4)  edit - edit an existing task
        syntax: rusks edit <task_id>
                rusks edit --name <task_name>
";

pub fn show_help() {
    println!("{}", HELP_STRING)
}
