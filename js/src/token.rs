use std::char;

/* token stuff */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
enum TokenType {
	/* general token */
    TK_EOF_ = -100,
	TK_COMMENT_,
    TK_SYMBOL_,
    TK_STRING_,
    TK_PUNCT_,

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
    TK_SEMICOLON,
    TK_COMMA,
    TK_POINT,
    TK_NEWLN,

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
	TK_AND,
	TK_OR,
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

struct Token {
    tk_type:     TokenType,
    tk_value:    Option<String>,
}

impl Token {
    fn new(tt: TokenType) -> Self {
        Token {
            tk_type: tt,
            tk_value: None
        }
    }

    fn new_with(tt: TokenType, value: String) -> Self {
        Token {
            tk_type: tt,
            tk_value: Some(value)
        }
    }
}

/// Parsing token from code string, return next token.
///
fn next_general_token (script: &String, cursor: usize) -> Result<(Token, usize), &'static str> {
    // local helper
	#[allow(non_camel_case_types)]
	enum ct {
		CT_LETTER,
		CT_SPACE,
        CT_NEWLN,
		CT_PUNCT,
		CT_EOF,
	}

    #[allow(non_camel_case_types)]
	#[derive(Clone, Debug, PartialEq)]
    enum ps {
		PS_NULL,
		PS_SYMBOL,
        PS_PUNCT,
        PS_STRING_SINGLE,
        PS_STRING_DOUBLE,
        PS_COMMENT_LINE,
		PS_COMMENT_BLOCK,
	}

    fn check_ct(chr: Option<char>) -> ct {
		if  chr == None  {
			return ct::CT_EOF;
		}
		let ch = chr.unwrap();

        if ch == ' ' || ch == '\t' || ch == '\r' {
			return ct::CT_SPACE;
		}
        if ch == '\n' {
            return ct::CT_NEWLN;
        }
        if ch == '.' || ch == '-' || ch == '+' || ch == '*' || ch == '/' || ch == '%' || ch == '=' || ch == ';' || ch == '\\'
            || ch == '&' || ch == '!' || ch == '|' || ch == '^' || ch == '!' || ch == '\'' || ch == '"' || ch == '\n'
            || ch == '<' || ch == '>' || ch == '(' || ch == ')' || ch == '[' || ch == ']' || ch == '{' || ch == '}' {
            return ct::CT_PUNCT;
        }
        return ct::CT_LETTER;
	}

    fn check_escape(c: char) -> char {
        if c == 't' {
            return '\t';
        }
        if c == 'n' {
            return '\n';
        }
        return c;
    }

    //
    // main code starting here
    //
    let code = &script[cursor..];
	let mut chars = code.chars();
	let mut pos = cursor;

	let mut ps = ps::PS_NULL;
	let mut tkbuf: Vec<char> = Vec::new();

    // executing token parsing LSM
	loop {
        let chr = chars.next();
		let ct = check_ct(chr);
        pos = pos + 1;

        // state handler
        if ps == ps::PS_NULL {
            match ct {
                ct::CT_EOF => {
                    let eof = Token::new(TokenType::TK_EOF_);
                    return Ok((eof, pos));
                },
                ct::CT_SPACE => {
                    continue;
                },
                ct::CT_NEWLN => {
                    let ln = Token::new_with(TokenType::TK_PUNCT_, String::from("\n"));
                    return Ok((ln, pos));
                },
                ct::CT_LETTER => {
                    tkbuf.push( chr.unwrap());
                    ps = ps::PS_SYMBOL;
                    continue;
                },
                ct::CT_PUNCT => {
                    let ch = chr.unwrap();
                    if ch == '\'' {
                        ps = ps::PS_STRING_SINGLE;
                        continue;
                    }
                    if ch == '"' {
                        ps = ps::PS_STRING_DOUBLE;
                        continue;
                    }
                    tkbuf.push(ch);
                    ps = ps::PS_PUNCT;
                    continue;
                },
            }
            panic!("FIXME, Can't run here!");
        }

        // state handler
        if ps == ps::PS_SYMBOL {
            match ct {
                ct::CT_EOF | ct::CT_SPACE => {
                    let value = tkbuf.into_iter().collect();
                    let symbol = Token::new_with(TokenType::TK_SYMBOL_, value);
                    return Ok((symbol, pos));
                },
                ct::CT_NEWLN => {
                    let value = tkbuf.into_iter().collect();
                    let symbol = Token::new_with(TokenType::TK_SYMBOL_, value);
                    return Ok((symbol, pos - 1));
                },
                ct::CT_LETTER => {
                    tkbuf.push( chr.unwrap());
                    continue;
                },
                ct::CT_PUNCT => {
                    let value = tkbuf.into_iter().collect();
                    let symbol = Token::new_with(TokenType::TK_SYMBOL_, value);
                    return Ok((symbol, pos-1));
                }
            }
            panic!("FIXME, Can't run here!");
        }

        // state handler
        if ps == ps::PS_STRING_SINGLE || ps == ps::PS_STRING_DOUBLE {
            match ct {
                ct::CT_EOF => {
                    return Err("Parsing string get end of file!");
                },
                ct::CT_NEWLN | ct::CT_LETTER | ct::CT_SPACE => {
                    tkbuf.push( chr.unwrap());
                    continue;
                },
                ct::CT_PUNCT => {
                    let ch = chr.unwrap();
                    if tkbuf.len() > 0 && tkbuf[tkbuf.len() - 1] == '\0' {
                        let last = tkbuf.len() - 1;
                        tkbuf[last] = check_escape( ch );
                        continue;
                    }
                    if ch == '\'' && ps == ps::PS_STRING_SINGLE {
                        let value = tkbuf.into_iter().collect();
                        let string = Token::new_with(TokenType::TK_STRING_, value);
                        return Ok((string, pos));
                    }
                    if ch == '"' && ps == ps::PS_STRING_DOUBLE {
                        let value = tkbuf.into_iter().collect();
                        let string = Token::new_with(TokenType::TK_STRING_, value);
                        return Ok((string, pos));
                    }
                    if ch == '\\' {
                         tkbuf.push( '\0' );
                         continue;
                    }
                    tkbuf.push( chr.unwrap());
                    continue;
                }
            }
            panic!("FIXME, Can't run here!");
        }

        // state handler
        if ps == ps::PS_COMMENT_BLOCK {
            match ct {
                ct::CT_EOF => {
                    return Err("Parsing block comment get end of file!");
                },
                ct::CT_PUNCT | ct::CT_LETTER | ct::CT_SPACE | ct::CT_NEWLN => {
                    tkbuf.push( chr.unwrap());

                    if tkbuf.len() >= 2 && tkbuf[tkbuf.len() - 2] == '*' && tkbuf[tkbuf.len() - 1] == '/' {
                        tkbuf.pop();
                        tkbuf.pop();
                        let value = tkbuf.into_iter().collect();
                        let comment = Token::new_with(TokenType::TK_COMMENT_, value);
                        return Ok((comment, pos));
                    }
                    continue;
                },
            }
            panic!("FIXME, Can't run here!");
        }

        // state handler
        if ps == ps::PS_COMMENT_LINE {
            match ct {
                ct::CT_EOF | ct::CT_NEWLN => {
                    let value = tkbuf.into_iter().collect();
                    let comment = Token::new_with(TokenType::TK_COMMENT_, value);
                    return Ok((comment, pos));
                },
                ct::CT_PUNCT | ct::CT_LETTER | ct::CT_SPACE => {
                    tkbuf.push( chr.unwrap());
                    continue;
                },
            }
            panic!("FIXME, Can't run here!");
        }

        // state handler

    }
    return Err("Hello");
}
