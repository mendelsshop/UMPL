# Use new when calling a function ie: 
```printf(new funtion_name(args))``` for java.

# Number interpolation ie:
``` 
23{7}4       # 2,374
1{2.4}4      # 12.44
22{3}4.4{3}5 # 2234.435
``` 
#### Note you can use variables, function, and other things too.


# Indentation and {} both required ie:
```
if (expr) {
  do this;
  if (expr) {
    do this
    }
  }
```
# Array, string and other sybscripts start at one (thanks Lua).
```
hello: int = [2,5,5,5]
print(hello[1]) # displays 2
``` 
#### Note the syntax is not final, its just meant to show this idea.

# Some random case (aLtErNaTe cAsE, tOGGLE cASE,  StudlyCaps Case, etc) for standard functions (I get annoyed at Java for using lowerCamelCase and CamelCase depending on if its a class, object, method etc).

# Comparison operator (equals, less than) will have to be in two letter form (thanks bash) ie:
```
if (i eq 5) {
  code;
  }
```
eq for equals, ne for not equals.
lt for less than, gt for greater than.
le for less than or equals, ge for greater than or equals.
!(), for not ie :
```
if (!(tRuE) {
  code;
  }
```
#### note some things like if and not may change syntaticly.

# There will only be one error message, along with line number and filename/traces
```
Segmentation fault (core dumped)
  in Line: 7, Filename: main.umpl
  in Line: 5, Filename: module.umpl
```
# Operator precednce will no be a thing we will go left to right, unless you use parenthesis

# Indentation, using tabs or more than one space will result in a runtime error.
Indentation is bad for your health and the readability of your code.

# potato is the keyword for creating a function.

# function names can only be a single emoji, no spaces, no special characters.
