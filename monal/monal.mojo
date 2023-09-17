from python import Python
from python import Dictionary 
from python.object import PythonObject


fn main():
    print("Welcome to Monal!")
    try:
        run_repl()
    except e:
        print("error running monal. exiting shell:", e.value)


fn run_repl() raises:
    print_no_newline("mnl>>")
    let x = Python.evaluate("input()").to_string()
    print(x)
    
    run_repl()

fn keywords() raises -> Dictionary:
    let keywords = Python.dict()
    print(keywords.__str__())
    keywords["fn"] = TokenType.FN.value()
    return keywords

@value
struct TokenType:
    var _val: String

    alias ASSIGN = TokenType("=")
    alias EQUAL = TokenType("==")
    alias FN = TokenType("fn")

    fn value(self) -> String:
        return self._val


struct Token:
    var token_type: TokenType
    var line: Int
    var col: Int


struct Lexer:
    var current_index: Int
    var next_index: Int
    var soure: String

    fn __init__(inout self, source: String):
        self.soure = source
        self.current_index = 0
        self.next_index = 0

    fn tokens(inout self) -> String:
        return self.soure
