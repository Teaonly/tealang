use crate::common::*;
use crate::value::*;
use crate::execute::*;

// The Object class 
fn object_constructor(rt: &mut JsRuntime) {
    let value = rt.top(-1);
    if value.is_something() {        
        rt.push( value.duplicate() );
    }
    rt.push( SharedValue::new_vanilla() );
}

/*
fn create_builtin_class(constructor: builtin, properties: HashMap<String, builtin>) -> JsObject {
    
}
*/

pub fn prototypes_init(rt: &mut JsRuntime) {

}