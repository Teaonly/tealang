use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

/*
 *  Data Struct
 */

#[derive(Debug)]
pub enum ExpErr {
    Reason(String),
}

macro_rules! builderr {
    ($msg: expr) => {
        Err(ExpErr::Reason($msg.to_string()))
    };
}

#[derive(Clone)]
pub enum ExpNode {
    TSymbol(String),
    TNull(()),
    TBool(bool),
    TLong(i64),
    TDouble(f64),
    TPattern(String),
    TMap(Rc<HashMap<String, ExpNode>>),
    TVec(Rc<RefCell<Vec<ExpNode>>>),
    TList(Vec<ExpNode>),
    TFunc(fn(&[ExpNode], &mut ExpEnv) -> Result<ExpNode, ExpErr>),
    TLambda(ExpLambda),
}

#[derive(Clone)]
pub struct ExpLambda {
    head:       Rc<Vec<ExpNode>>,
    body:       Rc<ExpNode>,

    //closure:    Rc<RefCell<HashMap<String, ExpNode>>>,
}

#[derive(Clone)]
pub struct ExpEnv<'a> {
    macros: Rc<RefCell<HashMap<String, (Vec<ExpNode>, Vec<ExpNode>)>>>,
    data: Rc<RefCell<HashMap<String, ExpNode>>>,
    outer: Option<&'a ExpEnv<'a>>,
}

impl fmt::Display for ExpNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let str = match self {
      ExpNode::TBool(v) => v.to_string(),
      ExpNode::TSymbol(v) => v.to_string(),
      ExpNode::TLong(v) => v.to_string(),
      ExpNode::TDouble(v) => v.to_string(),
      ExpNode::TPattern(v) => v.to_string(),
      ExpNode::TNull(_) => "null".to_string(),
      ExpNode::TMap(m) => {
        let mut str = "{".to_string();
        for (k, v) in m.as_ref() {
            str.push_str(k);
            str.push_str(" : ");
            str.push_str(&v.to_string());
            str.push_str(",");
        }
        str.push_str("}");
        str
      }
      ExpNode::TVec(v) => {
        let xs: Vec<String> = v
          .borrow()
          .iter()
          .map(|x| x.to_string())
          .collect();
        format!("[{}]", xs.join(","))
      },
      ExpNode::TList(list) => {
        let xs: Vec<String> = list
          .iter()
          .map(|x| x.to_string())
          .collect();
        format!("({})", xs.join(","))
      },
      ExpNode::TFunc(_) => "Func {}".to_string(),
      ExpNode::TLambda(_) => "Lambda {}".to_string(),
    };

    write!(f, "{}", str)
  }
}


/*
 *  Parser
 */
fn parse_atom(token: &String) -> Result<ExpNode, ExpErr> {
    if token == "false" {
        return Ok(ExpNode::TBool(false));
    }

    if token == "true" {
        return Ok(ExpNode::TBool(true));
    }

    if    token.contains("\"")
       || token.contains("#")
       || token.contains("$")
       || token.contains("^")
       || token.contains("`")
       || token.contains(",")
       || token.contains("|")
       || token.contains("\\")
       || token.contains("~") {
        return builderr!("Token finds special symbol, like \" @ $ #")
    }

    if token.starts_with("@") {
        if token == "@" {
            return builderr!("Pattern token must begin with @ and other charaters")
        }
        let mut pattern = token.clone();
        pattern.remove(0);
        if pattern.contains("@") {
            return builderr!("Pattern token can't contain ', except begin")
        }
        return Ok(ExpNode::TPattern(pattern));
    }

    let lv = token.parse::<i64>();
    if lv.is_ok() {
        return Ok(ExpNode::TLong(lv.unwrap()));
    }

    let dv = token.parse::<f64>();
    if dv.is_ok() {
        return Ok(ExpNode::TDouble(dv.unwrap()));
    }

    return Ok(ExpNode::TSymbol(token.clone()));
}

fn parse_tokens(pos: usize, tokens: &Vec<String>) -> Result<(ExpNode, usize), ExpErr> {
    let mut ret:Vec<ExpNode> = vec![];
    let mut i = pos;

    if tokens[i] != "(" {
        return builderr!("Tokens's paren not begin with '(' ");
    }
    i = i + 1;

    loop {
        if i >= tokens.len() {
            break;
        }

        // 0. check is end of a list
        let token = &tokens[i];
        if token == ")" {
            return Ok((ExpNode::TList(ret), i+1))
        }

        // 1. an new list begin
        if token == "(" {
            let next = parse_tokens(i, tokens);
            if let Err(reason) = next {
                return Err(reason)
            }
            let (node, end) = next.unwrap();

            ret.push(node);
            i = end;
            continue;
        }

        // 2. an new symbol
        let node = parse_atom(&tokens[i]);
        if let Err(reason) = node {
            return Err(reason)
        }
        ret.push(node.unwrap());
        i = i + 1;
    }

    return builderr!("Tokens not end with ')'");
}

fn parse(expr: &String) -> Result<Vec<ExpNode>, ExpErr> {
    fn tokenize(expr: &String) -> Vec<String> {
          expr.replace("{", "(map ")
              .replace("}", ")")
              .replace("[", "(vec ")
              .replace("]", ")")
              .replace("(", " ( ")
              .replace(")", " ) ")
              .replace("\n", " ")
              .split_whitespace()
              .map(|x| x.to_string())
              .collect()
    }

    let mut ret : Vec<ExpNode> = Vec::new();
    let tokens = tokenize(expr);

    if tokens.len() > 1 {
        let mut i:usize = 0;
        while i < tokens.len() {
            match parse_tokens(i, &tokens) {
                Err(e) => {
                    return Err(e);
                },
                Ok(r) => {
                    let (v, pos) = r;
                    ret.push(v);
                    i = pos;
                }
            }
        }
    } else if tokens.len() == 1 {
        let n =  parse_atom(&tokens[0])?;
        ret.push(n);
    }
    Ok(ret)
}

/*
 *  Enviroment
 */
impl<'a> ExpEnv<'a> {
    fn set(&mut self, name : &String, node: &ExpNode) {
        self.data.borrow_mut().insert(name.clone(), node.clone());
    }
    fn get(&self, name : &String) -> Option<ExpNode> {
        if name == "x" {
            println!("=====");
        }
        match self.data.borrow().get(name) {
             Some(exp) => Some(exp.clone()),
             None => {
                 match &self.outer {
                     Some(outer_env) => outer_env.get(name),
                     None => None
                 }
             }
         }
    }
    fn add_macro(&self, name: &String, head: &Vec<ExpNode>, body: &Vec<ExpNode>) {
        let new_macro = (head.clone(), body.clone());
        self.macros.borrow_mut().insert(name.clone(), new_macro);
    }
    fn find_macro(&self, name: &String) -> Option<(Vec<ExpNode>, Vec<ExpNode>)> {
        match self.macros.borrow().get(name) {
            None => None,
            Some(ref v) => Some( (v.0.clone(), v.1.clone()) ),
        }
    }
}

pub fn env_new<'a>() -> ExpEnv<'a> {
    let mut data: HashMap<String, ExpNode> = HashMap::new();
    init_env(&mut data);
    let data = Rc::new(RefCell::new(data));

    let macros:HashMap<String, (Vec<ExpNode>, Vec<ExpNode>)> = HashMap::new();
    let macros = Rc::new(RefCell::new(macros));

    let outer = None;
    let env = ExpEnv {macros, data, outer};
    env
}

fn init_env(data: &mut HashMap<String, ExpNode>) {

    // math algorithm
    let add = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        let mut total:i64 = 0;
        for arg in args.iter() {
            match arg {
                ExpNode::TLong(v) => { total = total + v},
                _ => {return Err( ExpErr::Reason("+ only support TLong".to_string()) );}
            }
        }
        Ok( ExpNode::TLong( total))
    });
    data.insert("+".to_string(), add);

    let sub = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TLong(v1 - v2));
                }
            }
        }
        builderr!("mod only support two Tlong")
    });
    data.insert("-".to_string(), sub);

    let mod_func = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TLong(v1 % v2));
                }
            }
        }
        builderr!("mod only support two Tlong")
    });
    data.insert("%".to_string(), mod_func);

    let mul = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        let mut acc:i64 = 1;
        for arg in args.iter() {
            match arg {
                ExpNode::TLong(v) => { acc = acc * v},
                _ => {return Err( ExpErr::Reason("+ only support TLong".to_string()) );}
            }
        }
        Ok( ExpNode::TLong(acc))
    });
    data.insert("*".to_string(), mul);

    let div = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TLong(v1 / v2));
                }
            }
        }
        builderr!("div only support two Tlong")
    });
    data.insert("/".to_string(), div);

    // relation compair
    let gt = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TBool(v1 > v2));
                }
            }
        }
        builderr!("> only support two Tlong")
    });
    data.insert(">".to_string(), gt);

    let gte = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TBool(v1 >= v2));
                }
            }
        }
        builderr!(">= only support two Tlong")
    });
    data.insert(">=".to_string(), gte);

    let lt = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TBool(v1 < v2));
                }
            }
        }
        builderr!("< only support two Tlong")
    });
    data.insert("<".to_string(), lt);

    let lte = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TLong(v1) = args[0] {
                if let ExpNode::TLong(v2) = args[1] {
                    return Ok(ExpNode::TBool(v1 <= v2));
                }
            }
        }
        builderr!("<= only support two Tlong")
    });
    data.insert("<=".to_string(), lte);

    let eq = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            match &args[0] {
                ExpNode::TLong(ref v1) => {
                    if let ExpNode::TLong(ref v2) = args[1] {
                        return Ok(ExpNode::TBool(v1 == v2));
                    }
                },
                ExpNode::TPattern(ref v1) => {
                    if let ExpNode::TPattern(ref v2) = args[1] {
                        return Ok(ExpNode::TBool(v1 == v2));
                    }
                },
                _=> (),
            }
        }
        builderr!("== only support two Tlong")
    });
    data.insert("==".to_string(), eq);

    // logic operator
    let and = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        let mut result:bool = true;
        for arg in args.iter() {
            match arg {
                ExpNode::TBool(v) => { result = result && *v},
                _ => {return Err( ExpErr::Reason("and only support TBool".to_string()) );}
            }
        }
        Ok( ExpNode::TBool(result))
    });
    data.insert("&&".to_string(), and);

    let or = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        let mut result:bool = false;
        for arg in args.iter() {
            match arg {
                ExpNode::TBool(v) => { result = result || *v},
                _ => {return Err( ExpErr::Reason("or only support TBool".to_string()) );}
            }
        }
        Ok( ExpNode::TBool(result))
    });
    data.insert("||".to_string(), or);

    let not = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 1 {
            if let ExpNode::TBool(v1) = args[0] {
                return Ok( ExpNode::TBool(!v1) );
            }
        }
        builderr!("! only support one TBool")
    });
    data.insert("!".to_string(), not);

    // vector operators
    let push = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() < 2 {
            return builderr!("push to vector must include 2 items");
        }
        if let ExpNode::TVec(ref vec) = args[0] {
            for i in 1..args.len() {
                vec.borrow_mut().push( args[i].clone() );
            }
            return Ok(args[0].clone());
        }
        builderr!("push to vector syntax error")
    });
    data.insert("push".to_string(), push);

    let pop = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 1 {
            if let ExpNode::TVec(ref vec) = args[0] {
                if let Some(n) = vec.borrow_mut().pop() {
                    return Ok(n);
                }
                return builderr!("pop an empyt vector");
            }
        }
        builderr!("pop from vector syntax error")
    });
    data.insert("pop".to_string(), pop);

    let nth = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 2 {
            if let ExpNode::TVec(ref vec) = args[0] {
                if let ExpNode::TLong(i) = args[1] {
                    let i = i as usize;
                    if i < vec.borrow().len() {
                        return Ok(  vec.borrow()[i].clone() );
                    } else {
                        return builderr!("index out of vector");
                    }
                }
            } else if let ExpNode::TList(ref lst) = args[0] {
                if let ExpNode::TLong(i) = args[1] {
                    let i = i as usize;
                    if i < lst.len() {
                        return Ok( lst[i].clone() );
                    } else {
                        return builderr!("index out of vector");
                    }
                }
            }
        }
        builderr!("nth vector syntax error")
    });
    data.insert("nth".to_string(), nth);

    let size = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() == 1 {
            if let ExpNode::TVec(ref vec) = args[0] {
                return Ok(ExpNode::TLong( vec.borrow().len() as i64));
            } else if let ExpNode::TList(ref lst) = args[0] {
                return Ok(ExpNode::TLong( lst.len() as i64));
            }
        }
        builderr!("size of vector syntax error")
    });
    data.insert("size".to_string(), size);

    let append = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() < 2 {
            return builderr!("append vector must include 2 items at least");
        }
        if let ExpNode::TVec(ref vec) = args[0] {
            for i in 1..args.len() {
                match &args[i] {
                    ExpNode::TVec(ref nl) => vec.borrow_mut().extend( nl.borrow().iter().cloned() ),
                    _ => return builderr!("concat support combin vector only")
                }
            }
            return Ok( args[0].clone() );
        }
        builderr!("append to vector syntax error")
    });
    data.insert("append".to_string(), append);

    let concat = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        if args.len() < 2 {
            return builderr!("concat vector must include 2 items at least");
        }
        let vec : Rc<RefCell<Vec<ExpNode>>>  = Rc::new(RefCell::new(Vec::new()));
        for i in 0..args.len() {
            match &args[i] {
                ExpNode::TVec(ref nl) => vec.borrow_mut().extend( nl.borrow().iter().cloned() ),
                _ => return builderr!("concat support combin vector only")
            }
        }
        Ok( ExpNode::TVec(vec))
    });
    data.insert("concat".to_string(), concat);

    let evalfn = ExpNode::TFunc( |args: &[ExpNode], env: &mut ExpEnv| {
        if args.len() != 1 {
            return builderr!("eval only support onte item");
        }
        eval(&args[0], env)
    });
    data.insert("eval".to_string(), evalfn);

    let listfn = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        let mut lst:Vec<ExpNode> = vec![];
        for arg in args.iter() {
            lst.push(arg.clone());
        }
        return Ok(ExpNode::TList(lst));
    });
    data.insert("list".to_string(), listfn);

    let probe = ExpNode::TFunc( |args: &[ExpNode], _env: &mut ExpEnv| {
        for i in 0..args.len() {
            println!("{}", args[i]);
        }
        Ok(ExpNode::TNull(()))
    });
    data.insert("probe".to_string(), probe);
}

/*
 *  Evaluate
 */

fn eval_def(args: &[ExpNode], env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() != 2 {
        return builderr!("let must follow with two item");
    }
    if let ExpNode::TSymbol(ref name) = args[0] {
        let value = eval(&args[1], env)?;
        env.set(name, &value);
        return Ok(ExpNode::TNull(()));
    }
    return builderr!("let syntax error!");
}

fn eval_begin(args: &[ExpNode], env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() == 0 {
        return builderr!("begin need at least one item to eval.");
    }
    for i in 0..args.len()-1 {
        eval(&args[i], env)?;
    }
    eval(&args[args.len()-1], env)
}

fn eval_if(args: &[ExpNode], env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() < 2 || args.len() > 3 {
        return builderr!("if need two or three item to eval.");
    }

    if let ExpNode::TBool(b) = eval(&args[0], env)? {
        if b {
            return eval(&args[1], env);
        } else {
            if args.len() == 3 {
                return eval(&args[2], env);
            }
            return Ok(ExpNode::TNull(()))
        }
    }
    builderr!("if first item must be an TBool")
}

fn eval_while(args: &[ExpNode], env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() < 1 || args.len() > 2 {
        return builderr!("while need one or two item to eval.");
    }

    loop {
        if let ExpNode::TBool(b) = eval(&args[0], env)? {
            if b && args.len() == 2 {
                eval(&args[1], env)?;
            } else if b == false {
                break;
            }
        } else {
            break;
        }
    }

    Ok(ExpNode::TNull(()))
}

fn eval_map(args: &[ExpNode], env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() % 2 != 0 {
        return builderr!("map must include even items");
    }

    let mut map: HashMap<String, ExpNode> = HashMap::new();
    for i in 0..args.len()/2 {
        if let ExpNode::TPattern(key) = eval(&args[i*2], env)? {
            let value = eval(&args[i*2+1], env)?;
            map.insert(key, value);
        } else {
            return builderr!("map only support @pattern as key")
        }
    }

    Ok(ExpNode::TMap(Rc::new(map)))
}

fn eval_vec(args: &[ExpNode], env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    let mut vec: Vec<ExpNode> = Vec::new();

    for i in 0..args.len() {
        let n = eval(&args[i], env)?;
        vec.push(n);
    }

    Ok(ExpNode::TVec(Rc::new(RefCell::new(vec))))
}

fn eval_lambda(args: &[ExpNode], _env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() != 2 {
        return builderr!("fn must include two list");
    }

    if let ExpNode::TList(ref body) = args[1] {
        if body.len() == 0 {
            return builderr!("fn body can't be empty");
        }
        if let ExpNode::TSymbol(_) = args[0] {
            let mut head : Vec<ExpNode> = Vec::new();
            head.push( args[0].clone() );

            let head = Rc::new(head);
            let body = Rc::new(ExpNode::TList(body.clone()));

            let lambda = ExpLambda{head, body};
            return Ok(ExpNode::TLambda(lambda));
        }
        if let ExpNode::TList(ref head) = args[0] {
            for i in 0..head.len() {
                match &head[i] {
                    ExpNode::TSymbol(_) => (),
                    _ => {
                        return builderr!("lambda's argment must be a symbol");
                    }
                }
            }

            let head = Rc::new(head.clone());
            let body = Rc::new(ExpNode::TList(body.clone()));

            let lambda = ExpLambda{head, body};
            return Ok(ExpNode::TLambda(lambda));
        }
    }
    return builderr!("fn must include two list or symbol and list");
}

fn eval_quote(args: &[ExpNode], _env: &mut ExpEnv) -> Result<ExpNode, ExpErr> {
    if args.len() != 1  {
        return builderr!("quote must followed one symbol");
    }

    Ok(args[0].clone())
}

fn eval_builtin(head: &ExpNode,
                args: &[ExpNode],
                env: &mut ExpEnv) -> Option<Result<ExpNode, ExpErr>> {
    match head {
        ExpNode::TSymbol(s) => {
            match s.as_ref() {
                "def" => Some(eval_def(args, env)),
                "begin" => Some(eval_begin(args, env)),
                "if" => Some(eval_if(args, env)),
                "while" => Some(eval_while(args, env)),
                "map" => Some(eval_map(args, env)),
                "vec" => Some(eval_vec(args, env)),
                "fn" => Some(eval_lambda(args, env)),
                "'" => Some(eval_quote(args, env)),
                _ => None,
            }
        },
        _ => None
    }
}


fn eval<'a>(exp: &ExpNode, env: &mut ExpEnv<'a>) -> Result<ExpNode, ExpErr> {
    match exp {
        // atom type
        ExpNode::TNull(_) => Ok(exp.clone()),
        ExpNode::TBool(_) => Ok(exp.clone()),
        ExpNode::TLong(_) => Ok(exp.clone()),
        ExpNode::TDouble(_) => Ok(exp.clone()),
        ExpNode::TPattern(_) => Ok(exp.clone()),
        ExpNode::TLambda(_) => Ok(exp.clone()),
        // can't eval
        ExpNode::TVec(_) => {
            builderr!("Can't eval Vec directly, it is not atom type")
        },
        ExpNode::TMap(_) => {
            builderr!("Can't eval Map directly, it is not atom type")
        }
        ExpNode::TFunc(_) => {
            builderr!("Can't eval TFunc directly, it is not atom type")
        }
        // from env
        ExpNode::TSymbol(v) => {
            if let Some(node) = env.get(v) {
                return Ok(node);
            }
            builderr!("Can't find symbol")
        },
        // execute
        ExpNode::TList(list) => {
            // get head
            let head = list
                .first()
                .ok_or(ExpErr::Reason("expected a non-empty list".to_string()))?;

            // step.1 check is a builtin
            let args = &list[1..];
            if let Some(result) = eval_builtin(head, args, env) {
                return result;
            }

            // step.2 eval all the args
            let mut args : Vec<ExpNode> = Vec::new();
            for i in  1..list.len() {
                match eval(&list[i], env) {
                    Ok(n) => args.push(n),
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            // step.3 apply function
            let head : ExpNode = eval(&head, env)?;
            match head {
                ExpNode::TPattern(k) => {
                    // map operator, unmutable reading
                    if args.len() == 1 {
                        if let ExpNode::TMap(ref m) = args[0] {
                            match m.as_ref().get(&k) {
                                None => {
                                    return builderr!("can't find patter(key) in map");
                                },
                                Some(node) => {
                                    return Ok(node.clone());
                                }
                            }
                        }
                    }
                    return builderr!("TPattern map syntax error");
                },
                ExpNode::TFunc(f) => {
                    f(&args, env)
                },
                ExpNode::TLambda(f) => {
                    // copy args to head
                    let mut data: HashMap<String, ExpNode> = HashMap::new();
                    if f.head.len() != args.len() {
                        return builderr!("apply lambda must with same args number");
                    }
                    for i in 0..args.len() {
                        if let ExpNode::TSymbol(ref name) = f.head.as_ref()[i] {
                            data.insert(name.clone(), args[i].clone());
                        } else {
                            panic!("Find lambda with non symble args");
                        }
                    }
                    let mut env2 = ExpEnv{ macros:  env.macros.clone(),
                                           data:    Rc::new(RefCell::new(data)),
                                           outer:   Some(env)};
                    eval( f.body.as_ref(), &mut env2)
                }
                _ => {
                    builderr!("Can't eval none function in list first item")
                }
            }
        }
    }
}

fn check_macro(exp: &ExpNode, top_env: &mut ExpEnv) -> Result<bool, String> {
    if let ExpNode::TList(ref lst) = exp {
        if lst.len() >= 1 {
            if let ExpNode::TSymbol(ref defmacro) = lst[0] {
                if defmacro == "defmacro" {
                    if lst.len() != 4 {
                        return Err("defmacro syntax error".to_string());
                    }
                    if let ExpNode::TSymbol(ref name) = lst[1] {
                        if let ExpNode::TList(ref head) = lst[2] {
                            if let ExpNode::TList(ref body) = lst[3] {
                                top_env.add_macro(name, head, body);
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
        for i in 0..lst.len() {
            if let ExpNode::TSymbol(ref defmacro) = lst[i] {
                if defmacro == "defmacro" {
                    return Err("defmacro must be used in global env".to_string());
                }
            }
        }
    }
    Ok(false)
}

fn expand_macro(body: &[ExpNode], head: &[ExpNode], args: &[ExpNode]) -> Vec<ExpNode> {
    let mut ret: Vec<ExpNode> = Vec::new();
    for i in 0..body.len() {
        if let ExpNode::TList(ref new_body) = body[i] {
            ret.push(ExpNode::TList( expand_macro(new_body, head, args)));
        } else if let ExpNode::TSymbol(ref item) = body[i] {
            let mut find : i32 = -1;
            for j in 0..head.len() {
                if let ExpNode::TSymbol(ref arg) = head[j] {
                    if item == arg {
                        find = j as i32;
                        break;
                    }
                }
            }
            if find == -1 {
                ret.push(body[i].clone());
            } else {
                ret.push(args[find as usize].clone());
            }
        } else {
            ret.push(args[i].clone());
        }
    }
    ret
}

fn compile_macro(exp: &ExpNode, top_env: &ExpEnv) -> Result<ExpNode, ExpErr> {
    if let ExpNode::TList(ref lst) = exp {
        if lst.len() >= 1 {
            if let ExpNode::TSymbol(ref macro_name) = lst[0] {
                if macro_name == "'" {
                    return Ok(exp.clone());
                }
                if let Some((head, body)) = top_env.find_macro(macro_name) {
                    if lst.len() != head.len() + 1 {
                        return builderr!("macro expand error");
                    }
                    // macroexpand
                    let expand = ExpNode::TList( expand_macro(&body, &head, &lst[1..]));
                    return compile_macro(&expand, top_env)
                }
            }

            let mut new_list:Vec<ExpNode> = vec![];
            for i in 0.. lst.len() {
                new_list.push( compile_macro(&lst[i], top_env)? );
            }
            return Ok( ExpNode::TList(new_list));
        }
    }
    return Ok(exp.clone());
}

pub fn run(code : &String, env: &mut ExpEnv) -> String {
    let nodes = parse(code);
    if let Err(e) = nodes {
        let ExpErr::Reason(estr) = e;
        return format!("ERR:{}", estr);
    }
    let nodes = nodes.unwrap();

    let mut ret:ExpNode = ExpNode::TNull(());
    for n in &nodes {
        let r = check_macro(n, env);
        if r.is_ok() {
            // check defmacro
            let r = r.unwrap();
            if r {
                continue;
            }

            // compile macro
            let n = compile_macro(n, env);
            if let Err(e) = n {
                let ExpErr::Reason(estr) = e;
                return format!("ERR:{}", estr);
            }
            let n = n.unwrap();

            // execute lisp code
            let r = eval(&n, env);
            if let Err(e) = r {
                let ExpErr::Reason(estr) = e;
                return format!("ERR:{}", estr);
            }
            ret = r.unwrap();
        } else if let Err(msg) = r {
            return format!("ERR:{}", msg);
        }
    }
    ret.to_string()
}

