#pragma once

#include <assert.h>
#include <string>
#include <memory>
#include <list>
#include <vector>
#include <map>
#include <iterator>
#include <iostream>
#include <sstream>

namespace tea {
using namespace std;

struct _TeaExtern {};
typedef struct _TeaExtern TeaExtern;

struct TeaEnvironment;
struct TeaObject;
struct TeaResult;

typedef shared_ptr<TeaObject> tobject;
typedef shared_ptr<TeaEnvironment> tenv;

typedef TeaResult (*TeaFunc) (vector<tobject> &args, tenv &env);
struct TeaLambda {
    vector<tobject>  head;
    tobject body;

    shared_ptr<map<string, tobject>> closure;

    TeaLambda() {
    }
};

struct TeaObject {
    enum TeaType {
        T_NULL,
        T_INT,
        T_FLOAT,
        T_BOOL,
        T_PATTERN,
        T_LIST,
        T_MAP,
        T_EXT,
        T_SYMBOL,
        T_LAMBDA,
        T_FUNC,
    };
    const TeaType type;

    // user values
    int64_t                                 v_int;
    float                                   v_float;
    bool                                    v_bool;
    const string                            v_string;
    const shared_ptr<vector<tobject>>       v_list;
    const shared_ptr<map<string, tobject>>  v_map;
    const shared_ptr<TeaExtern>             v_ext;

    // internal values
    const shared_ptr<TeaLambda>         v_lambda;
    TeaFunc                             v_func;

    TeaObject():type(T_NULL) {}
    TeaObject(int64_t value):type(T_INT) {v_int = value;}
    TeaObject(float value):type(T_FLOAT) {v_float = value;}
    TeaObject(bool value):type(T_BOOL), v_bool(value) {}
    TeaObject(const string& value):type(T_PATTERN), v_string(value) {}
    TeaObject(shared_ptr<vector<tobject>> value):type(T_LIST), v_list(value) {}
    TeaObject(shared_ptr<map<string, tobject>> value):type(T_MAP), v_map(value) {}
    TeaObject(shared_ptr<TeaExtern> value):type(T_EXT), v_ext(value) {}
    TeaObject(shared_ptr<TeaLambda> value):type(T_LAMBDA), v_lambda(value) {}
    TeaObject(TeaFunc value):type(T_FUNC) {v_func = value;}
    TeaObject(TeaType t, const string& symbol):type(T_SYMBOL), v_string(symbol) {
        assert(t == T_SYMBOL);
    }

    string to_string() {
        stringstream ss;
        if (type == T_NULL) {
            return "(nil)";
        }
        if (type == T_INT) {
            ss << v_int;
        }
        if (type == T_FLOAT) {
            ss << v_float;
        }
        if (type == T_BOOL) {
            ss << v_bool;
        }
        if (type == T_PATTERN) {
            ss << "@" << v_string;
        }
        if (type == T_EXT) {
            ss << "(extern)";
        }
        if (type == T_LIST) {
            ss << "(list with " << v_list->size() << " items)";
        }
        if (type == T_MAP) {
            ss << "(map with " << v_map->size() << " items)";
        }
        if (type == T_SYMBOL) {
            ss << v_string;
        }
        if (type == T_FUNC) {
            ss << "(func)";
        }
        if (type == T_LAMBDA) {
            ss << "(lambda)";
        }
        return ss.str();
    }

    static tobject build(const TeaObject& obj) {
        return std::make_shared<TeaObject>(obj);
    }
};

const auto tea_null = TeaObject::build(TeaObject());
const auto tea_true = TeaObject::build(TeaObject(true));
const auto tea_false = TeaObject::build(TeaObject(false));

struct TeaEnvironment {
    shared_ptr<map<string, tobject>> global;
    shared_ptr<map<string, tobject>> closure;
    shared_ptr<map<string, tobject>> data;

    // global enviroment
    TeaEnvironment() {
        data = make_shared<map<string, tobject>>();
    }

    // new env for lambda
    TeaEnvironment(tenv& out_env, shared_ptr<map<string, tobject>> lambda_closue) {
        if (out_env->is_global()) {
            global = out_env->data;
        } else {
            global = out_env->global;
        }
        closure = lambda_closue;
        data = make_shared<map<string, tobject>>();
    }

    bool is_global() {
        if (global == nullptr) {
            return true;
        }
        return false;
    }

    void set(const string& key, tobject obj) {
        (*data)[key] = obj;
    }

    tobject get(const string& key) {
        auto query = data->find(key);
        if (query != data->end()) {
            return query->second;
        }

        if (closure != nullptr) {
            auto query = closure->find(key);
            if (query != closure->end() ) {
                return query->second;
            }
        }

        if (global != nullptr) {
            auto query = global->find(key);
            if (query != global->end() ) {
                return query->second;
            }
        }

        return tea_null;
    }
};

struct TeaResult {
    vector<string>  trace;;
    tobject         result;

    bool is_error() {
        if (trace.size() == 0) {
            return false;
        }
        return true;
    }
    bool is_ok() {
        return !is_error();
    }

    TeaResult(tobject obj) {
        result = obj;
    }

    TeaResult(string err_message) {
        trace.push_back(err_message);
    }

    TeaResult(TeaResult& r, string& err_message) {
        trace = r.trace;
        trace.push_back(err_message);
    }
};

struct TeaLang {
private:
    // main functions
    static void init(shared_ptr<map<string, tobject>> data_) {
        map<string, tobject>& data(*data_);

        data["+"] = build_fobj(TeaLang::add);
        data["-"] = build_fobj(TeaLang::sub);
        data["*"] = build_fobj(TeaLang::mul);
        data["/"] = build_fobj(TeaLang::div);
        data["%"] = build_fobj(TeaLang::mod);

        data["++"] = build_fobj(TeaLang::inc);
        data["--"] = build_fobj(TeaLang::dec);

        data[">"] = build_fobj(TeaLang::more);
        data["<"] = build_fobj(TeaLang::less);
        data[">="] = build_fobj(TeaLang::moreeq);
        data["<="] = build_fobj(TeaLang::lesseq);
        data["=="] = build_fobj(TeaLang::eqeq);
        data["!"] = build_fobj(TeaLang::notnot);

        data["size"] = build_fobj(TeaLang::size);
        data["push"] = build_fobj(TeaLang::push);
        data["pop"] = build_fobj(TeaLang::pop);
        data["nth"] = build_fobj(TeaLang::nth);

        data["set"] = build_fobj(TeaLang::set);
        data["has"] = build_fobj(TeaLang::has);
        data["rm"] = build_fobj(TeaLang::rm);
    }

    static tobject build_fobj(TeaFunc f) {
        return TeaObject::build(f);
    }

    // math
    static TeaResult add(vector<tobject> &args, tenv& env) {
        if (args.size() == 0) {
            return TeaResult("+ func synatax error, need more items");
        }
        if (args[0]->type == TeaObject::T_INT) {
            int64_t sum = 0;
            for (size_t i = 0; i < args.size(); i++) {
                if ( args[i]->type != TeaObject::T_INT) {
                    return TeaResult("+ func synatax error, only support int/float");
                }
                sum += args[i]->v_int;
            }
            return TeaObject::build(sum);
        }
        if (args[0]->type == TeaObject::T_FLOAT) {
            float sum = 0.0;
            for (size_t i = 0; i < args.size(); i++) {
                if ( args[i]->type != TeaObject::T_FLOAT) {
                    return TeaResult("+ func synatax error, only support int/float");
                }
                sum += args[i]->v_float;
            }
            return TeaObject::build(sum);
        }
        return TeaResult("+ func synatax error, only support int/float");
    }

    static TeaResult sub(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("- func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            auto result = args[0]->v_int - args[1]->v_int;
            return TeaObject::build(result);
        }
        if (args[0]->type == TeaObject::T_FLOAT && args[1]->type == TeaObject::T_FLOAT) {
            auto result = args[0]->v_float - args[1]->v_float;
            return TeaObject::build(result);
        }
        return TeaResult("+ func synatax error, only support int/float");
    }

    static TeaResult mul(vector<tobject> &args, tenv& env) {
        if (args.size() == 0) {
            return TeaResult("* func synatax error, need more items");
        }
        if (args[0]->type == TeaObject::T_INT) {
            int64_t result = 1;
            for (size_t i = 0; i < args.size(); i++) {
                if (args[i]->type != TeaObject::T_INT) {
                    return TeaResult("* func synatax error, only support int/float");
                }
                result *= args[i]->v_int;
            }
            return TeaObject::build(result);
        }
        if (args[0]->type == TeaObject::T_FLOAT) {
            float result = 1.0;
            for (size_t i = 0; i < args.size(); i++) {
                if (args[i]->type != TeaObject::T_FLOAT) {
                    return TeaResult("* func synatax error, only support int/float");
                }
                result *= args[i]->v_float;
            }
            return TeaObject::build(result);
        }
        return TeaResult("* func synatax error, only support int/float");
    }

    static TeaResult div(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("/ func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            auto result = args[0]->v_int / args[1]->v_int;
            return TeaObject::build(result);
        }
        if (args[0]->type == TeaObject::T_FLOAT && args[1]->type == TeaObject::T_FLOAT) {
            auto result = args[0]->v_float / args[1]->v_float;
            return TeaObject::build(result);
        }
        return TeaResult("/ func synatax error, only support int/float");
    }

    static TeaResult mod(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("mod func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            auto result = args[0]->v_int % args[1]->v_int;
            return TeaObject::build(result);
        }
        return TeaResult("mod func synatax error, only support int/float");
    }

    static TeaResult inc(vector<tobject> &args, tenv& env) {
        if (args.size() != 1) {
            return TeaResult("++ func synatax error, only support one items");
        }
        if (args[0]->type == TeaObject::T_INT) {
            args[0]->v_int = args[0]->v_int + 1;
            return args[0];
        }
        return TeaResult("++ func synatax error, only support int");
    }

    static TeaResult dec(vector<tobject> &args, tenv& env) {
        if (args.size() != 1) {
            return TeaResult("-- func synatax error, only support one items");
        }
        if (args[0]->type == TeaObject::T_INT) {
            args[0]->v_int = args[0]->v_int - 1;
            return args[0];
        }
        return TeaResult("-- func synatax error, only support int");
    }

    // comapir
    static TeaResult less(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("< func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            bool result = args[0]->v_int < args[1]->v_int;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        if (args[0]->type == TeaObject::T_FLOAT && args[1]->type == TeaObject::T_FLOAT) {
            bool result = args[0]->v_float < args[1]->v_float;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        return TeaResult("< func synatax error, only support int/float");
    }

    static TeaResult more(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("> func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            bool result = args[0]->v_int > args[1]->v_int;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        if (args[0]->type == TeaObject::T_FLOAT && args[1]->type == TeaObject::T_FLOAT) {
            bool result = args[0]->v_float > args[1]->v_float;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        return TeaResult("> func synatax error, only support int/float");
    }

    static TeaResult lesseq(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("<= func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            bool result = args[0]->v_int <= args[1]->v_int;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        return TeaResult("<= func synatax error, only support int");
    }

    static TeaResult moreeq(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult(">= func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            bool result = args[0]->v_int >= args[1]->v_int;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        return TeaResult(">= func synatax error, only support int");
    }

    static TeaResult eqeq(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult(">= func synatax error, need two items");
        }
        if (args[0]->type == TeaObject::T_INT && args[1]->type == TeaObject::T_INT) {
            bool result = args[0]->v_int == args[1]->v_int;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        if (args[0]->type == TeaObject::T_FLOAT && args[1]->type == TeaObject::T_FLOAT) {
            bool result = args[0]->v_float == args[1]->v_float;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        if (args[0]->type == TeaObject::T_BOOL && args[1]->type == TeaObject::T_BOOL) {
            bool result = args[0]->v_bool == args[1]->v_bool;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }
        if (args[0]->type == TeaObject::T_PATTERN && args[1]->type == TeaObject::T_PATTERN) {
            bool result = args[0]->v_string == args[1]->v_string;
            if (result == true) {
                return tea_true;
            } else {
                return tea_false;
            }
        }

        return TeaResult(">= func synatax error, only support int/float/bool/pattern");
    }

    // logic
    static TeaResult notnot(vector<tobject> &args, tenv& env) {
        if (args.size() != 1) {
            return TeaResult("! func synatax error, only support one items");
        }
        if (args[0]->type == TeaObject::T_BOOL) {
            if (args[0]->v_bool == true) {
                return tea_false;
            } else {
                return tea_true;
            }
        }
        return TeaResult("-- func synatax error, only support bool");
    }

    // list&map operation
    static TeaResult size(vector<tobject> &args, tenv& env) {
        if (args.size() != 1) {
            return TeaResult("size func synatax error, only support one items");
        }

        if (args[0]->type == TeaObject::T_LIST) {
            int64_t n = args[0]->v_list->size();
            return TeaObject::build(n);
        }
        if (args[0]->type == TeaObject::T_MAP) {
            int64_t n = args[0]->v_map->size();
            return TeaObject::build(n);
        }
        return TeaResult("size func synatax error, only support list/map");
    }

    static TeaResult push(vector<tobject> &args, tenv& env) {
        if (args.size() < 2) {
            return TeaResult("push func synatax error, only support two more items");
        }
        if (args[0]->type == TeaObject::T_LIST) {
            auto lst = args[0]->v_list;
            for (size_t i = 1; i < args.size(); i++) {
                lst->push_back(args[i]);
            }
            return tea_null;
        }
        return TeaResult("push func synatax error, only support list");
    }

    static TeaResult pop(vector<tobject> &args, tenv& env) {
        if (args.size() != 1) {
            return TeaResult("pop func synatax error, only support one");
        }
        if (args[0]->type == TeaObject::T_LIST) {
            auto lst = args[0]->v_list;

            if (lst->size() > 0) {
                auto last = lst->back();
                lst->pop_back();
                return last;
            }
            return tea_null;
        }
        return TeaResult("pop func synatax error, only support list");
    }

    static TeaResult nth(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("push func synatax error, only support two items");
        }
        if (args[0]->type == TeaObject::T_LIST) {
            auto lst = args[0]->v_list;
            if (args[1]->type == TeaObject::T_INT) {
                auto pos = args[1]->v_int;
                return (*lst)[pos];
            }
        }
        return TeaResult("nth func synatax error!");
    }

    static TeaResult set(vector<tobject> &args, tenv& env) {
        if (args.size() != 3) {
            return TeaResult("set func synatax error, only support two items");
        }
        if (args[0]->type == TeaObject::T_MAP) {
            auto hash = args[0]->v_map;
            if (args[1]->type == TeaObject::T_PATTERN) {
                auto &key = args[1]->v_string;
                (*hash)[key] = args[2];
                return tea_null;
            }
        }
        return TeaResult("set func synatax error!");
    }

    static TeaResult has(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("has func synatax error, only support two items");
        }
        if (args[0]->type == TeaObject::T_MAP) {
            auto hash = args[0]->v_map;
            if (args[1]->type == TeaObject::T_PATTERN) {
                auto &key = args[1]->v_string;
                if (hash->find(key) == hash->end()) {
                    return tea_false;
                } else {
                    return tea_true;
                }
            }
        }
        return TeaResult("has func synatax error!");
    }

    static TeaResult rm(vector<tobject> &args, tenv& env) {
        if (args.size() != 2) {
            return TeaResult("rm func synatax error, only support two items");
        }
        if (args[0]->type == TeaObject::T_MAP) {
            auto hash = args[0]->v_map;
            if (args[1]->type == TeaObject::T_PATTERN) {
                auto &key = args[1]->v_string;
                hash->erase(key);
                return tea_null;
            }
        }
        return TeaResult("set func synatax error!");
    }

private:
    // core implementation
    static bool is_builtin(const string& symbol) {
        if ( symbol == "if"     ||
             symbol == "def"    ||
             symbol == "begin"  ||
             symbol == "while"  ||
             symbol == "map"    ||
             symbol == "list"   ||
             symbol == "&&"     ||
             symbol == "||"     ||
             symbol == "fn" ) {
            return true;
        }
        return false;
    }

    static TeaResult eval_builtin(vector<tobject>& lst, tenv& env) {
        assert(lst.size() > 0);
        tobject head = lst[0];

        string symbol = lst[0]->v_string;

        if (symbol == "begin") {
            return eval_begin(lst, env);
        }
        if (symbol == "if") {
            return eval_if(lst, env);
        }
        if (symbol == "def") {
            return eval_def(lst, env);
        }
        if (symbol == "while") {
            return eval_while(lst, env);
        }
        if (symbol == "map") {
            return eval_map(lst, env);
        }
        if (symbol == "list") {
            return eval_list(lst, env);
        }
        if (symbol == "&&") {
            return eval_and(lst, env);
        }
        if (symbol == "||") {
            return eval_or(lst, env);
        }
        if (symbol == "fn") {
            return eval_fn(lst, env);
        }
        return TeaResult("Bug, code can't reach here!");
    }

    static TeaResult eval_fn(vector<tobject>& lst, tenv& env) {
        TeaLambda lambda;

        if ( lst.size() == 3) {
            return TeaResult("(fn) synatax error: args body");
        }
        if ( lst[1]->type != TeaObject::T_LIST ) {
            return TeaResult("(fn) synatax error: args should be a list");
        }
        for (size_t i = 0; i < lst[1]->v_list->size(); i++) {
            assert( (*lst[1]->v_list)[i]->type == TeaObject::T_SYMBOL);
            lambda.head.push_back( (*lst[1]->v_list)[i] );
        }
        lambda.body = lst[2];

        auto obj = TeaObject( make_shared<TeaLambda>(lambda) );
        auto tobj = make_shared<TeaObject>(obj);
        return TeaResult(tobj);
    }

    static TeaResult eval_and(vector<tobject>& lst, tenv& env) {
        if ( lst.size() < 3) {
            return TeaResult("(&&) synatax error: need 3 items at least");
        }

        bool result = true;
        for (size_t i = 1; i < lst.size(); ) {
            TeaResult r = eval(lst[i], env);
            if (r.is_error()) {
                return r;
            }
            if (r.result->type != TeaObject::T_BOOL) {
                return TeaResult("(&&) synatax error: item must be bool ");
            }
            result = result && r.result->v_bool;
            if (result == false) {
                break;
            }
        }
        if (result) {
            return TeaResult(tea_true);
        }
        return TeaResult(tea_false);
    }

    static TeaResult eval_or(vector<tobject>& lst, tenv& env) {
        if ( lst.size() < 3) {
            return TeaResult("(&&) synatax error: need 3 items at least");
        }

        bool result = false;
        for (size_t i = 1; i < lst.size(); ) {
            TeaResult r = eval(lst[i], env);
            if (r.is_error()) {
                return r;
            }
            if (r.result->type != TeaObject::T_BOOL) {
                return TeaResult("(&&) synatax error: item must be bool ");
            }
            result = result || r.result->v_bool;
            if (result == true) {
                break;
            }
        }
        if (result) {
            return TeaResult(tea_true);
        }
        return TeaResult(tea_false);
    }

    static TeaResult eval_list(vector<tobject>& lst, tenv& env) {
        vector<tobject> new_lst;

        for (size_t i = 1; i < lst.size(); ) {
            TeaResult r = eval(lst[i], env);
            if (r.is_error()) {
                return r;
            }
            new_lst.push_back(r.result);
        }

        auto obj = TeaObject( make_shared<vector<tobject>>(new_lst) );
        return TeaResult( make_shared<TeaObject>(obj) );
    }

    static TeaResult eval_map(vector<tobject>& lst, tenv& env) {
        if ( (lst.size() % 2) != 1) {
            return TeaResult("'(map)' synatax error, must include @pattern/value pair");
        }

        map<string, tobject> hash;

        for (size_t i = 1; i < lst.size(); ) {
            assert(lst[i]->type == TeaObject::T_PATTERN);

            TeaResult r = eval(lst[i+1], env);
            if (r.is_error()) {
                return r;
            }
            hash[lst[i]->v_string] = r.result;
            i += 2;
        }

        auto obj = TeaObject( make_shared<map<string, tobject>>(hash) );
        return TeaResult( make_shared<TeaObject>(obj) );
    }

    static TeaResult eval_while(vector<tobject>& lst, tenv& env) {
        if (lst.size() < 2) {
            return TeaResult("'(while)' synatax error, must include condition");
        }
        for(;;) {
            TeaResult r = eval(lst[2], env);
            if (r.is_error()) {
                return r;
            }

            auto cond = r.result;
            if ( cond->type != TeaObject::T_BOOL ) {
                return TeaResult("'(while)' synatax error, first item should reutrn bool!");
            }
            if (cond->v_bool == false) {
                break;
            }
            for (size_t i = 2; i < lst.size(); i++) {
                TeaResult r = eval(lst[2], env);
                if (r.is_error()) {
                    return r;
                }
            }
        }
        return TeaResult(tea_null);
    }

    static TeaResult eval_def(vector<tobject>& lst, tenv& env) {
        if (lst.size() == 3) {
            TeaResult r = eval(lst[2], env);
            if (r.is_error()) {
                return r;
            }
            assert(lst[1]->type == TeaObject::T_SYMBOL);
            env->set(lst[1]->v_string, r.result);
            return TeaResult(tea_null);
        }
        return TeaResult("'(def)' synatax error!");
    }

    static TeaResult eval_if(vector<tobject>& lst, tenv& env) {
        if (lst.size() >= 3) {
            TeaResult r = eval(lst[1], env);
            if (r.is_error()) {
                return r;
            }
            auto cond = r.result;
            if ( cond->type != TeaObject::T_BOOL ) {
                return TeaResult("'(if)' synatax error, first item should reutrn bool!");
            }
            if (cond->v_bool == true) {
                return eval(lst[2], env);
            }
            if (lst.size() == 3) {
                return TeaResult(tea_null);
            }
            if (lst.size() == 4) {
                return eval(lst[3], env);
            }
        }
        return TeaResult("'(if)' synatax error!");
    }

    static TeaResult eval_begin(vector<tobject>& lst, tenv& env) {
        if (lst.size() == 1) {
            return TeaResult(tea_null);
        }
        for ( size_t i = 1; i < lst.size() - 1; i++) {
            TeaResult r = eval(lst[i], env);
            if (r.is_error()) {
                return r;
            }
        }
        return eval(lst[lst.size()-1], env);
    }

    static TeaResult eval_lambda(tobject& head, vector<tobject>& args, tenv& env) {
        assert(head->type == TeaObject::T_LAMBDA);
        auto lambda = head->v_lambda;

        TeaEnvironment new_env(env, lambda->closure);
        for (size_t i = 0; i < lambda->head.size(); i++) {
            assert(lambda->head[i]->type == TeaObject::T_SYMBOL);
            if ( i >= args.size() ) {
                new_env.set(lambda->head[i]->v_string, tea_null);
            } else {
                new_env.set(lambda->head[i]->v_string, args[i]);
            }
        }

        auto tenv = make_shared<TeaEnvironment>(new_env);
        return eval(lambda->body, tenv);
    }

    static TeaResult eval(tobject& obj, tenv& env) {
        if ( obj->type == TeaObject::T_NULL      ||
             obj->type == TeaObject::T_BOOL      ||
             obj->type == TeaObject::T_INT       ||
             obj->type == TeaObject::T_FLOAT     ||
             obj->type == TeaObject::T_PATTERN   ||
             obj->type == TeaObject::T_MAP       ||
             obj->type == TeaObject::T_EXT       ||
             obj->type == TeaObject::T_LAMBDA) {
            return TeaResult(obj);
        }

        if ( obj->type == TeaObject::T_SYMBOL ) {
            return TeaResult( env->get(obj->v_string));
        }

        // execute list
        if ( obj->type == TeaObject::T_LIST ) {
            // empty list return null
            if (obj->v_list->size() == 0) {
                return TeaResult( tea_null );
            }

            // get head of list
            vector<tobject>& lst(*obj->v_list);
            tobject head = lst[0];
            assert(head->type == TeaObject::T_SYMBOL);

            if (is_builtin(head->v_string)) {
                // step.1 check builtin call or eval
                auto tresult = eval_builtin(lst, env);
                if (tresult.is_error()){
                    tresult.trace.push_back(obj->to_string());
                }
                return tresult;
            } else {
                // step.2 eval head of list
                auto tresult  = eval(head, env);
                if (tresult.is_error()) {
                    tresult.trace.push_back(obj->to_string());
                    return tresult;
                }
                head = tresult.result;
            }

            // step.3 eval all the args
            vector<tobject> args;
            for (size_t i = 1; i < lst.size(); i++) {
                TeaResult tresult = eval(lst[i], env);
                if (tresult.is_error()) {
                    tresult.trace.push_back(obj->to_string());
                    return tresult;
                }
                args.push_back(tresult.result);
            }

            // step.4 check PATTERN & FUNC & LAMBODA
            if ( head->type == TeaObject::T_PATTERN ) {
                if (args.size() == 1) {
                    if (args[0]->type == TeaObject::T_MAP) {
                        auto query = args[0]->v_map->find(head->v_string);
                        if (query == args[0]->v_map->end()) {
                            string msg = "Can't find " + head->v_string + " in map!";
                            auto tresult = TeaResult(msg);
                            tresult.trace.push_back(obj->to_string());
                            return tresult;
                        } else {
                            return TeaResult(query->second);
                        }
                    }
                }
                auto tresult = TeaResult("query map synatax error!");
                tresult.trace.push_back(obj->to_string());
                return tresult;
            }
            if ( head->type == TeaObject::T_FUNC ) {
                auto tresult = head->v_func(args, env);
                if ( tresult.is_error() ) {
                    tresult.trace.push_back(obj->to_string());
                }
                return tresult;
            }
            if ( head->type == TeaObject::T_LAMBDA ) {
                auto tresult =  eval_lambda(head, args, env);
                if ( tresult.is_error() ) {
                    tresult.trace.push_back(obj->to_string());
                }
                return tresult;
            }
        }

        auto tresult =  TeaResult("BUG!");
        if ( tresult.is_error() ) {
            tresult.trace.push_back(obj->to_string());
        }
        return tresult;
    }

    static TeaResult eval_all(vector<tobject>& codes, tenv& env) {
        int last = codes.size() - 1;
        if (last < 0) {
            return TeaResult(tea_null);
        }
        for (int i = 0; i < last; i++) {
            auto r = eval(codes[i], env);
            if (r.is_error()) {
                return r;
            }
        }
        return eval(codes[last], env);
    }

private:
    // parser, compiler
    static tobject parse_atom(const string& token) {
        // true or false
        if (token == "true") {
            return tea_true;
        }
        if (token == "false") {
            return tea_false;
        }

        // number
        if (isdigit(token.at(0)) || (token.at(0) == '-' && token.length() >= 2 && isdigit(token.at(1)))) {
            if (token.find('.') != string::npos || token.find('e') != string::npos) { // double
                float value = atof(token.c_str());
                return TeaObject::build(value);
            } else {
                int64_t value = atol(token.c_str());
                return TeaObject::build(value);
            }
        }

        // pattern
        if (token.at(0) == '@') {
            string pattern = token;
            pattern.erase(0, 1);
            if (pattern == "") {
                pattern = " ";
            }
            return TeaObject::build(pattern);
        }

        // symbol
        return make_shared<TeaObject>( TeaObject(TeaObject::T_SYMBOL, token) );
    }

    static tobject parse_tokens(const vector<string>& tokens, size_t pos, size_t &next_pos) {
        size_t i = pos;
        if ( tokens[i] != "(") {
            return nullptr;
        }

        auto ret = make_shared<vector<tobject>>();
        i = i + 1;

        for (;;) {
            if ( i >= tokens.size() ) {
                break;
            }

            // 0. check is end of a list
            const string& token(tokens[i]);
            if (token == ")") {
                next_pos = i + 1;
                return TeaObject::build(ret);
            }

            // 1. an new list begin
            if (token == "(") {
                size_t end_pos = 0;
                auto next = parse_tokens(tokens, i, end_pos);
                if (next == nullptr) {
                    return nullptr;
                }
                ret->push_back(next);
                i = end_pos;
            }

            // 2. an new symbol
            ret->push_back( parse_atom(tokens[i]) );
            i++;
        }

        return nullptr;
    }

    static string parse_and_compile(string& code, tenv& env, vector<tobject>& codes) {
        struct _ {
         	static void findAndReplaceAll(std::string & data, const std::string toSearch, const std::string replaceStr) {
                size_t pos = data.find(toSearch);
                while( pos != std::string::npos){
                    data.replace(pos, toSearch.size(), replaceStr);
                    pos =data.find(toSearch, pos + replaceStr.size());
                }
            }
            static void tokenize(std::string const &str, const char delim,
                        std::vector<std::string> &out) {

                size_t start;
                size_t end = 0;

                while ((start = str.find_first_not_of(delim, end)) != std::string::npos) {
                    end = str.find(delim, start);
                    out.push_back(str.substr(start, end - start));
                }
            }
        };

        _::findAndReplaceAll(code, "{", "(map");
        _::findAndReplaceAll(code, "}", ")");
        _::findAndReplaceAll(code, "[", "(list");
        _::findAndReplaceAll(code, "]", ")");
        _::findAndReplaceAll(code, "(", " ( ");
        _::findAndReplaceAll(code, ")", " ) ");
        _::findAndReplaceAll(code, "\n"," ");

        vector<string> tokens;
        _::tokenize(code, ' ', tokens);

        codes.clear();
        if (tokens.size() == 1) {
            codes.push_back( parse_atom( tokens[0] ));
        } else {
            size_t i = 0;
            while ( i < tokens.size() ) {
                size_t next = 0;
                auto ret = parse_tokens(tokens, i, next);
                if (ret == nullptr) {
                    return "parse token error!";
                }
                codes.push_back(ret);
                i = next;
            }
        }
        return "";
    }

public:
    static tenv new_env() {
        auto env = TeaEnvironment();
        init(env.data);
        return make_shared<TeaEnvironment>(env);
    }

    static string run(string& code, tenv& env) {
        vector<tobject> codes;
        auto result = parse_and_compile(code, env, codes);
        if (result != "") {
            return result;
        }

        auto ret = eval_all(codes, env);
        if (ret.is_error()) {
            return ret.trace[0];
        }
        return ret.result->to_string();
    }
};

}
