---
title: 'sepBy'
kind: 'composite'
description: 'sepBy combinator parses zero or more occurrences of parser, separated by sep. Returns a list of values (without separator) returned by parser.'
---

## Signature

```ts
function sepBy<T, S>(parser: Parser<T>, sep: Parser<S>): Parser<Array<T>>
```

## Description

`sepBy` combinator parses *zero* or more occurrences of `parser`, separated by `sep`. Returns a list of values (without separator) returned by `parser`. This combinator never fails and returns an empty list if nothing matched.

---
title: 'many1'
kind: 'composite'
description: 'many1 combinator applies parser one or more times. Returns an array of the returned values of parser.'
---

## Signature

```ts
function many1<T>(parser: Parser<T>): Parser<Array<T>>
```

## Description

`many1` combinator applies `parser` *one* or more times. Returns an array of the returned values of `parser`.

---
title: 'takeRight'
kind: 'composite'
description: 'takeRight combinator takes exactly two parsers and applies them in order. Returns the result of the rightmost parser.'
---

## Signature

```ts
function takeRight<T1, T2>(p1: Parser<T1>, p2: Parser<T2>): Parser<T2>
```

## Description

`takeRight` combinator takes exactly **two** parsers and applies them in order. Returns the result of the rightmost `p2` parser.

---
title: 'map'
kind: 'primitive'
description: "map combinator applies a function to the parser's result and returns the result of that function."
---
## Signature

```ts
function map<T, R>(parser: Parser<T>, fn: (value: T) => R): Parser<R>
```

## Description

`map` combinator applies `fn` to the `parser`'s result and returns the result of that `fn`.

---
title: 'takeMid'
kind: 'composite'
description: 'takeMid combinator takes exactly three parsers and applies them in order. Returns the result of the parser in the middle.'
---

## Signature

```ts
function takeMid<T1, T2, T3>(
  p1: Parser<T1>,
  p2: Parser<T2>,
  p3: Parser<T3>
): Parser<T2>
```

## Description

`takeMid` combinator takes exactly **three** parsers and applies them in order. Returns the result of the `p2` parser in the middle.

---
title: 'many'
kind: 'primitive'
description: 'many combinator applies parser zero or more times. Returns an array of the returned values of parser.'
---
## Signature

```ts
function many<T>(parser: Parser<T>): SafeParser<Array<T>>
```

## Description

`many` combinator applies `parser` *zero* or more times. Returns an array of the returned values of `parser`. This combinator never fails and returns an empty list if nothing matched.

---
title: 'mapTo'
kind: 'composite'
description: "mapTo combinator maps the parser's result to a constant value."
---

## Signature

```ts
function mapTo<T, R>(parser: Parser<T>, value: R): Parser<R>
```

## Description

`mapTo` combinator maps the `parser`'s result to a constant `value`.

---
title: 'takeSides'
kind: 'composite'
description: 'takeSides combinator takes exactly three parsers and applies them in order. Returns a tuple of the results of the first and the last parsers.'
---

## Signature

```ts
function takeSides<T1, T2, T3>(
  p1: Parser<T1>,
  p2: Parser<T2>,
  p3: Parser<T3>
): Parser<[T1, T3]>
```

## Description

`takeSides` combinator takes exactly **three** parsers and applies them in order. Returns a tuple of the results of `p1` and `p3` parsers.

---
title: 'chainl'
kind: 'composite'
description: 'chainl combinator parses zero or more occurrences of parser, separated by op. Returns a value obtained by a recursive left-associative application of a function to the values returned by op and parser.'
---

## Signature

```ts
type Fn<L, R> = (left: L, right: R) => L

function chainl<T, L extends T, R>(
  parser: Parser<L>,
  op: Parser<R>,
  fn: Fn<T, R>
): Parser<T>
```

## Description

`chainl` combinator parses _zero_ or more occurrences of `parser`, separated by `op` (in [EBNF] notation: `parser (op parser)*`). Returns a value obtained by a recursive left-associative application of `fn` to the values returned by `op` and `parser`. This combinator is particularly useful for eliminating left recursion, which typically occurs in expression grammars.

---
title: 'when'
kind: 'primitive'
description: 'when combinator allows to create chained, context-aware parsers, that may depend on the output of the context parser.'
---
## Signature

```ts
function when<T, R extends Parser<unknown>>(
  context: Parser<T>,
  parser: (ctx: Context<T>) => R
): ToParser<R>
```

## Description

`when` combinator allows to create chained, context-aware parsers, that may depend on the output of the `context` parser. Returns a parser produced by the `parser` callback, which is called only if the `context` parser succeeds, i.e. if it fails, then `when` fails as well.

---
title: 'skipUntil'
kind: 'primitive'
description: "skipUntil combinator applies source parser, ignores its output, and stops after terminator parser succeeds. Returns a terminator's value. Fails if parser fails."
---
## Signature

```ts
function skipUntil<T, S>(parser: Parser<T>, terminator: Parser<S>): Parser<S>
```

## Description

`skipUntil` combinator applies source `parser`, ignores its output, and stops after `terminator` parser succeeds. Returns a `terminator`'s value. Fails if `parser` fails.

---
title: 'lookahead'
kind: 'primitive'
description: 'lookahead combinator applies parser without consuming any input. If parser fails and consumes some input, so does lookahead.'
---
## Signature

```ts
function lookahead<T>(parser: Parser<T>): Parser<T>
```

## Description

`lookahead` combinator applies `parser` without consuming any input. If `parser` fails and consumes some input, so does `lookahead`.

---
title: 'error'
kind: 'primitive'
description: 'error combinator allows to replace error message for parser.'
---
## Signature

```ts
function error<T>(parser: Parser<T>, expected: string): Parser<T>
```

## Description

`error` combinator allows to replace `parser`'s error message with `expected`.

---
title: 'optional'
kind: 'composite'
description: 'optional combinator tries to apply parser. Returns the result of parser or null, and only fails if parser fails.'
---

## Signature

```ts
function optional<T>(parser: Parser<T>): Parser<T | null>
```

## Description

`optional` combinator tries to apply `parser`. Returns the result of `parser` or `null`, and only fails if `parser` fails.

---
title: 'attempt'
kind: 'primitive'
description: "attempt combinator applies parser without consuming any input. It doesn't care if parser succeeds or fails, it won't consume any input."
---
## Signature

```ts
function attempt<T>(parser: Parser<T>): Parser<T>
```

## Description

`attempt` combinator applies `parser` without consuming any input. It doesn't care if `parser` succeeds or fails, it won't consume any input.

---
title: 'sepBy1'
kind: 'composite'
description: 'sepBy combinator parses zero or more occurrences of parser, separated by sep. Returns a list of values (without separator) returned by parser.'
---

## Signature

```ts
function sepBy1<T, S>(parser: Parser<T>, sep: Parser<S>): Parser<Array<T>>
```

## Description

`sepBy1` combinator parses *one* or more occurrences of `parser`, separated by `sep`. Returns a list of values (without separator) returned by `parser`. Otherwise returns an error produced by `parser`.

---
title: 'takeUntil'
kind: 'primitive'
description: 'takeUntil combinator applies source parser, collects its output, and stops after terminator parser succeeds. Returns a tuple of values collected by parser and terminator. Fails if parser fails.'
---
## Signature

```ts
function takeUntil<T, S>(parser: Parser<T>, terminator: Parser<S>): Parser<[Array<T>, S]>
```

## Description

`takeUntil` combinator applies source `parser`, collects its output, and stops after `terminator` parser succeeds. Returns a tuple of values collected by `parser` and `terminator`. Fails if `parser` fails.

---
title: 'sequence'
kind: 'primitive'
description: 'sequence combinator applies parsers in order, until all of them succeed. Returns a tuple of values returned by parsers.'
---
## Signature

```ts
function sequence<T extends Array<Parser<unknown>>>(...ps: T): Parser<ToTuple<T>>
function sequence<T>(...ps: Array<Parser<T>>): Parser<Array<T>>
```

## Description

`sequence` combinator applies `ps` parsers in order, until *all* of them succeed. Returns [a tuple][typescript-tuple] of values returned by `ps` parsers.

---
title: 'choice'
kind: 'primitive'
description: 'choice combinator tries to apply parsers in order, until one of them succeeds. Returns a value of the succeeding parser.'
---
## Signature

```ts
function choice<T extends Array<Parser<unknown>>>(...ps: T): Parser<ToUnion<T>>
function choice<T>(...ps: Array<Parser<T>>): Parser<T>
```

## Description

`choice` combinator tries to apply `ps` parsers in order, until one of them succeeds. Returns a value of the succeeding parser.

---
title: 'takeLeft'
kind: 'composite'
description: 'takeLeft combinator takes exactly two parsers and applies them in order. Returns the result of the leftmost parser.'
---

## Signature

```ts
function takeLeft<T1, T2>(p1: Parser<T1>, p2: Parser<T2>): Parser<T1>
```

## Description

`takeLeft` combinator takes exactly **two** parsers and applies them in order. Returns the result of the leftmost `p1` parser.

---
title: 'defer'
kind: 'primitive'
description: 'defer is a special parser that is tailored for creating mutually recursive parsers.'
---
## Signature

```ts
interface Deferred<T> extends Parser<T> {
  with(parser: Parser<T>): void
}

function defer<T>(): Deferred<T>
```

## Description

`defer` is a special parser that has an additional `with` method, which should be used to define the parser. This parser is tailored for creating mutually recursive parsers.

---
title: 'eof'
kind: 'primitive'
description: 'eof only succeeds at the end of the input.'
---
## Signature

```ts
function eof(): Parser<null>
```

## Description

`eof` only succeeds (with `null`) at the end of the input.

---
title: 'letter'
kind: 'composite'
description: 'letter parses a single alphabetical character. Returns the matched character. Unicode friendly.'
---

## Signature

```ts
function letter(): Parser<string>
```

## Description

`letter` parses a single alphabetical character. Returns the matched character. Unicode friendly.

---
title: 'hex'
kind: 'composite'
description: "hexadecimal parses a hexadecimal number prefixed with '0x' or '0X', e.g. '0xFF', '0XFF', '0xff'. Returns a decimal number obtained using parseInt with radix of 16."
---

## Signature

```ts
function hex(): Parser<number>
```

## Description

`hex` parses a hexadecimal number prefixed with `0x` or `0X`, e.g. `0xFF`, `0XFF`, `0xff`. Returns **a decimal number** obtained using [parseInt] with radix of 16.

---
title: 'integer'
kind: 'composite'
description: "integer parses an integer number with an optional minus sign, e.g. '0', '-7', '420'. Returns a decimal number obtained using parseInt with radix of 10."
---

## Signature

```ts
function integer(): Parser<number>
```

## Description

`integer` parses an integer number with an optional minus sign, e.g. `0`, `-7`, `420`. Returns **a decimal number** obtained using [parseInt] with radix of 10.

---
title: 'regexp'
kind: 'primitive'
description: 'regexp parses a string that matches a provided regular expression. Returns the matched string, or fails with a provided message.'
---
## Signature

```ts
function regexp(rs: RegExp, expected: string): Parser<string>
```

## Description

`regexp` parses a string that matches a provided `re` regular expression. Returns the matched string, or fails with an `expected` message.

## Implementation notes

::: warning
If `g` flag is missing, it will be automatically injected. It's still better to always provide it to avoid small performance penalty and clearly document the intention.
:::

The regular expression must obey two simple rules:

- It *does* use g flag. Flags like u and i are allowed and can be added if needed.
- It *doesn't* use `^` and `$` to match at the beginning or at the end of the text.

---
title: 'whitespace'
kind: 'composite'
description: 'whitespace parses whitespace, either a single character or consecutive ones. Returns the matched character(s).'
---

## Signature

```ts
function whitespace(): Parser<string>
```

## Description

`whitespace` parses whitespace, either a single character or consecutive ones. Returns the matched character(s).

---
title: 'eol'
kind: 'composite'
description: 'eol only succeeds at the end of the line with a matched line break character.'
---

## Signature

```ts
function eol(): Parser<string>
```

## Description

`eol` only succeeds at the end of the line with a matched line break character.

---
title: 'string'
kind: 'primitive'
description: 'string parses an ASCII string. Returns the parsed string.'
---
## Signature

```ts
function string(match: string): Parser<string>
```

## Description

> For parsing Unicode strings, consider using [ustring].

`string` parses an *ASCII* string. Returns the parsed string.

---
title: 'octal'
kind: 'composite'
description: "octal parses an octal number prefixed with '0o' or '0O', e.g. '0o42', '0O42'. Returns a decimal number obtained using parseInt with radix of 8."
---

## Signature

```ts
function octal(): Parser<number>
```

## Description

`octal` parses an octal number prefixed with `0o` or `0O`, e.g. `0o42`, `0O42`. Returns **a decimal number** obtained using [parseInt] with radix of 8.

---
title: 'whole'
kind: 'composite'
description: "whole parses a positive whole number without leading zeros, e.g. '0', '7', '420'. Returns a decimal number obtained using parseInt with radix of 10."
---

## Signature

```ts
function whole(): Parser<number>
```

## Description

`whole` parses a positive whole number without leading zeros, e.g. `0`, `7`, `420`. Returns **a decimal number** obtained using [parseInt] with radix of 10.

---
title: 'any'
kind: 'primitive'
description: 'any parses any single character from the input and returns it; it fails at the end of input.'
---
## Signature

```ts
function any(): Parser<string>
```

## Description

`any` parses any single character from the input and returns it. It fails at the end of input.

---
title: 'letters'
kind: 'composite'
description: 'letters parses a sequence of alphabetical characters. Returns matched characters as a string. Unicode friendly.'
---

## Signature

```ts
function letters(): Parser<string>
```

## Description

`letters` parses a sequence of alphabetical characters. Returns matched characters as a string. Unicode friendly.

---
title: 'tryRun'
kind: 'primitive'
description: 'tryRun is used to run parser with provided input, throwing an error on failure.'
---
## Signature

```ts
interface Runnable<T> {
  with(input: string): Success<T>
}

function tryRun<T>(parser: Parser<T>): Runnable<T>
```

## Description

`tryRun` is is used to run `parser` with provided input, **throwing `ParserError` on failure**.

---
title: 'run'
kind: 'primitive'
description: 'run is used to run parser with provided input.'
---
## Signature

```ts
interface Runnable<T> {
  with(input: string): Result<T>
}

function run<T>(parser: Parser<T>): Runnable<T>
```

## Description

`run` is used to run `parser` with provided input.

---
title: 'noneOf'
kind: 'primitive'
description: 'noneOf ensures that none of the characters in the given string matches the current character.'
---
## Signature

```ts
function noneOf(): Parser<string>
```

## Description

`noneOf` ensures that none of the characters in the given string matches the current character.

---
title: 'binary'
kind: 'composite'
description: "binary parses a binary number prefixed with '0b' or '0B', e.g. '0b10', '0B10'. Returns a decimal number obtained using parseInt with radix of 2."
---

## Signature

```ts
function binary(): Parser<number>
```

## Description

`binary` parses a binary number prefixed with `0b` or `0B`, e.g. `0b10`, `0B10`. Returns **a decimal number** obtained using [parseInt] with radix of 2.

---
title: 'ustring'
kind: 'primitive'
description: 'ustring parses a Unicode string. Returns the parsed string.'
---
## Signature

```ts
function ustring(match: string): Parser<string>
```

## Description

> For parsing ASCII-only strings, consider using [string].

`ustring` parses a Unicode string. Returns the parsed string.

## Implementation notes

This parser is very similar to the [string] parser, except it takes a bit hacky (though performant) approach, that is based on counting length of the given `match` string *in bytes*. It then subslices and compares string slice with that `match` string.

It was tested on code points from the [Basic Multilingual Plane][bmp], but various tests showed that other planes are consumable as well, but that is not guaranteed. If you need guaranteed parsing of code points outside of the BMP, consider using [regexp] with `u` flag.

---
title: 'oneOf'
kind: 'primitive'
description: 'oneOf ensures that one of the characters in the given string matches the current character.'
---
## Signature

```ts
function oneOf(): Parser<string>
```

## Description

`oneOf` ensures that one of the characters in the given string matches the current character.

---
title: 'float'
kind: 'composite'
description: "float parses a float number with an optional minus sign, e.g. '0.25', '-7.90', '4.20'. Returns a decimal number obtained using parseInt with radix of 8."
---

## Signature

```ts
function float(): Parser<number>
```

## Description

> Note: It doesn't handle floats with exponent parts.

`float` parses a float number with an optional minus sign, e.g. `0.25`, `-7.90`, `4.20`. Returns **a decimal number** obtained using [parseFloat].

---
title: 'rest'
kind: 'primitive'
description: 'rest simply returns the unparsed input as a string. Never fails.'
---
## Signature

```ts
function rest(): Parser<string>
```

## Description

`rest` simply returns the unparsed input as a string. Never fails.

