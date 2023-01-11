
# [![cargo clippy](https://github.com/mendelsshop/UMPL/actions/workflows/cargo_clippy.yml/badge.svg)](https://github.com/mendelsshop/UMPL/actions/workflows/cargo_clippy.yml) [![crates.io](https://img.shields.io/crates/v/umpl.svg?label=latest%20version)](https://crates.io/crates/umpl) [![Crates.io](https://img.shields.io/crates/d/umpl?label=crates.io%20downloads)](https://crates.io/crates/umpl) ![msrv](./resources/msrv.svg) [![](https://tokei.rs/b1/github/mendelsshop/UMPL?category=lines)](https://github.com/mendelsshop/UMPL) [![license](https://img.shields.io/github/license/mendelsshop/UMPL)](https://github.com/mendelsshop/UMPL/blob/main/LICENSE)

# UMPL

## About

UMPL is a highly verbose, both c and lisp-like language.

With UMPL we stive to break backwards compatibility, so have fun trying to write the same code for different versions of the language, if this project even last long enough.

At UMPL to fix the null problem, we have invented a new type called hempty, which is the same as null, but it sounds better, and it adds to the long list of null like types across all the programming languages.

All bug reports head straight to /dev/hempty

# Installation/Building

## Building

- Install rust and cargo
- Clone the repository
- run `cargo build --release`

## Installation

- cargo
  - Install rust and cargo
  - run `cargo install umpl`
- releases
  - Download the latest release for your platform from [here](https://github.com/mendelsshop/UMPL/releases)

## Case conventions

Variables in UMPL must follow the kebab-case naming convention and have no uppercase letters.

All internal keywords case depend on compiler options.

## Examples

To see example code for UMPL you can see [this directory](https://github.com/mendelsshop/UMPL/tree/main/umpl_examples), and to look at the formal language grammar refer to [this file](https://github.com/mendelsshop/UMPL/blob/main/grammer.md).

## IDE Support

There is semi working VSCode extension for UMPL, you can find it [here](https://github.com/mendelsshop/UMPL_VSCode_Extension)

## compiler options

umpl [`File`] [`Options`](#options)]

### Options

- `-r, i`: start the REPL
- `-h`: print the help message
- `-t=<number>`: set the toggle case for keywords
- `-f`: put forceful mode on, useful for when you write a program via the REPL
- `-e`: turns on evil mode

along with other dark secrets hidden in the code.

# language documentation

table of contents:

- [`Types`](#types)
  - [`number`](#number)
  - [`string`](#string)
    - [`string escape sequences`](#string-escape-sequences)
  - [`boolean`](#boolean)
  - [`hempty`](#hempty)
  - [`file`](#file)
- [`Declarations`](#declarations)
- [`Control Flow`](#control-flow)
- [`Keywords`](#keywords)

## Types

### number

Number is a hexadecimal floating point number, when shown to the user it will be in decimal.
valid examples: `0xa`, `0x14343.a1`, `10`, `10.1`

### string

String is a sequence of characters. string start and end with ``` ` ```.

#### string-escape-sequences

| Escape sequence | Description |
|:-:|:-:|
| `\n` | newline |
| `\t` | tab |
| `\r` | carriage return |
| `\b` | backspace |
| `\f` | form feed |
| `\a` | alert |
| `\v` | vertical tab |
| `\e` | escape |
| `\\` | backslash |
| ```\` ``` | single quote |
| ```\x{hex}``` | hexadecimal value in ascii representation |
| `\u{hex}` | Unicode character |

### boolean

Boolean is either true or false.

### hempty

Hempty is the same as null, but it sounds better.

### File

File is a path to a file.

## Declarations

| name | description | usage | special keywords | special variables | example(s) |
|:---:|:---:|:---:|:---:|:---:|:---|
| create | creates a variable | create var-name with literal or expression| N/A | N/A| ```create num-var with 5``` <br>  ```create str-var with ((input `>> `))>``` <br> ```create var with str-var```|
| list | creates a list | list var-name with [literal or expression literal or expression]| N/A | N/A| ```list num-list with [1 3]```<br> ```list str-list with [8, ((input `>> `))]```|
| potato | declares a function | potato emoji-name num-of-arguments ⧼code⧽| return literal-or-expression| for each argument you get `$argument-number` i.e. `$1` for the first one etc. | ```potato 😀 2 ⧼return ((plus $1 $2))>⧽```|

## Control-Flow

|name|description|usage|special keywords| example(s) |
|:---:|:---:|:---:|:---:|:---|
| if statement| if boolean is true do if code else do else code |if {boolean: literal or expression} ⧼if code⧽ else ⧼else code⧽|N/A| ```if {true} ⧼(`true`)>⧽ else ⧼(`false`)>⧽``` <br>  ```if {not((true))>} ⧼(`true`)>⧽ else ⧼(`false`)>⧽``` <br>  ```if {boolean-var} ⧼if {true} ⧼(`true`)>⧽ else ⧼(`false`)>⧽ ⧽ else ⧼(`false`)>⧽```|
| loop statement | loop until the code reaches break of continue |loop ⧼code⧽ |break, continue| ```loop ⧼ if {true} ⧼(`true`)> break ⧽ else ⧼(`false`)> continue ⧽⧽```

## Keywords

To call a keyword you first need create an expression so ()> or ()>> or ()< and in the expression you put another pair of parentheses and the keyword and its arguments

| name | description| parameters | returns | example(s) |
| :-: | :-: | :-: | :-: | :- |
| plus | if the first argument is a number, returns the sum of all the arguments, if its a string, each argument after is concatenated to the string, anything else wont work | any*1: argument | any | `((plus 5 6 7))>` <br> ```((plus `s` true 7))>``` |
| minus| sets the first parameter to the original value each next argument is subtract to it unless there is one argument in which case it is negated returning the negative value of it | number*1: argument | number |`((minus 5 6 7))>` <br> ```((minus 1))>``` |
|multiply| if the first arguments is string, multiplies the string by the next argument, if its a number, sets the first parameter to the original value each next argument is multiplied to it, any other thing does not work | any*1: argument | any |    `((multiply 5 6 7))>` <br> ```((multiply `s` 7))>``` |
|divide| sets the first argument to the original value each next argument is divided by the previous divisor | number*1: argument | number | `((divide 5 6 7 3))>` <br> ```((divide 1))>```
|not| returns true if the value is false, false otherwise | [boolean: value] | boolean | `((not true))>` <br> `((not boolean-var))>` |
|or| compares value1 and value2 and returns true if either is true | [boolean: value1, boolean: value2] | boolean | `((or true false))>` <br> `((or boolean-var boolean-var-1))>` |
|and| compares value1 and value2 and returns true if both are true | [boolean: value1, boolean: value2] | boolean |    `((and true false))>` <br> `((and boolean-var boolean-var-1))>` |
|eq| compare two values if they are equal | [any: value1, any: value2] | boolean |  `((eq true false))>` <br> ```((eq `t` string-var))>``` `((eq 5 6))>` |
|ne | compare two values if not equal | [any: value1, any: value2] |  boolean | `((ne true false))>` <br> ```((ne `t` string-var))>```  |
|gt| checks if the number1 is greater than the number2 | [number: number1, number: number2] | boolean | `((gt 5 6))>` |
|lt| check if the number1 is less than the number2 | [number: number1, number: number2] | boolean | `((lt 5 6))>` |
|le| checks if the number1 is less than or equal to the right number2 | [number: number1, number: number2] | boolean |  `((le 5 6))>` |
|ge| check if the number1 is greater than or equal to the number2 | [number: number1, number: number2] |   boolean |    `((ge 5 6))>` |
|addwith| adds value to variable in place, if the variable is a string anything can added to it, but if its a number only number can, anything cannot be added to |  [variable: variable, any: value] | any | `((addwith num-var 5))>` <br> ```((addwith str-var `s`))>``` <br >```((addwith str-var 5))>``` |
|subtractwith| subtracts value from variable in place | [variable: variable, number: value] | number |  `((subtractwith num-var 5))>` |
|dividewith| divides value by variable in place | [variable: variable, number: value] | number | `((dividewith num-var 5))>` |
|multiplywith| multiplies value by variable in place, if variable is a string than the variable becomes the string value times, if the variable is a number we multiply the variable by the value, any other variable wont work | [variable: variable, number: value] | any | `((multiplywith num-var 5))>` <br> ```((multiplywith str-var 5))>``` |
|input| input with message | [string: message] | string |   `((input "enter your name"))>` <br> `((input string-var))>` |
|setwith| sets a variable to a value | [variable: variable , value*: any] | any | `((setwith num-var 5))>` <br> ```((setwith str-var `s`))>``` |
|exit| exits with number provided | [number: number] | hempty | `((exit 5))>` |
|error| errors with error message provided |  [string: message] | hempty |  `((error "error"))>`  <br> `((error string-var))>` |
|strtonum| converts string to number | [string: string] |   number |    ```((strtonum `5`))>```  <br> ```((strtonum `0x5`))>``` |
|strtobool| converts string to boolean | [string: string] | boolean |   ```((strtobool `true`))>```  <br> ```((strtobool `false`))>``` |
|strtohempty| converts string to hempty | [string: string] |    hempty |   ```((strtohempty `empty`))>``` |
|runcommand| runs os command | [string: command] |  string |    `((runcommand "ls"))>` |
|open| opens file | [string: file] |  file |    `((open "file.txt"))>` |
|close| closes file | [file: file] |    hempty |    `((close file-var))>` |
|write| writes message to file with mode | [file: file, string: message, string: mode] |    hempty |    `((write file-var "message" "w"))>` <br>    `((write file-var "message" "a"))>` |
|writeline| writes message to file at line with mode | [file: file, string: message, number: line, string: mode] |  hempty |   `((writeline file-var "message" 1 "w"))>` <br>    `((writeline file-var "message" 1 "a"))>` |
|read| reads from file | [file: file] |   string |  `((read file-var))>` |
|readline| reads the line specified from the file | [file: file, number: line] |  string |  `((readline file-var 1))>` |
|delete| deletes variable | [variable: variable] |  hempty |    `((delete num-var))>` <br>    `((delete str-var))>` |
|deletefile | deletes file | [file: file] |   hempty |  `((deletefile file-var))>` |
|createfile| creates new file | [string: file] | file |  `((createfile "file.txt"))>` |
|new| run custom function | function: name, arguments |  whatever the function returns | ```((new 😀 3 5))>``` |
|type | returns the type of the value | [any: value] | string | ```((type 1))>```|

`*`:
sometimes you can set a variable to two values for making list in lists

`*1`:
one or more arguments

# Contributing

If you have any Ideas for the language put them in [this file](https://github.com/mendelsshop/compiler/blob/main/Ideas.md)
along with an example or two according of how to use it in the language.
