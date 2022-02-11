use std::os::unix;
use std::str::from_utf8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 { 
        match args[1].as_str() {
            "run" => run(&args[2]),
            _ => println!("Try invoking a command like 'run'\n{} not understood", args[1])
        }
    }
    else {
        println!("No args detected")
    }
}

fn run(command: &String) {
    println!("Running {}!\nOutput resumes below", &command);
    let output = std::process::Command::new(command)
        .output()
        .expect("Could not launch program");
    println!("{:?}", from_utf8(&output.stdout));
}