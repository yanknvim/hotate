// Hotate shell v0.1.2

use std::ffi::CString;
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::exit;

use nix::unistd::{execvp, chdir, fork, ForkResult};
use nix::sys::wait::waitpid;

use dirs::home_dir;

fn main() {
    println!("Hotate shell v0.1.2");
    loop {
        shell_loop();
    }
}

fn shell_loop() {
    show_prompt();
    let input = get_input();
    execute(input);
}

fn show_prompt() {
    let current_directory = env::current_dir().expect("hotate: failed to read current directory");
    print!("{} > ", current_directory.display());
    stdout().flush().unwrap();
}

fn get_input() -> Vec<String> {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => println!("hotate: failed to read input")
    }
    
    let input: Vec<String> = buffer
                    .as_str()
                    .trim_end()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
    return input;
}

fn execute(args: Vec<String>) {
    if args.len() == 0 {
        return;
    }
    match args[0].as_str() {
        "cd" => run_cd(args),
        "exit" => run_exit(),
        "help" => run_help(),
        _ => run_external_command(args),
    }
}

fn run_cd(args: Vec<String>) {
    if args.len() < 2 {
        match chdir(&home_dir().unwrap()) {
            Ok(_) => {}
            Err(_) => println!("cd: failed to chdir")
        }
    }else{
        match chdir(args[1].as_str()) {
            Ok(_) => {}
            Err(_) => println!("cd: invalid argument")
        }
    }
}

fn run_exit() {
    exit(0);
}

fn run_help() {
    println!("Hotate shell v0.1.2");
    println!("cd: move current directory");
    println!("exit: exit Hotate");
    println!("help: show this help");
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
