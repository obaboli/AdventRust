use std::collections::HashMap;

fn main() {
    let mut count = 0;

    for i in 271973..785961 {
        let v: Vec<char> = i.to_string().chars().collect();
        let mut check_flag = false;

        for window in v.windows(2) {
            if window[0] == window[1] {
                check_flag = true;
            }

            if window[0] > window[1] {
                check_flag = false;
                break;
            }
        }

        if check_flag {
            count += 1;
            //println!("{:?}", i)
        }
    }

    println!("{}", count);
    let mut count = 0;

    for i in 271973..785961 {
        let v: Vec<char> = i.to_string().chars().collect();
        let mut h = HashMap::new();
        let mut check_flag = true;

        for window in v.windows(2) {
            if window[0] == window[1] {
                *h.entry(window[0]).or_insert(0) += 1;
            }

            if window[0] > window[1] {
                check_flag = false;
                break;
            }
        }

        if check_flag {
            check_flag = false;
            for (_h, &n) in h.iter() {
                if n == 1 {
                    //println!("{}", i);
                    check_flag = true;
                    break;
                }
            }
        }

        if check_flag {
            count += 1;
        }
    }

    println!("{}", count);
}
