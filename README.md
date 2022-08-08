[![cargo clippy](https://github.com/mendelsshop/UMPL/actions/workflows/cargo_clippy.yml/badge.svg)](https://github.com/mendelsshop/UMPL/actions/workflows/cargo_clippy.yml)
# UMPL
If you have any Ideas for the language put contribute them in [this file](https://github.com/mendelsshop/compiler/blob/main/Ideas.md)
<br>
along with an example or two according of how to use it in the language.
<br>
With UMPL we stive to break backwards compatibility, so have fun trying to write the same code for different versions of the language, if this project even last long enough.
<br>
To see example code for UMPL you can see [this directory](https://github.com/mendelsshop/UMPL/tree/main/umpl_examples), and to look at the formal language grammar refer to [this file](https://github.com/mendelsshop/UMPL/blob/main/grammer.md).
<br>
variables in UMPL must follow the kebab-case naming convention and have no uppercase letters.
<br>
Too see the understand the keywords and their meaning, refer to [this file](https://github.com/mendelsshop/UMPL/blob/main/keywords.md).
<br>
There is semi working vscode extension for UMPL, you can find it [here](https://github.com/mendelsshop/UMPL_VSCode_Extension)
<br>
At UMPL to fix the null problem, we have invented a new type called hempty, which is the same as null, but it sounds better, and it adds to the long list of null like types across all the proggraming languages.
<br> 
All bug reports head straight to /dev/hempty

# language-documentation
## Declarations
| name | description | usage | special keywords | special variables | example(s) |
|:---:|:---:|:---:|:---:|:---:|:---|
| create | creates a variable | create var-name with literal or expression| N/A | N/A| ```create num-var with 5``` <br> ```create str-var with ((input `>> `))>``` ```create var with str-var```|
| list | creates a list | list var-name with [literal or expression literal or expression]| N/A | N/A| ```list num-list with [1 3]``` <br> ```list str-list with [8, ((input `>> `))]```|
| potato | declares a function | potato emoji-name num-of-args ⧼code⧽| return literal-or-expression| for each argument you get `$argument-number` ie `$1` for the first one etc | ```potato 😀 2 ⧼return ((plus $1 $2))>⧽```|

## Control Flow
|name|description|usage|special keywords| example(s) |
|:---:|:---:|:---:|:---:|:---|
| if statement| if boolean is true do if code else do else code |if {boolean literal or expression} ⧼if code⧽ else ⧼else code⧽|N/A| ```if {true} ⧼(`true`)>⧽ else ⧼(`false`)>⧽``` <br>  ```if {not((true))>} ⧼(`true`)>⧽ else ⧼(`false`)>⧽``` <br>  ```if {boolean-var} ⧼if {true} ⧼(`true`)>⧽ else ⧼(`false`)>⧽ ⧽ else ⧼(`false`)>⧽```|
| loop statement | loop until the code reaches break of continue |loop ⧼code⧽ |break, continue| ```loop ⧼ if {true} ⧼(`true`)> break ⧽ else ⧼(`false`)> continue ⧽⧽```

## Keywords
### To call a keyword you first need create an expression so ()> or ()>> or ()< and in the expression you put another pair of parentheses and the keyeword and its arguments.
| name | description| paremeters | returns | example(s) |
| :-: | :-: | :-: | :-: | :- |
| plus | if the first argument is a number, returns the sum of all the arguments, if its a string, each argument after is conctenated to the string, anything else wont work | any*1: argument | any |
| minus| sets the first parameter to the original value each next argument is subtract to it unless there is one argument in which case it is negated returning the negative value of it | number*1: argument | number |
|multiply| if the first arguments is string, multiplies the string by the next argument, if its a number, sets the first parameter to the original value each next argument is multiplied to it, any other thing does not work | any*1: argument | any |
|divide| sets the first argument to the original value each next argument is divided by the previous divisor | number*1: argument | number |
|not| returns true if the value is false, false otherwise | [boolean: value] | boolean |
|or| comapares value1 and value2 and returns true if either is true | [boolean: value1, boolean: value2] | boolean |
|and| comapares value1 and value2 and returns true if both are true | [boolean: value1, boolean: value2] | boolean |
|eq| compare two values if they are equal | [boolean: value1, boolean: value2] | boolean |
|ne | compare two values if not equal | [boolean: value1, boolean: value2] |  boolean |
|gt| checks if the number1 is greater than the number2 | [number: number1, number: number2] | boolean |
|lt| check if the number1 is less than the number2 | [number: number1, number: number2] | boolean |
|le| checks if the number1 is less than or equal to the right number2 | [number: number1, number: number2] | boolean |
|ge| check if the number1 is greater than or equal to the number2 | [number: number1, number: number2] |   boolean |
|addwith| adds value to variable in place, if the variable is a string anything can added to it, but if its a number only number can, anything cannot be added to |  [variable: variable, any: value] | number |
|dividewith| divides value by variable in place | [variable: variable, number: value] | number |
|subtractwith| subtracts value from variable in place | [variable: variable, number: value] | number |
|multiplywith| multiplies value by variable in place, if variable is a string than the variable becomes the string value times, if the variable is a number we multiply the variable by the value, any other variable wont work | [variable: variable, number: value] | number |
|input| input with message | [string: message] | string |
|setwith| sets a variable to a value | [variable: variable , value*: any] | any |
|exit| exits with number provided | [number: number] | hempty |
|error| errors with error message provided |  [string: message] | hempty |
|strtonum| converts string to number | [string: string] |   number |
|strtobool| converts string to boolean | [string: string] | boolean |
|strtohempty| converts string to hempty | [string: string] |    hempty |
|runcommand| runs os command | [string: command] |  string |
|open| opens file | [string: file] |  file |
|close| closes file | [file: file] |    hempty |
|write| writes message to file with mode | [file: file, string: message, string: mode] |    hempty |
|writeline| writes message to file at line with mode | [file: file, string: message, number: line, string: mode] |  hempty |
|read| reads from file | [file: file] |   string |
|readline| reads the line specified from the file | [file: file, number: line] |  string |
|delete| deletes variable | [variable: variable] |  hempty |
|deletefile | deletes file | [file: file] |   hempty |
|createfile| creates new file | [file: file] | file |
|new| run custom function | function: name, arguments |  whatever the function returns | ```((new 😀 3 5))>``` |
|type | returns the type of the value | [any: value] | string | ```((type 1))>```|

`*`: 
sometimes you can set a variable to two values for making list in lists
<br>
`*1`: 
one or more arguments