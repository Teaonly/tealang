use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::*;
use crate::vm::*;

/* implementation for JsValue/JsObject/JsEnvironment/JsRuntime */

impl JsValue {
	pub fn new_null() -> Self {
		JsValue::JSNULL
	}
	pub fn new_undefined() -> Self {
		JsValue::JSUndefined
	}
	pub fn new_false() -> Self {
		JsValue::JSBoolean(false)
	}
	pub fn new_true() -> Self {
		JsValue::JSBoolean(true)
	}
	pub fn new_number(v:f64) -> Self {
		JsValue::JSNumber(v)
	}
	pub fn new_string(v:String) -> Self {
		JsValue::JSString(v)
	}
	pub fn new_object(obj:JsObject) -> Self {
		let shared_obj = SharedObject_new(obj);
		JsValue::JSObject(shared_obj)  
	}
}

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

impl JsRuntime {
	pub fn newobj_from_vmf(&mut self, vmf: VMFunction) -> JsObject {
		let f = JsFunction {
			scope:	self.genv.clone(),
			vmf:	vmf,
		};
		let jclass = JsClass::function(f);
		let fobj = JsObject::new_with_class(self.prototypes.function_prototype.clone(), jclass);
		return fobj;
	} 
}


/*
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
*/

pub fn execute_global(rt: &mut JsRuntime, vmf: VMFunction) {
	// building function object and push to stack
	let jv = JsValue::new_object(rt.newobj_from_vmf(vmf));
	rt.push(jv);


}