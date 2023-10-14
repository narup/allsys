from python import Python
from python import Dictionary
from python.object import PythonObject
from testing import assert_true, assert_equal
from utils.vector import DynamicVector
from utils.static_tuple import StaticTuple
from memory import memset_zero


fn main():
    print("Welcome to Monal!")
    try:
        print("Running tests")
        run_tests()
        print("Finished running tests")
        run_repl()
    except e:
        print("error running monal. exiting shell:", e.value)


fn run_repl() raises:
    print_no_newline("mnl>>")
    let source = Python.evaluate("input()").to_string()

    let lexer = Lexer(source)
    let tokens = lexer.tokens()
    for i in range(0, tokens.size):
        let token = tokens[i]

    run_repl()


fn keywords() raises -> Dictionary:
    let keywords = Python.dict()
    print(keywords.__str__())
    keywords["fn"] = TokenType.FN.raw()
    return keywords


@value
@register_passable("trivial")
struct TokenType:
    var _val: StringLiteral

    alias ASSIGN = TokenType("=")
    alias EQUAL = TokenType("==")
    alias FN = TokenType("fn")
    alias IDENTIFIER = TokenType("identifier")

    fn raw(self) -> String:
        return self._val

    fn ptype(self) -> PythonObject:
        return Python.type(self._val)


@value
struct Token:
    var token_type: TokenType
    var value: String
    var line: Int
    var col: Int

struct Lexer:
    var current_index: Int
    var next_index: Int
    var source: String

    fn __init__(inout self, source: String):
        self.source = source
        self.current_index = 0
        self.next_index = 0

    fn tokens(self) -> DynamicVector[Token]:
        var tokens:[Token] = []
        print("Lexing tokens...")

        for index in range(len(self.source)):
            let token = Token(TokenType.IDENTIFIER, self.source[index], 0, 0)
            tokens.push_back(token)

        return tokens

    fn _source(self) -> String:
        return self.source


from Pointer import Pointer
from IO import print_no_newline

struct HeapArray:
    var data: Pointer[YourCustomStruct]  # Replace YourCustomStruct with your actual struct
    var size: Int
    var cap: Int

    fn __init__(inout self):
        self.cap = 16
        self.size = 0
        self.data = Pointer[YourCustomStruct].alloc(self.cap)

    fn __init__(inout self, size: Int, val: YourCustomStruct):  # Replace YourCustomStruct with your actual struct
        self.cap = size * 2
        self.size = size
        self.data = Pointer[YourCustomStruct].alloc(self.cap)  # Replace YourCustomStruct with your actual struct
        for i in range(self.size):
            self.data.store(i, val)

    fn add(inout self, val: YourCustomStruct):  # Replace YourCustomStruct with your actual struct
        if self.size == self.cap:
            self.cap *= 2
            new_data = Pointer[YourCustomStruct].alloc(self.cap)  # Replace YourCustomStruct with your actual struct
            for i in range(self.size):
                new_data.store(i, self.data.load(i))
            self.data.free()
            self.data = new_data
        self.data.store(self.size, val)
        self.size += 1
     
    fn __del__(owned self):
        self.data.free()

    fn dump(self):
        print_no_newline("[")
        for i in range(self.size):
            if i > 0:
                print_no_newline(", ")
            print_no_newline(self.data.load(i))
        print("]")
    

fn run_tests():
    try:
        test_lexer_exprs()
    except:
        print("test failed")


fn test_lexer_exprs() raises:
    let keys: PythonObject = ["a"]

    let cases = Python.dict()
    cases[keys[0].to_string()] = ["a"]

    let N: Int = keys.__len__().__index__()
    for index in range(N):
        let test_case = cases.get(keys[index])

        print("Test case at", index, ":", test_case)
        let expected_tokens = cases.get(keys[index])

        let lexer = Lexer(keys[index].to_string())
        var actual_tokens = lexer.tokens()

        let same_length = assert_equal(
            len(actual_tokens), expected_tokens.__len__().__index__()
        )
        _ = assert_true(same_length, "Length should be equal")

        for ci in range(expected_tokens.__len__().__index__()):
            let actual_token: Token = actual_tokens.pop_back()
            print("actual token:", actual_token._value_())
            print("expected token:", expected_tokens[ci].to_string())
            print(actual_token._value_().__eq__(expected_tokens[ci].to_string()))
            _ = assert_true(
                actual_token._value_().__eq__(expected_tokens[ci].to_string()),
                "token mismatch",
            )
