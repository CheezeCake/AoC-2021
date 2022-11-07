use std::collections::HashMap;
use std::io;

fn read_player_starting_position() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let pos = input
        .trim()
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap());
    if pos == 0 {
        panic!("error reading player starting position");
    }
    pos as usize
}

struct Dice {
    next_value: usize,
}

impl Dice {
    fn new() -> Self {
        Self { next_value: 1 }
    }

    fn roll(&mut self) -> usize {
        let r = self.next_value;
        self.next_value = if r == 100 { 1 } else { r + 1 };
        r
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn turn(&mut self, steps: usize) {
        for _ in 0..steps {
            self.position += 1;
            if self.position > 10 {
                self.position = 1;
            }
        }

        self.score += self.position;
    }
}

fn count_wins(
    player1: Player,
    player2: Player,
    player1_turn: bool,
    mem: &mut HashMap<(Player, Player, bool), (usize, usize)>,
) -> (usize, usize) {
    if let Some(&n) = mem.get(&(player1, player2, player1_turn)) {
        return n;
    }
    if player1.score >= 21 {
        mem.insert((player1, player2, true), (1, 0));
        mem.insert((player1, player2, false), (1, 0));
        return (1, 0);
    }
    if player2.score >= 21 {
        mem.insert((player1, player2, true), (0, 1));
        mem.insert((player1, player2, false), (0, 1));
        return (0, 1);
    }

    let mut wins = (0, 0);

    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                let roll = r1 + r2 + r3;
                let (a, b) = if player1_turn {
                    let mut p1 = player1;
                    p1.turn(roll);
                    count_wins(p1, player2, false, mem)
                } else {
                    let mut p2 = player2;
                    p2.turn(roll);
                    count_wins(player1, p2, true, mem)
                };
                wins.0 += a;
                wins.1 += b;
            }
        }
    }

    mem.insert((player1, player2, player1_turn), wins);

    wins
}

fn main() {
    let player1_initial_pos = read_player_starting_position();
    let player2_initial_pos = read_player_starting_position();

    let mut player1 = Player::new(player1_initial_pos);
    let mut player2 = Player::new(player2_initial_pos);
    let mut dice = Dice::new();
    let mut player1_turn = true;
    let mut n = 0;

    while player1.score < 1000 && player2.score < 1000 {
        let r = dice.roll() + dice.roll() + dice.roll();
        n += 3;

        if player1_turn {
            player1.turn(r);
        } else {
            player2.turn(r);
        }

        player1_turn = !player1_turn;
    }

    println!("part 1: {}", player1.score.min(player2.score) * n);

    let wins = count_wins(
        Player::new(player1_initial_pos),
        Player::new(player2_initial_pos),
        true,
        &mut HashMap::new(),
    );
    println!("part 2: {}", wins.0.max(wins.1));
}
