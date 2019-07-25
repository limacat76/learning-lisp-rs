use std::io;
use std::io::prelude::*;

fn intro_blurb() {
    let software = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{} v{}", software, version);
    let mut ctrl_d = "CTRL+D";
    if cfg!(target_os = "windows") {
        ctrl_d = "CTRL+Z";
    }
    println!("Press {} with an empty line to exit", ctrl_d);
}

fn press_any_key() {
    let mut stdout = io::stdout();
    write!(stdout, "\nPress any key to exit\n").unwrap();
    stdout.flush().unwrap();

    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

fn read() -> (String, bool) {
    print!("user> ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    let bytes = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let cont = bytes > 0;

    (input, cont)
}

fn eval(input: &String) -> (String, bool) {
    (input.clone(), true)
}

fn print(result: &String) {
    println!("{}", result);
}

fn repl(
    reader: fn() -> (String, bool),
    evaluator: fn(&String) -> (String, bool),
    printer: fn(&String) -> (),
) {
    loop {
        let (x, cont) = reader();
        if !cont {
            break;
        }
        let (y, ok) = evaluator(&x);
        if !ok {
            println!("Error detected");
        } else {
            printer(&y);
        }
    }
}

fn main() {
    intro_blurb();
    repl(read, eval, print);
    press_any_key();
}
