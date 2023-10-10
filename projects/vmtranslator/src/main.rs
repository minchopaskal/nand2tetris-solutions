use core::panic;
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::path::Path;
use std::process::exit;

#[derive(Debug, Copy, Clone)]
enum Segment {
    Unknown,

    Local,
    Argument,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

enum Op {
    Unknown,

    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Pop(Segment, i32),
    Push(Segment, i32),
}

fn get_segment(segment: &str) -> Segment {
    match segment {
        "local" => Segment::Local,
        "argument" => Segment::Argument,
        "static" => Segment::Static,
        "constant" => Segment::Constant,
        "this" => Segment::This,
        "that" => Segment::That,
        "pointer" => Segment::Pointer,
        "temp" => Segment::Temp,
        _ => Segment::Unknown,
    }
}

fn parse(src: BufReader<fs::File>) -> Result<Vec<Op>, Box<dyn Error>> {
    let mut res = Vec::new();

    for line in src.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let tokens = line.split(" ").collect::<Vec<&str>>();
        assert!(tokens.len() < 4, "{}", line);

        let op = match tokens[0] {
            "add" => Op::Add,
            "sub" => Op::Sub,
            "neg" => Op::Neg,
            "eq" => Op::Eq,
            "gt" => Op::Gt,
            "lt" => Op::Lt,
            "and" => Op::And,
            "or" => Op::Or,
            "not" => Op::Not,
            "pop" => {
                let seg = get_segment(tokens[1]);
                let idx = tokens[2].parse::<i32>()?;
                Op::Pop(seg, idx)
            },
            "push" => {
                let seg = get_segment(tokens[1]);
                let idx = tokens[2].parse::<i32>()?;
                Op::Push(seg, idx)
            }
            _ => Op::Unknown,
        };

        res.push(op);
    }

    Ok(res)
}

fn pop_segment(idx: i32, buf: &mut BufWriter<fs::File>, base_var: &str) -> std::io::Result<()> {
    // --sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M-1")?;

    // Ram[13] = base_var + idx
    writeln!(buf, "@{}", idx)?;
    writeln!(buf, "D=A")?;
    writeln!(buf, "@{}", base_var)?;
    writeln!(buf, "A=D+M")?;
    writeln!(buf, "D=A")?;
    writeln!(buf, "@R13")?;
    writeln!(buf, "M=D")?;

    // d = *sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M")?;
    writeln!(buf, "D=M")?;

    // A = ram[13] = base_var+idx
    writeln!(buf, "@R13")?;
    writeln!(buf, "A=M")?;

    // Ram[base_var + idx] = d = *sp
    writeln!(buf, "M=D")?;

    Ok(())
}

fn push_write_and_inc(buf: &mut BufWriter<fs::File>) -> std::io::Result<()> {
    // d = *sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M")?;
    writeln!(buf, "M=D")?;

    // ++sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M+1")?;
    
    Ok(())
}

fn push_segment(idx: i32, buf: &mut BufWriter<fs::File>, base_var: &str) -> std::io::Result<()> {
    // D = Ram[base_var + idx]
    writeln!(buf, "@{}", idx)?;
    writeln!(buf, "D=A")?;
    writeln!(buf, "@{}", base_var)?;
    writeln!(buf, "A=D+M")?;
    writeln!(buf, "D=M")?;

    push_write_and_inc(buf)
}

fn comp(buf: &mut BufWriter<fs::File>, condition: &str, cont_idx: &mut i32) -> std::io::Result<()> {
    //--sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M-1")?;

    // d = *sp = x
    writeln!(buf, "A=M")?;
    writeln!(buf, "D=M")?;

    // let *(sp-1) = y

    // *(sp - 1) = x `condition` y
    writeln!(buf, "A=A-1")?; // A = sp-1
    writeln!(buf, "MD=M-D")?;

    writeln!(buf, "@__eq.true{}", cont_idx)?;
    writeln!(buf, "D; {}", condition)?;
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M")?;
    writeln!(buf, "A=A-1")?;
    writeln!(buf, "M=0")?;
    writeln!(buf, "@__cont{}", cont_idx)?;
    writeln!(buf, "0; JMP")?;
    writeln!(buf, "(__eq.true{})", cont_idx)?;
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M")?;
    writeln!(buf, "A=A-1")?;
    writeln!(buf, "M=-1")?;
    writeln!(buf, "(__cont{})", cont_idx)?;

    *cont_idx += 1;

    Ok(())
}

fn arith(buf: &mut BufWriter<fs::File>, op: &str) -> std::io::Result<()> {
    //--sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M-1")?;

    // d = *sp
    writeln!(buf, "A=M")?;
    writeln!(buf, "D=M")?;

    // *(sp-1) = *(sp-1) + *sp
    writeln!(buf, "A=A-1")?;
    writeln!(buf, "M=M{}D", op)?;

    Ok(())
}

fn output(bytecode: &Vec<Op>, out_file: &mut BufWriter<fs::File>, filename: &str) -> std::io::Result<()> {
    let mut cont_idx = 0;
    for op in bytecode {
        match op {
            Op::Pop(seg, idx) => {
                writeln!(out_file, "// pop {:?} {}", seg, idx)?;
                match seg {
                    Segment::Local => {
                        pop_segment(*idx, out_file, "LCL")?;
                    },
                    Segment::Argument => {
                        pop_segment(*idx, out_file, "ARG")?;
                    },
                    Segment::This => {
                        pop_segment(*idx, out_file, "THIS")?;
                    },
                    Segment::That => {
                        pop_segment(*idx, out_file, "THAT")?;
                    },
                    Segment::Static => {
                        // --sp
                        writeln!(out_file, "@SP")?;
                        writeln!(out_file, "M=M-1")?;

                        // A = sp
                        writeln!(out_file, "A=M")?;

                        // D = *sp
                        writeln!(out_file, "D=M")?;

                        // Ram[static.idx] = D = *sp 
                        writeln!(out_file, "@{}.{}", filename, *idx)?;
                        writeln!(out_file, "M=D")?;

                    },
                    Segment::Pointer => {
                        // --sp
                        writeln!(out_file, "@SP")?;
                        writeln!(out_file, "M=M-1")?;

                        // A = sp
                        writeln!(out_file, "A=M")?;
                        // D = *sp
                        writeln!(out_file, "D=M")?;

                        if *idx == 0 {
                            // THIS = *sp
                            writeln!(out_file, "@THIS")?;
                            writeln!(out_file, "M=D")?;
                        } else {
                            assert!(*idx == 1);
                            // THAT = *sp
                            writeln!(out_file, "@THAT")?;
                            writeln!(out_file, "M=D")?;
                        }
                    },
                    Segment::Temp => {
                        assert!(*idx < 8);
                        // --sp
                        writeln!(out_file, "@SP")?;
                        writeln!(out_file, "M=M-1")?;

                        // A = sp
                        writeln!(out_file, "A=M")?;

                        // D = *sp
                        writeln!(out_file, "D=M")?;

                        // Temp.idx = D = *sp
                        writeln!(out_file, "@R{}", 5 + *idx)?;
                        writeln!(out_file, "M=D")?;
                    },
                    _ => {
                        panic!("Unsupported pop segment!");
                    },
                }
            },
            Op::Push(seg, idx) => {
                writeln!(out_file, "// push {:?} {}", seg, idx)?;
                match seg {
                    Segment::Local => {
                        push_segment(*idx, out_file, "LCL")?;
                    },
                    Segment::Argument => {
                        push_segment(*idx, out_file, "ARG")?;
                    },
                    Segment::This => {
                        push_segment(*idx, out_file, "THIS")?;
                    },
                    Segment::That => {
                        push_segment(*idx, out_file, "THAT")?;
                    },
                    Segment::Static => {
                        assert!(*idx < 240);

                        // D = Ram[static.idx]
                        writeln!(out_file, "@{}.{}", filename, *idx)?;
                        writeln!(out_file, "D=M")?;

                        push_write_and_inc(out_file)?;
                    },
                    Segment::Pointer => {
                        assert!(*idx == 0 || *idx == 1);

                        // D = THIS/THAT
                        if *idx == 0 {
                            writeln!(out_file, "@THIS")?;
                        } else {
                            writeln!(out_file, "@THAT")?;
                        }
                        writeln!(out_file, "D=M")?;

                        push_write_and_inc(out_file)?;
                    },
                    Segment::Temp => {
                        assert!(*idx < 8);
                        
                        // D = Ram[idx + 5]
                        writeln!(out_file, "@R{}", *idx + 5)?;
                        writeln!(out_file, "D=M")?;

                        push_write_and_inc(out_file)?;
                    },
                    Segment::Constant => {
                        // D = idx
                        writeln!(out_file, "@{}", *idx)?;
                        writeln!(out_file, "D=A")?;

                        push_write_and_inc(out_file)?;
                    },
                    _ => {
                        panic!("Unknown push segment!");
                    },
                }
            },
            Op::Add => {
                writeln!(out_file, "// add")?;
                arith(out_file, "+")?
            },
            Op::Sub => {
                writeln!(out_file, "// sub")?;
                arith(out_file, "-")?
            },
            Op::Neg => {
                writeln!(out_file, "// neg")?;

                // *sp = -*sp
                writeln!(out_file, "@SP")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "A=A-1")?;
                writeln!(out_file, "M=-M")?;
            },
            Op::Eq => {
                writeln!(out_file, "// eq")?;
                comp(out_file, "JEQ", &mut cont_idx)?;
            },
            Op::Gt => {
                writeln!(out_file, "// gt")?;
                comp(out_file, "JGT", &mut cont_idx)?;
            },
            Op::Lt => {
                writeln!(out_file, "// LT")?;
                comp(out_file, "JLT", &mut cont_idx)?;
            },
            Op::And => {
                writeln!(out_file, "// and")?;
                arith(out_file, "&")?
            },
            Op::Or => {
                writeln!(out_file, "// or")?;
                arith(out_file, "|")?
            },
            Op::Not => {
                writeln!(out_file, "// not")?;

                // *sp = !*sp
                writeln!(out_file, "@SP")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "A=A-1")?;
                writeln!(out_file, "M=!M")?;
            },
            Op::Unknown => {
                panic!("Unknown bytecode op!");
            },
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_file> <output_file>", args[0]);
        exit(1);
    }

    let in_filename = &args[1];
    let filename = Path::new(in_filename).file_name().unwrap().to_str().unwrap();

    let out_filename = args[2].to_owned();

    let in_file = fs::File::open(in_filename)?;
    let out_file = fs::File::create(out_filename)?;

    let in_reader = BufReader::new(in_file);
    let bytecode = parse(in_reader)?;

    let mut out_writer = BufWriter::new(out_file);
    output(&bytecode, &mut out_writer, filename)?;

    Ok(())
}
