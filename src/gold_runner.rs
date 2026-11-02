//! Ladder-and-dig arcade game, laid out on the approved 1600×1000 design.
use super::*;
use std::collections::VecDeque;

const GW: f32 = 1600.;
const GH: f32 = 1000.;
const TILE: f32 = 48.;
const LEFT: f32 = 32.;
const ROWS: [f32; 5] = [204., 360., 516., 672., 828.];
const COLS: usize = 32;
const LEVELS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum GoldState {
    Playing,
    Paused,
    Over,
    Won,
}
#[derive(Clone, Copy)]
struct Ladder {
    col: usize,
    top: usize,
    bottom: usize,
}
#[derive(Clone)]
struct Actor {
    x: f32,
    feet: f32,
    vy: f32,
    facing: i8,
    climbing: bool,
    hanging: bool,
    moving: bool,
}
impl Actor {
    fn at(col: usize, row: usize) -> Self {
        Self {
            x: cx(col),
            feet: ROWS[row],
            vy: 0.,
            facing: 1,
            climbing: false,
            hanging: false,
            moving: false,
        }
    }
    fn node(&self) -> (usize, usize) {
        (
            col(self.x),
            tier(self.feet - if self.hanging { 45. } else { 0. }),
        )
    }
}
struct Guard {
    body: Actor,
    spawn: (usize, usize),
    trapped: f32,
    hole: Option<(usize, usize)>,
    respawn: f32,
}
struct Hole {
    c: usize,
    r: usize,
    time: f32,
}
struct Gold {
    c: usize,
    r: usize,
    taken: bool,
}
#[derive(Clone, Copy, Default)]
struct GoldInput {
    horizontal: i8,
    vertical: i8,
    dig: i8,
}
fn cx(c: usize) -> f32 {
    LEFT + (c as f32 + 0.5) * TILE
}
fn col(x: f32) -> usize {
    ((x - LEFT) / TILE).floor().clamp(0., 31.) as usize
}
fn tier(feet: f32) -> usize {
    (0..5)
        .min_by(|a, b| (ROWS[*a] - feet).abs().total_cmp(&(ROWS[*b] - feet).abs()))
        .unwrap()
}

pub(super) struct GoldRunner {
    pub(super) state: GoldState,
    pub(super) level: usize,
    pub(super) lives: u8,
    player: Actor,
    floors: [[bool; COLS]; 5],
    ropes: [Option<(usize, usize)>; 5],
    ladders: Vec<Ladder>,
    guards: Vec<Guard>,
    gold: Vec<Gold>,
    holes: Vec<Hole>,
    elapsed: f32,
    invincible: f32,
    hurt_flash: f32,
    dig_cooldown: f32,
    dig_flash: f32,
    dig_side: i8,
    pub(super) collected: usize,
    pub(super) unlocked: usize,
    exit_col: usize,
}
impl GoldRunner {
    pub(super) fn new() -> Self {
        let unlocked = fs::read_to_string(save_directory().join("gold-runner-progress.txt"))
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .unwrap_or(1)
            .clamp(1, LEVELS);
        Self::stage(unlocked - 1, 3, unlocked)
    }
    fn stage(level: usize, lives: u8, unlocked: usize) -> Self {
        let mut floors = [[true; COLS]; 5];
        for c in 13..20 {
            floors[0][c] = false;
        }
        for c in 16..20 {
            floors[1][c] = false;
        }
        for c in 15..21 {
            floors[2][c] = false;
        }
        let mut ropes = [None; 5];
        ropes[0] = Some((13, 19));
        ropes[2] = Some((15, 20));
        let mut ladders = vec![
            Ladder {
                col: 5,
                top: 0,
                bottom: 1,
            },
            Ladder {
                col: 28,
                top: 0,
                bottom: 1,
            },
            Ladder {
                col: 13,
                top: 1,
                bottom: 2,
            },
            Ladder {
                col: 23,
                top: 1,
                bottom: 2,
            },
            Ladder {
                col: 9,
                top: 2,
                bottom: 3,
            },
            Ladder {
                col: 28,
                top: 2,
                bottom: 4,
            },
            Ladder {
                col: 7,
                top: 3,
                bottom: 4,
            },
        ];
        let mut locations = vec![
            (4, 0),
            (25, 0),
            (13, 1),
            (29, 1),
            (3, 2),
            (22, 2),
            (20, 3),
            (30, 3),
            (8, 4),
            (23, 4),
        ];
        let mirrored = level % 2 == 1;
        if mirrored {
            for row in &mut floors {
                row.reverse();
            }
            for range in &mut ropes {
                if let Some((a, b)) = range {
                    *range = Some((31 - *b, 31 - *a));
                }
            }
            for ladder in &mut ladders {
                ladder.col = 31 - ladder.col;
            }
            for (c, _) in &mut locations {
                *c = 31 - *c;
            }
        }
        // Later maps retain the approved architecture, with additional route choices.
        if level >= 2 {
            ladders.push(Ladder {
                col: if mirrored { 15 } else { 16 },
                top: 3,
                bottom: 4,
            });
        }
        if level >= 4 {
            ladders.push(Ladder {
                col: if mirrored { 8 } else { 23 },
                top: 0,
                bottom: 1,
            });
        }
        let spawns = if mirrored {
            vec![(23, 1), (5, 2), (16, 4), (5, 0)]
        } else {
            vec![(8, 1), (26, 2), (15, 4), (26, 0)]
        };
        let guards = spawns
            .into_iter()
            .take(2 + level / 2)
            .map(|spawn| Guard {
                body: Actor::at(spawn.0, spawn.1),
                spawn,
                trapped: 0.,
                hole: None,
                respawn: 0.,
            })
            .collect();
        Self {
            state: GoldState::Playing,
            level,
            lives,
            player: Actor::at(if mirrored { 19 } else { 12 }, 3),
            floors,
            ropes,
            ladders,
            guards,
            gold: locations
                .into_iter()
                .map(|(c, r)| Gold { c, r, taken: false })
                .collect(),
            holes: vec![],
            elapsed: 0.,
            invincible: 1.8,
            hurt_flash: 0.,
            dig_cooldown: 0.,
            dig_flash: 0.,
            dig_side: 1,
            collected: 0,
            unlocked,
            exit_col: 29,
        }
    }
    pub(super) fn reset(&mut self) {
        *self = Self::stage(self.level, 3, self.unlocked);
    }
    pub(super) fn next(&mut self) {
        if self.state == GoldState::Won {
            *self = Self::stage((self.level + 1) % LEVELS, self.lives.max(3), self.unlocked);
        }
    }
    pub(super) fn toggle(&mut self) {
        self.state = match self.state {
            GoldState::Playing => GoldState::Paused,
            GoldState::Paused => GoldState::Playing,
            s => s,
        };
    }
    fn open(&self, c: usize, r: usize) -> bool {
        self.holes.iter().any(|h| h.c == c && h.r == r)
    }
    fn solid(&self, c: usize, r: usize) -> bool {
        self.floors[r][c] && !self.open(c, r)
    }
    fn on_floor(&self, a: &Actor) -> Option<usize> {
        (0..5).find(|&r| (a.feet - ROWS[r]).abs() < 0.5 && self.solid(col(a.x), r))
    }
    fn rope_at(&self, x: f32, r: usize) -> bool {
        self.ropes[r]
            .is_some_and(|(a, b)| x >= LEFT + a as f32 * TILE && x < LEFT + (b + 1) as f32 * TILE)
    }
    fn dig(&mut self, direction: i8) -> bool {
        if direction == 0 || self.dig_cooldown > 0. || self.state != GoldState::Playing {
            return false;
        }
        let Some(r) = self.on_floor(&self.player) else {
            return false;
        };
        if r == 4 {
            return false;
        }
        let c = col(self.player.x) as i32 + direction as i32;
        if !(0..COLS as i32).contains(&c) {
            return false;
        }
        let c = c as usize;
        if !self.solid(c, r)
            || self
                .ladders
                .iter()
                .any(|l| l.col == c && r >= l.top && r <= l.bottom)
        {
            return false;
        }
        self.holes.push(Hole { c, r, time: 4.8 });
        self.dig_cooldown = 0.4;
        self.dig_flash = 0.22;
        self.dig_side = direction;
        self.player.facing = direction;
        true
    }
    fn move_actor(&self, a: &mut Actor, input: GoldInput, speed: f32, dt: f32) {
        a.moving = input.horizontal != 0 || input.vertical != 0;
        if input.horizontal != 0 {
            a.facing = input.horizontal;
            a.x = (a.x + input.horizontal as f32 * speed * dt)
                .clamp(LEFT + 17., LEFT + COLS as f32 * TILE - 17.);
            a.climbing = false;
        }
        if input.vertical != 0 {
            if self.collected == self.gold.len()
                && input.vertical < 0
                && (a.x - cx(self.exit_col)).abs() < 26.
                && a.feet <= ROWS[0] + 2.
            {
                a.x = cx(self.exit_col);
                a.feet = (a.feet - speed * dt).max(120.);
                a.climbing = true;
                return;
            }
            if let Some(l) = self
                .ladders
                .iter()
                .filter(|l| {
                    (a.x - cx(l.col)).abs() < 29.
                        && a.feet >= ROWS[l.top] - 1.
                        && a.feet <= ROWS[l.bottom] + 1.
                })
                .find(|l| {
                    if input.vertical < 0 {
                        a.feet > ROWS[l.top] + 0.1
                    } else {
                        a.feet < ROWS[l.bottom] - 0.1
                    }
                })
            {
                a.x = cx(l.col);
                let before = a.feet;
                a.feet = (a.feet + input.vertical as f32 * speed * 0.84 * dt)
                    .clamp(ROWS[l.top], ROWS[l.bottom]);
                if let Some(stop) = ROWS[l.top..=l.bottom]
                    .iter()
                    .copied()
                    .filter(|y| {
                        if input.vertical < 0 {
                            *y < before - 0.01 && *y >= a.feet
                        } else {
                            *y > before + 0.01 && *y <= a.feet
                        }
                    })
                    .min_by(|x, y| (x - before).abs().total_cmp(&(y - before).abs()))
                {
                    a.feet = stop;
                }
                a.vy = 0.;
                a.hanging = false;
                a.climbing = !ROWS.iter().any(|y| (a.feet - y).abs() < 0.1);
                return;
            }
        }
        if a.climbing && input.horizontal == 0 {
            return;
        }
        if a.hanging {
            let r = tier(a.feet - 45.);
            if self.solid(col(a.x), r) {
                a.feet = ROWS[r];
                a.hanging = false;
                a.vy = 0.;
            } else if self.rope_at(a.x, r) && input.vertical <= 0 {
                a.vy = 0.;
                return;
            } else {
                a.hanging = false;
                a.vy = 100.;
            }
        }
        if self.on_floor(a).is_some() {
            a.vy = 0.;
            return;
        }
        let before = a.feet;
        a.vy = (a.vy + 1450. * dt).min(650.);
        a.feet += a.vy * dt;
        for r in 0..5 {
            if before <= ROWS[r] + 0.1 && a.feet >= ROWS[r] && self.solid(col(a.x), r) {
                a.feet = ROWS[r];
                a.vy = 0.;
                return;
            }
            let rope_y = ROWS[r] + 45.;
            if input.vertical <= 0 && before <= rope_y && a.feet >= rope_y && self.rope_at(a.x, r) {
                a.feet = rope_y;
                a.hanging = true;
                a.vy = 0.;
                return;
            }
        }
    }
    fn available(&self, c: usize, r: usize) -> bool {
        self.floors[r][c] || self.rope_at(cx(c), r)
    }
    fn neighbours(&self, (c, r): (usize, usize)) -> Vec<(usize, usize)> {
        let mut out = vec![];
        for nc in [c.checked_sub(1), c.checked_add(1).filter(|c| *c < COLS)]
            .into_iter()
            .flatten()
        {
            if self.available(nc, r) {
                out.push((nc, r));
            } else if let Some(nr) = ((r + 1)..5).find(|&nr| self.available(nc, nr)) {
                out.push((nc, nr));
            }
        }
        for l in &self.ladders {
            if l.col == c && r >= l.top && r <= l.bottom {
                if r > l.top {
                    out.push((c, r - 1));
                }
                if r < l.bottom {
                    out.push((c, r + 1));
                }
            }
        }
        out
    }
    fn route(&self, start: (usize, usize), target: (usize, usize)) -> Option<(usize, usize)> {
        if start == target {
            return Some(target);
        }
        let mut parent = [None; 160];
        let mut visited = [false; 160];
        let index = |(c, r): (usize, usize)| r * COLS + c;
        let mut queue = VecDeque::from([start]);
        visited[index(start)] = true;
        while let Some(at) = queue.pop_front() {
            for next in self.neighbours(at) {
                if visited[index(next)] {
                    continue;
                }
                visited[index(next)] = true;
                parent[index(next)] = Some(at);
                if next == target {
                    let mut node = next;
                    while let Some(prev) = parent[index(node)] {
                        if prev == start {
                            return Some(node);
                        }
                        node = prev;
                    }
                    return None;
                }
                queue.push_back(next);
            }
        }
        None
    }
    fn chase(&self, a: &Actor, target: &Actor) -> GoldInput {
        // Complete the current ladder segment before replanning at a platform.
        if a.climbing {
            let above = ROWS.iter().copied().filter(|y| *y < a.feet - 0.5).last();
            let below = ROWS.iter().copied().find(|y| *y > a.feet + 0.5);
            let v = if target.feet < a.feet { -1 } else { 1 };
            if (v < 0 && above.is_some()) || (v > 0 && below.is_some()) {
                return GoldInput {
                    vertical: v,
                    ..default()
                };
            }
        }
        let start = a.node();
        let dest = target.node();
        if start == dest {
            return GoldInput {
                horizontal: if (target.x - a.x).abs() < 5. {
                    0
                } else if target.x < a.x {
                    -1
                } else {
                    1
                },
                ..default()
            };
        }
        let Some((c, r)) = self.route(start, dest) else {
            return GoldInput::default();
        };
        if r != start.1 && c == start.0 {
            if (a.x - cx(c)).abs() > 8. {
                return GoldInput {
                    horizontal: if a.x < cx(c) { 1 } else { -1 },
                    ..default()
                };
            }
            GoldInput {
                vertical: if r < start.1 { -1 } else { 1 },
                ..default()
            }
        } else {
            GoldInput {
                horizontal: if cx(c) < a.x { -1 } else { 1 },
                ..default()
            }
        }
    }
    fn hurt(&mut self) {
        if self.invincible > 0. {
            return;
        }
        self.lives = self.lives.saturating_sub(1);
        self.invincible = 2.;
        self.hurt_flash = 2.;
        self.player = Actor::at(if self.level % 2 == 1 { 19 } else { 12 }, 3);
        if self.lives == 0 {
            self.state = GoldState::Over;
        }
    }
    fn step(&mut self, dt: f32, input: GoldInput) {
        self.elapsed += dt;
        self.invincible = (self.invincible - dt).max(0.);
        self.hurt_flash = (self.hurt_flash - dt).max(0.);
        self.dig_cooldown = (self.dig_cooldown - dt).max(0.);
        self.dig_flash = (self.dig_flash - dt).max(0.);
        if input.dig != 0 {
            self.dig(input.dig);
        }
        let mut player = self.player.clone();
        self.move_actor(&mut player, input, 205., dt);
        self.player = player;
        for gold in &mut self.gold {
            if !gold.taken
                && (self.player.x - cx(gold.c)).abs() < 27.
                && (self.player.feet - ROWS[gold.r]).abs() < 28.
            {
                gold.taken = true;
                self.collected += 1;
            }
        }
        let mut guards = std::mem::take(&mut self.guards);
        for guard in &mut guards {
            if guard.respawn > 0. {
                guard.respawn -= dt;
                if guard.respawn <= 0. {
                    guard.body = Actor::at(guard.spawn.0, guard.spawn.1);
                }
                continue;
            }
            if guard.trapped > 0. {
                guard.trapped -= dt;
                if guard.trapped <= 0. {
                    if let Some((c, r)) = guard.hole.take() {
                        guard.body = Actor::at(
                            if self.solid(c.saturating_sub(1), r) {
                                c.saturating_sub(1)
                            } else {
                                (c + 1).min(31)
                            },
                            r,
                        );
                    }
                }
            } else {
                let command = self.chase(&guard.body, &self.player);
                self.move_actor(&mut guard.body, command, 82. + self.level as f32 * 8., dt);
                for hole in &self.holes {
                    if (guard.body.x - cx(hole.c)).abs() < 23.
                        && guard.body.feet > ROWS[hole.r] + 5.
                        && guard.body.feet < ROWS[hole.r] + 115.
                    {
                        guard.body.x = cx(hole.c);
                        guard.body.feet = ROWS[hole.r] + 62.;
                        guard.body.vy = 0.;
                        guard.trapped = 2.6;
                        guard.hole = Some((hole.c, hole.r));
                        break;
                    }
                }
            }
            if guard.trapped <= 0.
                && (guard.body.x - self.player.x).abs() < 34.
                && (guard.body.feet - self.player.feet).abs() < 48.
            {
                self.hurt();
            }
        }
        self.guards = guards;
        for hole in &mut self.holes {
            hole.time -= dt;
        }
        let closing: Vec<_> = self
            .holes
            .iter()
            .filter(|h| h.time <= 0.)
            .map(|h| (h.c, h.r))
            .collect();
        for (c, r) in closing {
            if col(self.player.x) == c
                && self.player.feet > ROWS[r] + 3.
                && self.player.feet - 58. < ROWS[r] + 36.
            {
                self.hurt();
            }
            for guard in &mut self.guards {
                if guard.hole == Some((c, r)) {
                    guard.trapped = 0.;
                    guard.hole = None;
                    guard.respawn = 1.2;
                }
            }
        }
        self.holes.retain(|h| h.time > 0.);
        if self.player.feet > GH + 60. {
            self.hurt();
        }
        if self.collected == self.gold.len()
            && self.player.feet <= 143.
            && (self.player.x - cx(self.exit_col)).abs() < 26.
            && self.state == GoldState::Playing
        {
            self.state = GoldState::Won;
            self.unlocked = self.unlocked.max((self.level + 2).min(LEVELS));
            if !cfg!(test) {
                if let Err(e) = fs::create_dir_all(save_directory()).and_then(|_| {
                    fs::write(
                        save_directory().join("gold-runner-progress.txt"),
                        self.unlocked.to_string(),
                    )
                }) {
                    eprintln!("Cannot save gold runner progress: {e}");
                }
            }
        }
    }
    fn advance(&mut self, dt: f32, input: GoldInput) {
        let mut remaining = dt.min(0.2);
        while remaining > 0.00001 && self.state == GoldState::Playing {
            let step = remaining.min(1. / 120.);
            self.step(step, input);
            remaining -= step;
        }
    }
    #[cfg(debug_assertions)]
    pub(super) fn qa_position(&self) -> Vec2 {
        Vec2::new(self.player.x, self.player.feet)
    }
    #[cfg(debug_assertions)]
    pub(super) fn qa_holes(&self) -> usize {
        self.holes.len()
    }
}

#[derive(Resource)]
pub(super) struct GoldArt {
    background: Handle<Image>,
    atlas: Handle<Image>,
    reference: Handle<Image>,
}
pub(super) fn setup_gold(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GoldArt {
        background: assets.load("gold_runner/temple-background.png"),
        atlas: assets.load("gold_runner/temple-atlas.png"),
        reference: assets.load("gold_runner/approved.png"),
    });
}

fn gold_point(p: Vec2, size: Vec2) -> Vec2 {
    p / size * Vec2::new(GW, GH)
}
pub(super) fn gold_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::GoldRunner || s.library_open || s.help_open {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let previous = s.gold_runner.state;
    if keys.just_pressed(KeyCode::KeyP)
        || keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Space)
    {
        s.gold_runner.toggle();
    }
    if keys.just_pressed(KeyCode::KeyR) {
        s.gold_runner.reset();
    }
    if keys.just_pressed(KeyCode::Enter) && s.gold_runner.state == GoldState::Won {
        s.gold_runner.next();
    }
    if std::env::var_os("MAGNET_CAPTURE").is_none()
        && !window.focused
        && s.gold_runner.state == GoldState::Playing
    {
        s.gold_runner.toggle();
    }
    let mut flags = [
        keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA),
        keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD),
        keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW),
        keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS),
        keys.pressed(KeyCode::KeyZ) || keys.pressed(KeyCode::KeyQ),
        keys.pressed(KeyCode::KeyX) || keys.pressed(KeyCode::KeyE),
    ];
    for p in touches.iter().map(|t| t.position()).chain(
        mouse
            .pressed(MouseButton::Left)
            .then(|| window.cursor_position())
            .flatten(),
    ) {
        let p = gold_point(p, Vec2::new(window.width(), window.height()));
        for (i, rect) in gold_controls().iter().enumerate() {
            if rect.contains(p) {
                flags[i] = true;
            }
        }
    }
    let input = GoldInput {
        horizontal: flags[1] as i8 - flags[0] as i8,
        vertical: flags[3] as i8 - flags[2] as i8,
        dig: flags[5] as i8 - flags[4] as i8,
    };
    s.gold_runner.advance(time.delta_secs(), input);
    s.ui_dirty |= previous != s.gold_runner.state || s.gold_runner.state == GoldState::Playing;
}
fn gold_controls() -> [Rect; 6] {
    [
        Rect::new(28., 820., 120., 908.),
        Rect::new(200., 820., 292., 908.),
        Rect::new(116., 748., 208., 836.),
        Rect::new(116., 888., 208., 976.),
        Rect::new(1270., 820., 1412., 964.),
        Rect::new(1424., 820., 1566., 964.),
    ]
}
fn gn(x: f32, y: f32, w: f32, h: f32, sx: f32, sy: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x * sx),
        top: Val::Px(y * sy),
        width: Val::Px(w * sx),
        height: Val::Px(h * sy),
        ..default()
    }
}
fn gold_rect(index: usize) -> Rect {
    let boxes = [
        (94., 185., 248., 333.),
        (418., 94., 534., 146.),
        (658., 224., 923., 283.),
        (1003., 226., 1189., 347.),
        (58., 447., 249., 650.),
        (369., 446., 565., 650.),
        (683., 442., 876., 645.),
        (954., 463., 1244., 630.),
        (57., 726., 280., 960.),
        (378., 726., 588., 969.),
        (674., 730., 893., 967.),
        (1031., 735., 1174., 952.),
        (65., 1007., 250., 1194.),
        (381., 1006., 566., 1194.),
        (697., 1007., 882., 1194.),
        (1013., 1007., 1198., 1194.),
    ];
    let (x, y, r, b) = boxes[index];
    Rect::new(x, y, r, b)
}
fn gs(
    p: &mut ChildSpawnerCommands,
    a: &GoldArt,
    index: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    sx: f32,
    sy: f32,
    flip: bool,
) {
    p.spawn((
        gn(x, y, w, h, sx, sy),
        ImageNode {
            image: a.atlas.clone(),
            rect: Some(gold_rect(index)),
            flip_x: flip,
            ..default()
        },
    ));
}
fn gr(
    p: &mut ChildSpawnerCommands,
    a: &GoldArt,
    rect: Rect,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    sx: f32,
    sy: f32,
) {
    p.spawn((
        gn(x, y, w, h, sx, sy),
        ImageNode {
            image: a.reference.clone(),
            rect: Some(rect),
            ..default()
        },
    ));
}
fn gt(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    size: f32,
    color: Color,
    sx: f32,
    sy: f32,
) {
    p.spawn((
        gn(x, y, w, size * 1.4, sx, sy),
        Text::new(text),
        TextFont {
            font: art.font.clone(),
            font_size: size * sx,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}
fn gb(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    title: &str,
    action: Action,
    x: f32,
    y: f32,
    w: f32,
    sx: f32,
    sy: f32,
) {
    p.spawn((
        Button,
        action,
        gn(x, y, w, 62., sx, sy),
        BackgroundColor(Color::srgb(0.55, 0.28, 0.07)),
        BorderColor(Color::srgb(0.97, 0.67, 0.22)),
        BorderRadius::all(Val::Px(7.)),
    ))
    .with_children(|b| {
        gt(
            b,
            art,
            title,
            0.,
            9.,
            w,
            29.,
            Color::srgb(1., 0.91, 0.65),
            sx,
            sy,
        )
    });
}
pub(super) fn gold_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    a: &GoldArt,
    width: f32,
    height: f32,
) {
    let g = &s.gold_runner;
    let sx = width / GW;
    let sy = height / GH;
    commands
        .spawn((
            UiRoot,
            WorldInputBlocker,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.03, 0.06, 0.09)),
        ))
        .with_children(|p| {
            p.spawn((
                gn(0., 0., GW, GH, sx, sy),
                ImageNode::new(a.background.clone()),
            ));
            for r in 0..5 {
                for c in 0..COLS {
                    if g.solid(c, r) {
                        gs(
                            p,
                            a,
                            0,
                            LEFT + c as f32 * TILE,
                            ROWS[r],
                            TILE,
                            35.,
                            sx,
                            sy,
                            false,
                        );
                    } else if g.open(c, r) {
                        // The open well is passable; its small bottom lip has no collision.
                        gs(
                            p,
                            a,
                            0,
                            LEFT + c as f32 * TILE,
                            ROWS[r] + 54.,
                            TILE,
                            13.,
                            sx,
                            sy,
                            false,
                        );
                        let h = g.holes.iter().find(|h| h.c == c && h.r == r).unwrap();
                        if h.time < 1.1 {
                            gs(
                                p,
                                a,
                                0,
                                LEFT + c as f32 * TILE,
                                ROWS[r],
                                TILE * (1.1 - h.time) / 1.1,
                                15.,
                                sx,
                                sy,
                                false,
                            );
                        }
                    }
                }
            }
            for l in &g.ladders {
                let mut y = ROWS[l.top];
                while y < ROWS[l.bottom] {
                    let h = 32_f32.min(ROWS[l.bottom] - y);
                    gs(p, a, 1, cx(l.col) - 17., y, 34., h, sx, sy, false);
                    y += h;
                }
            }
            for r in 0..5 {
                if let Some((left, right)) = g.ropes[r] {
                    gs(
                        p,
                        a,
                        2,
                        LEFT + left as f32 * TILE,
                        ROWS[r] - 6.,
                        (right - left + 1) as f32 * TILE,
                        26.,
                        sx,
                        sy,
                        false,
                    );
                }
            }

            if g.collected == g.gold.len() {
                let mut y = 124.;
                while y < ROWS[0] {
                    gs(p, a, 1, cx(g.exit_col) - 17., y, 34., 28., sx, sy, false);
                    y += 28.;
                }
            }
            for gold in &g.gold {
                if !gold.taken {
                    gs(
                        p,
                        a,
                        3,
                        cx(gold.c) - 25.,
                        ROWS[gold.r] - 31.,
                        50.,
                        31.,
                        sx,
                        sy,
                        false,
                    );
                }
            }
            for guard in &g.guards {
                if guard.respawn > 0. {
                    continue;
                }
                let frame = if guard.body.moving {
                    9 + (g.elapsed * 6.) as usize % 2
                } else {
                    8
                };
                gs(
                    p,
                    a,
                    frame,
                    guard.body.x - 36.,
                    guard.body.feet - 79.,
                    72.,
                    79.,
                    sx,
                    sy,
                    guard.body.facing < 0,
                );
            }
            if g.hurt_flash <= 0. || (g.elapsed * 9.) as u32 % 2 == 0 {
                let frame = if g.dig_flash > 0. {
                    7
                } else if g.player.climbing {
                    11
                } else if g.player.moving {
                    5 + (g.elapsed * 8.) as usize % 2
                } else {
                    4
                };
                let (w, h) = if frame == 7 {
                    (108., 62.)
                } else if frame == 11 {
                    (48., 73.)
                } else {
                    (74., 70.)
                };
                let x = if frame == 7 && g.player.facing < 0 {
                    g.player.x - 71.
                } else if frame == 11 {
                    g.player.x - 24.
                } else {
                    g.player.x - 37.
                };
                gs(
                    p,
                    a,
                    frame,
                    x,
                    g.player.feet - h,
                    w,
                    h,
                    sx,
                    sy,
                    g.player.facing < 0,
                );
            }
            if g.dig_flash > 0. {
                for n in 0..5 {
                    let t = 1. - g.dig_flash / 0.22;
                    let x = g.player.x + g.dig_side as f32 * (36. + n as f32 * 8. + t * 22.);
                    let y = g.player.feet - 12. - (t * 3.14).sin() * 26. + n as f32 * 4.;
                    gs(p, a, 0, x, y, 6., 6., sx, sy, false);
                }
            }
            // Reuse the approved title, frame and control artwork directly.
            gr(p, a, Rect::new(0., 0., 1586., 65.), 0., 0., GW, 65., sx, sy);
            p.spawn((
                gn(670., 8., 250., 46., sx, sy),
                BackgroundColor(Color::srgb(0.055, 0.06, 0.065)),
            ));
            gt(
                p,
                art,
                format!("第 {} 关", g.level + 1),
                670.,
                8.,
                250.,
                32.,
                Color::srgb(1., 0.9, 0.66),
                sx,
                sy,
            );
            p.spawn((
                gn(1015., 8., 150., 46., sx, sy),
                BackgroundColor(Color::srgb(0.055, 0.06, 0.065)),
            ));
            gt(
                p,
                art,
                format!("{} / {}", g.collected, g.gold.len()),
                1015.,
                10.,
                150.,
                30.,
                Color::srgb(1., 0.9, 0.55),
                sx,
                sy,
            );
            for i in g.lives..3 {
                p.spawn((
                    gn(1292. + i as f32 * 50., 9., 43., 42., sx, sy),
                    BackgroundColor(Color::srgb(0.055, 0.06, 0.065)),
                ));
            }
            p.spawn((
                Button,
                Action::GoldToggle,
                gn(1530., 7., 56., 52., sx, sy),
                BackgroundColor(Color::NONE),
            ));
            let ui_src = [
                Rect::new(28., 815., 120., 899.),
                Rect::new(199., 815., 290., 899.),
                Rect::new(118., 743., 206., 827.),
                Rect::new(118., 883., 206., 965.),
                Rect::new(1265., 816., 1402., 955.),
                Rect::new(1420., 816., 1558., 955.),
            ];
            for (i, rect) in gold_controls().into_iter().enumerate() {
                p.spawn((
                    gn(rect.min.x, rect.min.y, rect.width(), rect.height(), sx, sy),
                    ImageNode {
                        image: a.reference.clone(),
                        rect: Some(ui_src[i]),
                        ..default()
                    },
                    if i >= 4 {
                        BorderRadius::MAX
                    } else {
                        BorderRadius::all(Val::Px(8. * sx))
                    },
                ));
            }
            if g.state != GoldState::Playing {
                p.spawn((
                    gn(0., 65., GW, GH - 65., sx, sy),
                    BackgroundColor(Color::srgba(0.01, 0.025, 0.04, 0.8)),
                ))
                .with_children(|panel| {
                    let title = match g.state {
                        GoldState::Paused => "暂停探险",
                        GoldState::Over => "探险结束",
                        GoldState::Won => {
                            if g.level + 1 == LEVELS {
                                "全部遗迹已通关"
                            } else {
                                "成功逃出遗迹"
                            }
                        }
                        _ => "",
                    };
                    gt(
                        panel,
                        art,
                        title,
                        400.,
                        200.,
                        800.,
                        64.,
                        Color::srgb(1., 0.82, 0.37),
                        sx,
                        sy,
                    );
                    gt(
                        panel,
                        art,
                        format!(
                            "第 {} 关 · 黄金 {} / {}",
                            g.level + 1,
                            g.collected,
                            g.gold.len()
                        ),
                        400.,
                        312.,
                        800.,
                        33.,
                        Color::srgb(0.95, 0.9, 0.74),
                        sx,
                        sy,
                    );
                    gb(
                        panel,
                        art,
                        if g.state == GoldState::Paused {
                            "继续探险"
                        } else if g.state == GoldState::Won {
                            if g.level + 1 == LEVELS {
                                "再挑战一轮"
                            } else {
                                "进入下一关"
                            }
                        } else {
                            "重试本关"
                        },
                        if g.state == GoldState::Paused {
                            Action::GoldToggle
                        } else if g.state == GoldState::Won {
                            Action::GoldNext
                        } else {
                            Action::GoldReset
                        },
                        600.,
                        422.,
                        400.,
                        sx,
                        sy,
                    );
                    gb(
                        panel,
                        art,
                        "重玩本关",
                        Action::GoldReset,
                        600.,
                        507.,
                        400.,
                        sx,
                        sy,
                    );
                    gb(
                        panel,
                        art,
                        "返回游戏库",
                        Action::Library,
                        600.,
                        592.,
                        400.,
                        sx,
                        sy,
                    );
                });
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn game() -> GoldRunner {
        let mut g = GoldRunner::stage(0, 3, 1);
        g.guards.clear();
        g
    }
    fn run(g: &mut GoldRunner, seconds: f32, input: GoldInput) {
        for _ in 0..(seconds * 120.) as usize {
            g.advance(1. / 120., input);
        }
    }
    #[test]
    fn every_gold_and_exit_has_a_route_on_all_six_maps() {
        for level in 0..LEVELS {
            let g = GoldRunner::stage(level, 3, 1);
            let start = g.player.node();
            for gold in &g.gold {
                assert!(g.available(gold.c, gold.r));
                assert!(
                    g.route(start, (gold.c, gold.r)).is_some(),
                    "level {level}: {},{}",
                    gold.c,
                    gold.r
                );
            }
            assert!(g.route(start, (g.exit_col, 0)).is_some());
        }
    }
    #[test]
    fn dig_opens_floor_and_player_falls_through() {
        let mut g = game();
        assert!(g.dig(1));
        assert!(!g.solid(13, 3));
        run(
            &mut g,
            0.55,
            GoldInput {
                horizontal: 1,
                ..default()
            },
        );
        assert!(g.player.feet > ROWS[3] + 40.);
        assert!(!g.dig(-1));
    }
    #[test]
    fn cannot_dig_bottom_ladder_or_repeatedly_without_cooldown() {
        let mut g = game();
        g.player = Actor::at(8, 3);
        assert!(!g.dig(1));
        g.player = Actor::at(12, 4);
        assert!(!g.dig(1));
        g.player = Actor::at(12, 3);
        assert!(g.dig(1));
        assert!(!g.dig(-1));
    }
    #[test]
    fn pits_close_and_restore_the_floor() {
        let mut g = game();
        g.dig(1);
        run(&mut g, 5.0, GoldInput::default());
        assert!(g.holes.is_empty());
        assert!(g.solid(13, 3));
    }
    #[test]
    fn ladder_reaches_both_endpoints() {
        let mut g = game();
        g.player = Actor::at(9, 3);
        run(
            &mut g,
            1.2,
            GoldInput {
                vertical: -1,
                ..default()
            },
        );
        assert_eq!(g.player.feet, ROWS[2]);
        run(
            &mut g,
            1.2,
            GoldInput {
                vertical: 1,
                ..default()
            },
        );
        assert_eq!(g.player.feet, ROWS[3]);
    }
    #[test]
    fn rope_catches_falling_player_and_allows_crossing() {
        let mut g = game();
        g.player = Actor::at(12, 0);
        run(
            &mut g,
            1.0,
            GoldInput {
                horizontal: 1,
                ..default()
            },
        );
        assert!(g.player.hanging);
        run(
            &mut g,
            1.0,
            GoldInput {
                horizontal: 1,
                ..default()
            },
        );
        assert!(!g.player.hanging);
        assert_eq!(g.player.feet, ROWS[0]);
    }
    #[test]
    fn guardian_falls_into_pit_and_escapes_later() {
        let mut g = game();
        g.dig(1);
        g.guards.push(Guard {
            body: Actor::at(13, 3),
            spawn: (20, 1),
            trapped: 0.,
            hole: None,
            respawn: 0.,
        });
        run(&mut g, 0.35, GoldInput::default());
        assert!(g.guards[0].trapped > 0.);
        assert_eq!(g.guards[0].hole, Some((13, 3)));
        run(&mut g, 2.8, GoldInput::default());
        assert!(g.guards[0].hole.is_none());
        assert!(g.guards[0].trapped <= 0.);
    }
    #[test]
    fn guardian_uses_ladders_to_chase_across_tiers() {
        let mut g = game();
        g.player = Actor::at(9, 2);
        g.guards.push(Guard {
            body: Actor::at(9, 3),
            spawn: (9, 3),
            trapped: 0.,
            hole: None,
            respawn: 0.,
        });
        run(&mut g, 1.0, GoldInput::default());
        assert!(g.guards[0].body.feet < ROWS[3] - 50.);
    }
    #[test]
    fn gold_collects_once_and_exit_stays_locked_until_all_gold() {
        let mut g = game();
        g.player = Actor::at(4, 0);
        run(&mut g, 0.05, GoldInput::default());
        assert_eq!(g.collected, 1);
        run(&mut g, 0.05, GoldInput::default());
        assert_eq!(g.collected, 1);
        g.player = Actor::at(g.exit_col, 0);
        run(
            &mut g,
            0.6,
            GoldInput {
                vertical: -1,
                ..default()
            },
        );
        assert_eq!(g.player.feet, ROWS[0]);
        assert_eq!(g.state, GoldState::Playing);
    }
    #[test]
    fn pause_damage_immunity_and_restart() {
        let mut g = game();
        let x = g.player.x;
        g.toggle();
        run(
            &mut g,
            1.,
            GoldInput {
                horizontal: 1,
                ..default()
            },
        );
        assert_eq!(g.player.x, x);
        g.toggle();
        g.invincible = 0.;
        g.hurt();
        g.hurt();
        assert_eq!(g.lives, 2);
        g.invincible = 0.;
        g.hurt();
        g.invincible = 0.;
        g.hurt();
        assert_eq!(g.state, GoldState::Over);
        g.reset();
        assert_eq!(g.lives, 3);
        assert_eq!(g.state, GoldState::Playing);
    }
    #[test]
    fn physical_controller_can_reach_every_gold_on_all_maps() {
        for level in 0..LEVELS {
            let mut g = GoldRunner::stage(level, 3, 1);
            g.guards.clear();
            for n in 0..g.gold.len() {
                let target = Actor::at(g.gold[n].c, g.gold[n].r);
                for _ in 0..12000 {
                    if g.gold[n].taken {
                        break;
                    }
                    let input = g.chase(&g.player, &target);
                    g.advance(1. / 120., input);
                }
                assert!(
                    g.gold[n].taken,
                    "level {level} gold {n}, player {:?}, target {:?}",
                    g.player.node(),
                    target.node()
                );
            }
            assert_eq!(g.collected, 10);
            let exit = Actor::at(g.exit_col, 0);
            for _ in 0..12000 {
                if g.player.node() == exit.node() && (g.player.x - exit.x).abs() < 10. {
                    break;
                }
                let input = g.chase(&g.player, &exit);
                g.advance(1. / 120., input);
            }
            run(
                &mut g,
                0.7,
                GoldInput {
                    vertical: -1,
                    ..default()
                },
            );
            assert_eq!(g.state, GoldState::Won, "exit level {level}");
            let old_level = g.level;
            g.next();
            assert_eq!(g.level, (old_level + 1) % LEVELS);
            assert_eq!(g.collected, 0);
        }
    }
    #[test]
    fn touch_hitboxes_match_the_six_visible_controls() {
        let controls = gold_controls();
        for (i, rect) in controls.iter().enumerate() {
            let p = rect.center();
            let scaled = gold_point(p * Vec2::new(960. / GW, 540. / GH), Vec2::new(960., 540.));
            assert!(rect.contains(scaled));
            assert_eq!(
                controls.iter().filter(|r| r.contains(scaled)).count(),
                1,
                "control {i}"
            );
        }
    }
}
