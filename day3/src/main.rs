use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Point {
    xx: i64,
    yy: i64,
}

impl Point {
    fn up(&mut self) {
        self.yy -= 1;
    }
    fn down(&mut self) {
        self.yy += 1;
    }
    fn right(&mut self) {
        self.xx += 1;
    }
    fn left(&mut self) {
        self.xx -= 1
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = BufReader::new(file).lines();
    let mut hash1 = HashSet::new();
    let mut hash2 = HashSet::new();

    let mut point = Point { xx: 0, yy: 0 };

    for word in lines.next().unwrap().unwrap().split(',') {
        //println!("{} ", word);
        let (d, n) = word.split_at(1);
        let n = n.parse::<u64>().unwrap();
        let d = d.chars().next().unwrap();

        for _ in 0..n {
            match d {
                'R' => point.right(),
                'L' => point.left(),
                'U' => point.up(),
                'D' => point.down(),
                _ => panic!("Invalid Direction; {}", d),
            }
            hash1.insert((point.xx, point.yy));
        }
    }

    let mut point = Point { xx: 0, yy: 0 };

    for word in lines.next().unwrap().unwrap().split(',') {
        //println!("{} ", word);
        let (d, n) = word.split_at(1);
        let n = n.parse::<u64>().unwrap();
        let d = d.chars().next().unwrap();

        for _ in 0..n {
            match d {
                'R' => point.right(),
                'L' => point.left(),
                'U' => point.up(),
                'D' => point.down(),
                _ => panic!("Invalid Direction; {}", d),
            }
            hash2.insert((point.xx, point.yy));
        }
    }

    //println!("Intersection: {:?}", hash1.intersection(&hash2).collect::<Vec<&(i64,i64)>>());
    let intersections = hash1.intersection(&hash2).collect::<Vec<&(i64, i64)>>();
    let mut min = u64::MAX;
    for i in intersections {
        let c_min: u64 = (i.0 - 0).abs() as u64 + (i.1 - 0).abs() as u64;

        if min > c_min {
            min = c_min
        }
    }

    println!("{}", min);

    Ok(())
}
