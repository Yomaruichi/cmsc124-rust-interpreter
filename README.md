# [TagaCode]

## Creators

- Brent Michael A. Mendoza (TheJuanderer)
- John Dave F. Valentin (Yomaruichi)

## Overview

TagaCode is a Filipino-themed programming language that is designed to make coding feel conversational. It blends standard programming language with Tagalog keywords and introduces 'chismis()', a unique visual runtime tracing tool to map variable dependencies and relationships during debugging.

## Host language and build

- Host language: Rust 1.98.0
- Version metadata: rust-toolchain.toml
- Build: `./build.sh`
- To build the executable, ensure Rust is installed and execute './build.sh' at the root directory using bash.

## Running it


| Command | What it does |
|---|---|
| `./run <file>` | [Executes a program. Available from Lab 4.] |
| `./run --tokenize <file>` | [Prints the token stream.] |
| `./run --parse <file>` | [Prints the parsed tree.] |
| `./run --eval <file>` | [Evaluates each expression and prints its value.] |
| `./run` | [Starts the REPL.] |

Exit codes:

- '0': Successful scan or execution
- '65': Lexical Analysis Error (e.g. unterminated string or unexpected characters)
- '70': Runttime Error

## File extension

`.po` - References to the Filipino courtesy when talking to an elder

## Lexical structure

### Keywords


| Keyword | Purpose |
|---|---|
| `itakda` | Declares a variable. |
| `tiyak` | Declares a constant value that cannot be changed. |
| `gawa` | Declares a function. |
| `kung` | Executes a block of code when a condition is true. |
| `kundi` | Executes a block of code when the preceding condition is false. |
| `habang` | Repeats a block of code while a condition is true. |
| `gawin` | Instructs the program to execute a block of code before evaluating conditions |
| `tuwing` | Repeat a block of code a predetermined number of times |
| `at` | Evaluates two or more condition and returns true when all conditions are met |
| `okaya` | Evaluates two or more condition and returns true when at least one condition is met |
| `tigil` | Stops the current loop. |
| `ituloy` | Skips the current iteration of a loop and proceeds to the next one. |
| `totoo` | Represents a true Boolean value. |
| `mali` | Represents a false Boolean value. |
| `wala` | A null value |
| `uri` | Defines a data type. |
| `isama` | Imports functionality from another module or file. |
| `ipakita` | Displays output to the console, used for printing. |
| `ipasok` | Reads input from the user. |
| `piliin` | Compares values against multiple options |
| `kapag` | An option |
| `edi` | Default option |
| `ibalik` | Returns a value from a function. |


### Operators


| Operator | Category | Operands | Associativity | Precedence |
|---|---|---|---|---|
| `=` | assignment | binary | right | 1 |
| `okaya` | logical | binary | left | 2 |
| `at` | logical | binary | left | 3 |
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
  | number | '42', '3.14' | An integer or floating-point number value. |
  | string | '"hello"' | A string value containing a sequence of characters, escapes like '\n' and '\"' are supported. |
  | boolean | totoo, mali | A Boolean value representing true or false. |
  | nil | wala | A nil value representing the absence of a value. |


### Identifiers

- Start characters: ASCII letters (`a`-`z`, `A`-`Z`), `_`
- Continue characters: ASCII alphanumerics (`a`-`z`, `A`-`Z`, `0`-`9`), `_`
- Case-sensitive: yes, 'kumusta' and 'Kumusta' are different
- 

### Comments

- Line comments: `//`
- Block comments: '/*...*/'
- Nesting: not supported
- Harness note: comment_prefix in tests/lab*/manifest.json is set to the
  token above

## Whitespace and termination

- Whitespace significant: no (used only to separate lexemes)
- Statement terminator: semicolon ';'
- Block delimiters: curly braces `{` and `}`
- Grouping delimiters: parentheses `(` and `)`

## Token output format

```
Token(type= IDENTIFIER, lexeme= 'chismis', literal= null, line= 1)
```

Tokens are output one per line

- 'type' - Outputs the token type from the list from the different lexical keywords and operators
- 'lexeme' - The actual source text of the token
- 'literal' - Value or 'null' for tokens like keywords, operators, and identifiers
- 'line' - The line number the lexeme is scanned, it is the line it began on

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
| 'chismis()' | 1 variable | string | Debugger feature, inspects and traces variable reference chain (e.g. a -> b) |


## Errors and diagnostics

Message format:

```
[line 5] Error: Unterminated string.
```


| Failure | Exit code |
|---|---|
| lexical error (unclosed string, unknown char) | 65 |
| syntax error | 65 |
| runtime error | 70 |


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

- The language is themed around the Filipino language to lower barriers for local learners and give the language a cultural identity, the inclusion of the function chismis() as a tracer fits the theme while offering a practical debugger tool.
-Strings are single-line. A multi-line string means one missing closing quote consumes every line after it, including valid code, and the real error is reported far from its cause. Ending the string at the newline confines the damage to one line, and the scanner resumes on the next. This also removes the heap of bugs that come with allowing these.
- Keywords are recognised after the whole word is scanned. `identifier()` consumes every letter, digit and underscore, then checks the table. That's why `kungoo` is one identifier and not `kung` + `oo`.
- Comments like `//` and `/* */` are borrowed from the C family because `/` already needs one character of lookahead for division. We made block comments nest, so commenting out code that already contains a block comment works.
- Errors don't stop the scan. Every error prints to stderr, sets `had_error`, and returns from the helper that found it, so the main loop carries on and one run reports every problem. `main` decides the exit code only after the whole file is scanned. The REPL ignores `had_error` so a bad line can't end the session.
- Token output format. `Token(type= X, lexeme= '...', literal= ..., line= N)`, one token per line. The lexeme is wrapped in single quotes so the empty EOF lexeme shows up as `''` and trailing whitespace can't hide.

## Known limitations

- Decimals need a digit on both sides of the dot. `.5` and `5.` are both rejected, and there is no standalone `.` token.
- Negative numbers aren't literals. `-5` scans as `MINUS` then `NUMBER`; the parser has to combine them.
- Only three escapes: `\n`, `\"`, `\\`. No `\t`, `\r`, or unicode escapes. An unknown escape is reported as an error, but the string is still emitted.
- Errors carry a line number only, no column.
- The REPL scans each line in isolation. Line numbers always read 1, and a block comment can't span REPL lines.
- Unterminated block comments report the wrong line. The error uses the line where the file ended, not where the `/*` opened, and everything after the `/*` is swallowed.


## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | Initial lexical grammar: single- and two-character operators, single-line strings with `\n` `\"` `\\` escapes, integer and decimal literals, identifiers, 22 Tagalog keywords, `//` and nested `/* */` comments, `--tokenize` flag and REPL. |
