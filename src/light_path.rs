use super::*;

const BOARD: i8 = 7;
const LEVELS: [&[(i8, i8)]; 12] = [
    &[(0, 3), (2, 3), (2, 5), (6, 5)],
    &[(0, 2), (4, 2), (4, 4), (6, 4)],
    &[(0, 4), (1, 4), (1, 1), (5, 1), (5, 5), (6, 5)],
    &[(0, 3), (2, 3), (2, 1), (0, 1), (0, 5), (6, 5)],
    &[(0, 1), (3, 1), (3, 5), (1, 5), (1, 3), (6, 3)],
    &[(0, 5), (4, 5), (4, 1), (2, 1), (2, 3), (6, 3)],
    &[
        (0, 3),
        (5, 3),
        (5, 1),
        (1, 1),
        (1, 5),
        (4, 5),
        (4, 4),
        (6, 4),
    ],
    &[
        (0, 1),
        (2, 1),
        (2, 5),
        (5, 5),
        (5, 2),
        (3, 2),
        (3, 4),
        (6, 4),
    ],
    &[
        (0, 5),
        (1, 5),
        (1, 1),
        (5, 1),
        (5, 4),
        (3, 4),
        (3, 2),
        (6, 2),
    ],
    &[
        (0, 2),
        (4, 2),
        (4, 0),
        (1, 0),
        (1, 6),
        (5, 6),
        (5, 4),
        (6, 4),
    ],
    &[
        (0, 4),
        (2, 4),
        (2, 0),
        (5, 0),
        (5, 6),
        (1, 6),
        (1, 2),
        (6, 2),
    ],
    &[
        (0, 1),
        (6, 1),
        (6, 5),
        (4, 5),
        (4, 0),
        (1, 0),
        (1, 3),
        (5, 3),
    ],
];

#[derive(Serialize, Deserialize)]
struct LightPathProgress {
    version: u8,
    level: usize,
    completed: u16,
    mirrors: Vec<bool>,
    started: bool,
    won: bool,
}

pub(super) struct LightPathGame {
    level: usize,
    completed: u16,
    mirrors: Vec<bool>,
    path: Vec<Vec2>,
    reaches_crystal: bool,
    started: bool,
    elapsed: f32,
    complete: bool,
    won: bool,
    ui_clock: f32,
    pulse_time: f32,
    save_path: Option<PathBuf>,
}

fn solution(level: usize) -> Vec<bool> {
    LEVELS[level]
        .windows(3)
        .map(|points| {
            let incoming = (points[1].0 - points[0].0, points[1].1 - points[0].1);
            let outgoing = (points[2].0 - points[1].0, points[2].1 - points[1].1);
            (-incoming.1.signum(), -incoming.0.signum())
                == (outgoing.0.signum(), outgoing.1.signum())
        })
        .collect()
}

impl LightPathGame {
    #[cfg(debug_assertions)]
    pub(super) fn qa_state(&self) -> serde_json::Value {
        serde_json::json!({"level": self.level, "mirrors": self.mirrors,
            "solution": solution(self.level), "won": self.won, "completed": self.completed})
    }
    pub(super) fn new() -> Self {
        Self::load(save_directory().join("lightpath-v1.json"))
    }

    fn fresh(save_path: Option<PathBuf>) -> Self {
        let mut game = Self {
            level: 0,
            completed: 0,
            mirrors: Vec::new(),
            path: Vec::new(),
            reaches_crystal: false,
            started: false,
            elapsed: 0.,
            complete: false,
            won: false,
            ui_clock: 0.,
            pulse_time: 0.,
            save_path,
        };
        game.reset_board();
        game
    }

    fn load(path: PathBuf) -> Self {
        let mut game = Self::fresh(Some(path.clone()));
        if let Some(progress) = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<LightPathProgress>(&bytes).ok())
            .filter(Self::valid_progress)
        {
            game.level = progress.level;
            game.completed = progress.completed;
            game.mirrors = progress.mirrors;
            game.started = progress.started;
            game.retrace();
            if progress.won {
                game.elapsed = game.total_distance() / 10.;
                game.complete = true;
                game.won = true;
            }
        }
        game
    }

    fn valid_progress(progress: &LightPathProgress) -> bool {
        if progress.version != 1
            || progress.level >= LEVELS.len()
            || progress.mirrors.len() != LEVELS[progress.level].len() - 2
            || progress.completed >= (1 << LEVELS.len())
        {
            return false;
        }
        // Completion bits must form a continuous prefix; a save cannot unlock skipped levels.
        let count = progress.completed.count_ones() as usize;
        if progress.completed != ((1u16 << count) - 1)
            || progress.level > count.min(LEVELS.len() - 1)
        {
            return false;
        }
        !progress.won
            || (progress.started
                && progress.completed & (1 << progress.level) != 0
                && trace(progress.level, &progress.mirrors).1)
    }

    fn persist(&self) {
        let Some(path) = &self.save_path else {
            return;
        };
        let progress = LightPathProgress {
            version: 1,
            level: self.level,
            completed: self.completed,
            mirrors: self.mirrors.clone(),
            started: self.started,
            won: self.won,
        };
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let temporary = path.with_extension("tmp");
            fs::write(&temporary, serde_json::to_vec(&progress)?)?;
            fs::rename(temporary, path)?;
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("Cannot save light path progress: {error}");
        }
    }

    fn reset_board(&mut self) {
        self.mirrors = solution(self.level)
            .into_iter()
            .map(|slash| !slash)
            .collect();
        self.started = true;
        self.elapsed = 0.;
        self.complete = false;
        self.won = false;
        self.ui_clock = 0.;
        self.pulse_time = 0.;
        self.retrace();
    }

    pub(super) fn reset(&mut self) {
        self.reset_board();
        self.persist();
    }

    pub(super) fn next_level(&mut self) {
        if !self.won || self.level + 1 >= LEVELS.len() {
            return;
        }
        self.level += 1;
        self.reset_board();
        self.persist();
    }

    pub(super) fn rotate(&mut self, index: usize) {
        if index >= self.mirrors.len() || self.won {
            return;
        }
        self.mirrors[index] = !self.mirrors[index];
        self.elapsed = 0.;
        self.started = true;
        self.complete = false;
        self.ui_clock = 0.;
        self.pulse_time = 0.;
        self.retrace();
        self.persist();
    }

    fn retrace(&mut self) {
        (self.path, self.reaches_crystal) = trace(self.level, &self.mirrors);
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        if !self.started {
            return false;
        }
        self.ui_clock += dt;
        self.pulse_time += dt;
        if !self.complete {
            self.elapsed += dt;
            let end_time = self.total_distance() / 10.;
            if self.elapsed >= end_time {
                self.elapsed = end_time;
                self.complete = true;
                self.won = self.reaches_crystal;
                if self.won {
                    self.completed |= 1 << self.level;
                    self.persist();
                    return true;
                }
            }
        }
        if self.ui_clock >= 1. / 30. {
            self.ui_clock %= 1. / 30.;
            true
        } else {
            false
        }
    }

    fn total_distance(&self) -> f32 {
        self.path
            .windows(2)
            .map(|pair| pair[0].distance(pair[1]))
            .sum::<f32>()
            .max(0.5)
    }

    pub(super) fn is_solved(&self) -> bool {
        self.won
    }

    // Only the capture harness calls this; normal play never auto-solves a level.
    pub(super) fn capture_demo(&mut self, level: usize) {
        self.level = level.min(LEVELS.len() - 1);
        self.completed = (1 << self.level) - 1;
        self.reset_board();
        self.mirrors = solution(self.level);
        self.started = true;
        self.retrace();
    }
}

fn trace(level: usize, mirrors: &[bool]) -> (Vec<Vec2>, bool) {
    let route = LEVELS[level];
    let source = route[0];
    let target = route[route.len() - 1];
    let positions = &route[1..route.len() - 1];
    let mut path = vec![Vec2::new(source.0 as f32, source.1 as f32)];
    if mirrors.len() != positions.len() {
        return (path, false);
    }
    let (mut x, mut y) = source;
    let (mut dx, mut dy) = (1_i8, 0_i8);
    let mut visited = Vec::new();
    for _ in 0..(BOARD as usize * BOARD as usize * 4 + 1) {
        if (x, y) == target {
            return (path, true);
        }
        if visited.contains(&(x, y, dx, dy)) {
            return (path, false);
        }
        visited.push((x, y, dx, dy));
        if let Some(index) = positions.iter().position(|cell| *cell == (x, y)) {
            (dx, dy) = if mirrors[index] { (-dy, -dx) } else { (dy, dx) };
        }
        let next = (x + dx, y + dy);
        if next.0 < 0 || next.1 < 0 || next.0 >= BOARD || next.1 >= BOARD {
            path.push(Vec2::new(
                x as f32 + dx as f32 * 0.5,
                y as f32 + dy as f32 * 0.5,
            ));
            return (path, false);
        }
        (x, y) = next;
        path.push(Vec2::new(x as f32, y as f32));
    }
    (path, false)
}

// Coordinates follow one 656 x 1182 screen in the approved multi-level design.
// Fit the whole scene uniformly so neither the board nor its mirrors stretch.
const DESIGN_SIZE: Vec2 = Vec2::new(656., 1182.);

pub(super) fn light_path_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    width: f32,
    height: f32,
) {
    let scale = (width / DESIGN_SIZE.x).min(height / DESIGN_SIZE.y);
    let size = DESIGN_SIZE * scale;
    let game = &s.light_path;
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
            BackgroundColor(Color::srgb(0.16, 0.19, 0.16)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(size.x),
                height: Val::Px(size.y),
                flex_shrink: 0.,
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|scene| {
                let mut plate = ImageNode::new(art.lightpath_level_plates.clone());
                let left = if game.won { 675. } else { 0. };
                plate.rect = Some(Rect::new(left, 0., left + DESIGN_SIZE.x, DESIGN_SIZE.y));
                scene.spawn((plate, design_node(Vec2::ZERO, DESIGN_SIZE, scale)));
                design_text(
                    scene,
                    art,
                    format!("第 {:02} / 12 关", game.level + 1),
                    Vec2::new(240., 174.),
                    Vec2::new(228., 34.),
                    21.,
                    Color::srgb(0.025, 0.28, 0.31),
                    scale,
                );
                let route = LEVELS[game.level];
                let source = route[0];
                let target = route[route.len() - 1];
                draw_beam(scene, game, scale);
                for (index, &(x, y)) in route[1..route.len() - 1].iter().enumerate() {
                    let center = cell_px(Vec2::new(x as f32, y as f32));
                    optical_object(
                        scene,
                        art,
                        if game.mirrors[index] { 0 } else { 1 },
                        center + Vec2::new(0., -5.),
                        Vec2::new(76., 82.),
                        scale,
                        Color::WHITE,
                    );
                    optical_hit(
                        scene,
                        Action::LightRotate(index),
                        center - Vec2::splat(39.),
                        Vec2::splat(78.),
                        scale,
                    );
                }
                optical_object(
                    scene,
                    art,
                    2,
                    cell_px(Vec2::new(source.0 as f32, source.1 as f32)) - Vec2::new(13., 0.),
                    Vec2::new(106., 88.),
                    scale,
                    Color::WHITE,
                );
                optical_object(
                    scene,
                    art,
                    3,
                    cell_px(Vec2::new(target.0 as f32, target.1 as f32)),
                    Vec2::new(85., 120.),
                    scale,
                    if game.won {
                        Color::WHITE
                    } else {
                        Color::srgb(0.78, 0.90, 0.91)
                    },
                );
                optical_hit(
                    scene,
                    Action::Library,
                    Vec2::new(38., 43.),
                    Vec2::new(76., 76.),
                    scale,
                );
                optical_hit(
                    scene,
                    Action::Help,
                    Vec2::new(579., 43.),
                    Vec2::new(75., 76.),
                    scale,
                );
                if game.won {
                    let last = game.level + 1 == LEVELS.len();
                    let subtitle = if last {
                        "12 关全部完成".to_owned()
                    } else {
                        format!("第 {} 关已解锁", game.level + 2)
                    };
                    design_text(
                        scene,
                        art,
                        subtitle,
                        Vec2::new(278., 990.),
                        Vec2::new(295., 35.),
                        23.,
                        Color::srgb(0.25, 0.12, 0.055),
                        scale,
                    );
                    design_text(
                        scene,
                        art,
                        if last {
                            "返回游戏库"
                        } else {
                            "下一关  ›"
                        },
                        Vec2::new(320., 1072.),
                        Vec2::new(225., 49.),
                        if last { 24. } else { 27. },
                        Color::srgb(1., 0.94, 0.72),
                        scale,
                    );
                    optical_hit(
                        scene,
                        Action::LightReset,
                        Vec2::new(89., 1061.),
                        Vec2::new(201., 75.),
                        scale,
                    );
                    optical_hit(
                        scene,
                        if last {
                            Action::Library
                        } else {
                            Action::LightNext
                        },
                        Vec2::new(307., 1061.),
                        Vec2::new(246., 75.),
                        scale,
                    );
                } else {
                    optical_hit(
                        scene,
                        Action::LightReset,
                        Vec2::new(187., 1061.),
                        Vec2::new(290., 75.),
                        scale,
                    );
                }
            });
        });
}

fn design_text(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    origin: Vec2,
    size: Vec2,
    font_size: f32,
    color: Color,
    scale: f32,
) {
    parent
        .spawn((design_node(origin, size, scale), ZIndex(5)))
        .insert(Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..design_node(origin, size, scale)
        })
        .with_children(|container| {
            label(container, art, text, font_size * scale, color);
        });
}

fn design_node(origin: Vec2, size: Vec2, scale: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(origin.x * scale),
        top: Val::Px(origin.y * scale),
        width: Val::Px(size.x * scale),
        height: Val::Px(size.y * scale),
        ..default()
    }
}

fn optical_hit(
    parent: &mut ChildSpawnerCommands,
    action: Action,
    origin: Vec2,
    size: Vec2,
    scale: f32,
) {
    parent.spawn((
        Button,
        action,
        design_node(origin, size, scale),
        BackgroundColor(Color::NONE),
        ZIndex(10),
    ));
}

fn optical_object(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    index: usize,
    center: Vec2,
    size: Vec2,
    scale: f32,
    tint: Color,
) {
    // Tight sprite sampling preserves transparent padding without changing the source atlas.
    let bounds = [
        (116., 73., 537., 530.),
        (698., 73., 1120., 530.),
        (85., 690., 587., 1106.),
        (742., 627., 1140., 1188.),
    ];
    let (x0, y0, x1, y1) = bounds[index];
    let mut image = ImageNode::new(art.lightpath_objects.clone());
    image.rect = Some(Rect::new(x0, y0, x1, y1));
    image.color = tint;
    parent.spawn((
        image,
        design_node(center - size * 0.5, size, scale),
        ZIndex(4),
    ));
}

fn draw_beam(scene: &mut ChildSpawnerCommands, game: &LightPathGame, scale: f32) {
    if !game.started || game.path.len() < 2 {
        return;
    }
    let segments = visible_beam_segments(game);
    let head = segments
        .last()
        .map(|segment| segment.1)
        .unwrap_or(game.path[0]);
    for (from, end) in segments {
        let a = cell_px(from);
        let b = cell_px(end);
        for (thickness, color) in [
            (32., Color::srgba(1., 0.61, 0.02, 0.08)),
            (24., Color::srgba(1., 0.61, 0.02, 0.12)),
            (16., Color::srgba(1., 0.65, 0.02, 0.25)),
            (9., Color::srgba(1., 0.74, 0.02, 0.55)),
            (5., Color::srgb(1., 0.86, 0.12)),
            (3., Color::srgb(1., 0.98, 0.65)),
        ] {
            let thickness = thickness * 0.70;
            let horizontal = (a.y - b.y).abs() < 0.01;
            let (origin, size) = if horizontal {
                (
                    Vec2::new(a.x.min(b.x) - thickness * 0.5, a.y - thickness * 0.5),
                    Vec2::new((b.x - a.x).abs() + thickness, thickness),
                )
            } else {
                (
                    Vec2::new(a.x - thickness * 0.5, a.y.min(b.y) - thickness * 0.5),
                    Vec2::new(thickness, (b.y - a.y).abs() + thickness),
                )
            };
            scene.spawn((
                design_node(origin, size, scale),
                BackgroundColor(color),
                BorderRadius::all(Val::Px(thickness * scale * 0.5)),
                ZIndex(2),
            ));
        }
    }
    if game.won {
        let center = cell_px(head);
        let diameter = 35. + (game.pulse_time * 4.).sin() * 6.;
        scene.spawn((
            design_node(
                center - Vec2::splat(diameter * 0.5),
                Vec2::splat(diameter),
                scale,
            ),
            BackgroundColor(Color::srgba(0.31, 1., 0.89, 0.15)),
            BorderRadius::all(Val::Percent(50.)),
            ZIndex(3),
        ));
    }
}

fn visible_beam_segments(game: &LightPathGame) -> Vec<(Vec2, Vec2)> {
    let mut remaining = game.elapsed * 10.;
    let mut segments: Vec<(Vec2, Vec2)> = Vec::new();
    for pair in game.path.windows(2) {
        let distance = pair[0].distance(pair[1]);
        if distance <= 0.0001 {
            continue;
        }
        let shown = remaining.clamp(0., distance);
        let end = pair[0].lerp(pair[1], shown / distance);
        if shown > 0.001 {
            // Draw one continuous stroke between reflections: per-cell strokes
            // would stack their halos into bright dots at every grid boundary.
            if let Some(last) = segments.last_mut().filter(|last| {
                last.1.distance(pair[0]) < 0.001
                    && (last.1 - last.0)
                        .normalize()
                        .distance((end - pair[0]).normalize())
                        < 0.001
            }) {
                last.1 = end;
            } else {
                segments.push((pair[0], end));
            }
        }
        if remaining <= distance {
            break;
        }
        remaining -= distance;
    }
    segments
}

fn cell_px(point: Vec2) -> Vec2 {
    // Match the artwork's mirror anchors while keeping every beam orthogonal.
    const ROW_Y: [f32; 7] = [344., 382., 474., 565., 646., 727., 811.];
    let y = if point.y < 0. {
        ROW_Y[0] + point.y * 77.
    } else if point.y >= 6. {
        ROW_Y[6] + (point.y - 6.) * 77.
    } else {
        let row = point.y.floor() as usize;
        ROW_Y[row] + (ROW_Y[row + 1] - ROW_Y[row]) * point.y.fract()
    };
    Vec2::new(100. + point.x * (464. / 6.), y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(game: &mut LightPathGame) {
        for (index, desired) in solution(game.level).into_iter().enumerate() {
            if game.mirrors[index] != desired {
                game.rotate(index);
            }
        }
        assert!(game.reaches_crystal);
        assert!(!game.won, "unlock must wait for the beam animation");
        game.advance(100.);
        assert!(game.won);
    }

    #[test]
    fn all_twelve_levels_have_one_solution_using_every_mirror() {
        let mut layouts = std::collections::HashSet::new();
        for (level, route) in LEVELS.iter().enumerate() {
            assert!(layouts.insert(route.to_vec()));
            let count = route.len() - 2;
            let expected = if level < 2 {
                2
            } else if level < 6 {
                4
            } else {
                6
            };
            assert_eq!(count, expected);
            assert!(
                route
                    .iter()
                    .all(|&(x, y)| (0..BOARD).contains(&x) && (0..BOARD).contains(&y))
            );
            let mut solutions = Vec::new();
            for mask in 0..(1 << count) {
                let mirrors: Vec<_> = (0..count).map(|i| mask & (1 << i) != 0).collect();
                let (path, reaches) = trace(level, &mirrors);
                assert!(path.len() <= BOARD as usize * BOARD as usize * 4 + 2);
                if reaches {
                    for &(x, y) in &route[1..route.len() - 1] {
                        assert!(
                            path.contains(&Vec2::new(x as f32, y as f32)),
                            "level {} has a shortcut",
                            level + 1
                        );
                    }
                    solutions.push(mirrors);
                }
            }
            assert_eq!(solutions, vec![solution(level)], "level {}", level + 1);
            let initial: Vec<_> = solution(level).into_iter().map(|v| !v).collect();
            assert!(!trace(level, &initial).1);
        }
    }

    #[test]
    fn campaign_cannot_skip_and_replay_does_not_clear_unlocks() {
        let mut game = LightPathGame::fresh(None);
        for level in 0..LEVELS.len() {
            assert_eq!(game.level, level);
            assert!(!game.won);
            game.next_level();
            assert_eq!(game.level, level);
            solve(&mut game);
            let completed = game.completed;
            assert_eq!(completed, (1 << (level + 1)) - 1);
            game.reset();
            assert_eq!(game.level, level);
            assert_eq!(game.completed, completed);
            assert!(!game.won);
            solve(&mut game);
            game.next_level();
        }
        assert_eq!(game.level, 11);
        assert!(game.won);
        assert_eq!(game.completed, 4095);
    }

    #[test]
    fn saved_mid_puzzle_completed_and_next_level_states_restore() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("lightpath-test-{}-{unique}", std::process::id()));
        let path = directory.join("progress.json");
        let mut game = LightPathGame::load(path.clone());
        assert_eq!(game.level, 0);
        game.rotate(0);
        let resumed = LightPathGame::load(path.clone());
        assert_eq!(resumed.mirrors, game.mirrors);
        assert_eq!(resumed.level, 0);
        assert!(!resumed.won);
        solve(&mut game);
        let mut resumed = LightPathGame::load(path.clone());
        assert!(resumed.won);
        assert_eq!(resumed.completed, 1);
        resumed.next_level();
        let resumed = LightPathGame::load(path.clone());
        assert_eq!(resumed.level, 1);
        assert_eq!(resumed.completed, 1);
        assert!(!resumed.won);
        fs::write(&path, b"{truncated").unwrap();
        assert_eq!(LightPathGame::load(path.clone()).level, 0);
        let invalid = LightPathProgress {
            version: 1,
            level: 11,
            completed: 0,
            mirrors: solution(11),
            started: true,
            won: false,
        };
        fs::write(&path, serde_json::to_vec(&invalid).unwrap()).unwrap();
        assert_eq!(LightPathGame::load(path).level, 0);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn visible_rays_are_continuous_and_only_turn_on_mirrors() {
        let mut game = LightPathGame::fresh(None);
        for level in 0..LEVELS.len() {
            game.level = level;
            game.reset_board();
            solve(&mut game);
            let segments = visible_beam_segments(&game);
            assert_eq!(segments.len(), game.mirrors.len() + 1);
            for (index, &(from, to)) in segments.iter().enumerate() {
                let a = cell_px(from);
                let b = cell_px(to);
                assert!((a.x - b.x).abs() < 0.001 || (a.y - b.y).abs() < 0.001);
                if index + 1 < segments.len() {
                    assert_eq!(to, segments[index + 1].0);
                }
            }
            let saved = game.mirrors.clone();
            game.rotate(0);
            assert_eq!(
                game.mirrors, saved,
                "completed board is frozen until replay"
            );
        }
    }
}
