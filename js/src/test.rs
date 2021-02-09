mod common;
mod token;
mod ast;
mod compile;

use common::*;
use token::*;
use ast::*;
use compile::*;

fn test_compile() {
    let script = r#"
        var s = 0;
        for (var a = 1; a < 10; a++) {
            s += a;
        }
    "#;

    if let Ok(vm) = build_function_from_code(script) {
        dump_function(&vm);
    }
    
}

fn test_ast() {
    let script = r#"
        var a = b
        var afunc = function (arr) {
            //start the endIndex at the last index of the array
            var endIndex = arr.length - 1;
            return endIndex;
        }

        while(endIndex > 0) {            
            endIndex--;
        } 
    "#;

    let result = build_ast_from_script(script);
    println!("{:?}", result);
}

fn test_token() {
    let script = r#"
        var a = b
        var afunc = function (arr) {
            //start the endIndex at the last index of the array
            var endIndex = arr.length - 1;
            return endIndex;
        }

        while(endIndex > 0){            
            endIndex--;
        } 
    "#;

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
    test_compile();
}