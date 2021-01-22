//
// common/shared/public struct/enum
//

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/* token stuff */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
	TK_EOF = -1,

	/* immedial primitive */
	TK_IDENTIFIER = 0,
    TK_NUMBER,
	TK_STRING,

	/* keywords */
	TK_BREAK,
	TK_CASE,
	TK_CATCH,
	TK_CONTINUE,
	TK_DEBUGGER,
	TK_DEFAULT,
	TK_DELETE,
	TK_DO,
	TK_ELSE,
	TK_FALSE,
	TK_FINALLY,
	TK_FOR,
	TK_FUNCTION,
	TK_IF,
	TK_IN,
	TK_INSTANCEOF,
	TK_NEW,
	TK_NULL,
	TK_RETURN,
	TK_SWITCH,
	TK_THIS,
	TK_THROW,
	TK_TRUE,
	TK_TRY,
	TK_TYPEOF,
	TK_VAR,
	TK_VOID,
	TK_WHILE,
	TK_WITH,

	/* single-character punctuators */
    TK_BRACE_LEFT,		// {}
    TK_BRACE_RIGHT,
    TK_PAREN_LEFT,		// ()
    TK_PAREN_RIGHT,
    TK_BRACKET_LEFT,	// []
    TK_BRACKET_RIGHT,

    TK_NEWLN,
    TK_SEMICOLON,
    TK_COMMA,
	TK_POINT,
	TK_QUEST,
	TK_COLON,

    TK_ASS,
    TK_ADD,
    TK_SUB,
    TK_MUL,
    TK_DIV,
    TK_MOD,
    TK_NOT,
    TK_AND,
    TK_OR,
	TK_XOR,
	TK_BITNOT,
    TK_LT,
	TK_GT,

	/* multi-character punctuators */
	TK_LE,
	TK_GE,
	TK_EQ,
	TK_NE,
	TK_STRICTEQ,
	TK_STRICTNE,
	TK_SHL,
	TK_SHR,
	TK_USHR,
	TK_AND_AND,
	TK_OR_OR,
	TK_ADD_ASS,
	TK_SUB_ASS,
	TK_MUL_ASS,
	TK_DIV_ASS,
	TK_MOD_ASS,
	TK_SHL_ASS,
	TK_SHR_ASS,
	TK_USHR_ASS,
	TK_AND_ASS,
	TK_OR_ASS,
	TK_XOR_ASS,
	TK_INC,
	TK_DEC
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tk_type:    TokenType,
    pub tk_value:   Option<String>,
    pub src_line:   u32,
}

/* ast stuff */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AstType {
	AST_NULL = -1,

    AST_LIST = 0,
	AST_FUNDEC,
	AST_IDENTIFIER,

	EXP_IDENTIFIER,
	EXP_NUMBER,
	EXP_STRING,

	/* literals */
	EXP_UNDEF, /* for array elisions */
	EXP_NULL,
	EXP_TRUE,
	EXP_FALSE,
	EXP_THIS,

	EXP_ARRAY,
	EXP_OBJECT,
	EXP_PROP_VAL,
	EXP_PROP_GET,
	EXP_PROP_SET,

	EXP_FUN,

	/* expressions */
	EXP_INDEX,
	EXP_MEMBER,
	EXP_CALL,
	EXP_NEW,

	EXP_POSTINC,
	EXP_POSTDEC,

	EXP_DELETE,
	EXP_VOID,
	EXP_TYPEOF,
	EXP_PREINC,
	EXP_PREDEC,
	EXP_POS,
	EXP_NEG,
	EXP_BITNOT,
	EXP_LOGNOT,

	EXP_MOD,
	EXP_DIV,
	EXP_MUL,
	EXP_SUB,
	EXP_ADD,
	EXP_USHR,
	EXP_SHR,
	EXP_SHL,
	EXP_INSTANCEOF,
	EXP_GE,
	EXP_LE,
	EXP_GT,
	EXP_LT,
	EXP_STRICTNE,
	EXP_STRICTEQ,
	EXP_NE,
	EXP_EQ,
	EXP_BITAND,
	EXP_BITXOR,
	EXP_BITOR,
	EXP_LOGAND,
	EXP_LOGOR,

	EXP_COND,

	EXP_ASS,
	EXP_ASS_MUL,
	EXP_ASS_DIV,
	EXP_ASS_MOD,
	EXP_ASS_ADD,
	EXP_ASS_SUB,
	EXP_ASS_SHL,
	EXP_ASS_SHR,
	EXP_ASS_USHR,
	EXP_ASS_BITAND,
	EXP_ASS_BITXOR,
	EXP_ASS_BITOR,

	EXP_COMMA,

	EXP_VAR, /* var initializer */

	/* statements */
	STM_BLOCK,
	STM_EMPTY,
	STM_VAR,
	STM_IF,
	STM_DO,
	STM_WHILE,
	STM_FOR,
	STM_FOR_VAR,
	STM_FOR_IN,
	STM_FOR_IN_VAR,
	STM_CONTINUE,
	STM_BREAK,
	STM_RETURN,
	STM_SWITCH,
	STM_THROW,
	STM_TRY,
	STM_DEBUGGER,

	STM_LABEL,
	STM_CASE,
	STM_DEFAULT,
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub ast_type:   AstType,
    pub src_line:   u32,
    pub num_value:  Option<f64>,
	pub str_value:  Option<String>,

    pub a:      Option<Box<AstNode>>,
    pub b:      Option<Box<AstNode>>,
    pub c:      Option<Box<AstNode>>,
    pub d:      Option<Box<AstNode>>,
}

/* bytecode stuff */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpcodeType {
	OP_NOP = 0,
	OP_POP = 1,	/* A -- */
	OP_DUP,		/* A -- A A */
	OP_DUP2,	/* A B -- A B A B */
	OP_ROT2,	/* A B -- B A */
	OP_ROT3,	/* A B C -- C A B */
	OP_ROT4,	/* A B C D -- D A B C */

	OP_INTEGER,	/* -K- (number-32768) */
	OP_NUMBER,	/* -N- <number> */
	OP_STRING,	/* -S- <string> */
	OP_CLOSURE,	/* -F- <closure> */

	OP_NEWARRAY,
	OP_NEWOBJECT,
	OP_NEWREGEXP,	/* -S,opts- <regexp> */

	OP_UNDEF,
	OP_NULL,
	OP_TRUE,
	OP_FALSE,

	OP_THIS,
	OP_CURRENT,	/* currently executing function object */

	OP_GETLOCAL,	/* -K- <value> */
	OP_SETLOCAL,	/* <value> -K- <value> */
	OP_DELLOCAL,	/* -K- false */

	OP_HASVAR,	/* -S- ( <value> | undefined ) */
	OP_GETVAR,	/* -S- <value> */
	OP_SETVAR,	/* <value> -S- <value> */
	OP_DELVAR,	/* -S- <success> */


	OP_INITPROP,	/* <obj> <key> <val> -- <obj> */
	OP_INITGETTER,	/* <obj> <key> <closure> -- <obj> */
	OP_INITSETTER,	/* <obj> <key> <closure> -- <obj> */

	OP_GETPROP,	/* <obj> <name> -- <value> */
	OP_GETPROP_S,	/* <obj> -S- <value> */
	OP_SETPROP,	/* <obj> <name> <value> -- <value> */
	OP_SETPROP_S,	/* <obj> <value> -S- <value> */
	OP_DELPROP,	/* <obj> <name> -- <success> */
	OP_DELPROP_S,	/* <obj> -S- <success> */

	OP_ITERATOR,	/* <obj> -- <iobj> */
	OP_NEXTITER,	/* <iobj> -- ( <iobj> <name> true | false ) */

	OP_EVAL,	/* <args...> -(numargs)- <returnvalue> */
	OP_CALL,	/* <closure> <this> <args...> -(numargs)- <returnvalue> */
	OP_NEW,		/* <closure> <args...> -(numargs)- <returnvalue> */

	OP_TYPEOF,
	OP_POS,
	OP_NEG,
	OP_BITNOT,
	OP_LOGNOT,
	OP_INC,		/* <x> -- ToNumber(x)+1 */
	OP_DEC,		/* <x> -- ToNumber(x)-1 */
	OP_POSTINC,	/* <x> -- ToNumber(x)+1 ToNumber(x) */
	OP_POSTDEC,	/* <x> -- ToNumber(x)-1 ToNumber(x) */

	OP_MUL,
	OP_DIV,
	OP_MOD,
	OP_ADD,
	OP_SUB,
	OP_SHL,
	OP_SHR,
	OP_USHR,
	OP_LT,
	OP_GT,
	OP_LE,
	OP_GE,
	OP_EQ,
	OP_NE,
	OP_STRICTEQ,
	OP_STRICTNE,
	OP_JCASE,
	OP_BITAND,
	OP_BITXOR,
	OP_BITOR,

	OP_INSTANCEOF,

	OP_THROW,

	OP_TRY,		/* -ADDR- /jump/ or -ADDR- <exception> */
	OP_ENDTRY,

	OP_CATCH,	/* push scope chain with exception variable */
	OP_ENDCATCH,

	OP_WITH,
	OP_ENDWITH,

	OP_DEBUGGER,
	OP_JUMP,
	OP_JTRUE,
	OP_JFALSE,
	OP_RETURN,

	OP_LAST,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum VMJumpType {
	BreakJump(usize),
	ContinueJump(usize),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum VMJumpScope {
	TryScope(Option<AstNode>),
	CatchScope,
	SwitchScope,
	ForLoop,
	ForInLoop,
	DoLoop,
	WhileLoop,
	LabelSection(String),
}

#[allow(non_camel_case_types)]
pub struct VMJumpTable {
	pub scope:	VMJumpScope,
	pub lst: 	Vec<VMJumpType>
}

#[allow(non_camel_case_types)]
pub struct VMFunction {	
	pub script:		bool,
	pub numparams:	usize,
	pub code:		Vec<u16>,

	pub num_tab:	Vec<f64>,
	pub str_tab:	Vec<String>,
	pub var_tab:	Vec<String>,
	pub func_tab:	Vec<Box<VMFunction>>,

	pub jumps:		Vec<VMJumpTable>,
}

// runtime stuff
pub type SharedObject = Rc<RefCell<JsObject>>;
pub type SharedScope = Rc<RefCell<JsEnvironment>>;

pub fn SharedObject_new(obj: JsObject) -> SharedObject {
	Rc::new(RefCell::new(obj))
}
pub fn SharedScope_new(scope: JsEnvironment) -> SharedScope {
	Rc::new(RefCell::new(scope))
}

// JsValue for access fast and memory effective 
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum JsValue {
	JSUndefined,
	JSNULL,
	JSBoolean(bool),
	JSNumber(f64),
	JSString(String),
	JSObject(SharedObject),
}

#[allow(non_camel_case_types)]
pub struct JsFunction {
	pub scope:	SharedScope,
	pub vmf:	VMFunction, 
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct JsNatveFunction {
	pub f:		fn(&mut JsRuntime),
	pub argc:	usize,
}

#[allow(non_camel_case_types)]
pub enum JsClass {
	object,
	boolean(bool),
	number(f64),
	string(String),
	array(Vec<JsValue>),
	function(JsFunction),
	native(JsNatveFunction),
}

#[allow(non_camel_case_types)]
pub struct JsObject {
	pub prototype:	Option<SharedObject>,
	pub properties: HashMap<String, JsProperty>,
	pub value:	JsClass,
}

/* Property attribute flags */
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum JsPropertyAttr {
	NONE,
	READONLY,
	DONTENUM,
	DONTCONF,
	READONLY_DONTENUM,
	READONLY_DONTCONF,
	DONTENUM_DONTCONF,
	READONLY_DONTENUM_DONTCONF,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct JsProperty {
	pub value:	JsValue,
	pub attr:	JsPropertyAttr,
	pub getter:	Option<SharedObject>,
	pub setter:	Option<SharedObject>,
}

#[allow(non_camel_case_types)]
pub struct JsEnvironment {
	pub variables: JsObject,		// variables stored in properties 
	pub outer: Option<SharedScope>,
}

#[allow(non_camel_case_types)]
pub struct JsPrototype {
	/* prototype for different objects */
	pub object_prototype:	SharedObject,
	pub array_prototype:	SharedObject,
	pub function_prototype: SharedObject,
	pub boolean_prototype:	SharedObject,
	pub number_prototype:	SharedObject,
	pub string_prototype:	SharedObject,
	
	pub error_prototype:	SharedObject,
	pub range_err_proto:	SharedObject,
	pub ref_err_proto:		SharedObject,
	pub syntax_err_proto:	SharedObject,
	pub type_err_proto:		SharedObject,
}

#[allow(non_camel_case_types)]
pub struct JsRuntime {
	pub prototypes:		JsPrototype,

	pub genv:			SharedScope,	
	pub cenv:			SharedScope,

	pub stack:			Vec<JsValue>,
}
