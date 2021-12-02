use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::{process, io};
use clap::{App, Arg, ArgMatches, SubCommand};
use anyhow::Result;
use semver::{Version, VersionReq};

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
