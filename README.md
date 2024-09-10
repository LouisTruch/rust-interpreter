# Rust-interpreter
This a simple interpreter written in Rust. It is based on the book ["Writing an Interpreter in Go"](https://interpreterbook.com/) by [Thorsten Ball](https://thorstenball.com/).
It interprets a simple and not-so-far-from-C language. You can see at the [bottom of this page](https://interpreterbook.com/) what does the lang looks like.

## Why ?
This repo/crate has not pretension to be a fully working interpreter, nor to be the best implementation of an interpreter. I started this project for sole purpose of **learning**.

### But learning what ?
I really wanted to learn some "real" parsing and how it is done in a production environment.
So I learnt, with this book, how to implement a top down recursive descent parser, called Pratt parsing (from the name of [the one](https://en.wikipedia.org/wiki/Vaughan_Pratt) describing it the first time in a [paper](https://tdop.github.io/) in 1973),
to parse generated token by the lexer into an abstract syntax tree.
I also wanted to learn more about Rust, and needed a good project to do so. As of now, I have been using the language for about six months, in a context of frontend development with the Dioxus framework. This helped me take a more raw approach
to the langage itself, by not using any external crate, which made me realise and like even more one of the best feature of this language, being the Enum. The book is written in Go, using all sorts of interface to parse and evaluate, where with Rust we can leverage the power of the Enum. And if I'm being honest, it also made me realise what I like a little less about the language, the ownership. It is great because I did not to have de-allocate anything, or track any sneaky leaking memory, but the sementic can be quite cumbersome, and this is a really small project, I can't imagine what it would be like in a bigger project.
*As you may have understood at this point, I'm still a beginner with Rust (and with programming in general), so that is probably some not very nice looking code.*

## Highlights
* Lexer which tokenizes the input
* Parser which generates an AST from the tokens
* Interpreter which evaluates the AST
* REPL (Read-Eval-Print-Loop) to interact with the interpreter, parser or lexer

## Usage
To run the REPL

```
cargo run
```
