#[derive(Debug)]
struct Game {
    cups: Vec<usize>,
    current: usize,
}

impl Game {
    pub fn new(start_pattern: &[usize]) -> Self {
        assert!(start_pattern.len() > 5);
        assert!((1..=start_pattern.len()).all(|i| start_pattern.contains(&i)));
        let mut cups = vec![0; start_pattern.len()];
        let mut cups_iterator = start_pattern.iter().map(|c| c - 1);
        let current = cups_iterator.next().unwrap();
        let mut prev = current;
        for next in cups_iterator {
            cups[prev] = next;
            prev = next;
        }
        cups[prev] = current;
        Self { cups, current }
    }

    pub fn new_million(start_pattern: &[usize]) -> Self {
        let mut cups = (1..=1_000_000).collect::<Vec<_>>();

        let mut cups_iterator = start_pattern.iter().map(|c| c - 1);
        let current = cups_iterator.next().unwrap();
        let mut prev = current;
        for next in cups_iterator {
            cups[prev] = next;
            prev = next;
        }

        cups[prev] = start_pattern.len();
        cups[999_999] = current;

        Self { cups, current }
    }

    pub fn play_turn(&mut self) {
        let mut next3 = [self.cups[self.current]; 3];
        let mut prev = next3[0];
        for p in next3[1..].iter_mut() {
            *p = self.cups[prev];
            prev = *p;
        }

        let mut next = self.current.checked_sub(1).unwrap_or(self.cups.len() - 1);
        while next3.contains(&next) {
            next = next.checked_sub(1).unwrap_or(self.cups.len() - 1);
        }

        self.cups[self.current] = self.cups[next3[2]];
        self.cups[next3[2]] = self.cups[next];
        self.cups[next] = next3[0];
        self.current = self.cups[self.current];
    }

    pub fn labels_after_1(&self) -> u64 {
        assert!(self.cups.len() < 10);
        let mut result = 0;
        let mut next = self.cups[0];
        while next != 0 {
            result = result * 10 + next as u64 + 1;
            next = self.cups[next];
        }

        result
    }

    pub fn score_after_1(&self) -> u64 {
        let first = self.cups[0] as u64 + 1;
        let second = self.cups[self.cups[0]] as u64 + 1;
        first * second
    }
}

const INPUT: [usize; 9] = [9, 4, 2, 3, 8, 7, 6, 1, 5];

fn main() {
    let part1 = {
        let mut game = Game::new(&INPUT);
        for _ in 0..100 {
            game.play_turn();
        }
        game.labels_after_1()
    };

    println!("Part 1: result = {}", part1);

    let part2 = {
        let mut game = Game::new_million(&INPUT);
        for _ in 0..10_000_000 {
            game.play_turn();
        }
        game.score_after_1()
    };

    println!("Part 2: result = {}", part2);
}

#[cfg(test)]
mod test {
    use super::Game;

    const TEST_INPUT: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn example_game_10() {
        let mut game = Game::new(&TEST_INPUT);
        for _ in 0..10 {
            game.play_turn();
        }
        let result = game.labels_after_1();
        assert_eq!(result, 92658374);
    }

    #[test]
    fn example_game_100() {
        let mut game = Game::new(&TEST_INPUT);
        for _ in 0..100 {
            game.play_turn();
        }
        let result = game.labels_after_1();
        assert_eq!(result, 67384529);
    }

    #[test]
    fn example_game_10_000_000() {
        let mut game = Game::new_million(&TEST_INPUT);
        for _ in 0..10_000_000 {
            game.play_turn();
        }
        let result = game.score_after_1();
        assert_eq!(result, 149245887792);
    }
}
