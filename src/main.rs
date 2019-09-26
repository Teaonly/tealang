extern crate rustyline;
mod tealang;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use tealang::*;

fn build_extern(env: &mut ExpEnv) {
    let mut data: HashMap<String, ExpNode> = HashMap::new();

    let build = ExpNode::TFunc( |_args: &[ExpNode], _env: &mut ExpEnv| {
        Ok(ExpNode::TExtern(0))
    });
    data.insert("build".to_string(), build);

    env.extend(&data);
}

fn main() {
    let mut env:ExpEnv = env_new();
    build_extern(&mut env);

    println!("TeaLang v0.1.0");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let buffered = BufReader::new(input);

        let mut contents = "".to_string();
        for line in buffered.lines() {
            let mut line = line.unwrap().to_string();
            if let Some(pos) = line.find(";") {
                let (code, _) = line.split_at(pos);
                line = code.to_string();
            }

            contents.push_str( &line );
            contents.push_str("\n");
        }

        println!("{}", tealang::run(&contents, &mut env));
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


