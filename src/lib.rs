//! Boilerplate code for [mdbook](https://rust-lang.github.io/mdBook/index.html) preprocessors.
//!
//! Handles the CLI, checks whether the renderer is supported, checks the mdbook version, and runs
//! your preprocessor. All you need to do is implement the [mdbook::preprocess::Preprocessor] trait.
//!
//! This boilerplate has a few heavy dependencies (like serde_json). If you want a small executable,
//! you'll have to implement this functionality yourself.
//!
//! # Example
//!
//! The following is functionally identical to the [No-Op Preprocessor Example](https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs)
//! given by mdbook.
//!
//! ```no_run
//! use mdbook::book::Book;
//! use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
//! use anyhow::{bail, Result};
//!
//! fn main() {
//!     mdbook_preprocessor_boilerplate::run(
//!         NoOpPreprocessor,
//!         "An mdbook preprocessor that does nothing" // CLI description
//!     );
//! }
//!
//! struct NoOpPreprocessor;
//!
//! impl Preprocessor for NoOpPreprocessor {
//!     fn name(&self) -> &str {
//!         "nop-preprocessor"
//!     }
//!
//!     fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
//!         // In testing we want to tell the preprocessor to blow up by setting a
//!         // particular config value
//!         if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
//!             if nop_cfg.contains_key("blow-up") {
//!                 anyhow::bail!("Boom!!1!");
//!             }
//!         }
//!
//!         // we *are* a no-op preprocessor after all
//!         Ok(book)
//!     }
//!
//!     fn supports_renderer(&self, renderer: &str) -> bool {
//!         renderer != "not-supported"
//!     }
//! }
//! ```

use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::{process, io};
use clap::{App, Arg, ArgMatches, SubCommand};
use anyhow::Result;
use semver::{Version, VersionReq};

/// Checks renderer support and runs the preprocessor.
pub fn run(preprocessor: impl Preprocessor, description: &str) {
    let matches = App::new(preprocessor.name())
        .about(description)
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor")
        ).get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: impl Preprocessor) -> Result<()> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    let out = serde_json::to_string(&processed_book)?;
    println!("{}", out);

    Ok(())
}

fn handle_supports(pre: impl Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
