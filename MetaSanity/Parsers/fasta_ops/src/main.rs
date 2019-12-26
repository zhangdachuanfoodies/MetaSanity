extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};
mod fasta_parser;

fn main() -> std::io::Result<()> {
    let mut fasta_file = String::new();
    let mut program = String::from("simplify");
    let mut header_adj = false;
    let mut query = String::new();
    
    // Parse arguments using scoped borrows
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("FastaOps - Simple operations of large fasta files");
        parser.refer(&mut program)
            .add_argument("program", Store, "Program to run. Select from: simplify/get")
            .required();
        parser.refer(&mut fasta_file)
            .add_argument("fasta-file", Store, "Fasta file to modify")
            .required();
        parser.refer(&mut header_adj)
            .add_option(&["-n", "--name-by-file"], StoreTrue, 
                "Name by file instead of default name by record");
        parser.refer(&mut query)
            .add_option(&["-q", "--query"], Store, 
                "Query id to gather");
        parser.parse_args_or_exit();
    }

    // Run calling program
    match program.as_ref() {
        "simplify" => {
            fasta_parser::FastaParser::parse_to_std(&fasta_file, header_adj);
        },
        "get" => {
            if query == "" {
                println!("Provide query id!");
                std::process::exit(1);
            }
            let query: std::vec::Vec<&str> = query.split(",").collect();
            // fasta_parser::FastaParser::new(&fasta_file, true).get(&query);
            if query.len() == 1 {
                fasta_parser::FastaParser::new(&fasta_file, true).get(query[0]);
            }
            else {
                fasta_parser::FastaParser::new(&fasta_file, true).get_list(&query);
            }
        },
        _ => {
            println!("Program {} does not exist! Select from: simplify/get", program);
            std::process::exit(1);
         }
    }
    // Notify Ok status
    Ok(())
}
