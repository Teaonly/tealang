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
	pub fn string(&self, pc:&mut usize) -> String {
		if *pc >= self.code.len() {
			panic!("fetch raw out of code");
		}
		let id = self.code[*pc] as usize;
		if id > self.str_tab.len() {
			panic!("string out of vm");
		}
		let value = String::clone(&self.str_tab[id]);

		*pc = *pc + 1;
		return value;
	}
}



impl JsRuntime {
	/* properties operation */

    // make a new proptery for object
    pub fn defproperty(&mut self, target: &mut JsObject, name: &str, value: JsValue,
		attr:JsPropertyAttr, getter: Option<SharedObject>, setter: Option<SharedObject>) {

		fn goto_readonly() {
			println!("Hello");
		}
		
		if let JsClass::array(ref _v) = target.value {
			if name == "length" {
				goto_readonly();
			}
		} else if let JsClass::string(ref s) = target.value {
			if name == "length" {
				goto_readonly();
			}
			if let Ok(index) = name.parse::<usize>() {
				if index > s.len() {
					goto_readonly();
				}
			}			
		}

		
		
	}

	pub fn setproperty(&mut self, target: &mut JsObject, name: &str, value: JsValue) {

	}

	/* stack operations */
	pub fn push(&mut self, jv: JsValue) {
		self.stack.push(jv);
	}
	pub fn push_undefined(&mut self) {
		let jv = JsValue::new_undefined();
		self.stack.push(jv);
	}
	pub fn push_number(&mut self, v:f64) {
		let jv = JsValue::new_number(v);
		self.stack.push(jv);
	}
	pub fn push_string(&mut self, v:String) {
		let jv = JsValue::new_string(v);
		self.stack.push(jv);
	}
	pub fn push_from(&mut self, from: usize) {
		if from >= self.stack.len() {
			panic!("stack underflow! @ push_from");
		}
		let jv = JsValue::clone( &self.stack[from] );
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
			let nv: JsValue = JsValue::clone(v);
			self.stack.push(nv);
		} else {
			panic!("stack underflow! @ dup");
		}
	}
	pub fn dup2(&mut self) {
		if self.stack.len() < 2 {
			panic!("stack underflow! @ dup2");
		}

		let top = self.stack.len();
		let nv1: JsValue = JsValue::clone( &self.stack[top-2] );
		let nv2: JsValue = JsValue::clone( &self.stack[top-1] );
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
				rt.push(JsValue::new_undefined());
			},
			OpcodeType::OP_NULL => {
				rt.push(JsValue::new_null());
			},
			OpcodeType::OP_FALSE => {
				rt.push(JsValue::new_false());
			},
			OpcodeType::OP_TRUE => {
				rt.push(JsValue::new_false());
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
				rt.push_string(v);
			},

			OpcodeType::OP_THIS => {
				rt.push_from(bot);
			},
			OpcodeType::OP_CURRENT => {
				rt.push_from(bot - 1);
			},

			/*
			// TODO
			OpcodeType::OP_GETLOCAL => {
				let v = func.string(&mut pc);

			}
			*/
			_ => {}
		}
	}
}

fn jscall_script(rt: &mut JsRuntime, argc: usize) {
	let bot = rt.stack.len() - 1 - argc;

	let fobj = rt.stack[bot-1].as_object();
	let rfobj = fobj.borrow();
	let vmf = &rfobj.get_func().vmf;

	/* init var in current env*/
	for var in &vmf.var_tab {
		let jv = JsValue::new_undefined();
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

	let fobj = rt.stack[bot-1].as_object();
	let rfobj = fobj.borrow();
	let vmf = &rfobj.get_func().vmf;

	/* create new scope */
	let new_env = JsEnvironment::new_from(rfobj.get_func().scope.clone());
	let old_env = rt.cenv.clone();
	rt.cenv = new_env;

	/* create arguments */
	if vmf.numparams > 0 {
		let mut arg_obj = JsObject::new_with_class( rt.prototypes.object_prototype.clone(), JsClass::object);

		let jv = JsValue::new_number(argc as f64);
		rt.defproperty(&mut arg_obj, "length", jv,  JsPropertyAttr::DONTENUM, None, None);

		for i in 0..argc {
			let name = i.to_string();
			let jv = rt.stack[bot+1+i].clone();
			rt.defproperty(&mut arg_obj, &name, jv, JsPropertyAttr::NONE, None, None);
		}

		let arg_value = JsValue::new_object(arg_obj);
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
		let jv = JsValue::new_undefined();
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
	let fobj = rt.stack[bot-1].as_object();
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

	let fobj = rt.stack[bot-1].as_object();
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
