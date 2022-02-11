use std::os::unix;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "run" => run(&args[2]),
        _ => println!("Try invoking a command like 'run'{}", args[1])
    }
}

fn run(command: &String) {
    println!("Running {}!", &command);
    std::process::Command::new(command)
        .output()
        .expect("Could not launch program");
}