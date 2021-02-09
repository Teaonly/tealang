use crate::common::*;
use crate::value::*;
use crate::execute::*;

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

	let runtime = JsRuntime {
		prototypes:	prototypes,
		genv:		genv,
		cenv:		cenv,
		stack:		Vec::new(),
	};

	return runtime;
}

pub fn run_script(rt: &mut JsRuntime, vmf: SharedFunction) {
	let fobj = JsObject::new_function(vmf, rt.genv.clone());

	
}

