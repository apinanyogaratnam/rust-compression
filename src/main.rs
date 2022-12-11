extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 4 {
        eprintln!("Usage <source> <target>");
        eprint!("--compress to compress source file to target file");
        eprint!("--decompress to decompress source file to target file");
        return;
    }

    // compress
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());

    // decompress
    let input = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
    let mut output = File::create(args().nth(2).unwrap()).unwrap();
    let mut decoder = flate2::read::GzDecoder::new(input);
    let start = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    println!("Elapsed: {:?}", start.elapsed());
}
