use std::error::Error;
use std::{path::Path, process::exit};

mod compiler;
mod utils;

use compiler::analyzer::Analyzer;
use compiler::analyzers::vm_generator::VMGenerator;
use compiler::analyzers::xml::XMLAnalyzer;
use compiler::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: {} <input_file|dir> [--xml <xml_output_folder>] [--vm <vm_output_folder>]",
            args[0]
        );
        exit(1);
    }

    let mut vm_out = None;
    let mut xml_out = None;
    for (i, arg) in args.iter().enumerate() {
        if arg == "--xml" {
            xml_out = Some(args[i + 1].clone());
        }

        if arg == "--vm" {
            vm_out = Some(args[i + 1].clone());
        }
    }

    let input_path = Path::new(&args[1]);
    let mut in_files = Vec::new();
    utils::get_files(input_path, &mut in_files)?;

    for path in in_files {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let mut parser = Parser::new(path);
        let parse_res = parser.parse();
        let tree = if let Ok(t) = parse_res {
            t
        } else {
            println!("Parse error: {}", parse_res.err().unwrap());
            continue;
        };

        if let Some(dir) = &xml_out {
            let analyzer = XMLAnalyzer::new(dir);
            analyzer.analyze(&tree);
        }

        if let Some(dir) = &vm_out {
            let analyzer = VMGenerator::new(dir, &filename);
            if let Err(e) = analyzer.analyze(&tree) {
                println!("VMGenerator error: {e}");
            }
        }
    }

    Ok(())
}
