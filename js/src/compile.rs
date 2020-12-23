use std::cell::RefCell;
use std::rc::Rc;
use std::cell::RefMut;

use crate::common::*;
use crate::ast::*;

/* Local help function */
impl VMFunction {
    fn new(name: &str, fname: &str, script: bool) -> Self {
        VMFunction {
            name: name.to_string(),
            script: script,
            arguments: 0,
            code:       Vec::new(),
            num_tab:    Vec::new(),
            str_tab:    Vec::new(),
            var_tab:    Vec::new(),
            fun_tab:    Vec::new(),
            file_name:  fname.to_string(),
        }
    }

    fn emit(&mut self, value: u16) {
        self.code.push(value);
    }
}

fn new_fun(name: &AstNode , params: &AstNode, body: &AstNode, script: bool) -> Result<VMFunction, String> {
    panic!("TODO")
}

pub fn build_function_from_code(script: &str) -> Result<VMFunction, String> {
    let ast = build_ast_from_script("<script>", script).unwrap();

    let null = AstNode::null();
    let func = new_fun(&null, &null, &ast, false)?;
    return Ok(func);
}

