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

    fn new_a_b_c_d(ntype: AstType, line: u32, a: Self, b: Self, c: Self, d: Self) -> Self {
        AstNode {
            ast_type: ntype,
            src_line: line, 
            num_value: None,
            str_value: None,
            a: Some(Box::new(a)),
            b: Some(Box::new(b)),
            c: Some(Box::new(c)),
            d: Some(Box::new(d)),
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

fn tk_expect(tkr: &mut Tokenlizer, tkt: TokenType) -> Result<Token, String> {
    let ntk = tkr.next()?;
    if ntk.tk_type != tkt {
        return Err(format!("AST error: except {:?} but got {:?} @ {}", tkt, ntk.tk_type, ntk.src_line));
    }
    return Ok(ntk);
}

fn ast_identifier(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let id = tk_expect(tkr, TokenType::TK_IDENTIFIER)?;
    let node = AstNode::new_string(AstType::AST_IDENTIFIER, tkr.line(), &id.tk_value.unwrap());
    return Ok(node);
}

fn ast_identifier_opt(tkr: &mut Tokenlizer) -> Result<Option<AstNode>, String> {
    let ntk = tkr.forward()?;
    if ntk.tk_type == TokenType::TK_IDENTIFIER {
        let node = AstNode::new_string(AstType::AST_IDENTIFIER, tkr.line(), &ntk.tk_value.unwrap());
        return Ok(Some(node));
    } else {
        return Ok(None);
    }
}

fn ast_formula(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    panic!("TODO")
}

fn ast_assignment(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let a = ast_formula(tkr)?;

    if tk_accept(tkr, TokenType::TK_QUEST)? {
        let b = ast_assignment(tkr)?;
        tk_expect(tkr, TokenType::TK_COLON)?;
        let c = ast_assignment(tkr)?;
        let node = AstNode::new_a_b_c(AstType::EXP_COND, a.src_line, a, b, c);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_MUL_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_MUL, a.src_line, a, b);
        return Ok(node);
    }  else if tk_accept(tkr, TokenType::TK_DIV_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_DIV, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_MOD_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_MOD, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_ADD_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_ADD, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_SUB_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_SUB, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_SHL_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_SHL, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_SHR_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_SHR, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_USHR_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_USHR, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_AND_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_AND, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_XOR_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_XOR, a.src_line, a, b);
        return Ok(node);
    } else if tk_accept(tkr, TokenType::TK_OR_ASS)? {
        let b = ast_assignment(tkr)?;
        let node = AstNode::new_a_b(AstType::EXP_ASS_OR, a.src_line, a, b);
        return Ok(node);
    }
    return Ok(a);
}

fn ast_expression(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let mut a = ast_assignment(tkr)?;
    while tk_accept(tkr, TokenType::TK_COMMA)? {
        let b = ast_assignment(tkr)?;
        a = AstNode::new_a_b(AstType::EXP_COMMA, tkr.line(), a, b);
    }
    return Ok(a);
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

fn ast_vardeclist(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let node = ast_vardec(tkr)?;
    let mut head = AstNode::new_list( node );
    let mut tail: &mut AstNode = &mut head;
    while tk_accept(tkr, TokenType::TK_COMMA)? {
        AstNode::list_tail_push(tail, ast_vardec(tkr)?);
        tail = tail.b.as_mut().unwrap();
    }
    return Ok(head);
}

fn ast_parameters(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let n = tkr.forward()?;
    if n.tk_type == TokenType::TK_PAREN_RIGHT {
        return Ok(AstNode::new(AstType::AST_EMPTY, n.src_line)); 
    }

    let node = ast_identifier(tkr)?;

    let mut head = AstNode::new_list( node );
    let mut tail: &mut AstNode = &mut head;
    while tk_accept(tkr, TokenType::TK_COMMA)?  {
        AstNode::list_tail_push(tail, ast_identifier(tkr)?);
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
    tk_expect(tkr, TokenType::TK_BRACE_LEFT)?;
    let a = ast_script(tkr)?;
    tk_expect(tkr, TokenType::TK_BRACE_RIGHT)?;
    return Ok(a);
}

fn ast_fundec(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    let a = ast_identifier(tkr)?;
    tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
    let b = ast_parameters(tkr)?;
    tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;  
    let c = ast_funbody(tkr)?;

    let func = AstNode::new_a_b_c(AstType::AST_FUNDEC, a.src_line, a, b, c);
    return Ok(func);
}

fn ast_funstm(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    panic!("TODO")
}

fn ast_forstatement(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    panic!("TODO")
}

fn ast_caselist(tkr: &mut Tokenlizer) -> Result<AstNode, String> {
    panic!("TODO")    
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
    let leftbrace = tk_expect(tkr, TokenType::TK_BRACE_LEFT)?;
    let a = ast_statement_list(tkr)?;
    tk_expect(tkr, TokenType::TK_BRACE_RIGHT)?;
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

    } else if tk_accept(tkr, TokenType::TK_SEMICOLON)? {        
        return Ok( AstNode::new(AstType::STM_EMPTY, tkr.line()) );

    } else if tk_accept(tkr, TokenType::TK_IF)? {      
        tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
        let a = ast_expression(tkr)?;
        tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;
        let b = ast_statement(tkr)?;
        if tk_accept(tkr, TokenType::TK_ELSE)? {
            let c = ast_statement(tkr)?;
            return Ok(AstNode::new_a_b_c(AstType::STM_IF, tkr.line(), a, b, c));
        }
        return Ok(AstNode::new_a_b(AstType::STM_IF, tkr.line(), a, b));

    } else if tk_accept(tkr, TokenType::TK_DO)? {
        let a = ast_statement(tkr)?;
        tk_expect(tkr, TokenType::TK_WHILE)?;
        tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
        let b = ast_expression(tkr)?;
        tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;
        ast_semicolon(tkr)?;
        
        let stm = AstNode::new_a_b(AstType::STM_DO, tkr.line(), a, b);
        return Ok(stm);
    
    } else if tk_accept(tkr, TokenType::TK_WHILE)? {        
        tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
        let a = ast_expression(tkr)?;
        tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;
        let b = ast_statement(tkr)?;

        let stm = AstNode::new_a_b(AstType::STM_WHILE, tkr.line(), a, b);
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_FOR)? {                
        let stm = ast_forstatement(tkr)?;
        return Ok(stm);
        
    } else if tk_accept(tkr, TokenType::TK_CONTINUE)? {                
        let id_opt = ast_identifier_opt(tkr)?;
        ast_semicolon(tkr)?;
        if id_opt.is_some() {
            let a = id_opt.unwrap();
            let stm = AstNode::new_a(AstType::STM_CONTINUE, tkr.line(), a);
            return Ok(stm);
        }
        let stm = AstNode::new(AstType::STM_CONTINUE, tkr.line());
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_BREAK)? {                
        let id_opt = ast_identifier_opt(tkr)?;
        ast_semicolon(tkr)?;
        if id_opt.is_some() {
            let a = id_opt.unwrap();
            let stm = AstNode::new_a(AstType::STM_BREAK, tkr.line(), a);
            return Ok(stm);
        }
        let stm = AstNode::new(AstType::STM_BREAK, tkr.line());
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_RETURN)? {                
        let ntk = tkr.next()?;
        if ntk.tk_type != TokenType::TK_SEMICOLON && ntk.tk_type != TokenType::TK_BRACE_RIGHT {
            let a = ast_expression(tkr)?;
            let stm = AstNode::new_a(AstType::STM_RETURN, tkr.line(), a);
            return Ok(stm);
        }
        ast_semicolon(tkr)?;
        let stm = AstNode::new(AstType::STM_RETURN, tkr.line());        
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_WITH)? {                
        tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
        let a = ast_expression(tkr)?;
        tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;
        let b = ast_statement(tkr)?;
        
        let stm = AstNode::new_a_b(AstType::STM_WITH, tkr.line(), a, b);
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_SWITCH)? {                
        tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
        let a = ast_expression(tkr)?;
        tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;
        tk_expect(tkr, TokenType::TK_BRACE_LEFT)?;
        let b = ast_caselist(tkr)?;
        tk_expect(tkr, TokenType::TK_BRACE_RIGHT)?;
        let stm = AstNode::new_a_b(AstType::STM_SWITCH, tkr.line(), a, b);
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_THROW)? {                
        let a = ast_expression(tkr)?;
        ast_semicolon(tkr)?;

        let stm = AstNode::new_a(AstType::STM_THROW, tkr.line(), a);
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_TRY)? {       
        let a = ast_block(tkr)?;        
        if tk_accept(tkr, TokenType::TK_CATCH)? {
            tk_expect(tkr, TokenType::TK_PAREN_LEFT)?;
            let b = ast_identifier(tkr)?;
            tk_expect(tkr, TokenType::TK_PAREN_RIGHT)?;
            let c = ast_block(tkr)?;
            
            if tk_accept(tkr, TokenType::TK_FINALLY)? {
                let d = ast_block(tkr)?;
                let stm = AstNode::new_a_b_c_d(AstType::STM_THROW, tkr.line(), a, b, c, d);
                return Ok(stm);
            }
            
            let stm = AstNode::new_a_b_c(AstType::STM_THROW, tkr.line(), a, b, c);
            return Ok(stm);
        }
        if tk_accept(tkr, TokenType::TK_FINALLY)? {
            let a = ast_block(tkr)?;
            let stm = AstNode::new_a(AstType::STM_THROW, tkr.line(), a);
            return Ok(stm);
        }
        return Err(format!("unexpected token in try: {:?} (expected 'catch' or 'finally')", tkr.forward()? ));

    } else if tk_accept(tkr, TokenType::TK_DEBUGGER)? {                
        ast_semicolon(tkr)?;
        let stm = AstNode::new(AstType::STM_DEBUGGER, tkr.line());
        return Ok(stm);

    } else if tk_accept(tkr, TokenType::TK_FUNCTION)? {                
        let stm = ast_funstm(tkr)?;
        return Ok(stm);
    
    } else if tk_accept(tkr, TokenType::TK_IDENTIFIER)? {                
        let mut a = ast_expression(tkr)?;
        if a.ast_type == AstType::EXP_IDENTIFIER {
            if tk_accept(tkr, TokenType::TK_COLON)? {
                a.ast_type = AstType::AST_IDENTIFIER;
                let b = ast_statement(tkr)?;
                let stm = AstNode::new_a_b(AstType::STM_LABEL, tkr.line(), a, b);
                return Ok(stm);
            }
        }
        ast_semicolon(tkr)?;
        return Ok(a);
    }

    let stm = ast_expression(tkr)?;
    return Ok(stm);
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

