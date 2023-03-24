# UMPL Semantics

This document explains how different expressions are handled in different contexts.

## Scope

There two kinds of scopes the Global Scope which is the outermost scope anything defined/declared by itself ie not defined inside a function, loop, or if is defined in the outer scope.

The other scope is a local scope which is defined by using the `⧼`  `⧽` brackets which are used for functions, if, and loop.

### Accesing things from scopes

When you Define a variable or function it will be set in the current scope shadowing any thing defined with the same name in previous scopes.

When mutating a variable (you cannot change a function definition), UMPL will first loop in the current scope, if it cannot be found in the current scope it will recursivly do the same thing for the current scopes parent (if it has one).

If at any point in the process UMPL finds a variable of the same name it will change its value, otherwise it will return an error that the variable with that name cannot be found.

The same rules apply to getting a variable, if the variable is not found in the current scope it will recursivly look in the parent scope.

## Definition of functions

If a function is defined outside of in the outer part of a scope then even if its defined after the you call it your program will still work.

Otherwise is defined in a call ie:

```umpl
(plus potato 🍕 2 ⧼ (* $1 $2) ⧽)
```

Then the function can only be called after it has been defined.

So in the above example you would be able to do the following:

```umpl
(plus potato
    ! define the function 🍕
    🍕 2 ⧼ 
        (* $1 $2)
    ⧽
)
! call the function 🍕
(🍕 3 5)
```

Notice how the call to plus does not create a new scope while evaluating its arguments, so the function is defined in the same scope as the call to plus.

Defining a function returns has the value of the String `function added`

## If

The value of an if statement is the value of the last expression evaluated in the if statement.

### Loop

The value of a loop is the value of the expression given by using break keyword in the loop.

## Defining a variable

The rules for defining a variable are the different from defining a function. A variable can only be accessed after it has been defined.

Defining a variable has the value of the value of the String `variable added`

## Break Continue and Return

If at any point you want to exit a function and or repeat a loop you can use the `break` , `continue` , and `return` keywords respectively. Even if they are put as arguments to a call.

Example:

```umpl
potato 🍕 2 ⧼ 
    ! return 2 even if its in the arguments of a call
    (plus return 2)
⧽

! break out of the loop

loop ⧼ 
    ! break out of the loop
    (plus break 2)
    ! same applies to continue
⧽
```

## importing modules

The module system has the same semantics as defining a function. That is that if a module is declared in the outermost part of a scope, then it can be used even before it has been defined.

## how imports work

The same semantics that make function and module declarations that are in the outermost part of a scope are also the the only functions and modules that will be imported with a module declaration