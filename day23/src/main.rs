#[derive(Debug)]
struct Game {
    cups: Vec<u8>,
}

impl Game {
    pub fn new(cups: &[u8]) -> Self {
        assert!((5..10).contains(&cups.len()));
        assert!((1..=cups.len() as u8).all(|i| cups.contains(&i)));
        Self {
            cups: cups.iter().copied().collect(),
        }
    }

    fn decrement(&self, value: u8) -> u8 {
        match value - 1 {
            0 => self.cups.len() as u8,
            r => r,
        }
    }

    pub fn play_turn(&mut self) {
        let position = {
            let mut destination = self.decrement(*self.cups.first().unwrap());
            loop {
                match self.cups[4..].iter().position(|&c| c == destination) {
                    Some(p) => break p + 5,
                    None => destination = self.decrement(destination),
                }
            }
        };

        self.cups[1..position].rotate_left(3);
        self.cups.rotate_left(1);
    }

    pub fn finalize(&mut self) -> u64 {
        let position = self.cups.iter().position(|&c| c == 1).unwrap();
        self.cups.rotate_left(position);
        self.cups[1..]
            .iter()
            .fold(0, |acc, &next| acc * 10 + next as u64)
    }
}

fn main() {
    let mut game = Game::new(&[9, 4, 2, 3, 8, 7, 6, 1, 5]);
    for _ in 0..100 {
        game.play_turn();
    }

    println!("Part 1: result = {}", game.finalize());
}

#[cfg(test)]
mod test {
    use super::Game;

    #[test]
    fn example_game_10() {
        let mut game = Game::new(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);
        for _ in 0..10 {
            game.play_turn();
        }
        let result = game.finalize();
        assert_eq!(result, 92658374);
    }

    #[test]
    fn example_game_100() {
        let mut game = Game::new(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);
        for _ in 0..100 {
            game.play_turn();
        }
        let result = game.finalize();
        assert_eq!(result, 67384529);
    }
}
