use std::io;

/// Propms the user to confirm a command, then executes `f`.
/// If `f` was not executed, returns `None`,
/// otherwise returns the value returned from `f`
pub fn confirm_and_run<R>(question: &str, f: fn() -> R) -> Option<R> {
    println!("{} [Y/n]", question);
    
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        return None;
    }
    
    if input.to_lowercase() == "y\n" {
        return Some(f());
    }

    return None;
}
