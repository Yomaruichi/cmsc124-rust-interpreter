# [Language name] (To be discussed)

## Creators

- Brent Michael A. Mendoza (TheJuanderer)
- John Dave F. Valentin (Yomaruichi)

## Overview

[One paragraph: what the language is for, who would use it, what writing it
feels like.]

## Host language and build

- Host language: Rust 1.98.0
- Version metadata: rust-toolchain.toml
- Build: `./build.sh`
- [Anything a fresh clone needs to know.]

## Running it


| Command | What it does |
|---|---|
| `./run <file>` | [Executes a program. Available from Lab 4.] |
| `./run --tokenize <file>` | [Prints the token stream.] |
| `./run --parse <file>` | [Prints the parsed tree.] |
| `./run --eval <file>` | [Evaluates each expression and prints its value.] |
| `./run` | [Starts the REPL.] |


Exit codes: 0 [when], 65 [when], 70 [when].

## File extension

`.src`

## Lexical structure

### Keywords


| Keyword | Purpose |
|---|---|
| `let` | Declares a variable. |
| `const` | Declares a constant value that cannot be changed. |
| `fn` | Declares a function. |
| `if` | Executes a block of code when a condition is true. |
| `else` | Executes a block of code when the preceding condition is false. |
| `while` | Repeats a block of code while a condition is true. |
| `for` | Repeats a block of code for each item in a sequence or range. |
| `break` | Stops the current loop. |
| `continue` | Skips the current iteration of a loop and proceeds to the next one. |
| `true` | Represents a true Boolean value. |
| `false` | Represents a false Boolean value. |
| `null` | Represents the absence of a value. |
| `type` | Defines a data type. |
| `struct` | Defines a structure containing related data. |
| `import` | Imports functionality from another module or file. |
| `print` | Displays output to the console. |
| `input` | Reads input from the user. |
| `faint` | terminate/stop |
| `switch` | multiple choices |
| `return` | Returns a value from a function. |


### Operators


| Operator | Category | Operands | Associativity | Precedence |
|---|---|---|---|---|
| [op] | [arithmetic, comparison, logical, assignment, other] | [unary or binary] | [left, right, none] | [1 = loosest] |

| `=` | assignment | binary | right | 1 |
| `||` | logical | binary | left | 2 |
| `&&` | logical | binary | left | 3 |
| `==` | comparison | binary | left | 4 |
| `!=` | comparison | binary | left | 4 |
| `<` | comparison | binary | left | 5 |
| `>` | comparison | binary | left | 5 |
| `<=` | comparison | binary | left | 5 |
| `>=` | comparison | binary | left | 5 |
| `+` | arithmetic | binary | left | 6 |
| `-` | arithmetic | binary | left | 6 |
| `*` | arithmetic | binary | left | 7 |
| `/` | arithmetic | binary | left | 7 |
| `%` | arithmetic | binary | left | 7 |
| `!` | logical | unary | right | 8 |
| `-` | arithmetic | unary | right | 8 |


### Literals


| Kind | Syntax | Produces |
|---|---|---|
| [number] | [e.g. 42, 3.14] | [An integer or floating-point number value.] |
| [string] | [e.g. "hello", escapes "\" supported] | [A string value containing a sequence of characters.] |
| [boolean] | [true, false] | [A Boolean value representing true or false.] |
| [nil] | [void] | [A nil value representing the absence of a value.] |


### Identifiers

- Start characters: [which]
- Continue characters: [which]
- Case-sensitive: [yes or no]
- [Reserved patterns, length limits, or other restrictions.]

### Comments

- Line comments: [`//`]
- Block comments: not supported
- Nesting: not
- [Harness note: comment_prefix in tests/lab*/manifest.json is set to the
  token above.]

## Whitespace and termination

- Whitespace significant: [no, when declaring a variable, arithmetic operations]
- Statement terminator: [e.g. semicolon, newline, none]
- Block delimiters: [braces `{` and `}`]
- Grouping delimiters: [parentheses `(` and `)`]

## Token output format

```
[one line of real --tokenize output]
```

[What each field means. Frozen as of Lab 1; changes are recorded in the
changelog.]

## Grammar

```
[Your complete context-free grammar, current as of the latest activity.
Unambiguous, with precedence and associativity encoded in rule structure.]
```

## Parse output format

```
[one line of real --parse output, e.g. (+ 1.0 (* 2.0 3.0))]
```

- Groupings print as: [form]
- Numbers print as: [form]

## Semantics

### Values and types

[What runtime values exist, and how they are represented in the host
language.]

### Value printing

- Numbers: [e.g. 5 rather than 5.0]
- Nil: [spelling]
- Strings: [with or without quotes]

### Truthiness

[The complete rule. Which values are false in a condition; everything else is
true.]

### Operator semantics

- Arithmetic: [accepted operand types]
- `+` on strings: [concatenation, error, or coercion]
- Mixed types: [what happens]
- Comparison: [accepted operand types]
- Equality across types: [false, or an error]
- Division by zero: [value produced, or runtime error]

### Scope and bindings

- Redeclaration in the same scope: [allowed or an error]
- Uninitialized variable holds: [value]
- Shadowing: [behavior]
- Undefined name: [static error with exit 65, or runtime error with exit 70]

### Control flow and functions

- Logical operators return: [booleans, or the operand]
- Dangling else binds to: [which if]
- Closure capture of a loop variable: [per iteration, or shared]
- Function with no return statement produces: [value]
- Arity mismatch: [message and exit code]

## Native functions


| Name | Arguments | Returns | Notes |
|---|---|---|---|
| [name] | [count and types] | [type] | [caveats] |


## Errors and diagnostics

Message format:

```
[one real static error]
[one real runtime error]
```


| Failure | Exit code |
|---|---|
| [lexical error] | 65 |
| [syntax error] | 65 |
| [runtime error] | 70 |


## Testing conventions


| Folder | Activity | Mode | Flag |
|---|---|---|---|
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | `--eval` |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Functions | inline | none |


```
[specific tests]...
```

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
python3 run_tests.py tests/lab1
```

## Sample code

```
[a short program]
```

Output:

```
[its output]
```

## Design rationale

[Why the language is the way it is. Cover the choices that surprised you, the
features you cut, and the decisions you reversed. Specific reasons, not
approval of your own work.]

## Known limitations

- [What doesn't work, what is unimplemented, where behavior is worse than you
  would like.]

## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | [entry] |
