use std::convert::TryFrom;
use std::cell::Cell;
use std::rc::Rc;
use std::collections::HashMap;
use std::cmp;

use crate::common::*;
use crate::runtime::*;

impl TryFrom<u16> for OpcodeType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
			x if x == OpcodeType::OP_NOP as u16 => Ok(OpcodeType::OP_NOP),
			x if x == OpcodeType::OP_POP as u16 => Ok(OpcodeType::OP_POP),
			x if x == OpcodeType::OP_DUP as u16 => Ok(OpcodeType::OP_DUP),
			x if x == OpcodeType::OP_DUP2 as u16 => Ok(OpcodeType::OP_DUP2),
			x if x == OpcodeType::OP_ROT2 as u16 => Ok(OpcodeType::OP_ROT2),
			x if x == OpcodeType::OP_ROT3 as u16 => Ok(OpcodeType::OP_ROT3),
			x if x == OpcodeType::OP_ROT4 as u16 => Ok(OpcodeType::OP_ROT4),
			x if x == OpcodeType::OP_INTEGER as u16 => Ok(OpcodeType::OP_INTEGER),
			x if x == OpcodeType::OP_NUMBER as u16 => Ok(OpcodeType::OP_NUMBER),
			x if x == OpcodeType::OP_STRING as u16 => Ok(OpcodeType::OP_STRING),
			x if x == OpcodeType::OP_CLOSURE as u16 => Ok(OpcodeType::OP_CLOSURE),
			x if x == OpcodeType::OP_NEWARRAY as u16 => Ok(OpcodeType::OP_NEWARRAY),
			x if x == OpcodeType::OP_NEWOBJECT as u16 => Ok(OpcodeType::OP_NEWOBJECT),
			x if x == OpcodeType::OP_NEWREGEXP as u16 => Ok(OpcodeType::OP_NEWREGEXP),
			x if x == OpcodeType::OP_UNDEF as u16 => Ok(OpcodeType::OP_UNDEF),
			x if x == OpcodeType::OP_NULL as u16 => Ok(OpcodeType::OP_NULL),
			x if x == OpcodeType::OP_TRUE as u16 => Ok(OpcodeType::OP_TRUE),
			x if x == OpcodeType::OP_FALSE as u16 => Ok(OpcodeType::OP_FALSE),
			x if x == OpcodeType::OP_THIS as u16 => Ok(OpcodeType::OP_THIS),
			x if x == OpcodeType::OP_CURRENT as u16 => Ok(OpcodeType::OP_CURRENT),
			x if x == OpcodeType::OP_GETLOCAL as u16 => Ok(OpcodeType::OP_GETLOCAL),
			x if x == OpcodeType::OP_SETLOCAL as u16 => Ok(OpcodeType::OP_SETLOCAL),
			x if x == OpcodeType::OP_DELLOCAL as u16 => Ok(OpcodeType::OP_DELLOCAL),
			x if x == OpcodeType::OP_HASVAR as u16 => Ok(OpcodeType::OP_HASVAR),
			x if x == OpcodeType::OP_GETVAR as u16 => Ok(OpcodeType::OP_GETVAR),
			x if x == OpcodeType::OP_SETVAR as u16 => Ok(OpcodeType::OP_SETVAR),
			x if x == OpcodeType::OP_DELVAR as u16 => Ok(OpcodeType::OP_DELVAR),
			x if x == OpcodeType::OP_INITPROP as u16 => Ok(OpcodeType::OP_INITPROP),
			x if x == OpcodeType::OP_INITGETTER as u16 => Ok(OpcodeType::OP_INITGETTER),
			x if x == OpcodeType::OP_INITSETTER as u16 => Ok(OpcodeType::OP_INITSETTER),
			x if x == OpcodeType::OP_GETPROP as u16 => Ok(OpcodeType::OP_GETPROP),
			x if x == OpcodeType::OP_GETPROP_S as u16 => Ok(OpcodeType::OP_GETPROP_S),
			x if x == OpcodeType::OP_SETPROP as u16 => Ok(OpcodeType::OP_SETPROP),
			x if x == OpcodeType::OP_SETPROP_S as u16 => Ok(OpcodeType::OP_SETPROP_S),
			x if x == OpcodeType::OP_DELPROP as u16 => Ok(OpcodeType::OP_DELPROP),
			x if x == OpcodeType::OP_DELPROP_S as u16 => Ok(OpcodeType::OP_DELPROP_S),
			x if x == OpcodeType::OP_ITERATOR as u16 => Ok(OpcodeType::OP_ITERATOR),
			x if x == OpcodeType::OP_NEXTITER as u16 => Ok(OpcodeType::OP_NEXTITER),
			x if x == OpcodeType::OP_EVAL as u16 => Ok(OpcodeType::OP_EVAL),
			x if x == OpcodeType::OP_CALL as u16 => Ok(OpcodeType::OP_CALL),
			x if x == OpcodeType::OP_NEW as u16 => Ok(OpcodeType::OP_NEW),
			x if x == OpcodeType::OP_TYPEOF as u16 => Ok(OpcodeType::OP_TYPEOF),
			x if x == OpcodeType::OP_POS as u16 => Ok(OpcodeType::OP_POS),
			x if x == OpcodeType::OP_NEG as u16 => Ok(OpcodeType::OP_NEG),
			x if x == OpcodeType::OP_BITNOT as u16 => Ok(OpcodeType::OP_BITNOT),
			x if x == OpcodeType::OP_LOGNOT as u16 => Ok(OpcodeType::OP_LOGNOT),
			x if x == OpcodeType::OP_INC as u16 => Ok(OpcodeType::OP_INC),
			x if x == OpcodeType::OP_DEC as u16 => Ok(OpcodeType::OP_DEC),
			x if x == OpcodeType::OP_POSTINC as u16 => Ok(OpcodeType::OP_POSTINC),
			x if x == OpcodeType::OP_POSTDEC as u16 => Ok(OpcodeType::OP_POSTDEC),
			x if x == OpcodeType::OP_MUL as u16 => Ok(OpcodeType::OP_MUL),
			x if x == OpcodeType::OP_DIV as u16 => Ok(OpcodeType::OP_DIV),
			x if x == OpcodeType::OP_MOD as u16 => Ok(OpcodeType::OP_MOD),
			x if x == OpcodeType::OP_ADD as u16 => Ok(OpcodeType::OP_ADD),
			x if x == OpcodeType::OP_SUB as u16 => Ok(OpcodeType::OP_SUB),
			x if x == OpcodeType::OP_SHL as u16 => Ok(OpcodeType::OP_SHL),
			x if x == OpcodeType::OP_SHR as u16 => Ok(OpcodeType::OP_SHR),
			x if x == OpcodeType::OP_USHR as u16 => Ok(OpcodeType::OP_USHR),
			x if x == OpcodeType::OP_LT as u16 => Ok(OpcodeType::OP_LT),
			x if x == OpcodeType::OP_GT as u16 => Ok(OpcodeType::OP_GT),
			x if x == OpcodeType::OP_LE as u16 => Ok(OpcodeType::OP_LE),
			x if x == OpcodeType::OP_GE as u16 => Ok(OpcodeType::OP_GE),
			x if x == OpcodeType::OP_EQ as u16 => Ok(OpcodeType::OP_EQ),
			x if x == OpcodeType::OP_NE as u16 => Ok(OpcodeType::OP_NE),
			x if x == OpcodeType::OP_STRICTEQ as u16 => Ok(OpcodeType::OP_STRICTEQ),
			x if x == OpcodeType::OP_STRICTNE as u16 => Ok(OpcodeType::OP_STRICTNE),
			x if x == OpcodeType::OP_JCASE as u16 => Ok(OpcodeType::OP_JCASE),
			x if x == OpcodeType::OP_BITAND as u16 => Ok(OpcodeType::OP_BITAND),
			x if x == OpcodeType::OP_BITXOR as u16 => Ok(OpcodeType::OP_BITXOR),
			x if x == OpcodeType::OP_BITOR as u16 => Ok(OpcodeType::OP_BITOR),
			x if x == OpcodeType::OP_INSTANCEOF as u16 => Ok(OpcodeType::OP_INSTANCEOF),
			x if x == OpcodeType::OP_THROW as u16 => Ok(OpcodeType::OP_THROW),
			x if x == OpcodeType::OP_TRY as u16 => Ok(OpcodeType::OP_TRY),
			x if x == OpcodeType::OP_ENDTRY as u16 => Ok(OpcodeType::OP_ENDTRY),
			x if x == OpcodeType::OP_CATCH as u16 => Ok(OpcodeType::OP_CATCH),
			x if x == OpcodeType::OP_ENDCATCH as u16 => Ok(OpcodeType::OP_ENDCATCH),
			x if x == OpcodeType::OP_WITH as u16 => Ok(OpcodeType::OP_WITH),
			x if x == OpcodeType::OP_ENDWITH as u16 => Ok(OpcodeType::OP_ENDWITH),
			x if x == OpcodeType::OP_DEBUGGER as u16 => Ok(OpcodeType::OP_DEBUGGER),
			x if x == OpcodeType::OP_JUMP as u16 => Ok(OpcodeType::OP_JUMP),
			x if x == OpcodeType::OP_JTRUE as u16 => Ok(OpcodeType::OP_JTRUE),
			x if x == OpcodeType::OP_JFALSE as u16 => Ok(OpcodeType::OP_JFALSE),
			x if x == OpcodeType::OP_RETURN as u16 => Ok(OpcodeType::OP_RETURN),
			x if x == OpcodeType::OP_LAST as u16 => Ok(OpcodeType::OP_LAST),
			_ => Err(()),
        }
    }
}


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
}

impl JsRuntime {
	/* environment's variables */
	pub fn delvariable(&mut self, name: &str) -> bool {
		let mut env: SharedScope = self.cenv.clone();
		loop {			
			let r = env.borrow().query_variable(name);
			if r {
				let prop = env.borrow_mut().variables.get_property(name);
				if !prop.configable() {
					return false;
				}
				env.borrow_mut().variables.drop_property(name);
				return true;
			}

			if env.borrow().outer.is_none() {
				return false;
			} 
			let r = env.borrow().fetch_outer();
			env = r; 
		}
	}

	pub fn getvariable(&mut self, name: &str) -> bool {
		let mut env: SharedScope = self.cenv.clone();
		loop {			
			let r = env.borrow().query_variable(name);
			if r {
				let prop = env.borrow_mut().variables.get_property(name);
				if prop.getter.is_some() {
					self.push_object(prop.getter.unwrap().clone());		// function object
					self.push(prop.value.clone());						// this object
					jscall(self, 0);
				} else {
					self.push(prop.value.clone());
				}
				return true;
			}
			if env.borrow().outer.is_none() {
				return false;
			} 
			let r = env.borrow().fetch_outer();
			env = r; 
		}
	}

	pub fn setvariable(&mut self, name: &str) {
		let mut env: SharedScope = self.cenv.clone();
		loop {
			let r = env.borrow().query_variable(name);
			if r {
				let prop = env.borrow_mut().variables.get_property(name);
				if prop.setter.is_some() {
					self.push_object(prop.setter.unwrap().clone());		// function object
					self.push(prop.value.clone());						// this object
					self.push_from( self.stack.len() - 3);				// value
					jscall(self, 1);
					self.pop(1);					
				} else {
					if !prop.readonly() {
						prop.value.swap( self.stack.first().unwrap().clone() );
					}
				}	
				return;		
			} 
			if env.borrow().outer.is_none() {
				break;
			} 
			let r = env.borrow().fetch_outer();
			env = r; 
		}
		
		let value = self.stack.first().unwrap().clone();
		self.genv.borrow_mut().variables.put_property(name);
		let mut prop = self.genv.borrow_mut().variables.get_property(name);
		prop.value = value;
		self.genv.borrow_mut().variables.set_property(name, prop);
	}

	/* properties operation */
    // make a new  or replace proptery o for object
    pub fn defproperty(&mut self, target_: SharedObject, name: &str, value: SharedValue,
		attr:JsPropertyAttr, getter: Option<SharedObject>, setter: Option<SharedObject>) {

		let mut target = target_.borrow_mut();

		match target.value {
			JsClass::object => {},
			_ => {
				println!("Cant define property for specia object!");
				return;
			}
		}

		if target.put_property(name) {
			let mut prop = target.get_property(name);
			if !prop.readonly() {
				if !value.is_undefined() {
					prop.value = value;
				}
			}
			if !prop.configable() {
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
			if attr != JsPropertyAttr::NONE {
				prop.attr = attr;
			}
			target.set_property(name, prop);
		}
	}

	// change value of the proptery for object
	pub fn setproperty(&mut self, target_: SharedObject, name: &str, value: SharedValue) {		
		let mut target = target_.borrow_mut();
		let target_ = target_.clone();

		match target.value {
			JsClass::object => {},
			_ => {
				println!("Cant write property for specia object!");
				return;
			}
		}

		let prop_r = target.query_property(name);
		if let Some((mut prop, own)) = prop_r {
			if let Some(setter) = prop.setter {
				self.push_object(setter.clone());
				self.push_object(target_);
				self.push(value);
				jscall(self, 1);
				self.pop(1);
				return;
			}
			if prop.readonly() {
				println!("Cant write property for specia object!");
				return;
			} else {				
				prop.value.swap( value );
				return;
			}
		}

		/* Property not found on this object, so create one */
		return self.defproperty(target_, name, value, JsPropertyAttr::NONE, None, None);		
	}	

	// get value from the proptery for object
	pub fn hasproperty(&mut self, target_: SharedObject, name: &str) -> bool {		
		let mut target = target_.borrow_mut();
		let target_ = target_.clone();

		match target.value {
			JsClass::string(ref s) => {
				if name == "length" {
					self.push_number( s.len() as f64);
					return true;
				} 
				if let Ok(idx) = name.parse::<usize>() {
					if idx < s.len() {
						self.push_string( s[idx..idx+1].to_string() ); 
						return true;
					}
				}
			},
			JsClass::array(ref v) => {
				if name == "length" {
					self.push_number( v.len() as f64);
					return true;
				} 
				if let Ok(idx) = name.parse::<usize>() {
					if idx < v.len() {
						self.push( v[idx].clone() );
						return true;
					}
				}
			},
			_ => {}
		}

		let prop_r = target.query_property(name);
		if let Some((mut prop, own)) = prop_r {
			if let Some(getter) = prop.getter {
				self.push_object(getter.clone());
				self.push_object(target_);
				jscall(self, 0);
			} else {
				self.push(prop.value.clone());
			}
			return true;
		}
		return false;
	}
	pub fn getproperty(&mut self, target: SharedObject, name: &str) {		
		if !self.hasproperty(target, name) {
			self.push_undefined();
		}
	}	

	/* convert object to string */
	pub fn as_string(&mut self, target: SharedValue) -> String {
		/* primitive value to string */
		if let Some(s) = target.to_string() {
			return s;
		}

		/* try to executing toString() */
		self.getproperty(target.get_object(), "toString");
		let object = self.top(-1);
		self.pop(1);
		if object.get_object().borrow().callable() {
			self.push(object);	// func
			self.push(target);	// this
			jscall(self, 0);
			let object = self.top(-1);
			self.pop(1);
			if let Some(s) = object.to_string() {
				return s;
			}
		}
		return "[object]".to_string();
	}

	/* stack operations */
	pub fn top(&self, offset: isize) -> SharedValue {
		if offset < 0 {
			let offset: usize = (-1 * offset) as usize;
			let pos = self.stack.len() + offset;
			return self.stack[pos].clone();
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
	pub fn push_from(&mut self, from: usize) {
		if from >= self.stack.len() {
			panic!("stack underflow! @ push_from");
		}
		let jv = SharedValue::clone( &self.stack[from] );
		self.stack.push(jv);
	}
	/* opcode helper*/
	pub fn pop(&mut self, mut n: usize) {
		if n > self.stack.len() {
			panic!("stack underflow! @ pop");
		}
		while n > 0 {
			self.stack.pop();
			n = n - 1;
		}
	}
	pub fn dup(&mut self) {
		if let Some(ref v) = self.stack.first() {
			let nv: SharedValue = SharedValue::clone(v);
			self.stack.push(nv);
		} else {
			panic!("stack underflow! @ dup");
		}
	}
	pub fn dup2(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ dup2");
		}

		let nv1: SharedValue = self.top(-2);
		let nv2: SharedValue = self.top(-1);
		self.stack.push(nv1);
		self.stack.push(nv2);
	}
	pub fn rot2(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ rot2");
		}
		/* A B -> B A */
		let top = self.stack.len();
		self.stack.swap(top-1, top-2);
	}
	pub fn rot3(&mut self) {
		if self.stack.len() < 3 {
			panic!("stack underflow! @ rot3");
		}
		/* A B C -> C A B */
		let top = self.stack.len();
		self.stack.swap(top-1, top-2);
		self.stack.swap(top-2, top-3);
	}
	pub fn rot4(&mut self) {
		if self.stack.len() < 4 {
			panic!("stack underflow! @ rot4");
		}
		/* A B C D -> D A B C */
		let top = self.stack.len();
		self.stack.swap(top-1, top-2);
		self.stack.swap(top-2, top-3);
		self.stack.swap(top-3, top-4);
	}
	pub fn rot3pop2(&mut self) {
		if self.stack.len() < 3 {
			panic!("stack underflow! @ rot3pop2");
		}
		/* A B C -> C */
		let top = self.stack.len();
		self.stack[top-3] = self.stack[top-1].clone(); 
		self.pop(2);
	}
	pub fn rot2pop1(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ rot3pop2");
		}
		/* A B -> B */
		let top = self.stack.len();
		self.stack[top-2] = self.stack[top-1].clone(); 
		self.pop(1);
	}
}

fn jsrun (rt: &mut JsRuntime, func: &VMFunction) {
	assert!(rt.stack.len() > 0);
	let mut pc:usize = 0;
	let bot:usize = rt.stack.len() - 1;

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

			OpcodeType::OP_THIS => {
				rt.push_from(bot);
			},
			OpcodeType::OP_CURRENT => {
				rt.push_from(bot - 1);
			},
			
			OpcodeType::OP_GETLOCAL => {
				let v = func.var(&mut pc);
				if rt.getvariable(&v) == false {
					println!("'{}' is not defined", v);
				}
			},
			OpcodeType::OP_SETLOCAL => {
				let v = func.var(&mut pc);
				rt.setvariable(v);
			},
			OpcodeType::OP_DELLOCAL => {
				let v = func.var(&mut pc);
				let r = rt.delvariable(v);
				rt.push_boolean(r);
			},

			OpcodeType::OP_GETVAR => {
				let s = func.string(&mut pc);
				if rt.getvariable(s) == false {
					println!("'{}' is not defined", s);
				}
			},
			OpcodeType::OP_HASVAR => {
				let s = func.string(&mut pc);
				if rt.getvariable(&s) == false {
					rt.push_undefined();
				}
			},
			OpcodeType::OP_SETVAR => {
				let s = func.string(&mut pc);
				rt.setvariable(s);
			},
			OpcodeType::OP_DELVAR => {
				let s = func.string(&mut pc);
				let r = rt.delvariable(s);
				rt.push_boolean(r);
			},
			
			OpcodeType::OP_INITPROP => {
				let target = rt.top(-3).get_object();
				let name = rt.as_string( rt.top(-2));
				let value = rt.top(-1);
				rt.setproperty(target, &name, value);
				rt.pop(2);
			},
			OpcodeType::OP_INITGETTER => {
				let target = rt.top(-3).get_object();
				let name = rt.as_string( rt.top(-2));
				let func = rt.top(-1);
				if func.is_object() {
					rt.defproperty(target, &name, SharedValue::new_undefined(), JsPropertyAttr::NONE, Some(func.get_object()), None);
				} else {
					println!("getter should be a object!");
				}
				rt.pop(2);
			},
			OpcodeType::OP_INITSETTER => {
				let target = rt.top(-3).get_object();
				let name = rt.as_string( rt.top(-2));
				let func = rt.top(-1);
				if func.is_object() {
					rt.defproperty(target, &name, SharedValue::new_undefined(), JsPropertyAttr::NONE, None, Some(func.get_object()));
				} else {
					println!("setter should be a object!");
				}
				rt.pop(2);
			},
			OpcodeType::OP_GETPROP => {
				let target = rt.top(-2).get_object();
				let name = rt.as_string( rt.top(-2));
				rt.getproperty(target, &name);
				rt.rot3pop2();
			},
			OpcodeType::OP_GETPROP_S => {
				let target = rt.top(-1).get_object();
				let name = func.string(&mut pc);
				rt.getproperty(target, &name);
				rt.rot2pop1();
			},

			_ => {}
		}
	}
}

fn jscall_script(rt: &mut JsRuntime, argc: usize) {
	let bot = rt.stack.len() - 1 - argc;

	let fobj = rt.stack[bot-1].get_object();
	let rfobj = fobj.borrow();
	let vmf = &rfobj.get_func().vmf;

	/* init var in current env*/
	for var in &vmf.var_tab {
		let jv = SharedValue::new_undefined();
		rt.cenv.borrow_mut().init_var(var, jv);
	}

	/* scripts take no arguments */
	rt.pop(argc);
	jsrun(rt, vmf);

	/* clear stack */
	let jv = rt.stack.pop().unwrap();
	rt.pop(2);
	rt.push(jv);
}

fn jscall_function(rt: &mut JsRuntime, argc: usize) {
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
		rt.defproperty(arg_value.get_object(), "length", jv,  JsPropertyAttr::DONTENUM, None, None);

		for i in 0..argc {
			let name = i.to_string();
			let jv = rt.stack[bot+1+i].clone();
			rt.defproperty(arg_value.get_object(), &name, jv, JsPropertyAttr::NONE, None, None);
		}

		
		rt.cenv.borrow_mut().init_var("arguments", arg_value);
	}

	/* setup remained arguments*/
	let min_argc = cmp::min(argc, vmf.numparams);
	for i in 0..min_argc {
		let argv = rt.stack[i + 1 + bot].clone();
		rt.cenv.borrow_mut().init_var(&vmf.var_tab[i], argv);
	}
	rt.pop(argc);

	/* init var in current env*/
	for i in min_argc..vmf.var_tab.len() {
		let jv = SharedValue::new_undefined();
		rt.cenv.borrow_mut().init_var(&vmf.var_tab[i], jv);
	}

	jsrun(rt, vmf);

	/* clear stack */
	let jv = rt.stack.pop().unwrap();
	rt.pop(2);
	rt.push(jv);

	/* restore old env */
	rt.cenv = old_env;
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
	rt.pop(argc + 2);
	rt.push(jv);
}

pub fn jscall(rt: &mut JsRuntime, argc: usize) {
	assert!(rt.stack.len() >= argc + 2);
	let bot = rt.stack.len() - 1 - argc;

	let fobj = rt.stack[bot-1].get_object();
	if fobj.borrow().is_function() == true {
		if fobj.borrow().get_func().vmf.script {
			jscall_script(rt, argc);
		} else {
			jscall_function(rt, argc);
		}
	} else if fobj.borrow().is_builtin() == true {
		jscall_builtin(rt, argc);
	} else {
        panic!("Can't call none function object");
    }
}
