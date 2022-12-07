use std::{cell::RefCell, rc::Rc};

fn setup() -> (Vec<Vec<char>>, Vec<Vec<i32>>) {
    let path = std::path::Path::new("./src/day_5/data.txt");
    let data_text = std::fs::read_to_string(path).unwrap();

    let by_line: Vec<&str> = data_text.split("\n").collect();

    let ln = by_line[0].len() + 1;
    let ln = (ln - (ln % 4)) / 4;

    let stack_depth = {
        let mut depth: usize = 0;
        for (i, x) in by_line.iter().enumerate() {
            if !x.is_empty() {
                continue;
            }
            depth = i - 1;
            break;
        }
        depth
    };

    let raw_crates = &by_line[0..stack_depth];
    let raw_moves = &by_line[stack_depth + 2..];

    let raw_crates: Vec<Vec<char>> = raw_crates
        .clone()
        .iter()
        .map(|x| {
            let f: Vec<char> = x
                .chars()
                .skip(1)
                .into_iter()
                .step_by(4)
                .map(|x| {
                    if x == ' ' {
                        return '-';
                    }
                    return x;
                })
                .collect();

            return f;
        })
        .rev()
        .collect();

    let crates: Vec<Vec<char>> = (0..ln)
        .map(|x| {
            let y: Vec<char> = raw_crates
                .iter()
                .filter_map(|y| {
                    let x = y.get(x).unwrap_or(&'-');
                    if *x == '-' {
                        return None;
                    }
                    return Some(*x);
                })
                .collect();

            return y;
        })
        .collect();

    let moves: Vec<Vec<i32>> = raw_moves
        .iter()
        .map(|x| {
            let spl = x.split_whitespace();
            let spl: Vec<i32> = spl
                .filter_map(|x| {
                    let nr = x.parse::<i32>().ok();
                    return nr;
                })
                .collect();

            return spl;
        })
        .collect();

    return (crates, moves);
}

pub fn calculate() -> String {
    let (crates, moves) = setup();
    let crates = Rc::new(RefCell::new(crates));

    let folded = moves.iter().fold(crates, |acc, x| {
        let acc_ = Rc::clone(&acc);

        let (take, from, to) = (x[0], x[1], x[2]);
        let (from, to) = (from - 1, to - 1);
        let ln = { acc_.borrow().get(from as usize).unwrap().len() };
        let from_take = ln - take as usize;

        let mut to_add: Vec<char> = acc_
            .as_ref()
            .borrow_mut()
            .get_mut(from as usize)
            .unwrap()
            .drain(from_take..)
            // part one
            // .rev()
            // part two withgoes the reversal of the vector
            .collect();

        acc_.as_ref()
            .borrow_mut()
            .get_mut(to as usize)
            .unwrap()
            .append(&mut to_add);

        return acc_;
    });

    let result = folded
        .as_ref()
        .borrow()
        .iter()
        .fold(String::from(""), |acc, x| {
            let reverse: Vec<&char> = x.iter().collect();
            if reverse.len() == 0 {
                return acc;
            }
            let created = format!("{}{}", acc, reverse.last().unwrap());
            return created;
        });

    return result;
}
