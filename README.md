# mdbook-preprocessor-boilerplate

Boilerplate code for [mdbook](https://rust-lang.github.io/mdBook/index.html) preprocessors.

Handles the CLI, checks whether the renderer is supported, checks the mdbook version, and runs
your preprocessor. All you need to do is implement the [mdbook::preprocess::Preprocessor] trait.

This boilerplate has a few heavy dependencies (like serde_json and mdbook). If you want a small executable,
you'll have to implement this functionality yourself.

## Example

The following is functionally identical to the [No-Op Preprocessor Example](https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs)
given by mdbook.

```rust
use mdbook::book::Book;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use anyhow::{bail, Result};

fn main() {
    mdbook_preprocessor_boilerplate::run(
        NoOpPreprocessor,
        "An mdbook preprocessor that does nothing" // CLI description
    );
}

struct NoOpPreprocessor;

impl Preprocessor for NoOpPreprocessor {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        // we *are* a no-op preprocessor after all
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
```

License: GPL-3.0
