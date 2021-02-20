use crate::common::*;
use crate::value::*;
use crate::execute::*;

fn assert(rt: &mut JsRuntime) {    
    let b = rt.top(-2).to_boolean();
    if !b {
        let info = rt.top(-1).to_string();
        panic!("ASSERT: {}", info);
    }
    rt.push_undefined();
}

fn print(rt: &mut JsRuntime) {
    let info = rt.top(-1).to_string();
    println!("{}", info);
    rt.push_undefined();
}

pub fn builtin_init(rt: &mut JsRuntime) {
    rt.genv.borrow_mut().init_var("assert", SharedValue::new_object(JsObject::new_builtin(assert, 2)) );
    rt.genv.borrow_mut().init_var("print", SharedValue::new_object(JsObject::new_builtin(print, 1)) );    
}