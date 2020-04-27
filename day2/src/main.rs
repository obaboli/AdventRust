use std::fs::File;
use std::io::{self, prelude::*};
use itertools::Itertools;

fn run(vec: &Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut v = vec.clone();
    v[1] = noun;
    v[2] = verb;
    let mut op = 0;

    loop {
        match v[op] {
            1 => {
                let ip1 = v[v[op + 1] as usize];
                let ip2 = v[v[op + 2] as usize];
                let ip3: usize = v[op + 3] as usize; 

                v[ip3] = ip1 + ip2;            
            } 
            2 => {
                let ip1 = v[v[op + 1] as usize];
                let ip2 = v[v[op + 2] as usize];
                let ip3: usize = v[op + 3] as usize;
                v[ip3] = ip1 * ip2;
            } 
            _ => break
        }
        
        op += 4;
    }

    v[0]    
}

//used for debugging 
#[allow(dead_code)]
fn print(vec: &Vec<u32>) {
    let mut v_iter = vec.iter();

    while let Some((i1, i2, i3, i4)) = v_iter.next_tuple() {
        println!("{} {} {} {}", i1, i2, i3, i4);
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut v: Vec<u32> = Vec::new();
    
    for s in contents.split(",") {
        v.push(s.trim().parse::<u32>().unwrap());
    }

    println!("{}", run(&v, 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            if 19690720 ==  run(&v, noun, verb){
                println!("{}", (100 * noun + verb));
                break;
            }
        }
    }  
    Ok(())
}
