use crate::common::*;
use crate::ast::*;

/* Local help function and struct */

fn checkfutureword(_name: &str) {

}

struct AstListIterator<'a> {
    begin:  &'a AstNode,
    cursor: Option<&'a AstNode>
}

impl<'a> AstListIterator<'a> {
    pub fn new(lst: &'a AstNode ) -> Self {
        assert!(lst.ast_type == AstType::AST_LIST);
        return AstListIterator {
            begin: lst,
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
        }
    }

    fn emit(&mut self, value: u16) {
        self.code.push(value);
    }

    fn emitop(&mut self, op: OpcodeType) {
        self.code.push(op as u16);
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

fn compile_func(_name: &AstNode, params: &AstNode, body: &AstNode, script: bool) -> Result<VMFunction, String> {
    let mut f = VMFunction::new(script);

    // parsing params
    if !params.is_null() {       
        let it = params.iter();
        for node in it {
            f.numparams = f.numparams + 1;
            f.addlocal(node);
        }
    }

    if !body.is_null() {
		f.parsing_vardec(body);
		//cfundecs(J, F, body);
	}

    return Ok(f);
}

pub fn build_function_from_code(script: &str) -> Result<VMFunction, String> {
    let ast = build_ast_from_script(script).unwrap();

    let null = AstNode::null();
    let func = compile_func(&null, &null, &ast, false)?;
    return Ok(func);
}

