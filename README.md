# UMPL

umpl is a joke programming language implentation/compiler written in rust that targets llvm (version 15)

It's like scheme but:

- you use `;`​ for quotation
- you use `!`​ for comments
- you enclose strings in `.​`
- cons is a tree: (`car`, `cdr`, `cgr`) (yes I know `car` and `cdr` have historical significance, but it makes sense to follow the pattern)
- a list/application can be enclosed in any open and close brackets from the Unicode BIDI_BRACKETS class, besides for `᚜` and `᚛` (which are used to denote scope) so you can do `(a c]<` or `[c d e)>>`
- The boolean type also has a maybe variant, which when evaluated will randomly be true or false. so boolean values are `|` (false), `&` (true), `?` (maybe)
- It has for loops (`go-through ... of ... ...`) while loops (`until ... then ...`)  loops (`continue-doing ...`)
- you can use `stop` and `skip` for breaking and continuing in a loop (`stop` is also used to return from a function early)
- It also has `if ... do ... otherwise​ ...` and `unless ... than ... else ...` (I plan on making slightly different semantics between `if` and `unless`)
- functions can only be named emojis if using the `fanction​` keyword (you can just create an anonymous function and assign it to a more descriptive name via a `let​`)
- you specify function parameters by optionally giving a number and then access them with index by any variation of single and double quotes, index starts at 0, here is an example 

```umpl
let cons  fanction  2 ᚜ !using let to get around emoji name requirements
    ! we have save the the parameter passed into cons because the inner function "overwrites" the first parameter temporaraly
    let x '0"
    let y '1'
    !implicitly returning inner function we could also write stop fanction 1 ...
    fanction  1 ᚜
        if '0'
            do ᚜x᚛
            otherwise ᚜y᚛
    ᚛
᚛
```

- it supports 2 types of function that take a variable number of parameters, 1 that has to have at least one extra parameter specified via `+`​ or the other type which accepts 0 or more additional parameters specified by `*`​, this is not really implemented yet because I thought I could piggyback off (llvm/c)'s va_arg instruction, but It's not always implemented, and it will only work with non-compound types (so I'll probably use the easier way of just passing in a linked list of parameters).

- so here are some forms of function signatures: `fanction 1* ...​`, `fanction 🚗 + ...` ,`fanction 3 ...`​

- a shortcut for `car`/`cdr`/`cgr` is to do expr​(`^`(`car`|`cdr`|`cgr`))* which just expands to the equivalent application ie `(expr)^car^cgr` becomes `(cgr (car expr))`
    currently each application must be followed by (`>>`|`>`|`<`) to determine if it should be printed or not `>>`​ means print without newline, `>`​ means to print with newline, `<`​ means to not print (I might change it to be a general rule for all expressions like  expr​(`^`(`car`|`cdr`|`cgr`))
- it also has labels specified like `@ident`​ which can be placed in a `link` stmt which tells the compiler that if it should jump to the first label if it finds any other labels given (global goto) example:
    ```link @x  @y
    if &  ! & means true
        do ᚜@y᚛  !goto @x
        otherwise ᚜(print .hi.)<᚛ !print hi
    @x
    (.done.)> !print done (using > notation)
    ```
- it has lazy evaluation, but since I currently cannot differentiate between primitives and normal functions at compile time, primitives applications must also thunk their arguments
- instead of nil​ it has `hempty​`
- number are reprsented in code as floating point hexadecimal and use  `%​` instead of .​ to seperate whole part from decimal part. example: `0xa1%1bc` or `0xa1` or `1%3` (not you must specify `0x` to use hexadecimal characters)
- any type of opening bracket does not need to be closed, becuase just like HTML the compiler (well really the parser) well auto close brackets for you
- and some hidden compiler speciallties (that are not implemented yet) (like parser/compiletime errors being stack overflows, randomly changing casing of stdlib function)
    
    and probably some other "features" i'm missing