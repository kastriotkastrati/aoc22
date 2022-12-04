use std::collections::{HashMap, HashSet};

struct S1 {}
struct S2 {}

trait Differ {
    fn diff(words: &Vec<&str>) -> Vec<char>;
}

impl S1 {
    fn diff(words: &Vec<&str>) -> Vec<char> {
        let diffs: Vec<char> = words.iter().fold(vec![], |acc, word| {
            let (f, s) = (
                &word[..(word.len() / 2)].to_string(),
                &word[(word.len() / 2)..].to_string(),
            );

            let (f, s) = (
                f.chars()
                    .into_iter()
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .collect::<String>(),
                s.chars()
                    .into_iter()
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .collect::<String>(),
            );

            let repeated = XYZ::get_repeated(&vec![&format!("{}{}", f, s)]);
            return vec![acc, vec![repeated]].concat();
        });
        return diffs;
    }
}

impl S2 {
    fn diff(words: &Vec<&str>) -> Vec<char> {
        let accumulated = words
            .iter()
            .fold((vec![], vec![]), |acc, x| {
                let (c, n) = acc;
                let result = vec![n.clone(), vec![x]].concat();
                if result.len() == 3 {
                    return (vec![c, vec![result]].concat(), vec![]);
                }
                return (c, result);
            })
            .0;

        let diffed: Vec<char> = accumulated
            .iter()
            .map(|x| {
                let parsed: Vec<&str> = x
                    .iter()
                    .map(|x| {
                        return **x;
                    })
                    .collect();
                return XYZ::get_repeated(&parsed);
            })
            .collect();

        return diffed;
    }
}

struct XYZ {
    alphabet: HashMap<char, usize>,
}

impl XYZ {
    pub fn new() -> Self {
        let s = String::from("abcdefghijklmnopqrstuvwxyz");
        let s = format!("{}{}", s, s.to_uppercase());
        let alphabet: HashMap<char, usize> = s
            .char_indices()
            .into_iter()
            .map(|x| {
                return (x.1, x.0);
            })
            .collect();

        return XYZ { alphabet };
    }

    pub fn get_priority(&self, c: &char) -> usize {
        return self.alphabet.get(&c).unwrap().clone();
    }

    pub fn get_repeated(s: &Vec<&str>) -> char {
        let hs_array: Vec<String> = s
            .iter()
            .map(|x| {
                let res = x
                    .chars()
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .collect::<String>();
                return res;
            })
            .collect();

        let hs_array: Vec<&str> = hs_array.iter().map(|x| &**x).collect();
        let base_hashset = hs_array[0]
            .chars()
            .into_iter()
            .fold(HashSet::new(), |mut acc, x| {
                acc.insert(x);
                return acc;
            });

        let repeated = hs_array.iter().fold(base_hashset.clone(), |mut acc, x| {
            let found = x.chars().into_iter().fold(HashSet::new(), |mut acc2, ch| {
                let current = acc.get(&ch);
                if current == None {
                    acc.insert(ch);
                    return acc2;
                }
                acc2.insert(ch);
                return acc2;
            });

            return found;
        });

        return *repeated.iter().next().unwrap();
    }
}

pub fn calculate() -> i32 {
    let setup = XYZ::new();
    let path = std::path::Path::new("./src/day_3/data.txt");
    let data_text = std::fs::read_to_string(path).unwrap();
    let in_words: Vec<&str> = data_text.split("\n").collect();

    // part 1
    // let diffs = S1::diff(&in_words);

    // part 2
    let diffs = S2::diff(&in_words);

    let of_values: i32 = diffs.iter().fold(0, |acc, x| {
        let vl = setup.get_priority(x) + 1;
        return acc + (vl as i32);
    });

    return of_values;
}
