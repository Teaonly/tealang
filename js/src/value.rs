use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::*;

impl JsObject {
    pub fn new() -> JsObject {
        JsObject {
            prototype: None,
            properties: HashMap::new(),
            value: JsClass::object,
        }
    }

    pub fn new_with_class(prototype: SharedObject, value: JsClass) -> JsObject {
        JsObject {
            prototype: Some(prototype),
            properties: HashMap::new(),
            value: value
        }
    }
    
    
    /* property's help functions */
    pub fn new_property<'a>(obj: &'a mut JsObject, name: &str) -> Option<&'a mut JsProperty> {
        let prop = JsProperty {
            value: JsValue::JSUndefined,
            getter: None,
            setter: None,
        };

        obj.properties.insert(name.to_string(), prop);
        return obj.properties.get_mut(name);
    }
}