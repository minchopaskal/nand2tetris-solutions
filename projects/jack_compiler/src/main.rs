use std::error::Error;
use std::{path::Path, process::exit};

mod compiler;
mod utils;

use compiler::analyzer::{Analyzer, XMLAnalyzer};
use compiler::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_file|dir> [--output_xml]", args[0]);
        exit(1);
    }

    let output_xml = args.len() == 3 && args[2] == "--output_xml";

    let input_path = Path::new(&args[1]);
    let mut in_files = Vec::new();
    utils::get_files(input_path, &mut in_files)?;

    for path in in_files {
        let mut parser = Parser::new(path);
        let parse_res = parser.parse();
        let tree = if let Ok(t) = parse_res {
            t
        } else {
            println!("Parse error: {}", parse_res.err().unwrap());
            continue;
        };

        if output_xml {
            let analyzer = XMLAnalyzer::new();
            analyzer.analyze(&tree);
        }
    }

    Ok(())
}
