
extern crate argparse;

use argparse::{ArgumentParser, Store};
use memmap::Mmap;
use memmap::MmapMut;
use memmap::MmapOptions;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::SeekFrom;
use std::io::Write;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process;

pub struct BSort2 {
    input: Mmap,
    output: MmapMut,
    file_length: u64,
    record_width: u64,
    index_width: u64
}


struct Histogram {
    total: u64,
    slots: [u64;256],
}

impl Histogram {
    fn update<'a>(&mut self, items: &mut impl Iterator<Item=&'a u8>) {
	for item in items {
	    self.slots[*item as usize] += 1;
	}
    }
	

    fn new() -> Self {
	Self {
	    total: 0,
	    slots: [0; 256],
	}
    }

    fn is_ascii(&self) -> bool {
	for index in 0..31 {
	    if self.slots[index] != 0 {
		return false;
	    }
	}
	for index in 128..255 {
	    if self.slots[index] != 0 {
		return false;
	    }
	}
	return true;
    }

}

impl BSort2 {
    
    fn new(input_filename: &str, output_filename: &str, input_file_length: u64, record_width: u64, index_width: u64) -> Self {
    
	if let Ok(input_file) = File::open(input_filename) {
	    if let Ok(input) = unsafe {MmapOptions::new().map(&input_file)}
	    { 
		let path: PathBuf = PathBuf::from(output_filename);
		if let Ok(mut output_file) = {OpenOptions::new().read(true).write(true).create(true).open(&path)} {
		    output_file.seek(SeekFrom::Start(input_file_length - 1));
		    output_file.write(" ". as_bytes());
		    if let Ok(mut output) = unsafe {MmapOptions::new().map_mut(&output_file)} {
			return  Self {input:input, output:output, file_length:input_file_length, record_width:record_width, index_width:index_width}
		    } else {
			println!("Unable to mmap output file: {}", output_filename);
			process::exit(1);
		    }   
		} else {
		    println!("Could not create {output_filename}");
		    process::exit(1);
		}
	    }
	    else
	    {
		println!("Unable to mmap input file: {input_filename} ");
		process::exit(1);
	    }
	}
	else
	{
	    println!("Can't open input_filename ({input_filename})");
	    process::exit(1);
	}
    }

    fn sort(&self) {
	let mut histogram =  Histogram::new();
	
	// Our initial histogram computation happens solo and on the input data set. 
	histogram.update(&mut self.input.iter().step_by(self.record_width as usize));
	// This is where the fun goes.
    }
}




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

    BSort2::new(&input_filename, &output_filename, input_length, 100, 10).sort();
}
