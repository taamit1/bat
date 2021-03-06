/// A very simple colorized `cat` clone, using `bat` as a library.
/// See `src/bin/bat` for the full `bat` application.
use bat::{
    config::{Config, InputFile, StyleComponent, StyleComponents},
    Controller, HighlightingAssets,
};
use console::Term;
use std::process;

fn main() {
    let files = std::env::args_os().skip(1).collect::<Vec<_>>();

    if files.is_empty() {
        eprintln!("No input files specified");
        process::exit(1);
    }

    let config = Config {
        term_width: Term::stdout().size().1 as usize,
        colored_output: true,
        true_color: true,
        style_components: StyleComponents::new(&[
            StyleComponent::Header,
            StyleComponent::Grid,
            StyleComponent::Numbers,
        ]),
        files: files.iter().map(|file| InputFile::Ordinary(file)).collect(),
        ..Default::default()
    };
    let assets = HighlightingAssets::from_binary();

    Controller::new(&config, &assets).run().expect("no errors");
}
