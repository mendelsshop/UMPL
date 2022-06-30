`anything in "" is part of the language`
<br>

`anything that is a link is part of the language`
<br>

`when the + is used it means zero or more` 
<br>

`when the * is used it means one or more`
<br>

`& means and`
<br>

`&| means and or`
<br>

`! means not`
<br>

# code:
[`<expression>+`](#expression)   `&|`  [`<definitions>+`](#definitions)
# expression:
`"(",` [`<stuff>`](#stuff) `, ")", "<" | ">"` 

# stuff:
[`<literal>`](#literal) `|` [`<calling>`](#calling)

# literal:
[`<number>`](#number) `|` [`<string>`](#string) `| "true" | "false" | "null"`

# number:
`"0x" &| 0-9A-F`

# string:
``"`", Any string of unicode characters, "`"  ``

# calling:
`"(",`[ `<internal>`](#internal) `|` [`<functions>`](#functions) `,")"`

# internal: 
[`<fn-keyword>`](#fn-keywords) `,` [`<function-params>`](#function-params)

# function-params: 
`"[",` [`<stuff>`](#stuff)+ `&|` [`<identifier>`](#identifier)`+ , ":"+,"]"`

# functions:
`"new"` [`<function>`](#function) `,` [`<function-params>`](#function-params)

# function: 
`any single unicode emoji`

# definitions:
[`<variable-definitions>`](#variable-definitions) `|` [`<function definitions>`]((#function-definitions)) `|` [`<list-definitions>`](#list-definitions) 

# variable-definitions:
`"create", ` [`<variable>`](#variable) `,"with",` [`<expression>`](#expression)

# variable:
[`<identifier>`](#identifier)

# identifier:
`!` [`<keyword>`](#keyword)
<br>
[`<ident-first>`](#ident-first) `,` [`<ident-other>+`](#ident-other)

# ident-first:
`!` [`<number>`](#number) `a-zA-Z`

# ident-other:
[`<number>`](#number) ` | a-zA-Z`

# function-definitions:
`"potato,"` [`<function>`](#function) `,` [`<function-args>`](#functions-args) `,"(",` [`<code>`](#code) `,")"`

# functions-args:
`"[",` [`<identifier>+`](#identifier) `,":"+,"]"`

# list-definitions:
`"list"` [`<variable>`](#variable) `"with"` [`<list-element>`](#list-elements)

# list-elements:
`"[",` [`<stuff>`](#stuff)`,":",`[`<stuff>`](#stuff) `,"]", "<" | ">"`

