use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Point {
    xx: i64,
    yy: i64,
    dd: u64,
}

impl Point {
    fn up(&mut self) {
        self.yy -= 1;
        self.dd += 1;
    }
    fn down(&mut self) {
        self.yy += 1;
        self.dd += 1;
    }
    fn right(&mut self) {
        self.xx += 1;
        self.dd += 1;
    }
    fn left(&mut self) {
        self.xx -= 1;
        self.dd += 1;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = BufReader::new(file).lines();

    //Part1 Collections
    let mut hash1 = HashSet::new();
    let mut hash2 = HashSet::new();

    //Part2 Collections
    let mut hash3 = HashMap::new();
    let mut distance = Vec::new();

    let mut point = Point {
        xx: 0,
        yy: 0,
        dd: 0,
    };

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
            hash3.insert((point.xx, point.yy), point.dd);
        }
    }

    let mut point = Point {
        xx: 0,
        yy: 0,
        dd: 0,
    };

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
            if hash3.contains_key(&(point.xx, point.yy)) {
                let dis1 = hash3.get(&(point.xx, point.yy)).unwrap();
                distance.push(dis1 + point.dd);
            }
        }
    }

    let intersections = hash1
        .intersection(&hash2)
        .map(|x| x.0.abs() + x.1.abs())
        .collect::<Vec<i64>>();
    println!("Part1: {:?}", intersections.iter().min().unwrap());

    println!("Part2: {:?}", distance.iter().min().unwrap());
    Ok(())
}
