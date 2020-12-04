//
// common/shared/public struct/enum
//


/* token stuff */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
	/* immedial primitive */
	TK_IDENTIFIER = 0,
    TK_NUMBER,
	TK_STRING,
	TK_REGEXP,

	/* single-character punctuators */
    TK_BRACE_LEFT,
    TK_BRACE_RIGHT,
    TK_PAREN_LEFT,
    TK_PAREN_RIGHT,
    TK_BRACKET_LEFT,
    TK_BRACKET_RIGHT,

    TK_NEWLN,
    TK_SEMICOLON,
    TK_COMMA,
    TK_POINT,

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
    TK_LESS,
    TK_GREAT,

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
	TK_DEC,

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
	TK_WITH
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tk_type:    TokenType,
    pub tk_value:   Option<String>,
    pub src_line:   u32,
}


