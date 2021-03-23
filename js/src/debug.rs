mod common;
mod token;
mod ast;
mod bytecode;
mod compile;
mod optimizer;
mod value;
mod execute;
mod builtin;
mod runtime;

use token::*;
use ast::*;
use compile::*;
use runtime::*;

static SCRIPT: &str = r#"
var Greeting = /** @class */ (function () {
    function Greeting() {
        this.message = "KaKa";
    }
    Greeting.prototype.greet = function () {        
        println("Hello World!!!");
        println(this.message);
        this.message = "XaXa";
    };
    return Greeting;
}());
var obj = new Greeting();
obj.greet();
println(obj.message);
"#;

fn debug_runtime() {
    let vmf = SharedFunction_new(build_function_from_code(SCRIPT).unwrap());    
    let mut rt = new_runtime();
    run_script(&mut rt, vmf.clone());
}

fn debug_compile() {
    if let Ok(vm) = build_function_from_code(SCRIPT) {
        dump_function(&vm);
    }
}

fn debug_ast() {
    let result = build_ast_from_script(SCRIPT);
    println!("{:?}", result);
}

fn debug_token() {
    let mut tokens = Tokenlizer::new(SCRIPT);
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
    debug_token();
    debug_ast();
    debug_compile();
    debug_runtime();
}