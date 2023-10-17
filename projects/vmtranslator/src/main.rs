use core::panic;
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Clone)]
enum Segment {
    Unknown,

    Local,
    Argument,
    Static(String),
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

type Label = String;
type Value = i32;

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
    Pop(Segment, Value),
    Push(Segment, Value),
    Label(Label),
    Goto(Label),
    IfGoto(Label),
    Function(String, i32),
    Call(String, i32),
    Return,
}

fn get_segment(segment: &str, filename: Option<String>) -> Segment {
    match segment {
        "local" => Segment::Local,
        "argument" => Segment::Argument,
        "static" => Segment::Static(filename.unwrap()),
        "constant" => Segment::Constant,
        "this" => Segment::This,
        "that" => Segment::That,
        "pointer" => Segment::Pointer,
        "temp" => Segment::Temp,
        _ => Segment::Unknown,
    }
}                       

fn parse(src: BufReader<fs::File>, res: &mut Vec<Op>, filename: &str) -> Result<(), Box<dyn Error>> {
    for line in src.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Everything after first "//" is a comment
        let tokens = line.split("//").collect::<Vec<&str>>();

        let tokens = tokens[0].trim().split(" ").collect::<Vec<&str>>();
        assert!(tokens.len() < 4, "{}", line);

        let filename = if tokens.len() == 3 && tokens[1] == "static" {
            Some(filename.to_owned())
        } else {
            None
        };

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
                let seg = get_segment(tokens[1], filename);
                let idx = tokens[2].parse::<i32>()?;
                Op::Pop(seg, idx)
            },
            "push" => {
                let seg = get_segment(tokens[1], filename);
                let idx = tokens[2].parse::<i32>()?;
                Op::Push(seg, idx)
            },
            "label" => Op::Label(tokens[1].to_string()),
            "goto" => Op::Goto(tokens[1].to_string()),
            "if-goto" => Op::IfGoto(tokens[1].to_string()),
            "function" => {
                let name = tokens[1];
                let locals = tokens[2].parse::<i32>()?;
                Op::Function(name.to_string(), locals)
            },
            "call" => {
                let name = tokens[1];
                let args = tokens[2].parse::<i32>()?;
                Op::Call(name.to_string(), args)
            },
            "return" => Op::Return,
            _ => Op::Unknown,
        };

        res.push(op);
    }

    Ok(())
}

fn pop(buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    // --sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M-1")?;

    // A = sp
    writeln!(buf, "A=M")?;

    // D = *sp
    writeln!(buf, "D=M")?;

    *instr_cnt += 4;

    Ok(())
}

fn pop_static(idx: i32, buf: &mut BufWriter<fs::File>, filename: &str, instr_cnt: &mut i32) -> std::io::Result<()> {
    pop(buf, instr_cnt)?;

    // Ram[static.idx] = D = *sp 
    writeln!(buf, "@{}.{}", filename, idx)?;
    writeln!(buf, "M=D")?;

    *instr_cnt += 2;

    Ok(())
}

fn pop_pointer(idx: i32, buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    pop(buf, instr_cnt)?;

    if idx == 0 {
        // THIS = *sp
        writeln!(buf, "@THIS")?;
        writeln!(buf, "M=D")?;
    } else {
        assert!(idx == 1);
        // THAT = *sp
        writeln!(buf, "@THAT")?;
        writeln!(buf, "M=D")?;
    }

    *instr_cnt += 2;

    Ok(())
}

fn pop_temp(idx: i32, buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    assert!(idx < 8);

    pop(buf, instr_cnt)?;

    // Temp.idx = D = *sp
    writeln!(buf, "@R{}", 5 + idx)?;
    writeln!(buf, "M=D")?;

    *instr_cnt += 2;

    Ok(())
}

fn pop_segment(idx: i32, buf: &mut BufWriter<fs::File>, base_var: &str, instr_cnt: &mut i32) -> std::io::Result<()> {
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

    *instr_cnt += 15;

    Ok(())
}

fn push_write_and_inc(buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    // d = *sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M")?;
    writeln!(buf, "M=D")?;

    // ++sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M+1")?;

    *instr_cnt += 5;
    
    Ok(())
}

fn push_segment(idx: i32, buf: &mut BufWriter<fs::File>, base_var: &str, instr_cnt: &mut i32) -> std::io::Result<()> {
    // D = Ram[base_var + idx]
    writeln!(buf, "@{}", idx)?;
    writeln!(buf, "D=A")?;
    writeln!(buf, "@{}", base_var)?;
    writeln!(buf, "A=D+M")?;
    writeln!(buf, "D=M")?;

    *instr_cnt += 5;

    push_write_and_inc(buf, instr_cnt)
}

fn push_static(idx: i32, buf: &mut BufWriter<fs::File>, filename: &str, instr_cnt: &mut i32) -> std::io::Result<()> {
    assert!(idx < 240);

    // D = Ram[static.idx]
    writeln!(buf, "@{}.{}", filename, idx)?;
    writeln!(buf, "D=M")?;

    *instr_cnt += 2;

    push_write_and_inc(buf, instr_cnt)
}

fn push_pointer(idx: i32, buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    assert!(idx == 0 || idx == 1);

    // D = THIS/THAT
    if idx == 0 {
        writeln!(buf, "@THIS")?;
    } else {
        writeln!(buf, "@THAT")?;
    }
    writeln!(buf, "D=M")?;

    *instr_cnt += 2;

    push_write_and_inc(buf, instr_cnt)
}

fn push_temp(idx: i32, buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    assert!(idx < 8);
                        
    // D = Ram[idx + 5]
    writeln!(buf, "@R{}", idx + 5)?;
    writeln!(buf, "D=M")?;

    *instr_cnt += 2;

    push_write_and_inc(buf, instr_cnt)
}

fn push_const(idx: i32, buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    // D = idx
    writeln!(buf, "@{}", idx)?;
    writeln!(buf, "D=A")?;

    *instr_cnt += 2;

    push_write_and_inc(buf, instr_cnt)
}

fn comp(buf: &mut BufWriter<fs::File>, condition: &str, cont_idx: &mut i32, instr_cnt: &mut i32) -> std::io::Result<()> {
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
    *instr_cnt += 18;

    Ok(())
}

fn arith(buf: &mut BufWriter<fs::File>, op: &str, instr_cnt: &mut i32) -> std::io::Result<()> {
    //--sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "M=M-1")?;

    // d = *sp
    writeln!(buf, "A=M")?;
    writeln!(buf, "D=M")?;

    // *(sp-1) = *(sp-1) + *sp
    writeln!(buf, "A=A-1")?;
    writeln!(buf, "M=M{}D", op)?;

    *instr_cnt += 6;

    Ok(())
}

fn neg(buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    // *(sp-1) = -*(sp-1)
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M-1")?;
    writeln!(buf, "M=-M")?;

    *instr_cnt += 3;

    Ok(())
}

fn not(buf: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    // *sp = !*sp
    writeln!(buf, "@SP")?;
    writeln!(buf, "A=M-1")?;
    writeln!(buf, "M=!M")?;

    *instr_cnt += 3;

    Ok(())
}

fn call(out_file: &mut BufWriter<fs::File>, instr_cnt: &mut i32, name: &str, nargs: i32, curr_fun: &str, call_idx: &mut i32) -> std::io::Result<()> {
    // 45 instructions before return label
    writeln!(out_file, "@{}", 45 + *instr_cnt)?;
    writeln!(out_file, "D=A")?;
    *instr_cnt += 2;
    push_write_and_inc(out_file, instr_cnt)?; // 7

    writeln!(out_file, "@LCL")?;
    writeln!(out_file, "D=M")?;
    *instr_cnt += 2;
    push_write_and_inc(out_file, instr_cnt)?; // 14

    writeln!(out_file, "@ARG")?;
    writeln!(out_file, "D=M")?;
    *instr_cnt += 2;
    push_write_and_inc(out_file, instr_cnt)?; // 21

    writeln!(out_file, "@THIS")?;
    writeln!(out_file, "D=M")?;
    *instr_cnt += 2;
    push_write_and_inc(out_file, instr_cnt)?; // 28

    writeln!(out_file, "@THAT")?;
    writeln!(out_file, "D=M")?;
    *instr_cnt += 2;
    push_write_and_inc(out_file, instr_cnt)?; // 35

    writeln!(out_file, "@SP")?;
    writeln!(out_file, "D=M")?;
    writeln!(out_file, "@LCL")?;
    writeln!(out_file, "M=D")?;
    writeln!(out_file, "@{}", 5 + nargs)?;
    writeln!(out_file, "D=D-A")?;
    writeln!(out_file, "@ARG")?;
    writeln!(out_file, "M=D")?;

    writeln!(out_file, "@{}", generate_entry_point(name))?;
    writeln!(out_file, "0;JMP")?; // 45

    *instr_cnt += 10;

    writeln!(out_file, "({})", generate_return_addr(curr_fun, call_idx))?;

    Ok(())
}

fn generate_label(curr_fun: &str, label: &str) -> String {
    let mut res = String::new();
    if !curr_fun.is_empty() {
        res += curr_fun;
        res.push('$');
    }
    res += label;

    res
}

fn generate_entry_point(fun_name: &str) -> String {
    let mut res = String::new();
    res += fun_name;

    res
}

fn generate_return_addr(curr_fun: &str, call_idx: &mut i32) -> String {
    let mut res = String::new();
    assert!(!curr_fun.is_empty(), "Calling from outside a function: {}", call_idx);
    res += &curr_fun;
    res += "$ret.";
    res += &call_idx.to_string();

    *call_idx += 1;
    
    res
}

fn output(bytecode: &Vec<Op>, out_file: &mut BufWriter<fs::File>, instr_cnt: &mut i32) -> std::io::Result<()> {
    let mut cont_idx = 0;
    let mut curr_fun = String::new();
    let mut call_idx = 0;

    for op in bytecode {
        match op {
            Op::Pop(seg, idx) => {
                writeln!(out_file, "// pop {:?} {}", seg, idx)?;
                match seg {
                    Segment::Local => {
                        pop_segment(*idx, out_file, "LCL", instr_cnt)?;
                    },
                    Segment::Argument => {
                        pop_segment(*idx, out_file, "ARG", instr_cnt)?;
                    },
                    Segment::This => {
                        pop_segment(*idx, out_file, "THIS", instr_cnt)?;
                    },
                    Segment::That => {
                        pop_segment(*idx, out_file, "THAT", instr_cnt)?;
                    },
                    Segment::Static(filename) => {
                        pop_static(*idx, out_file, filename, instr_cnt)?;
                    },
                    Segment::Pointer => {
                        pop_pointer(*idx, out_file, instr_cnt)?;
                    },
                    Segment::Temp => {
                        pop_temp(*idx, out_file, instr_cnt)?;
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
                        push_segment(*idx, out_file, "LCL", instr_cnt)?;
                    },
                    Segment::Argument => {
                        push_segment(*idx, out_file, "ARG", instr_cnt)?;
                    },
                    Segment::This => {
                        push_segment(*idx, out_file, "THIS", instr_cnt)?;
                    },
                    Segment::That => {
                        push_segment(*idx, out_file, "THAT", instr_cnt)?;
                    },
                    Segment::Static(filename) => {
                        push_static(*idx, out_file, filename, instr_cnt)?;
                    },
                    Segment::Pointer => {
                        push_pointer(*idx, out_file, instr_cnt)?;
                    },
                    Segment::Temp => {
                        push_temp(*idx, out_file, instr_cnt)?;
                    },
                    Segment::Constant => {
                        push_const(*idx, out_file, instr_cnt)?;
                    },
                    _ => {
                        panic!("Unknown push segment!");
                    },
                }
            },
            Op::Add => {
                writeln!(out_file, "// add")?;
                arith(out_file, "+", instr_cnt)?
            },
            Op::Sub => {
                writeln!(out_file, "// sub")?;
                arith(out_file, "-", instr_cnt)?
            },
            Op::Neg => {
                writeln!(out_file, "// neg")?;
                neg(out_file, instr_cnt)?;
            },
            Op::Eq => {
                writeln!(out_file, "// eq")?;
                comp(out_file, "JEQ", &mut cont_idx, instr_cnt)?;
            },
            Op::Gt => {
                writeln!(out_file, "// gt")?;
                comp(out_file, "JGT", &mut cont_idx, instr_cnt)?;
            },
            Op::Lt => {
                writeln!(out_file, "// LT")?;
                comp(out_file, "JLT", &mut cont_idx, instr_cnt)?;
            },
            Op::And => {
                writeln!(out_file, "// and")?;
                arith(out_file, "&", instr_cnt)?
            },
            Op::Or => {
                writeln!(out_file, "// or")?;
                arith(out_file, "|", instr_cnt)?
            },
            Op::Not => {
                writeln!(out_file, "// not")?;
                not(out_file, instr_cnt)?;
            },
            Op::Label(label) => {
                let label = generate_label(&curr_fun, label);
                writeln!(out_file, "// label {}", label)?;
                writeln!(out_file, "({})", label)?;
            },
            Op::Goto(label) => {
                let label = generate_label( &curr_fun, label);
                writeln!(out_file, "// goto {}", label)?;
                writeln!(out_file, "@{}", label)?;
                writeln!(out_file, "0;JMP")?;
                *instr_cnt += 2;
            },
            Op::IfGoto(label) => {
                let label = generate_label( &curr_fun, label);

                writeln!(out_file, "// if-goto {}", label)?;
                // --sp
                writeln!(out_file, "@SP")?;
                writeln!(out_file, "M=M-1")?;

                // d = Ram[SP]
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "D=M")?;
                
                writeln!(out_file, "@{}", label)?;
                writeln!(out_file, "D;JLT")?;

                *instr_cnt += 6;
            },
            Op::Call(name, nargs) => {                
                writeln!(out_file, "// call {} {}", name, nargs)?;

                call(out_file, instr_cnt, &name, *nargs, &curr_fun, &mut call_idx)?;
            },
            Op::Function(name, nlocals) => {
                curr_fun = name.clone();

                writeln!(out_file, "// function {} {}", name, nlocals)?;

                writeln!(out_file, "({})", generate_entry_point(&name))?;
                if *nlocals > 0 {
                    writeln!(out_file, "@SP")?;
                    writeln!(out_file, "A=M")?;
                    for _ in 0..*nlocals {
                        writeln!(out_file, "M=0")?;
                        writeln!(out_file, "A=A+1")?;
                        *instr_cnt += 2;
                    }
                    writeln!(out_file, "D=A")?;
                    writeln!(out_file, "@SP")?;
                    writeln!(out_file, "M=D")?;
                    *instr_cnt += 5;
                }
            },
            Op::Return => {
                writeln!(out_file, "// return")?;

                writeln!(out_file, "@LCL")?;
                writeln!(out_file, "D=M-1")?; // D = address of old frame last value
                writeln!(out_file, "@R13")?;
                writeln!(out_file, "M=D")?; // RAM[13] = old frame end

                // save return address in case it's
                // overwritten by return value. This will
                // happen if function is called with 0 args.
                writeln!(out_file, "@4")?;
                writeln!(out_file, "D=D-A")?; // D = address of old frame first value = return address
                writeln!(out_file, "A=D")?;
                writeln!(out_file, "D=M")?;
                writeln!(out_file, "@R14")?;
                writeln!(out_file, "M=D")?;

                writeln!(out_file, "@SP")?;
                writeln!(out_file, "A=M-1")?;
                writeln!(out_file, "D=M")?; // d holds return value now

                writeln!(out_file, "@ARG")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "M=D")?; // RAM[ARG] holds return value now
                
                writeln!(out_file, "D=A")?; // D=ARG
                writeln!(out_file, "@SP")?;
                writeln!(out_file, "M=D+1")?; // SP = ARG + 1
                
                writeln!(out_file, "@R13")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "D=M")?; // d = that
                writeln!(out_file, "@THAT")?;
                writeln!(out_file, "M=D")?; // that restored
                writeln!(out_file, "@R13")?;
                writeln!(out_file, "M=M-1")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "D=M")?; // d = this
                writeln!(out_file, "@THIS")?;
                writeln!(out_file, "M=D")?; // this restored
                writeln!(out_file, "@R13")?;
                writeln!(out_file, "M=M-1")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "D=M")?; // d = ARG
                writeln!(out_file, "@ARG")?;
                writeln!(out_file, "M=D")?; // arg restored
                writeln!(out_file, "@R13")?;
                writeln!(out_file, "M=M-1")?;
                writeln!(out_file, "A=M")?;
                writeln!(out_file, "D=M")?; // d = LCL
                writeln!(out_file, "@LCL")?;
                writeln!(out_file, "M=D")?; // lcl restored

                writeln!(out_file, "@R14")?;
                writeln!(out_file, "A=M")?; // A = return address
                writeln!(out_file, "0;JMP")?;

                *instr_cnt += 45;
            },
            Op::Unknown => {
                panic!("Unknown bytecode op!");
            },
        }
    }

    Ok(())
}

fn get_vm_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if let Some("vm") = ext.to_str() {
                files.push(path.to_owned());
            }
        }
        return Ok(());
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        get_vm_files(&path, files)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_file|dir> <output_file>", args[0]);
        exit(1);
    }

    let mut in_files = Vec::new();
    let input_path = Path::new(&args[1]);
    get_vm_files(&input_path, &mut in_files)?;

    let out_filename = args[2].to_owned();
    let out_file = fs::File::create(out_filename)?;
    let mut out_writer = BufWriter::new(out_file);

    let mut instr_cnt = 0;
    let mut call_idx = 0;
    if in_files.len() > 1 {
        writeln!(out_writer, "@256")?;
        writeln!(out_writer, "D=A")?;
        writeln!(out_writer, "@SP")?;
        writeln!(out_writer, "M=D")?;
        instr_cnt += 4;

        call(&mut out_writer, &mut instr_cnt, "Sys.init", 0, "_", &mut call_idx)?;
    }

    let mut bytecode = Vec::new();
    for in_filepath in in_files {
        let filename = in_filepath.file_stem().unwrap().to_str().unwrap();

        let in_file = fs::File::open(&in_filepath)?;
        let in_reader = BufReader::new(in_file);
        println!("Parsing: {}...", filename);
        parse(in_reader, &mut bytecode, filename)?;
    }

    output(&bytecode, &mut out_writer, &mut instr_cnt)?;

    Ok(())
}
