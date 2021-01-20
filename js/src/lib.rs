pub mod common;
mod token;
mod ast;
mod compile;
mod runtime;
mod vm;


// public interface

pub fn dummy() {
    let script = r#"
    "use strict";
    for (;;) {
    }
    "#;

    compile::build_function_from_code(script).unwrap();
}