use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(FromPrimitive)]
enum Opcode {
    Add = 1,
    Multiply,
    Input,
    Output,
}

fn get_mode(v: &Vec<i64>, pos: usize, mode: bool) -> i64 {
    if mode {
        return v[pos] as i64;
    }
    v[v[pos] as usize]
}

fn run(vec: &Vec<i64>) {
    let mut v = vec.clone();
    let mut program_counter = 0;
    let input = 1;

    loop {
        let mut v_split = format!("{}", v[program_counter])
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();

        let opcode = v_split.pop().unwrap();
        let _ = v_split.pop().unwrap_or(0);

        let mode1 = v_split.pop().unwrap_or(0);
        let mode2 = v_split.pop().unwrap_or(0);

        match FromPrimitive::from_u8(opcode) {
            Some(Opcode::Add) => {
                let ip1 = get_mode(&v, program_counter + 1, mode1 != 0);
                let ip2 = get_mode(&v, program_counter + 2, mode2 != 0);
                let ip3: usize = v[program_counter + 3] as usize;

                v[ip3] = ip1 + ip2;
                program_counter += 4;
            }
            Some(Opcode::Multiply) => {
                let ip1 = get_mode(&v, program_counter + 1, mode1 != 0);
                let ip2 = get_mode(&v, program_counter + 2, mode2 != 0);
                let ip3: usize = v[program_counter + 3] as usize;

                v[ip3] = ip1 * ip2;
                program_counter += 4;
            }
            Some(Opcode::Input) => {
                let ip1 = get_mode(&v, program_counter + 1, true);
                v[ip1 as usize] = input;
                program_counter += 2;
            }
            Some(Opcode::Output) => {
                println!("{}", get_mode(&v, program_counter + 1, mode1 != 0));
                program_counter += 2;
            }
            None => break,
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test.txt")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut v: Vec<i64> = Vec::new();

    for s in contents.split(",") {
        v.push(s.trim().parse::<i64>().unwrap());
    }

    run(&v);

    Ok(())
}
