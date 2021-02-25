use std::rc::Rc;
use std::cmp;

use crate::common::*;
use crate::value::*;

/* implementation for JsEnvironment, partly JsRuntime and jscall */

impl JsEnvironment {
	pub fn new()  -> SharedScope {
		let env = JsEnvironment {
			variables: SharedObject_new(JsObject::new()),
			outer: None,
		};
		SharedScope_new(env)
	}
	pub fn new_from(outer: SharedScope) -> SharedScope {
		let env = JsEnvironment {
			variables: SharedObject_new(JsObject::new()),
			outer: Some(outer),
		};
		SharedScope_new(env)
	}
	pub fn target(&self) -> SharedObject {
		self.variables.clone()
	}

	pub fn init_var(&mut self, name: &str, jv: SharedValue) {
		let mut prop = JsProperty::new();
		prop.fill(jv, JsDefaultAttr, None, None);
		
		if self.variables.borrow_mut().put_property(name) {
			self.variables.borrow_mut().set_property(name, prop);
		}
	}

	fn fetch_outer(&self) -> SharedScope {
		if let Some(scope) = &self.outer {
			return scope.clone();
		}
		panic!("Can't fetch outer from env!")
	}
	
	fn query_variable(&self, name: &str) -> bool {
		if let Some((_rprop, own)) = self.variables.borrow().query_property(name) {
			if own {
				return true;
			}
		}
		return false;
	}

	fn get_variable(&self, name: &str) -> JsProperty {
		self.variables.borrow().get_property(name)
	}

	fn put_variable(&self, name: &str) {
		self.variables.borrow_mut().put_property(name);
	}

	fn set_variable(&self, name: &str, prop: JsProperty) {
		self.variables.borrow_mut().set_property(name, prop);
	}

	fn drop_variable(&self, name: &str) {
		self.variables.borrow_mut().drop_property(name);
	}
}

impl JsRuntime {
	/* environment's variables */
	fn delvariable(&mut self, name: &str) -> bool {
		let mut env: SharedScope = self.cenv.clone();
		loop {			
			let r = env.borrow().query_variable(name);
			if r {
				env.borrow().drop_variable(name);
				return true;
			}

			if env.borrow().outer.is_none() {
				return false;
			} 
			let r = env.borrow().fetch_outer();
			env = r; 
		}
	}
	
	fn getvariable(&mut self, name: &str) -> Result<bool, JsException> {
		let mut env: SharedScope = self.cenv.clone();
		loop {			
			let r = env.borrow().query_variable(name);
			if r {
				let prop = env.borrow().get_variable(name);				
				self.push(prop.value.clone());
				return Ok(true);
			}
			if env.borrow().outer.is_none() {
				return Ok(false);
			} 
			let r = env.borrow().fetch_outer();
			env = r;
		}
	}

	fn setvariable(&mut self, name: &str) -> Result<(), JsException> {
		let mut env: SharedScope = self.cenv.clone();
		loop {
			let r = env.borrow().query_variable(name);
			if r {
				let mut prop = env.borrow().get_variable(name);				
				prop.value.replace( self.top(-1) );
				return Ok(());
			}
			if env.borrow().outer.is_none() {
				break;
			}
			let r = env.borrow().fetch_outer();
			env = r;
		}
		
		let value = self.top(-1);
		self.cenv.borrow().put_variable(name);
		let mut prop = self.genv.borrow().get_variable(name);
		prop.value = value;
		self.cenv.borrow().set_variable(name, prop);

		return Ok(());
	}

	/* properties operation */
    // make a new  or replace proptery o for object
    fn defproperty(&mut self, target_: SharedObject, name: &str, value: SharedValue,
		attr:JsPropertyAttr, getter: Option<SharedObject>, setter: Option<SharedObject>) {
		let mut target = target_.borrow_mut();

		if !target.is_vanilla() {
			println!("Cant define property for specia object!");
			return;			
		}

		if target.put_property(name) {
			let mut prop = target.get_property(name);
			if value.is_undefined() && prop.writeable() {
				prop.value = value;
			}
			if prop.configable() {
				if let Some(setter) = setter {
					if setter.borrow().callable() {
						prop.setter = Some(setter);
					} else {
						println!("setter should be callable");
					}
				}
				if let Some(getter) = getter {
					if getter.borrow().callable() {
						prop.getter = Some(getter);
					} else {
						println!("getter should be callable");
					}
				}
			}			
			prop.fill_attr(attr);
			target.set_property(name, prop);
		}
	}

	// change value of the proptery for object
	fn setproperty(&mut self, target_: SharedObject, name: &str, value: SharedValue) -> Result<(), JsException> {		
		let target = target_.borrow_mut();
		let target_ = target_.clone();

		if !target.is_vanilla() {
			println!("Cant set property for specia object!");
			return Err( JsException::new());
		}

		let prop_r = target.query_property(name);
		if let Some((mut prop, _own)) = prop_r {
			if let Some(setter) = prop.setter {
				self.push_object(setter.clone());
				self.push_object(target_);
				self.push(value);
				jscall(self, 1)?;
				self.pop(1);
				return Ok(());
			}
			if prop.writeable() {
				prop.value.replace( value );
				return Ok(());
			} else {								
				println!("Cant write property for specia object!");
				return Err( JsException::new());
			}
		}

		/* Property not found on this object, so create one */
		self.defproperty(target_, name, value, JsDefaultAttr, None, None);
		return Ok(());	
	}	

	// get value from the proptery for object
	fn hasproperty(&mut self, target_: SharedObject, name: &str) -> Result<bool, JsException> {		
		let target = target_.borrow_mut();
		let target_ = target_.clone();

		match target.value {
			JsClass::string(ref s) => {
				if name == "length" {
					self.push_number( s.len() as f64);
					return Ok(true);
				} 
				if let Ok(idx) = name.parse::<usize>() {
					if idx < s.len() {
						self.push_string( s[idx..idx+1].to_string() ); 
						return Ok(true);
					}
				}
			},
			JsClass::array(ref v) => {
				if name == "length" {
					self.push_number( v.len() as f64);
					return Ok(true);
				} 
				if let Ok(idx) = name.parse::<usize>() {
					if idx < v.len() {
						self.push( v[idx].clone() );
						return Ok(true);
					}
				}
			},
			_ => {}
		}

		let prop_r = target.query_property(name);
		if let Some((prop, _own)) = prop_r {
			if let Some(getter) = prop.getter {
				self.push_object(getter.clone());
				self.push_object(target_);
				jscall(self, 0)?;
			} else {
				self.push(prop.value.clone());
			}
			return Ok(true);
		}
		return Ok(false);
	}
	fn getproperty(&mut self, target: SharedObject, name: &str) -> Result<(), JsException> {
		if !self.hasproperty(target, name)? {
			self.push_undefined();
		}
		return Ok(());
	}	
	fn delproperty(&mut self, target_: SharedObject, name: &str) -> bool {		
		let mut target = target_.borrow_mut();

		match target.value {
			JsClass::object => {},
			_ => {
				println!("Cant delete property for specia object!");
				return false;
			}
		}

		let prop_r = target.query_property(name);
		if let Some((prop, own)) = prop_r {
			if own {
				if !prop.configable() {
					target.drop_property(name);
					return true;
				}
			} 
		}
		return false;
	}	

	/* item + item */
	fn concat_add(&mut self) {
		let x = self.top(-2);
		let y = self.top(-1);
		self.pop(2);

		if x.is_number() {			
			let x = x.to_number();
			let y = y.to_number();
			self.push_number(x+y);
			return;
		}
		
		let x = x.to_string();		
		let y = y.to_string();

		self.push_string( x + &y);
	}

	/* item op item */
	fn equal(&mut self) -> bool {
		let x = self.top(-2);
		let y = self.top(-1);
		self.pop(2);

		// string with others
		if x.is_string() {
			let x_str = x.to_string();
			if y.is_string() {
				let y_str = y.to_string();
				if x_str == y_str {
					return true;
				} else {
					return false;
				}
			} else if y.is_number() {
				let y_str = y.to_number().to_string();
				if x_str == y_str {
					return true;
				} else {
					return false;
				}
			}
			return false;
		}

		// null with defineded
		if x.is_undefined() {
			if y.is_undefined() {
				return true;
			}
			if y.is_null() {
				return true;
			}
			return false;
		}

		if x.is_null() {
			if y.is_undefined() {
				return true;
			}
			if y.is_null() {
				return true;
			}
			return false;
		}
		
		// boolean with boolean
		if x.is_boolean()  {
			if y.is_boolean() {
				return x.to_boolean() == y.to_boolean();
			}
			return false;
		}

		// number with others
		if x.is_number() {
			let x_num = x.to_number();
			if y.is_number() {
				let y_num = y.to_number();
				if x_num == y_num {
					return true;
				} else {
					return false;
				}
			}
			if y.is_string() {
				let y_str = y.to_string();
				if let Ok(y_num) = y_str.parse::<f64>() {
					return x_num == y_num;
				}
			} 
			return false;
		}

		// object with object
		let x_obj = x.get_object();
		if y.is_object() {
			let y_obj = y.get_object();
			return Rc::ptr_eq(&x_obj, &y_obj);
		}
		return false;
		
	}

	fn strict_equal(&mut self) -> bool {
		let x = self.top(-2);
		let y = self.top(-1);		
		
		// string with others
		if x.is_string() {
			let x_str = x.to_string();
			if y.is_string() {
				let y_str = y.to_string();
				if x_str == y_str {
					return true;
				}
			} 
			return false;
		}

		// null with defineded
		if x.is_undefined() {
			if y.is_undefined() {
				return true;
			}			
			return false;
		}

		if x.is_null() {			
			if y.is_null() {
				return true;
			}
			return false;
		}

		// boolean with boolean
		if x.is_boolean()  {
			if y.is_boolean() {
				return x.to_boolean() == y.to_boolean();
			}
			return false;
		}

		// number with others
		if x.is_number() {
			let x_num = x.to_number();
			if y.is_number() {
				let y_num = y.to_number();
				if x_num == y_num {
					return true;
				}
			}
			return false;
		}

		// object with object
		let x_obj = x.get_object();
		if y.is_object() {
			let y_obj = y.get_object();
			return Rc::ptr_eq(&x_obj, &y_obj);
		}
		return false;
	}

	fn compare_item(&mut self) -> Option<i32> {
		let x = self.top(-2);
		let y = self.top(-1);
		self.pop(2);

		if x.is_number() {
			let x = x.to_number();
			let y = y.to_number();
			if x == f64::NAN || y == f64::NAN {
				return None;
			}
			if x > y {
				return Some(1);
			} else if x == y {
				return Some(0);
			} else  {
				return Some(-1);
			}
		}
		if x.is_string() {
			let x = x.to_string();
			let y = y.to_string();
			if x > y {
				return Some(1);
			} else if x == y {
				return Some(0);
			} else  {
				return Some(-1);
			}
		}
		return None;
	}

	fn instanceof(&mut self) -> Result<bool, JsException> {
		let x = self.top(-2);
		let y = self.top(-1);
		self.pop(2);
		
		if !x.is_object() {
			return Ok(false);
		}
		if !y.is_object() {
			println!("instanceof: invalid operand");
			self.push_boolean(false);
			return Ok(false);
		}
		let mut x = x.get_object();
		let y = y.get_object();
		if !y.borrow().callable() {
			println!("instanceof: invalid operand");
			self.push_boolean(false);
			return Ok(false);
		}

		self.getproperty(y, "prototype")?;
		let o = self.top(-1);
		self.pop(1);
		if !o.is_object() {			
			println!("instanceof: 'prototype' property is not an object");
			self.push_boolean(false);
			return Ok(false);
		}
		let o = o.get_object();

		loop {
			let proto = x.borrow().prototype.clone();
			if let Some( proto ) = proto {
				x = proto;
				if o.as_ptr() == x.as_ptr() {
					self.push_boolean(true);
					return Ok(true);
				}
			} else {
				break;
			}
		}

		self.push_boolean(false);
		return Ok(false);
	}

	/* convert object to string */
	fn to_string(&mut self, target: SharedValue) -> Result<String, JsException> {
		
		/* try to executing toString() */
		self.getproperty(target.get_object(), "toString")?;
		let object = self.top(-1);
		self.pop(1);
		if object.is_object() {
			if object.get_object().borrow().callable() {
				self.push(object);	// func
				self.push(target);	// this
				jscall(self, 0)?;
				let str_result = self.top(-1);
				self.pop(1);
				return Ok(str_result.to_string());
			}
		}

		return Ok(target.to_string());	
	}

	/* create new object */
	fn new_call(&mut self, argc: usize) -> Result<(), JsException> {
		let obj = self.top(-1 - argc as isize).get_object();
		let fobj = obj.borrow();
		let obj = obj.clone();

		/* built-in constructors create their own objects, give them a 'null' this */
		if fobj.is_builtin() {
			self.push_null();
			if argc > 0 {
				self.rot(argc+1);				
			}
			jscall_builtin(self, argc);
			return Ok(());
		}
		
		/* extract the function object's prototype property */
		self.getproperty(obj, "prototype")?;
		
		let proto = if self.top(-1).is_object() {
			self.top(-1).get_object()
		} else {
			self.prototypes.object_prototype.clone()
		};
		
		self.pop(1);

		/* create a new object with above prototype, and shift it into the 'this' slot */
		let mut nobj = JsObject::new();
		nobj.prototype = Some(proto);
		let nobj = SharedObject_new(nobj);
		self.push_object(nobj.clone());
		if argc > 0 {
			self.rot(argc+1);				
		}

		/* call the function */
		jscall(self, argc)?;

		/* if result is not an object, return the original object we created */
		if !self.top(-1).is_object() {
			self.pop(1);
			self.push_object(nobj);
		}
		return Ok(());
	}

	fn new_closure(&mut self, f: SharedFunction) {
		let fobj = SharedObject_new(JsObject::new_function(f.clone(), self.cenv.clone()));	
		fobj.borrow_mut().prototype = Some(self.prototypes.function_prototype.clone());

		let v = SharedValue::new_number(f.numparams as f64);
		self.defproperty(fobj, "length", v, JsReadonlyAttr, None, None);		
	}

	/* Exceptions */

	/* stack operations */
	pub fn top(&self, offset: isize) -> SharedValue {
		if offset < 0 {
			let offset: usize = (self.stack.len() as isize + offset) as usize;			
			return self.stack[offset].clone();
		}
		panic!("top access only support negtive offset!")
	}
	pub fn push(&mut self, jv: SharedValue) {
		self.stack.push(jv);
	}
	pub fn push_undefined(&mut self) {
		let jv = SharedValue::new_undefined();
		self.stack.push(jv);
	}
	pub fn push_null(&mut self) {
		let jv = SharedValue::new_null();
		self.stack.push(jv);
	}
	pub fn push_boolean(&mut self, v: bool) {
		let jv = SharedValue::new_boolean(v);
		self.stack.push(jv);
	}
	pub fn push_number(&mut self, v:f64) {
		let jv = SharedValue::new_number(v);
		self.stack.push(jv);
	}
	pub fn push_string(&mut self, v:String) {
		let jclass = JsClass::string(v);
		let jobj = JsObject::new_with_class(self.prototypes.string_prototype.clone(), jclass);
		let jv = SharedValue::new_object(jobj);
		self.stack.push(jv);
	}
	pub fn push_object(&mut self, target: SharedObject) {		
		let jv = SharedValue::new_sobject(target);
		self.stack.push(jv);
	}
	fn push_from(&mut self, from: usize) {
		if from >= self.stack.len() {
			panic!("stack underflow! @ push_from");
		}
		let jv = SharedValue::clone( &self.stack[from] );
		self.stack.push(jv);
	}
	/* opcode helper*/
	fn pop(&mut self, mut n: usize) {
		if n > self.stack.len() {
			panic!("stack underflow! @ pop");
		}
		while n > 0 {
			self.stack.pop();
			n = n - 1;
		}
	}
	fn dup(&mut self) {
		if self.stack.len() < 1 {
			panic!("stack underflow! @ dup");
		}
		let nv = self.top(-1);
		self.stack.push(nv);
	}
	fn dup2(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ dup2");
		}

		let nv1: SharedValue = self.top(-2);
		let nv2: SharedValue = self.top(-1);
		self.stack.push(nv1);
		self.stack.push(nv2);
	}
	fn rot(&mut self, n: usize) {
		if self.stack.len() < n {
			panic!("stack underflow! @ rot");
		}
		let top = self.stack.len();
		for i in 0..n-1 {
			self.stack.swap(top-1-i, top-2-i);
		}
	}
	fn rot2(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ rot2");
		}
		/* A B -> B A */
		let top = self.stack.len();
		self.stack.swap(top-1, top-2);
	}
	fn rot3(&mut self) {
		if self.stack.len() < 3 {
			panic!("stack underflow! @ rot3");
		}
		/* A B C -> C A B */
		let top = self.stack.len();
		self.stack.swap(top-1, top-2);
		self.stack.swap(top-2, top-3);
	}
	fn rot4(&mut self) {
		if self.stack.len() < 4 {
			panic!("stack underflow! @ rot4");
		}
		/* A B C D -> D A B C */
		let top = self.stack.len();
		self.stack.swap(top-1, top-2);
		self.stack.swap(top-2, top-3);
		self.stack.swap(top-3, top-4);
	}
	fn rot3pop2(&mut self) {
		if self.stack.len() < 3 {
			panic!("stack underflow! @ rot3pop2");
		}
		/* A B C -> C */
		let top = self.stack.len();
		self.stack[top-3] = self.stack[top-1].clone(); 
		self.pop(2);
	}
	fn rot2pop1(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ rot3pop2");
		}
		/* A B -> B */
		let top = self.stack.len();
		self.stack[top-2] = self.stack[top-1].clone(); 
		self.pop(1);
	}

	fn debugger(&mut self) {
		// runtime virtual machine debugger
		println!("=======>{}", self.stack.len());
	}
	
}


fn jsrun(rt: &mut JsRuntime, func: &VMFunction, pc: usize) -> Result<(), JsException> {
	assert!(rt.stack.len() > 0);
	let mut pc:usize = pc;
	let bot:usize = rt.stack.len() - 1;

	let mut with_exception: Option<JsException> = None;
	let mut catch_scopes: Vec<(usize, usize)> = Vec::new();

	loop {
		let opcode = func.opcode(&mut pc);

		match opcode {
			OpcodeType::OP_POP => {
				rt.pop(1);
			},
			OpcodeType::OP_DUP => {
				rt.dup();
			},
			OpcodeType::OP_DUP2 => {
				rt.dup2();
			},
			OpcodeType::OP_ROT2 => {
				rt.rot2();
			},
			OpcodeType::OP_ROT3 => {
				rt.rot3();
			},
			OpcodeType::OP_ROT4 => {
				rt.rot4();
			},

			OpcodeType::OP_UNDEF => {
				rt.push(SharedValue::new_undefined());
			},
			OpcodeType::OP_NULL => {
				rt.push(SharedValue::new_null());
			},
			OpcodeType::OP_FALSE => {
				rt.push_boolean(false);
			},
			OpcodeType::OP_TRUE => {
				rt.push_boolean(true);
			},

			OpcodeType::OP_INTEGER => {
				let v = func.int(&mut pc);
				rt.push_number(v);
			},
			OpcodeType::OP_NUMBER => {
				let v = func.number(&mut pc);
				rt.push_number(v);
			},
			OpcodeType::OP_STRING => {
				let v = func.string(&mut pc);
				rt.push_string(v.to_string());
			},

			/* Creating objects */
			OpcodeType::OP_CLOSURE => {
				let f = func.function(&mut pc);
				rt.new_closure(f);
			},
			OpcodeType::OP_NEWOBJECT => {
				let obj = SharedValue::new_vanilla();
				rt.push(obj);
			},
			OpcodeType::OP_NEWARRAY => {
				let a = JsClass::array(Vec::new());
				let obj = JsObject::new_with_class(rt.prototypes.array_prototype.clone(), a);
				let jv = SharedValue::new_object(obj);
				rt.push(jv);
			},

			OpcodeType::OP_THIS => {
				let thiz = rt.stack[bot].clone();
				if thiz.is_object() {
					rt.push_from(bot);
				} else {
					let global = rt.genv.borrow().target();
					rt.push_object(global);
				}
			},
			OpcodeType::OP_CURRENT => {
				rt.push_from(bot - 1);
			},
			
			OpcodeType::OP_GETVAR => {
				let s = func.string(&mut pc);
				let result = rt.getvariable(&s);
				let excp = match result {
					Ok(br) => {
						if br == true {
							continue;
						} else {
							println!("'{}' is not defined", s);
							JsException::new()
						}
					},
					Err(e) => {
						e
					},
				};
				with_exception = Some(excp);
				break;
			},
			OpcodeType::OP_HASVAR => {
				let s = func.string(&mut pc);
				let result = rt.getvariable(&s);
				let excp = match result {
					Ok(br) => {
						if br == false {
							rt.push_undefined();
						}
						continue;
					},
					Err(e) => {
						e
					},
				};
				with_exception = Some(excp);
				break;
			},
			OpcodeType::OP_SETVAR => {
				let s = func.string(&mut pc);
				let result = rt.setvariable(s);
				if let Err(e) = result {
					with_exception = Some(e);
					break;
				}
			},
			OpcodeType::OP_DELVAR => {
				let s = func.string(&mut pc);
				let r = rt.delvariable(s);
				rt.push_boolean(r);
			},
			
			OpcodeType::OP_INITPROP => {
				let target = rt.top(-3).get_object();
				let name = match rt.to_string( rt.top(-2)) {
					Ok(s) => s,
					Err(e) => {
						with_exception = Some(e);
						break;
					}
				};
				let value = rt.top(-1);
				if let Err(e) = rt.setproperty(target, &name, value) {
					with_exception = Some(e);
					break;
				}
				rt.pop(2);
			},
			OpcodeType::OP_INITGETTER => {
				let target = rt.top(-3).get_object();
				let name = match rt.to_string( rt.top(-2)) {
					Ok(s) => s,
					Err(e) => {
						with_exception = Some(e);
						break;
					}
				};
				let func = rt.top(-1);
				if func.is_object() {
					rt.defproperty(target, &name, SharedValue::new_undefined(), JsReadonlyAttr, Some(func.get_object()), None);
				} else {
					println!("getter should be a object!");
				}
				rt.pop(2);
			},
			OpcodeType::OP_INITSETTER => {
				let target = rt.top(-3).get_object();
				let name = match rt.to_string( rt.top(-2)) {
					Ok(s) => s,
					Err(e) => {
						with_exception = Some(e);
						break;
					}
				};
				let func = rt.top(-1);
				if func.is_object() {
					rt.defproperty(target, &name, SharedValue::new_undefined(), JsReadonlyAttr, None, Some(func.get_object()));
				} else {
					println!("setter should be a object!");
				}
				rt.pop(2);
			},

			OpcodeType::OP_GETPROP => {
				let target = rt.top(-2).get_object();
				let name = match rt.to_string( rt.top(-1)) {
					Ok(s) => s,
					Err(e) => {
						with_exception = Some(e);
						break;
					}
				};
				if let Err(e) = rt.getproperty(target, &name) {
					with_exception = Some(e);
					break;
				}
				rt.rot3pop2();
			},
			OpcodeType::OP_GETPROP_S => {
				let target = rt.top(-1).get_object();
				let name = func.string(&mut pc);
				if let Err(e) = rt.getproperty(target, &name) {
					with_exception = Some(e);
					break;
				}
				rt.rot2pop1();
			},
			OpcodeType::OP_SETPROP => {
				let target = rt.top(-3).get_object();
				let name = rt.to_string( rt.top(-2))?;
				let value = rt.top(-1);
				if let Err(e) = rt.setproperty(target, &name, value) {
					with_exception = Some(e);
					break;
				}
				rt.rot3pop2();
			},
			OpcodeType::OP_SETPROP_S => {
				let target = rt.top(-2).get_object();
				let value = rt.top(-1);
				let name = func.string(&mut pc);
				if let Err(e) = rt.setproperty(target, &name, value) {
					with_exception = Some(e);
					break;
				}
				rt.rot2pop1();
			},
			OpcodeType::OP_DELPROP => {
				let target = rt.top(-2).get_object();
				let name = match rt.to_string( rt.top(-1)) {
					Ok(s) => s,
					Err(e) => {
						with_exception = Some(e);
						break;
					}
				};
				let b = rt.delproperty(target, &name);
				rt.pop(2);
				rt.push_boolean(b);
			},
			OpcodeType::OP_DELPROP_S => {
				let target = rt.top(-1).get_object();
				let name = func.string(&mut pc);
				let b = rt.delproperty(target, &name);
				rt.pop(1);
				rt.push_boolean(b);
			},

			OpcodeType::OP_ITERATOR => {
				if rt.top(-1).is_object() {
					let target = rt.top(-1).get_object();
					if target.borrow().is_vanilla() {
						let iter = JsObject::new_iterator(target);
						rt.pop(1);
						rt.push( SharedValue::new_object(iter) );
					}
				}
			},
			OpcodeType::OP_NEXTITER => {
				if rt.top(-1).is_object() {
					let target = rt.top(-1).get_object();
					if target.borrow().is_iterator() {
						let mut target = target.borrow_mut();
						let it: &mut JsIterator = target.get_iterator();
						if let Some(s) = it.next() {						
							rt.push_string(s);
							rt.push_boolean(true);
						} else {
							rt.pop(1);
							rt.push_boolean(false);
						}
						continue;
					}
				}
				rt.pop(1);
				rt.push_boolean(false);
			},
			
			/* Function calls */
			OpcodeType::OP_CALL => {
				let n = func.int(&mut pc) as usize;
				if let Err(e) = jscall(rt, n) {
					with_exception = Some(e);
					break;
				}
			},
			OpcodeType::OP_NEW => {
				let n = func.int(&mut pc) as usize;
				if let Err(e) = rt.new_call(n) {
					with_exception = Some(e);
					break;
				}
			},

			/* Unary operators */
			OpcodeType::OP_TYPEOF => {
				let target = rt.top(-1);
				let str = target.type_string();
				rt.pop(1);
				rt.push_string(str);
			},

			OpcodeType::OP_POS => {
				let n = rt.top(-1).to_number();
				rt.pop(1);
				rt.push_number(n);
			},
			OpcodeType::OP_NEG => {
				let n = rt.top(-1).to_number();
				rt.pop(1);
				rt.push_number(-n);
			},
			OpcodeType::OP_BITNOT => {
				let n = rt.top(-1).to_number() as i32;
				rt.pop(1);
				rt.push_number( (!n) as f64 );
			},
			OpcodeType::OP_LOGNOT => {
				let n = rt.top(-1).to_boolean();
				rt.pop(1);
				rt.push_boolean(!n);
			},
			OpcodeType::OP_INC => {
				let n = rt.top(-1).to_number();
				rt.pop(1);
				rt.push_number(n+1.0);
			},
			OpcodeType::OP_DEC => {
				let n = rt.top(-1).to_number();
				rt.pop(1);
				rt.push_number(n-1.0);
			},
			OpcodeType::OP_POSTINC => {
				let n = rt.top(-1).to_number();
				rt.pop(1);
				rt.push_number(n+1.0);
				rt.push_number(n);
			},
			OpcodeType::OP_POSTDEC => {
				let n = rt.top(-1).to_number();
				rt.pop(1);
				rt.push_number(n-1.0);
				rt.push_number(n);
			},
			
			/* Multiplicative operators */
			OpcodeType::OP_MUL => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();
				rt.pop(2);
				rt.push_number(x * y);
			},
			OpcodeType::OP_DIV => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();
				rt.pop(2);
				rt.push_number(x / y);
			},
			OpcodeType::OP_MOD => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();
				rt.pop(2);
				rt.push_number(x % y);
			},

			/* Additive operators */
			OpcodeType::OP_ADD => {
				rt.concat_add();
			},
			OpcodeType::OP_SUB => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();
				rt.pop(2);
				rt.push_number(x - y);
			},

			/* Shift operators */
			OpcodeType::OP_SHL => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();				
				rt.pop(2);
				if x == f64::NAN || y == f64::NAN {
					rt.push_number(0.0);					
				} else if x == f64::INFINITY || y == f64::INFINITY {
					rt.push_number(0.0);
				} else if x == f64::NEG_INFINITY || y == f64::NEG_INFINITY {
					rt.push_number(0.0);
				} else {
					let x = x as i64;
					let y = y as u64;
					rt.push_number( (x << (y&0x1F)) as f64);
				}
			},
			OpcodeType::OP_SHR => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();				
				rt.pop(2);
				if x == f64::NAN || y == f64::NAN {
					rt.push_number(0.0);					
				} else if x == f64::INFINITY || y == f64::INFINITY {
					rt.push_number(0.0);
				} else if x == f64::NEG_INFINITY || y == f64::NEG_INFINITY {
					rt.push_number(0.0);
				} else {
					let x = x as i64;
					let y = y as u64;
					rt.push_number( (x >> (y&0x1F)) as f64);	
				}
			},
			OpcodeType::OP_USHR => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();				
				rt.pop(2);
				if x == f64::NAN || y == f64::NAN {
					rt.push_number(0.0);					
				} else if x == f64::INFINITY || y == f64::INFINITY {
					rt.push_number(0.0);
				} else if x == f64::NEG_INFINITY || y == f64::NEG_INFINITY {
					rt.push_number(0.0);
				} else {
					let x = x as u64;
					let y = y as u64;					
					rt.push_number( (x >> (y&0x1F)) as f64);	
				}
			},

			/* Relational operators */
			OpcodeType::OP_LT => {
				let r = rt.compare_item();
				if let Some(b) = r {
					rt.push_boolean( b < 0 );
				} else {
					rt.push_boolean(false);
				}
			},
			OpcodeType::OP_GT => {
				let r = rt.compare_item();
				if let Some(b) = r {
					rt.push_boolean( b > 0);
				} else {
					rt.push_boolean(false);
				}
			},
			OpcodeType::OP_LE => {
				let r = rt.compare_item();
				if let Some(b) = r {
					rt.push_boolean( b <= 0 );
				} else {
					rt.push_boolean(false);
				}
			},
			OpcodeType::OP_GE => {
				let r = rt.compare_item();
				if let Some(b) = r {
					rt.push_boolean( b >= 0);
				} else {
					rt.push_boolean(false);
				}
			},

			OpcodeType::OP_INSTANCEOF => {
				if let Err(e) = rt.instanceof() {
					with_exception = Some(e);
					break;
				}
			},

			/* Equality */
			OpcodeType::OP_EQ => {
				let b = rt.equal();
				rt.push_boolean(b);
			},
			OpcodeType::OP_NE => {
				let b = rt.equal();
				rt.push_boolean(!b);
			},
			OpcodeType::OP_STRICTEQ => {
				let b = rt.strict_equal();
				rt.pop(2);
				rt.push_boolean(b);
			},
			OpcodeType::OP_STRICTNE => {
				let b = rt.strict_equal();
				rt.pop(2);
				rt.push_boolean(!b);
			},

			/* Binary bitwise operators */
			OpcodeType::OP_BITAND => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();				
				rt.pop(2);
				if x == f64::NAN || y == f64::NAN {
					rt.push_number(0.0);
				} else if x == f64::INFINITY || y == f64::INFINITY {
					rt.push_number(0.0);
				} else if x == f64::NEG_INFINITY || y == f64::NEG_INFINITY {
					rt.push_number(0.0);
				} else {
					rt.push_number( (x as i64 & y as i64) as f64);	
				}
			},
			OpcodeType::OP_BITXOR => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();
				rt.pop(2);
				if x == f64::NAN || y == f64::NAN {
					rt.push_number(0.0);
				} else if x == f64::INFINITY || y == f64::INFINITY {
					rt.push_number(0.0);
				} else if x == f64::NEG_INFINITY || y == f64::NEG_INFINITY {
					rt.push_number(0.0);
				} else {
					rt.push_number( (x as i64 ^ y as i64) as f64);	
				}
			},
			OpcodeType::OP_BITOR => {
				let x = rt.top(-2).to_number();
				let y = rt.top(-1).to_number();
				rt.pop(2);
				if x == f64::NAN || y == f64::NAN {
					rt.push_number(0.0);
				} else if x == f64::INFINITY || y == f64::INFINITY {
					rt.push_number(0.0);
				} else if x == f64::NEG_INFINITY || y == f64::NEG_INFINITY {
					rt.push_number(0.0);
				} else {
					rt.push_number( (x as i64 | y as i64) as f64);
				}
			},

			/* Try and Catch */	
			OpcodeType::OP_TRY => {
				let catch_block = func.address(&mut pc);
				catch_scopes.push((pc, rt.stack.len()));
				pc = catch_block;
			},
			OpcodeType::OP_ENDTRY => {				
				catch_scopes.pop();
			},
			OpcodeType::OP_CATCH => {				
				let str = func.string(&mut pc);
				let eobj = rt.top(-1);
				rt.pop(1);

				let new_env = JsEnvironment::new_from(rt.cenv.clone());
				new_env.borrow_mut().init_var(str, eobj);
				rt.cenv = new_env;
			},
			OpcodeType::OP_ENDCATCH => {				
				let outer = rt.cenv.borrow().fetch_outer();
				rt.cenv = outer;
			},
			OpcodeType::OP_THROW => {
				let evalue = rt.top(-1);
				rt.pop(1);
				let e = evalue.get_object().borrow().get_exception();
				with_exception = Some(e);
				break;		
			},
			
			/* Branching & Flow control */			
			OpcodeType::OP_JCASE => {
				let offset = func.address(&mut pc);
				let b = rt.strict_equal();
				if b {
					rt.pop(2);
					pc = offset;
				} else {
					rt.pop(1);
				}
			},	
			OpcodeType::OP_JUMP => {
				let addr = func.address(&mut pc);
				pc = addr;
			},
			OpcodeType::OP_JTRUE => {
				let addr = func.address(&mut pc);
				let b = rt.top(-1).to_boolean();
				rt.pop(1);
				if b {
					pc = addr;
				}
			},
			OpcodeType::OP_JFALSE => {
				let addr = func.address(&mut pc);
				let b = rt.top(-1).to_boolean();
				rt.pop(1);
				if !b {
					pc = addr;
				}
			},
			OpcodeType::OP_RETURN => {
				break;
			},

			OpcodeType::OP_DEBUG => {
				rt.debugger();
				panic!("Exiting with debug");
			},

			/* do nothing */
			OpcodeType::OP_EVAL => {},
			OpcodeType::OP_NOP => {},
			OpcodeType::OP_LAST => {},
		}
	}

	// handle exception
	if let Some(e) = with_exception {
		if let Some((new_pc, new_top)) = catch_scopes.pop() {
			let dropped = rt.stack.len() - new_top;
			rt.pop(dropped);

			rt.push( SharedValue::new_object( JsObject::new_exception(e)));
			return jsrun(rt, func, new_pc);
		}
		
		return Err(e);
	} 
	return Ok(());
}

fn jscall_script(rt: &mut JsRuntime, argc: usize) -> Result<(), JsException> {
	let bot = rt.stack.len() - 1 - argc;

	let fobj = rt.stack[bot-1].get_object();
	let rfobj = fobj.borrow();
	let vmf = &rfobj.get_func().vmf;

	/* init var in current env*/
	for i in (0..vmf.numvars) {
		let jv = SharedValue::new_undefined();
		let var = &vmf.str_tab[i];
		rt.cenv.borrow_mut().init_var(var, jv);
	}

	/* scripts take no arguments */
	rt.pop(argc);
	jsrun(rt, vmf, 0)?;

	/* clear stack */
	let jv = rt.stack.pop().unwrap();
	rt.pop(2);
	rt.push(jv);

	return Ok(())
}

fn jscall_function(rt: &mut JsRuntime, argc: usize) -> Result<(), JsException> {	
	let bot = rt.stack.len() - 1 - argc;

	let fobj = rt.stack[bot-1].get_object();
	let rfobj = fobj.borrow();
	let vmf = &rfobj.get_func().vmf;

	/* create new scope */
	let new_env = JsEnvironment::new_from(rfobj.get_func().scope.clone());
	let old_env = rt.cenv.clone();
	rt.cenv = new_env;

	/* create arguments */
	if vmf.numparams > 0 {
		let arg_obj = JsObject::new_with_class( rt.prototypes.object_prototype.clone(), JsClass::object);
		let arg_value = SharedValue::new_object(arg_obj);

		let jv = SharedValue::new_number(argc as f64);
		rt.defproperty(arg_value.get_object(), "length", jv,  JsReadonlyAttr, None, None);

		for i in 0..argc {
			let name = i.to_string();
			let jv = rt.stack[bot+1+i].clone();
			rt.defproperty(arg_value.get_object(), &name, jv, JsDefaultAttr, None, None);
		}

		rt.cenv.borrow_mut().init_var("arguments", arg_value);
	}

	/* setup remained arguments*/
	let min_argc = cmp::min(argc, vmf.numparams);
	for i in 0..min_argc {
		let argv = rt.stack[i + 1 + bot].clone();
		rt.cenv.borrow_mut().init_var(&vmf.str_tab[i], argv);
	}
	rt.pop(argc);

	/* init var in current env*/
	for i in min_argc..(vmf.numvars + vmf.numparams) {
		let jv = SharedValue::new_undefined();
		rt.cenv.borrow_mut().init_var(&vmf.str_tab[i], jv);
	}

	jsrun(rt, vmf, 0)?;

	/* clear stack */
	let jv = rt.stack.pop().unwrap();
	rt.pop(2);
	rt.push(jv);

	/* restore old env */
	rt.cenv = old_env;

	return Ok(());
}

fn jscall_builtin(rt: &mut JsRuntime, argc: usize) {
	let bot = rt.stack.len() - 1 - argc;
	let fobj = rt.stack[bot-1].get_object();
	let builtin = fobj.borrow().get_builtin();

	for _i in argc .. builtin.argc {
		rt.push_undefined();
	}

	(builtin.f)(rt);

	let jv = rt.stack.pop().unwrap();
	rt.pop(builtin.argc + 2);
	rt.push(jv);
}

pub fn jscall(rt: &mut JsRuntime, argc: usize) -> Result<(), JsException> {
	assert!(rt.stack.len() >= argc + 2);
	let bot = rt.stack.len() - 1 - argc;

	let fobj = rt.stack[bot-1].get_object();
	if fobj.borrow().is_function() == true {

		if fobj.borrow().get_func().vmf.script {
			jscall_script(rt, argc)?;
		} else {
			jscall_function(rt, argc)?;
		};

	} else if fobj.borrow().is_builtin() == true {
		jscall_builtin(rt, argc);
	} else {
        panic!("Can't call none function object");
	}
	
	return Ok(());
}
