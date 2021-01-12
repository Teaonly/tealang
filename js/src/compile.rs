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

    fn emitnumber(&mut self, value:f64) {
        self.emitop(OpcodeType::OP_NUMBER);
        let id = self.addnumber(value);
        self.emit(id);
    }

    fn emitstring(&mut self, op: OpcodeType, var: &str) {
        self.emitop(op);
        let id = self.addstring(var);
        self.emit(id);
    }

    fn emitlocal(&mut self, oploc: OpcodeType, opvar: OpcodeType, var: &str) {
        checkfutureword(var);
        
        let (found, i) =  self.findlocal(var);
        if found {
            self.emitstring(opvar, var);
        } else {
            self.emitop(oploc);
            self.emit(i);
        }
    }

    fn findlocal(&self, var: &str) -> (bool, u16) {
        for i in 0..self.var_tab.len() {
            if self.var_tab[i].eq(var) {
                return (true, i as u16);
            }
        }
        return (false, 0);
    }

    fn addnumber(&mut self, value: f64) -> u16 {
        for i in 0..self.num_tab.len() {
            if self.num_tab[i] == value {
                return i as u16;
            }
        }
        let r = self.num_tab.len();
        self.num_tab.push( value);

        return r as u16;
    }
    fn addstring(&mut self, var: &str) -> u16 {
        for i in 0..self.str_tab.len() {
            if self.str_tab[i].eq(var) {
                return i as u16;
            }
        }

        let r = self.str_tab.len();
        self.str_tab.push( var.to_string() );

        return r as u16;
    }

    fn current(& self) -> usize {
        return self.code.len();
    }

    fn label_current_to(&mut self, addr: usize) {
        self.labelto(addr, self.current());
    }

    fn labelto(&mut self, addr:usize,  target_addr: usize) {
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

fn compile_object(f: &mut VMFunction, exp: &AstNode) {

}
fn compile_array(f: &mut VMFunction, exp: &AstNode) {

}

fn compile_exp(f: &mut VMFunction, exp: &AstNode) {
    match exp.ast_type {
        /* immediately value*/ 
        AstType::EXP_STRING => {
            let value = exp.str_value.as_ref().unwrap();
            f.emitstring(OpcodeType::OP_STRING, value);
        },
        AstType::EXP_NUMBER => {
            let value = exp.num_value.unwrap();
            f.emitnumber(value);
        },
        AstType::EXP_UNDEF => {
            f.emitop(OpcodeType::OP_UNDEF);
        },
        AstType::EXP_NULL => {
            f.emitop(OpcodeType::OP_NULL);
        },
        AstType::EXP_TRUE => {
            f.emitop(OpcodeType::OP_TRUE);
        },
        AstType::EXP_FALSE => {
            f.emitop(OpcodeType::OP_FALSE);
        },
        AstType::EXP_THIS => {
            f.emitop(OpcodeType::OP_THIS);
        },

        /* complex value*/
        AstType::EXP_OBJECT => {
            f.emitop(OpcodeType::OP_NEWOBJECT);
            compile_object(f, exp.a());
        },

        AstType::EXP_ARRAY => {
            f.emitop(OpcodeType::OP_NEWOBJECT);
            compile_array(f, exp.a());
        },

        AstType::EXP_FUN => {
            
        }

        _ => {

        }
    }
}

/*
static void cexp(JF, js_Ast *exp)
{
	int then, end;
	int n;

	switch (exp->type) {
	case EXP_FUN:
		emitline(J, F, exp);
		emitfunction(J, F, newfun(J, exp->line, exp->a, exp->b, exp->c, 0, F->strict));
		break;

	case EXP_IDENTIFIER:
		emitline(J, F, exp);
		emitlocal(J, F, OP_GETLOCAL, OP_GETVAR, exp);
		break;

	case EXP_INDEX:
		cexp(J, F, exp->a);
		cexp(J, F, exp->b);
		emitline(J, F, exp);
		emit(J, F, OP_GETPROP);
		break;

	case EXP_MEMBER:
		cexp(J, F, exp->a);
		emitline(J, F, exp);
		emitstring(J, F, OP_GETPROP_S, exp->b->string);
		break;

	case EXP_CALL:
		ccall(J, F, exp->a, exp->b);
		break;

	case EXP_NEW:
		cexp(J, F, exp->a);
		n = cargs(J, F, exp->b);
		emitline(J, F, exp);
		emit(J, F, OP_NEW);
		emitarg(J, F, n);
		break;

	case EXP_DELETE:
		cdelete(J, F, exp);
		break;

	case EXP_PREINC:
		cassignop1(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_INC);
		cassignop2(J, F, exp->a, 0);
		break;

	case EXP_PREDEC:
		cassignop1(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_DEC);
		cassignop2(J, F, exp->a, 0);
		break;

	case EXP_POSTINC:
		cassignop1(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_POSTINC);
		cassignop2(J, F, exp->a, 1);
		emit(J, F, OP_POP);
		break;

	case EXP_POSTDEC:
		cassignop1(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_POSTDEC);
		cassignop2(J, F, exp->a, 1);
		emit(J, F, OP_POP);
		break;

	case EXP_VOID:
		cexp(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_POP);
		emit(J, F, OP_UNDEF);
		break;

	case EXP_TYPEOF: ctypeof(J, F, exp); break;
	case EXP_POS: cunary(J, F, exp, OP_POS); break;
	case EXP_NEG: cunary(J, F, exp, OP_NEG); break;
	case EXP_BITNOT: cunary(J, F, exp, OP_BITNOT); break;
	case EXP_LOGNOT: cunary(J, F, exp, OP_LOGNOT); break;

	case EXP_BITOR: cbinary(J, F, exp, OP_BITOR); break;
	case EXP_BITXOR: cbinary(J, F, exp, OP_BITXOR); break;
	case EXP_BITAND: cbinary(J, F, exp, OP_BITAND); break;
	case EXP_EQ: cbinary(J, F, exp, OP_EQ); break;
	case EXP_NE: cbinary(J, F, exp, OP_NE); break;
	case EXP_STRICTEQ: cbinary(J, F, exp, OP_STRICTEQ); break;
	case EXP_STRICTNE: cbinary(J, F, exp, OP_STRICTNE); break;
	case EXP_LT: cbinary(J, F, exp, OP_LT); break;
	case EXP_GT: cbinary(J, F, exp, OP_GT); break;
	case EXP_LE: cbinary(J, F, exp, OP_LE); break;
	case EXP_GE: cbinary(J, F, exp, OP_GE); break;
	case EXP_INSTANCEOF: cbinary(J, F, exp, OP_INSTANCEOF); break;
	case EXP_IN: cbinary(J, F, exp, OP_IN); break;
	case EXP_SHL: cbinary(J, F, exp, OP_SHL); break;
	case EXP_SHR: cbinary(J, F, exp, OP_SHR); break;
	case EXP_USHR: cbinary(J, F, exp, OP_USHR); break;
	case EXP_ADD: cbinary(J, F, exp, OP_ADD); break;
	case EXP_SUB: cbinary(J, F, exp, OP_SUB); break;
	case EXP_MUL: cbinary(J, F, exp, OP_MUL); break;
	case EXP_DIV: cbinary(J, F, exp, OP_DIV); break;
	case EXP_MOD: cbinary(J, F, exp, OP_MOD); break;

	case EXP_ASS: cassign(J, F, exp); break;
	case EXP_ASS_MUL: cassignop(J, F, exp, OP_MUL); break;
	case EXP_ASS_DIV: cassignop(J, F, exp, OP_DIV); break;
	case EXP_ASS_MOD: cassignop(J, F, exp, OP_MOD); break;
	case EXP_ASS_ADD: cassignop(J, F, exp, OP_ADD); break;
	case EXP_ASS_SUB: cassignop(J, F, exp, OP_SUB); break;
	case EXP_ASS_SHL: cassignop(J, F, exp, OP_SHL); break;
	case EXP_ASS_SHR: cassignop(J, F, exp, OP_SHR); break;
	case EXP_ASS_USHR: cassignop(J, F, exp, OP_USHR); break;
	case EXP_ASS_BITAND: cassignop(J, F, exp, OP_BITAND); break;
	case EXP_ASS_BITXOR: cassignop(J, F, exp, OP_BITXOR); break;
	case EXP_ASS_BITOR: cassignop(J, F, exp, OP_BITOR); break;

	case EXP_COMMA:
		cexp(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_POP);
		cexp(J, F, exp->b);
		break;

	case EXP_LOGOR:
		cexp(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_DUP);
		end = emitjump(J, F, OP_JTRUE);
		emit(J, F, OP_POP);
		cexp(J, F, exp->b);
		label(J, F, end);
		break;

	case EXP_LOGAND:
		cexp(J, F, exp->a);
		emitline(J, F, exp);
		emit(J, F, OP_DUP);
		end = emitjump(J, F, OP_JFALSE);
		emit(J, F, OP_POP);
		cexp(J, F, exp->b);
		label(J, F, end);
		break;

	case EXP_COND:
		cexp(J, F, exp->a);
		emitline(J, F, exp);
		then = emitjump(J, F, OP_JTRUE);
		cexp(J, F, exp->c);
		end = emitjump(J, F, OP_JUMP);
		label(J, F, then);
		cexp(J, F, exp->b);
		label(J, F, end);
		break;

	default:
		jsC_error(J, exp, "unknown expression: (%s)", jsP_aststring(exp->type));
	}
}
*/

/* Emit code to rebalance stack and scopes during an abrupt exit */
fn compile_exit(f: &mut VMFunction, scope_index: usize, jump_type: AstType) {
    if f.jumps.len() == 0 {
        return;
    }
    for i in (scope_index .. f.jumps.len()).rev() {
        let scope_type = f.jumps[i].scope.clone();
        match scope_type {
            VMJumpScope::WithScope => {
                panic!("'with' statements are not allowed in strict mode");
            },
            VMJumpScope::TryScope(stm_d) => {
                f.emitop(OpcodeType::OP_ENDTRY);
                if stm_d.is_some() {
                    compile_stm(f, stm_d.as_ref().unwrap());
                }
            },
            VMJumpScope::CatchScope(stm_b) => {
                f.emitop(OpcodeType::OP_ENDCATCH);
                if stm_b.is_some() {
                    f.emitop(OpcodeType::OP_ENDCATCH);
                    compile_stm(f, stm_b.as_ref().unwrap());
                }
            },
            VMJumpScope::ForInLoop => {
                if jump_type == AstType::STM_BREAK {
                    /* pop the iterator */
                    f.emitop(OpcodeType::OP_POP);
                } else if jump_type == AstType::STM_CONTINUE {
                    if scope_index != i {
                        /* pop the iterator */
                        f.emitop(OpcodeType::OP_POP);
                    }
                } else if jump_type == AstType::STM_RETURN {
                    /* pop the iterator, save the return value */
                    f.emitop(OpcodeType::OP_ROT2);
                    f.emitop(OpcodeType::OP_POP);
                } else {
                    panic!("compile_exit error: only break/continue/return supported!");
                }
            },
            _ => {
                
            }
        }
    }
}

/* Try/catch/finally */
fn compile_trycatchfinally(f: &mut VMFunction, try_block: &AstNode, catch_var: &AstNode, catch_block: &AstNode, finally_block: &AstNode) {
    let L1:usize;
    let L2:usize;
    let L3:usize;

    //f.new_scope(VMJumpScope::TryScope);
    L1 = f.emitjump(OpcodeType::OP_TRY);
    {
        /* if we get here, we have caught an exception in the try block */
        //f.new_scope(VMJumpScope::TryScope);
        L2 = f.emitjump(OpcodeType::OP_TRY);
        {
            /* if we get here, we have caught an exception in the catch block */
            compile_stm(f, finally_block);  /* inline finally block */
            f.emitop(OpcodeType::OP_THROW);
        }
        f.label_current_to(L2);

        let catchvar = catch_var.str_value.as_ref().unwrap();
        checkfutureword(catchvar);
        //f.new_scope(VMJumpScope::CatchScope);
        f.emitstring(OpcodeType::OP_CATCH, catchvar);
        compile_stm(f, catch_block);
        f.emitop(OpcodeType::OP_ENDCATCH);
        //f.delete_scope();
        f.emitop(OpcodeType::OP_ENDTRY);
        //f.delete_scope();
        L3 = f.emitjump(OpcodeType::OP_JUMP);
    }
    f.label_current_to(L1);
    compile_stm(f, try_block);
    f.emitop(OpcodeType::OP_ENDTRY);
    //f.delete_scope();
    f.label_current_to(L3);
    compile_stm(f, finally_block);
} 

fn compile_trycatch(f: &mut VMFunction, a: &AstNode, b: &AstNode, c: &AstNode) {
    let L1:usize;
    let L2:usize;

    //f.new_scope(VMJumpScope::TryScope);
    L1 = f.emitjump(OpcodeType::OP_TRY);
    {
        /* if we get here, we have caught an exception in the try block */
        let catchvar = b.str_value.as_ref().unwrap();
        checkfutureword(catchvar);
        f.emitstring(OpcodeType::OP_CATCH, catchvar);
        compile_stm(f, c);
        f.emitop(OpcodeType::OP_ENDCATCH);
    }
    f.label_current_to(L1);
    compile_stm(f, a);
    f.emitop(OpcodeType::OP_ENDTRY);
    //f.delete_scope();
    compile_stm(f, b);
}

fn compile_finally(f: &mut VMFunction, a: &AstNode, b: &AstNode) {
    let L1:usize;
    L1 = f.emitjump(OpcodeType::OP_TRY);
    {
        /* if we get here, we have caught an exception in the try block */
        compile_stm(f, b);
        f.emitop(OpcodeType::OP_THROW);
    }
    f.label_current_to(L1);
    compile_stm(f, a);
    f.emitop(OpcodeType::OP_ENDTRY);
    compile_stm(f, b);
} 

/* Switch */
fn compile_switch(f: &mut VMFunction, stm: &AstNode) {
    let mut def = None;

    compile_exp(f, stm.a());

    let mut case_jumps = Vec::new();

    if stm.has_b() {
        let it = stm.b().iter();
        for n in it {
            let clause = n.a();
            if clause.ast_type == AstType::STM_CASE {
                compile_exp(f, clause.a());
                let addr = f.emitjump(OpcodeType::OP_JCASE);
                case_jumps.push(addr);
            } else if clause.ast_type == AstType::STM_DEFAULT {
                if !def.is_none() {
                    panic!("more than one default label in switch");
                }
                def = Some(n);
            } else {
                panic!("Case list only support STM_CASE and STM_DEFAULT!");
            }
        }
    }

    f.emitop(OpcodeType::OP_POP);
    let last_jump = f.emitjump(OpcodeType::OP_JUMP);

    if stm.has_b() {
        let mut i:usize = 0;

        let it = stm.b().iter();
        for n in it {
            let clause = n.a();
            if clause.ast_type == AstType::STM_CASE {
                let addr = case_jumps[i];
                f.label_current_to(addr);
                compile_stmlist(f, clause.b());
                i = i + 1;
            } else if clause.ast_type == AstType::STM_DEFAULT {
                f.label_current_to(last_jump);
                compile_stmlist(f, clause.a());
            }
        }
    }

    if def.is_none() {
        f.label_current_to(last_jump);
    }
}

/* Statements */
fn compile_varinit(f: &mut VMFunction, lst: &AstNode) {
    let it = lst.iter();
    for n in it {
        if n.has_b() {
            compile_exp(f, n.b());
            let var_str = n.a().str_value.as_ref().unwrap();
            f.emitlocal(OpcodeType::OP_SETLOCAL, OpcodeType::OP_SETVAR, var_str); 
            f.emitop(OpcodeType::OP_POP);
        }
    }
}

fn compile_assignforin(f: &mut VMFunction, stm: &AstNode) {
    let lhs = stm.a();
    if stm.ast_type == AstType::STM_FOR_IN_VAR {
        if !lhs.is_list() {
            panic!("for var in statement must include an var list!");
        }
        if lhs.has_b() {
            panic!("more than one loop variable in for-in statement");
        }
        let var = lhs.a().str_value.as_ref().unwrap();
        f.emitlocal(OpcodeType::OP_SETLOCAL, OpcodeType::OP_SETVAR, var);
        f.emitop(OpcodeType::OP_POP);
        return;
    }

    if lhs.ast_type != AstType::EXP_IDENTIFIER {
        panic!("invalid l-value in for-in loop assignment");
    }

    let var = lhs.str_value.as_ref().unwrap();
    f.emitlocal(OpcodeType::OP_SETLOCAL, OpcodeType::OP_SETVAR, var);
    f.emitop(OpcodeType::OP_POP);
    return;
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
            
            compile_exit(f, break_scope - 1, AstType::STM_BREAK);
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

            compile_exit(f, continue_scope - 1, AstType::STM_CONTINUE);
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

        let name_str = name.str_value.as_ref().unwrap();

        let (found, localid) = f.findlocal( name_str );
        if !found {
            f.emitop(OpcodeType::OP_CURRENT);
            f.emitop(OpcodeType::OP_SETLOCAL);
            let id = f.addlocal(name);
            f.emit(id);
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

