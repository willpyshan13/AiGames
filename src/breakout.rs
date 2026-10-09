use super::*;

const FIELD_W: f32 = 900.;
const FIELD_H: f32 = 500.;
const PADDLE_Y: f32 = 451.;
const BALL_R: f32 = 9.;

#[derive(Clone, Copy, PartialEq, Eq)]
enum BonusKind {
    Wide,
    Multi,
    Laser,
}

impl BonusKind {
    fn label(self) -> &'static str {
        match self {
            Self::Wide => "宽",
            Self::Multi => "多",
            Self::Laser => "激",
        }
    }
    fn color(self) -> Color {
        match self {
            Self::Wide => Color::srgb(0.33, 0.82, 0.96),
            Self::Multi => Color::srgb(0.98, 0.70, 0.28),
            Self::Laser => Color::srgb(0.96, 0.36, 0.57),
        }
    }
}

#[derive(Clone)]
struct BreakoutBall {
    pos: Vec2,
    vel: Vec2,
    trail: Vec<Vec2>,
}

#[derive(Clone)]
struct BreakoutBrick {
    base_x: f32,
    pos: Vec2,
    size: Vec2,
    hp: u8,
    max_hp: u8,
    hue: usize,
    moving: bool,
    boss: bool,
    flash: f32,
}

struct BreakoutDrop {
    pos: Vec2,
    kind: BonusKind,
}

struct BreakoutSpark {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    color: Color,
}

struct BreakoutBeam {
    x: f32,
    end_y: f32,
    life: f32,
}

pub(super) struct BreakoutGame {
    pub(super) stage: u8,
    pub(super) score: u32,
    pub(super) lives: u8,
    pub(super) paddle_x: f32,
    balls: Vec<BreakoutBall>,
    bricks: Vec<BreakoutBrick>,
    drops: Vec<BreakoutDrop>,
    sparks: Vec<BreakoutSpark>,
    beams: Vec<BreakoutBeam>,
    wide_for: f32,
    laser_for: f32,
    laser_cooldown: f32,
    combo: u8,
    combo_for: f32,
    destroyed: u32,
    elapsed: f32,
    redraw: f32,
    pub(super) ready: bool,
    pub(super) paused: bool,
    pub(super) over: bool,
    pub(super) won: bool,
}

impl BreakoutGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            stage: 1,
            score: 0,
            lives: 3,
            paddle_x: FIELD_W * 0.5,
            balls: Vec::new(),
            bricks: Vec::new(),
            drops: Vec::new(),
            sparks: Vec::new(),
            beams: Vec::new(),
            wide_for: 0.,
            laser_for: 0.,
            laser_cooldown: 0.,
            combo: 0,
            combo_for: 0.,
            destroyed: 0,
            elapsed: 0.,
            redraw: 0.,
            ready: true,
            paused: false,
            over: false,
            won: false,
        };
        game.load_stage(1);
        game
    }

    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    fn stage_name(&self) -> &'static str {
        match self.stage {
            1 => "彩虹街区",
            2 => "移动砖墙",
            _ => "机械堡垒",
        }
    }

    fn paddle_width(&self) -> f32 {
        if self.wide_for > 0. { 164. } else { 112. }
    }

    fn load_stage(&mut self, stage: u8) {
        self.stage = stage;
        self.bricks.clear();
        self.drops.clear();
        self.beams.clear();
        self.balls.clear();
        self.ready = true;
        self.paused = false;
        self.wide_for = 0.;
        self.laser_for = 0.;
        self.combo = 0;
        self.combo_for = 0.;
        self.elapsed = 0.;
        let rows = match stage {
            1 => 4,
            2 => 5,
            _ => 4,
        };
        for row in 0..rows {
            for col in 0..10 {
                if stage == 3 && row == 0 && (3..=6).contains(&col) {
                    continue;
                }
                let x = 70. + col as f32 * 77.;
                let y = 73. + row as f32 * 38.;
                let hp = if stage > 1 && (row + col) % 5 == 0 {
                    2
                } else {
                    1
                };
                self.bricks.push(BreakoutBrick {
                    base_x: x,
                    pos: Vec2::new(x, y),
                    size: Vec2::new(70., 29.),
                    hp,
                    max_hp: hp,
                    hue: (row as usize + stage as usize - 1) % 6,
                    moving: stage >= 2 && row % 2 == 1,
                    boss: false,
                    flash: 0.,
                });
            }
        }
        if stage == 3 {
            self.bricks.push(BreakoutBrick {
                base_x: 306.,
                pos: Vec2::new(306., 62.),
                size: Vec2::new(288., 42.),
                hp: 7,
                max_hp: 7,
                hue: 5,
                moving: true,
                boss: true,
                flash: 0.,
            });
        }
        self.attach_ball();
    }

    fn attach_ball(&mut self) {
        self.balls.clear();
        self.balls.push(BreakoutBall {
            pos: Vec2::new(self.paddle_x, PADDLE_Y - BALL_R - 2.),
            vel: Vec2::ZERO,
            trail: Vec::new(),
        });
        self.ready = true;
    }

    pub(super) fn launch(&mut self) {
        if self.over || self.won || self.paused || !self.ready {
            return;
        }
        self.ready = false;
        if let Some(ball) = self.balls.first_mut() {
            ball.vel = Vec2::new(195., -340.);
        }
    }

    pub(super) fn move_paddle(&mut self, x: f32) {
        let half = self.paddle_width() * 0.5;
        self.paddle_x = x.clamp(half + 12., FIELD_W - half - 12.);
        if self.ready {
            if let Some(ball) = self.balls.first_mut() {
                ball.pos.x = self.paddle_x;
            }
        }
    }

    fn brick_color(hue: usize) -> Color {
        match hue % 6 {
            0 => Color::srgb(0.98, 0.36, 0.45),
            1 => Color::srgb(1., 0.60, 0.29),
            2 => Color::srgb(0.98, 0.79, 0.33),
            3 => Color::srgb(0.38, 0.83, 0.66),
            4 => Color::srgb(0.32, 0.65, 0.99),
            _ => Color::srgb(0.70, 0.44, 0.96),
        }
    }

    fn sparks_at(&mut self, center: Vec2, color: Color, count: usize) {
        for i in 0..count {
            let angle = i as f32 * std::f32::consts::TAU / count as f32;
            self.sparks.push(BreakoutSpark {
                pos: center,
                vel: Vec2::new(angle.cos(), angle.sin()) * (80. + (i % 3) as f32 * 35.),
                life: 0.38,
                color,
            });
        }
    }

    fn on_brick_hit(&mut self, center: Vec2, hue: usize, destroyed: bool, boss: bool) {
        self.combo = if self.combo_for > 0. {
            self.combo.saturating_add(1).min(9)
        } else {
            1
        };
        self.combo_for = 2.3;
        self.score += (if boss { 60 } else { 35 }) * u32::from(self.combo);
        self.sparks_at(
            center,
            Self::brick_color(hue),
            if destroyed { 7 } else { 4 },
        );
        if destroyed {
            self.destroyed += 1;
            if self.destroyed % 6 == 0 {
                let kind = match (self.destroyed / 6) % 3 {
                    1 => BonusKind::Wide,
                    2 => BonusKind::Multi,
                    _ => BonusKind::Laser,
                };
                self.drops.push(BreakoutDrop { pos: center, kind });
            }
        }
    }

    fn apply_bonus(&mut self, kind: BonusKind) {
        match kind {
            BonusKind::Wide => {
                self.wide_for = 12.;
                self.move_paddle(self.paddle_x);
            }
            BonusKind::Multi => {
                if self.ready {
                    self.launch();
                }
                if let Some(ball) = self.balls.first().cloned() {
                    for direction in [-1., 1.] {
                        let mut extra = ball.clone();
                        extra.vel = Vec2::new(direction * 270., -320.);
                        extra.trail.clear();
                        self.balls.push(extra);
                    }
                }
            }
            BonusKind::Laser => {
                self.laser_for = 10.;
                self.laser_cooldown = 0.;
            }
        }
        self.score += 100;
        self.sparks_at(Vec2::new(self.paddle_x, PADDLE_Y), kind.color(), 10);
    }

    fn fire_laser(&mut self) {
        let mut hits = Vec::new();
        for x in [self.paddle_x - 27., self.paddle_x + 27.] {
            let target = self
                .bricks
                .iter()
                .enumerate()
                .filter(|(_, brick)| {
                    brick.hp > 0 && x >= brick.pos.x && x <= brick.pos.x + brick.size.x
                })
                .max_by(|(_, a), (_, b)| a.pos.y.total_cmp(&b.pos.y))
                .map(|(index, _)| index);
            let end_y = if let Some(index) = target {
                let brick = &mut self.bricks[index];
                brick.hp -= 1;
                brick.flash = 0.18;
                hits.push((
                    brick.pos + brick.size * 0.5,
                    brick.hue,
                    brick.hp == 0,
                    brick.boss,
                ));
                brick.pos.y + brick.size.y
            } else {
                14.
            };
            self.beams.push(BreakoutBeam {
                x,
                end_y,
                life: 0.15,
            });
        }
        for (center, hue, destroyed, boss) in hits {
            self.on_brick_hit(center, hue, destroyed, boss);
        }
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        if self.paused || self.over || self.won {
            return false;
        }
        let dt = dt.min(0.05);
        self.elapsed += dt;
        self.wide_for = (self.wide_for - dt).max(0.);
        self.laser_for = (self.laser_for - dt).max(0.);
        self.combo_for = (self.combo_for - dt).max(0.);
        for brick in &mut self.bricks {
            brick.flash = (brick.flash - dt).max(0.);
            if brick.moving {
                brick.pos.x = brick.base_x + (self.elapsed * 1.7 + brick.pos.y * 0.07).sin() * 13.;
            }
        }
        for spark in &mut self.sparks {
            spark.pos += spark.vel * dt;
            spark.vel.y += 140. * dt;
            spark.life -= dt;
        }
        self.sparks.retain(|spark| spark.life > 0.);
        for beam in &mut self.beams {
            beam.life -= dt;
        }
        self.beams.retain(|beam| beam.life > 0.);
        let mut collected = Vec::new();
        let paddle_half = self.paddle_width() * 0.5;
        for drop in &mut self.drops {
            drop.pos.y += 130. * dt;
            if drop.pos.y >= PADDLE_Y - 12.
                && drop.pos.y <= PADDLE_Y + 20.
                && (drop.pos.x - self.paddle_x).abs() <= paddle_half + 14.
            {
                collected.push(drop.kind);
                drop.pos.y = FIELD_H + 100.;
            }
        }
        self.drops.retain(|drop| drop.pos.y < FIELD_H + 20.);
        for kind in collected {
            self.apply_bonus(kind);
        }
        if self.ready {
            self.balls[0].pos.x = self.paddle_x;
        } else {
            let steps = (dt / 0.009).ceil().max(1.) as usize;
            let step = dt / steps as f32;
            let mut hit_events = Vec::new();
            for _ in 0..steps {
                for ball in &mut self.balls {
                    ball.pos += ball.vel * step;
                    if ball.pos.x < BALL_R + 9. {
                        ball.pos.x = BALL_R + 9.;
                        ball.vel.x = ball.vel.x.abs();
                    } else if ball.pos.x > FIELD_W - BALL_R - 9. {
                        ball.pos.x = FIELD_W - BALL_R - 9.;
                        ball.vel.x = -ball.vel.x.abs();
                    }
                    if ball.pos.y < BALL_R + 9. {
                        ball.pos.y = BALL_R + 9.;
                        ball.vel.y = ball.vel.y.abs();
                    }
                    let half = paddle_half;
                    if ball.vel.y > 0.
                        && ball.pos.y + BALL_R >= PADDLE_Y
                        && ball.pos.y - BALL_R <= PADDLE_Y + 15.
                        && (ball.pos.x - self.paddle_x).abs() <= half + BALL_R
                    {
                        ball.pos.y = PADDLE_Y - BALL_R;
                        let offset = ((ball.pos.x - self.paddle_x) / half).clamp(-1., 1.);
                        ball.vel.x = (offset * 315. + ball.vel.x * 0.17).clamp(-380., 380.);
                        ball.vel.y = -(370. + self.stage as f32 * 24.);
                        self.combo_for = 0.;
                    }
                    for brick in &mut self.bricks {
                        if brick.hp == 0 {
                            continue;
                        }
                        let nearest = ball.pos.clamp(brick.pos, brick.pos + brick.size);
                        if ball.pos.distance_squared(nearest) > BALL_R * BALL_R {
                            continue;
                        }
                        let left = ball.pos.x + BALL_R - brick.pos.x;
                        let right = brick.pos.x + brick.size.x - (ball.pos.x - BALL_R);
                        let top = ball.pos.y + BALL_R - brick.pos.y;
                        let bottom = brick.pos.y + brick.size.y - (ball.pos.y - BALL_R);
                        if left.min(right) < top.min(bottom) {
                            ball.vel.x = -ball.vel.x;
                        } else {
                            ball.vel.y = -ball.vel.y;
                        }
                        brick.hp -= 1;
                        brick.flash = 0.18;
                        hit_events.push((
                            brick.pos + brick.size * 0.5,
                            brick.hue,
                            brick.hp == 0,
                            brick.boss,
                        ));
                        break;
                    }
                }
            }
            for (center, hue, destroyed, boss) in hit_events {
                self.on_brick_hit(center, hue, destroyed, boss);
            }
            for ball in &mut self.balls {
                ball.trail.push(ball.pos);
                if ball.trail.len() > 5 {
                    ball.trail.remove(0);
                }
            }
            self.balls.retain(|ball| ball.pos.y < FIELD_H + BALL_R);
            if self.balls.is_empty() {
                self.lives = self.lives.saturating_sub(1);
                self.combo = 0;
                if self.lives == 0 {
                    self.over = true;
                } else {
                    self.attach_ball();
                }
            }
        }
        if self.laser_for > 0. && !self.ready {
            self.laser_cooldown -= dt;
            if self.laser_cooldown <= 0. {
                self.fire_laser();
                self.laser_cooldown = 0.34;
            }
        }
        if self.bricks.iter().all(|brick| brick.hp == 0) {
            if self.stage == 3 {
                self.won = true;
                self.score += 1000 + u32::from(self.lives) * 250;
            } else {
                self.score += 400 * u32::from(self.stage);
                self.load_stage(self.stage + 1);
            }
        }
        self.redraw += dt;
        if self.redraw >= 0.033 {
            self.redraw = 0.;
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub(super) struct BreakoutBoard;

#[derive(Resource, Default)]
pub(super) struct BreakoutPointer {
    touch: Option<u64>,
    mouse: bool,
}

pub(super) fn breakout_pointer_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    boards: Query<(&ComputedNode, &GlobalTransform), With<BreakoutBoard>>,
    mut pointer: ResMut<BreakoutPointer>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::Breakout || s.library_open || s.help_open {
        *pointer = BreakoutPointer::default();
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((node, transform)) = boards.single() else {
        return;
    };
    let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
    let scale = window.scale_factor();
    if pointer.touch.is_none() && !pointer.mouse {
        if let Some(touch) = touches
            .iter_just_pressed()
            .find(|touch| rect.contains(touch.position() * scale))
        {
            pointer.touch = Some(touch.id());
            s.breakout.launch();
        } else if mouse.just_pressed(MouseButton::Left)
            && window
                .cursor_position()
                .is_some_and(|point| rect.contains(point * scale))
        {
            pointer.mouse = true;
            s.breakout.launch();
        }
    }
    let point = if let Some(id) = pointer.touch {
        touches
            .get_pressed(id)
            .map(|touch| touch.position() * scale)
    } else if pointer.mouse && mouse.pressed(MouseButton::Left) {
        window.cursor_position().map(|point| point * scale)
    } else {
        None
    };
    if let Some(point) = point {
        let x = ((point.x - rect.min.x) / rect.width()).clamp(0., 1.) * FIELD_W;
        s.breakout.move_paddle(x);
        s.ui_dirty = true;
    } else {
        *pointer = BreakoutPointer::default();
    }
}

pub(super) fn breakout_key_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::Breakout || s.library_open || s.help_open {
        return;
    }
    let left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    if left != right {
        let speed = if left { -520. } else { 520. };
        let x = s.breakout.paddle_x + speed * time.delta_secs();
        s.breakout.move_paddle(x);
        s.ui_dirty = true;
    }
    if keys.just_pressed(KeyCode::Space) {
        s.breakout.launch();
        s.ui_dirty = true;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        s.breakout.paused = !s.breakout.paused;
        s.ui_dirty = true;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        s.breakout.reset();
        s.ui_dirty = true;
    }
}

fn place(
    parent: &mut ChildSpawnerCommands,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    scale: f32,
    color: Color,
    radius: f32,
) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x * scale),
            top: Val::Px(y * scale),
            width: Val::Px(w * scale),
            height: Val::Px(h * scale),
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::all(Val::Px(radius * scale)),
    ));
}

pub(super) fn breakout_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    viewport_width: f32,
    viewport_height: f32,
) {
    let game = &s.breakout;
    let board_w = (viewport_width - 48.)
        .min((viewport_height - 118.) * 1.8)
        .min(1100.)
        .max(300.);
    let board_h = board_w * FIELD_H / FIELD_W;
    let scale = board_w / FIELD_W;
    let small = viewport_height < 680.;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(12.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(8.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.08, 0.16)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(board_w),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|bar| {
                label(
                    bar,
                    art,
                    "霓虹打砖块",
                    if small { 22. } else { 28. },
                    Color::srgb(0.91, 0.96, 1.),
                );
                label(
                    bar,
                    art,
                    format!(
                        "第 {} 关  ·  {} 分  ·  ♥ {}",
                        game.stage, game.score, game.lives
                    ),
                    if small { 15. } else { 18. },
                    Color::srgb(1., 0.82, 0.46),
                );
                bar.spawn(Node {
                    column_gap: Val::Px(6.),
                    ..row()
                })
                .with_children(|actions| {
                    button(
                        actions,
                        art,
                        if game.ready {
                            "发球"
                        } else if game.paused {
                            "继续"
                        } else {
                            "暂停"
                        },
                        Action::BreakoutToggle,
                        if small { 62. } else { 72. },
                        38.,
                        Color::srgb(0.18, 0.58, 0.59),
                        Color::WHITE,
                    );
                    button(
                        actions,
                        art,
                        "重来",
                        Action::BreakoutReset,
                        if small { 62. } else { 72. },
                        38.,
                        Color::srgb(0.34, 0.34, 0.56),
                        Color::WHITE,
                    );
                    button(
                        actions,
                        art,
                        "游戏库",
                        Action::Library,
                        if small { 72. } else { 82. },
                        38.,
                        SLATE,
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                BreakoutBoard,
                WorldInputBlocker,
                Node {
                    width: Val::Px(board_w),
                    height: Val::Px(board_h),
                    position_type: PositionType::Relative,
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.12, 0.22)),
                BorderColor(Color::srgb(0.30, 0.65, 0.78)),
                BorderRadius::all(Val::Px(15.)),
            ))
            .with_children(|board| {
                // Small stars and lane glows make the playfield feel like an arcade cabinet.
                for i in 0..31 {
                    let x = ((i * 173 + 39) % 850 + 24) as f32;
                    let y = ((i * 97 + 31) % 420 + 25) as f32;
                    place(
                        board,
                        x,
                        y,
                        2.5,
                        2.5,
                        scale,
                        Color::srgba(0.65, 0.85, 1., 0.29),
                        2.,
                    );
                }
                place(
                    board,
                    0.,
                    478.,
                    FIELD_W,
                    22.,
                    scale,
                    Color::srgb(0.10, 0.22, 0.34),
                    0.,
                );
                for brick in game.bricks.iter().filter(|brick| brick.hp > 0) {
                    let color = if brick.flash > 0. {
                        Color::WHITE
                    } else if brick.boss {
                        Color::srgb(0.47, 0.30, 0.72)
                    } else {
                        BreakoutGame::brick_color(brick.hue)
                    };
                    let x = brick.pos.x * scale;
                    let y = brick.pos.y * scale;
                    let w = brick.size.x * scale;
                    let h = brick.size.y * scale;
                    board
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(x),
                                top: Val::Px(y),
                                width: Val::Px(w),
                                height: Val::Px(h),
                                border: UiRect::all(Val::Px(if brick.max_hp > 1 {
                                    2.
                                } else {
                                    0.
                                })),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderColor(Color::srgba(1., 1., 1., 0.68)),
                            BorderRadius::all(Val::Px(7. * scale)),
                        ))
                        .with_children(|tile| {
                            if brick.boss {
                                label(
                                    tile,
                                    art,
                                    format!("核心  ♥ {}", brick.hp),
                                    17. * scale.max(0.8),
                                    Color::WHITE,
                                );
                            }
                            tile.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(8.),
                                    right: Val::Percent(8.),
                                    top: Val::Px(4. * scale),
                                    height: Val::Px(3. * scale),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(1., 1., 1., 0.50)),
                                BorderRadius::all(Val::Px(2.)),
                            ));
                        });
                }
                for beam in &game.beams {
                    place(
                        board,
                        beam.x - 3.,
                        beam.end_y,
                        6.,
                        PADDLE_Y - beam.end_y,
                        scale,
                        Color::srgba(0.48, 0.96, 1., (beam.life / 0.15).clamp(0., 1.)),
                        3.,
                    );
                }
                for drop in &game.drops {
                    let color = drop.kind.color();
                    board
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px((drop.pos.x - 17.) * scale),
                                top: Val::Px((drop.pos.y - 17.) * scale),
                                width: Val::Px(34. * scale),
                                height: Val::Px(34. * scale),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderRadius::all(Val::Percent(50.)),
                        ))
                        .with_children(|p| {
                            label(p, art, drop.kind.label(), 17. * scale.max(0.8), NAVY)
                        });
                }
                for spark in &game.sparks {
                    let alpha = (spark.life / 0.38).clamp(0., 1.);
                    place(
                        board,
                        spark.pos.x - 3.,
                        spark.pos.y - 3.,
                        6.,
                        6.,
                        scale,
                        spark.color.with_alpha(alpha),
                        3.,
                    );
                }
                for ball in &game.balls {
                    for (index, point) in ball.trail.iter().enumerate() {
                        let alpha = (index as f32 + 1.) / (ball.trail.len() as f32 + 1.) * 0.26;
                        place(
                            board,
                            point.x - BALL_R,
                            point.y - BALL_R,
                            BALL_R * 2.,
                            BALL_R * 2.,
                            scale,
                            Color::srgba(1., 0.78, 0.37, alpha),
                            BALL_R,
                        );
                    }
                    place(
                        board,
                        ball.pos.x - 13.,
                        ball.pos.y - 13.,
                        26.,
                        26.,
                        scale,
                        Color::srgba(1., 0.68, 0.24, 0.30),
                        13.,
                    );
                    place(
                        board,
                        ball.pos.x - BALL_R,
                        ball.pos.y - BALL_R,
                        BALL_R * 2.,
                        BALL_R * 2.,
                        scale,
                        Color::srgb(1., 0.93, 0.65),
                        BALL_R,
                    );
                }
                let width = game.paddle_width();
                place(
                    board,
                    game.paddle_x - width * 0.5 - 6.,
                    PADDLE_Y - 4.,
                    width + 12.,
                    24.,
                    scale,
                    Color::srgba(0.20, 0.84, 0.96, 0.22),
                    11.,
                );
                place(
                    board,
                    game.paddle_x - width * 0.5,
                    PADDLE_Y,
                    width,
                    15.,
                    scale,
                    Color::srgb(0.27, 0.82, 0.96),
                    8.,
                );
                place(
                    board,
                    game.paddle_x - width * 0.39,
                    PADDLE_Y + 3.,
                    width * 0.78,
                    3.,
                    scale,
                    Color::srgba(1., 1., 1., 0.65),
                    2.,
                );
                if game.ready || game.paused || game.over || game.won {
                    let title = if game.won {
                        "全部通关！"
                    } else if game.over {
                        "游戏结束"
                    } else if game.paused {
                        "已暂停"
                    } else {
                        "准备发球"
                    };
                    let hint = if game.won {
                        "好厉害！再玩一次刷新纪录"
                    } else if game.over {
                        "点击重来，再试一次"
                    } else if game.paused {
                        "点继续或按 P 回到战场"
                    } else {
                        "点击棋盘或按空格开始"
                    };
                    board
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(30.),
                                top: Val::Percent(51.),
                                width: Val::Percent(40.),
                                padding: UiRect::all(Val::Px(10.)),
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.05, 0.12, 0.22, 0.78)),
                            BorderRadius::all(Val::Px(12.)),
                        ))
                        .with_children(|p| {
                            label(
                                p,
                                art,
                                title,
                                if small { 22. } else { 28. },
                                Color::srgb(1., 0.84, 0.53),
                            );
                            label(p, art, hint, if small { 12. } else { 15. }, Color::WHITE);
                        });
                }
            });
            label(
                root,
                art,
                format!(
                    "{}  ·  左右拖动挡板  ·  接住道具：宽 / 多 / 激{}",
                    game.stage_name(),
                    if game.combo >= 2 && game.combo_for > 0. {
                        format!("  ·  连击 ×{}", game.combo)
                    } else {
                        String::new()
                    }
                ),
                if small { 13. } else { 16. },
                SOFT,
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_has_bricks_and_serve_ball_tracks_paddle() {
        let mut game = BreakoutGame::new();
        assert_eq!(game.stage, 1);
        assert_eq!(game.bricks.len(), 40);
        game.move_paddle(700.);
        assert_eq!(game.balls[0].pos.x, 700.);
        game.launch();
        assert!(!game.ready);
        assert!(game.balls[0].vel.y < 0.);
    }

    #[test]
    fn missing_last_ball_costs_one_life_and_reserves() {
        let mut game = BreakoutGame::new();
        game.launch();
        game.balls[0].pos.y = FIELD_H + BALL_R + 20.;
        game.advance(0.01);
        assert_eq!(game.lives, 2);
        assert!(game.ready);
        assert_eq!(game.balls.len(), 1);
    }

    #[test]
    fn clearing_each_stage_reaches_victory() {
        let mut game = BreakoutGame::new();
        for expected in 2..=3 {
            for brick in &mut game.bricks {
                brick.hp = 0;
            }
            game.advance(0.01);
            assert_eq!(game.stage, expected);
            assert!(game.ready);
        }
        for brick in &mut game.bricks {
            brick.hp = 0;
        }
        game.advance(0.01);
        assert!(game.won);
    }

    #[test]
    fn powerups_change_paddle_and_create_multiple_balls() {
        let mut game = BreakoutGame::new();
        game.apply_bonus(BonusKind::Wide);
        assert!(game.paddle_width() > 112.);
        game.apply_bonus(BonusKind::Multi);
        assert_eq!(game.balls.len(), 3);
        game.apply_bonus(BonusKind::Laser);
        assert!(game.laser_for > 0.);
    }

    #[test]
    fn ball_breaks_a_brick_and_scores() {
        let mut game = BreakoutGame::new();
        game.launch();
        let brick = &game.bricks[34];
        game.balls[0].pos = Vec2::new(
            brick.pos.x + brick.size.x * 0.5,
            brick.pos.y + brick.size.y + 11.,
        );
        game.balls[0].vel = Vec2::new(0., -360.);
        game.advance(0.05);
        assert_eq!(game.bricks[34].hp, 0);
        assert!(game.score > 0);
        assert!(game.balls[0].vel.y > 0.);
    }

    #[test]
    fn paddle_bounces_ball_back_up() {
        let mut game = BreakoutGame::new();
        game.launch();
        game.balls[0].pos = Vec2::new(game.paddle_x + 20., PADDLE_Y - 20.);
        game.balls[0].vel = Vec2::new(0., 370.);
        game.advance(0.05);
        assert!(game.balls[0].vel.y < 0.);
        assert!(game.balls[0].vel.x > 0.);
    }
}
