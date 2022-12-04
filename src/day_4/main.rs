use std::i32;

struct S1 {}
struct S2 {}

impl S1 {
    pub fn solver(x: &((i32, i32), (i32, i32))) -> i32 {
        let s1 = x.0 .0 - x.1 .0;
        let s2 = x.0 .1 - x.1 .1;

        if s1 * s2 * -1 >= 0 {
            return 1;
        }

        return 0;
    }
}

impl S2 {
    pub fn solver(x: &((i32, i32), (i32, i32))) -> i32 {
        let min_max = std::cmp::max(x.0 .0, x.1 .0);
        let max_min = std::cmp::min(x.0 .1, x.1 .1);

        if min_max <= max_min {
            return 1;
        }
        return 0;
    }
}

pub fn calculate() -> i32 {
    let path = std::path::Path::new("./src/day_4/data.txt");
    let data_text = std::fs::read_to_string(path).unwrap();
    let by_line: Vec<&str> = data_text.split("\n").collect();

    let complete: Vec<((i32, i32), (i32, i32))> = by_line
        .iter()
        .map(|x| {
            let x: Vec<&str> = x.split(",").collect();
            let x: Vec<(i32, i32)> = x
                .iter()
                .map(|y| {
                    let y: Vec<&str> = y.split("-").collect();
                    return (
                        y[0].parse::<i32>().unwrap_or(0),
                        y[1].parse::<i32>().unwrap_or(0),
                    );
                })
                .collect();
            return (x[0], x[1]);
        })
        .collect();

    let parsed: Vec<i32> = complete
        .iter()
        .map(|x| {
            // part 1
            // let solution = S1::solver(x);

            // part 2
            let solution = S2::solver(x);
            return solution;
        })
        .collect();

    let summed: i32 = parsed.iter().sum();
    return summed;
}
