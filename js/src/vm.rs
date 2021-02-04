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
			x if x == OpcodeType::OP_DEBUGGER as u16 => Ok(OpcodeType::OP_DEBUGGER),
			x if x == OpcodeType::OP_JUMP as u16 => Ok(OpcodeType::OP_JUMP),
			x if x == OpcodeType::OP_JTRUE as u16 => Ok(OpcodeType::OP_JTRUE),
			x if x == OpcodeType::OP_JFALSE as u16 => Ok(OpcodeType::OP_JFALSE),
			x if x == OpcodeType::OP_RETURN as u16 => Ok(OpcodeType::OP_RETURN),
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
	pub fn delproperty(&mut self, target_: SharedObject, name: &str) -> bool {		
		let mut target = target_.borrow_mut();
		let target_ = target_.clone();

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
	pub fn concat_add(&mut self) {
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
	pub fn equal(&mut self) -> bool {
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

	pub fn strict_equal(&mut self) -> bool {
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

	pub fn compare_item(&mut self) -> Option<i32> {
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
		return None;
	}

	pub fn instanceof(&mut self) -> bool {
		let x = self.top(-2);
		let y = self.top(-1);
		self.pop(2);
		
		if !x.is_object() {
			return false;
		}
		if !y.is_object() {
			println!("instanceof: invalid operand");
			self.push_boolean(false);
			return false;
		}
		let mut x = x.get_object();
		let y = y.get_object();
		if !y.borrow().callable() {
			println!("instanceof: invalid operand");
			self.push_boolean(false);
			return false;
		}

		self.getproperty(y, "prototype");
		let o = self.top(-1);
		self.pop(1);
		if !o.is_object() {			
			println!("instanceof: 'prototype' property is not an object");
			self.push_boolean(false);
			return false;
		}
		let o = o.get_object();

		loop {
			let proto = x.borrow().prototype.clone();
			if let Some( proto ) = proto {
				x = proto;
				if o.as_ptr() == x.as_ptr() {
					self.push_boolean(true);
					return true;
				}
			} else {
				break;
			}
		}

		self.push_boolean(false);
		return false;
	}

	/* convert object to string */
	pub fn to_string(&mut self, target: SharedValue) -> String {
		
		/* try to executing toString() */
		self.getproperty(target.get_object(), "toString");
		let object = self.top(-1);
		self.pop(1);
		if object.is_object() {
			if object.get_object().borrow().callable() {
				self.push(object);	// func
				self.push(target);	// this
				jscall(self, 0);
				let str_result = self.top(-1);
				self.pop(1);
				return str_result.to_string();
			}
		}

		return target.to_string();
		
	}

	/* create new object */
	pub fn new_call(&mut self, argc: usize) {
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
			return;
		}
		
		/* extract the function object's prototype property */
		self.getproperty(obj, "prototype");
		
		let proto = if self.top(-1).is_object() {
			self.top(-1).get_object()
		} else {
			self.prototypes.object_prototype.clone()
		};
		
		self.pop(1);

		/* create a new object with above prototype, and shift it into the 'this' slot */
		let mut nobj = JsObject::new();
		nobj.prototype = Some(self.prototypes.object_prototype.clone());
		let nobj = SharedObject_new(nobj);
		self.push_object(nobj.clone());
		if argc > 0 {
			self.rot(argc+1);				
		}

		/* call the function */
		jscall(self, argc);

		/* if result is not an object, return the original object we created */
		if !self.top(-1).is_object() {
			self.pop(1);
			self.push_object(nobj);
		}		
	}

	pub fn new_closure(&mut self, f: SharedFunction) {
		let fobj = SharedObject_new(JsObject::new_function(f.clone(), self.cenv.clone()));		

		let v = SharedValue::new_number(f.numparams as f64);
		self.defproperty(fobj, "length", v,  JsPropertyAttr::READONLY_DONTENUM_DONTCONF, None, None);		
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
	pub fn rot(&mut self, n: usize) {
		if self.stack.len() < n {
			panic!("stack underflow! @ rot");
		}
		let top = self.stack.len();
		for i in 0..n-1 {
			self.stack.swap(top-1, top-2);
		}
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
				let name = rt.to_string( rt.top(-2));
				let value = rt.top(-1);
				rt.setproperty(target, &name, value);
				rt.pop(2);
			},
			OpcodeType::OP_INITGETTER => {
				let target = rt.top(-3).get_object();
				let name = rt.to_string( rt.top(-2));
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
				let name = rt.to_string( rt.top(-2));
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
				let name = rt.to_string( rt.top(-1));
				rt.getproperty(target, &name);
				rt.rot3pop2();
			},
			OpcodeType::OP_GETPROP_S => {
				let target = rt.top(-1).get_object();
				let name = func.string(&mut pc);
				rt.getproperty(target, &name);
				rt.rot2pop1();
			},
			OpcodeType::OP_SETPROP => {
				let target = rt.top(-3).get_object();
				let name = rt.to_string( rt.top(-2));
				let value = rt.top(-1);
				rt.setproperty(target, &name, value);
				rt.rot3pop2();
			},
			OpcodeType::OP_SETPROP_S => {
				let target = rt.top(-2).get_object();
				let value = rt.top(-1);
				let name = func.string(&mut pc);
				rt.setproperty(target, &name, value);
				rt.rot2pop1();
			},
			OpcodeType::OP_DELPROP => {
				let target = rt.top(-2).get_object();
				let name = rt.to_string( rt.top(-1));
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
				jscall(rt, n);
			},
			OpcodeType::OP_NEW => {
				let n = func.int(&mut pc) as usize;
				rt.new_call(n);
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
				rt.push_boolean(n);
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
				let x = rt.top(-1).to_number() as i32;
				let y = rt.top(-1).to_number() as u32;
				rt.pop(2);
				rt.push_number( (x << (y&0x1F)) as f64);	
			},
			OpcodeType::OP_SHR => {
				let x = rt.top(-1).to_number() as i32;
				let y = rt.top(-1).to_number() as u32;
				rt.pop(2);
				rt.push_number( (x >> (y&0x1F)) as f64);	
			},
			OpcodeType::OP_USHR => {
				let x = rt.top(-1).to_number() as u32;
				let y = rt.top(-1).to_number() as u32;
				rt.pop(2);
				rt.push_number( (x >> (y&0x1F)) as f64);	
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
				rt.instanceof();
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
				rt.push_boolean(b);
			},
			OpcodeType::OP_STRICTNE => {
				let b = rt.strict_equal();
				rt.push_boolean(!b);
			},

			/* Binary bitwise operators */
			OpcodeType::OP_BITAND => {
				let x = rt.top(-1).to_number() as i32;
				let y = rt.top(-1).to_number() as i32;
				rt.pop(2);
				rt.push_number( (x & y) as f64);	
			},
			OpcodeType::OP_BITXOR => {
				let x = rt.top(-1).to_number() as i32;
				let y = rt.top(-1).to_number() as i32;
				rt.pop(2);
				rt.push_number( (x ^ y) as f64);	
			},
			OpcodeType::OP_BITOR => {
				let x = rt.top(-1).to_number() as i32;
				let y = rt.top(-1).to_number() as i32;
				rt.pop(2);
				rt.push_number( (x | y) as f64);	
			},

			/* Try and Catch */		
			// TODO	
			OpcodeType::OP_TRY => {
			},
			OpcodeType::OP_ENDTRY => {
				
			},
			OpcodeType::OP_CATCH => {
				let catch_varname = func.string(&mut pc);
				let catch_value = rt.top(-1);
				rt.pop(1);

				let new_env = JsEnvironment::new_from(rt.cenv.clone());
				new_env.borrow_mut().init_var(catch_varname, catch_value);
				rt.cenv = new_env;				
			},
			OpcodeType::OP_ENDCATCH => {
				let outer: SharedScope = rt.cenv.borrow().outer.as_ref().unwrap().clone();
				rt.cenv = outer;
			},
			OpcodeType::OP_THROW => {
				
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
				return;
			},

			/* do nothing */
			OpcodeType::OP_DEBUGGER => {},			
			OpcodeType::OP_EVAL => {},
			OpcodeType::OP_NOP => {},
			OpcodeType::OP_LAST => {},
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
