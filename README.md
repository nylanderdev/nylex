# nylex
## A macro-based lexer generator

Nylex offers two macros that can be used for generating lexing functions and token structs.
`lexer!` for lexing `&str` and `byte_lexer!` for `&[u8]`.
An example of the syntax follows:

```rust
lexer! {
  (r"[_a-zA-Z][a-zA-Z0-9]*" Identifier .) // Generate variant TokenKind::Identifier for pattern. Dot signifies lexeme should be saved
  (r"\d+" Integer .) // Match pattern as variant TokenKind::Integer. Dot signifies lexeme should be saved
  (r"var" KwVar) // Match the word "var" as TokenKind::KwVar. Do not save lexeme
  (r"\s+") // Skip over any whitespace. Will generate a token of TokenKind::Ignored
}
```

which will then generate the following code:

```rust
pub struct Token(TokenKind, Option<String>); // TokenKind and lexeme, if saved

pub enum TokenKind {
  Ignored,
  Identifier,
  Integer,
  KwVar
}

pub fn lex(src: &str) -> Result<Vec<Token>, usize> { // Either returns tokens or the index where a lexing error occured
  /* ... */
}
```

Note that the code generated will include public structs and `use`-statements, so for finer control of API it is best to use the macro within a module of its own, and then import things from that module, like so:

```rust
pub use _lex::{Token, TokenKind, lex};
mod _lex {
  nylex::lexer! {
    (r"\d" Digit .)
    (r"\s+")
  }
}
```

Nylex will match patterns in the order they are declared, so if you want to match longer patterns first, they will have to be declared prior to any shorter colliding patterns.
For example, if you want to match `==` as `Eq` and `=` as `Asgn` you will have to use the following order:

```rust
nylex::lexer! {
  (r"==" Eq)
  (r"=" Asgn)
}
```

rather than the erroneous:

```rust
nylex::lexer! {
  (r"=" Asgn)
  (r"==" Eq)
}
```

which will match all `==` as two separate `=`.

## Dependencies
Nylex uses and re-exports parts of `regex` and `lazy_static`.
