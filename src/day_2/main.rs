/*
--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach.
To decide whose tent gets to be closest to the snack storage,
 a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players.
Each game contains many rounds; in each round,
the players each simultaneously choose one of Rock, Paper,
or Scissors using a hand shape. Then, a winner for that round is selected:
Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an
encrypted strategy guide (your puzzle input)
that they say will be sure to help you win.
"The first column is what your opponent is going to play:
A for Rock, B for Paper, and C for Scissors. The second column--"
Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response:
X for Rock, Y for Paper, and Z for Scissors.
Winning every time would be suspicious,
so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the
highest score. Your total score is the sum of your scores for each round.
The score for a single round is the score for the
shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
plus the score for the outcome of the round
(0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you,
you should calculate the score you would get if
you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A),
and you should choose Paper (Y).
This ends in a win for you with a score of 8 (2 because you
chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B),
and you should choose Rock (X).
This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors,
giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide,
you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly
according to your strategy guide?

*/

use std::collections::HashMap;
use std::rc::Rc;

struct RPS<'a> {
    from_x: Rc<HashMap<[&'a str; 2], i32>>,
    from_res: Rc<HashMap<(&'a str, i32), &'a str>>,
    mv_values: Rc<HashMap<&'a str, u32>>,
    mv_n_values: Rc<HashMap<&'a str, i32>>,
}

impl<'a> RPS<'a> {
    pub fn new() -> Self {
        let position = [
            ("A", "X", 0),
            ("A", "Y", 1),
            ("A", "Z", -1),
            ("B", "X", -1),
            ("B", "Y", 0),
            ("B", "Z", 1),
            ("C", "X", 1),
            ("C", "Y", -1),
            ("C", "Z", 0),
        ];

        let from_x = position
            .iter()
            .map(|x| {
                return (([x.0, x.1]), x.2);
            })
            .collect();

        let from_res = position.iter().map(|x| return ((x.0, x.2), x.1)).collect();
        let mv_values = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
        let mv_n_values = HashMap::from([("X", -1), ("Y", 0), ("Z", 1)]);

        return RPS {
            from_x: Rc::new(from_x),
            from_res: Rc::new(from_res),
            mv_values: Rc::new(mv_values),
            mv_n_values: Rc::new(mv_n_values),
        };
    }

    pub fn calculate_score(&self, mv: &[&str]) -> u32 {
        let win = **&self.from_x.get(mv).unwrap();
        let win_value = match win {
            _ if win == 0 => 3,
            _ if win > 0 => 6,
            _ if win < 0 => 0,
            _ => unreachable!(),
        };
        let move_value = **&self.mv_values.get(mv[1]).unwrap();

        return win_value + move_value;
    }

    pub fn calculate_needed_score(&self, given_mv: &[&str]) -> u32 {
        let m1 = given_mv[0];
        let m2_based_result = self
          .mv_n_values
          .get(given_mv[1])
          .unwrap();

        let n_result = self
            .from_res
            .get(&(m1.clone(), m2_based_result.clone()))
            .unwrap();

        
        return self.calculate_score(&[m1, *n_result]);
    }
}

pub fn calculate() -> u32 {

    let setup = RPS::new();
    let path = std::path::Path::new("./src/day_2/data.txt");
    let data_text = std::fs::read_to_string(path).unwrap();

    let split: Vec<&str> = data_text.split("\n").collect();

    let valued = split.iter().fold(vec![], |acc, x| {
        let x: Vec<&str> = x.split_whitespace().collect();
        let x = &x[0..2];

        // part 1
        // let value = setup.calculate_score(x);

        // part 2 
        let value = setup.calculate_needed_score(x);
        return vec![acc, vec![value]].concat();
    });

    let sum: u32 = valued.iter().sum();
    return sum;
}
