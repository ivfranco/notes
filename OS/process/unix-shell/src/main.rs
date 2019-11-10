use itertools::Itertools;
use nix::{
    fcntl::{open, OFlag},
    libc::{STDERR_FILENO, STDOUT_FILENO},
    sys::stat::Mode,
    sys::wait::wait,
    unistd::{dup2, execvp, fork, ForkResult},
};
use std::{
    ffi::CString,
    io::{stdin, stdout, Write},
};
use unix_shell::Result;

const PROMPT: &str = "osc";

fn main() -> Result<()> {
    let mut history: Vec<(Vec<CString>, bool)> = vec![];

    loop {
        print!("{}> ", PROMPT);
        stdout().flush()?;
        let command = match parse_command_line() {
            Ok(command) => command,
            Err(err) => return Err(err),
        };

        match command {
            Command::Exit => break,
            Command::Exec(args, background) => {
                spawn_child(&args, background)?;
                history.push((args, background));
            }
            Command::History => {
                for (idx, (args, _)) in history.iter().enumerate().rev() {
                    println!(
                        "{} {}",
                        idx + 1,
                        args.iter().map(|cstr| cstr.to_str().unwrap()).format(" ")
                    );
                }
            }
            Command::RetrieveLast => {
                retrieve_and_spawn(&history, history.len())?;
            }
            Command::Retrieve(idx) => {
                retrieve_and_spawn(&history, idx)?;
            }
            _ => (),
        }
    }

    Ok(())
}

fn retrieve_and_spawn(history: &[(Vec<CString>, bool)], idx: usize) -> Result<()> {
    if idx < 1 || idx > history.len() {
        eprintln!("Error: history index out of bound");
        Ok(())
    } else {
        let (args, bool) = &history[idx - 1];
        spawn_child(args, *bool)
    }
}

enum Command {
    Empty,
    Exit,
    Exec(Vec<CString>, bool),
    History,
    RetrieveLast,
    Retrieve(usize),
}

fn parse_command_line() -> Result<Command> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    assert_eq!(buf.pop(), Some('\n'));

    let command = match buf.as_str() {
        "" => Command::Empty,
        "exit" => Command::Exit,
        "history" => Command::History,
        "!!" => Command::RetrieveLast,
        _ if buf.starts_with('!') => {
            if let Ok(idx) = buf[1..].parse::<usize>() {
                Command::Retrieve(idx)
            } else {
                eprintln!("Error: history index is not a non-negative integer");
                Command::Empty
            }
        }
        _ => {
            let mut args = buf
                .split(' ')
                .map(|arg| CString::new(arg).unwrap())
                .collect::<Vec<_>>();

            let background = args.last().and_then(|arg| arg.to_str().ok()) == Some("&");
            if background {
                args.pop();
            }

            Command::Exec(args, background)
        }
    };

    Ok(command)
}

fn spawn_child(args: &[CString], background: bool) -> Result<()> {
    if let ForkResult::Parent { child } = fork()? {
        if !background {
            wait()?;
        } else {
            println!("spawned child process {}", child);
        }
    } else {
        if background {
            let dev_null = open("/dev/null", OFlag::O_WRONLY, Mode::empty())?;
            dup2(dev_null, STDOUT_FILENO)?;
            dup2(dev_null, STDERR_FILENO)?;
        }
        execvp(&args[0], args)?;
    }

    Ok(())
}
