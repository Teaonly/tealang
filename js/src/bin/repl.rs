use ezjs;

extern crate rustyline;

use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn main() {
    let mut rt = ezjs::new_runtime();

    println!("Express REPL v0.1.0");
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let content = fs::read_to_string(&args[i]).unwrap();
        let vmf = ezjs::build_function_from_code(&content).unwrap();
        ezjs::run_script(&mut rt, vmf).unwrap();
    }

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("=>");
        match readline {
            Ok(line) => {
                if line != "" {
                    let begin = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    
                    let vmf = ezjs::build_function_from_code(&line).unwrap();
                    let ret = ezjs::run_script(&mut rt, vmf).unwrap();
                    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                    println!("<{}>", end - begin);
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

    //express_end();
}