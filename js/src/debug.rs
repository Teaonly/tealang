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
    assert((NaN | 0) === 0);
    assert((Infinity | 0) === 0);
    assert(((-Infinity) | 0) === 0);

    assert(("12345" | 0) === 12345);    
    assert(("0x12345" | 0) === 0x12345);

    assert(("12345" >>> 0) === 12345);
    assert(("0x12345" >>> 0) === 0x12345);

    assert((NaN >>> 0) === 0);
    assert((Infinity >>> 0) === 0);
    assert(((-Infinity) >>> 0) === 0);

    assert(null == undefined);
    assert(undefined == null);

    assert("123" == 123);
    assert("122" != 123);

    // FIXME
    // assert(((4294967296 * 3 - 4) | 0) === -4);
    // assert(((4294967296 * 3 - 4) >>> 0) === (4294967296 - 4));

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
