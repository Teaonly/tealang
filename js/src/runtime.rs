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
	pub fn is_object(&self) -> bool {
		if let JsValue::JSObject(_obj) = self {
			return true;
		}
		return false;
	}
	pub fn as_object(&self) -> SharedObject {
		if let JsValue::JSObject(obj) = self {
			return obj.clone();
		}
		panic!("JsValue is not an object!");
	}
}

impl JsObject {
    pub fn new() -> JsObject {
        JsObject {
			extensible:	true,
            prototype: None,
            properties: HashMap::new(),
            value: JsClass::object,
        }
	}
	
	pub fn new_with_class(prototype: SharedObject, value: JsClass) -> JsObject {
        JsObject {
			extensible:	true,
            prototype: Some(prototype),
            properties: HashMap::new(),
            value: value
        }
	}

	pub fn is_builtin(&self) -> bool {
		if let JsClass::builtin(_) = self.value {
			return true;
		}
		return false;
	}
	pub fn get_builtin(&self) -> JsBuiltinFunction {
		if let JsClass::builtin(ref func) = self.value {
			return func.clone();
		}
		panic!("Object can't be a func!")
	}
	pub fn is_function(&self) -> bool {
		if let JsClass::function(ref _func) = self.value {
			return true;
		}
		return false;
	}
	pub fn get_func(&self) -> &JsFunction {
		if let JsClass::function(ref func) = self.value {
			return func;
		}
		panic!("Object can't be a func!")
	}

	/* property's help functions */
	pub fn setproperty(&mut self, name: &str, v: JsValue, attr: JsPropertyAttr, getter: Option<SharedObject>, setter: Option<SharedObject>) { 
		if let JsClass::array(ref vec) = self.value {
			if name == "length" {
				panic!("length is readonly for array object!");
			}
		} 
	
		let property = JsProperty {
			value:	v,
			attr:	attr,
			getter:	None,
			setter: None,
		};

		self.properties.insert(name.to_string(), property);		
	}
}

impl JsEnvironment {
	pub fn init_var(&mut self, name: &str, jv: JsValue) {		
		let attr = JsPropertyAttr::DONTENUM_DONTCONF;
		self.variables.setproperty(name, jv, attr, None, None);
	}
	pub fn new_from(outer: SharedScope) -> SharedScope {
		let env = JsEnvironment {
			variables: JsObject::new(),
			outer: Some(outer),
		};
		SharedScope_new(env)
	}
}

impl JsRuntime {
	pub fn newobj_from_vmf(&mut self, vmf: VMFunction) -> JsObject {
		let f = JsFunction {
			scope:	self.cenv.clone(),
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
	// variable to the enviroment
	rt.cenv = rt.genv.clone();	
	let jv = JsValue::new_object(rt.newobj_from_vmf(vmf));
	rt.push(jv);			// function object
	rt.push_undefined();	// this, undefined for global in strict mode

	jscall(rt, 0);
}