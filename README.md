# minicalc

A simple interpreter for calculation of arithmetic expressions. It will start
be developing from [these twitch streams](https://www.twitch.tv/computavel),
but the idea is adding more features over time.

#### Features

Programs are:
- `print E` where E is an expression.
- Expressions can be: `E ::= (E + E) | (E * E)`. Example: `print (4 + (3 * 2))`.
- Variables can be declared and used in expressions.
- Which variable, a sequence of characters starting by a letter, is an `<identifier>`.
- Operator precedence.

#### Notes

`INPUT -> LEXICAL ANALYSIS (front-end) -> SYNTAX ANALYSIS (back-end) -> OUTPUT`

The Lexical analysis will read the input and try to understand what the program does.
While the Syntax analysis uses this information to create the resulting program.
In this case, using Rust, it'll generate the machine code.

```
example:                    (treewalk)
4 + 3 * 2 =>             +
                       /   \
                      4     *
                           /  \
                          3    2
```

The first step will build a [abstract syntax
tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) from the input. It's
common for us to break this step into two: lexical analysis and syntactic
analysics.

`input -> lexical analysis -> syntactic analysis -> abstract syntax tree`

Input is text file, a set of characters. The Lexical analysis will generate
tokens of this text. Grouping the rules of each language. This process is
called `Tokenizer`.

```
example:
if (tolerancy < 0.5)
|  |  |- identifier
|  |
|  |- parenthesis
|
|- reserved word
```
