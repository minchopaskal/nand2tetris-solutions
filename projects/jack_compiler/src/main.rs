use std::{process::exit, path::Path};

mod compiler;
mod utils;

use compiler::parser::Parser;
use compiler::analyzer::{Analyzer, XMLAnalyzer, NoopAnalyzer};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_file|dir> [--output_xml]", args[0]);
        exit(1);
    }

    let output_xml = args.len() == 3 && args[2] == "--output_xml";

    let input_path = Path::new(&args[1]);
    let mut in_files = Vec::new();
    utils::get_files(&input_path, &mut in_files)?;

    // let out_filename = args[2].to_owned();
    // let out_file = fs::File::create(out_filename)?;
    // let mut out_writer = BufWriter::new(out_file);

    let analyzer: Box<dyn Analyzer<Output = ()>> = if output_xml {
        Box::new(XMLAnalyzer::new())
    } else {
        Box::new(NoopAnalyzer::new())
    };

    for path in in_files {
        let mut parser = Parser::new(path);
        let tree = parser.parse();

        analyzer.analyze(&tree);
    }

    Ok(())
}
