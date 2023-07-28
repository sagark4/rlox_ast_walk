# rlox_ast_walk

Rust implementation of the tree-walk interpreter from the AMAZING book [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/contents.html).

If you have Rust installed, from the directory of this repo do the following for a `Lox` interpreter
```bash
cargo run
```

or to run a `Lox` script, say, `fib.lox`, do the following
```bash
cargo run fib.lox
```
That's it!  🍉

Shameless plug: I gave a talk at the Rust Vienna meetup on the visitor pattern, based on my experience doing this implementation.  You can find the slides [here](https://github.com/RustVienna/meetup-history/blob/master/2023-06/Sagar_Kale_Visitor_Pattern_2023_06_29.pdf).
