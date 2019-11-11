# mdbook-singlepage

This is a simple backend for [Rust's mdBook](https://github.com/rust-lang/mdBook) that spits out a single markdown page.

It is tailored to my very specific needs, most likely full of bugs, and makes ridiculous assumptions about the book's titling semantics and overall organization. But, hey, _it exists_.

See [Asynchronous Rust from the bottom up](https://github.com/teh-cmc/rust-async) for an example of a rendered book.

## Usage

Install `mdbook-singlepage` in your PATH:
```
cargo install mdbook-singlepage
```

Then add the following to your `book.toml` to enable the backend:
```toml
[output.singlepage]
```

That's it.  
You'll find a minimal working example [here](./example/).
