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
    assert(null == undefined);
    //assert(undefined == null);
    //assert(true == 1);
    //assert(0 == false);
    //assert("" == 0);
    //assert("123" == 123);
    //assert("122" != 123);    
    //assert((new Number(1)) == 1);
    //assert(2 == (new Number(2)));
    //assert((new String("abc")) == "abc");    
    //assert({} != "abc");

    print("-------- END TESTING -----------");
"#;

fn debug_runtime() {
    let vmf = SharedFunction_new(build_function_from_code(script).unwrap());    
    let mut rt = new_runtime();    
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
