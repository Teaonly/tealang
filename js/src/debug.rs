use std::rc::Rc;

mod common;
mod token;
mod ast;
mod compile;
mod value;
mod execute;
mod builtin;
mod runtime;

use common::*;
use token::*;
use ast::*;
use compile::*;
use runtime::*;

static script: &str = r#"
    var a = 10;
    a += 1
    assert(a == 11, "XXXX");
    a += 2;
    assert(a == 13, "YYYY");
    print(a);
"#;

fn debug_runtime() {
    let mut rt = new_runtime();
    let vmf = SharedFunction_new(build_function_from_code(script).unwrap());
    run_script(&mut rt, vmf.clone());
}

fn debug_compile() {
    if let Ok(vm) = build_function_from_code(script) {
        dump_function(&vm);
    }
}

fn debug_ast() {
    let result = build_ast_from_script(script);
    println!("{:?}", result);
}

fn debug_token() {
    let mut tokens = Tokenlizer::new(script);
    loop {
        let token = tokens.next();
        if token.is_ok() {
            let tk = token.unwrap();
            println!(">> {:?}", tk);
            if tk.tk_type == TokenType::TK_EOF {
                break;
            }
        } else {
            println!("** {:?}", token);
            break;
        }
    }
}

pub fn main() {
    //debug_token();
    //debug_ast();
    //debug_compile();
    debug_runtime();
}
