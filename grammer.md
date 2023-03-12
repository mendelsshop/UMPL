# UMPL grammar

`anything in "" is part of the language`

`anything that is a link is part of the language`

`when the + is used it means zero or more`

`when the * is used it means one or more`

`when the ? is used it means one or two`

`& means and`

`&| means and or`

`! means not`

## opening

`"(" | "༺" | "༼" | "᚛" | "⁅" | "⁽" | "₍" | "⌈" | "⌊" | "〈" | "❨" | "❪" | "❬" | "❮" | "❰" | "❲" | "❴" | "⟅" | "⟦" | "⟨" | "⟪" | "⟬" | "⟮" | "⦃" | "⦅" | "⦇" | "⦉" | "⦋" | "⦍" | "⦏" | "⦑" | "⦓" | "⦕" | "⦗" | "⧘" | "⧚" | "⸢" | "⸤" | "⸦" | "⸨" | "\u2e55" | "\u2e57" | "\u2e59" | "\u2e5b" | "〈" | "《" | "「" | "『" | "【" | "〔" | "〖" | "〘" | "〚" | "﹙" | "﹛" | "﹝" | "（" | "［" | "｛" | "｟" | "｢"`

## closing

`")" | "༻" | "༽" | "᚜" | "⁆" | "⁾" | "₎" | "⌉" | "⌋" | "〉" | "❩" | "❫" | "❭" | "❯" | "❱" | "❳" | "❵" | "⟆" | "⟧" | "⟩" | "⟫" | "⟭" | "⟯" | "⦄" | "⦆" | "⦈" | "⦊" | "⦌" | "⦎" | "⦐" | "⦒" | "⦔" | "⦖" | "⦘" | "⧙" | "⧛" | "⸣" | "⸥" | "⸧" | "⸩" | "\u2e56" | "\u2e58" | "\u2e5a" | "\u2e5c" | "〉" | "》" | "」" | "』" | "】" | "〕" | "〗" | "〙" | "〛" | "﹚" | "﹜" | "﹞" | "）" | "］" | "｝" | "｠" | "｣"`

## code

[`<expression>+`](#expression)   `&|`  [`<definitions>+`](#definitions) `&|` [`<statements>+`](#statements)

## expression

[`<opening>`](#opening) [`<stuff>`](#stuff) `,` [`<colsing>`](#closing) `, "<" | ">"?`

## stuff

[`<literal>`](#literal) `|` [`<calling>`](#calling) `|` [`<identifier>`](#identifier)

## literal

[`<number>`](#number) `|` [`<string>`](#string) `|` [`<boolean>`](#boolean) | `"hempty"`

## number

`"0x" &| 0-9A-F`

## string

``"`", Any string of Unicode characters, "`"``

## boolean

`"true" | "false"`

## calling

`"(",`[<internal>](#internal) `|` [`<functions>`](#functions) `,")"`

## internal

[`<fn-keyword>`](#fn-keywords) `,` [`<function-params>`](#function-params)

## function-params

`"[",` [`<stuff>+`](#stuff) `,"]"`

## other-stuff

[`<literal>`](#literal) `|` [`<expression>`](#expression) `|` [`<identifier>`](#identifier)

## functions

`"new"` [`<function>`](#function) `,` [`<function-params>`](#function-params)

## function

`any single unicode emoji`

## definitions

[`<variable-definitions>`](#variable-definitions) `|` [`<function definitions>`](#function-definitions) `|` [`<list-definitions>`](#list-definitions)

## variable-definitions

`"create",` [`<variable>`](#variable) `,"with",` [`<other-stuff>`](#other-stuff)

## variable

[`<identifier>`](#identifier)

## identifier

`!` [`<keyword>`](#keyword)

[`<ident-first>`](#ident-first) `,` [`<ident-other>+`](#ident-other)

## ident-first

`!` [`<number>`](#number) `a-z/-`

## ident-other

[`<number>`](#number) `| a-z/-`

## function-definitions

`"potato,"` [`<function>`](#function) `,` [`<function-args>`](#functions-args) `,"⧼",` [`<code>`](#code) `&|` [`<return>`](#return) `,` [`"⧽"`

## functions-args

[`<number of arguments>`](#number)

## return

`"return" ,` [`<other-stuff>+`](#other-stuff) `| ":"`

## list-definitions

`"list"` [`<variable>`](#variable) `"with"` [`<list-element>`](#list-elements)

## list-elements

`"[",` [`<stuff>`](#other-stuff) `,` [`<stuff>`](#other-stuff) `,"]", "<" | ">"`

## statements

[`<loop>`](#loop) `|` [`if-else`](#if-else)

## loop

`"loop", "⧼"` [`<code>`](#code) `&| ("break" | "continue") ,"⧽"`

## if-else

`"if", "{",` [`<boolean:`](#boolean)[`(literal`](#literal)`|`[`expression)>`](#expression) `"}", ⧼",` [`<code>`](#code) `,"⧽","else", "⧼,"` [`<code>`](#code) `,"⧽"`
