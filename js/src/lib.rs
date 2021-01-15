pub mod common;
mod token;
mod ast;
mod compile;
mod run;
mod value;

pub fn dummy() {
    let script = r#"
    "use strict";
    for (;;) {
    }
    "#;

    compile::build_function_from_code(script).unwrap();
}