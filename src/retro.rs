use super::*;
use std::collections::VecDeque;

const SNAKE_W: i8 = 18;
const SNAKE_H: i8 = 24;

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
        let center=(SNAKE_W/2-1,SNAKE_H/2);
        self.body = vec![center,(center.0-1,center.1),(center.0-2,center.1)];
        self.previous_body = self.body.clone();
        self.direction = (1, 0);
        self.pending = (1, 0);
        self.fruit = (SNAKE_W/2+2,SNAKE_H/2);
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
        let cells=SNAKE_W as usize*SNAKE_H as usize;
        let start = (self.fruits_eaten as usize * 37) % cells;
        for step in 0..cells {
            let index = (start + step * 37) % cells;
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
        let step_time = (0.28 - self.fruits_eaten as f32 * 0.003).max(0.16);
        while self.clock >= step_time && !self.over {
            self.clock -= step_time;
            self.previous_body.clone_from(&self.body);
            self.direction = self.pending;
            let head = self.body[0];
            let next = ((head.0 + self.direction.0).rem_euclid(SNAKE_W), (head.1 + self.direction.1).rem_euclid(SNAKE_H));
            let eating = next == self.fruit;
            let collision_len = if eating {
                self.body.len()
            } else {
                self.body.len().saturating_sub(1)
            };
            if self.body[..collision_len].contains(&next)
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
                let mut from = self.previous_body.get(index).copied().unwrap_or((x, y));
                if (from.0-x).abs()>1 || (from.1-y).abs()>1 {from=(x,y);}
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
    _compact: bool,
    width: f32,
    height: f32,
) {
    // One approved renderer for every viewport. Resizing must never select a legacy theme.
    snake_ui_pixel(commands, s, art, width, height);
}

#[derive(Component)]
pub(super) struct SnakePhoneBoard;
fn snake_pixel_node(x:f32,y:f32,w:f32,h:f32)->Node {
    Node {position_type:PositionType::Absolute,left:Val::Px(x),top:Val::Px(y),width:Val::Px(w),height:Val::Px(h),..default()}
}
fn snake_pixel_sample(p:&mut ChildSpawnerCommands,art:&Art,source:[f32;4],destination:[f32;4])->Entity {
    p.spawn((snake_pixel_node(destination[0],destination[1],destination[2],destination[3]),ImageNode {image:art.snake_pixel.clone(),rect:Some(Rect::new(source[0],source[1],source[2],source[3])),..default()})).id()
}
fn snake_pixel_number(p:&mut ChildSpawnerCommands,n:u32,x:f32,y:f32,scale:f32) {
    const DIGITS:[[u8;7];10]=[
        [14,17,19,21,25,17,14],[4,12,4,4,4,4,14],[14,17,1,2,4,8,31],[30,1,1,14,1,1,30],[2,6,10,18,31,2,2],
        [31,16,16,30,1,1,30],[14,16,16,30,17,17,14],[31,1,2,4,8,8,8],[14,17,17,14,17,17,14],[14,17,17,15,1,1,14]
    ];
    let text=n.to_string();let pixel=5.*scale;let start=x+(110.*scale-text.len() as f32*pixel*6.)*0.5;
    for (i,ch) in text.bytes().enumerate(){for (r,bits) in DIGITS[(ch-b'0') as usize].iter().enumerate(){for c in 0..5 {if bits&(1<<(4-c))!=0 {p.spawn((snake_pixel_node(start+(i*6+c) as f32*pixel,y+r as f32*pixel,pixel,pixel),BackgroundColor(Color::srgb(1.,0.88,0.08))));}}}}
}
fn snake_ui_pixel(commands:&mut Commands,s:&Session,art:&Art,viewport_width:f32,height:f32) {
    let width=viewport_width.min(height*0.64);
    let g=&s.snake;let k=width/941.;
    let header=212.*k;let controls_h=124.*k;let footer_h=48.*k;
    let cell=((width-60.*k)/SNAKE_W as f32).min((height-header-controls_h-footer_h-50.*k)/SNAKE_H as f32);
    let bw=cell*SNAKE_W as f32;let bh=cell*SNAKE_H as f32;
    let bx=(width-bw)*0.5;let by=header+24.*k;
    let keys_y=(height-controls_h-footer_h-10.*k).max(by+bh+24.*k);
    commands.spawn((UiRoot,WorldInputBlocker,Node {width:Val::Percent(100.),height:Val::Percent(100.),align_items:AlignItems::Center,justify_content:JustifyContent::Center,overflow:Overflow::clip(),..default()},BackgroundColor(Color::srgb(0.015,0.055,0.15))))
    .with_children(|frame| {
        frame.spawn(Node {width:Val::Px(width),height:Val::Px(height),flex_shrink:0.,position_type:PositionType::Relative,..default()})
        .with_children(|root| {
        root.spawn((snake_pixel_node(0.,0.,width,height),ImageNode {image:art.snake_pixel.clone(),rect:Some(Rect::new(0.,0.,32.,32.)),image_mode:NodeImageMode::Tiled{tile_x:true,tile_y:true,stretch_value:k},..default()}));
        snake_pixel_sample(root,art,[0.,0.,941.,212.],[0.,0.,width,header]);
        // Remove baked example digits while preserving the exact approved capsule artwork.
        snake_pixel_sample(root,art,[54.,151.,76.,190.],[210.*k,150.*k,118.*k,40.*k]);
        snake_pixel_sample(root,art,[54.,151.,76.,190.],[561.*k,150.*k,116.*k,40.*k]);
        snake_pixel_number(root,g.score,213.*k,153.*k,k);
        snake_pixel_number(root,g.best,564.*k,153.*k,k);
        for (action,rect) in [(Action::SnakeReset,[578.,35.,148.,81.]),(Action::Library,[747.,35.,174.,81.]),(Action::SnakeToggle,[740.,137.,172.,66.])] {
            root.spawn((Button,WorldInputBlocker,action,snake_pixel_node(rect[0]*k,rect[1]*k,rect[2]*k,rect[3]*k),ZIndex(10)));
        }
        if g.started || g.over {
        snake_pixel_sample(root,art,[754.,157.,767.,174.],[777.*k,147.*k,107.*k,42.*k]);
        root.spawn((snake_pixel_node(767.*k,143.*k,122.*k,51.*k),Text::new(if g.over{"重开"}else if g.running{"暂停"}else if g.started{"继续"}else{"开始"}),TextFont{font:art.font.clone(),font_size:35.*k,..default()},TextColor(Color::WHITE),TextLayout::new_with_justify(JustifyText::Center)));
        }
        snake_pixel_sample(root,art,[16.,221.,925.,1448.],[bx-15.*k,by-15.*k,bw+30.*k,bh+30.*k]);
        root.spawn((SnakePhoneBoard,snake_pixel_node(bx,by,bw,bh),BackgroundColor(Color::srgb(0.035,0.29,0.33)))).with_children(|board| {
            for y in 0..SNAKE_H {for x in 0..SNAKE_W {
                let source=if (x+y)%2==0 {[40.,246.,75.,279.]}else{[90.,246.,125.,279.]};
                snake_pixel_sample(board,art,source,[x as f32*cell,y as f32*cell,cell-k,cell-k]);
            }}
            let body=g.interpolated_body();
            for point in body.iter().skip(1).rev(){snake_pixel_sample(board,art,[371.,777.,424.,829.],[point.x*cell,point.y*cell,cell,cell]);}
            if let Some(point)=body.first(){
                let angle=match g.direction{(0,-1)=>-std::f32::consts::FRAC_PI_2,(0,1)=>std::f32::consts::FRAC_PI_2,(-1,0)=>std::f32::consts::PI,_=>0.};
                board.spawn((snake_pixel_node(point.x*cell-cell*0.06,point.y*cell-cell*0.08,cell*1.5,cell*1.14),ImageNode{image:art.snake_pixel.clone(),rect:Some(Rect::new(562.,770.,645.,833.)),..default()},Transform::from_rotation(Quat::from_rotation_z(angle))));
            }
            board.spawn((snake_pixel_node(g.fruit.0 as f32*cell-cell*0.03,g.fruit.1 as f32*cell-cell*0.1,cell*1.06,cell*1.2),ImageNode{image:if g.golden_fruit{art.snake_gold.clone()}else{art.snake_pixel.clone()},rect:Some(if g.golden_fruit{Rect::new(407.,380.,849.,897.)}else{Rect::new(731.,623.,793.,697.)}),..default()}));
            if g.over||(!g.running&&g.started){board.spawn((snake_pixel_node(0.,bh*0.4,bw,cell*2.2),BackgroundColor(Color::srgba(0.01,0.04,0.12,0.9)))).with_children(|message|{
                label(message,art,if g.over{"撞到自己了 · 点重开再来"}else{"已暂停 · 点继续"},24.*k,Color::srgb(1.,0.86,0.3));
            });}
        });
        let sources=[[94.,1465.,276.,1598.],[286.,1465.,471.,1598.],[477.,1465.,663.,1598.],[670.,1465.,853.,1598.]];
        let actions=[Action::SnakeMove(-1,0),Action::SnakeMove(0,-1),Action::SnakeMove(0,1),Action::SnakeMove(1,0)];
        let kw=174.*k;let gap=18.*k;let kh=124.*k;let start=(width-kw*4.-gap*3.)*0.5;
        for i in 0..4 {root.spawn((Button,actions[i],snake_pixel_node(start+i as f32*(kw+gap),keys_y,kw,kh),ImageNode {image:art.snake_pixel.clone(),rect:Some(Rect::new(sources[i][0],sources[i][1],sources[i][2],sources[i][3])),..default()}));}
        root.spawn((snake_pixel_node(0.,height-45.*k,width,40.*k),Text::new("碰边穿行 · 慢速起步"),TextFont{font:art.font.clone(),font_size:25.*k,..default()},TextColor(Color::srgb(0.62,0.8,1.)),TextLayout::new_with_justify(JustifyText::Center)));
    });    });
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
        let mut snake=SnakeGame::new();let start=snake.body[0];snake.fruit=(start.0+1,start.1);
        assert!(!snake.advance(0.3));assert_eq!(snake.body[0],start);
        snake.turn(1,0);snake.advance(0.28);assert_eq!((snake.score,snake.body.len()),(1,4));
        snake.fruit=(0,0);snake.advance(0.28);assert_eq!(snake.body[0],(start.0+2,start.1));
        snake.toggle();let stopped=snake.body[0];assert!(!snake.advance(1.));assert_eq!(snake.body[0],stopped);
    }
    #[test]
    fn snake_wraps_all_four_edges_without_ending() {
        for (head,dir,expected) in [((SNAKE_W-1,8),(1,0),(0,8)),((0,8),(-1,0),(SNAKE_W-1,8)),((8,0),(0,-1),(8,SNAKE_H-1)),((8,SNAKE_H-1),(0,1),(8,0))] {
            let mut g=SnakeGame::new();g.body=vec![head];g.previous_body=g.body.clone();g.direction=dir;g.pending=dir;g.running=true;g.fruit=(2,2);g.advance(0.29);
            assert_eq!(g.body[0],expected);assert!(!g.over);let p=g.interpolated_body()[0];assert!(p.distance(Vec2::new(expected.0 as f32,expected.1 as f32))<0.0001);
        }
    }
    #[test]
    fn snake_still_ends_on_self_collision() {
        let mut g=SnakeGame::new();g.body=vec![(2,2),(2,3),(3,3),(3,2),(3,1)];g.running=true;g.advance(0.28);assert!(g.over);assert!(!g.running);
    }
    #[test]
    fn snake_starts_slow_and_golden_scores_do_not_accelerate_it() {
        let mut g=SnakeGame::new();let start=g.body[0];g.score=999;g.turn(1,0);g.advance(0.16);assert_eq!(g.body[0],start);g.advance(0.12);assert_eq!(g.body[0],(start.0+1,start.1));
        g.fruits_eaten=1000;let before=g.body[0];g.advance(0.15);assert_eq!(g.body[0],before);g.advance(0.01);assert_eq!(g.body[0],(before.0+1,before.1));
    }
    #[test]
    fn snake_fruit_search_covers_the_entire_large_board() {
        let mut g=SnakeGame::new();g.body=(0..SNAKE_H).flat_map(|y|(0..SNAKE_W).map(move|x|(x,y))).collect();let free=g.body.pop().unwrap();g.next_fruit();assert_eq!(g.fruit,free);
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
