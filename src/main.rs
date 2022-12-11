extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn compress(compression_type: Option<Compression>) {
    let compression_type = compression_type.unwrap_or(Compression::default());
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, compression_type);
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}

fn decompress() {
    let input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let mut output = File::create(args().nth(2).unwrap()).unwrap();
    let mut decoder = flate2::read::GzDecoder::new(input);
    let start = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    println!("Elapsed: {:?}", start.elapsed());
}

fn main() {
    if args().len() < 4 {
        eprintln!("Usage <source> <target> <compression_type>");
        eprintln!("--compress to compress source file to target file");
        eprintln!("--decompress to decompress source file to target file");
        eprintln!("--help to show this help");
        eprintln!("<level> to set compression level (1-9). Used with --compress only");
        return;
    }

    match args().nth(3).unwrap().as_str() {
        "--compress" => compress(None),
        "--decompress" => decompress(),
        "--help" => eprintln!("Usage <source> <target> <command>"),
        "--best" => compress(Some(Compression::best())),
        "--fast" => compress(Some(Compression::fast())),
        _ => eprintln!("Unknown command"),
    }
}
