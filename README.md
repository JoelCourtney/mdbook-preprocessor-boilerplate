# mdbook-preprocessor-boilerplate

Boilerplate code for [mdbook](https://rust-lang.github.io/mdBook/index.html) preprocessors.

Handles the CLI, checks whether the renderer is supported, checks the mdbook version, and runs
your preprocessor. All you need to do is implement the [mdbook_preprocessor::Preprocessor] trait.

## Example

The following is functionally identical to the [No-Op Preprocessor Example](https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs)
given by mdbook.

```rust
use mdbook_preprocessor::{book::Book, Preprocessor, PreprocessorContext};
use anyhow::{bail, Result};

fn main() -> Result<()> {
    mdbook_preprocessor_boilerplate::run(
        NoOpPreprocessor,
        "An mdbook preprocessor that does nothing" // CLI description
    )
}

struct NoOpPreprocessor;

impl Preprocessor for NoOpPreprocessor {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Ok(Some(true)) = ctx.config.get(&format!("{}.blow-up", self.name())) {
            anyhow::bail!("Boom!!1!");
        }

        // we *are* a no-op preprocessor after all
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer != "not-supported")
    }
}
```

License: GPL-3.0
