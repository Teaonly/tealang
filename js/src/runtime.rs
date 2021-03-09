use crate::common::*;
use crate::value::*;
use crate::execute::*;
use crate::builtin::*;

// global functions for runtime 
fn assert(rt: &mut JsRuntime) {    
    let b = rt.top(-2).to_boolean();
    if !b {
        let info = rt.top(-1).to_string();
        panic!("ASSERT: {}", info);
    }
    rt.push_undefined();
}

fn println(rt: &mut JsRuntime) {
	let info = rt.to_string( rt.top(-1) );
	if let Ok(msg) = info {
    	println!("{}", msg);
    	rt.push_undefined();
		return;
	} 
	if let Err(e) = info {
		rt.new_exception(e);
	}
}

// TODO : isFinite() isNaN() parseFloat() parseInt()

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

	// some basic utilities
	runtime.genv.borrow_mut().init_var("assert", SharedValue::new_object(JsObject::new_builtin(assert, 2)) );
    runtime.genv.borrow_mut().init_var("println", SharedValue::new_object(JsObject::new_builtin(println, 1)) );

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

