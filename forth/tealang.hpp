#ifndef _TEA_LANG_HPP_
#define _TEA_LANG_HPP_

#include <vector>
#include <string>
#include <memory>
#include <map>
#include <utility>
#include <iterator>
#include <iostream>
#include <sstream>
#include <cstring>

#include "debug.hpp"

namespace tea {

using SPFLOAT = float;
using SPEXTEND = std::weak_ptr<int>;

struct CellStack;
struct Cell {
    enum CellType {
        T_Number = -3,
        T_String = -2,
        T_Extend = -1,
        T_Tuple = 0,
    };

    const int type_;
    union {
        SPFLOAT           _float;
        const SPFLOAT*    _tuple;
        const char*       _str;
    } v;
    SPEXTEND              ext_;

    Cell() : type_(T_Number) {
        v._float = 0.0;
    }
    Cell(SPFLOAT value): type_(T_Number) {
        v._float = value;
    }
    Cell(SPEXTEND ext): type_(T_Extend) {
        ext_ = ext;
    }

private:
    Cell(SPFLOAT* t, size_t len) : type_( len ) {
        v._tuple = t;
    }
    Cell(const char* str): type_(T_String) {
        v._str = str;
    }

    friend struct CellStack;
};

struct CellStack {
public:
    virtual ~CellStack() {
        for (size_t i = 0; i < all_string_.size(); i++) {
            free( (void *)all_string_[i] );
        }

        for (size_t i = 0; i < all_tuple_.size(); i++) {
            delete all_tuple_[i];
        }
    }
    Cell new_tuple( std::vector<SPFLOAT>& vec) {
        SPFLOAT* t = new SPFLOAT[ vec.size() ];

        for (size_t i = 0; i < vec.size(); i++) {
            t[i] = vec[i];
        }
        all_tuple_.push_back(t);

        Cell c(t, vec.size());
        return c;
    }
    Cell new_string( std::string& s) {
        const char* local_str = strdup( s.c_str() );
        all_string_.push_back(local_str);

        return Cell(local_str);
    }

    virtual void push_float(SPFLOAT v) = 0;
    virtual SPFLOAT back_float() = 0;
    virtual SPFLOAT pop_float() = 0;
    virtual const char* pop_string() = 0;
    virtual const SPFLOAT* pop_tuple();
    virtual void push_extend(SPEXTEND ext) = 0;
    virtual SPEXTEND pop_extend() = 0;
    virtual void push_cell(Cell& c);
    virtual Cell pop_cell();

    virtual void drop() = 0;
    virtual void dup() = 0;
    virtual void dup2() = 0;
    virtual void swap() = 0;
    virtual void rot() = 0;
    virtual size_t size() = 0;
    virtual void clear() = 0;

private:
    std::vector<const char*> all_string_;
    std::vector<const SPFLOAT*> all_tuple_;
};

struct TeaLang : public CellStack  {
    TeaLang() {
    }
    ~TeaLang() {
    }

    CellStack& stack() {
        return *this;
    }

public:
    // CellStatic interfaces
    virtual void push_float(SPFLOAT v) {
        stack_.push_back( Cell(v) );
    }
    virtual SPFLOAT back_float() {
        if (stack_.size() == 0) {
            tt_panic("Can't pop from empty stack!");
        }
        auto c = stack_.back();
        tt_assert(c.type_ == Cell::T_Number, "cell is not float!");
        return c.v._float;
    }
    virtual SPFLOAT pop_float() {
        if (stack_.size() == 0) {
            tt_panic("Can't pop from empty stack!");
        }
        auto c = stack_.back();
        stack_.pop_back();
        tt_assert(c.type_ == Cell::T_Number, "cell is not float!");
        return c.v._float;
    }
    virtual const char* pop_string() {
        if (stack_.size() == 0) {
            tt_panic("Can't pop from empty stack!");
        }
        auto c = stack_.back();
        stack_.pop_back();
        tt_assert(c.type_ == Cell::T_String, "cell is not string!");
        return c.v._str;
    }
    virtual const SPFLOAT* pop_tuple() {
        if (stack_.size() == 0) {
            tt_panic("Can't pop from empty stack!");
        }
        auto c = stack_.back();
        stack_.pop_back();
        tt_assert(c.type_ >= Cell::T_Tuple, "cell is not tuple!");
        return c.v._tuple;
    }
    virtual void push_extend(SPEXTEND ext) {
        Cell c(ext);
        stack_.push_back(c);
    }
    virtual SPEXTEND pop_extend() {
        if (stack_.size() == 0) {
            tt_panic("Can't pop from empty stack!");
        }
        auto c = stack_.back();
        stack_.pop_back();
        tt_assert(c.type_ == Cell::T_Extend, "cell is not extend!");
        return c.ext_;
    }
    virtual void push_cell(Cell& c) {
        stack_.push_back(c);
    }
    virtual Cell pop_cell() {
        auto c = stack_.back();
        stack_.pop_back();
        return c;
    }

    virtual void drop() {
        if (stack_.size() == 0) {
            tt_panic("Can't pop from empty stack!");
        }
        stack_.pop_back();
    }
    virtual void dup() {
        stack_.push_back( stack_.back() );
    }
    virtual void dup2() {
        if (stack_.size() < 2) {
            tt_panic("Can't dup2 stack !");
        }
        auto v1 = stack_.back();stack_.pop_back();
        auto v2 = stack_.back();stack_.pop_back();
        stack_.push_back( v2 );
        stack_.push_back( v1 );
        stack_.push_back( v2 );
        stack_.push_back( v1 );
    }
    virtual void swap() {
        if (stack_.size() < 2) {
            tt_panic("Can't swap stack !");
        }
        auto v1 = stack_.back();stack_.pop_back();
        auto v2 = stack_.back();stack_.pop_back();
        stack_.push_back( v1 );
        stack_.push_back( v2 );
    }
    virtual void rot() {
         if (stack_.size() < 3) {
            tt_panic("Can't rot stack !");
        }
        auto v1 = stack_.back();stack_.pop_back();
        auto v2 = stack_.back();stack_.pop_back();
        auto v3 = stack_.back();stack_.pop_back();
        stack_.push_back( v2 );
        stack_.push_back( v1 );
        stack_.push_back( v3 );
    }

    virtual size_t size() {
        return stack_.size();
    }
    virtual void clear() {
        stack_.clear();
    }

private:
    std::vector<Cell> stack_;
};


}   // end of namespace tea
#endif
