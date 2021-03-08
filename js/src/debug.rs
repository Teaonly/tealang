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
    function F(x)
    {
        this.x = x;
    }

    var a, b;
    a = new Object();
    a.x = 1;
    assert(a.x == 1, "new 1");
    b = new F(2);
    assert(b.x == 2, "new 2");

    a = {};
    b = "Hello";
    assert((a instanceof Object) == true, "instanceof 1");
    assert((b instanceof Object) == true, "instanceof 2");
    assert((a instanceof String) == false, "instanceof 3");
    assert((b instanceof String) == true, "instanceof 4");


    assert((typeof 1) == "number", "typeof 1");
    assert((typeof Object) == "function", "typeof 2");
    assert((typeof null) == "object", "typeof 3");
    assert((typeof unknown_var) == "undefined", "typeof 4");

    println("-------- END TESTING -----------");
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