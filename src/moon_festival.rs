use super::*;

const RED: Color = Color::srgb(0.78, 0.08, 0.10);
const GOLD: Color = Color::srgb(1.00, 0.77, 0.30);
const CREAM: Color = Color::srgb(1.00, 0.94, 0.78);

pub(super) struct MoonFestivalGame {
    dice: [u8; 6],
    pub(super) score: u32,
    pub(super) rolls_left: u8,
    rolling: bool,
    roll_clock: f32,
    sparkle: f32,
    seed: u32,
    collection: [u8; 3],
    feedback: String,
}

impl MoonFestivalGame {
    pub(super) fn new() -> Self {
        Self {
            dice: [1, 4, 2, 5, 2, 3],
            score: 0,
            rolls_left: 5,
            rolling: false,
            roll_clock: 0.,
            sparkle: 0.,
            seed: 0xC0FF_EE12,
            collection: [0; 3],
            feedback: "点「摇一摇」，让六颗骰子落进博饼盘！".into(),
        }
    }

    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    fn next(&mut self) -> u32 {
        self.seed = self
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        self.seed
    }

    pub(super) fn shake(&mut self) {
        if self.rolling {
            return;
        }
        if self.rolls_left == 0 {
            self.feedback = "今天的博饼次数用完啦，点「再来一局」开启新的免费局。".into();
            return;
        }
        self.rolling = true;
        self.roll_clock = 0.;
        self.feedback = "月兔摇呀摇，福气要落下来啦…".into();
    }

    fn settle(&mut self) {
        let mut dice = [0; 6];
        for die in &mut dice {
            *die = (self.next() % 6 + 1) as u8;
        }
        self.dice = dice;
        self.rolls_left -= 1;
        let red_fours = dice.iter().filter(|&&die| die == 4).count();
        let same = dice.iter().filter(|&&die| die == dice[0]).count();
        let (title, reward, badge) = if same == 6 {
            ("六杯满福", 88, 2)
        } else if red_fours >= 4 {
            ("状元福", 66, 2)
        } else if red_fours == 3 {
            ("三红福", 36, 1)
        } else if red_fours == 2 {
            ("双福临门", 22, 1)
        } else if red_fours == 1 {
            ("一秀", 10, 0)
        } else {
            ("月光小福", 4, 0)
        };
        self.score += reward;
        self.collection[badge] = self.collection[badge].saturating_add(1);
        self.sparkle = if reward >= 36 { 0.8 } else { 0.38 };
        self.feedback = if self.rolls_left == 0 {
            format!("{}！+{} 福气 · 本局完成", title, reward)
        } else {
            format!(
                "{}！+{} 福气 · 还可摇 {} 次",
                title, reward, self.rolls_left
            )
        };
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0., 0.05);
        self.sparkle = (self.sparkle - dt).max(0.);
        if !self.rolling {
            return self.sparkle > 0.;
        }
        self.roll_clock += dt;
        // Rapid face changes sell the shake before the deterministic result is
        // revealed. It is an offline toy with no purchase or redemption path.
        for index in 0..self.dice.len() {
            let random = self.next();
            self.dice[index] = ((random >> 9) % 6 + 1) as u8;
        }
        if self.roll_clock >= 0.82 {
            self.rolling = false;
            self.settle();
        }
        true
    }
}

pub(super) fn moon_festival_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let game = &s.moon;
    let die_size = if compact { 52. } else { 62. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.44, 0.30, 0.58)),
        ))
        .with_children(|root| {
            // A moonlit paper-scroll backdrop, deliberately original rather
            // than a reproduction of any event page or branded reward list.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(-18.),
                    top: Val::Percent(-3.),
                    width: Val::Percent(78.),
                    height: Val::Percent(34.),
                    ..default()
                },
                BackgroundColor(Color::srgba(1., 0.91, 0.62, 0.26)),
                BorderRadius::all(Val::Percent(50.)),
            ));
            for (left, top, diameter) in [
                (9., 22., 13.),
                (82., 27., 8.),
                (77., 65., 5.),
                (17., 70., 6.),
            ] {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(left),
                        top: Val::Percent(top),
                        width: Val::Px(diameter),
                        height: Val::Px(diameter),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1., 0.93, 0.65, 0.54)),
                    BorderRadius::all(Val::Px(diameter)),
                ));
            }
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(if compact { 16. } else { 28. }),
                    right: Val::Px(if compact { 16. } else { 28. }),
                    top: Val::Px(if compact { 12. } else { 18. }),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                },
                ZIndex(10),
            ))
            .with_children(|top| {
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "中秋博饼会", if compact { 23. } else { 29. }, CREAM);
                    label(
                        p,
                        art,
                        "免费趣味局 · 收集月兔福礼",
                        if compact { 12. } else { 15. },
                        Color::srgb(1., 0.84, 0.50),
                    );
                });
                top.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "?",
                        Action::Help,
                        38.,
                        36.,
                        Color::srgba(0.38, 0.18, 0.30, 0.72),
                        CREAM,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        if compact { 70. } else { 86. },
                        36.,
                        Color::srgba(0.38, 0.18, 0.30, 0.72),
                        CREAM,
                    );
                });
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(11.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..row()
            })
            .with_children(|p| {
                label(
                    p,
                    art,
                    "月圆摇骰 · 福气满盘",
                    if compact { 18. } else { 22. },
                    CREAM,
                );
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(7.),
                    top: Val::Percent(17.),
                    width: Val::Percent(86.),
                    height: Val::Percent(38.),
                    border: UiRect::all(Val::Px(if compact { 7. } else { 10. })),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.62, 0.035, 0.06)),
                BorderColor(GOLD),
                BorderRadius::all(Val::Percent(50.)),
            ))
            .with_children(|tray| {
                tray.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(5.),
                        top: Val::Percent(7.),
                        width: Val::Percent(90.),
                        height: Val::Percent(86.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    BorderColor(Color::srgba(1., 0.84, 0.36, 0.78)),
                    BorderRadius::all(Val::Percent(50.)),
                ));
                for (index, (left, top)) in [
                    (38., 19.),
                    (57., 20.),
                    (22., 48.),
                    (43., 49.),
                    (64., 46.),
                    (50., 68.),
                ]
                .into_iter()
                .enumerate()
                {
                    let wobble_x = if game.rolling {
                        (game.roll_clock * 22. + index as f32).sin() * 4.
                    } else {
                        0.
                    };
                    let wobble_y = if game.rolling {
                        (game.roll_clock * 18. + index as f32 * 2.).cos() * 3.
                    } else {
                        0.
                    };
                    tray.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(left),
                            top: Val::Percent(top),
                            width: Val::Px(die_size),
                            height: Val::Px(die_size),
                            margin: UiRect {
                                left: Val::Px(wobble_x),
                                top: Val::Px(wobble_y),
                                ..default()
                            },
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.98, 0.96, 0.86)),
                        BorderColor(Color::srgba(1., 1., 1., 0.82)),
                        BorderRadius::all(Val::Px(10.)),
                    ))
                    .with_children(|die| die_sprite(die, game.dice[index], die_size));
                }
            });
            if game.sparkle > 0. {
                let alpha = (game.sparkle / 0.8).min(1.) * 0.26;
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1., 0.78, 0.31, alpha)),
                ));
            }
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(57.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..row()
            })
            .with_children(|p| {
                label(
                    p,
                    art,
                    &game.feedback,
                    if compact { 14. } else { 17. },
                    CREAM,
                );
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(61.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..row()
            })
            .with_children(|p| {
                button(
                    p,
                    art,
                    if game.rolling {
                        "福气摇动中…"
                    } else if game.rolls_left > 0 {
                        "摇 一 摇"
                    } else {
                        "今日已摇完"
                    },
                    Action::MoonShake,
                    if compact { 214. } else { 248. },
                    if compact { 62. } else { 70. },
                    Color::srgb(0.91, 0.20, 0.08),
                    CREAM,
                );
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(7.),
                    top: Val::Percent(71.),
                    width: Val::Percent(86.),
                    height: Val::Percent(22.),
                    padding: UiRect::all(Val::Px(if compact { 10. } else { 14. })),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(7.),
                    ..default()
                },
                BackgroundColor(Color::srgba(1., 0.91, 0.70, 0.94)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|album| {
                album
                    .spawn(Node {
                        justify_content: JustifyContent::SpaceBetween,
                        ..row()
                    })
                    .with_children(|head| {
                        label(
                            head,
                            art,
                            format!("月兔福礼册 · 福气 {}", game.score),
                            if compact { 16. } else { 19. },
                            RED,
                        );
                        button(
                            head,
                            art,
                            "再来一局",
                            Action::MoonReset,
                            82.,
                            30.,
                            RED,
                            CREAM,
                        );
                    });
                album
                    .spawn(Node {
                        justify_content: JustifyContent::SpaceBetween,
                        column_gap: Val::Px(8.),
                        ..row()
                    })
                    .with_children(|cards| {
                        for (index, (name, color)) in [
                            ("桂花签", Color::srgb(0.92, 0.54, 0.18)),
                            ("月兔灯", Color::srgb(0.79, 0.40, 0.72)),
                            ("团圆章", Color::srgb(0.33, 0.60, 0.76)),
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            cards
                                .spawn((
                                    Node {
                                        flex_grow: 1.,
                                        height: Val::Px(if compact { 58. } else { 68. }),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(
                                        color.to_srgba().red,
                                        color.to_srgba().green,
                                        color.to_srgba().blue,
                                        0.20,
                                    )),
                                    BorderRadius::all(Val::Px(11.)),
                                ))
                                .with_children(|card| {
                                    label(card, art, name, if compact { 13. } else { 15. }, RED);
                                    label(
                                        card,
                                        art,
                                        format!("收集 ×{}", game.collection[index]),
                                        if compact { 11. } else { 13. },
                                        Color::srgb(0.46, 0.25, 0.20),
                                    );
                                });
                        }
                    });
                label(
                    album,
                    art,
                    "仅为本地趣味收藏，不含兑换或现实奖品",
                    if compact { 10. } else { 12. },
                    Color::srgb(0.52, 0.35, 0.30),
                );
            });
        });
}

fn die_sprite(parent: &mut ChildSpawnerCommands, value: u8, size: f32) {
    let pips = match value {
        1 => &[(50., 50.)][..],
        2 => &[(27., 27.), (73., 73.)][..],
        3 => &[(27., 27.), (50., 50.), (73., 73.)][..],
        4 => &[(27., 27.), (73., 27.), (27., 73.), (73., 73.)][..],
        5 => &[(27., 27.), (73., 27.), (50., 50.), (27., 73.), (73., 73.)][..],
        _ => &[
            (27., 23.),
            (73., 23.),
            (27., 50.),
            (73., 50.),
            (27., 77.),
            (73., 77.),
        ][..],
    };
    for &(left, top) in pips {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(left - 8.),
                top: Val::Percent(top - 8.),
                width: Val::Px(size * 0.16),
                height: Val::Px(size * 0.16),
                ..default()
            },
            BackgroundColor(if value == 4 {
                RED
            } else {
                Color::srgb(0.12, 0.13, 0.17)
            }),
            BorderRadius::all(Val::Px(size)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shake_spends_one_free_turn_and_awards_a_virtual_collectible() {
        let mut game = MoonFestivalGame::new();
        game.shake();
        for _ in 0..20 {
            game.advance(0.05);
        }
        assert!(!game.rolling);
        assert_eq!(game.rolls_left, 4);
        assert!(game.score >= 4);
        assert_eq!(game.collection.iter().sum::<u8>(), 1);
    }

    #[test]
    fn finished_day_does_not_start_another_roll() {
        let mut game = MoonFestivalGame::new();
        game.rolls_left = 0;
        game.shake();
        assert!(!game.rolling);
        assert_eq!(game.score, 0);
    }
}
