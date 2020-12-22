use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use ahash::AHashSet;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Clone)]
pub struct Game {
    player1: VecDeque<u64>,
    player2: VecDeque<u64>,
    winner: Option<Player>,
}

impl Game {
    pub fn new(player1: VecDeque<u64>, player2: VecDeque<u64>) -> Self {
        Self {
            player1,
            player2,
            winner: None,
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut player1 = VecDeque::new();
        let mut player2 = VecDeque::new();
        let lines = BufReader::new(file).lines();

        #[derive(Debug)]
        enum ParseState {
            Player1,
            Player2,
        }

        let mut state = ParseState::Player1;
        for line_result in lines {
            let line = line_result?;

            match line.as_str() {
                "" => (),
                "Player 1:" => state = ParseState::Player1,
                "Player 2:" => state = ParseState::Player2,
                _ => {
                    let value = line.parse()?;
                    match state {
                        ParseState::Player1 => player1.push_back(value),
                        ParseState::Player2 => player2.push_back(value),
                    }
                }
            }
        }
        Ok(Self::new(player1, player2))
    }

    pub fn play(&mut self) {
        while !self.player1.is_empty() && !self.player2.is_empty() {
            let card1 = self.player1.pop_front().unwrap();
            let card2 = self.player2.pop_front().unwrap();

            if card1 > card2 {
                self.player1.push_back(card1);
                self.player1.push_back(card2);
            } else {
                self.player2.push_back(card2);
                self.player2.push_back(card1);
            }
        }

        self.winner = if self.player2.is_empty() {
            Some(Player::Player1)
        } else {
            Some(Player::Player2)
        }
    }

    pub fn play_recursive(&mut self) {
        let mut previous_rounds = AHashSet::new();
        while !self.player1.is_empty() && !self.player2.is_empty() {
            if !previous_rounds.insert((self.player1.clone(), self.player2.clone())) {
                self.winner = Some(Player::Player1);
                return;
            }

            let card1 = self.player1.pop_front().unwrap();
            let card2 = self.player2.pop_front().unwrap();

            let winner = if self.player1.len() as u64 >= card1 && self.player2.len() as u64 >= card2
            {
                let mut sub_game = Self::new(
                    self.player1.iter().take(card1 as usize).copied().collect(),
                    self.player2.iter().take(card2 as usize).copied().collect(),
                );
                sub_game.play_recursive();
                sub_game.winner.unwrap()
            } else if card1 > card2 {
                Player::Player1
            } else {
                Player::Player2
            };

            match winner {
                Player::Player1 => {
                    self.player1.push_back(card1);
                    self.player1.push_back(card2);
                }
                Player::Player2 => {
                    self.player2.push_back(card2);
                    self.player2.push_back(card1);
                }
            }
        }

        self.winner = if self.player2.is_empty() {
            Some(Player::Player1)
        } else {
            Some(Player::Player2)
        }
    }

    pub fn winning_score(&self) -> Option<u64> {
        let winning_deck = self.winner.map(|p| match p {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        })?;

        let length = winning_deck.len();
        Some(
            winning_deck
                .iter()
                .enumerate()
                .map(|(i, card)| card * ((length - i) as u64))
                .sum(),
        )
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut game1 = {
        let path = ["data", "day22", "input.txt"].iter().collect::<PathBuf>();
        Game::load(path)?
    };
    let mut game2 = game1.clone();

    game1.play();
    println!("Part 1: score = {}", game1.winning_score().unwrap());

    game2.play_recursive();
    println!("Part 2: score = {}", game2.winning_score().unwrap());

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            1
        }
    });
}

#[cfg(test)]
mod test {
    use super::{Game, Player};

    #[test]
    fn part1_test() {
        let mut game = Game::new(
            [9, 2, 6, 3, 1].iter().copied().collect(),
            [5, 8, 4, 7, 10].iter().copied().collect(),
        );
        game.play();
        let result = game.winning_score();
        assert_eq!(result, Some(306));
    }

    #[test]
    fn part2_test() {
        let mut game = Game::new(
            [9, 2, 6, 3, 1].iter().copied().collect(),
            [5, 8, 4, 7, 10].iter().copied().collect(),
        );
        game.play_recursive();
        assert!(matches!(game.winner, Some(Player::Player2)));
        let result = game.winning_score();
        assert_eq!(result, Some(291));
    }
}
