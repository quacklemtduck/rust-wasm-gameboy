use std::env;
use std::fs::File;
use std::io::Read;
use gameboy::GameBoy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = load_file_to_vec(args[1].as_str()).unwrap();

    let mut gb = GameBoy::new(file);
    //
    // gb.start();
    // for i in 0..100010000 {
    //     gb.step();
    //     if i%4 == 0 {
    //         gb.advance_line();
    //     }
    // }
    //
    // gb.print();
}

fn load_file_to_vec(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}