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
    assert(true == true, "XXXX");
"#;

fn test_runtime() {
    let mut rt = new_runtime();
    let vmf = SharedFunction_new(build_function_from_code(script).unwrap());
    dump_function(&vmf);

    run_script(&mut rt, vmf.clone());
}

fn test_compile() {
    if let Ok(vm) = build_function_from_code(script) {
        dump_function(&vm);
    }
}

fn test_ast() {
    let result = build_ast_from_script(script);
    println!("{:?}", result);
}

fn test_token() {
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
    //test_token();
    //test_ast();
    //test_compile();
    test_runtime();
}
