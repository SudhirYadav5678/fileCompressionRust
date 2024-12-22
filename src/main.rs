extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{self, copy, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Check for correct number of arguments
    if args().len() != 3 {
        eprintln!("Usage: <source> <target>");
        return Ok(());
    }

    // Get the source file from args()
    let source_file_path = args().nth(1).expect("No source file provided");
    let target_file_path = args().nth(2).expect("No target file provided");

    // Open the source file for reading
    let file = File::open(&source_file_path)?;
    let mut input = BufReader::new(file);

    // Open the target file for writing the compressed data
    let output = File::create(&target_file_path)?;

    // Create the GzEncoder to compress the data
    let mut encoder = GzEncoder::new(output, Compression::default());

    // Start timing the operation
    let start = Instant::now();

    // Perform the compression
    copy(&mut input, &mut encoder)?;

    // Finalize compression and get the output writer
    let output = encoder.finish()?;

    // Get metadata of the source file (size before compression)
    let source_metadata = input.get_ref().metadata()?;
    println!("Source file size: {} bytes", source_metadata.len());

    // Get metadata of the target compressed file (size after compression)
    let target_metadata = output.metadata()?;
    println!("Compressed file size: {} bytes", target_metadata.len());

    // Output elapsed time
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
