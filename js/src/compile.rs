use crate::common::*;
use crate::ast::*;

/* Local help function and struct */

fn checkfutureword(name: &str) {

}

struct AstListIterator<'a> {
    cursor: Option<&'a AstNode>
}

impl<'a> AstListIterator<'a> {
    pub fn new(lst: &'a AstNode ) -> Self {
        assert!(lst.ast_type == AstType::AST_LIST);
        return AstListIterator {
            cursor: Some(lst),
        }
    }
}

impl<'a> Iterator for AstListIterator<'a> {
    type Item = &'a AstNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_none() {
            return None;
        }

        let node = self.cursor.take().unwrap();
        if node.b.is_none() {
            self.cursor = Some( node.b.as_ref().unwrap() );
        }
        return Some(node);
    }
}

impl AstNode {

    fn a(&self) -> &AstNode {
        return self.a.as_ref().unwrap();
    }
    fn b(&self) -> &AstNode {
        return self.b.as_ref().unwrap();
    }
    fn c(&self) -> &AstNode {
        return self.c.as_ref().unwrap();
    }
    fn d(&self) -> &AstNode {
        return self.d.as_ref().unwrap();
    }

    fn has_a(&self) -> bool {
        self.a.is_some()
    }

    fn has_b(&self) -> bool {
        self.b.is_some()
    }
    
    fn has_c(&self) -> bool {
        self.c.is_some()
    }
    
    fn has_d(&self) -> bool {
        self.d.is_some()
    }

    fn is_null(&self) -> bool {
        if self.ast_type == AstType::AST_NULL {
            return true;
        }
        return false;
    }

    fn is_list(&self) -> bool {
        if self.ast_type == AstType::AST_LIST {
            return true;
        }
        return false;
    }

    fn is_loop(&self) -> bool {        
        let at = self.ast_type;
        if at == AstType::STM_DO || at == AstType::STM_WHILE || 
           at == AstType::STM_FOR || at == AstType::STM_FOR_VAR || 
           at == AstType::STM_FOR_IN ||  at == AstType::STM_FOR_IN_VAR {
            return true;
        }
        return false;
    }

    fn is_func(&self) -> bool {
        let at = self.ast_type;
        if at == AstType::AST_FUNDEC || at == AstType::EXP_FUN || at == AstType::EXP_PROP_GET || at == AstType::EXP_PROP_SET {
            return true;
        }
        return false;
    }

    fn len(&self) -> usize {
        if self.is_list() == false {
            return 0;
        }
        let mut l:usize = 1;

        let mut thiz = self;
        while thiz.b.is_some() {
            l = l + 1;
            thiz = self.b.as_ref().unwrap();
        }
        return l;
    }

    fn iter<'a> (&'a self) -> AstListIterator<'a> {
        return AstListIterator::new(self);
    }
}

/* component stuff */
impl VMFunction {
    fn new(script: bool) -> Self {
        VMFunction {
            script: script,
            numparams: 0,
            code:       Vec::new(),
            num_tab:    Vec::new(),
            str_tab:    Vec::new(),
            var_tab:    Vec::new(),
            func_tab:   Vec::new(),

            jumps:      Vec::new(),
        }
    }

    fn emit(&mut self, value: u16) {
        self.code.push(value);
    }

    fn emitop(&mut self, op: OpcodeType) {
        self.code.push(op as u16);
    }

    fn emitjump(&mut self, op: OpcodeType) -> usize {
        self.code.push(op as u16);
        let addr = self.code.len();
        if addr > 0xFFFFFFFF {
            panic!("code length is out of 4G!");
        }
        self.code.push(0);
        self.code.push(0);
        return addr;
    }

    fn emitjumpto(&mut self, op: OpcodeType, dst: usize) {
        self.code.push(op as u16);
        self.code.push((dst & 0xFFFF) as u16);
        self.code.push(((dst >> 16) & 0xFFFF) as u16);
    }

    fn current(& self) -> usize {
        return self.code.len();
    }

    fn label_current_to(&mut self, addr: usize) {
        self.labelto_(addr, self.current());
    }

    fn labelto_(&mut self, addr:usize,  target_addr: usize) {
        if target_addr > 0xFFFFFFFF {
            panic!("current address is out of 4G!");
        }
        self.code[addr] = (target_addr & 0xFFFF) as u16;
        self.code[addr+1] = ((target_addr >> 16) & 0xFFFF) as u16;
    }

    fn new_scope(&mut self, scope: VMJumpScope) {
        let jump = VMJumpTable{
            scope: scope,
            lst: Vec::new(),
        };
        self.jumps.push(jump);
    }

    fn add_jump(&mut self, scope: usize, jmp: VMJumpType) {
        let jmp_lst = &mut self.jumps[scope].lst;
        jmp_lst.push(jmp);
    }

    fn fill_jumps(&mut self, break_addr: usize, continue_addr: usize) {
        let jmp_lst = self.jumps.last().unwrap();
        for j in &jmp_lst.lst {
            match j {
                VMJumpType::BreakJump(pos) => {
                    self.code[*pos] = (break_addr & 0xFFFF) as u16;
                    self.code[*pos+1] = ((break_addr >> 16) & 0xFFFF) as u16;
                },
                VMJumpType::ContinueJump(pos) => {
                    self.code[*pos] = (continue_addr & 0xFFFF) as u16;
                    self.code[*pos+1] = ((continue_addr >> 16) & 0xFFFF) as u16;
                },
            }
        }
    }

    #[allow(non_camel_case_types)]
    fn targetScopeByName(&self, name: &str) -> usize {
        let mut brk_index = 0;
        for i in (0..self.jumps.len()).rev() {
            match &self.jumps[i].scope {
                VMJumpScope::LabelSection(label) => {
                    if label.eq(name) {
                        brk_index = i + 1;
                        return brk_index;
                    }
                },
                _ => {}
            }
        }
        return brk_index;
    }

    #[allow(non_camel_case_types)]
    fn targetBreakScope(&self) -> usize {
        let mut brk_index = 0;
        for i in (0..self.jumps.len()).rev() {
            match &self.jumps[i].scope {
                VMJumpScope::ForLoop | VMJumpScope::ForInLoop | VMJumpScope::DoLoop | VMJumpScope::WhileLoop | VMJumpScope::SwitchScope => {
                    brk_index = i + 1;
                    break;
                },
                _ => {}
            }
        }
        return brk_index;
    }

    #[allow(non_camel_case_types)]
    fn targetContinueScope(&self) -> usize {
        let mut brk_index = 0;
        for i in (0..self.jumps.len()).rev() {
            match &self.jumps[i].scope {
                VMJumpScope::ForLoop | VMJumpScope::ForInLoop | VMJumpScope::DoLoop | VMJumpScope::WhileLoop => {
                    brk_index = i + 1;
                    break;
                },
                _ => {}
            }
        }
        return brk_index;
    }

    fn delete_scope(&mut self) {
        self.jumps.pop();
    }

    fn addlocal(&mut self, node: &AstNode) -> u16 {
        let name = node.str_value.as_ref().unwrap();
        checkfutureword(name);

        self.var_tab.push(name.clone());

        let id = self.var_tab.len() as u16;
        return id;
    }

    fn findlocal(&self, node: &AstNode) -> u16 {
        let name = node.str_value.as_ref().unwrap();
        checkfutureword(name);

        for i in 0..self.var_tab.len() {
            if self.var_tab[i].eq(name) {
                return (i+1) as u16;
            }
        }
        return 0;
    }

    fn addfunc(&mut self, func: VMFunction) -> u16 {
        self.func_tab.push(Box::new(func));
        return self.func_tab.len() as u16;
    }

    fn parsing_vardec(&mut self, node: &AstNode) {
        if node.is_list() {
            let it = node.iter();
            for n in it {
                self.parsing_vardec(n);
            }
            return;
        }

        if node.is_func() {
            return; /* stop at inner functions */
        }

        if node.ast_type == AstType::EXP_VAR {
            self.addlocal(node);
        }

        if node.a.is_some() {
            self.parsing_vardec(node.a());
        }
        if node.b.is_some() {
            self.parsing_vardec(node.b());
        }

        if node.c.is_some() {
            self.parsing_vardec(node.c());
        }

        if node.d.is_some() {
            self.parsing_vardec(node.d());
        }
    }

    fn parsing_fundec(&mut self, lst: &AstNode) {
        if lst.is_list() {
            let it = lst.iter();
            for n in it {
                if n.ast_type == AstType::AST_FUNDEC {
                    let newfunc = compile_func( n.a(), n.b(), n.c(), false).unwrap();
                    let fid = self.addfunc(newfunc);
                    self.emitop(OpcodeType::OP_CLOSURE);
                    self.emit(fid);

                    let vid = self.addlocal( n.a() );
                    self.emitop(OpcodeType::OP_SETLOCAL);
                    self.emit(vid);
                    self.emitop(OpcodeType::OP_POP);
                }
            }
            return;
        }
    }
}

/* Expressions */
fn compile_exp(f: &mut VMFunction, exp: &AstNode) {

}

/* Emit code to rebalance stack and scopes during an abrupt exit */
fn compile_exit(f: &mut VMFunction, scope_index: usize, jump_type: AstType) {

}
/*

static void cexit(JF, enum js_AstType T, js_Ast *node, js_Ast *target)
{
	js_Ast *prev;
	do {
		prev = node, node = node->parent;
		switch (node->type) {
		default:
			/* impossible */
			break;
		case STM_WITH:
			emitline(J, F, node);
			emit(J, F, OP_ENDWITH);
			break;
		case STM_FOR_IN:
		case STM_FOR_IN_VAR:
			emitline(J, F, node);
			/* pop the iterator if leaving the loop */
			if (F->script) {
				if (T == STM_RETURN || T == STM_BREAK || (T == STM_CONTINUE && target != node)) {
					/* pop the iterator, save the return or exp value */
					emit(J, F, OP_ROT2);
					emit(J, F, OP_POP);
				}
				if (T == STM_CONTINUE)
					emit(J, F, OP_ROT2); /* put the iterator back on top */
			} else {
				if (T == STM_RETURN) {
					/* pop the iterator, save the return value */
					emit(J, F, OP_ROT2);
					emit(J, F, OP_POP);
				}
				if (T == STM_BREAK || (T == STM_CONTINUE && target != node))
					emit(J, F, OP_POP); /* pop the iterator */
			}
			break;
		case STM_TRY:
			emitline(J, F, node);
			/* came from try block */
			if (prev == node->a) {
				emit(J, F, OP_ENDTRY);
				if (node->d) cstm(J, F, node->d); /* finally */
			}
			/* came from catch block */
			if (prev == node->c) {
				/* ... with finally */
				if (node->d) {
					emit(J, F, OP_ENDCATCH);
					emit(J, F, OP_ENDTRY);
					cstm(J, F, node->d); /* finally */
				} else {
					emit(J, F, OP_ENDCATCH);
				}
			}
			break;
		}
	} while (node != target);
}
*/

/* Try/catch/finally */

fn compile_trycatchfinally(f: &mut VMFunction, a: &AstNode, b: &AstNode, c: &AstNode, d: &AstNode) {

} 

fn compile_trycatch(f: &mut VMFunction, a: &AstNode, b: &AstNode, c: &AstNode) {

}

fn compile_finally(f: &mut VMFunction, a: &AstNode, b: &AstNode) {

} 

/* Switch */

fn compile_switch(f: &mut VMFunction, exp: &AstNode) {

}

/* Statements */

fn compile_varinit(f: &mut VMFunction, exp: &AstNode) {

}

fn compile_assignforin(f: &mut VMFunction, exp: &AstNode) {

}

fn compile_stm(f: &mut VMFunction, stm: &AstNode) {
    match stm.ast_type {
        AstType::STM_BLOCK => {
            let block = stm.a.as_ref().unwrap();
            compile_stmlist(f, block);
        },
        AstType::STM_EMPTY => {
            // do nothing
        },
        AstType::STM_IF => {
            if stm.c.is_some() {
                compile_exp(f, stm.a.as_ref().unwrap());
                let then = f.emitjump(OpcodeType::OP_JTRUE);
                compile_stm(f, stm.c.as_ref().unwrap());
                let end = f.emitjump(OpcodeType::OP_JUMP);
                f.label_current_to(then);
                compile_stm(f, stm.b.as_ref().unwrap());
                f.label_current_to(end);
            } else {
                compile_exp(f, stm.a.as_ref().unwrap());
                let end = f.emitjump(OpcodeType::OP_JFALSE);
                compile_stm(f, stm.b.as_ref().unwrap());
                f.label_current_to(end);
            }
        },
        AstType::STM_DO => {
            f.new_scope(VMJumpScope::DoLoop);
    
            let lop = f.current();
            compile_stm(f, stm.a.as_ref().unwrap());
            let cont = f.current();
            compile_exp(f, stm.b.as_ref().unwrap());
            f.emitjumpto(OpcodeType::OP_JTRUE, lop);
            
            f.fill_jumps(f.current(), cont);
            f.delete_scope();
        },

        AstType::STM_WHILE => {
            f.new_scope(VMJumpScope::WhileLoop);

            let lop = f.current();
            compile_exp(f, stm.a());
            let end = f.emitjump(OpcodeType::OP_JFALSE);
            compile_stm(f, stm.b());
            f.emitjumpto(OpcodeType::OP_JUMP, lop);
            f.label_current_to(end);

            f.fill_jumps(f.current(), lop);
            f.delete_scope();
        },

        AstType::STM_FOR |  AstType::STM_FOR_VAR => {
            f.new_scope(VMJumpScope::ForLoop);

            if stm.ast_type == AstType::STM_FOR_VAR {
                compile_varinit(f, stm.a());
            } else {       
                let a = stm.a();
                if ! a.is_null() {
                    compile_exp(f, a);
                    f.emitop(OpcodeType::OP_POP);
                }
            }

            let lop = f.current();
            let b = stm.b();
            let end = if ! b.is_null() {
                compile_exp(f, b);
                f.emitjump(OpcodeType::OP_JFALSE)
            } else {
                0
            };

            compile_stm(f, stm.d.as_ref().unwrap());

            let cont = f.current();
            let c = stm.c();
            if !c.is_null() {
                compile_exp(f, c);
                f.emitop(OpcodeType::OP_POP);
            }
            f.emitjumpto(OpcodeType::OP_JUMP, lop);

            if end > 0 {
                f.label_current_to(end);
            } 

            f.fill_jumps(f.current(), cont);
            f.delete_scope();
        },
        
        AstType::STM_FOR_IN |  AstType::STM_FOR_IN_VAR => {
            f.new_scope(VMJumpScope::ForInLoop);

            compile_exp(f, stm.b());
            f.emitop(OpcodeType::OP_ITERATOR);
            let lop = f.current();
            
            f.emitop(OpcodeType::OP_NEXTITER);
            let end = f.emitjump(OpcodeType::OP_JFALSE);
            compile_assignforin(f, stm);

            compile_stm(f, stm.c.as_ref().unwrap());
            
            f.emitjumpto(OpcodeType::OP_JUMP, lop);
            f.label_current_to(end);

            f.fill_jumps(f.current(), lop);
            f.delete_scope();
        },
        
        AstType::STM_SWITCH => {
            f.new_scope(VMJumpScope::SwitchScope);
            compile_switch(f, stm);
            f.fill_jumps(f.current(), f.current());
            f.delete_scope();
        },

        AstType::STM_LABEL => {
            let a = stm.a.as_ref().unwrap();
            f.new_scope(VMJumpScope::LabelSection(a.str_value.as_ref().unwrap().to_string()));
           
            compile_stm(f, stm.b.as_ref().unwrap());
            /* skip consecutive labels */
            let mut node = stm;
            while node.ast_type == AstType::STM_LABEL {
                node = stm.b.as_ref().unwrap();
            }

            /* loops and switches have already been labelled */
            if !node.is_loop() && node.ast_type != AstType::STM_SWITCH {
                f.fill_jumps(f.current(), f.current());
            }

            f.delete_scope();
        },

        AstType::STM_BREAK => {
            let a = stm.a();
            let break_scope: usize;

            if !a.is_null() {
                let break_target = a.str_value.as_ref().unwrap();
                checkfutureword(break_target);
                break_scope = f.targetScopeByName(break_target);
            } else {
                break_scope = f.targetBreakScope();
            }
            if break_scope == 0 {
                panic!("Can't find break target!");
            }
            
            compile_exit(f, break_scope, AstType::STM_BREAK);
            let from = f.emitjump(OpcodeType::OP_JUMP);
            let jump = VMJumpType::BreakJump(from);
            f.add_jump(break_scope, jump);
        },
        
        AstType::STM_CONTINUE => {
            let a = stm.a();
            let continue_scope: usize;

            if !a.is_null() {
                let continue_target = a.str_value.as_ref().unwrap();
                checkfutureword(continue_target);
                continue_scope = f.targetScopeByName(continue_target);
            } else {
                continue_scope = f.targetContinueScope();
            }
            if continue_scope == 0 {
                panic!("Can't find continue target!");                
            }

            compile_exit(f, continue_scope, AstType::STM_CONTINUE);
            let from = f.emitjump(OpcodeType::OP_JUMP);
            let jump = VMJumpType::ContinueJump(from);
            f.add_jump(continue_scope, jump);
        },
        
        AstType::STM_RETURN => {
            if f.script {
                panic!("Find return in script code!");
            }

            let a = stm.a.as_ref().unwrap();
            if a.is_null() {
                f.emitop(OpcodeType::OP_UNDEF);
            } else {
                compile_exp(f, a);
            }
            
            compile_exit(f, 0, AstType::STM_RETURN);
            f.emitop(OpcodeType::OP_RETURN);
        },

        AstType::STM_THROW => {
            compile_exp(f, stm.a.as_ref().unwrap());
            f.emitop(OpcodeType::OP_THROW);
        },

        AstType::STM_WITH => {
            panic!("'with' statements are not allowed in strict mode");
        },

        AstType::STM_TRY => {
            if stm.has_b() && stm.has_c() {
                if stm.has_d() {
                    compile_trycatchfinally(f, stm.a(), stm.b(), stm.c(), stm.d());
                } else {
                    compile_trycatch(f, stm.a(), stm.b(), stm.c());
                }
            } else {
                compile_finally(f, stm.a(), stm.b()); 
            }
        },

        AstType::STM_DEBUGGER => {
            f.emitop(OpcodeType::OP_DEBUGGER);
        },

        _ => {
            compile_exp(f, stm);
            f.emitop(OpcodeType::OP_POP);
        }    
    }
}

fn compile_stmlist(f: &mut VMFunction, lst: &AstNode) {
    for stm in lst.iter() {
        compile_stm(f, stm);
    }
}

fn compile_func(name: &AstNode, params: &AstNode, body: &AstNode, script: bool) -> Result<VMFunction, String> {
    let mut f = VMFunction::new(script);

    // parsing params
    if !params.is_null() {
        f.numparams = params.len();
        let it = params.iter();
        for node in it {
            f.addlocal(node);
        }
    }

    if !body.is_null() {
		f.parsing_vardec(body);
		f.parsing_fundec(body);
    }

    if !name.is_null() {

        let mut localid = f.findlocal( name );
        if localid == 0 {
            f.emitop(OpcodeType::OP_CURRENT);
            f.emitop(OpcodeType::OP_SETLOCAL);
            localid = f.addlocal(name);
            f.emit(localid);
            f.emitop(OpcodeType::OP_POP);
        }
    }

    if f.script {
        f.emitop(OpcodeType::OP_UNDEF);
        compile_stmlist(&mut f, body);
        f.emitop(OpcodeType::OP_RETURN);
    } else {
        compile_stmlist(&mut f, body);
        f.emitop(OpcodeType::OP_UNDEF);
        f.emitop(OpcodeType::OP_RETURN);
    }

    return Ok(f);
}

pub fn build_function_from_code(script: &str) -> Result<VMFunction, String> {
    let ast = build_ast_from_script(script).unwrap();

    let null = AstNode::null();
    let func = compile_func(&null, &null, &ast, true)?;
    return Ok(func);
}

