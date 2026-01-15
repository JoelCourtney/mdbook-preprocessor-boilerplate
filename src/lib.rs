//! Boilerplate code for [mdbook](https://rust-lang.github.io/mdBook/index.html) preprocessors.
//!
//! Handles the CLI, checks whether the renderer is supported, checks the mdbook version, and runs
//! your preprocessor. All you need to do is implement the [mdbook_preprocessor::Preprocessor] trait.
//!
//! # Example
//!
//! The following is functionally identical to the [No-Op Preprocessor Example](https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs)
//! given by mdbook.
//!
//! ```no_run
//! use mdbook_preprocessor::{book::Book, Preprocessor, PreprocessorContext};
//! use anyhow::{bail, Result};
//!
//! fn main() -> Result<()> {
//!     mdbook_preprocessor_boilerplate::run(
//!         NoOpPreprocessor,
//!         "An mdbook preprocessor that does nothing" // CLI description
//!     )
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
//!         if let Ok(Some(true)) = ctx.config.get(&format!("{}.blow-up", self.name())) {
//!             anyhow::bail!("Boom!!1!");
//!         }
//!
//!         // we *are* a no-op preprocessor after all
//!         Ok(book)
//!     }
//!
//!     fn supports_renderer(&self, renderer: &str) -> Result<bool> {
//!         Ok(renderer != "not-supported")
//!     }
//! }
//! ```

use clap::{Arg, ArgMatches, Command};
use mdbook_preprocessor::{Preprocessor, errors::Result, parse_input};
use semver::{Version, VersionReq};
use std::{io, process};

/// Checks renderer support and runs the preprocessor.
pub fn run(preprocessor: impl Preprocessor, description: &'static str) -> Result<()> {
    let name = preprocessor.name().to_string();
    let args = Command::new(name)
        .about(description)
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    if let Some(supports) = args.subcommand_matches("supports") {
        handle_supports(preprocessor, supports);
    } else {
        handle_preprocessing(preprocessor)
    }
}

fn handle_preprocessing(pre: impl Preprocessor) -> Result<()> {
    let (ctx, book) = parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook_preprocessor::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    let out = serde_json::to_string(&processed_book)?;
    println!("{}", out);

    Ok(())
}

fn handle_supports(pre: impl Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if matches!(supported, Ok(true)) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
