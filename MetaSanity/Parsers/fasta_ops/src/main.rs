use std::io::{BufRead, BufReader};
extern crate argparse;
use argparse::ArgumentParser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: <program-name> <file>");
        std::process::exit(1);
    }

    let reader = BufReader::new(std::fs::File::open(&args[1]).unwrap());
    let mut buffer: [u8; 80];

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let _line = line.as_bytes();
        if _line.len() > 0 {
            match _line[0] {
                b'>' => println!("{}", get_header(&_line)),
                _ => println!("{}", line)
                // _ => print_line_to_80(&_line, &buffer)
            };
        }
    }
}

/// Calls print for 80 char line segments
/// Builds passed buffer and writes once it is 80 chars long
fn print_line_to_80(line: &[u8], mut buffer: Option<&[u8]>) {
    let buffer_len = buffer.unwrap().len();
    // let total_length = line.len();
    // match total_length {
    //     val if val > 80 => {
    //         let mut _buffer = buffer.unwrap();
    //         match _buffer.len() {
    //             80 => {
    //                 println!("{}", std::str::from_utf8(&_buffer).unwrap())

    //             },
    //             _ => ()
    //         }
    //     },
    //     _ => println!("{}", std::str::from_utf8(&line[..]).unwrap())
    // };
}

/// Returns up to the first space in the header
fn get_header(line: &[u8]) -> &str {
    for (i, &item) in line.iter().enumerate() {
        if item == b' ' {
            return std::str::from_utf8(&line[0..i]).unwrap();
        }
    }
    std::str::from_utf8(&line[..]).unwrap()
}
