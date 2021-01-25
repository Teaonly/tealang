use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::*;
use crate::vm::*;

/* implementation for JsValue/JsObject/JsRuntime */

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
	pub fn is_null(&self) -> bool {
		if let JsValue::JSNULL = self {
			return true;
		}
		return false;
	}
	pub fn is_undefined(&self) -> bool {
		if let JsValue::JSUndefined = self {
			return true;
		}
		return false;
	}
	pub fn is_object(&self) -> bool {
		if let JsValue::JSObject(_obj) = self {
			return true;
		}
		return false;
	}
	pub fn get_object(&self) -> SharedObject {
		if let JsValue::JSObject(obj) = self {
			return obj.clone();
		}
		panic!("JsValue is not an object!");		
	}
	pub fn to_number(&self) -> Option<f64> {
		if let JsValue::JSNumber(v) = self {
			return Some(*v);
		}
		if let JsValue::JSObject(obj) = self {
			return obj.borrow().to_number();
		}
		return None;
	}
}

impl JsProperty {
	pub fn new() -> Self {
		JsProperty {
			value: JsValue::new_undefined(),
			attr: JsPropertyAttr::NONE,
			getter: None,
			setter: None,
		}
	}

	pub fn readonly(&self) -> bool {
		if self.attr == JsPropertyAttr::READONLY || self.attr == JsPropertyAttr::READONLY_DONTENUM 
			|| self.attr == JsPropertyAttr::READONLY_DONTCONF || self.attr == JsPropertyAttr::READONLY_DONTENUM_DONTCONF {
			return true;
		}
		return false;
	} 
	
	pub fn configable(&self) -> bool {
		if self.attr == JsPropertyAttr::DONTCONF || self.attr == JsPropertyAttr::READONLY_DONTCONF 
			|| self.attr == JsPropertyAttr::DONTENUM_DONTCONF || self.attr == JsPropertyAttr::READONLY_DONTENUM_DONTCONF {
			return false;
		}
		return true;
	}

	pub fn fill(&mut self, jv: JsValue, attr: JsPropertyAttr, getter:Option<SharedObject>, setter: Option<SharedObject>) {
		self.value = jv;
		self.attr = attr;
		self.getter = getter;
		self.setter = setter;
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

	pub fn to_number(&self) -> Option<f64> {
		if let JsClass::number(v) = self.value {
			return Some(v);
		}
		return None;
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
	
	/* array helper functions */



	/* property's help functions */
	pub fn query_property(&self, name: &str) -> Option<(JsProperty, bool)> {
		let r = self.properties.get(name);
		if r.is_some() {
			return Some((r.unwrap().clone(), true));
		}

		if self.prototype.is_some() {
			let proto = self.prototype.as_ref().unwrap().borrow();
			return Some((proto.query_property(name).unwrap().0, false));
		}
		return None;
	}

	pub fn get_property<'a>(&'a mut self, name: &str) -> &'a mut JsProperty {
		return self.properties.get_mut(name).unwrap();
	}
	pub fn put_property(&mut self, name: &str) -> bool {		
		let result = self.properties.get(name);
		if result.is_some() {
			return true;
		}
		if self.extensible == false {
			return false;
		}
		self.properties.insert(name.to_string(), JsProperty::new());
		return true;
	}
	pub fn fetch_property<'a>(&'a mut self, name: &str) -> Option<&'a mut JsProperty> {
		if self.put_property(name) {
			return self.properties.get_mut(name);
		}
		return None;
	}
}

impl JsEnvironment {
	pub fn init_var(&mut self, name: &str, jv: JsValue) {		
		let attr = JsPropertyAttr::DONTENUM_DONTCONF;
		self.variables.fetch_property(name).unwrap().fill(jv, attr, None, None);
	}
	pub fn new_from(outer: SharedScope) -> SharedScope {
		let env = JsEnvironment {
			variables: JsObject::new(),
			outer: Some(outer),
		};
		SharedScope_new(env)
	}

	pub fn fetch_outer(&self) -> SharedScope {
		if let Some(scope) = &self.outer {
			return scope.clone();
		}
		panic!("Can't fetch outer from env!")
	}
	
	pub fn query_variable(&self, name: &str) -> bool {
		if let Some((_rprop, own)) = self.variables.query_property(name) {
			if own {
				return true;
			}
		}
		return false;
	}
}

impl JsRuntime {
	pub fn newobj_from_vmf(&mut self, vmf: VMFunction) -> JsObject {
		let f = JsFunction {
			scope:	self.cenv.clone(),
			vmf:	Rc::new(Box::new(vmf)),
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