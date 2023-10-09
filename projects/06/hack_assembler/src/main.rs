use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use phf::phf_map;

static JMP_TABLE: phf::Map<&'static str, u16> = phf_map! {
    "" => 0u16,
    "jgt" => 1u16,
    "jeq" => 2u16,
    "jge" => 3u16,
    "jlt" => 4u16,
    "jne" => 5u16,
    "jle" => 6u16,
    "jmp" => 7u16,
};

static DEST_TABLE: phf::Map<&'static str, u16> = phf_map! {
    "" => 0u16,
    "m" => 1u16,
    "d" => 2u16,
    "md" => 3u16,
    "a" => 4u16,
    "am" => 5u16,
    "ad" => 6u16,
    "amd" => 7u16,
};

static COMP_TABLE: phf::Map<&'static str, u16> = phf_map! {
    "0" =>   0b101010u16,
    "1" =>   0b111111u16,
    "-1" =>  0b111010u16,
    "d" =>   0b001100u16,
    "a" =>   0b110000u16,
    "!d" =>  0b001101u16,
    "!a" =>  0b110001u16,
    "-d" =>  0b001111u16,
    "-a" =>  0b110011u16,
    "d+1" => 0b011111u16,
    "a+1" => 0b110111u16,
    "d-1" => 0b001110u16,
    "a-1" => 0b110010u16,
    "d+a" => 0b000010u16,
    "d-a" => 0b010011u16,
    "a-d" => 0b000111u16,
    "d&a" => 0b000000u16,
    "d|a" => 0b010101u16,
};

static INTRINSIC_TABLE: phf::Map<&'static str, u16> = phf_map! {
    "r0" => 0,
    "r1" => 1,
    "r2" => 2,
    "r3" => 3,
    "r4" => 4,
    "r5" => 5,
    "r6" => 6,
    "r7" => 7,
    "r8" => 8,
    "r9" => 9,
    "r1O" => 10,
    "r11" => 11,
    "r12" => 12,
    "r13" => 13,
    "r14" => 14,
    "r15" => 15,
    "sp" => 0,
    "lcl" => 1,
    "arg" => 2,
    "this" => 3,
    "that" => 4,
    "screen" => 16384,
    "kbd" => 24576,
};

type SymMap = HashMap<String, (u16, u16)>;

fn read_asm(file: &str, asm : &mut Vec<String>, table: &mut SymMap) {
    let label_bytes : &[_] = &['(', ')'];
    let mut idx : u16 = 0;
    for line in fs::read_to_string(file).unwrap().lines() {
        let mut trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("//") {
            continue;
        }

        if let Some(cmt) = trimmed.find("//") {
            trimmed = trimmed[0..cmt].trim();
        }

        if trimmed.starts_with('(') {
            trimmed = trimmed.trim_matches(label_bytes);
            table.insert(trimmed.to_string(), (idx, u16::MAX));
            continue;
        }

        if trimmed.starts_with('@') {
            let addr_str = &trimmed[1..];
            if !INTRINSIC_TABLE.contains_key(&addr_str.to_lowercase()) {
                // If not label make sure to update the last index this
                // variable was used
                if let Some(p) = table.get_mut(addr_str) {
                    if p.1 != u16::MAX {
                        *p = (0xffffu16, idx);
                    }
                } else {
                    table.insert(addr_str.to_string(), (0xffffu16, idx));
                }
            }
        }

        asm.push(trimmed.to_string());

        idx += 1;
    }
}

fn assemble(out_file: &str, asm: &Vec<String>, table: &mut SymMap) {
    let mut binary_asm = Vec::<u16>::new();
    let mut stack: u16 = 16;
    let mut idx = 0;
    for instr in asm {
        //println!("Instr: {}", instr);

        // A-Instruction
        if instr.starts_with('@') {
            let addr : u16;
            let addr_str = &instr[1..];
            let addr_str = addr_str.trim();
            if let Ok(a) = addr_str.parse::<u16>() {
                addr = a;
            } else if let Some(p) = table.get_mut(addr_str) {
                if p.0 == 0xffffu16 {
                    addr = stack;
                    stack += 1;
                    *p = (addr, p.1);
                } else {
                    addr = p.0;
                }

                // This is the last usage of this variable
                if p.1 == idx {
                    stack -= 1;
                }
            } else if let Some(i) = INTRINSIC_TABLE.get(&addr_str.to_lowercase()) {
                addr = *i;
            } else {
                addr = 0;
                assert!(false);
            }

            binary_asm.push(addr);

            continue;
        }

        // C-Instruction
        let dest_jmp : Vec<&str> = instr.split(";").collect();
        let dest_eq : Vec<&str> = dest_jmp[0].split("=").collect();
        
        let mut bcode : u16 = 0xE000u16;
        if dest_jmp.len() > 1 {
            let jmp = dest_jmp[1].trim();
            if let Some(jmp_opcode) = JMP_TABLE.get(&jmp.to_lowercase()) {
                bcode = bcode | jmp_opcode;
            } else {
                println!("Unknown jmp: {}", jmp.to_lowercase());
            }
        }

        let dest = dest_eq[0].trim();
        let mut comp = dest;
        if dest_eq.len() > 1 {
            comp = dest_eq[1];

            if let Some(dst) = DEST_TABLE.get(&dest.to_lowercase()) {
                bcode = bcode | (dst << 3); 
            } else {
                println!("Unknown dest: {}", dest.to_lowercase());
            }
        }

        let comp = comp.to_lowercase();
        let comp = comp.trim();

        let comp_a = comp.replace("m", "a");
        let is_m = comp_a != comp;
        bcode = bcode | if is_m { 1u16 << 12 } else { 0 };

        let comp_a = comp_a.trim();
        if let Some(comp) = COMP_TABLE.get(comp_a) {
            bcode = bcode | (comp << 6);
        } else {
            println!("Unknown comp: {}", comp_a);
        }
        
        binary_asm.push(bcode);

        idx += 1;
    }

    let mut f = fs::File::create(out_file).unwrap();
    for instr in binary_asm {
        let instr_str = String::from(format!("{instr:0width$b}", width=16));
        writeln!(f, "{}", instr_str).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <hack_asm-file> <hack_output-file>", args[0]);
        return;
    }
    
    let mut asm = Vec::<String>::new();
    let mut table = SymMap::new();
    read_asm(args[1].as_str(), &mut asm, &mut table);
    assemble(args[2].as_str(), &asm, &mut table);
}
