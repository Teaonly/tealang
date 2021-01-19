use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::*;

/* implementation for JsValue/JsObject/JsEnvironment/JsRuntime */
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


pub fn new_runtime<'a>() -> JsRuntime<'a> {
	let obj = Rc::new(Cell::new(JsObject::new()));
	let boolean = Rc::new(Cell::new(JsObject::new_with_class(obj.clone(), JsClass::boolean(false))));
	let number = Rc::new(Cell::new(JsObject::new_with_class(obj.clone(), JsClass::number(0.0))));	
	let prototypes = JsPrototype {
		object_prototype:	obj.clone(),
		array_prototype:	obj.clone(),
		function_prototype:	obj.clone(),
		boolean_prototype:	boolean,
		number_prototype:	number,
		string_prototype:	obj.clone(),

		error_prototype:	obj.clone(),
		range_err_proto:	obj.clone(),
		ref_err_proto:		obj.clone(),
		syntax_err_proto:	obj.clone(),
		type_err_proto:		obj.clone(),			
	};
	let gobj =  Rc::new(Cell::new(JsObject::new_with_class(obj.clone(), JsClass::object)));
	let genv = JsEnvironment {
		data: 	HashMap::new(),
		outer: 	None,
	};
	let runtime = JsRuntime {
		prototypes:	prototypes,
		gobj:		gobj,
		genv:		genv,
		stack:		Vec::new(),
	};

	return runtime;
}


