use colorize::AnsiColor;
use figlet_rs::FIGfont;

use crate::Container;

fn banner(input: &str) -> String {
    let standard_font = FIGfont::from_content(include_str!("doom.flf")).unwrap();

    let figure = standard_font.convert(input);
    match figure {
        Some(s) => s.to_string(),
        None => input.to_string(),
    }
}

pub fn pprint(container: &Container) -> String {
    banner(&container.config.image).green()
}
