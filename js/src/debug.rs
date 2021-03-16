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
function test_for_in()
{
    var i, tab, a, b;

    tab = [];
    for(i in {x:1, y: 2}) {
        tab.push(i);
    }
    assert( tab[1] == "y" && tab[0] == "x", "for in 1");

    /* prototype chain test */
    a = {x:1, y: 2, "1": 3};
    b = {"4" : 4 };
    Object.setPrototypeOf(a, b);
    assert(a["4"] == 4, "prototype 1");

    /* non enumerable properties hide enumerables ones in the
       prototype chain */
    a = {y: 2, "1": 3};    
    b = {"x" : 3 };
    Object.setPrototypeOf(a, b);
    tab = [];
    for(i in a) {
        tab.push(i);
    }
    a = tab.toString();
    assert(a == "1, y" || a == "y, 1", "for_in 2");
    
    // TODO
    /* array optimization */
    /* 
    a = [];
    for(i = 0; i < 10; i++)
        a.push(i);
    tab = [];
    for(i in a) {
        tab.push(i);
    }
    assert(tab.toString(), "0,1,2,3,4,5,6,7,8,9", "for_in");
    */


    /* variable definition in the for in */
    tab = [];
    for(var j in {x:1, y: 2}) {
        tab.push(j);
    }
    assert(tab.toString() == "x, y" || tab.toString() == "y, x", "for_in 3");

    println("-------- END TESTING -----------");
}

test_for_in();

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