use std::convert::TryFrom;
use std::cell::Cell;
use std::rc::Rc;
use std::collections::HashMap;

use crate::common::*;
use crate::value::*;

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


pub fn new_runtime() {
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
	panic!("TODO")
}