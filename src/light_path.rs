use super::*;

const BOARD: i8 = 7;
const SOURCE: (i8, i8) = (0, 3);
const TARGET: (i8, i8) = (6, 5);
const MIRRORS: [(i8, i8); 4] = [(2, 3), (2, 1), (0, 1), (0, 5)];

pub(super) struct LightPathGame {
    mirrors: [bool; 4], // true is /, false is \
    path: Vec<Vec2>,
    reaches_crystal: bool,
    started: bool,
    elapsed: f32,
    complete: bool,
    won: bool,
    ui_clock: f32,
    pulse_time: f32,
}

impl LightPathGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            mirrors: [false, true, true, true],
            path: Vec::new(),
            reaches_crystal: false,
            started: false,
            elapsed: 0.,
            complete: false,
            won: false,
            ui_clock: 0.,
            pulse_time: 0.,
        };
        game.reset();
        game
    }

    pub(super) fn reset(&mut self) {
        self.mirrors = [false, true, true, true];
        self.started = false;
        self.elapsed = 0.;
        self.complete = false;
        self.won = false;
        self.ui_clock = 0.;
        self.pulse_time = 0.;
        self.retrace();
    }

    pub(super) fn rotate(&mut self, index: usize) {
        if index >= self.mirrors.len() {
            return;
        }
        self.mirrors[index] = !self.mirrors[index];
        self.elapsed = 0.;
        self.started = true;
        self.complete = false;
        self.won = false;
        self.ui_clock = 0.;
        self.pulse_time = 0.;
        self.retrace();
    }

    fn retrace(&mut self) {
        let (path, reaches_crystal) = trace(&self.mirrors);
        self.path = path;
        self.reaches_crystal = reaches_crystal;
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        if !self.started {
            return false;
        }
        self.ui_clock += dt;
        self.pulse_time += dt;
        if !self.complete {
            self.elapsed += dt;
            let end_time = self.total_distance() / 10.0;
            if self.elapsed >= end_time {
                self.elapsed = end_time;
                self.complete = true;
                self.won = self.reaches_crystal;
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

    fn status(&self) -> &'static str {
        if self.won {
            "光束命中水晶！整条光路都对上了。"
        } else if !self.started {
            "点镜片换个方向，让光束折向水晶。"
        } else if !self.complete {
            "光束正在传播……"
        } else {
            "光束离开棋盘了，换个镜片方向再试。"
        }
    }
}

fn trace(mirrors: &[bool; 4]) -> (Vec<Vec2>, bool) {
    let mut path = vec![Vec2::new(SOURCE.0 as f32, SOURCE.1 as f32)];
    let (mut x, mut y) = SOURCE;
    let (mut dx, mut dy) = (1_i8, 0_i8);
    let mut visited = Vec::new();

    for _ in 0..96 {
        if x == TARGET.0 && y == TARGET.1 {
            return (path, true);
        }
        if let Some(index) = MIRRORS.iter().position(|cell| *cell == (x, y)) {
            if visited.contains(&(x, y, dx, dy)) {
                return (path, false);
            }
            visited.push((x, y, dx, dy));
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
        x = next.0;
        y = next.1;
        path.push(Vec2::new(x as f32, y as f32));
    }
    (path, false)
}

pub(super) fn light_path_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let game = &s.light_path;
    page(
        commands,
        art,
        "光路小侦探",
        "旋转镜片，亲眼看光线怎样反射",
        Action::LightReset,
        |root| {
            root.spawn(Node {
                width: Val::Px(408.),
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(10.)),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            })
            .insert((
                BackgroundColor(Color::srgb(0.08, 0.14, 0.24)),
                BorderColor(Color::srgb(0.20, 0.42, 0.56)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|frame| {
                frame
                    .spawn(Node {
                        width: Val::Px(388.),
                        height: Val::Px(388.),
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.),
                        ..default()
                    })
                    .with_children(|board| {
                        for y in 0..BOARD {
                            board
                                .spawn(Node {
                                    width: Val::Px(388.),
                                    height: Val::Px(52.),
                                    column_gap: Val::Px(4.),
                                    ..default()
                                })
                                .with_children(|row| {
                                    for x in 0..BOARD {
                                        let cell = (x, y);
                                        let mirror = MIRRORS.iter().position(|m| *m == cell);
                                        let is_source = cell == SOURCE;
                                        let is_target = cell == TARGET;
                                        if let Some(index) = mirror {
                                            let slash = game.mirrors[index];
                                            row.spawn((
                                                Button,
                                                ButtonTone(Color::srgb(0.16, 0.29, 0.39)),
                                                Action::LightRotate(index),
                                                Node {
                                                    width: Val::Px(52.),
                                                    height: Val::Px(52.),
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    border: UiRect::all(Val::Px(2.)),
                                                    ..default()
                                                },
                                                BackgroundColor(Color::srgb(0.16, 0.29, 0.39)),
                                                BorderColor(Color::srgb(0.52, 0.77, 0.90)),
                                                BorderRadius::all(Val::Px(11.)),
                                            ))
                                            .with_children(|tile| {
                                                label(
                                                    tile,
                                                    art,
                                                    if slash { "/" } else { "\\" },
                                                    31.,
                                                    Color::srgb(0.83, 0.94, 1.),
                                                );
                                            });
                                        } else {
                                            let color = if is_source {
                                                Color::srgb(0.10, 0.42, 0.49)
                                            } else if is_target {
                                                if game.won {
                                                    let pulse =
                                                        (game.pulse_time * 4.).sin() * 0.5 + 0.5;
                                                    Color::srgb(
                                                        0.30 + pulse * 0.17,
                                                        0.58 + pulse * 0.22,
                                                        0.35 + pulse * 0.18,
                                                    )
                                                } else {
                                                    Color::srgb(0.34, 0.42, 0.27)
                                                }
                                            } else {
                                                Color::srgb(0.10, 0.18, 0.27)
                                            };
                                            row.spawn((
                                                Node {
                                                    width: Val::Px(52.),
                                                    height: Val::Px(52.),
                                                    border: UiRect::all(Val::Px(
                                                        if is_target && game.won { 2. } else { 0. },
                                                    )),
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                BackgroundColor(color),
                                                BorderColor(if is_target && game.won {
                                                    Color::srgb(1., 0.89, 0.51)
                                                } else {
                                                    Color::NONE
                                                }),
                                                BorderRadius::all(Val::Px(9.)),
                                            ))
                                            .with_children(|tile| {
                                                if is_source {
                                                    label(
                                                        tile,
                                                        art,
                                                        "光",
                                                        16.,
                                                        Color::srgb(0.76, 0.99, 1.),
                                                    );
                                                } else if is_target {
                                                    label(
                                                        tile,
                                                        art,
                                                        if game.won { "✦" } else { "晶" },
                                                        17.,
                                                        if game.won {
                                                            Color::srgb(1., 0.95, 0.63)
                                                        } else {
                                                            Color::srgb(0.83, 0.90, 0.63)
                                                        },
                                                    );
                                                }
                                            });
                                        }
                                    }
                                });
                        }
                    });
                draw_beam(frame, game);
            });

            label(
                root,
                art,
                game.status(),
                17.,
                if game.won {
                    Color::srgb(0.65, 0.98, 0.73)
                } else {
                    Color::srgb(0.77, 0.86, 0.94)
                },
            );
            label(
                root,
                art,
                "点击斜线镜片切换方向 · 让光从左侧一路折到右下角",
                12.,
                Color::srgb(0.55, 0.69, 0.78),
            );
        },
    );
}

fn draw_beam(board: &mut ChildSpawnerCommands, game: &LightPathGame) {
    if !game.started || game.path.len() < 2 {
        return;
    }
    let speed = 10.0;
    let mut remaining = game.elapsed * speed;
    let traveled = remaining;
    let mut walked = 0.;
    let mut head = game.path[0];
    for pair in game.path.windows(2) {
        let from = pair[0];
        let to = pair[1];
        let distance = from.distance(to);
        if distance <= 0.0001 {
            continue;
        }
        let shown = remaining.clamp(0., distance);
        let fraction = shown / distance;
        let end = from.lerp(to, fraction);
        let draw_segment = |spawn: &mut ChildSpawnerCommands, size: f32, color: Color| {
            let a = cell_px(from);
            let b = cell_px(end);
            let horizontal = (a.y - b.y).abs() < 1.;
            let (left, top, width, height) = if horizontal {
                (
                    a.x.min(b.x),
                    a.y - size * 0.5,
                    (b.x - a.x).abs().max(size),
                    size,
                )
            } else {
                (
                    a.x - size * 0.5,
                    a.y.min(b.y),
                    size,
                    (b.y - a.y).abs().max(size),
                )
            };
            spawn.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(left),
                    top: Val::Px(top),
                    width: Val::Px(width),
                    height: Val::Px(height),
                    ..default()
                },
                BackgroundColor(color),
                BorderRadius::all(Val::Px(size * 0.5)),
                ZIndex(3),
            ));
        };
        if shown > 0.01 {
            draw_segment(board, 10., Color::srgba(0.20, 0.75, 1.0, 0.24));
            draw_segment(board, 3., Color::srgb(0.57, 0.94, 1.));
            head = end;
        }
        walked += distance;
        if MIRRORS
            .iter()
            .any(|cell| Vec2::new(cell.0 as f32, cell.1 as f32).distance(to) < 0.01)
            && traveled >= walked
            && traveled - walked < 1.25
        {
            let center = cell_px(to);
            let size = 15. + ((game.pulse_time * 22.).sin() * 0.5 + 0.5) * 8.;
            board.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(center.x - size * 0.5),
                    top: Val::Px(center.y - size * 0.5),
                    width: Val::Px(size),
                    height: Val::Px(size),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.78, 0.97, 1., 0.72)),
                BorderRadius::all(Val::Percent(50.)),
                ZIndex(4),
            ));
        }
        if remaining <= distance {
            break;
        }
        remaining -= distance;
        head = to;
    }

    let p = cell_px(head);
    let glow = if game.won {
        20. + (game.pulse_time * 4.).sin() * 5.
    } else {
        15.
    };
    board.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(p.x - glow * 0.5),
            top: Val::Px(p.y - glow * 0.5),
            width: Val::Px(glow),
            height: Val::Px(glow),
            ..default()
        },
        BackgroundColor(if game.won {
            Color::srgba(1., 0.92, 0.50, 0.72)
        } else {
            Color::srgba(0.45, 0.92, 1., 0.78)
        }),
        BorderRadius::all(Val::Percent(50.)),
        ZIndex(4),
    ));
}

fn cell_px(point: Vec2) -> Vec2 {
    Vec2::new(36. + point.x * 56., 36. + point.y * 56.)
}
