use crate::common::*;
use crate::value::*;
use crate::execute::*;
use crate::builtin::*;

pub fn new_runtime() -> JsRuntime {
	let obj = SharedObject_new(JsObject::new());

	let prototypes = JsPrototype {
		object_prototype:	obj.clone(),
		string_prototype:	obj.clone(),
		array_prototype:	obj.clone(),
		function_prototype:	obj.clone(),		
	};

	let genv = JsEnvironment::new();
	let cenv = genv.clone();

	let mut runtime = JsRuntime {
		prototypes:	prototypes,
		genv:		genv,
		cenv:		cenv,
		stack:		Vec::new(),
	};

	builtin_init(&mut runtime);

	return runtime;
}

pub fn run_script(rt: &mut JsRuntime, vmf: SharedFunction) {
	assert!( vmf.script == true);
	let fobj = SharedObject_new(JsObject::new_function(vmf, rt.genv.clone()));
	let thiz = rt.genv.borrow().target(); 

	rt.push_object(fobj);	// function object
	rt.push_object(thiz);	// this

	jscall(rt, 0);

	if rt.stack.len() != 1 {
		println!("stack len should be 1 but get {}", rt.stack.len());
	}
}

