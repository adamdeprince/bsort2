
extern crate argparse;

uss memmap::MmamOptions;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::process;
use argparse::{ArgumentParser, Store};

fn main() {
    let mut record_width = 0;
    let mut sortable_prefix_width = 0;
    let mut output_filename = "".to_string();
    let mut input_filename = "".to_string();
    {
	let mut ap = ArgumentParser::new();
	ap.set_description("Bsort2 - the world's fastest sort");
	ap.refer(&mut record_width)
	    .add_option(&["-r", "--record-width"], Store, "Width of the entire record")
	    .required();
	ap.refer(&mut sortable_prefix_width)
	    .add_option(&["-s", "--sortable-prefix-width"], Store, "Width of the orderable record prefix")
	    .required();
	ap.refer(&mut output_filename)
	    .add_option(&["-o", "--output-file"], Store, "Output filename")
	    .required();
	ap.refer(&mut input_filename)
	    .add_option(&["-i", "--input-file"], Store, "Input filename")
	    .required();

	ap.parse_args_or_exit();
    }

    
    println!("Bsort starting");
    println!("Record width: {}", record_width);
    println!("Prefix width: {}", sortable_prefix_width);
    println!("Input filename: {}", input_filename);
    println!("Output filename: {}", output_filename);

    let input_path = Path::new(&input_filename);
    if !input_path.exists() {
	println!("Error: input {} does not exist", input_filename);
	process::exit(1);
    }
    
    if !input_path.is_file() {
	println!("Error: input {} is not a file", input_filename);
	process::exit(1);
    }

    let mut input_length = 0;
    if let Ok(input_meta) = fs::metadata(&input_filename) {
	input_length = input_meta.len();
    } else {
	println!("Unable to find the length of {}", input_filename);
	process::exit(1);
    }

    if input_length == 0 {
	println!("Error: input {} is empty.  Nothing to sort", input_filename);
	process::exit(1);
    }

    if input_length % record_width != 0 {
	println!("Error: input {} length {} isn't divisible by the record_length {}",
		 input_filename,
		 input_length,
		 record_width);
	process::exit(1);
    }
}
