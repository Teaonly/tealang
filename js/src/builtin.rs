use std::collections::HashMap;

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

fn create_builtin_class(constructor: JsBuiltinFunction, properties: HashMap<String, (fn(&mut JsRuntime), usize)>) -> JsObject {
    let mut class_obj = JsObject::new();
    class_obj.extensible = false;
    class_obj.value = JsClass::builtin(constructor);
    for (k, (f, argc)) in properties {
        let func_obj = JsObject::new_builtin(f, argc);
        
        let mut prop = JsProperty::new();
        prop.fill_attr(JsReadonlyAttr);
        prop.value = SharedValue::new_object(func_obj);

        class_obj.properties.insert(k, prop);
    }
    return class_obj;
}

pub fn prototypes_init(rt: &mut JsRuntime) {

}