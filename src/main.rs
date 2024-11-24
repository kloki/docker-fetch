use std::{io, io::IsTerminal};

use container::Container;
use printer::pprint;
mod container;
mod printer;
fn read_from_stdin() -> io::Result<Option<String>> {
    let input = io::stdin();
    if !input.is_terminal() {
        let lines = input
            .lines()
            .collect::<io::Result<Vec<String>>>()?
            .join("\n");
        Ok(Some(lines))
    } else {
        Ok(None)
    }
}

fn main() {
    let input = if let Some(input) = read_from_stdin().unwrap() {
        input
    } else {
        println!("Usage: 'docker inspect <container_id> | docker-fetch'");
        return;
    };

    let container: Vec<Container> = serde_json::from_str(&input).unwrap();
    println!("{}", pprint(&container[0]));
}
