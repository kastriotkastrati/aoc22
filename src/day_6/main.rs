use std::collections::HashSet;

struct S1 {}
struct S2 {}

impl S1 {
    fn get_threshold() -> usize {
        return 4;
    }
}

impl S2 {
    fn get_threshold() -> usize {
        return 14;
    }
}

fn setup() -> Vec<char> {
    let path = std::path::Path::new("./src/day_6/data.txt");
    let data_text = std::fs::read_to_string(path).unwrap();

    return data_text.chars().collect();
}

fn check(s: &str) -> bool {
    let mut hs = HashSet::new();
    for x in s.chars() {
        hs.insert(x);
    }
    let check = hs.len() == s.len();
    return check;
}

pub fn calculate() -> usize {
    let line = setup();
    let ln = line.len();

    let result = {
        let mut idx = 0;
        let mut s = "".to_string();

        // part 1
        // let threshold = S1::get_threshold();

        // part 2
        let threshold = S2::get_threshold();

        loop {
            if idx == (ln - 1) {
                break;
            }

            let c = line.get(idx).unwrap();
            idx = idx + 1;

            if s.len() >= threshold {
                s = format!("{}", &s[1..])
            }

            s = format!("{}{}", s, c);

            if s.len() < threshold {
                continue;
            }

            let matched = check(&s);

            if matched {
                break;
            }
        }

        idx
    };

    return result;
}
