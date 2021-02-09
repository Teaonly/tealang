use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::common::*;

/* implementation for VMFunction/JsValue/JsObject */

impl VMFunction {
	pub fn opcode(&self, pc:&mut usize) -> OpcodeType {
		if *pc >= self.code.len() {
			panic!("fetch opcode out of code");
		}
		if let Ok(op) = OpcodeType::try_from(self.code[*pc]) {
			*pc = *pc + 1;
			return op;
		}
		panic!("fetch opcode error!");
	}
	pub fn int(&self, pc:&mut usize) -> f64 {
		if *pc >= self.code.len() {
			panic!("fetch raw out of code");
		}
		let value = self.code[*pc] as f64;
		*pc = *pc + 1;
		return value;
	}
	pub fn number(&self, pc:&mut usize) -> f64 {
		if *pc >= self.code.len() {
			panic!("fetch raw out of code");
		}
		let id = self.code[*pc] as usize;
		if id > self.num_tab.len() {
			panic!("number out of vm");
		}
		let value = self.num_tab[id];

		*pc = *pc + 1;
		return value;
	}
	pub fn var(&self, pc:&mut usize) -> &str {
		if *pc >= self.code.len() {
			panic!("fetch raw out of code");
		}
		let id = self.code[*pc] as usize;
		if id > self.var_tab.len() {
			panic!("var out of vm");
		}

		*pc = *pc + 1;
		return &self.var_tab[id];
	}
	pub fn string(&self, pc:&mut usize) -> &str {
		if *pc >= self.code.len() {
			panic!("fetch raw out of code");
		}
		let id = self.code[*pc] as usize;
		if id > self.str_tab.len() {
			panic!("string out of vm");
		}

		*pc = *pc + 1;
		return &self.str_tab[id];
	}
	pub fn function(&self, pc:&mut usize) -> SharedFunction {
		if *pc >= self.code.len() {
			panic!("fetch function out of code");			
		}
		let id = self.code[*pc] as usize;
		if id > self.func_tab.len() {
			panic!("function out of vm");
		}
		*pc = *pc + 1;
		return self.func_tab[id].clone();
	}
	pub fn address(&self, pc:&mut usize) -> usize {
		let addr = self.code[*pc] as usize + (self.code[*pc+1] as usize) << 16;
		*pc = *pc + 2;
		return addr;
	}
}

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
	pub fn get_exception(&self) -> JsException {
		if let JsClass::exception(e) = &self.value {
			return e.clone();
		}
		panic!("Object can't be a exception!")
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