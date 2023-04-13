use std::ffi::CString;
use std::io::stdin;
use std::io::{stdout, Write};
use std::process::exit;

use nix::unistd::execvp;
use nix::unistd::{chdir, fork, ForkResult};
use nix::sys::wait::waitpid;

fn main() {
    println!("Hotate shell v0.1.0");
    loop {
        shell_loop();
    }
}

fn shell_loop() {
    print!("> ");
    stdout().flush().unwrap();
    let input = get_input();
    execute(input);
}

fn get_input() -> Vec<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    
    let input: Vec<String> = buffer
                    .as_str()
                    .trim_end()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
    return input;
}

fn execute(args: Vec<String>) {
    match args[0].as_str() {
        "cd" => run_cd(args),
        "exit" => run_exit(),
        _ => run_external_command(args),
    }
}

fn run_cd(args: Vec<String>) {
    if args.len() < 2 {
        println!("hotate: invalid argument");
    }else{
        match chdir(args[1].as_str()) {
            Ok(_) => {}
            Err(_) => println!("hotate: invalid argument")
        }
    }
}

fn run_exit() {
    exit(0);
}

fn run_external_command(args: Vec<String>) {
    match unsafe{ fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            let path = CString::new(args[0].to_string()).unwrap();
            let arg = args
                .into_iter()
                .map(|c| CString::new(c).unwrap())
                .collect::<Vec<_>>();
            execvp(&path, &arg).expect("hotate: failed exec");
        }
        Err(_) => {
            println!("hotate: fork failed");
        }
    }
}
