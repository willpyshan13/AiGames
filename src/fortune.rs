//! A small, animated property board game. Animation phases also gate input.
use super::*;

const COLS: usize = 11;
const ROWS: usize = 11;
const N: usize = 40;
const PLAYER_COUNT: usize = 4;
const ROUNDS: usize = 12;
const BOARD_WIDTH: f32 = 1248.;
const BOARD_HEIGHT: f32 = 1248.;
const BOARD_STEP_X: f32 = 95.;
const BOARD_STEP_Y: f32 = 84.;
const BOARD_ORIGIN_X: f32 = 150.;
const BOARD_ORIGIN_Y: f32 = 160.;
const TILE_W: f32 = 86.;
const TILE_H: f32 = 82.;
const BLUE: Color = Color::srgb(0.20, 0.53, 0.89);
const CORAL: Color = Color::srgb(0.95, 0.43, 0.31);
const GOLD: Color = Color::srgb(1., 0.75, 0.23);
const PLAYER_NAMES: [&str; PLAYER_COUNT] = ["小兔", "小狐狸", "栗子熊", "小企鹅"];
const PLAYER_COLORS: [Color; PLAYER_COUNT] = [
    Color::srgb(0.20, 0.53, 0.89),
    Color::srgb(0.95, 0.43, 0.31),
    Color::srgb(0.92, 0.65, 0.22),
    Color::srgb(0.56, 0.43, 0.82),
];
const NAMES: [&str; N] = [
    "起点",
    "面包屋",
    "花花店",
    "果汁铺",
    "幸运卡",
    "云朵街",
    "海风街",
    "糖果屋",
    "休息站",
    "书香里",
    "奖金站",
    "月亮湾",
    "奶茶铺",
    "彩虹路",
    "小公园",
    "玩具屋",
    "森林街",
    "幸运卡",
    "冰淇淋",
    "向日葵",
    "海港站",
    "贝壳街",
    "奖金站",
    "棉花糖",
    "休息站",
    "星星屋",
    "海边公园",
    "幸运卡",
    "风车街",
    "灯塔路",
    "集市站",
    "猫咪咖啡",
    "休息站",
    "椰子岛",
    "奖金站",
    "糖果街",
    "幸运卡",
    "游乐园",
    "休息站",
    "月光码头",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Phase {
    Ready,
    Rolling,
    Moving,
    Choice,
    Effect,
    BotWait,
    Finished,
}

pub(super) struct FortuneGame {
    pan: Vec2,
    follow: bool,
    preview_autoplay: bool,
    pub(super) phase: Phase,
    pub(super) cash: [i32; PLAYER_COUNT],
    positions: [usize; PLAYER_COUNT],
    owners: [Option<usize>; N],
    levels: [u8; N],
    active: usize,
    turns: usize,
    timer: f32,
    steps: u8,
    die: u8,
    seed: u64,
    age: f32,
    coin_age: f32,
    coin_delta: i32,
    build_tile: Option<usize>,
    build_age: f32,
    message: String,
}

pub(super) fn point(i: usize) -> Vec2 {
    match i % N {
        0..=9 => Vec2::new(i as f32, 0.),
        10..=19 => Vec2::new((COLS - 1) as f32, (i % N - 10) as f32),
        20..=29 => Vec2::new((30 - i % N) as f32, (ROWS - 1) as f32),
        _ => Vec2::new(0., (N - i % N) as f32),
    }
}
fn board_px(point: Vec2) -> Vec2 {
    Vec2::new(
        BOARD_ORIGIN_X + point.x * BOARD_STEP_X,
        BOARD_ORIGIN_Y + (ROWS as f32 - 1. - point.y) * BOARD_STEP_Y,
    )
}
fn property(i: usize) -> bool {
    ![0, 4, 8, 10, 12, 14, 16, 20, 22, 24, 26, 28, 32, 34, 36, 38].contains(&i)
}
fn price(i: usize) -> i32 {
    120 + (i % 4) as i32 * 40
}
fn rent(level: u8) -> i32 {
    [40, 80, 140][level as usize]
}

impl FortuneGame {
    pub(super) fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(7421)
            | 1;
        Self {
            pan: Vec2::ZERO,
            follow: true,
            preview_autoplay: false,
            phase: Phase::Ready,
            cash: [1200; PLAYER_COUNT],
            positions: [0; PLAYER_COUNT],
            owners: [None; N],
            levels: [0; N],
            active: 0,
            turns: 0,
            timer: 0.,
            steps: 0,
            die: 1,
            seed,
            age: 0.,
            coin_age: 5.,
            coin_delta: 0,
            build_tile: None,
            build_age: 5.,
            message: "掷骰子，让小兔出发！".into(),
        }
    }
    // Only used by explicit screenshot previews; normal games always start empty.
    pub(super) fn prepare_preview(&mut self, scene: &str) {
        self.seed = 42;
        for (tile, owner, level) in [
            (1, 0, 0),
            (5, 1, 1),
            (8, 2, 2),
            (12, 3, 0),
            (16, 0, 1),
            (19, 1, 2),
        ] {
            self.owners[tile] = Some(owner);
            self.levels[tile] = level;
        }
        self.positions = [2, 12, 26, 35];
        match scene {
            "demo" => {
                self.preview_autoplay = true;
                self.roll();
            }
            "roll" => self.roll(),
            "build" => {
                self.positions[0] = 3;
                self.land();
                self.choose(true);
            }
            "choice" => {
                self.positions[0] = 6;
                self.land();
            }
            "finished" => {
                self.turns = ROUNDS * PLAYER_COUNT - 1;
                self.end_turn();
            }
            _ => {}
        }
    }

    fn random(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    pub(super) fn recenter(&mut self) {
        self.follow = true;
    }

    fn start_roll(&mut self) {
        self.follow = true;
        self.die = (self.random() % 6 + 1) as u8;
        self.phase = Phase::Rolling;
        self.timer = 0.;
        self.message = if self.active == 0 {
            "骰子滚动中…".to_string()
        } else {
            format!("{}正在掷骰子…", PLAYER_NAMES[self.active])
        };
    }
    pub(super) fn roll(&mut self) {
        if self.active == 0 && self.phase == Phase::Ready {
            self.start_roll();
        }
    }
    fn effect(&mut self, message: String, delta: i32) {
        self.message = message;
        self.coin_delta = delta;
        self.coin_age = 0.;
        self.phase = Phase::Effect;
        self.timer = 0.;
    }
    fn gain(&mut self, amount: i32) {
        self.cash[self.active] += amount;
        self.coin_delta = amount;
        self.coin_age = 0.;
    }
    fn land(&mut self) {
        let tile = self.positions[self.active];
        if property(tile) {
            match self.owners[tile] {
                None => {
                    if self.cash[self.active] >= price(tile) {
                        self.phase = Phase::Choice;
                        self.timer = 0.;
                        self.message = format!("到达{}，买下来开一家小店？", NAMES[tile]);
                    } else {
                        self.effect("金币暂时不够，先逛逛吧。".into(), 0);
                    }
                }
                Some(owner) if owner == self.active => {
                    if self.levels[tile] < 2 && self.cash[self.active] >= 100 {
                        self.phase = Phase::Choice;
                        self.timer = 0.;
                        self.message = "回到自己的街区，要升级房子吗？".into();
                    } else {
                        self.effect("回到自己的房子，休息一下！".into(), 0);
                    }
                }
                Some(owner) => {
                    let amount = rent(self.levels[tile]).min(self.cash[self.active]);
                    self.cash[self.active] -= amount;
                    self.cash[owner] += amount;
                    self.effect(format!("拜访对方的小店，支付租金 {}", amount), -amount);
                }
            }
        } else if [4, 16, 28, 36].contains(&tile) {
            let event = self.random() % 3;
            let (amount, message) = match event {
                0 => (150, "幸运卡：参加市集，赚到 150 金币！"),
                1 => (100, "幸运卡：帮邻居送货，奖励 100 金币！"),
                _ => (-60, "幸运卡：修理小推车，花费 60 金币。"),
            };
            let amount = amount.max(-self.cash[self.active]);
            self.gain(amount);
            self.effect(message.into(), amount);
        } else if [10, 22, 34].contains(&tile) {
            self.gain(120);
            self.effect("奖金站！领取 120 金币。".into(), 120);
        } else if [14, 26].contains(&tile) {
            self.gain(50);
            self.effect("公园野餐！奖励 50 金币。".into(), 50);
        } else if [8, 24, 30, 32, 38].contains(&tile) {
            self.effect("在小镇歇一会儿，保存体力。".into(), 0);
        } else if [12, 20].contains(&tile) {
            let amount = 40.min(self.cash[self.active]);
            self.gain(-amount);
            self.effect("支付码头维护费。".into(), -amount);
        } else if tile == 0 {
            self.effect("回到起点，继续探索小镇！".into(), 0);
        } else {
            self.effect("绕城一圈，领取 200 金币！".into(), 200);
        }
    }
    pub(super) fn choose(&mut self, buy: bool) {
        if self.phase != Phase::Choice {
            return;
        }
        let tile = self.positions[self.active];
        let upgrade = self.owners[tile] == Some(self.active);
        let cost = if upgrade { 100 } else { price(tile) };
        if buy
            && property(tile)
            && self.cash[self.active] >= cost
            && (self.owners[tile].is_none() || (upgrade && self.levels[tile] < 2))
        {
            self.cash[self.active] -= cost;
            if upgrade {
                self.levels[tile] += 1;
            } else {
                self.owners[tile] = Some(self.active);
            }
            self.build_tile = Some(tile);
            self.build_age = 0.;
            self.effect(
                if upgrade {
                    "新楼层盖好啦，租金也提高了！".into()
                } else {
                    format!("{}开业！路过的对手会付租金。", NAMES[tile])
                },
                -cost,
            );
        } else {
            self.effect("先留着金币，下次再来！".into(), 0);
        }
    }
    fn wealth(&self, p: usize) -> i32 {
        self.cash[p]
            + (0..N)
                .filter(|&i| self.owners[i] == Some(p))
                .map(|i| price(i) + self.levels[i] as i32 * 100)
                .sum::<i32>()
    }
    fn end_turn(&mut self) {
        self.turns += 1;
        if self.turns >= ROUNDS * PLAYER_COUNT || self.cash.iter().any(|&n| n <= 0) {
            self.phase = Phase::Finished;
            let winner = (0..PLAYER_COUNT)
                .max_by_key(|&p| self.wealth(p))
                .unwrap_or(0);
            self.message = format!("{}成为小镇首富！", PLAYER_NAMES[winner]);
        } else {
            self.active = (self.active + 1) % PLAYER_COUNT;
            self.timer = 0.;
            self.phase = if self.active == 0 {
                Phase::Ready
            } else {
                Phase::BotWait
            };
            self.message = if self.active == 0 {
                "轮到你啦，掷骰子出发！".to_string()
            } else {
                format!("轮到{}啦…", PLAYER_NAMES[self.active])
            };
        }
    }
    pub(super) fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0., 0.1);
        self.age += dt;
        self.coin_age += dt;
        self.build_age += dt;
        self.timer += dt;
        match self.phase {
            Phase::Rolling if self.timer >= 1.35 => {
                self.phase = Phase::Moving;
                self.steps = self.die;
                self.timer = 0.;
                self.message = format!("掷出 {} 点，出发！", self.die);
                return true;
            }
            Phase::Moving if self.timer >= 0.30 => {
                self.timer -= 0.30;
                self.positions[self.active] = (self.positions[self.active] + 1) % N;
                if self.positions[self.active] == 0 {
                    self.gain(200);
                }
                self.steps -= 1;
                if self.steps == 0 {
                    self.land();
                }
                return true;
            }
            Phase::Choice if (self.active != 0 || self.preview_autoplay) && self.timer >= 1.0 => {
                let tile = self.positions[self.active];
                let cost = if self.owners[tile].is_some() {
                    100
                } else {
                    price(tile)
                };
                self.choose(self.cash[self.active] - cost >= 160);
                return true;
            }
            Phase::Effect if self.timer >= 1.6 => {
                self.end_turn();
                return true;
            }
            Phase::Ready if self.preview_autoplay && self.timer >= 0.8 => {
                self.start_roll();
                return true;
            }
            Phase::BotWait if self.timer >= 0.8 => {
                self.start_roll();
                return true;
            }
            _ => {}
        }
        false
    }
    fn pawn_position(&self, p: usize) -> Vec2 {
        let here = point(self.positions[p]);
        if self.active == p && self.phase == Phase::Moving {
            let t = (self.timer / 0.30).clamp(0., 1.);
            here.lerp(point((self.positions[p] + 1) % N), t * t * (3. - 2. * t))
        } else {
            here
        }
    }
}

#[derive(Component)]
pub(super) struct FortuneViewport {
    board_size: Vec2,
}

#[derive(Component)]
pub(super) enum FortuneMotion {
    Map,
    Pawn(usize, f32),
    Die(f32, f32),
    Landing(f32),
    DieShadow(f32, f32),
    Amount(f32),
    Pip(usize),
    Coin(usize, f32),
    Building(usize, f32),
    Spark(usize, f32),
}
fn shape(p: &mut ChildSpawnerCommands, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(w),
            height: Val::Px(h),
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::all(Val::Px(radius)),
    ));
}
fn text_at(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    p.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x),
        top: Val::Px(y),
        ..default()
    })
    .with_children(|p| label(p, art, text, size, color));
}
fn fortune_roll_button(parent: &mut ChildSpawnerCommands, art: &Art) {
    let bg = Color::srgb(0.91, 0.35, 0.20);
    parent
        .spawn((
            Button,
            ButtonTone(bg),
            Action::FortuneRoll,
            Node {
                width: Val::Px(190.),
                height: Val::Px(70.),
                min_height: Val::Px(70.),
                padding: UiRect::horizontal(Val::Px(12.)),
                column_gap: Val::Px(12.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(bg),
            BorderColor(Color::srgb(0.75, 0.31, 0.19)),
            BorderRadius::all(Val::Px(16.)),
        ))
        .with_children(|button| {
            button
                .spawn((
                    Node {
                        width: Val::Px(42.),
                        height: Val::Px(42.),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                    BorderColor(Color::srgb(0.78, 0.80, 0.71)),
                    BorderRadius::all(Val::Px(9.)),
                ))
                .with_children(|die| {
                    for (x, y) in [(8., 8.), (24., 8.), (16., 16.), (8., 24.), (24., 24.)] {
                        die.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(x),
                                top: Val::Px(y),
                                width: Val::Px(7.),
                                height: Val::Px(7.),
                                ..default()
                            },
                            BackgroundColor(NAVY),
                            BorderRadius::all(Val::Px(5.)),
                        ));
                    }
                });
            label(button, art, "掷骰子", 21., Color::WHITE);
        });
}
fn pawn(p: &mut ChildSpawnerCommands, who: usize) {
    let c = PLAYER_COLORS[who % PLAYER_COUNT];
    // Wooden stand, narrow collar, and colored pawn body; matching physical
    // board-game figurines rather than the old floating animal badges.
    shape(
        p,
        4.,
        36.,
        34.,
        8.,
        Color::srgba(0.10, 0.16, 0.18, 0.38),
        4.,
    );
    shape(p, 7., 31., 28., 9., Color::srgb(0.75, 0.57, 0.35), 5.);
    shape(p, 14., 21., 14., 13., c.mix(&Color::BLACK, 0.12), 5.);
    shape(p, 10., 12., 22., 17., c, 10.);
    shape(p, 13., 5., 16., 13., c.mix(&Color::WHITE, 0.20), 8.);
    shape(p, 16., 10., 3., 4., NAVY, 2.);
    shape(p, 23., 10., 3., 4., NAVY, 2.);
    shape(p, 18., 16., 6., 2., Color::srgb(0.71, 0.30, 0.25), 1.);
}
pub(super) fn fortune_ui(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let g = &s.fortune;
    let view_width = (width - 32.).max(300.);
    let view_height = (height - 220.).max(300.);
    let board_x = 16.;
    let board_y = 92.;
    let tile = g.positions[0];
    let tile_price = price(tile);
    let tile_rent = rent(g.levels[tile]);

    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.28, 0.20, 0.14)),
        ))
        .with_children(|root| {
            // The full 3D town board is a dedicated art layer, not an enlarged park crop.
            root.spawn((
                FortuneViewport {
                    board_size: Vec2::new(BOARD_WIDTH, BOARD_HEIGHT),
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(board_x),
                    top: Val::Px(board_y),
                    width: Val::Px(view_width),
                    height: Val::Px(view_height),
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.22, 0.39, 0.39)),
                BorderColor(Color::srgb(0.77, 0.57, 0.29)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|viewport| {
                viewport
                    .spawn((
                        FortuneMotion::Map,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(-g.pan.x),
                            top: Val::Px(-g.pan.y),
                            width: Val::Px(BOARD_WIDTH),
                            height: Val::Px(BOARD_HEIGHT),
                            ..default()
                        },
                        ImageNode::new(art.fortune_board.clone()),
                    ))
                    .with_children(|board| {
                        // Live labels sit on the blank property tiles in the 3D board art.
                        for i in 0..N {
                            let center = board_px(point(i));
                            let owned = g.owners[i];
                            if property(i) {
                                text_at(
                                    board,
                                    art,
                                    NAMES[i],
                                    center.x - 32.,
                                    center.y - 8.,
                                    11.,
                                    Color::srgb(0.31, 0.23, 0.17),
                                );
                                text_at(
                                    board,
                                    art,
                                    format!("{} / {}", price(i), rent(g.levels[i])),
                                    center.x - 25.,
                                    center.y + 10.,
                                    8.,
                                    Color::srgb(0.45, 0.34, 0.24),
                                );
                                // Ownership is indicated by a small colored flag planted at
                                // the lot edge. The miniature buildings stay part of the art.
                                if let Some(owner) = owned {
                                    board.spawn((
                                        FortuneMotion::Building(i, center.y + 22.),
                                        ZIndex(4),
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(center.x - 4.),
                                            top: Val::Px(center.y + 22.),
                                            width: Val::Px(8.),
                                            height: Val::Px(18.),
                                            ..default()
                                        },
                                        BackgroundColor(PLAYER_COLORS[owner]),
                                        BorderRadius::all(Val::Px(3.)),
                                    ));
                                    shape(
                                        board,
                                        center.x + 2.,
                                        center.y + 22.,
                                        14.,
                                        7.,
                                        PLAYER_COLORS[owner],
                                        2.,
                                    );
                                }
                            } else {
                                text_at(
                                    board,
                                    art,
                                    NAMES[i],
                                    center.x - 29.,
                                    center.y - 5.,
                                    10.,
                                    Color::srgb(0.45, 0.26, 0.16),
                                );
                            }
                        }

                        // Keep the four pawns in a separate, higher layer so they read as
                        // physical game pieces sitting on the town paths and never get buried
                        // beneath the shop façades or landmark props.
                        for p in 0..PLAYER_COUNT {
                            board
                                .spawn((
                                    FortuneMotion::Pawn(p, 34.),
                                    ZIndex(8),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Px(34.),
                                        height: Val::Px(42.),
                                        ..default()
                                    },
                                ))
                                .with_children(|piece| pawn(piece, p));
                        }

                        board.spawn((
                            FortuneMotion::Landing(TILE_W),
                            ZIndex(3),
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(TILE_W),
                                height: Val::Px(TILE_H),
                                border: UiRect::all(Val::Px(3.)),
                                ..default()
                            },
                            BorderColor(PLAYER_COLORS[g.active]),
                            BorderRadius::all(Val::Px(16.)),
                        ));
                        let die = Vec2::new(BOARD_WIDTH * 0.5 - 34., BOARD_HEIGHT * 0.5 - 34.);
                        board.spawn((
                            FortuneMotion::DieShadow(die.x, die.y),
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(72.),
                                height: Val::Px(16.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.08, 0.22, 0.21, 0.28)),
                            BorderRadius::all(Val::Px(40.)),
                        ));
                        board
                            .spawn((
                                FortuneMotion::Die(die.x, die.y),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(die.x),
                                    top: Val::Px(die.y),
                                    width: Val::Px(68.),
                                    height: Val::Px(68.),
                                    border: UiRect::bottom(Val::Px(6.)),
                                    ..default()
                                },
                                BackgroundColor(Color::WHITE),
                                BorderColor(Color::srgb(0.76, 0.80, 0.71)),
                                BorderRadius::all(Val::Px(16.)),
                            ))
                            .with_children(|die| {
                                for n in 0..9 {
                                    die.spawn((
                                        FortuneMotion::Pip(n),
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(13. + (n % 3) as f32 * 16.),
                                            top: Val::Px(10. + (n / 3) as f32 * 16.),
                                            width: Val::Px(10.),
                                            height: Val::Px(10.),
                                            ..default()
                                        },
                                        BackgroundColor(NAVY),
                                        BorderRadius::all(Val::Px(8.)),
                                    ));
                                }
                            });
                        board
                            .spawn((
                                FortuneMotion::Amount(1.),
                                ZIndex(12),
                                Node {
                                    position_type: PositionType::Absolute,
                                    padding: UiRect::all(Val::Px(4.)),
                                    ..default()
                                },
                                BackgroundColor(PAPER),
                                BorderRadius::all(Val::Px(8.)),
                            ))
                            .with_children(|p| {
                                label(
                                    p,
                                    art,
                                    format!("{:+}", g.coin_delta),
                                    20.,
                                    if g.coin_delta >= 0 { TEAL } else { CORAL },
                                )
                            });
                        for i in 0..10 {
                            board.spawn((
                                FortuneMotion::Coin(i, 1.),
                                ZIndex(10),
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: Val::Px(12.),
                                    height: Val::Px(12.),
                                    ..default()
                                },
                                BackgroundColor(GOLD),
                                BorderRadius::all(Val::Px(10.)),
                            ));
                        }
                        for i in 0..18 {
                            board.spawn((
                                FortuneMotion::Spark(i, 1.),
                                ZIndex(11),
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: Val::Px(5.),
                                    height: Val::Px(8.),
                                    ..default()
                                },
                                BackgroundColor([CORAL, GOLD, BLUE, MINT][i % 4]),
                                BorderRadius::all(Val::Px(2.)),
                            ));
                        }
                    });
            });

            // Wooden title plate, matching the approved mockup.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.),
                    top: Val::Px(12.),
                    width: Val::Px(268.),
                    height: Val::Px(68.),
                    padding: UiRect::axes(Val::Px(14.), Val::Px(7.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.31, 0.18, 0.10)),
                BorderColor(Color::srgb(0.93, 0.68, 0.26)),
                BorderRadius::all(Val::Px(16.)),
            ))
            .with_children(|p| {
                label(p, art, "口袋大富翁", 27., Color::srgb(1., 0.82, 0.39));
                label(
                    p,
                    art,
                    "和好友一起 · 建造梦想小镇",
                    11.,
                    Color::srgb(1., 0.91, 0.69),
                );
            });

            // Four live player cards, with the active player highlighted.
            let card_w = ((width - 540.) / 4.).clamp(142., 184.);
            for player in 0..PLAYER_COUNT {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(190. + (PLAYER_COUNT - 1 - player) as f32 * (card_w + 8.)),
                        top: Val::Px(12.),
                        width: Val::Px(card_w),
                        height: Val::Px(68.),
                        padding: UiRect::horizontal(Val::Px(8.)),
                        column_gap: Val::Px(7.),
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(if player == g.active { 2. } else { 1. })),
                        ..row()
                    },
                    BackgroundColor(Color::srgb(0.30, 0.19, 0.12)),
                    BorderColor(if player == g.active {
                        Color::srgb(1.0, 0.77, 0.34)
                    } else {
                        Color::srgb(0.68, 0.48, 0.29)
                    }),
                    BorderRadius::all(Val::Px(14.)),
                ))
                .with_children(|card| {
                    card.spawn(Node {
                        width: Val::Px(34.),
                        height: Val::Px(40.),
                        ..default()
                    })
                    .with_children(|p| pawn(p, player));
                    card.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|p| {
                        label(
                            p,
                            art,
                            format!("{}P  {}", player + 1, PLAYER_NAMES[player]),
                            13.,
                            Color::WHITE,
                        );
                        label(
                            p,
                            art,
                            format!("金币 {}", g.cash[player]),
                            11.,
                            Color::srgb(1., 0.79, 0.32),
                        );
                        label(
                            p,
                            art,
                            format!(
                                "地产 {}",
                                g.owners
                                    .iter()
                                    .filter(|&&owner| owner == Some(player))
                                    .count()
                            ),
                            9.,
                            Color::srgb(0.75, 0.84, 0.82),
                        );
                    });
                });
            }
            root.spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(18.),
                top: Val::Px(18.),
                ..default()
            })
            .with_children(|p| {
                p.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "?",
                        Action::Help,
                        42.,
                        42.,
                        Color::srgb(0.20, 0.33, 0.38),
                        Color::WHITE,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        82.,
                        42.,
                        Color::srgb(0.10, 0.18, 0.24),
                        Color::WHITE,
                    );
                });
            });

            // Pinned property card follows the human pawn and shows live economics.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    bottom: Val::Px(14.),
                    width: Val::Px(252.),
                    height: Val::Px(154.),
                    padding: UiRect::all(Val::Px(12.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.98, 0.91, 0.73, 0.97)),
                BorderColor(Color::srgb(0.72, 0.51, 0.28)),
                BorderRadius::all(Val::Px(16.)),
            ))
            .with_children(|p| {
                label(
                    p,
                    art,
                    format!("{:02}  {}", tile + 1, NAMES[tile]),
                    17.,
                    Color::srgb(0.32, 0.20, 0.13),
                );
                if property(tile) {
                    label(
                        p,
                        art,
                        format!(
                            "{}",
                            if g.owners[tile] == Some(0) {
                                "我的地产"
                            } else if g.owners[tile].is_some() {
                                "对手地产"
                            } else {
                                "可购买地产"
                            }
                        ),
                        11.,
                        if g.owners[tile] == Some(0) {
                            BLUE
                        } else {
                            TEAL
                        },
                    );
                    label(
                        p,
                        art,
                        format!("购买价格       {} 金币", tile_price),
                        12.,
                        NAVY,
                    );
                    label(
                        p,
                        art,
                        format!("当前租金       {} 金币", tile_rent),
                        12.,
                        NAVY,
                    );
                    label(p, art, "升级房屋       100 金币", 12., NAVY);
                } else {
                    label(p, art, "小镇特别地点", 12., TEAL);
                    label(p, art, &g.message, 11., NAVY);
                }
            });

            // Current turn and dice action remain outside the draggable viewport.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.),
                    bottom: Val::Px(18.),
                    width: Val::Px(508.),
                    height: Val::Px(102.),
                    margin: UiRect::left(Val::Px(-254.)),
                    padding: UiRect::all(Val::Px(12.)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(14.),
                    ..row()
                },
                BackgroundColor(Color::srgba(0.28, 0.17, 0.11, 0.96)),
                BorderColor(Color::srgb(0.82, 0.56, 0.24)),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|panel| {
                panel.spawn(row()).with_children(|p| {
                    p.spawn(Node {
                        width: Val::Px(48.),
                        height: Val::Px(50.),
                        ..default()
                    })
                    .with_children(|p| pawn(p, g.active));
                    p.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|p| {
                        label(
                            p,
                            art,
                            format!("{}的回合", PLAYER_NAMES[g.active]),
                            15.,
                            Color::WHITE,
                        );
                        label(p, art, &g.message, 10., Color::srgb(1., 0.84, 0.55));
                    });
                });
                match g.phase {
                    Phase::Ready => fortune_roll_button(panel, art),
                    Phase::Choice if g.active == 0 => {
                        let tile = g.positions[0];
                        button(
                            panel,
                            art,
                            if g.owners[tile].is_some() {
                                "升级房屋"
                            } else {
                                "买下地产"
                            },
                            Action::FortuneBuy,
                            150.,
                            58.,
                            Color::srgb(0.91, 0.35, 0.20),
                            Color::WHITE,
                        );
                        button(
                            panel,
                            art,
                            "暂不购买",
                            Action::FortuneSkip,
                            102.,
                            48.,
                            Color::WHITE,
                            MUTED,
                        );
                    }
                    Phase::Finished => button(
                        panel,
                        art,
                        "再玩一局",
                        Action::FortuneReset,
                        150.,
                        58.,
                        TEAL,
                        Color::WHITE,
                    ),
                    _ => label(
                        panel,
                        art,
                        "棋子正在小镇中移动…",
                        13.,
                        Color::srgb(1., 0.84, 0.55),
                    ),
                }
            });

            root.spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(18.),
                bottom: Val::Px(18.),
                ..default()
            })
            .with_children(|p| {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                })
                .with_children(|p| {
                    button(
                        p,
                        art,
                        "⌖ 回到棋子",
                        Action::FortuneCenter,
                        126.,
                        48.,
                        Color::srgb(0.98, 0.91, 0.73),
                        Color::srgb(0.35, 0.24, 0.16),
                    );
                    button(
                        p,
                        art,
                        "地图说明",
                        Action::Help,
                        126.,
                        42.,
                        Color::srgb(0.91, 0.78, 0.55),
                        Color::srgb(0.35, 0.24, 0.16),
                    );
                });
            });
        });
}

fn pan_limit(view: Vec2, board_size: Vec2) -> Vec2 {
    (board_size - view).max(Vec2::ZERO)
}

pub(super) fn fortune_camera(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    scale: Res<UiScale>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    viewport: Query<(&FortuneViewport, &ComputedNode, &GlobalTransform)>,
    mut last: Local<Option<Vec2>>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::Fortune || s.library_open || s.help_open {
        *last = None;
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((board, node, transform)) = viewport.single() else {
        return;
    };
    let board_size = board.board_size;
    let factor = window.scale_factor() * scale.0;
    let view = node.size() / factor;
    if view.x <= 0. || view.y <= 0. {
        return;
    }
    let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
    let pressed =
        mouse.just_pressed(MouseButton::Left) || touches.iter_just_pressed().next().is_some();
    let held = mouse.pressed(MouseButton::Left) || touches.iter().next().is_some();
    let point = touches
        .iter()
        .next()
        .map(|t| t.position())
        .or_else(|| window.cursor_position())
        .map(|p| p * window.scale_factor());
    let cancelled = touches.iter_just_canceled().next().is_some();
    if !held || cancelled {
        *last = None;
    } else if let Some(point) = point {
        if pressed && rect.contains(point) {
            *last = Some(point);
            s.fortune.follow = false;
        }
        if let Some(old) = *last {
            s.fortune.pan = (s.fortune.pan - (point - old) / factor)
                .clamp(Vec2::ZERO, pan_limit(view, board_size));
            *last = Some(point);
        }
    }
    let g = &mut s.fortune;
    if g.follow && last.is_none() {
        let target = (board_px(g.pawn_position(g.active)) - view * 0.5)
            .clamp(Vec2::ZERO, pan_limit(view, board_size));
        g.pan = g.pan.lerp(target, 1. - (-time.delta_secs() * 10.).exp());
    }
    g.pan = g.pan.clamp(Vec2::ZERO, pan_limit(view, board_size));
}

fn dice_width(g: &FortuneGame) -> f32 {
    if g.phase != Phase::Rolling || g.timer >= 1.05 {
        return 68.;
    }
    let t = (g.timer / 1.05).clamp(0., 1.);
    68. * (0.26
        + 0.74
            * ((1. - (1. - t).powi(2)) * std::f32::consts::PI * 4.)
                .cos()
                .abs())
}
fn bounce_out(t: f32) -> f32 {
    let n = 7.5625;
    let d = 2.75;
    if t < 1. / d {
        n * t * t
    } else if t < 2. / d {
        let t = t - 1.5 / d;
        n * t * t + 0.75
    } else if t < 2.5 / d {
        let t = t - 2.25 / d;
        n * t * t + 0.9375
    } else {
        let t = t - 2.625 / d;
        n * t * t + 0.984375
    }
}

pub(super) fn animate_fortune(
    s: Res<Session>,
    mut nodes: Query<(&FortuneMotion, &mut Node, Option<&mut BackgroundColor>)>,
) {
    // Pausing freezes game age in tick; still lay out freshly rebuilt UI nodes.
    if s.mode != GameMode::Fortune || s.library_open {
        return;
    }
    let g = &s.fortune;
    for (motion, mut node, color) in &mut nodes {
        match *motion {
            FortuneMotion::Map => {
                node.left = Val::Px(-g.pan.x);
                node.top = Val::Px(-g.pan.y);
            }
            FortuneMotion::Pawn(p, size) => {
                let pos = board_px(g.pawn_position(p));
                let offset = match p {
                    0 => Vec2::new(-16., -13.),
                    1 => Vec2::new(16., -13.),
                    2 => Vec2::new(-16., 13.),
                    _ => Vec2::new(16., 13.),
                };
                let hop = if p == g.active && g.phase == Phase::Moving {
                    (g.timer / 0.30 * std::f32::consts::PI).sin().abs() * 15.
                } else {
                    (g.age * 2. + p as f32).sin() * 1.5
                };
                node.left = Val::Px(pos.x + offset.x - size * 0.5);
                node.top = Val::Px(pos.y + offset.y - size * 0.70 - hop);
            }
            FortuneMotion::Die(x, y) => {
                let t = (g.timer / 1.05).clamp(0., 1.);
                let rolling = g.phase == Phase::Rolling;
                let damp = if rolling { 1. - t } else { 0. };
                node.width = Val::Px(dice_width(g));
                node.left = Val::Px(x + (68. - dice_width(g)) * 0.5 + (t * 22.).sin() * 20. * damp);
                node.top = Val::Px(y - (t * std::f32::consts::PI * 3.).sin().abs() * 34. * damp);
            }
            FortuneMotion::DieShadow(x, y) => {
                let t = (g.timer / 1.05).clamp(0., 1.);
                let lift = if g.phase == Phase::Rolling {
                    (t * std::f32::consts::PI * 3.).sin().abs() * (1. - t)
                } else {
                    0.
                };
                node.left = Val::Px(x + lift * 10.);
                node.top = Val::Px(y + 65.);
                node.width = Val::Px(68. - lift * 20.);
            }
            FortuneMotion::Landing(cell) => {
                let pos = board_px(point(g.positions[g.active]));
                let t = (g.timer / 0.6).clamp(0., 1.);
                let spread = if g.phase == Phase::Moving {
                    0.
                } else {
                    (1. - t) * 7.
                };
                node.left = Val::Px(pos.x - TILE_W * 0.5 - spread);
                node.top = Val::Px(pos.y - TILE_H * 0.5 - spread);
                node.width = Val::Px(cell + spread * 2.);
                node.height = Val::Px(TILE_H + spread * 2.);
            }
            FortuneMotion::Amount(_cell) => {
                let pos = board_px(point(g.positions[g.active]));
                node.display = if g.coin_delta != 0 && g.coin_age < 1.35 {
                    Display::Flex
                } else {
                    Display::None
                };
                node.left = Val::Px(pos.x - 18.);
                node.top = Val::Px(pos.y - 28. - g.coin_age * 24.);
            }
            FortuneMotion::Pip(n) => {
                let face = if g.phase == Phase::Rolling && g.timer < 1.05 {
                    ((g.timer * 18.) as u8 % 6) + 1
                } else {
                    g.die
                };
                let w = dice_width(g);
                let scale = w / 68.;
                node.left = Val::Px((13. + (n % 3) as f32 * 16.) * scale);
                node.width = Val::Px(10. * scale);
                let on = match face {
                    1 => n == 4,
                    2 => [0, 8].contains(&n),
                    3 => [0, 4, 8].contains(&n),
                    4 => [0, 2, 6, 8].contains(&n),
                    5 => [0, 2, 4, 6, 8].contains(&n),
                    _ => [0, 2, 3, 5, 6, 8].contains(&n),
                };
                node.display = if on { Display::Flex } else { Display::None };
            }
            FortuneMotion::Coin(i, _cell) => {
                let t = ((g.coin_age - i as f32 * 0.035) / 0.85).clamp(0., 1.);
                node.display = if g.coin_delta != 0 && t > 0. && t < 1. {
                    Display::Flex
                } else {
                    Display::None
                };
                let center = Vec2::new(BOARD_WIDTH * 0.5, BOARD_HEIGHT * 0.5);
                let target = board_px(point(g.positions[g.active]));
                let (a, b) = if g.coin_delta > 0 {
                    (center, target)
                } else {
                    (target, center)
                };
                let pos = a.lerp(b, t)
                    + Vec2::new(
                        (i as f32 - 4.) * 3.,
                        -(t * std::f32::consts::PI).sin() * 55.,
                    );
                node.width = Val::Px(3. + 9. * (g.coin_age * 19. + i as f32).cos().abs());
                if let Some(mut color) = color {
                    color.0 = GOLD.with_alpha((1. - t).min(0.25) * 4.);
                }
                node.left = Val::Px(pos.x);
                node.top = Val::Px(pos.y);
            }
            FortuneMotion::Building(i, y) => {
                node.top = Val::Px(
                    y + if g.build_tile == Some(i) {
                        (1. - bounce_out((g.build_age / 0.85).clamp(0., 1.))) * -38.
                    } else {
                        0.
                    },
                );
            }
            FortuneMotion::Spark(i, _cell) => {
                let age = if g.phase == Phase::Finished {
                    g.age % 2.0
                } else {
                    g.build_age
                };
                node.display =
                    if age < 1.4 && (g.build_tile.is_some() || g.phase == Phase::Finished) {
                        Display::Flex
                    } else {
                        Display::None
                    };
                let center = if g.phase == Phase::Finished {
                    Vec2::new(BOARD_WIDTH * 0.5, BOARD_HEIGHT * 0.5)
                } else {
                    board_px(point(g.build_tile.unwrap_or(0)))
                };
                let angle = i as f32 * 2.399;
                let r = age * 65.;
                node.left = Val::Px(center.x + angle.cos() * r);
                node.top = Val::Px(center.y + angle.sin() * r + age * age * 30.);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn large_map_exceeds_view_and_camera_bounds_include_all_corners() {
        let view = Vec2::new(648., 460.);
        let limit = pan_limit(view, Vec2::new(BOARD_WIDTH, BOARD_HEIGHT));
        assert!(limit.x > 0. && limit.y > 0.);
        let far = board_px(point(34));
        assert!(far.x - limit.x < view.x && far.y - limit.y < view.y);
        assert_eq!(
            pan_limit(Vec2::splat(2000.), Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)),
            Vec2::ZERO
        );
    }

    #[test]
    fn complete_games_finish_with_valid_economy_and_board_state() {
        for seed in 1..=32 {
            let mut g = FortuneGame::new();
            g.seed = seed;
            for _ in 0..5000 {
                if g.phase == Phase::Ready {
                    g.roll();
                }
                if g.phase == Phase::Choice && g.active == 0 {
                    g.choose(true);
                }
                g.advance(0.1);
                assert!(g.cash.iter().all(|&c| c >= 0));
                assert!(g.positions.iter().all(|&p| p < N));
                assert!(g.levels.iter().all(|&l| l <= 2));
                if g.phase == Phase::Finished {
                    break;
                }
            }
            assert_eq!(g.phase, Phase::Finished, "seed {seed} stalled");
            assert!(g.turns <= ROUNDS * PLAYER_COUNT);
        }
    }

    #[test]
    fn board_is_a_continuous_forty_tile_loop() {
        for i in 0..N {
            assert_eq!(point(i).distance(point((i + 1) % N)), 1.);
        }
    }
    #[test]
    fn buy_upgrade_and_rent_transfer_real_money() {
        let mut g = FortuneGame::new();
        g.positions[0] = 1;
        g.land();
        g.choose(true);
        assert_eq!(g.owners[1], Some(0));
        assert_eq!(g.cash[0], 1040);
        g.land();
        g.choose(true);
        assert_eq!(g.levels[1], 1);
        assert_eq!(g.cash[0], 940);
        g.active = 1;
        g.positions[1] = 1;
        g.land();
        assert_eq!(g.cash, [1020, 1120, 1200, 1200]);
    }
    #[test]
    fn roll_cannot_be_spammed_and_movement_awards_start_once() {
        let mut g = FortuneGame::new();
        g.roll();
        let die = g.die;
        g.roll();
        assert_eq!(g.die, die);
        g.positions[0] = N - 1;
        g.phase = Phase::Moving;
        g.steps = 1;
        g.timer = 0.29;
        g.advance(0.02);
        assert_eq!(g.cash[0], 1400);
        assert_eq!(g.positions[0], 0);
        g.advance(0.1);
        assert_eq!(g.cash[0], 1400);
    }
    #[test]
    fn unaffordable_purchase_is_not_a_debt_and_max_level_is_two() {
        let mut g = FortuneGame::new();
        g.positions[0] = 1;
        g.cash[0] = 10;
        g.land();
        g.choose(true);
        assert_eq!(g.owners[1], None);
        assert_eq!(g.cash[0], 10);
        g.cash[0] = 2000;
        g.owners[1] = Some(0);
        g.levels[1] = 2;
        g.land();
        g.choose(true);
        assert_eq!(g.levels[1], 2);
    }
    #[test]
    fn computer_finishes_its_turn_without_player_input() {
        let mut g = FortuneGame::new();
        g.active = 1;
        g.phase = Phase::BotWait;
        for _ in 0..150 {
            g.advance(0.1);
            if g.active == 0 {
                break;
            }
        }
        assert_eq!(g.active, 0);
        assert_eq!(g.phase, Phase::Ready);
    }
    #[test]
    fn twelve_rounds_end_in_a_result() {
        let mut g = FortuneGame::new();
        g.turns = ROUNDS * PLAYER_COUNT - 1;
        g.active = PLAYER_COUNT - 1;
        g.end_turn();
        assert_eq!(g.phase, Phase::Finished);
    }
}
