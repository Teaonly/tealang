use std::rc::Rc;

mod common;
mod token;
mod ast;
mod compile;
mod optimizer;
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

function test_object_literal()
{
    var a = {
        'x':    1234,
        'y':    4321,
        'z':    5678,
    }
    assert(a.x == 1234, "object literal 1");
    assert(a.z == 5678, "object literal 2");

    var b = {
        'x':    1234,
        'y':    {
            'y':    4321
        },
        'z':    5678,
    }
    assert(b.x == 1234, "object literal 3");
    assert(b.z == 5678, "object literal 4");
    assert(b.y.y == 4321, "object literal 5");

    println("-------- END TESTING -----------");
}

test_object_literal();

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
    debug_compile();
    debug_runtime();
}