use super::*;
const GOLD: Color = Color::srgb(1., 0.79, 0.28);
const DARK: Color = Color::srgb(0.035, 0.065, 0.15);
const RED: Color = Color::srgb(0.84, 0.25, 0.20);
const RATES: [u32; 5] = [3, 4, 5, 6, 10];
const NAMES: [&str; 5] = ["樱桃", "柠檬", "葡萄", "橙子", "幸运 7"];
pub(super) struct ArcadeGame {
    points: u32,
    bets: [u32; 5],
    running: bool,
    age: f32,
    clock: f32,
    head: usize,
    start: usize,
    target: usize,
    prize: u32,
    seed: u64,
    played: bool,
}
impl ArcadeGame {
    #[cfg(debug_assertions)]
    pub(super) fn qa_state(&self) -> serde_json::Value {
        serde_json::json!({"points": self.points, "bets": self.bets,
            "running": self.running, "played": self.played, "prize": self.prize})
    }
    pub(super) fn new() -> Self {
        Self {
            points: 1000,
            bets: [0; 5],
            running: false,
            age: 0.,
            clock: 0.,
            head: 0,
            start: 0,
            target: 0,
            prize: 0,
            seed: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(7)
                | 1,
            played: false,
        }
    }
    fn total(&self) -> u32 {
        self.bets.iter().sum()
    }
    pub(super) fn add(&mut self, i: usize) {
        if !self.running && i < 5 && self.total() + 10 <= self.points {
            self.bets[i] += 10;
        }
    }
    pub(super) fn clear(&mut self) {
        if !self.running {
            self.bets = [0; 5];
        }
    }
    pub(super) fn refill(&mut self) {
        if !self.running {
            self.points = self.points.saturating_add(1000).min(999999);
        }
    }
    pub(super) fn spin(&mut self) {
        if self.running || self.total() == 0 || self.total() > self.points {
            return;
        }
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.begin((self.seed % 20) as usize);
    }
    pub(super) fn begin(&mut self, target: usize) {
        if self.running || self.total() == 0 || self.total() > self.points {
            return;
        }
        self.points -= self.total();
        self.running = true;
        self.clock = 0.;
        self.prize = 0;
        self.start = self.head;
        self.target = target % 20;
        self.played = true;
    }
    pub(super) fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0., 0.1);
        self.age += dt;
        if !self.running {
            return false;
        }
        self.clock += dt;
        let t = (self.clock / 4.8).min(1.);
        let steps = 80 + (self.target + 20 - self.start) % 20;
        self.head = (self.start + (steps as f32 * (1. - (1. - t).powi(3))).floor() as usize) % 20;
        if t >= 1. {
            self.head = self.target;
            self.running = false;
            self.prize = self.bets[self.target % 5] * RATES[self.target % 5];
            self.points = self.points.saturating_add(self.prize).min(999999);
            return true;
        }
        false
    }
}
fn rect(p: &mut ChildSpawnerCommands, x: f32, y: f32, w: f32, h: f32, c: Color, r: f32) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(x),
            top: Val::Percent(y),
            width: Val::Percent(w),
            height: Val::Percent(h),
            ..default()
        },
        BackgroundColor(c),
        BorderRadius::all(Val::Px(r)),
    ));
}
fn fruit(p: &mut ChildSpawnerCommands, symbol: usize) {
    match symbol {
        0 => {
            rect(p, 32., 10., 5., 35., TEAL, 3.);
            rect(p, 55., 13., 5., 30., TEAL, 3.);
            rect(p, 32., 8., 28., 7., TEAL, 3.);
            rect(p, 12., 35., 42., 42., Color::srgb(0.91, 0.22, 0.32), 25.);
            rect(p, 45., 39., 40., 40., Color::srgb(0.98, 0.34, 0.38), 25.);
            rect(p, 22., 41., 9., 13., Color::srgb(1., 0.68, 0.66), 8.);
            rect(p, 54., 46., 8., 10., Color::srgb(1., 0.73, 0.7), 8.);
        }
        1 => {
            rect(p, 8., 29., 78., 47., GOLD, 28.);
            rect(p, 3., 47., 18., 15., GOLD, 8.);
            rect(p, 77., 42., 16., 15., GOLD, 8.);
            rect(p, 24., 38., 36., 8., Color::srgb(1., 0.94, 0.61), 5.);
            rect(p, 51., 15., 29., 14., MINT, 10.);
        }
        2 => {
            rect(p, 43., 4., 5., 20., TEAL, 3.);
            rect(p, 46., 6., 28., 13., MINT, 10.);
            for (x, y) in [
                (20., 25.),
                (45., 25.),
                (12., 44.),
                (36., 44.),
                (60., 44.),
                (25., 63.),
                (48., 63.),
                (37., 79.),
            ] {
                rect(p, x, y, 23., 23., Color::srgb(0.57, 0.36, 0.81), 14.);
                rect(p, x + 4., y + 3., 7., 6., Color::srgb(0.79, 0.64, 0.94), 5.);
            }
        }
        3 => {
            rect(p, 16., 21., 65., 65., Color::srgb(1., 0.55, 0.17), 36.);
            rect(p, 49., 7., 29., 15., MINT, 10.);
            rect(p, 43., 13., 5., 14., TEAL, 3.);
            rect(p, 26., 33., 9., 18., Color::srgb(1., 0.78, 0.4), 7.);
        }
        _ => {
            rect(p, 13., 10., 73., 78., GOLD, 18.);
            rect(p, 27., 25., 43., 12., DARK, 3.);
            rect(p, 57., 35., 12., 13., DARK, 2.);
            rect(p, 48., 46., 12., 15., DARK, 2.);
            rect(p, 39., 59., 12., 16., DARK, 2.);
            rect(p, 20., 17., 35., 4., PAPER, 3.);
        }
    }
}

#[derive(Component)]
pub(super) enum ArcadeMotion {
    Lamp(usize),
}
pub(super) fn arcade_ui(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let g = &s.arcade;
    let gold = Color::srgb(0.98, 0.72, 0.25);
    let ink = Color::srgb(0.025, 0.055, 0.085);
    let panel_color = Color::srgb(0.035, 0.11, 0.14);
    let frame = Color::srgb(0.39, 0.20, 0.13);
    let cw = (width - 340.).min(900.).max(540.);
    let ch = (height - 96.).max(400.);
    let sx = cw + 36.;
    let sw = (width - sx - 20.).max(230.);
    let tile = ((cw - 50.) / 6.).min((ch - 50.) / 6.).floor();
    let board_w = tile * 6.;
    let bx = (cw - board_w) * 0.5;
    let by = (ch - board_w) * 0.5;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(ink),
        ))
        .with_children(|root| {
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(22.),
                right: Val::Px(22.),
                top: Val::Px(12.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|header| {
                header
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|brand| {
                        label(brand, art, "经典水果机", 30., gold);
                        label(
                            brand,
                            art,
                            "复古跑灯 · 亮灯停下的水果决定结果",
                            13.,
                            Color::srgb(0.69, 0.68, 0.58),
                        );
                    });
                header.spawn(row()).with_children(|nav| {
                    button(
                        nav,
                        art,
                        "怎么玩",
                        Action::Help,
                        82.,
                        38.,
                        panel_color,
                        Color::WHITE,
                    );
                    button(
                        nav,
                        art,
                        "游戏库",
                        Action::Library,
                        86.,
                        38.,
                        panel_color,
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    top: Val::Px(72.),
                    width: Val::Px(cw),
                    height: Val::Px(ch),
                    border: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                BackgroundColor(frame),
                BorderColor(gold),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|cab| {
                cab.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(bx + tile),
                        top: Val::Px(by + tile),
                        width: Val::Px(tile * 4.),
                        height: Val::Px(tile * 4.),
                        border: UiRect::all(Val::Px(3.)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(panel_color),
                    BorderColor(gold),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|center| {
                    label(
                        center,
                        art,
                        "LUCKY",
                        if tile > 65. { 36. } else { 28. },
                        gold,
                    );
                    label(
                        center,
                        art,
                        "7",
                        if tile > 65. { 112. } else { 80. },
                        Color::srgb(1., 0.42, 0.22),
                    );
                    label(
                        center,
                        art,
                        if g.running {
                            "灯光正在转动…".to_owned()
                        } else if !g.played {
                            "选择水果，开始跑灯".to_owned()
                        } else if g.prize > 0 {
                            format!("恭喜！奖励 +{}", g.prize)
                        } else {
                            format!("停在 {}，再试一次", NAMES[g.target % 5])
                        },
                        17.,
                        gold,
                    );
                    label(
                        center,
                        art,
                        format!("本次奖励  {}", g.prize),
                        16.,
                        Color::WHITE,
                    );
                });
                for i in 0..20 {
                    let (col, row) = if i < 6 {
                        (i, 0)
                    } else if i < 10 {
                        (5, i - 5)
                    } else if i < 16 {
                        (15 - i, 5)
                    } else {
                        (0, 20 - i)
                    };
                    let active = i == g.head;
                    cab.spawn((
                        ArcadeMotion::Lamp(i),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(bx + col as f32 * tile + 2.),
                            top: Val::Px(by + row as f32 * tile + 2.),
                            width: Val::Px(tile - 4.),
                            height: Val::Px(tile - 4.),
                            border: UiRect::all(Val::Px(if active { 4. } else { 2. })),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(if active {
                            Color::srgb(0.35, 0.22, 0.10)
                        } else {
                            ink
                        }),
                        BorderColor(if active {
                            gold
                        } else {
                            Color::srgb(0.58, 0.35, 0.18)
                        }),
                        BorderRadius::all(Val::Px(7.)),
                    ))
                    .with_children(|piece| {
                        piece
                            .spawn(Node {
                                width: Val::Px(tile * 0.64),
                                height: Val::Px(tile * 0.64),
                                ..default()
                            })
                            .with_children(|icon| fruit(icon, i % 5));
                        for (x, y) in [
                            (5., 5.),
                            (tile - 13., 5.),
                            (5., tile - 13.),
                            (tile - 13., tile - 13.),
                        ] {
                            piece.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(x),
                                    top: Val::Px(y),
                                    width: Val::Px(8.),
                                    height: Val::Px(8.),
                                    ..default()
                                },
                                BackgroundColor(if active { Color::WHITE } else { gold }),
                                BorderRadius::all(Val::Px(8.)),
                            ));
                        }
                    });
                }
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(sx),
                top: Val::Px(74.),
                width: Val::Px(sw),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                ..default()
            })
            .with_children(|panel| {
                panel
                    .spawn((
                        Node {
                            width: Val::Percent(100.),
                            padding: UiRect::all(Val::Px(12.)),
                            justify_content: JustifyContent::SpaceBetween,
                            ..row()
                        },
                        BackgroundColor(panel_color),
                        BorderRadius::all(Val::Px(8.)),
                    ))
                    .with_children(|score| {
                        label(score, art, "积分", 18., gold);
                        label(score, art, g.points.to_string(), 27., gold);
                    });
                label(panel, art, "选择水果 · 每点 +10", 19., gold);
                let bet_h = ((height - 340.) / 5.).clamp(44., 60.);
                for i in 0..5 {
                    let selected = g.bets[i] > 0;
                    panel
                        .spawn((
                            Button,
                            Action::ArcadeBet(i),
                            ButtonTone(panel_color),
                            Node {
                                width: Val::Px(sw),
                                height: Val::Px(bet_h),
                                padding: UiRect::horizontal(Val::Px(9.)),
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.),
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            BackgroundColor(panel_color),
                            BorderColor(if selected {
                                gold
                            } else {
                                Color::srgb(0.28, 0.31, 0.28)
                            }),
                            BorderRadius::all(Val::Px(7.)),
                        ))
                        .with_children(|entry| {
                            entry
                                .spawn(Node {
                                    width: Val::Px(34.),
                                    height: Val::Px(34.),
                                    ..default()
                                })
                                .with_children(|icon| fruit(icon, i));
                            label(
                                entry,
                                art,
                                format!("{} ×{}  投入 {}", NAMES[i], RATES[i], g.bets[i]),
                                14.,
                                Color::WHITE,
                            );
                            label(entry, art, "＋", 22., gold);
                        });
                }
                label(panel, art, format!("本次投入  {} 点", g.total()), 16., gold);
                panel.spawn(row()).with_children(|actions| {
                    button(
                        actions,
                        art,
                        "清空",
                        Action::ArcadeClear,
                        (sw - 8.) * 0.35,
                        43.,
                        RED,
                        Color::WHITE,
                    );
                    button(
                        actions,
                        art,
                        if g.running {
                            "跑灯中…"
                        } else if g.total() == 0 {
                            "先选水果"
                        } else {
                            "开始跑灯"
                        },
                        Action::ArcadeSpin,
                        (sw - 8.) * 0.65,
                        43.,
                        Color::srgb(0.14, 0.57, 0.34),
                        Color::WHITE,
                    );
                });
                button(
                    panel,
                    art,
                    "免费补充 +1000",
                    Action::ArcadeRefill,
                    sw,
                    38.,
                    Color::srgb(0.14, 0.33, 0.62),
                    Color::WHITE,
                );
                label(
                    panel,
                    art,
                    "仅使用免费虚拟积分",
                    12.,
                    Color::srgb(0.62, 0.63, 0.59),
                );
            });
        });
}

pub(super) fn animate_arcade(
    s: Res<Session>,
    mut nodes: Query<(
        &ArcadeMotion,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Node,
    )>,
) {
    if s.mode != GameMode::FruitArcade || s.library_open {
        return;
    }
    for (motion, mut bg, mut border, mut node) in &mut nodes {
        let ArcadeMotion::Lamp(i) = *motion;
        let active = i == s.arcade.head;
        bg.0 = if active {
            Color::srgb(0.35, 0.22, 0.10)
        } else {
            Color::srgb(0.025, 0.055, 0.085)
        };
        border.0 = if active {
            Color::srgb(0.98, 0.72, 0.25)
        } else {
            Color::srgb(0.58, 0.35, 0.18)
        };
        node.border = UiRect::all(Val::Px(if active { 4. } else { 2. }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settles_once_and_locks_controls() {
        let mut g = ArcadeGame::new();
        g.add(4);
        g.begin(9);
        assert_eq!(g.points, 990);
        g.spin();
        g.add(1);
        g.clear();
        g.refill();
        assert_eq!(g.total(), 10);
        for _ in 0..60 {
            g.advance(0.1);
        }
        assert_eq!(g.head, 9);
        assert_eq!(g.points, 1090);
        for _ in 0..60 {
            g.advance(0.1);
        }
        assert_eq!(g.points, 1090);
    }
    #[test]
    fn all_positions_pay_correct_symbol() {
        for i in 0..20 {
            let mut g = ArcadeGame::new();
            g.bets = [10, 20, 30, 40, 50];
            g.begin(i);
            for _ in 0..60 {
                g.advance(0.1);
            }
            assert_eq!(g.head, i);
            assert_eq!(g.points, 850 + g.bets[i % 5] * RATES[i % 5]);
        }
    }
    #[test]
    fn cannot_overspend_or_start_empty() {
        let mut g = ArcadeGame::new();
        g.spin();
        assert!(!g.running);
        for _ in 0..110 {
            g.add(0);
        }
        assert_eq!(g.total(), 1000);
        g.begin(1);
        for _ in 0..60 {
            g.advance(0.1);
        }
        assert_eq!(g.points, 0);
        g.spin();
        assert!(!g.running);
        g.clear();
        g.refill();
        assert_eq!(g.points, 1000);
    }
}
