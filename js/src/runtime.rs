use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use std::ffi::c_void;

use crate::bytecode::*;

use crate::execute::*;
use crate::builtin::*;

// runtime stuff
pub type SharedObject = Rc<RefCell<JsObject>>;
pub type SharedScope = Rc<RefCell<JsEnvironment>>;
pub type SharedFunction = Rc<Box<VMFunction>>;

#[allow(non_snake_case)]
pub fn SharedObject_new(obj: JsObject) -> SharedObject {
	Rc::new(RefCell::new(obj))
}
#[allow(non_snake_case)]
pub fn SharedScope_new(scope: JsEnvironment) -> SharedScope {
	Rc::new(RefCell::new(scope))
}
#[allow(non_snake_case)]
pub fn SharedFunction_new(vmf: VMFunction) -> SharedFunction {
	Rc::new(Box::new(vmf))
}

// JsValue for access fast and memory effective 
// to simpilify implementation remvoed prototype for boolean/number
#[allow(non_camel_case_types)]
pub enum JsValue {
	JSUndefined,
	JSNULL,
	JSBoolean(bool),
	JSNumber(f64),	
	JSObject(SharedObject),
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct SharedValue {
	pub v:	Rc<RefCell<JsValue>>,
}

#[allow(non_camel_case_types)]
pub struct JsFunction {	
	pub vmf:	SharedFunction, 
	pub scope:	SharedScope,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct JsBuiltinFunction {
	pub f:		fn(&mut JsRuntime),
	pub argc:	usize,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct JsIterator {
	pub keys:	Vec<String>,
	pub index:	usize,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct JsException {
	pub msg:	String,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct JsExpander {
	pub ptr: *const c_void,
}

#[allow(non_camel_case_types)]
pub enum JsClass {
	object,
	expand(JsExpander),
	exception(JsException),
	iterator(JsIterator),
	string(String),
	array(Vec<SharedValue>),
	function(JsFunction),
	builtin(JsBuiltinFunction),
}

#[allow(non_camel_case_types)]
pub struct JsObject {
	pub __proto__:	Option<SharedObject>,
	pub extensible:	bool,
	pub properties: HashMap<String, JsProperty>,
	pub value:	JsClass,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct JsProperty {
	pub value:			SharedValue,
	pub getter:	Option<SharedObject>,
	pub setter:	Option<SharedObject>,

	// attribute flags
	pub attr_writable:		bool,
	pub attr_enumerable: 	bool,
	pub attr_configurable:	bool,
}

pub type JsPropertyAttr = (bool, bool, bool);	//writeable, enumerable, configurable 
pub const JS_DEFAULT_ATTR: JsPropertyAttr = (true, true, true);
pub const JS_READONLY_ATTR: JsPropertyAttr = (false, false, false);

#[allow(non_camel_case_types)]
pub struct JsEnvironment {
	pub variables: SharedObject,		// variables stored in properties 
	pub outer: Option<SharedScope>,
}

#[allow(non_camel_case_types)]
pub struct JsPrototype {
	/* prototype for different objects */
	pub object_prototype:	SharedObject,
	pub string_prototype:	SharedObject,
	pub array_prototype:	SharedObject,
	pub function_prototype: SharedObject,

	/* prototype for exceptions */
	pub exception_prototype: SharedObject,
}

#[allow(non_camel_case_types)]
pub struct JsRuntime {
	pub prototypes:		JsPrototype,

	pub genv:			SharedScope,	
	pub cenv:			SharedScope,

	pub stack:			Vec<SharedValue>,
}

pub fn new_runtime() -> JsRuntime {	
	let prototypes = JsPrototype {
		object_prototype:		SharedObject_new(JsObject::new()),
		string_prototype:		SharedObject_new(JsObject::new()),
		array_prototype:		SharedObject_new(JsObject::new()),
		function_prototype:		SharedObject_new(JsObject::new()),
		exception_prototype:	SharedObject_new(JsObject::new()),
	};

	let genv = JsEnvironment::new();
	let cenv = genv.clone();

	let mut runtime = JsRuntime {
		prototypes:	prototypes,
		genv:		genv,
		cenv:		cenv,
		stack:		Vec::new(),
	};

	// init prototypes
	prototypes_init(&mut runtime);
	builtin_init(&mut runtime);
	
	return runtime;
}

pub fn run_script(rt: &mut JsRuntime, vmf: SharedFunction) -> Result<SharedValue, String> {
	assert!( vmf.script == true);
	let fobj = SharedObject_new(JsObject::new_function(vmf, rt.genv.clone()));
	let thiz = rt.genv.borrow().target(); 

	rt.push_object(fobj);	// function object
	rt.push_object(thiz);	// this

	let result = jscall(rt, 0);
	if result.is_err() {
		let err_msg = format!("Exceptions: {:?}", result.err().unwrap());
		println!("{}", err_msg);
		rt.stack.clear();
		return Err(err_msg);
	}

	if rt.stack.len() != 1 {
		let err_msg = format!("stack len should be 1 but get {}", rt.stack.len());
		panic!(err_msg);
	}

	let value = rt.stack[0].clone();
	rt.stack.clear();
	return Ok(value);
}

