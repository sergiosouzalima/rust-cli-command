use std::env;

fn process_command(command: &str) {
    const VERSION: &str = "0.1.0";
    const DESC: &str = "This is a Rust-based CLI program.";

    println!("{}\n",DESC);
    match command {
        "--version" | "-v" => println!("Version {}", VERSION),
        "--help" | "-h" => println!("This is a command line program. \
                                     Usage: my_rust_program \
                                     [--version | -v | --help | -h]"),
        _ => println!("Unknown command: {}", command),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        process_command(command);
        return;
    }
}
