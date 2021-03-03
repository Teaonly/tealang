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
    var a, r;
            
    a = 1;
    r = a++;
    assert(r === 1 && a === 2, "++1");

    a = 1;
    r = ++a;
    assert(r === 2 && a === 2, "++2");

    a = 1;
    r = a--;
    assert(r === 1 && a === 0, "--1");

    a = 1;
    r = --a;
    assert(r === 0 && a === 0, "--2");

    a = {x:true};    
    a.x ++;
    assert(a.x == 2, "true++");

    a = {x:true};
    a.x--;
    assert(a.x == 0, "true--");

    a = [true];
    a[0]++;
    assert(a[0] == 2, "++4");

    a = {x:true};
    r = a.x++;
    assert(r === 1 && a.x === 2, "++5");

    a = {x:true};
    r = a.x--;
    assert(r === 1 && a.x === 0, "--4");

    a = [true];
    r = a[0]++;
    assert(r === 1 && a[0] === 2, "++6");

    a = [true];
    r = --a[0];
    a[0]--;
    assert(r === 0 && a[0] === -1, "--5");

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