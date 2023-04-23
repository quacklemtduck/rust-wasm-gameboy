use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, BufReader};
use gameboy::GameBoy;
use gameboy::cpu::CPU;
use gameboy::memory::Memory;
use gameboy::state::CpuTest;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_json_file(args[1].as_str()).unwrap();


    println!("Running tests for {} ⏳", args[1].as_str());

    let mut err = false;

    for test in &file {

        let mut cpu = CPU::new();
        let mut mem = Memory::new(None);

        cpu.load_state(&test.initial);
        mem.load_state(&test.initial);

        let mut counter: u32 = 0;
        while counter < test.cycles.len() as u32 {
            counter += cpu.run(&mut mem) as u32;
        }
        let cpu_result = cpu.compare_state(&test.r#final);
        let mem_result = mem.compare_state(&test.r#final);
        match cpu_result {
            Ok(_) => {},
            Err(s) => {
                println!("Error in CPU for test {} ❌", test.name);
                println!("{}", s);
                err = true;
                break
            },
        }
        match mem_result {
            Ok(_) => {},
            Err(s) => {
                println!("Error in memory for test {} ❌", test.name);
                println!("{}", s);
                err = true;
                break
            },
        }

        //println!("Ran test {} without any errors 👍", test.name);
    }

    if !err {
        println!("Ran without any errors 👍")
    }
    
}

fn read_json_file(filename: &str) -> Result<Vec<CpuTest>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let test = serde_json::from_reader(reader)?;

    Ok(test)
}

fn load_file_to_vec(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}