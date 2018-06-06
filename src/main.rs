extern crate syn;

use std::fs::File;
use std::io::{Read, Write};
use std::env;

use syn::{ItemConst, Item, Lit, Expr};

trait Token {
    fn compile(self) -> String;
}

struct Constant {
    label: String,
    value: String,
}

impl Token for Constant {
    fn compile(self) -> String {
        let mut base = "#define ".to_owned();
        base.push_str(&self.label);
        base.push_str(" ");
        base.push_str(&self.value);
        base.push_str("\n");
        base
    }
}

fn read_input_file(filename: &str) -> Result<syn::File, String> {
    let file_option = File::open(&filename);
    let mut file = match file_option {
        Ok(n) => n,
        Err(_) => return Err(format!("Unable to open file {:?}", filename)),
    };

    let mut src = String::new();
    let read_status = file.read_to_string(&mut src);
    if !read_status.is_ok() {
        return Err(String::from("Unable to read file to string"));
    }

    match syn::parse_file(&src) {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Unable to parse {:?} due to {:?}", filename, e)),
    }
}

fn write_output_file(filename: &str, data: Vec<String>) -> Result<(), String> {
    let mut output_file = match File::create(filename) {
        Ok(n) => n,
        Err(e) => return Err(format!("Error opening file {:?} due to {:?}", filename, e)),
    };

    for item in data {
        let write_status = output_file.write(item.as_bytes());
        if !write_status.is_ok() {
            return Err(format!("Error writing {:?}", item));
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Expected only two args, got {:?}", args.len());
        return;
    }

    let input_file = match args.get(1) {
        Some(n) => n,
        None => {
            println!("No input file provided");
            return;
        }
    };

    let output_file = match args.get(2) {
        Some(n) => n,
        None => {
            println!("No output file provided");
            return;
        }
    };

    let file_ast = match read_input_file(input_file) {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let data = match compile(file_ast) {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    match write_output_file(output_file, data) {
        Ok(()) => println!("Successfully generated files"),
        Err(e) => println!("Encountered error {:?}", e),
    }
}

fn compile(ast: syn::File) -> Result<Vec<String>, String> {
    let mut results = Vec::<String>::new();

    for item in ast.items {
        match item {
            Item::Const(item) => {
                match handle_constant(item) {
                    Ok(n) => results.push(n),
                    Err(e) => return Err(e),
                };
            }
            _ => return Err(String::from("Unknown type")),
        }
    }

    Ok(results)
}

fn handle_constant(constant_item: ItemConst) -> Result<String, String> {
    let exp = constant_item.expr;
    let name = constant_item.ident.to_string();

    let literal = match *exp {
        Expr::Lit(e) => e.lit,
        _ => return Err(String::from("Unknown expression type")),
    };

    let str_val = match literal {
        Lit::Str(inner) => inner.value(),
        Lit::Int(inner) => inner.value().to_string(),
        _ => return Err(String::from("Unknown literal type")),
    };

    let next_constant = Constant {
        label: name,
        value: str_val,
    };

    let compiled_val = next_constant.compile();
    Ok(compiled_val)
}
