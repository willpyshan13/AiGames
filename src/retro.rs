use super::*;
use std::collections::VecDeque;

const SNAKE_W: i8 = 12;
const SNAKE_H: i8 = 12;

pub(super) struct SnakeGame {
    pub(super) body: Vec<(i8, i8)>,
    previous_body: Vec<(i8, i8)>,
    pub(super) direction: (i8, i8),
    pending: (i8, i8),
    pub(super) fruit: (i8, i8),
    clock: f32,
    render_clock: f32,
    pub(super) move_progress: f32,
    pub(super) anim_time: f32,
    pub(super) eating_flash: f32,
    pub(super) score: u32,
    pub(super) best: u32,
    fruits_eaten: u32,
    pub(super) golden_fruit: bool,
    pub(super) running: bool,
    pub(super) started: bool,
    pub(super) over: bool,
}
impl SnakeGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            body: vec![],
            previous_body: vec![],
            direction: (1, 0),
            pending: (1, 0),
            fruit: (8, 6),
            clock: 0.,
            render_clock: 0.,
            move_progress: 0.,
            anim_time: 0.,
            eating_flash: 0.,
            score: 0,
            best: 0,
            fruits_eaten: 0,
            golden_fruit: false,
            running: false,
            started: false,
            over: false,
        };
        game.reset();
        game
    }
    pub(super) fn reset(&mut self) {
        self.body = vec![(5, 6), (4, 6), (3, 6)];
        self.previous_body = self.body.clone();
        self.direction = (1, 0);
        self.pending = (1, 0);
        self.fruit = (8, 6);
        self.clock = 0.;
        self.render_clock = 0.;
        self.move_progress = 0.;
        self.anim_time = 0.;
        self.eating_flash = 0.;
        self.score = 0;
        self.fruits_eaten = 0;
        self.golden_fruit = false;
        self.running = false;
        self.started = false;
        self.over = false;
    }
    pub(super) fn turn(&mut self, dx: i8, dy: i8) -> bool {
        if self.over || (dx, dy) == (-self.direction.0, -self.direction.1) {
            return false;
        }
        self.pending = (dx, dy);
        self.running = true;
        self.started = true;
        true
    }
    pub(super) fn toggle(&mut self) {
        if self.over {
            self.reset();
        }
        self.running = !self.running;
        self.started = true;
    }
    fn next_fruit(&mut self) {
        let start = (self.fruits_eaten as usize * 37) % 144;
        for step in 0..144 {
            let index = (start + step * 37) % 144;
            let point = (
                (index % SNAKE_W as usize) as i8,
                (index / SNAKE_W as usize) as i8,
            );
            if !self.body.contains(&point) {
                self.fruit = point;
                self.golden_fruit = (self.fruits_eaten + 1).is_multiple_of(4);
                return;
            }
        }
    }
    pub(super) fn advance(&mut self, dt: f32) -> bool {
        self.anim_time += dt;
        self.eating_flash = (self.eating_flash - dt).max(0.);
        if !self.running || self.over {
            return false;
        }
        self.clock += dt;
        let step_time = (0.16 - self.score as f32 * 0.004).max(0.085);
        while self.clock >= step_time && !self.over {
            self.clock -= step_time;
            self.previous_body.clone_from(&self.body);
            self.direction = self.pending;
            let head = self.body[0];
            let next = (head.0 + self.direction.0, head.1 + self.direction.1);
            let eating = next == self.fruit;
            let collision_len = if eating {
                self.body.len()
            } else {
                self.body.len().saturating_sub(1)
            };
            if next.0 < 0
                || next.1 < 0
                || next.0 >= SNAKE_W
                || next.1 >= SNAKE_H
                || self.body[..collision_len].contains(&next)
            {
                self.over = true;
                self.running = false;
                self.move_progress = 1.;
                return true;
            }
            self.body.insert(0, next);
            if eating {
                self.fruits_eaten += 1;
                self.score += if self.golden_fruit { 3 } else { 1 };
                self.best = self.best.max(self.score);
                self.eating_flash = 0.30;
                self.next_fruit();
            } else {
                self.body.pop();
            }
            self.move_progress = 0.;
        }
        self.move_progress = (self.clock / step_time).clamp(0., 1.);
        self.render_clock += dt;
        if self.render_clock >= 1. / 30. {
            self.render_clock %= 1. / 30.;
            true
        } else {
            false
        }
    }
    fn interpolated_body(&self) -> Vec<Vec2> {
        self.body
            .iter()
            .enumerate()
            .map(|(index, &(x, y))| {
                let from = self.previous_body.get(index).copied().unwrap_or((x, y));
                Vec2::new(from.0 as f32, from.1 as f32)
                    .lerp(Vec2::new(x as f32, y as f32), self.move_progress)
            })
            .collect()
    }
}

pub(super) struct MinesweeperGame {
    pub(super) mines: [bool; 64],
    pub(super) revealed: [bool; 64],
    pub(super) flagged: [bool; 64],
    pub(super) flag_mode: bool,
    pub(super) lost: bool,
    pub(super) won: bool,
}
impl MinesweeperGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            mines: [false; 64],
            revealed: [false; 64],
            flagged: [false; 64],
            flag_mode: false,
            lost: false,
            won: false,
        };
        game.reset();
        game
    }
    pub(super) fn reset(&mut self) {
        self.mines = [false; 64];
        for index in [2, 7, 13, 18, 27, 35, 41, 52, 58, 61] {
            self.mines[index] = true;
        }
        self.revealed = [false; 64];
        self.flagged = [false; 64];
        self.flag_mode = false;
        self.lost = false;
        self.won = false;
    }
    pub(super) fn around(&self, index: usize) -> u8 {
        let x = (index % 8) as i8;
        let y = (index / 8) as i8;
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| !(*dx == 0 && *dy == 0))
            .filter_map(|(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < 8 && ny < 8 {
                    Some(ny as usize * 8 + nx as usize)
                } else {
                    None
                }
            })
            .filter(|&cell| self.mines[cell])
            .count() as u8
    }
    pub(super) fn toggle_flag(&mut self, index: usize) {
        if !self.revealed[index] && !self.lost && !self.won {
            self.flagged[index] = !self.flagged[index];
        }
    }
    pub(super) fn reveal(&mut self, index: usize) {
        if self.flag_mode {
            self.toggle_flag(index);
            return;
        }
        if self.revealed[index] || self.flagged[index] || self.lost || self.won {
            return;
        }
        // The first move always opens a safe square.
        if !self.revealed.iter().any(|&revealed| revealed) && self.mines[index] {
            self.mines[index] = false;
            if let Some(next) = (0..64).find(|&cell| !self.mines[cell] && cell != index) {
                self.mines[next] = true;
            }
        }
        if self.mines[index] {
            self.lost = true;
            self.revealed[index] = true;
            return;
        }
        let mut queue = VecDeque::from([index]);
        while let Some(cell) = queue.pop_front() {
            if self.revealed[cell] || self.flagged[cell] || self.mines[cell] {
                continue;
            }
            self.revealed[cell] = true;
            if self.around(cell) == 0 {
                let x = (cell % 8) as i8;
                let y = (cell / 8) as i8;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && ny >= 0 && nx < 8 && ny < 8 {
                            queue.push_back(ny as usize * 8 + nx as usize);
                        }
                    }
                }
            }
        }
        self.won = (0..64).all(|cell| self.mines[cell] || self.revealed[cell]);
    }
}

pub(super) struct Classic2048Game {
    pub(super) cells: [u16; 16],
    pub(super) score: u32,
    pub(super) best: u32,
    seed: u32,
    pub(super) over: bool,
    pub(super) won: bool,
    pub(super) moves: u32,
}
impl Classic2048Game {
    pub(super) fn new() -> Self {
        let mut game = Self {
            cells: [0; 16],
            score: 0,
            best: 0,
            seed: 1,
            over: false,
            won: false,
            moves: 0,
        };
        game.reset();
        game
    }
    pub(super) fn reset(&mut self) {
        self.cells = [0; 16];
        self.score = 0;
        self.over = false;
        self.won = false;
        self.moves = 0;
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.spawn();
        self.spawn();
    }
    fn spawn(&mut self) {
        let empty: Vec<_> = (0..16).filter(|&i| self.cells[i] == 0).collect();
        if empty.is_empty() {
            return;
        }
        let index = empty[self.seed as usize % empty.len()];
        self.cells[index] = if self.seed % 5 == 0 { 4 } else { 2 };
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
    }
    pub(super) fn move_board(&mut self, dx: i8, dy: i8) -> bool {
        if self.over || !matches!((dx, dy), (-1, 0) | (1, 0) | (0, -1) | (0, 1)) {
            return false;
        }
        let mut changed = false;
        for lane in 0..4 {
            let indices: Vec<usize> = if dx != 0 {
                (0..4)
                    .map(|step| lane * 4 + if dx > 0 { 3 - step } else { step })
                    .collect()
            } else {
                (0..4)
                    .map(|step| (if dy > 0 { 3 - step } else { step }) * 4 + lane)
                    .collect()
            };
            let values: Vec<u16> = indices
                .iter()
                .map(|&i| self.cells[i])
                .filter(|&v| v > 0)
                .collect();
            let mut merged = Vec::new();
            let mut pos = 0;
            while pos < values.len() {
                if pos + 1 < values.len() && values[pos] == values[pos + 1] {
                    let v = values[pos].saturating_mul(2);
                    self.score += u32::from(v);
                    self.won |= v >= 2048;
                    merged.push(v);
                    pos += 2;
                } else {
                    merged.push(values[pos]);
                    pos += 1;
                }
            }
            for (slot, &index) in indices.iter().enumerate() {
                let next = merged.get(slot).copied().unwrap_or(0);
                changed |= self.cells[index] != next;
                self.cells[index] = next;
            }
        }
        if changed {
            self.moves += 1;
            self.best = self.best.max(self.score);
            self.spawn();
        }
        self.over = !self.cells.contains(&0)
            && ![(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .any(|&(x, y)| self.can_move(x, y));
        changed
    }
    fn can_move(&self, dx: i8, dy: i8) -> bool {
        for y in 0..4_i8 {
            for x in 0..4_i8 {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < 4 && ny < 4 {
                    let a = self.cells[y as usize * 4 + x as usize];
                    let b = self.cells[ny as usize * 4 + nx as usize];
                    if a == 0 || a == b {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Clone)]
pub(super) struct KlotskiPiece {
    pub(super) x: i8,
    pub(super) y: i8,
    pub(super) w: i8,
    pub(super) h: i8,
    pub(super) label: &'static str,
    pub(super) color: Color,
}
pub(super) struct KlotskiGame {
    pub(super) pieces: Vec<KlotskiPiece>,
    pub(super) selected: usize,
    pub(super) won: bool,
}
impl KlotskiGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            pieces: vec![],
            selected: 0,
            won: false,
        };
        game.reset();
        game
    }
    pub(super) fn reset(&mut self) {
        self.pieces = vec![
            KlotskiPiece {
                x: 1,
                y: 0,
                w: 2,
                h: 2,
                label: "曹操",
                color: Color::srgb(0.80, 0.25, 0.20),
            },
            KlotskiPiece {
                x: 0,
                y: 0,
                w: 1,
                h: 2,
                label: "关羽",
                color: Color::srgb(0.26, 0.53, 0.80),
            },
            KlotskiPiece {
                x: 3,
                y: 0,
                w: 1,
                h: 2,
                label: "张飞",
                color: Color::srgb(0.29, 0.65, 0.43),
            },
            KlotskiPiece {
                x: 1,
                y: 2,
                w: 2,
                h: 1,
                label: "赵云",
                color: Color::srgb(0.78, 0.52, 0.20),
            },
            KlotskiPiece {
                x: 0,
                y: 2,
                w: 1,
                h: 2,
                label: "马超",
                color: Color::srgb(0.52, 0.35, 0.72),
            },
            KlotskiPiece {
                x: 3,
                y: 2,
                w: 1,
                h: 2,
                label: "黄忠",
                color: Color::srgb(0.31, 0.62, 0.70),
            },
            KlotskiPiece {
                x: 1,
                y: 3,
                w: 1,
                h: 1,
                label: "兵",
                color: Color::srgb(0.62, 0.62, 0.66),
            },
            KlotskiPiece {
                x: 2,
                y: 3,
                w: 1,
                h: 1,
                label: "兵",
                color: Color::srgb(0.62, 0.62, 0.66),
            },
        ];
        self.selected = 0;
        self.won = false;
    }
    pub(super) fn move_selected(&mut self, dx: i8, dy: i8) -> bool {
        if self.won {
            return false;
        }
        let piece = &self.pieces[self.selected];
        let (nx, ny) = (piece.x + dx, piece.y + dy);
        if nx < 0 || ny < 0 || nx + piece.w > 4 || ny + piece.h > 5 {
            return false;
        }
        for (i, other) in self.pieces.iter().enumerate() {
            if i == self.selected {
                continue;
            }
            if nx < other.x + other.w
                && nx + piece.w > other.x
                && ny < other.y + other.h
                && ny + piece.h > other.y
            {
                return false;
            }
        }
        self.pieces[self.selected].x = nx;
        self.pieces[self.selected].y = ny;
        self.won = self.pieces[0].y == 3;
        true
    }
}

pub(super) struct SudokuGame {
    puzzle: [u8; 81],
    solution: [u8; 81],
    pub(super) cells: [u8; 81],
    pub(super) selected: Option<usize>,
    pub(super) mistakes: u8,
    pub(super) won: bool,
    pub(super) size: usize,
    pub(super) variant: u32,
}
impl SudokuGame {
    pub(super) fn new() -> Self {
        Self::with_size(9, 0)
    }
    fn with_size(size: usize, variant: u32) -> Self {
        let classic_puzzle = [
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];
        let classic_solution = [
            5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8, 1, 9, 8, 3, 4, 2, 5, 6, 7, 8, 5,
            9, 7, 6, 1, 4, 2, 3, 4, 2, 6, 8, 5, 3, 7, 9, 1, 7, 1, 3, 9, 2, 4, 8, 5, 6, 9, 6, 1, 5,
            3, 7, 2, 8, 4, 2, 8, 7, 4, 1, 9, 6, 3, 5, 3, 4, 5, 2, 8, 6, 1, 7, 9,
        ];
        let beginner_puzzle = [1, 0, 0, 4, 0, 4, 0, 2, 2, 0, 4, 0, 4, 0, 0, 1];
        let beginner_solution = [1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1];
        let mut puzzle = [0; 81];
        let mut solution = [0; 81];
        let box_size = if size == 4 { 2 } else { 3 };
        for row in 0..size {
            for col in 0..size {
                let shift = (variant as usize / 2) % box_size;
                let mut source_row =
                    (row / box_size) * box_size + (row % box_size + shift) % box_size;
                let mut source_col =
                    (col / box_size) * box_size + (col % box_size + shift) % box_size;
                if variant % 2 == 1 {
                    std::mem::swap(&mut source_row, &mut source_col);
                }
                let source = source_row * size + source_col;
                let target = row * size + col;
                let (raw_puzzle, raw_solution) = if size == 4 {
                    (beginner_puzzle[source], beginner_solution[source])
                } else {
                    (classic_puzzle[source], classic_solution[source])
                };
                let digit_shift = ((variant / 3) as usize) % size;
                let remap = |value: u8| {
                    if value == 0 {
                        0
                    } else {
                        ((value as usize - 1 + digit_shift) % size + 1) as u8
                    }
                };
                puzzle[target] = remap(raw_puzzle);
                solution[target] = remap(raw_solution);
            }
        }
        Self {
            cells: puzzle,
            puzzle,
            solution,
            selected: None,
            mistakes: 0,
            won: false,
            size,
            variant,
        }
    }
    pub(super) fn set_size(&mut self, size: usize) {
        if (size == 4 || size == 9) && self.size != size {
            *self = Self::with_size(size, 0);
        }
    }
    pub(super) fn new_puzzle(&mut self) {
        *self = Self::with_size(self.size, self.variant.wrapping_add(1));
    }
    pub(super) fn reset(&mut self) {
        self.cells = self.puzzle;
        self.selected = None;
        self.mistakes = 0;
        self.won = false;
    }
    pub(super) fn is_wrong(&self, index: usize) -> bool {
        self.cells[index] != 0 && self.cells[index] != self.solution[index]
    }

    pub(super) fn is_given(&self, index: usize) -> bool {
        self.puzzle[index] != 0
    }
    pub(super) fn select(&mut self, index: usize) {
        if index < self.size * self.size && !self.won {
            self.selected = Some(index);
        }
    }
    fn refresh(&mut self) {
        self.mistakes = self.cells[..self.size * self.size]
            .iter()
            .enumerate()
            .filter(|(index, value)| **value != 0 && **value != self.solution[*index])
            .count() as u8;
        self.won = self.cells[..self.size * self.size] == self.solution[..self.size * self.size];
    }
    pub(super) fn set_selected(&mut self, value: u8) {
        if let Some(index) = self
            .selected
            .filter(|index| !self.is_given(*index) && value <= self.size as u8)
        {
            self.cells[index] = value;
            self.refresh();
        }
    }
    pub(super) fn clear_selected(&mut self) {
        if let Some(index) = self.selected.filter(|index| !self.is_given(*index)) {
            self.cells[index] = 0;
            self.refresh();
        }
    }
    pub(super) fn hint(&mut self) {
        let index = self
            .selected
            .filter(|index| !self.is_given(*index))
            .or_else(|| {
                (0..self.size * self.size)
                    .find(|index| !self.is_given(*index) && self.cells[*index] == 0)
            });
        if let Some(index) = index {
            self.cells[index] = self.solution[index];
            self.selected = Some(index);
            self.refresh();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct SudokuSave {
    version: u8,
    size: usize,
    variant: u32,
    cells: Vec<u8>,
    selected: Option<usize>,
}
impl SudokuSave {
    pub(super) fn from_game(game: &SudokuGame) -> Self {
        Self {
            version: 1,
            size: game.size,
            variant: game.variant,
            cells: game.cells[..game.size * game.size].to_vec(),
            selected: game.selected,
        }
    }
    fn into_game(self) -> Option<SudokuGame> {
        if self.version != 1
            || !matches!(self.size, 4 | 9)
            || self.cells.len() != self.size * self.size
            || self.selected.is_some_and(|index| index >= self.cells.len())
        {
            return None;
        }
        let mut game = SudokuGame::with_size(self.size, self.variant);
        for (index, value) in self.cells.into_iter().enumerate() {
            if value > self.size as u8 || (game.is_given(index) && value != game.puzzle[index]) {
                return None;
            }
            game.cells[index] = value;
        }
        game.selected = self.selected;
        game.refresh();
        Some(game)
    }
}

pub(super) fn load_sudoku() -> SudokuGame {
    fs::read(save_directory().join("sudoku-v1.json"))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<SudokuSave>(&bytes).ok())
        .and_then(SudokuSave::into_game)
        .unwrap_or_else(SudokuGame::new)
}

pub(super) fn retro_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open || s.help_open {
        return;
    }
    if s.mode == GameMode::Snake && keys.just_pressed(KeyCode::Space) {
        s.snake.toggle();
        s.ui_dirty = true;
        return;
    }
    let direction = if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        Some((0, -1))
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        Some((0, 1))
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        Some((-1, 0))
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        Some((1, 0))
    } else {
        None
    };
    let Some((dx, dy)) = direction else {
        return;
    };
    let changed = match s.mode {
        GameMode::Snake => s.snake.turn(dx, dy),
        GameMode::Classic2048 => s.classic_2048.move_board(dx, dy),
        GameMode::Klotski => s.klotski.move_selected(dx, dy),
        GameMode::Sokoban => s.sokoban.move_player(dx, dy),
        _ => false,
    };
    if changed {
        s.ui_dirty = true;
    }
}

// The board redraws as cells open. Raw hit testing keeps the first tap tied to
// its cell even while that redraw replaces the underlying UI button.
pub(super) fn mine_board_input(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cells: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut gate: ResMut<PressGate>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::Minesweeper || gate.0 {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let points: Vec<Vec2> = touches
        .iter_just_pressed()
        .map(|touch| touch.position() * window.scale_factor())
        .chain(
            mouse
                .just_pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
                .map(|point| point * window.scale_factor()),
        )
        .collect();
    for point in points {
        for (action, node, transform) in &cells {
            let Action::MineReveal(index) = *action else {
                continue;
            };
            if Rect::from_center_size(transform.translation().truncate(), node.size())
                .contains(point)
            {
                s.minesweeper.reveal(index);
                s.ui_dirty = true;
                gate.0 = true;
                return;
            }
        }
    }
}

pub(super) fn page(
    commands: &mut Commands,
    art: &Art,
    title: &str,
    subtitle: &str,
    reset: Action,
    body: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(22.)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.13, 0.27, 0.25)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(760.),
                    flex_shrink: 0.,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.),
                    padding: UiRect::axes(Val::Px(18.), Val::Px(10.)),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(GARDEN_WOOD),
                BorderColor(GARDEN_WOOD_EDGE),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|head| {
                label(head, art, title, 28., Color::srgb(1., 0.95, 0.79));
                label(head, art, subtitle, 14., Color::srgb(0.88, 0.81, 0.65));
            });
            body(root);
        });
    retro_shortcuts(commands, art, reset);
}

fn designed_page(
    commands: &mut Commands,
    art: &Art,
    title: &str,
    subtitle: &str,
    reset: Action,
    background: Color,
    header: Color,
    title_color: Color,
    subtitle_color: Color,
    body: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(22.)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.),
                ..default()
            },
            BackgroundColor(background),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(760.),
                    flex_shrink: 0.,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.),
                    padding: UiRect::axes(Val::Px(18.), Val::Px(10.)),
                    ..default()
                },
                BackgroundColor(header),
                BorderRadius::all(Val::Px(14.)),
            ))
            .with_children(|head| {
                label(head, art, title, 27., title_color);
                label(head, art, subtitle, 13., subtitle_color);
            });
            body(root);
        });
    retro_shortcuts(commands, art, reset);
}

fn retro_shortcuts(commands: &mut Commands, art: &Art, reset: Action) {
    commands
        .spawn((
            UiRoot,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(22.),
                top: Val::Px(22.),
                column_gap: Val::Px(8.),
                ..row()
            },
            ZIndex(20),
        ))
        .with_children(|p| {
            button(p, art, "重来", reset, 68., 36., GARDEN_PAPER, INK);
            button(
                p,
                art,
                "游戏库",
                Action::Library,
                82.,
                36.,
                GARDEN_GREEN,
                Color::WHITE,
            );
        });
}

pub(super) fn snake_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    compact: bool,
    width: f32,
    height: f32,
) {
    if compact {
        snake_ui_compact(commands, s, art, true);
    } else {
        snake_ui_desktop(commands, s, art, width, height);
    }
}

fn snake_ui_desktop(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let game = &s.snake;
    let sx = width / 1664.;
    let sy = height / 960.;
    let cell_w = (748. * sx - 6.) / SNAKE_W as f32;
    let cell_h = (740. * sy - 6.) / SNAKE_H as f32;
    let panel_w = 437. * sx;
    let panel_h = 750. * sy;
    let scale = (sx + sy) * 0.5;
    let parchment = Color::srgb(0.99, 0.985, 0.96);
    let ink = Color::srgb(0.12, 0.18, 0.15);
    let green = Color::srgb(0.20, 0.36, 0.27);
    let muted = Color::srgb(0.43, 0.48, 0.42);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            let mut backdrop = ImageNode::new(art.snake_reference.clone());
            backdrop.image_mode = NodeImageMode::Stretch;
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                backdrop,
                ZIndex(-1),
            ));

            // Rebuild the design's board as live, cell-by-cell game UI over
            // the illustration. The source design is only scenery; the snake,
            // fruit, score, and controls below are all driven by SnakeGame.
            let board_left = 307. * sx;
            let board_top = 120. * sy;
            let board_width = 748. * sx;
            let board_height = 740. * sy;
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(board_left),
                    top: Val::Px(board_top),
                    width: Val::Px(board_width),
                    height: Val::Px(board_height),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(sx),
                    border: UiRect::all(Val::Px(3. * scale)),
                    flex_shrink: 0.,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.84, 0.88, 0.80)),
                BorderColor(if game.over {
                    Color::srgb(0.79, 0.39, 0.32)
                } else {
                    Color::srgb(0.42, 0.53, 0.42)
                }),
                BorderRadius::all(Val::Px(8. * scale)),
                ZIndex(1),
            ))
            .with_children(|board| {
                for y in 0..SNAKE_H {
                    board
                        .spawn(Node {
                            height: Val::Px(cell_h),
                            column_gap: Val::Px(sx),
                            ..row()
                        })
                        .with_children(|line| {
                            for x in 0..SNAKE_W {
                                let shade = if (x + y) % 2 == 0 {
                                    Color::srgb(0.88, 0.91, 0.85)
                                } else {
                                    Color::srgb(0.85, 0.89, 0.82)
                                };
                                line.spawn((
                                    Node {
                                        width: Val::Px(cell_w),
                                        height: Val::Px(cell_h),
                                        ..default()
                                    },
                                    BackgroundColor(shade),
                                ));
                            }
                        });
                }
                let body = game.interpolated_body();
                for (index, point) in body.iter().enumerate().skip(1).rev() {
                    let shade = index as f32 / body.len().max(1) as f32;
                    board.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(point.x * (cell_w + sx) + sx),
                            top: Val::Px(point.y * (cell_h + sx) + sx),
                            width: Val::Px(cell_w - sx * 2.),
                            height: Val::Px(cell_h - sx * 2.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(
                            0.19 + shade * 0.035,
                            0.34 + shade * 0.035,
                            0.24 + shade * 0.02,
                        )),
                        BorderRadius::all(Val::Px(5. * scale)),
                        ZIndex(4),
                    ));
                }
                if let Some(head) = body.first() {
                    board
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(head.x * (cell_w + sx) + sx),
                                top: Val::Px(head.y * (cell_h + sx) + sx),
                                width: Val::Px(cell_w - sx * 2.),
                                height: Val::Px(cell_h - sx * 2.),
                                ..default()
                            },
                            BackgroundColor(if game.over {
                                Color::srgb(0.72, 0.39, 0.31)
                            } else {
                                Color::srgb(0.34, 0.48, 0.34)
                            }),
                            BorderRadius::all(Val::Px(5. * scale)),
                            ZIndex(5),
                        ))
                        .with_children(|head_node| {
                            let eyes = match game.direction {
                                (1, 0) => [
                                    (cell_w * 0.66, cell_h * 0.25),
                                    (cell_w * 0.66, cell_h * 0.64),
                                ],
                                (-1, 0) => [
                                    (cell_w * 0.25, cell_h * 0.25),
                                    (cell_w * 0.25, cell_h * 0.64),
                                ],
                                (0, -1) => [
                                    (cell_w * 0.25, cell_h * 0.25),
                                    (cell_w * 0.64, cell_h * 0.25),
                                ],
                                _ => [
                                    (cell_w * 0.25, cell_h * 0.64),
                                    (cell_w * 0.64, cell_h * 0.64),
                                ],
                            };
                            for (left, top) in eyes {
                                head_node.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(left),
                                        top: Val::Px(top),
                                        width: Val::Px(5. * scale),
                                        height: Val::Px(5. * scale),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.94, 0.93, 0.84)),
                                    BorderRadius::all(Val::Percent(50.)),
                                ));
                            }
                        });
                }
                let fruit_left = game.fruit.0 as f32 * (cell_w + sx) + cell_w * 0.28;
                let fruit_top = game.fruit.1 as f32 * (cell_h + sx) + cell_h * 0.28;
                board.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(fruit_left),
                        top: Val::Px(fruit_top),
                        width: Val::Px(cell_w * 0.44),
                        height: Val::Px(cell_h * 0.44),
                        ..default()
                    },
                    BackgroundColor(if game.golden_fruit {
                        Color::srgb(0.83, 0.58, 0.25)
                    } else {
                        Color::srgb(0.82, 0.29, 0.25)
                    }),
                    BorderRadius::all(Val::Percent(45.)),
                    ZIndex(6),
                ));
                if !game.running {
                    board
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(3. * scale),
                                top: Val::Px(3. * scale),
                                width: Val::Px(board_width - 12. * scale),
                                height: Val::Px(board_height - 12. * scale),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8. * scale),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.96, 0.96, 0.91, 0.88)),
                            ZIndex(8),
                        ))
                        .with_children(|overlay| {
                            label(
                                overlay,
                                art,
                                if game.over {
                                    "哎呀，撞到啦！"
                                } else {
                                    "准备好了吗？"
                                },
                                26. * scale,
                                ink,
                            );
                            label(
                                overlay,
                                art,
                                if game.over {
                                    format!("这次吃到 {} 个果实", game.fruits_eaten)
                                } else {
                                    "红果 +1 · 金果 +3".to_owned()
                                },
                                16. * scale,
                                muted,
                            );
                            label(
                                overlay,
                                art,
                                if game.over {
                                    "点击右侧按钮再来一局"
                                } else {
                                    "点开始，或按方向键出发"
                                },
                                14. * scale,
                                muted,
                            );
                        });
                }
            });

            let panel_left = 1080. * sx;
            let panel_top = 120. * sy;
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(panel_left),
                    top: Val::Px(panel_top),
                    width: Val::Px(panel_w),
                    height: Val::Px(panel_h),
                    padding: UiRect::all(Val::Px(38. * scale)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14. * scale),
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                BackgroundColor(parchment),
                BorderColor(Color::srgb(0.90, 0.88, 0.81)),
                BorderRadius::all(Val::Px(18. * scale)),
                ZIndex(2),
            ))
            .with_children(|panel| {
                label(panel, art, "本局得分", 22. * scale, ink);
                label(panel, art, game.score.to_string(), 76. * scale, green);
                panel
                    .spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Px(38. * scale),
                        align_items: AlignItems::Center,
                        ..row()
                    })
                    .with_children(|divider| {
                        divider.spawn((
                            Node {
                                flex_grow: 1.,
                                height: Val::Px(1. * scale),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.88, 0.86, 0.80)),
                        ));
                        divider.spawn((
                            Node {
                                width: Val::Px(30. * scale),
                                height: Val::Px(24. * scale),
                                margin: UiRect::horizontal(Val::Px(12. * scale)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.82, 0.29, 0.25)),
                            BorderRadius::all(Val::Percent(45.)),
                        ));
                        divider.spawn((
                            Node {
                                flex_grow: 1.,
                                height: Val::Px(1. * scale),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.88, 0.86, 0.80)),
                        ));
                    });
                panel
                    .spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Px(54. * scale),
                            padding: UiRect::horizontal(Val::Px(16. * scale)),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..row()
                        },
                        BackgroundColor(Color::srgb(0.96, 0.95, 0.91)),
                        BorderRadius::all(Val::Px(12. * scale)),
                    ))
                    .with_children(|best| {
                        label(best, art, "♛  最高得分", 18. * scale, muted);
                        label(best, art, game.best.to_string(), 19. * scale, ink);
                    });
                button(
                    panel,
                    art,
                    if game.over {
                        "再来一局  ›"
                    } else if game.running {
                        "暂停游戏 Ⅱ"
                    } else if game.started {
                        "继续游戏  ›"
                    } else {
                        "开始游戏  ›"
                    },
                    Action::SnakeToggle,
                    panel_w - 76. * scale,
                    88. * scale,
                    green,
                    Color::WHITE,
                );
                panel
                    .spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Px(1. * scale),
                        ..default()
                    })
                    .insert(BackgroundColor(Color::srgb(0.90, 0.88, 0.82)));
                label(panel, art, "操作说明", 20. * scale, ink);
                snake_dpad_scaled(panel, art, scale);
                label(panel, art, "使用键盘方向键控制蛇的移动", 13. * scale, muted);
            });

            // Functional gear and library actions sit over the static header
            // locations from the concept art.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(112. * sx),
                    top: Val::Px(24. * sy),
                    ..row()
                },
                ZIndex(10),
            ))
            .with_children(|gear| {
                button(
                    gear,
                    art,
                    "⚙",
                    Action::Help,
                    54. * sx,
                    54. * sy,
                    parchment,
                    ink,
                )
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(100. * sx),
                    top: Val::Px(24. * sy),
                    ..row()
                },
                ZIndex(10),
            ))
            .with_children(|back| {
                button(
                    back,
                    art,
                    "游戏库",
                    Action::Library,
                    92. * sx,
                    42. * sy,
                    parchment,
                    muted,
                )
            });
        });
}

fn snake_ui_compact(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.snake;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(22.)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.98, 0.97, 0.93)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(850.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_shrink: 0.,
                ..row()
            })
            .with_children(|header| {
                header
                    .spawn(Node {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.),
                        ..row()
                    })
                    .with_children(|title| {
                        title
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(1.),
                                ..default()
                            })
                            .with_children(|text| {
                                label(text, art, "贪吃蛇", 28., Color::srgb(0.13, 0.20, 0.17));
                                label(
                                    text,
                                    art,
                                    "吃果实，转弯，刷新你的纪录",
                                    13.,
                                    Color::srgb(0.43, 0.49, 0.44),
                                );
                            });
                    });
            });

            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(850.),
                flex_grow: 1.,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: if compact {
                    FlexDirection::Column
                } else {
                    FlexDirection::Row
                },
                column_gap: Val::Px(20.),
                row_gap: Val::Px(14.),
                ..default()
            })
            .with_children(|main| {
                main.spawn((
                    Node {
                        width: Val::Px(438.),
                        height: Val::Px(438.),
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(1.),
                        border: UiRect::all(Val::Px(3.)),
                        flex_shrink: 0.,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.84, 0.88, 0.80)),
                    BorderColor(if g.over {
                        Color::srgb(0.79, 0.39, 0.32)
                    } else {
                        Color::srgb(0.42, 0.53, 0.42)
                    }),
                    BorderRadius::all(Val::Px(8.)),
                ))
                .with_children(|board| {
                    for y in 0..SNAKE_H {
                        board
                            .spawn(Node {
                                height: Val::Px(35.),
                                column_gap: Val::Px(1.),
                                ..row()
                            })
                            .with_children(|line| {
                                for x in 0..SNAKE_W {
                                    let shade = if (x + y) % 2 == 0 {
                                        Color::srgb(0.88, 0.91, 0.85)
                                    } else {
                                        Color::srgb(0.85, 0.89, 0.82)
                                    };
                                    line.spawn((
                                        Node {
                                            width: Val::Px(35.),
                                            height: Val::Px(35.),
                                            ..default()
                                        },
                                        BackgroundColor(shade),
                                    ));
                                }
                            });
                    }
                    let body = g.interpolated_body();
                    for (index, point) in body.iter().enumerate().skip(1).rev() {
                        let shade = index as f32 / body.len().max(1) as f32;
                        board.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(point.x * 36. + 2.),
                                top: Val::Px(point.y * 36. + 2.),
                                width: Val::Px(32.),
                                height: Val::Px(32.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(
                                0.19 + shade * 0.035,
                                0.34 + shade * 0.035,
                                0.24 + shade * 0.02,
                            )),
                            BorderRadius::all(Val::Px(8.)),
                            ZIndex(4),
                        ));
                    }
                    if let Some(head) = body.first() {
                        board
                            .spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(head.x * 36. + 2.),
                                    top: Val::Px(head.y * 36. + 2.),
                                    width: Val::Px(32.),
                                    height: Val::Px(32.),
                                    ..default()
                                },
                                BackgroundColor(if g.over {
                                    Color::srgb(0.72, 0.39, 0.31)
                                } else {
                                    Color::srgb(0.34, 0.48, 0.34)
                                }),
                                BorderRadius::all(Val::Px(9.)),
                                ZIndex(5),
                            ))
                            .with_children(|head_node| {
                                let eyes = match g.direction {
                                    (1, 0) => [(21., 8.), (21., 20.)],
                                    (-1, 0) => [(8., 8.), (8., 20.)],
                                    (0, -1) => [(8., 8.), (20., 8.)],
                                    _ => [(8., 20.), (20., 20.)],
                                };
                                for (left, top) in eyes {
                                    head_node.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(left),
                                            top: Val::Px(top),
                                            width: Val::Px(4.),
                                            height: Val::Px(4.),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.94, 0.93, 0.84)),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ));
                                }
                            });
                    }
                    let fruit_center =
                        Vec2::new(g.fruit.0 as f32 * 36. + 18., g.fruit.1 as f32 * 36. + 18.);
                    let fruit_size = 20. + if g.eating_flash > 0. { 4. } else { 0. };
                    board.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(fruit_center.x - fruit_size * 0.5),
                            top: Val::Px(fruit_center.y - fruit_size * 0.5),
                            width: Val::Px(fruit_size),
                            height: Val::Px(fruit_size),
                            ..default()
                        },
                        BackgroundColor(if g.golden_fruit {
                            Color::srgb(0.83, 0.58, 0.25)
                        } else {
                            Color::srgb(0.82, 0.29, 0.25)
                        }),
                        BorderRadius::all(Val::Percent(50.)),
                        ZIndex(6),
                    ));
                    if !g.running {
                        board
                            .spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(3.),
                                    top: Val::Px(3.),
                                    width: Val::Px(426.),
                                    height: Val::Px(426.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(7.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.96, 0.96, 0.91, 0.88)),
                                ZIndex(8),
                            ))
                            .with_children(|overlay| {
                                if g.over {
                                    label(
                                        overlay,
                                        art,
                                        "哎呀，撞到啦！",
                                        25.,
                                        Color::srgb(0.17, 0.23, 0.19),
                                    );
                                    label(
                                        overlay,
                                        art,
                                        format!("这次吃到 {} 个果实", g.fruits_eaten),
                                        16.,
                                        Color::srgb(0.39, 0.47, 0.40),
                                    );
                                    label(
                                        overlay,
                                        art,
                                        "点击右侧按钮再来一局",
                                        14.,
                                        Color::srgb(0.65, 0.39, 0.25),
                                    );
                                } else {
                                    label(
                                        overlay,
                                        art,
                                        "准备好了吗？",
                                        24.,
                                        Color::srgb(0.17, 0.23, 0.19),
                                    );
                                    label(
                                        overlay,
                                        art,
                                        "红果 +1 · 金果 +3",
                                        16.,
                                        Color::srgb(0.65, 0.43, 0.24),
                                    );
                                    label(
                                        overlay,
                                        art,
                                        "点开始，或按方向键出发",
                                        14.,
                                        Color::srgb(0.39, 0.47, 0.40),
                                    );
                                }
                            });
                    }
                });

                main.spawn((
                    Node {
                        width: Val::Px(270.),
                        min_height: Val::Px(438.),
                        padding: UiRect::all(Val::Px(18.)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.),
                        align_items: AlignItems::Stretch,
                        flex_shrink: 0.,
                        border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1., 0.995, 0.97)),
                    BorderColor(Color::srgb(0.90, 0.88, 0.81)),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|panel| {
                    label(panel, art, "本局得分", 16., Color::srgb(0.29, 0.36, 0.30));
                    label(
                        panel,
                        art,
                        g.score.to_string(),
                        54.,
                        Color::srgb(0.20, 0.36, 0.27),
                    );
                    panel.spawn((
                        Node {
                            height: Val::Px(1.),
                            width: Val::Percent(100.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.90, 0.88, 0.82)),
                    ));
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceBetween,
                            ..row()
                        })
                        .with_children(|best| {
                            label(best, art, "最高纪录", 14., Color::srgb(0.43, 0.48, 0.42));
                            label(
                                best,
                                art,
                                g.best.to_string(),
                                16.,
                                Color::srgb(0.18, 0.25, 0.20),
                            );
                        });
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceBetween,
                            ..row()
                        })
                        .with_children(|fruit| {
                            label(fruit, art, "金果进度", 14., Color::srgb(0.43, 0.48, 0.42));
                            label(
                                fruit,
                                art,
                                format!("{}/4", g.fruits_eaten % 4),
                                14.,
                                Color::srgb(0.65, 0.43, 0.24),
                            );
                        });
                    button(
                        panel,
                        art,
                        if g.over {
                            "再来一局"
                        } else if g.running {
                            "暂停游戏"
                        } else if g.started {
                            "继续游戏"
                        } else {
                            "开始游戏"
                        },
                        Action::SnakeToggle,
                        228.,
                        48.,
                        if g.running {
                            Color::srgb(0.38, 0.48, 0.39)
                        } else {
                            Color::srgb(0.26, 0.41, 0.31)
                        },
                        Color::WHITE,
                    );
                    label(panel, art, "操作说明", 15., Color::srgb(0.25, 0.33, 0.27));
                    label(
                        panel,
                        art,
                        "方向键 / WASD 控制移动",
                        12.,
                        Color::srgb(0.47, 0.51, 0.45),
                    );
                    label(
                        panel,
                        art,
                        "空格暂停 · 红果 +1 · 金果 +3",
                        11.,
                        Color::srgb(0.47, 0.51, 0.45),
                    );
                    snake_dpad(panel, art);
                    label(
                        panel,
                        art,
                        if g.over {
                            "这局结束啦，挑战更高纪录吧"
                        } else if g.running {
                            "方向键转弯，别撞到自己"
                        } else if g.started {
                            "已暂停，准备好再继续"
                        } else {
                            "不能掉头撞向自己"
                        },
                        11.,
                        Color::srgb(0.47, 0.51, 0.45),
                    );
                });
            });
        });
    retro_shortcuts(commands, art, Action::SnakeReset);
}

fn snake_dpad(parent: &mut ChildSpawnerCommands, art: &Art) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.),
            ..default()
        })
        .with_children(|pad| {
            let key_bg = Color::srgb(0.89, 0.91, 0.85);
            let key_fg = Color::srgb(0.22, 0.32, 0.24);
            button(
                pad,
                art,
                "↑",
                Action::SnakeMove(0, -1),
                52.,
                36.,
                key_bg,
                key_fg,
            );
            pad.spawn(Node {
                column_gap: Val::Px(4.),
                ..row()
            })
            .with_children(|row| {
                button(
                    row,
                    art,
                    "←",
                    Action::SnakeMove(-1, 0),
                    52.,
                    36.,
                    key_bg,
                    key_fg,
                );
                button(
                    row,
                    art,
                    "↓",
                    Action::SnakeMove(0, 1),
                    52.,
                    36.,
                    key_bg,
                    key_fg,
                );
                button(
                    row,
                    art,
                    "→",
                    Action::SnakeMove(1, 0),
                    52.,
                    36.,
                    key_bg,
                    key_fg,
                );
            });
        });
}

fn snake_dpad_scaled(parent: &mut ChildSpawnerCommands, art: &Art, scale: f32) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4. * scale),
            ..default()
        })
        .with_children(|pad| {
            let key_bg = Color::srgb(0.89, 0.91, 0.85);
            let key_fg = Color::srgb(0.22, 0.32, 0.24);
            button(
                pad,
                art,
                "↑",
                Action::SnakeMove(0, -1),
                52. * scale,
                36. * scale,
                key_bg,
                key_fg,
            );
            pad.spawn(Node {
                column_gap: Val::Px(4. * scale),
                ..row()
            })
            .with_children(|keys| {
                button(
                    keys,
                    art,
                    "←",
                    Action::SnakeMove(-1, 0),
                    52. * scale,
                    36. * scale,
                    key_bg,
                    key_fg,
                );
                button(
                    keys,
                    art,
                    "↓",
                    Action::SnakeMove(0, 1),
                    52. * scale,
                    36. * scale,
                    key_bg,
                    key_fg,
                );
                button(
                    keys,
                    art,
                    "→",
                    Action::SnakeMove(1, 0),
                    52. * scale,
                    36. * scale,
                    key_bg,
                    key_fg,
                );
            });
        });
}

const MINES_DESIGN: Vec2 = Vec2::new(1586., 992.);
const MINES_GRID_ORIGIN: Vec2 = Vec2::new(357., 188.);
const MINES_CELL_SIZE: Vec2 = Vec2::new(84.5, 85.5);

pub(super) fn minesweeper_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    width: f32,
    height: f32,
) {
    let g = &s.minesweeper;
    let scale = (width / MINES_DESIGN.x).min(height / MINES_DESIGN.y);
    let flags = g.flagged.iter().filter(|flag| **flag).count();
    let opened = (0..64).filter(|&i| g.revealed[i] && !g.mines[i]).count();
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.20, 0.29, 0.16)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(MINES_DESIGN.x * scale),
                height: Val::Px(MINES_DESIGN.y * scale),
                flex_shrink: 0.,
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|scene| {
                scene.spawn((
                    ImageNode::new(art.minesweeper_plate.clone()),
                    mines_node(Vec2::ZERO, MINES_DESIGN, scale),
                ));
                mines_text(
                    scene,
                    art,
                    Vec2::new(894., 70.),
                    Vec2::new(97., 59.),
                    format!("{flags}/10"),
                    35.,
                    Color::srgb(0.81, 0.23, 0.055),
                    scale,
                );
                mines_text(
                    scene,
                    art,
                    Vec2::new(1205., 70.),
                    Vec2::new(105., 59.),
                    format!("{opened}/54"),
                    35.,
                    Color::srgb(0.16, 0.32, 0.13),
                    scale,
                );
                for index in 0..64 {
                    let origin = mines_cell_origin(index);
                    let rect = if g.flagged[index] && !g.revealed[index] {
                        Rect::new(780., 444., 866., 531.)
                    } else if g.revealed[index] {
                        Rect::new(610., 358., 696., 445.)
                    } else {
                        Rect::new(357., 188., 442., 274.)
                    };
                    let atlas = if g.flagged[index] && !g.revealed[index] {
                        art.minesweeper_atlas.clone()
                    } else {
                        art.minesweeper_plate.clone()
                    };
                    mines_sample(scene, atlas, rect, origin, MINES_CELL_SIZE, scale, 1);
                    if g.revealed[index] {
                        if g.mines[index] {
                            mines_sample(
                                scene,
                                art.minesweeper_atlas.clone(),
                                Rect::new(503., 74., 556., 128.),
                                origin + Vec2::new(15., 15.),
                                Vec2::new(54., 55.),
                                scale,
                                3,
                            );
                        } else {
                            let n = g.around(index);
                            if n > 0 {
                                let color = match n {
                                    1 => Color::srgb(0.15, 0.33, 0.78),
                                    2 => Color::srgb(0.05, 0.40, 0.16),
                                    3 => Color::srgb(0.78, 0.17, 0.10),
                                    4 => Color::srgb(0.35, 0.20, 0.58),
                                    _ => Color::srgb(0.30, 0.16, 0.07),
                                };
                                mines_text(
                                    scene,
                                    art,
                                    origin,
                                    MINES_CELL_SIZE,
                                    n.to_string(),
                                    44.,
                                    color,
                                    scale,
                                );
                            }
                        }
                    }
                    mines_hit(
                        scene,
                        Action::MineReveal(index),
                        origin,
                        MINES_CELL_SIZE,
                        scale,
                    );
                }
                mines_text(
                    scene,
                    art,
                    Vec2::new(1204., 653.),
                    Vec2::new(186., 74.),
                    if g.flag_mode {
                        "标记模式"
                    } else {
                        "探索模式"
                    },
                    40.,
                    Color::srgb(1., 0.94, 0.73),
                    scale,
                );
                // The actual approved toggle thumb moves along its track with the mode.
                let mut track = ImageNode::new(art.minesweeper_atlas.clone());
                track.rect = Some(Rect::new(1395., 659., 1434., 724.));
                scene.spawn((
                    track,
                    mines_node(Vec2::new(1393., 654.), Vec2::new(108., 73.), scale),
                    BorderRadius::all(Val::Percent(50.)),
                    ZIndex(2),
                ));
                let mut thumb = ImageNode::new(art.minesweeper_atlas.clone());
                thumb.rect = Some(Rect::new(1439., 660., 1496., 723.));
                scene.spawn((
                    thumb,
                    mines_node(
                        Vec2::new(if g.flag_mode { 1398. } else { 1439. }, 659.),
                        Vec2::new(58., 64.),
                        scale,
                    ),
                    BorderRadius::all(Val::Percent(50.)),
                    ZIndex(3),
                ));
                mines_hit(
                    scene,
                    Action::MineToggleFlag,
                    Vec2::new(1074., 626.),
                    Vec2::new(456., 131.),
                    scale,
                );
                mines_hit(
                    scene,
                    Action::MineReset,
                    Vec2::new(1173., 772.),
                    Vec2::new(249., 81.),
                    scale,
                );
                mines_hit(
                    scene,
                    Action::Library,
                    Vec2::new(43., 43.),
                    Vec2::new(104., 104.),
                    scale,
                );
                mines_text(
                    scene,
                    art,
                    Vec2::new(577., 900.),
                    Vec2::new(254., 45.),
                    if g.lost {
                        "踩到地雷，点击重开"
                    } else if g.won {
                        "所有安全格都找到了！"
                    } else {
                        "首次翻开不会踩雷"
                    },
                    if g.lost || g.won { 23. } else { 25. },
                    Color::srgb(0.43, 0.25, 0.11),
                    scale,
                );
            });
        });
}

fn mines_cell_origin(index: usize) -> Vec2 {
    MINES_GRID_ORIGIN + MINES_CELL_SIZE * Vec2::new((index % 8) as f32, (index / 8) as f32)
}

fn mines_node(origin: Vec2, size: Vec2, scale: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(origin.x * scale),
        top: Val::Px(origin.y * scale),
        width: Val::Px(size.x * scale),
        height: Val::Px(size.y * scale),
        ..default()
    }
}

fn mines_hit(
    parent: &mut ChildSpawnerCommands,
    action: Action,
    origin: Vec2,
    size: Vec2,
    scale: f32,
) {
    parent.spawn((
        Button,
        action,
        mines_node(origin, size, scale),
        BackgroundColor(Color::NONE),
        ZIndex(10),
    ));
}

fn mines_sample(
    parent: &mut ChildSpawnerCommands,
    handle: Handle<Image>,
    rect: Rect,
    origin: Vec2,
    size: Vec2,
    scale: f32,
    layer: i32,
) {
    let mut image = ImageNode::new(handle);
    image.rect = Some(rect);
    parent.spawn((image, mines_node(origin, size, scale), ZIndex(layer)));
}

fn mines_text(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    origin: Vec2,
    size: Vec2,
    text: impl Into<String>,
    font: f32,
    color: Color,
    scale: f32,
) {
    let text = text.into();
    // The approved digits and button labels have a heavier weight than the shared font.
    for dx in [-0.35, 0.35] {
        parent
            .spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..mines_node(origin + Vec2::new(dx, 0.), size, scale)
                },
                ZIndex(5),
            ))
            .with_children(|container| {
                label(container, art, text.clone(), font * scale, color);
            });
    }
}

#[derive(Component)]
pub(super) struct ClassicBoard;

fn classic_tile_color(value: u16) -> Color {
    match value {
        0 => Color::srgb(0.92, 0.93, 0.89),
        2 => Color::srgb(0.98, 0.97, 0.93),
        4 => Color::srgb(0.90, 0.94, 0.85),
        8 => Color::srgb(0.80, 0.90, 0.78),
        16 => Color::srgb(0.68, 0.83, 0.69),
        32 => Color::srgb(0.54, 0.75, 0.59),
        64 => Color::srgb(0.39, 0.65, 0.49),
        128 => Color::srgb(0.79, 0.86, 0.69),
        256 => Color::srgb(0.67, 0.80, 0.58),
        512 => Color::srgb(0.55, 0.73, 0.48),
        1024 => Color::srgb(0.40, 0.62, 0.40),
        _ => Color::srgb(0.27, 0.47, 0.34),
    }
}

pub(super) fn classic_2048_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    viewport_width: f32,
    viewport_height: f32,
) {
    let g = &s.classic_2048;
    // The UI scale normalizes portrait phones to roughly 540 logical pixels.
    // Size the board from that usable width instead of a small fixed preset.
    let side = (viewport_width - 48.)
        .min(viewport_height - 210.)
        .min(560.)
        .max(280.);
    let cell = (side - 40.) / 4.;
    designed_page(
        commands,
        art,
        "经典 2048",
        "滑动棋盘合并数字",
        Action::ClassicReset,
        Color::srgb(0.97, 0.97, 0.94),
        Color::srgb(1., 1., 0.99),
        Color::srgb(0.13, 0.20, 0.17),
        Color::srgb(0.45, 0.49, 0.45),
        |root| {
            root.spawn(Node {
                width: Val::Px(side),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|bar| {
                bar.spawn((
                    Node {
                        width: Val::Px((side - 8.) / 2.),
                        height: Val::Px(68.),
                        padding: UiRect::all(Val::Px(6.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.91, 0.93, 0.88)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|box_| {
                    label(box_, art, "得分", 13., Color::srgb(0.43, 0.49, 0.44));
                    label(
                        box_,
                        art,
                        g.score.to_string(),
                        24.,
                        Color::srgb(0.16, 0.26, 0.19),
                    );
                });
                bar.spawn((
                    Node {
                        width: Val::Px((side - 8.) / 2.),
                        height: Val::Px(68.),
                        padding: UiRect::all(Val::Px(6.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.91, 0.93, 0.88)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|box_| {
                    label(box_, art, "最高分", 13., Color::srgb(0.43, 0.49, 0.44));
                    label(
                        box_,
                        art,
                        g.best.to_string(),
                        24.,
                        Color::srgb(0.32, 0.48, 0.34),
                    );
                });
            });
            root.spawn((
                ClassicBoard,
                Node {
                    width: Val::Px(side),
                    height: Val::Px(side),
                    padding: UiRect::all(Val::Px(8.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.84, 0.87, 0.81)),
                BorderRadius::all(Val::Px(12.)),
            ))
            .with_children(|board| {
                for y in 0..4 {
                    board
                        .spawn(Node {
                            height: Val::Px(cell),
                            column_gap: Val::Px(8.),
                            ..row()
                        })
                        .with_children(|line| {
                            for x in 0..4 {
                                let value = g.cells[y * 4 + x];
                                line.spawn((
                                    Node {
                                        width: Val::Px(cell),
                                        height: Val::Px(cell),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(classic_tile_color(value)),
                                    BorderRadius::all(Val::Px(14.)),
                                ))
                                .with_children(|tile| {
                                    if value != 0 {
                                        let size = match value {
                                            0..=64 => cell * 0.38,
                                            128..=512 => cell * 0.33,
                                            _ => cell * 0.27,
                                        };
                                        label(
                                            tile,
                                            art,
                                            value.to_string(),
                                            size,
                                            if value <= 256 {
                                                Color::srgb(0.22, 0.29, 0.24)
                                            } else {
                                                Color::WHITE
                                            },
                                        );
                                    }
                                });
                            }
                        });
                }
            });
            let status = if g.over {
                "没有可移动的方块了，再来一局吧"
            } else if g.won {
                "合成 2048！继续挑战高分"
            } else {
                "目标 2048  ·  合并得分"
            };
            label(
                root,
                art,
                status,
                16.,
                if g.over {
                    Color::srgb(0.68, 0.36, 0.29)
                } else if g.won {
                    Color::srgb(0.32, 0.48, 0.34)
                } else {
                    Color::srgb(0.45, 0.49, 0.45)
                },
            );
        },
    );
}

#[derive(Resource, Default)]
pub(super) struct ClassicSwipe {
    start: Option<(Option<u64>, Vec2)>,
}

pub(super) fn classic_swipe_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    boards: Query<(&ComputedNode, &GlobalTransform), With<ClassicBoard>>,
    mut swipe: ResMut<ClassicSwipe>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::Classic2048 || s.library_open || s.help_open {
        swipe.start = None;
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((node, transform)) = boards.single() else {
        return;
    };
    let bounds = Rect::from_center_size(transform.translation().truncate(), node.size());
    let scale = window.scale_factor();
    if swipe.start.is_none() {
        if let Some(touch) = touches.iter_just_pressed().next() {
            let point = touch.position() * scale;
            if bounds.contains(point) {
                swipe.start = Some((Some(touch.id()), point));
            }
        } else if mouse.just_pressed(MouseButton::Left) {
            if let Some(point) = window.cursor_position().map(|p| p * scale) {
                if bounds.contains(point) {
                    swipe.start = Some((None, point));
                }
            }
        }
    }
    let Some((id, start)) = swipe.start else {
        return;
    };
    if id.is_some()
        && touches
            .iter_just_canceled()
            .any(|touch| Some(touch.id()) == id)
    {
        swipe.start = None;
        return;
    }
    let end = if let Some(id) = id {
        touches
            .iter_just_released()
            .find(|touch| touch.id() == id)
            .map(|touch| touch.position() * scale)
    } else if mouse.just_released(MouseButton::Left) {
        window.cursor_position().map(|p| p * scale)
    } else {
        None
    };
    if let Some(end) = end {
        swipe.start = None;
        let delta = end - start;
        if delta.length() < 30. * scale {
            return;
        }
        let direction = if delta.x.abs() > delta.y.abs() {
            (if delta.x > 0. { 1 } else { -1 }, 0)
        } else {
            (0, if delta.y > 0. { 1 } else { -1 })
        };
        if s.classic_2048.move_board(direction.0, direction.1) {
            s.ui_dirty = true;
        }
    }
}

fn klotski_piece(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    index: usize,
    piece: &KlotskiPiece,
    selected: bool,
) {
    let color = match piece.label {
        "曹操" => Color::srgb(0.68, 0.27, 0.23),
        "关羽" | "张飞" | "马超" | "黄忠" => Color::srgb(0.49, 0.58, 0.46),
        "赵云" => Color::srgb(0.75, 0.61, 0.39),
        _ => piece.color,
    };
    parent
        .spawn((
            Button,
            ButtonTone(color),
            Action::KlotskiSelect(index),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(piece.x as f32 * 90. + 4.),
                top: Val::Px(piece.y as f32 * 90. + 4.),
                width: Val::Px(piece.w as f32 * 90. - 8.),
                height: Val::Px(piece.h as f32 * 90. - 8.),
                min_height: Val::Px(piece.h as f32 * 90. - 8.),
                border: UiRect::all(Val::Px(if selected { 4. } else { 2. })),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(color),
            BorderColor(if selected {
                Color::srgb(0.75, 0.32, 0.24)
            } else {
                Color::srgb(0.92, 0.88, 0.77)
            }),
            BorderRadius::all(Val::Px(8.)),
        ))
        .with_children(|tile| {
            label(
                tile,
                art,
                piece.label,
                if piece.label == "兵" { 20. } else { 18. },
                if piece.label == "曹操" {
                    Color::srgb(0.98, 0.93, 0.82)
                } else {
                    Color::srgb(0.24, 0.27, 0.21)
                },
            )
        });
}

pub(super) fn klotski_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.klotski;
    designed_page(
        commands,
        art,
        "华容道",
        "移开棋子，为曹操打开出口",
        Action::KlotskiReset,
        Color::srgb(0.91, 0.88, 0.80),
        Color::srgb(0.28, 0.25, 0.20),
        Color::srgb(0.94, 0.89, 0.77),
        Color::srgb(0.78, 0.73, 0.62),
        |root| {
            root.spawn(Node {
                flex_direction: if compact {
                    FlexDirection::Column
                } else {
                    FlexDirection::Row
                },
                column_gap: Val::Px(20.),
                row_gap: Val::Px(14.),
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|layout| {
                layout
                    .spawn((
                        Node {
                            width: Val::Px(360.),
                            height: Val::Px(450.),
                            position_type: PositionType::Relative,
                            border: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.90, 0.87, 0.78)),
                        BorderColor(Color::srgb(0.45, 0.39, 0.30)),
                        BorderRadius::all(Val::Px(10.)),
                    ))
                    .with_children(|board| {
                        board.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(92.),
                                bottom: Val::Px(-4.),
                                width: Val::Px(176.),
                                height: Val::Px(12.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.68, 0.27, 0.23)),
                        ));
                        for (i, piece) in g.pieces.iter().enumerate() {
                            klotski_piece(board, art, i, piece, i == g.selected);
                        }
                    });
                layout
                    .spawn((
                        Node {
                            width: Val::Px(230.),
                            min_height: Val::Px(if compact { 300. } else { 390. }),
                            padding: UiRect::all(Val::Px(18.)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(14.),
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.97, 0.95, 0.88)),
                        BorderColor(Color::srgb(0.81, 0.76, 0.65)),
                        BorderRadius::all(Val::Px(14.)),
                    ))
                    .with_children(|panel| {
                        label(
                            panel,
                            art,
                            "出口在棋盘下方",
                            14.,
                            Color::srgb(0.50, 0.43, 0.34),
                        );
                        label(
                            panel,
                            art,
                            if g.won {
                                "成功出阵！".to_owned()
                            } else {
                                format!("当前棋子：{}", g.pieces[g.selected].label)
                            },
                            19.,
                            Color::srgb(0.37, 0.31, 0.25),
                        );
                        panel.spawn((
                            Node {
                                height: Val::Px(1.),
                                width: Val::Percent(100.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.83, 0.78, 0.67)),
                        ));
                        label(
                            panel,
                            art,
                            "点选棋子，再按方向移动",
                            13.,
                            Color::srgb(0.50, 0.45, 0.37),
                        );
                        themed_dpad(
                            panel,
                            art,
                            Action::KlotskiMove(0, -1),
                            Action::KlotskiMove(0, 1),
                            Action::KlotskiMove(-1, 0),
                            Action::KlotskiMove(1, 0),
                            Color::srgb(0.86, 0.83, 0.74),
                            Color::srgb(0.30, 0.33, 0.26),
                        );
                    });
            });
        },
    );
}

fn themed_dpad(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    up: Action,
    down: Action,
    left: Action,
    right: Action,
    background: Color,
    foreground: Color,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(5.),
            ..default()
        })
        .with_children(|pad| {
            button(pad, art, "↑", up, 58., 42., background, foreground);
            pad.spawn(Node {
                column_gap: Val::Px(5.),
                ..row()
            })
            .with_children(|line| {
                button(line, art, "←", left, 58., 42., background, foreground);
                button(line, art, "↓", down, 58., 42., background, foreground);
                button(line, art, "→", right, 58., 42., background, foreground);
            });
        });
}

pub(super) fn sudoku_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.sudoku;
    let size = g.size;
    let box_size = if size == 4 { 2 } else { 3 };
    let tile =
        (400. - 8. - (size - 1) as f32 * 2. - (size / box_size - 1) as f32 * 4.) / size as f32;
    let filled = g.cells[..size * size]
        .iter()
        .filter(|value| **value != 0)
        .count();
    let status = if g.won {
        "全部填对了，继续挑战下一题".to_owned()
    } else if g.mistakes > 0 {
        format!("有 {} 格需要再看看", g.mistakes)
    } else {
        format!("已填 {filled}/{} 格 · 第 {} 题", size * size, g.variant + 1)
    };
    designed_page(
        commands,
        art,
        "数独乐园",
        "静下心来，享受数字推理",
        Action::SudokuReset,
        Color::srgb(0.97, 0.96, 0.92),
        Color::srgb(1., 0.995, 0.97),
        Color::srgb(0.14, 0.21, 0.29),
        Color::srgb(0.46, 0.49, 0.51),
        |root| {
            root.spawn(Node {
                flex_direction: if compact {
                    FlexDirection::Column
                } else {
                    FlexDirection::Row
                },
                column_gap: Val::Px(18.),
                row_gap: Val::Px(if compact { 8. } else { 12. }),
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|layout| {
                layout
                    .spawn((
                        Node {
                            width: Val::Px(400.),
                            height: Val::Px(400.),
                            padding: UiRect::all(Val::Px(4.)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(2.),
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.32, 0.39, 0.44)),
                        BorderColor(Color::srgb(0.32, 0.39, 0.44)),
                        BorderRadius::all(Val::Px(5.)),
                    ))
                    .with_children(|board| {
                        for y in 0..size {
                            board
                                .spawn(Node {
                                    height: Val::Px(tile),
                                    column_gap: Val::Px(2.),
                                    margin: UiRect::bottom(Val::Px(
                                        if (y + 1) % box_size == 0 && y + 1 < size {
                                            4.
                                        } else {
                                            0.
                                        },
                                    )),
                                    ..row()
                                })
                                .with_children(|line| {
                                    for x in 0..size {
                                        let index = y * size + x;
                                        let value = g.cells[index];
                                        let wrong = value != 0 && value != g.solution[index];
                                        let related = g.selected.is_some_and(|selected| {
                                            let sx = selected % size;
                                            let sy = selected / size;
                                            sx == x
                                                || sy == y
                                                || (sx / box_size == x / box_size
                                                    && sy / box_size == y / box_size)
                                        });
                                        let background = if wrong {
                                            Color::srgb(0.93, 0.78, 0.73)
                                        } else if g.selected == Some(index) {
                                            Color::srgb(0.92, 0.77, 0.69)
                                        } else if related {
                                            Color::srgb(0.88, 0.91, 0.87)
                                        } else if g.is_given(index) {
                                            Color::srgb(0.96, 0.95, 0.91)
                                        } else {
                                            Color::srgb(0.99, 0.985, 0.96)
                                        };
                                        line.spawn((
                                            Button,
                                            ButtonTone(background),
                                            Action::SudokuSelect(index),
                                            Node {
                                                width: Val::Px(tile),
                                                height: Val::Px(tile),
                                                margin: UiRect::right(Val::Px(
                                                    if (x + 1) % box_size == 0 && x + 1 < size {
                                                        4.
                                                    } else {
                                                        0.
                                                    },
                                                )),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                border: UiRect::all(Val::Px(
                                                    if g.selected == Some(index) { 2. } else { 0. },
                                                )),
                                                ..default()
                                            },
                                            BackgroundColor(background),
                                            BorderColor(Color::srgb(0.49, 0.58, 0.63)),
                                            BorderRadius::all(Val::Px(if size == 4 {
                                                7.
                                            } else {
                                                3.
                                            })),
                                        ))
                                        .with_children(
                                            |cell| {
                                                if value != 0 {
                                                    let color = if wrong {
                                                        Color::srgb(0.67, 0.29, 0.25)
                                                    } else if g.is_given(index) {
                                                        Color::srgb(0.18, 0.26, 0.33)
                                                    } else {
                                                        Color::srgb(0.30, 0.48, 0.62)
                                                    };
                                                    label(
                                                        cell,
                                                        art,
                                                        value.to_string(),
                                                        if size == 4 { 42. } else { 23. },
                                                        color,
                                                    );
                                                }
                                            },
                                        );
                                    }
                                });
                        }
                    });
                layout
                    .spawn((
                        Node {
                            width: Val::Px(232.),
                            min_height: Val::Px(if compact { 300. } else { 400. }),
                            padding: UiRect::all(Val::Px(if compact { 12. } else { 16. })),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(if compact { 8. } else { 12. }),
                            border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1., 0.995, 0.97)),
                        BorderColor(Color::srgb(0.88, 0.86, 0.80)),
                        BorderRadius::all(Val::Px(12.)),
                    ))
                    .with_children(|panel| {
                        panel
                            .spawn(Node {
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: Val::Px(5.),
                                ..row()
                            })
                            .with_children(|modes| {
                                button(
                                    modes,
                                    art,
                                    "4×4 入门",
                                    Action::SudokuSize(4),
                                    94.,
                                    36.,
                                    if size == 4 {
                                        Color::srgb(0.20, 0.31, 0.43)
                                    } else {
                                        Color::srgb(0.91, 0.91, 0.87)
                                    },
                                    if size == 4 {
                                        Color::srgb(0.98, 0.97, 0.93)
                                    } else {
                                        Color::srgb(0.27, 0.33, 0.36)
                                    },
                                );
                                button(
                                    modes,
                                    art,
                                    "9×9 经典",
                                    Action::SudokuSize(9),
                                    94.,
                                    36.,
                                    if size == 9 {
                                        Color::srgb(0.20, 0.31, 0.43)
                                    } else {
                                        Color::srgb(0.91, 0.91, 0.87)
                                    },
                                    if size == 9 {
                                        Color::srgb(0.98, 0.97, 0.93)
                                    } else {
                                        Color::srgb(0.27, 0.33, 0.36)
                                    },
                                );
                            });
                        label(
                            panel,
                            art,
                            status,
                            14.,
                            if g.mistakes > 0 {
                                Color::srgb(0.68, 0.35, 0.30)
                            } else {
                                Color::srgb(0.28, 0.39, 0.45)
                            },
                        );
                        panel.spawn((
                            Node {
                                height: Val::Px(4.),
                                width: Val::Percent(100.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.86, 0.87, 0.84)),
                            BorderRadius::all(Val::Px(2.)),
                        ));
                        panel
                            .spawn(Node {
                                width: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(5.),
                                flex_wrap: FlexWrap::Wrap,
                                row_gap: Val::Px(5.),
                                ..row()
                            })
                            .with_children(|keys| {
                                for value in 1..=size as u8 {
                                    button(
                                        keys,
                                        art,
                                        value.to_string(),
                                        Action::SudokuDigit(value),
                                        if size == 4 {
                                            82.
                                        } else if compact {
                                            52.
                                        } else {
                                            56.
                                        },
                                        if compact { 34. } else { 40. },
                                        Color::srgb(0.91, 0.92, 0.89),
                                        Color::srgb(0.18, 0.28, 0.36),
                                    );
                                }
                            });
                        panel
                            .spawn(Node {
                                column_gap: Val::Px(7.),
                                ..row()
                            })
                            .with_children(|actions| {
                                button(
                                    actions,
                                    art,
                                    "清除",
                                    Action::SudokuClear,
                                    87.,
                                    38.,
                                    Color::srgb(0.91, 0.92, 0.89),
                                    Color::srgb(0.24, 0.32, 0.38),
                                );
                                button(
                                    actions,
                                    art,
                                    "提示",
                                    Action::SudokuHint,
                                    87.,
                                    38.,
                                    Color::srgb(0.78, 0.84, 0.76),
                                    Color::srgb(0.25, 0.37, 0.29),
                                );
                            });
                        button(
                            panel,
                            art,
                            "换一题",
                            Action::SudokuNew,
                            194.,
                            38.,
                            Color::srgb(0.82, 0.43, 0.37),
                            Color::srgb(1., 0.97, 0.92),
                        );
                        label(
                            panel,
                            art,
                            "点选空格，再用数字键填写。每行、每列和每个宫格都不能重复。",
                            12.,
                            Color::srgb(0.49, 0.52, 0.51),
                        );
                    });
            });
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_grows_only_after_eating_a_fruit() {
        let mut snake = SnakeGame::new();
        snake.fruit = (6, 6);
        assert!(!snake.advance(0.2));
        assert_eq!((snake.score, snake.body.len(), snake.body[0]), (0, 3, (5, 6)));
        assert!(snake.turn(1, 0));
        assert!(snake.advance(0.2));
        assert_eq!((snake.score, snake.body.len()), (1, 4));
        snake.fruit = (0, 0);
        assert!(snake.advance(0.16));
        assert_eq!((snake.score, snake.body.len(), snake.body[0]), (1, 4, (7, 6)));
        snake.toggle();
        assert!(!snake.advance(1.));
        assert_eq!(snake.body[0], (7, 6));
    }

    #[test]
    fn minesweeper_flags_and_reveals_safe_cells() {
        let mut mines = MinesweeperGame::new();
        mines.toggle_flag(1);
        assert!(mines.flagged[1]);
        mines.reveal(0);
        assert!(mines.revealed[0]);
        assert!(!mines.lost);
    }

    #[test]
    fn minesweeper_first_reveal_is_safe() {
        let mut mines = MinesweeperGame::new();
        mines.reveal(2);
        assert!(mines.revealed[2]);
        assert!(!mines.lost);
        assert_eq!(mines.mines.iter().filter(|&&mine| mine).count(), 10);
    }

    #[test]
    fn classic_2048_merges_once_per_pair() {
        let mut game = Classic2048Game::new();
        game.cells = [2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        game.score = 0;
        assert!(game.move_board(-1, 0));
        assert_eq!(&game.cells[..4], &[4, 2, 0, 0]);
        assert_eq!(game.score, 4);
    }

    #[test]
    fn classic_2048_scores_two_pairs_and_keeps_best_after_restart() {
        let mut game = Classic2048Game::new();
        game.cells = [2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(game.move_board(-1, 0));
        assert_eq!(&game.cells[..2], &[4, 4]);
        assert_eq!((game.score, game.best, game.moves), (8, 8, 1));
        game.reset();
        assert_eq!((game.score, game.best, game.moves), (0, 8, 0));
        assert_eq!(game.cells.iter().filter(|&&v| v != 0).count(), 2);
    }

    #[test]
    fn classic_2048_recognizes_goal_and_ignores_invalid_moves() {
        let mut game = Classic2048Game::new();
        game.cells = [1024, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(!game.move_board(0, 0));
        assert_eq!(game.cells[0], 1024);
        assert!(game.move_board(-1, 0));
        assert!(game.won);
        assert_eq!(game.cells[0], 2048);
        assert_eq!(game.score, 2048);
    }

    #[test]
    fn klotski_moves_a_selected_soldier_into_an_open_cell() {
        let mut game = KlotskiGame::new();
        game.selected = 6;
        assert!(game.move_selected(0, 1));
        assert_eq!(game.pieces[6].y, 4);
    }

    #[test]
    fn sokoban_blocks_walls_but_allows_open_moves() {
        let mut game = SokobanGame::new();
        assert!(!game.move_player(1, 0));
        assert!(game.move_player(0, 1));
        assert_eq!(game.player, (1, 2));
    }

    #[test]
    fn sudoku_marks_wrong_entries_and_accepts_the_solution() {
        let mut game = SudokuGame::new();
        game.select(2);
        game.set_selected(9);
        assert_eq!(game.mistakes, 1);
        game.set_selected(4);
        assert_eq!(game.mistakes, 0);
        assert_eq!(game.cells[2], 4);
    }
}
