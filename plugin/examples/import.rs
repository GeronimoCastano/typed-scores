//! Print the Typst source the importer produces for a file:
//! `cargo run --example import -- FILE [TUNE]`.

use typed_scores_plugin::importer::{convert, Request};

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let Some(path) = arguments.get(1) else {
        eprintln!("usage: import FILE [TUNE]");
        std::process::exit(2);
    };
    let bytes = std::fs::read(path).unwrap_or_else(|error| {
        eprintln!("error: {path}: {error}");
        std::process::exit(1);
    });
    let name = std::path::Path::new(path).file_name().and_then(|name| name.to_str()).unwrap_or(path);
    let request = Request {
        format: "auto",
        tune: arguments.get(2).map(String::as_str),
        package: "../../src/lib.typ",
        source_name: name,
        scale: "0.7",
    };
    match convert(&bytes, &request) {
        Ok((emitted, numbers)) => print!("{}", emitted.source(request.package, request.source_name, request.scale, &numbers)),
        Err(error) => {
            eprintln!("error: {path}: {error}");
            std::process::exit(1);
        }
    }
}
