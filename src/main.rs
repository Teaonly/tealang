extern crate rustyline;
mod tealang;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use tealang::*;

fn main() {
    let mut env:ExpEnv = env_new();
    println!("TeaLang v0.1.0");
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("=>");
        match readline {
            Ok(line) => {
                if line != "" {
                    println!("{}", tealang::run(&line, &mut env));
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}


