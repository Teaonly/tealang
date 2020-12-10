use std::cell::RefCell;
use std::rc::Rc;
use std::cell::RefMut;

use crate::common::*;
use crate::token::*;

/* Local help function */
impl AstNode {
    fn new(ntype: AstType, line: u32) -> Self {
        AstNode {
            ast_type:  ntype,
            src_line:  line, 
            num_value: None,
            str_value: None,
            a: None,
            b: None,
            c: None,
            d: None
        }
    }

    fn new_number(ntype: AstType, line: u32, num: f64) -> Self {
        AstNode {
            ast_type: ntype,
            src_line: line, 
            num_value: Some(num),
            str_value: None,
            a: None,
            b: None,
            c: None,
            d: None
        }        
    }

    fn new_string(ntype: AstType, line: u32, string: &str) -> Self {
        AstNode {
            ast_type: ntype,
            src_line: line, 
            num_value: None,
            str_value: Some(String::from(string)),
            a: None,
            b: None,
            c: None,
            d: None
        }        
    }

    fn new_a(ntype: AstType, line: u32, a: Self) -> Self {
        AstNode {
            ast_type: ntype,
            src_line: line, 
            num_value: None,
            str_value: None,
            a: Some(Box::new(a)),
            b: None,
            c: None,
            d: None
        }          
    }

    fn new_a_b(ntype: AstType, line: u32, a: Self, b: Self) -> Self {
        AstNode {
            ast_type: ntype,
            src_line: line, 
            num_value: None,
            str_value: None,
            a: Some(Box::new(a)),
            b: Some(Box::new(b)),
            c: None,
            d: None
        }          
    }

    fn new_a_b_c(ntype: AstType, line: u32, a: Self, b: Self, c: Self) -> Self {
        AstNode {
            ast_type: ntype,
            src_line: line, 
            num_value: None,
            str_value: None,
            a: Some(Box::new(a)),
            b: Some(Box::new(b)),
            c: Some(Box::new(c)),
            d: None
        }          
    }

    // linked list
    fn new_list(anode: AstNode) -> Self {
        let mut new_list_item = AstNode::new(AstType::AST_LIST, anode.src_line);
        new_list_item.a = Some(Box::new(anode));
        return new_list_item;
    }
    fn list_tail_push(&mut self, anode: AstNode) {
        assert!(self.ast_type == AstType::AST_LIST);
        assert!(self.b.is_none());
        let mut new_list_item = AstNode::new(AstType::AST_LIST, anode.src_line);
        new_list_item.a = Some(Box::new(anode));
        self.b = Some(Box::new( new_list_item ));
    }
}

fn tk_accept(tkr: &mut Tokenlizer, tkt: TokenType) -> Result<bool, String> {
    let ntk = tkr.forward()?;
    if ntk.tk_type != tkt {
        return Ok(false);
    }
    tkr.next()?;
    return Ok(true);
}

fn tk_except(tkr: &mut Tokenlizer, tkt: TokenType) -> Result<Token, String> {
    let ntk = tkr.next()?;
    if ntk.tk_type != tkt {
        return Err(format!("AST error: except {:?} but got {:?} @ {}", tkt, ntk.tk_type, ntk.src_line));
    }
    return Ok(ntk);
}

fn ast_identifier(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let id = tk_except(tkr, TokenType::TK_IDENTIFIER)?;
    let node = AstNode::new_string(AstType::AST_IDENTIFIER, id.src_line, &id.tk_value.unwrap());
    return Ok(node);
}

fn ast_assignment(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    
    
    /*
    js_Ast *a = conditional(J, notin);
	int line = J->lexline;
	INCREC();
	if (jsP_accept(J, '=')) a = EXP2(ASS, a, assignment(J, notin));
	else if (jsP_accept(J, TK_MUL_ASS)) a = EXP2(ASS_MUL, a, assignment(J, notin));
	else if (jsP_accept(J, TK_DIV_ASS)) a = EXP2(ASS_DIV, a, assignment(J, notin));
	else if (jsP_accept(J, TK_MOD_ASS)) a = EXP2(ASS_MOD, a, assignment(J, notin));
	else if (jsP_accept(J, TK_ADD_ASS)) a = EXP2(ASS_ADD, a, assignment(J, notin));
	else if (jsP_accept(J, TK_SUB_ASS)) a = EXP2(ASS_SUB, a, assignment(J, notin));
	else if (jsP_accept(J, TK_SHL_ASS)) a = EXP2(ASS_SHL, a, assignment(J, notin));
	else if (jsP_accept(J, TK_SHR_ASS)) a = EXP2(ASS_SHR, a, assignment(J, notin));
	else if (jsP_accept(J, TK_USHR_ASS)) a = EXP2(ASS_USHR, a, assignment(J, notin));
	else if (jsP_accept(J, TK_AND_ASS)) a = EXP2(ASS_BITAND, a, assignment(J, notin));
	else if (jsP_accept(J, TK_XOR_ASS)) a = EXP2(ASS_BITXOR, a, assignment(J, notin));
	else if (jsP_accept(J, TK_OR_ASS)) a = EXP2(ASS_BITOR, a, assignment(J, notin));
	DECREC();
    return a;
    */
    panic!("TODO")
}

fn ast_vardec(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let a = ast_identifier(tkr)?;
    if tk_accept(tkr, TokenType::TK_ASS)? {
        let b = ast_assignment(tkr)?;
        let exp = AstNode::new_a_b(AstType::EXP_VAR, a.src_line, a, b);
        return Ok(exp);
    }
    let exp = AstNode::new_a(AstType::EXP_VAR, a.src_line, a);
    return Ok(exp);
}

fn ast_parameters(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let n = tkr.forward()?;
    if n.tk_type == TokenType::TK_PAREN_RIGHT {
        return Ok(AstNode::new(AstType::AST_EMPTY, n.src_line)); 
    }

    let node = ast_identifier(tkr)?;

    let mut head = AstNode::new_list( node );
    let mut tail: &mut AstNode = &mut head;
    while ( tk_accept(tkr, TokenType::TK_COMMA)? ) {
        AstNode::list_tail_push(tail, ast_identifier(tkr)?);
        tail = tail.b.as_mut().unwrap();
    }

    return Ok(head);
}

fn ast_vardeclist(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let node = ast_vardec(tkr)?;
    let mut head = AstNode::new_list( node );
    let mut tail: &mut AstNode = &mut head;
    while ( tk_accept(tkr, TokenType::TK_COMMA)? ) {
        AstNode::list_tail_push(tail, ast_vardec(tkr)?);
        tail = tail.b.as_mut().unwrap();
    }
    return Ok(head);
}

fn ast_semicolon(tkr: &mut Tokenlizer) -> Result<(), String> {
    let lookahead = tkr.forward()?;
    if lookahead.tk_type == TokenType::TK_SEMICOLON || lookahead.tk_type == TokenType::TK_NEWLN {
        tkr.next()?;
        return Ok(());
    }
    if lookahead.tk_type == TokenType::TK_BRACE_RIGHT {
        return Ok(());
    }

    if lookahead.tk_type == TokenType::TK_EOF {
        return Ok(());
    } 

    return Err(format!("unexpected token: {:?} (expected ';')", lookahead));
}

fn ast_funbody(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    tk_except(tkr, TokenType::TK_BRACE_LEFT)?;
    let a = ast_script(tkr)?;
    tk_except(tkr, TokenType::TK_BRACE_RIGHT)?;
    return Ok(a);
}

fn ast_fundec(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let a = ast_identifier(tkr)?;
    tk_except(tkr, TokenType::TK_PAREN_LEFT)?;
    let b = ast_parameters(tkr)?;
    tk_except(tkr, TokenType::TK_PAREN_RIGHT)?;  
    let c = ast_funbody(tkr)?;

    let func = AstNode::new_a_b_c(AstType::AST_FUNDEC, a.src_line, a, b, c);
    return Ok(func);
}

fn ast_statement_list(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let tk = tkr.forward()?;
    if tk.tk_type == TokenType::TK_BRACE_RIGHT || tk.tk_type == TokenType::TK_CASE || tk.tk_type == TokenType::TK_DEFAULT {
        return Ok(AstNode::new(AstType::AST_EMPTY, tk.src_line));
    }
    let mut head = AstNode::new_list( ast_statement(tkr)?);

    let mut tail: &mut AstNode = &mut head;
    loop {
        let tk = tkr.forward()?;
        if tk.tk_type == TokenType::TK_BRACE_RIGHT || tk.tk_type == TokenType::TK_CASE || tk.tk_type == TokenType::TK_DEFAULT {
            break;
        }
        AstNode::list_tail_push(tail, ast_statement(tkr)?);
        tail = tail.b.as_mut().unwrap();
    }
    
    return Ok(head);
}

fn ast_block(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let leftbrace = tk_except(tkr, TokenType::TK_BRACE_LEFT)?;
    let a = ast_statement_list(tkr)?;
    tk_except(tkr, TokenType::TK_BRACE_RIGHT)?;
    return Ok( AstNode::new_a(AstType::STM_BLOCK, leftbrace.src_line, a) );
}

fn ast_statement(tkr: &mut Tokenlizer) -> Result<AstNode, String> {        
    if tkr.forward()?.tk_type == TokenType::TK_BRACE_LEFT {
        return ast_block(tkr);
    } else if tk_accept(tkr, TokenType::TK_VAR)? {
        let a = ast_vardeclist(tkr)?;
        ast_semicolon(tkr)?;
        let stm = AstNode::new_a(AstType::STM_VAR, a.src_line, a);
        return Ok(stm);  
    }

    panic!("TODO")
}

fn ast_element(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    if tk_accept(tkr, TokenType::TK_FUNCTION)? {        
        return ast_fundec(tkr);
    }
    return ast_statement(tkr);
}

fn ast_script(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let mut head = AstNode::new_list( ast_element(tkr)?);

    let mut tail: &mut AstNode = &mut head;
    while tk_accept(tkr, TokenType::TK_EOF)? == false {
        AstNode::list_tail_push(tail, ast_element(tkr)?);
        tail = tail.b.as_mut().unwrap();
    }

    return Ok(head);
}

pub fn build_ast_from_script(filename: &str, script: &str) -> Result<AstNode, String> {
    let mut tkr = Tokenlizer::new(filename, script);
    
    if tk_accept(&mut tkr, TokenType::TK_EOF)? {
        let empty = AstNode::new( AstType::AST_EMPTY, 0);
        return Ok(empty);
    }
    
    return Ok( ast_script(&mut tkr)? );
}

#[cfg(test)]
mod tests {
    use super::*;

}

