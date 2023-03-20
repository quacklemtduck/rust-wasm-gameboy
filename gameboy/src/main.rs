use std::env;
use std::fs::File;
use std::io::Read;
use gameboy::GameBoy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = load_file_to_vec(args[1].as_str()).unwrap();

    let mut gb = GameBoy::new(file);

    gb.start();
    for _ in 0..20000 {
        gb.step();
    }

    gb.print();
}

fn load_file_to_vec(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}