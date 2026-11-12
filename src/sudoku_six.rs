//! Generates and validates six-by-six boards with two-row, three-column boxes.
use std::hash::{BuildHasher, RandomState};

pub(super) const CELLS: usize = 36;
const SIDE: usize = 6;
const ALL_DIGITS: u8 = 0b111111;
const ORDER: [usize; CELLS] = {
    let mut indices = [0; CELLS];
    let mut i = 0;
    while i < CELLS {
        indices[i] = i;
        i += 1;
    }
    indices
};

#[derive(Clone)]
struct Board {
    cells: [u8; CELLS],
    rows: [u8; SIDE],
    columns: [u8; SIDE],
    boxes: [u8; SIDE],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [0; CELLS],
            rows: [0; SIDE],
            columns: [0; SIDE],
            boxes: [0; SIDE],
        }
    }
}

impl Board {
    fn from_cells(cells: &[u8; CELLS]) -> Option<Self> {
        let mut board = Self::default();
        for (index, &digit) in cells.iter().enumerate() {
            if digit != 0 {
                if digit > SIDE as u8 || board.candidates(index) & bit(digit) == 0 {
                    return None;
                }
                board.place(index, digit);
            }
        }
        Some(board)
    }

    fn coordinates(index: usize) -> (usize, usize, usize) {
        let (row, column) = (index / SIDE, index % SIDE);
        (row, column, row / 2 * 2 + column / 3)
    }

    fn candidates(&self, index: usize) -> u8 {
        let (row, column, group) = Self::coordinates(index);
        ALL_DIGITS & !(self.rows[row] | self.columns[column] | self.boxes[group])
    }

    fn place(&mut self, index: usize, digit: u8) {
        self.cells[index] = digit;
        self.toggle(index, digit);
    }

    fn remove(&mut self, index: usize) {
        self.toggle(index, self.cells[index]);
        self.cells[index] = 0;
    }

    fn toggle(&mut self, index: usize, digit: u8) {
        let (row, column, group) = Self::coordinates(index);
        self.rows[row] ^= bit(digit);
        self.columns[column] ^= bit(digit);
        self.boxes[group] ^= bit(digit);
    }

    fn search(
        &mut self,
        order: &[usize; CELLS],
        digits: &[u8; SIDE],
        limit: u8,
        solution: &mut [u8; CELLS],
    ) -> u8 {
        let Some((index, candidates)) = order
            .iter()
            .copied()
            .filter(|&i| self.cells[i] == 0)
            .map(|i| (i, self.candidates(i)))
            .min_by_key(|(_, mask)| mask.count_ones())
        else {
            *solution = self.cells;
            return 1;
        };
        let mut count = 0;
        for &digit in digits {
            if candidates & bit(digit) != 0 {
                self.place(index, digit);
                count += self.search(order, digits, limit - count, solution);
                self.remove(index);
                if count == limit {
                    break;
                }
            }
        }
        count
    }

    fn solved_by_single_candidates(mut self) -> bool {
        while let Some((index, mask)) = ORDER
            .into_iter()
            .filter(|&i| self.cells[i] == 0)
            .map(|i| (i, self.candidates(i)))
            .find(|(_, mask)| mask.count_ones() == 1)
        {
            self.place(index, mask.trailing_zeros() as u8 + 1);
        }
        !self.cells.contains(&0)
    }
}

const DIGITS: [u8; SIDE] = [1, 2, 3, 4, 5, 6];

fn bit(digit: u8) -> u8 {
    1 << (digit - 1)
}

pub(super) fn unique_solution(puzzle: &[u8; CELLS]) -> Option<[u8; CELLS]> {
    let mut board = Board::from_cells(puzzle)?;
    let mut solution = [0; CELLS];
    (board.search(&ORDER, &DIGITS, 2, &mut solution) == 1).then_some(solution)
}

pub(super) fn generate(
    advanced: bool,
    previous: &[u8; CELLS],
) -> Option<([u8; CELLS], [u8; CELLS])> {
    generate_with(advanced, previous, &RandomState::new())
}

fn generate_with(
    advanced: bool,
    previous: &[u8; CELLS],
    random: &impl BuildHasher,
) -> Option<([u8; CELLS], [u8; CELLS])> {
    let clues = if advanced { 12 } else { 16 };
    let mut draw = 0u64;
    let mut next = || {
        draw += 1;
        random.hash_one(draw)
    };
    // Bound the work done on the UI thread. The caller can use a validated template
    // if none of these attempts meets the difficulty criteria.
    for _ in 0..32 {
        let mut order = ORDER;
        let mut digits = DIGITS;
        shuffle(&mut order, &mut next);
        shuffle(&mut digits, &mut next);
        let mut solution = [0; CELLS];
        Board::default().search(&order, &digits, 1, &mut solution);
        let mut board = Board::from_cells(&solution)?;
        shuffle(&mut order, &mut next);
        let mut remaining = CELLS;
        let mut scratch = [0; CELLS];
        for index in order {
            let digit = board.cells[index];
            board.remove(index);
            if board.search(&ORDER, &DIGITS, 2, &mut scratch) == 1 {
                remaining -= 1;
                if remaining == clues {
                    if &board.cells != previous
                        && board.clone().solved_by_single_candidates() != advanced
                    {
                        return Some((board.cells, solution));
                    }
                    break;
                }
            } else {
                board.place(index, digit);
            }
        }
    }
    None
}

fn shuffle<T>(items: &mut [T], next: &mut impl FnMut() -> u64) {
    for i in (1..items.len()).rev() {
        items.swap(i, (next() % (i as u64 + 1)) as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::DefaultHasher;

    struct Seed(u64);

    impl BuildHasher for Seed {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> DefaultHasher {
            use std::hash::Hasher;
            let mut hasher = DefaultHasher::new();
            hasher.write_u64(self.0);
            hasher
        }
    }

    #[test]
    fn generated_puzzles_have_varied_clue_positions_and_unique_solutions() {
        for advanced in [false, true] {
            let mut layouts = HashSet::new();
            let mut previous = [0; CELLS];
            for seed in 0..80 {
                let (puzzle, solution) = generate_with(advanced, &previous, &Seed(seed))
                    .expect("generator should produce a puzzle within its attempt budget");
                assert_ne!(puzzle, previous);
                assert_eq!(unique_solution(&puzzle), Some(solution));
                assert_eq!(
                    puzzle.iter().filter(|&&digit| digit != 0).count(),
                    if advanced { 12 } else { 16 }
                );
                assert_eq!(
                    Board::from_cells(&puzzle)
                        .unwrap()
                        .solved_by_single_candidates(),
                    !advanced
                );
                layouts.insert(puzzle.map(|digit| digit != 0));
                previous = puzzle;
            }
            assert!(layouts.len() >= 72);
        }
    }
}
