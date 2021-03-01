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

fn object_preventextensions(rt: &mut JsRuntime) {
    let value = rt.top(-1);
    if value.is_object() {
        value.get_object().borrow_mut().extensible = false;
    }
    rt.push(value);
}

fn object_tostring(rt: &mut JsRuntime)  {
    rt.push_string( "[object]".to_string() );
}

fn object_builtins() -> HashMap<String, JsBuiltinFunction> {
    let mut builtins = HashMap::new();
    builtins.insert("toString".to_string(), JsBuiltinFunction::new(object_tostring, 1));
    builtins.insert("preventExtensions".to_string(), JsBuiltinFunction::new(object_preventextensions, 1));
    return builtins;
}

// The String class
fn string_constructor(rt: &mut JsRuntime) {
    let value = rt.top(-1);
    if value.is_string() {
        rt.push(value);
        return;
    }
    if value.is_something() {
        object_tostring(rt);
        return;
    }
    rt.push_string("".to_string());
}

fn string_tostring(rt: &mut JsRuntime) {
    let value = rt.top(-1).duplicate();
    assert!(value.is_string());
    rt.push(value);
}

fn string_builtins() -> HashMap<String, JsBuiltinFunction> {
    // TODO
    let mut builtins = HashMap::new();
    builtins.insert("toString".to_string(), JsBuiltinFunction::new(string_tostring, 1));    
    return builtins;
}


// The Array class
fn array_constructor(rt: &mut JsRuntime) {
    let a = JsClass::array(Vec::new());
    let obj = JsObject::new_with(rt.prototypes.array_prototype.clone(), a);
    let jv = SharedValue::new_object(obj);
    rt.push(jv);
}

fn array_tostring(rt: &mut JsRuntime) {
    let value = rt.top(-1);
    assert!(value.is_object());
    let sobj = value.get_object();
    let object = sobj.borrow();
    assert!(object.is_array());

    let mut result = String::new();
    let v = object.get_array();
    for i in 0..v.len() {
        result.push_str( &v[i].to_string() );
        if i != v.len() - 1 {
            result.push_str(", ");
        }
    }

    rt.push_string(result);
}

fn array_builtins() -> HashMap<String, JsBuiltinFunction> {
    // TODO
    let mut builtins = HashMap::new();
    builtins.insert("toString".to_string(), JsBuiltinFunction::new(array_tostring, 1));    
    return builtins;
}

// build prototypes chian
fn create_builtin_class(constructor: JsBuiltinFunction, properties: HashMap<String, JsBuiltinFunction>, top: Option<SharedObject>) -> SharedObject {
    let mut class_obj = JsObject::new();
    class_obj.extensible = false;
    class_obj.value = JsClass::builtin(constructor);
    let class_obj =  SharedObject_new(class_obj);
    
    let mut prototype_obj = JsObject::new();
    prototype_obj.extensible = false;
    for (k, v) in properties {
        let f = v.f;
        let argc = v.argc; 
        let func_obj = JsObject::new_builtin(f, argc);
        
        let mut prop = JsProperty::new();
        prop.fill_attr(JsReadonlyAttr);
        prop.value = SharedValue::new_object(func_obj);

        prototype_obj.properties.insert(k, prop);
    }
    let mut prop = JsProperty::new();
    prop.fill_attr(JsReadonlyAttr);
    prop.value = SharedValue::new_sobject(class_obj.clone());
    prototype_obj.properties.insert("constructor".to_string(), prop);
        
    let mut prop = JsProperty::new();
    prop.fill_attr(JsReadonlyAttr);
    prop.value = SharedValue::new_object(prototype_obj);
    class_obj.borrow_mut().properties.insert("prototype".to_string(), prop);
    
    class_obj.borrow_mut().__proto__ = top;    
    return class_obj;
}
pub fn set_global_class(rt: &mut JsRuntime, name: &str, class_obj: SharedObject) {
    let mut prop = JsProperty::new();
    prop.fill_attr(JsReadonlyAttr);
    prop.value = SharedValue::new_sobject(class_obj);
    rt.genv.borrow_mut().target().borrow_mut().set_property(name, prop);
}
pub fn prototypes_init(rt: &mut JsRuntime) {
    // Object
    let top_object = create_builtin_class(JsBuiltinFunction::new(object_constructor, 1), object_builtins(), None);     
    set_global_class(rt, "Object", top_object.clone());
    rt.prototypes.object_prototype = top_object.clone();   
    
    // String
    let string_classs_object = create_builtin_class( JsBuiltinFunction::new(string_constructor, 1), string_builtins(), Some(top_object.clone()));
    set_global_class(rt, "String", string_classs_object.clone());
    rt.prototypes.string_prototype = string_classs_object.clone();

    // Array
    let array_classs_object = create_builtin_class( JsBuiltinFunction::new(array_constructor, 0), array_builtins(), Some(top_object.clone()));
    set_global_class(rt, "Array", array_classs_object.clone());
    rt.prototypes.array_prototype = array_classs_object.clone();

    // Function
}