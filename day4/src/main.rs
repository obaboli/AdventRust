use std::collections::HashMap;

fn main() {
    let mut count_part1 = 0;
    let mut count_part2 = 0;

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
            // part1
            if h.values().any(|&val| val > 0) {
                count_part1 += 1;
            }

            // part2
            if h.values().any(|&val| val == 1) {
                count_part2 += 1;
            }
        }
    }

    println!("{} {}", count_part1, count_part2);
}
