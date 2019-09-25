extern crate rustyline;
mod tealang;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use tealang::*;

fn main() {
    let mut env:ExpEnv = env_new();
    println!("TeaLang v0.1.0");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        /*
        let contents = fs::read_to_string( &args[1] ).expect("");
        if contents != "" {
            tealang::run(&contents, &mut env);
        }
        */

        let input = File::open(&args[1]).unwrap();
        let buffered = BufReader::new(input);

        let mut contents = "".to_string();
        for line in buffered.lines() {
            let line = line.unwrap().to_string();
            let new_line = line.replace(" ", "");
            if new_line.starts_with(";") {
                continue;
            }

            contents.push_str( &line );
            contents.push_str("\n");
        }

        tealang::run(&contents, &mut env);
    }

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


