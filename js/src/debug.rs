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
    var r, a;
    r = 1 + 2;
    assert(r == 3, "1 + 2 === 3");

    r = 1 - 2;
    assert(r == -1, "1 - 2 === -1");

    r = -1;
    assert(r == -1, "-1 === -1");

    r = +2;
    assert(r == 2, "+2 === 2");

    r = 2 * 3;
    assert(r == 6, "2 * 3 === 6");

    r = 4 / 2;
    assert(r == 2, "4 / 2 === 2");

    r = 4 % 3;
    assert(r == 1, "4 % 3 === 3");

    r = 4 << 2;
    assert(r == 16, "4 << 2 === 16");

    r = 1 << 0;
    assert(r == 1, "1 << 0 === 1");

    r = 1 << 31;
    assert(r == 2147483648, "1 << 31 === 2147483648");

    r = 1 << 32;
    assert(r == 1, "1 << 32 === 1");

    r = (1 << 31) > 0;
    assert(r == true, "(1 << 31) > 0 === true");

    r = -4 >> 1;
    assert(r == -2, "-4 >> 1 === -2");

    // FIXME 
    //r = -4 >>> 1;
    //assert(r == 0x7ffffffe, "-4 >>> 1 === 0x7ffffffe");

    r = 1 & 1;
    assert(r == 1, "1 & 1 === 1");

    r = 0 | 1;
    assert(r == 1, "0 | 1 === 1");

    r = 1 ^ 1;
    assert(r == 0, "1 ^ 1 === 0");

    r = ~1;
    assert(r == -2, "~1 === -2");

    r = !1;
    assert(r == false, "!1 === false");

    assert((1 < 2) == true, "(1 < 2) === true");

    assert((2 > 1) == true, "(2 > 1) === true");

    assert(('b' > 'a') == true, "('b' > 'a') === true");

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
    //debug_compile();
    debug_runtime();
}