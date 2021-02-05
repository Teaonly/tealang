use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::*;
use crate::vm::*;

/* implementation for JsValue/JsObject/JsRuntime */

impl SharedValue {
	pub fn swap(&self, other: SharedValue) {
		self.v.swap(&other.v);
	}
	pub fn new_null() -> Self {
		let v = JsValue::JSNULL;		
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}
	pub fn new_undefined() -> Self {
		let v = JsValue::JSUndefined;
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}
	pub fn new_boolean(v:bool) -> Self {
		let v = JsValue::JSBoolean(v);
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}
	pub fn new_number(v:f64) -> Self {
		let v = JsValue::JSNumber(v);
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}	
	pub fn new_vanilla() -> Self {
		let shared_obj = SharedObject_new(JsObject::new());
		let v = JsValue::JSObject(shared_obj);
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}
	pub fn new_object(obj:JsObject) -> Self {
		let shared_obj = SharedObject_new(obj);
		let v = JsValue::JSObject(shared_obj);
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}
	pub fn new_sobject(obj:SharedObject) -> Self {
		let v = JsValue::JSObject(obj);
		SharedValue {
			v: Rc::new(RefCell::new(v))
		}
	}
	pub fn is_null(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSNULL = *v {
			return true;
		}
		return false;
	}
	pub fn is_undefined(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSUndefined = *v {
			return true;
		}
		return false;
	}
	pub fn is_object(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSObject(ref _obj) = *v {
			return true;
		}
		return false;
	}
	pub fn get_object(&self) -> SharedObject {
		let v = self.v.borrow();
		if let JsValue::JSObject(ref obj) = *v {
			return obj.clone();
		}
		panic!("JsValue is not an object!");		
	}
	pub fn is_boolean(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSBoolean(ref _v) = *v {
			return true;
		}
		return false;
	}
	pub fn to_boolean(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSBoolean(ref v) = *v {
			return *v;
		}
		if self.is_null() {
			return false;
		}
		if self.is_undefined() {
			return false;
		}
		return true;
	}
	pub fn is_number(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSNumber(ref _v) = *v {
			return true;
		}
		return false;
	}
	pub fn to_number(&self) -> f64 {
		let v = self.v.borrow();
		if let JsValue::JSNumber(ref v) = *v {
			return *v;
		}
		return std::f64::NAN;
	}
	pub fn is_exception(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSObject(obj) = &*v {
			return obj.borrow().is_exception();
		}
		return false;
	}
	pub fn type_string(&self) -> String {
		let v = self.v.borrow();
		match &*v {
			JsValue::JSUndefined => {
				return "undefined".to_string();
			},
			JsValue::JSNULL => {
				return "null".to_string();
			},
			JsValue::JSBoolean(_b) => {
				return "boolean".to_string();
			},
			JsValue::JSNumber(_num) => {
				return "number".to_string();
			},
			JsValue::JSObject(obj) => {
				return obj.borrow().type_string();
			}
		}
	}
	pub fn is_string(&self) -> bool {
		let v = self.v.borrow();
		if let JsValue::JSObject(obj) = &*v {
			return obj.borrow().is_string();
		}
		return false;
	}
	pub fn to_string(&self) -> String {
		let v = self.v.borrow();
		match &*v {
			JsValue::JSUndefined => {
				return "undefined".to_string();
			},
			JsValue::JSNULL => {
				return "null".to_string();
			},
			JsValue::JSBoolean(b) => {
				if *b {
					return "true".to_string();
				} else {
					return "false".to_string();
				}
			},
			JsValue::JSNumber(num) => {
				return num.to_string();
			},
			JsValue::JSObject(obj) => {
				if obj.borrow().is_string() {
					return obj.borrow().get_string();
				} else {
					return "[object]".to_string();
				}
			}
		}
	}
}

impl JsProperty {
	pub fn new() -> Self {
		JsProperty {
			value: SharedValue::new_undefined(),
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

	pub fn fill(&mut self, jv: SharedValue, attr: JsPropertyAttr, getter:Option<SharedObject>, setter: Option<SharedObject>) {
		self.value = jv;
		self.attr = attr;
		self.getter = getter;
		self.setter = setter;
	}
}

impl JsException {
	pub fn new() -> JsException {
		JsException{}
	}
}

impl JsIterator {
	pub fn new(target_: SharedObject) -> Self {
		let target = target_.borrow();

		let keys = (*target).properties.keys().cloned().collect();
		JsIterator {
			keys: keys,
			index: 0,
		}
	}
	pub fn next(&mut self) -> Option<String> {
		if self.index >=  self.keys.len() {
			return None;
		}
		let s = self.keys[self.index].clone();
		self.index = self.index + 1;
		return Some(s);
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
	
	pub fn new_exception(e: JsException) -> JsObject {		
		JsObject {
			extensible:	false,
			prototype: None,
			properties: HashMap::new(),
			value: JsClass::exception(e),

		}
	}

	pub fn new_iterator(target_: SharedObject) -> JsObject {
		let it = JsIterator::new(target_);
		JsObject {
			extensible:	false,
			prototype: None,
			properties: HashMap::new(),
			value: JsClass::iterator(it),

		}
	}
	
	pub fn new_function(f: SharedFunction, scope: SharedScope) -> JsObject {
		let fvalue = JsClass::function(JsFunction {
			vmf: f,
			scope: scope,
		});
		JsObject {
			extensible:	false,
			prototype: None,
			properties: HashMap::new(),
			value: fvalue,
		}
	}

	pub fn type_string(&self) -> String {
		match &self.value {
			JsClass::string(_) => {
				"string".to_string()
			},
			JsClass::builtin(_) => {
				"builtin".to_string()
			},
			JsClass::function(_) => {
				"function".to_string()
			},
			_ => {
				"object".to_string()
			}
		}
	}

	pub fn is_vanilla(&self) -> bool {
		if let JsClass::object = self.value {
			return true;
		}
		return false;
	}
	pub fn is_exception(&self) -> bool {
		if let JsClass::exception(_e) = &self.value {
			return true;
		}
		return false;
	}
	pub fn is_iterator(&self) -> bool {
		if let JsClass::iterator(_) = self.value {
			return true;
		}
		return false;
	}
	pub fn get_iterator(&mut self) -> &mut JsIterator {
		if let JsClass::iterator(ref mut it) = self.value {
			return it;
		}
		panic!("Object can't be a iterator!")
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
		panic!("Object can't be a builtin!")
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
	pub fn is_string(&self) -> bool {
		if let JsClass::string(ref _func) = self.value {
			return true;
		}
		return false;
	}
	pub fn get_string(&self) -> String {
		if let JsClass::string(ref s) = self.value {
			return s.to_string();
		}
		panic!("Object can't be a string!")
	}
	pub fn callable(&self) -> bool {
		if self.is_function() || self.is_builtin() {
			return true;
		}
		return false;
	}

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

	pub fn get_property(&self, name: &str) -> JsProperty {
		return self.properties.get(name).unwrap().clone();
	}
	pub fn set_property(&mut self, name: &str, prop: JsProperty) {
		self.properties.insert(name.to_string(), prop);
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
	pub fn drop_property(&mut self, name: &str) {		
		self.properties.remove(name);
	}
}

impl JsEnvironment {
	pub fn init_var(&mut self, name: &str, jv: SharedValue) {
		let prop = JsProperty {
			value: jv,
			attr: JsPropertyAttr::DONTENUM_DONTCONF,
			getter: None,
			setter: None,
		};
		if self.variables.put_property(name) {
			self.variables.set_property(name, prop);
		}
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
	let jv = SharedValue::new_object(rt.newobj_from_vmf(vmf));
	rt.push(jv);			// function object
	rt.push_undefined();	// this, undefined for global in strict mode

	jscall(rt, 0);
}