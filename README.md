# Risp

A [LISP](https://en.wikipedia.org/wiki/Lisp_(programming_language)) implemented as a Rust procedural macro!

## What?

I got inspired by the neat stuff people are doing with procedural macros in Rust, like [`yew`'s `html!` macro](https://github.com/yewstack/yew#jsx-like-templates-with-html-macro), and thought, could you write a programming language that's just a procedural macro?

As a proof of concept, I decided to implement an interperter for a basic LISP. The limitations of this LISP are:
- The entire program must be a single [S-Expression](https://en.wikipedia.org/wiki/S-expression)
- The only functions available are `+`, `-`, `*`, and `/`
- Each expression is either an integer or a function apply

## Use

You must have [rust installed](https://www.rust-lang.org/tools/install).

```sh
# clone the repo
git clone https://github.com/MainShayne233/risp

# enter directory
cd risp

# call the executable
./bin/risp '(* 2 (- 5 2))'
6
```

## Crates

Risp leans on the [`proc-macro-hack`](https://github.com/dtolnay/proc-macro-hack) crate to allow the `risp!` macro to be invoked in statement or expression position.

`proc-macro-hack crate` requires seperate crates for implementation, declaration, and use. You can read more about this [here](https://github.com/dtolnay/proc-macro-hack#defining-procedural-macros).

Due to this requirement, this project consists of the following crates:

- `risp`: Main crate that exports everything
- `risp_ast`: Defines the AST for the the parsed risp
- `risp_macro`: The implementation crate for the risp! macro
- `risp_test`: The crate for testing all of this code
