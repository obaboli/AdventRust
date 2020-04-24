#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn getFuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn recurisveFuel(mut fuel: i64) -> i64 {
   let mut rsum: i64 = 0;
 
    while fuel > 0 {
        rsum += fuel;
        fuel = getFuel(fuel);
    }
        
    rsum
}

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum: i64 = 0;
    let mut rsum: i64 = 0;

    for line in reader.lines() { 
        let mass: i64 = line.unwrap().parse::<i64>().unwrap(); 
        let fuel: i64 = getFuel(mass);
        
        sum += fuel;
        rsum += recurisveFuel(fuel);
    }

    println!("{} {}", sum, rsum);

    Ok(())

}
