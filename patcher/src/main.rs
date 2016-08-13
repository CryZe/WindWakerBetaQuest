extern crate regex;
extern crate rustc_demangle;

mod dol;
mod assembler;

use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;
use assembler::Assembler;
use regex::Regex;
use rustc_demangle::demangle;

const FRAMEWORK_MAP: &'static str = include_str!("../framework.map");
const HEADER: &'static str = r".text section layout
  Starting        Virtual
  address  Size   address
  -----------------------";

fn create_framework_map() {
    let regex = Regex::new(r".text.(.+)\s*\n*\s*0x(\w+)\s*\n*\s*0x(\w+)\s*\n*\s*.+\((.+)\)")
        .unwrap();

    let mut file = BufReader::new(File::open("../build/intermediate.elf.map").unwrap());
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut file = BufWriter::new(File::create("../build/framework.map").unwrap());

    writeln!(file, "{}", HEADER).unwrap();

    for captures in regex.captures_iter(&content) {
        let fn_name = demangle(captures.at(1).unwrap().trim()).to_string();
        let address = captures.at(2).unwrap();
        let length = captures.at(3).unwrap();
        let source_file = captures.at(4).unwrap();
        let length = u32::from_str_radix(length, 16).unwrap();

        let mut fn_name: &str = &fn_name.replace(' ', "_")
            .replace("$C$", ",")
            .replace("$SP$", "@")
            .replace("$BP$", "*")
            .replace("$RF$", "&")
            .replace("$LT$", "<")
            .replace("$GT$", ">")
            .replace("$LP$", "(")
            .replace("$RP$", ")")
            .replace("$u20$", "_")
            .replace("$u27$", "'")
            .replace("$u5b$", "[")
            .replace("$u5d$", "]")
            .replace("$u7e$", "~")
            .replace("$u3b$", ";")
            .replace("$u7b$", "{")
            .replace("$u7d$", "}")
            .replace("..", "::")
            .replace(".", "-")
            .replace("_<", "<");

        let fn_name_bytes = fn_name.as_bytes();

        if fn_name.len() >= 19 && &fn_name_bytes[fn_name.len() - 19..][..3] == b"::h" {
            fn_name = &fn_name[..fn_name.len() - 19];
        }

        if address != "00000000" {
            writeln!(file,
                     "  00000000 {:06x} {}  4 {} \t{}",
                     length,
                     address,
                     fn_name,
                     source_file)
                .unwrap();
        }
    }

    write!(file, "{}", FRAMEWORK_MAP).unwrap();
}

fn main() {
    let mut asm = String::new();
    let _ = File::open("../src/src/patch.asm")
                .expect("Couldn't find \"src/src/patch.asm\". If you don't need to patch the dol, just create an empty file.")
                .read_to_string(&mut asm);

    let lines = &asm.lines().collect::<Vec<_>>();

    let mut assembler = Assembler::new("../build/intermediate.elf");
    let instructions = &assembler.assemble_all_lines(lines);

    let mut original = Vec::new();
    let _ = File::open("../game/original.dol")
                .expect("Couldn't find \"game/original.dol\". You need to copy the game's main.dol there.")
                .read_to_end(&mut original);

    let mut intermediate = Vec::new();
    let _ = File::open("../build/intermediate.dol")
                .expect("Couldn't find \"build/intermediate.dol\". Did you build the project correctly using \"make\"?")
                .read_to_end(&mut intermediate);

    let mut original = dol::DolFile::new(&original);
    let intermediate = dol::DolFile::new(&intermediate);
    original.append(intermediate);

    // println!("{:#?}", tww);

    original.patch(instructions);

    let data = original.to_bytes();
    let mut file = File::create("../game/sys/main.dol")
                       .expect("Couldn't create \"game/sys/main.dol\". You might need to provide higher privileges.");

    let _ = file.write(&data);

    create_framework_map();
}
