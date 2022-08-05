| name | description| paremeters | returns |
| :-: | :-: | :-: | :-: |
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

`*`: 
sometimes you can set a variable to two values for making list in lists
<br>
`*1`: 
one or more arguments