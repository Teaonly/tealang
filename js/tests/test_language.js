function test_op1()
{
    var r, a;
    r = 1 + 2;
    assert(r == 3, "1 + 2 === 3");

    r = 1 - 2;
    assert(r == -1, "1 - 2 === -1");

    r = -1;
    assert(r == -1, "-1 === -1");

    r = +2;
    assert(r == 2, "+2 === 2");

    r = 2 * 3;
    assert(r == 6, "2 * 3 === 6");

    r = 4 / 2;
    assert(r == 2, "4 / 2 === 2");

    r = 4 % 3;
    assert(r == 1, "4 % 3 === 3");

    r = 4 << 2;
    assert(r == 16, "4 << 2 === 16");

    r = 1 << 0;
    assert(r == 1, "1 << 0 === 1");

    r = 1 << 31;
    assert(r == 2147483648, "1 << 31 === 2147483648");

    r = 1 << 32;
    assert(r == 1, "1 << 32 === 1");

    r = (1 << 31) > 0;
    assert(r == true, "(1 << 31) > 0 === true");

    r = -4 >> 1;
    assert(r == -2, "-4 >> 1 === -2");

    // FIXME 
    //r = -4 >>> 1;
    //assert(r == 0x7ffffffe, "-4 >>> 1 === 0x7ffffffe");

    r = 1 & 1;
    assert(r == 1, "1 & 1 === 1");

    r = 0 | 1;
    assert(r == 1, "0 | 1 === 1");

    r = 1 ^ 1;
    assert(r == 0, "1 ^ 1 === 0");

    r = ~1;
    assert(r == -2, "~1 === -2");

    r = !1;
    assert(r == false, "!1 === false");

    assert((1 < 2) == true, "(1 < 2) === true");

    assert((2 > 1) == true, "(2 > 1) === true");

    assert(('b' > 'a') == true, "('b' > 'a') === true");

    println("-------- END TESTING -----------");
}

function test_cvt()
{    
    assert((NaN | 0) === 0);
    assert((Infinity | 0) === 0);
    assert(((-Infinity) | 0) === 0);

    assert(("12345" | 0) === 12345);    
    assert(("0x12345" | 0) === 0x12345);
    
    assert(("12345" >>> 0) === 12345);
    assert(("0x12345" >>> 0) === 0x12345);
    
    assert((NaN >>> 0) === 0);
    assert((Infinity >>> 0) === 0);
    assert(((-Infinity) >>> 0) === 0);

    assert(null == undefined);
    assert(undefined == null);

    assert("123" == 123);
    assert("122" != 123);

    // FIXME
    // assert(((4294967296 * 3 - 4) | 0) === -4);
    // assert(((4294967296 * 3 - 4) >>> 0) === (4294967296 - 4));

    println("-------- END TESTING -----------");
}

function test_inc_dec()
{
    var a, r;
            
    a = 1;
    r = a++;
    assert(r === 1 && a === 2, "++1");

    a = 1;
    r = ++a;
    assert(r === 2 && a === 2, "++2");

    a = 1;
    r = a--;
    assert(r === 1 && a === 0, "--1");

    a = 1;
    r = --a;
    assert(r === 0 && a === 0, "--2");

    a = {x:true};    
    a.x ++;
    assert(a.x == 2, "true++");

    a = {x:true};
    a.x--;
    assert(a.x == 0, "true--");

    a = [true];
    a[0]++;
    assert(a[0] == 2, "++4");

    a = {x:true};
    r = a.x++;
    assert(r === 1 && a.x === 2, "++5");

    a = {x:true};
    r = a.x--;
    assert(r === 1 && a.x === 0, "--4");

    a = [true];
    r = a[0]++;
    assert(r === 1 && a[0] === 2, "++6");

    a = [true];
    r = --a[0];
    a[0]--;
    assert(r === 0 && a[0] === -1, "--5");

    println("-------- END TESTING -----------");
}

function test_op2()
{
    function F(x)
    {
        this.x = x;
    }

    var a, b;
    a = new Object();
    a.x = 1;
    assert(a.x == 1, "new 1");
    b = new F(2);
    assert(b.x == 2, "new 2");

    a = {};
    b = "Hello";
    assert((a instanceof Object) == true, "instanceof 1");
    assert((b instanceof Object) == true, "instanceof 2");
    assert((a instanceof String) == false, "instanceof 3");
    assert((b instanceof String) == true, "instanceof 4");


    assert((typeof 1) == "number", "typeof 1");
    assert((typeof Object) == "function", "typeof 2");
    assert((typeof null) == "object", "typeof 3");
    assert((typeof unknown_var) == "undefined", "typeof 4");

    println("-------- END TESTING -----------");
}

function test_delete()
{
    var a, err;

    a = {x: 1, y: 1};
    assert(("x" in a) == true, "in operator 1");
    assert((delete a.x) == true, "delete 1");
    assert(("x" in a) == false, "in operator 2");
    
    err = false;
    try {
        delete null.a;
    } catch(e) {
        err = (e instanceof Exception);
    }
    assert(err == true, "delete");

    println("-------- END TESTING -----------");
}

function test_arguments()
{
    function f2() {
        assert(arguments.length == 2, "arguments 1");
        assert(arguments[0] == 1, "arguments 2");
        assert(arguments[1] == 3, "arguments 3");
    }
    f2(1, 3);

    assert(f2.prototype.constructor === f, "prototype");

    println("-------- END TESTING -----------");
}

function test_object_literal()
{
    var a = {
        'x':    1234,
        'y':    4321,
        'z':    5678,
    }
    assert(a.x == 1234, "object literal 1");
    assert(a.z == 5678, "object literal 2");

    var b = {
        'x':    1234,
        'y':    {
            'y':    4321
        },
        'z':    5678,
    }
    assert(b.x == 1234, "object literal 3");
    assert(b.z == 5678, "object literal 4");
    assert(b.y.y == 4321, "object literal 5");

    println("-------- END TESTING -----------");
}


function test_labels()
{
    do x: { break x; } while(0);
    
    if (1)
        x: { break x; }
    else
        x: { break x; }

    while (0) x: { 
        break x; 
    };
}


function test_argument_scope()
{
    var f;
    var c = "global";
    
    f = function(a = eval("var arguments")) {};
    assert_throws(SyntaxError, f);

    f = function(a = eval("1"), b = arguments[0]) { return b; };
    assert(f(12), 12);

    f = function(a, b = arguments[0]) { return b; };
    assert(f(12), 12);

    f = function(a, b = () => arguments) { return b; };
    assert(f(12)()[0], 12);

    f = function(a = eval("1"), b = () => arguments) { return b; };
    assert(f(12)()[0], 12);

    (function() {
        "use strict";
        f = function(a = this) { return a; };
        assert(f.call(123), 123);

        f = function f(a = f) { return a; };
        assert(f(), f);

        f = function f(a = eval("f")) { return a; };
        assert(f(), f);
    })();

    f = (a = eval("var c = 1"), probe = () => c) => {
        var c = 2;
        assert(c, 2);
        assert(probe(), 1);
    }
    f();

    f = (a = eval("var arguments = 1"), probe = () => arguments) => {
        var arguments = 2;
        assert(arguments, 2);
        assert(probe(), 1);
    }
    f();

    f = function f(a = eval("var c = 1"), b = c, probe = () => c) {
        assert(b, 1);
        assert(c, 1);
        assert(probe(), 1)
    }
    f();

    assert(c, "global");
    f = function f(a, b = c, probe = () => c) {
        eval("var c = 1");
        assert(c, 1);
        assert(b, "global");
        assert(probe(), "global")
    }
    f();
    assert(c, "global");

    f = function f(a = eval("var c = 1"), probe = (d = eval("c")) => d) {
        assert(probe(), 1)
    }
    f();
}

function test_function_expr_name()
{
    var f;

    /* non strict mode test : assignment to the function name silently
       fails */
    
    f = function myfunc() {
        myfunc = 1;
        return myfunc;
    };
    assert(f(), f);

    f = function myfunc() {
        myfunc = 1;
        (() => {
            myfunc = 1;
        })();
        return myfunc;
    };
    assert(f(), f);

    f = function myfunc() {
        eval("myfunc = 1");
        return myfunc;
    };
    assert(f(), f);
    
    /* strict mode test : assignment to the function name raises a
       TypeError exception */

    f = function myfunc() {
        "use strict";
        myfunc = 1;
    };
    assert_throws(TypeError, f);

    f = function myfunc() {
        "use strict";
        (() => {
            myfunc = 1;
        })();
    };
    assert_throws(TypeError, f);

    f = function myfunc() {
        "use strict";
        eval("myfunc = 1");
    };
    assert_throws(TypeError, f);
}

test_op1();
test_cvt();
test_inc_dec();
test_op2();
test_delete();
test_arguments();
test_object_literal();

test_labels();
test_function_length();
test_argument_scope();
test_function_expr_name();