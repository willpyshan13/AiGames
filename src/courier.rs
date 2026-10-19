//! A multi-stop route-planning adventure for young players.
use super::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(super) enum CourierStage {
    Map,
    Route(usize),
    Event(usize),
    Delivery(usize),
    Supply,
    Final,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CourierAction {
    Planet(usize),
    SafeRoute,
    ChallengeRoute,
    EventAnswer(usize),
    Delivery(usize),
    Answer(usize),
    Hint,
    Navigation,
    Repair,
    Candy,
    Refuel,
    SupplyAnswer(usize),
    Back,
    FinalAnswer(usize),
    NextChapter,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct CourierGame {
    pub(super) chapter: u32,
    pub(super) delivered: [bool; 9],
    pub(super) fuel: u8,
    pub(super) stars: u32,
    pub(super) tools: [u8; 3],
    pub(super) planet: Option<usize>,
    pub(super) stage: CourierStage,
    pub(super) feedback: String,
}

impl Default for CourierGame {
    fn default() -> Self {
        Self {
            chapter: 1,
            delivered: [false; 9],
            fuel: 6,
            stars: 0,
            tools: [1, 1, 2],
            planet: None,
            stage: CourierStage::Route(1),
            feedback: String::new(),
        }
    }
}

struct CourierQuestion {
    prompt: String,
    choices: [String; 3],
    answer: usize,
    hint: String,
}

impl CourierGame {
    pub(super) fn load() -> Self {
        let mut game: Self = fs::read(save_directory().join("courier-v1.json"))
            .ok()
            .and_then(|data| serde_json::from_slice(&data).ok())
            .unwrap_or_default();
        if matches!(game.stage, CourierStage::Map) && game.planet.is_none() && game.total() == 0 {
            game.stage = CourierStage::Route(1);
            game.feedback.clear();
        }
        game
    }

    pub(super) fn persist(&self) {
        if let Ok(data) = serde_json::to_vec(self) {
            let dir = save_directory();
            let _ =
                fs::create_dir_all(&dir).and_then(|_| fs::write(dir.join("courier-v1.json"), data));
        }
    }

    fn total(&self) -> usize {
        self.delivered.iter().filter(|v| **v).count()
    }

    fn question(&self, slot: usize) -> CourierQuestion {
        let bump = (self.chapter.saturating_sub(1) % 4) as usize;
        let math = |a: usize, b: usize, subject: &str| {
            let a = a + bump;
            let b = b + bump;
            CourierQuestion {
                prompt: format!("{subject}有 {a} 个，又收到 {b} 个，一共有多少个？"),
                choices: [
                    format!("{} 个", a + b),
                    format!("{} 个", a + b + 1),
                    format!("{} 个", a + b - 1),
                ],
                answer: 0,
                hint: format!("可以从 {a} 开始，再往后数 {b} 个。"),
            }
        };
        match slot {
            0 => math(2, 3, "月球站的包裹"),
            1 => CourierQuestion {
                prompt: "收件人说：我要蓝色、圆圆的包裹。应该送哪一个？".into(),
                choices: ["红色方盒".into(), "蓝色圆盒".into(), "黄色长盒".into()],
                answer: 1,
                hint: "再看一遍颜色和形状线索。".into(),
            },
            2 => CourierQuestion {
                prompt: "路线上的星星顺序是：黄、蓝、黄、蓝。下一个是什么颜色？".into(),
                choices: ["绿色".into(), "蓝色".into(), "黄色".into()],
                answer: 2,
                hint: "观察黄、蓝交替出现的规律。".into(),
            },
            3 => math(3, 4, "蓝莓星的果篮"),
            4 => CourierQuestion {
                prompt: "居民说包裹放在‘树旁边’。哪条线索最有用？".into(),
                choices: [
                    "找有大树的房子".into(),
                    "找最高的山".into(),
                    "找蓝色的小河".into(),
                ],
                answer: 0,
                hint: "‘旁边’表示距离很近。".into(),
            },
            5 => CourierQuestion {
                prompt: "飞船朝北，左转后会朝向哪里？".into(),
                choices: ["东".into(), "西".into(), "南".into()],
                answer: 1,
                hint: "想象自己面朝北方，左手指向哪里。".into(),
            },
            6 => math(5, 2, "糖果星的糖果"),
            7 => CourierQuestion {
                prompt: "三份礼物要送给三位居民，每人一份。还剩几份？".into(),
                choices: ["0 份".into(), "1 份".into(), "2 份".into()],
                answer: 0,
                hint: "每份都送出去，就没有剩下的了。".into(),
            },
            _ => CourierQuestion {
                prompt: "糖果屋在灯塔的东边。应该沿哪个方向找？".into(),
                choices: ["向西".into(), "向东".into(), "向南".into()],
                answer: 1,
                hint: "地图右边通常是东方。".into(),
            },
        }
    }

    pub(super) fn act(&mut self, action: CourierAction) {
        match action {
            CourierAction::Planet(p) if p < 3 => {
                if self.total() == 9 {
                    self.stage = CourierStage::Final;
                    return;
                }
                if self.planet == Some(p) {
                    self.stage = CourierStage::Map;
                    return;
                }
                self.stage = CourierStage::Route(p);
            }
            CourierAction::SafeRoute => {
                if let CourierStage::Route(p) = self.stage {
                    if self.fuel >= 1 {
                        self.fuel -= 1;
                        self.planet = Some(p);
                        self.stage = CourierStage::Map;
                        self.feedback = "安全抵达！挑一件包裹开始送吧。".into();
                    } else {
                        self.feedback = "燃料不够啦，可以去补给站补充。".into();
                    }
                }
            }
            CourierAction::ChallengeRoute => {
                if let CourierStage::Route(p) = self.stage {
                    if self.fuel >= 1 {
                        self.fuel -= 1;
                        self.stage = CourierStage::Event(p);
                        self.feedback = "挑战陨石带：算出答案就能找到星星捷径！".into();
                    } else {
                        self.feedback = "燃料不够啦，可以去补给站补充。".into();
                    }
                }
            }
            CourierAction::EventAnswer(answer) => {
                if let CourierStage::Event(p) = self.stage {
                    if answer == 1 {
                        self.stars += 2;
                        self.planet = Some(p);
                        self.stage = CourierStage::Map;
                        self.feedback = "穿越成功！找到星星补给，继续送件吧。".into();
                    } else {
                        self.feedback = "陨石带有 2 + 3 颗闪亮碎片，再算一次试试！".into();
                    }
                }
            }
            CourierAction::Delivery(slot) if slot < 9 && !self.delivered[slot] => {
                self.stage = CourierStage::Delivery(slot);
                self.feedback.clear();
            }
            CourierAction::Answer(answer) => {
                if let CourierStage::Delivery(slot) = self.stage {
                    let q = self.question(slot);
                    if answer == q.answer {
                        self.delivered[slot] = true;
                        self.stars += 1;
                        self.feedback = "送达成功！获得 1 颗星星。".into();
                        self.stage = CourierStage::Map;
                    } else {
                        self.feedback = q.hint;
                    }
                }
            }
            CourierAction::Hint => match self.stage {
                CourierStage::Delivery(slot) => self.feedback = self.question(slot).hint,
                CourierStage::Event(_) => self.feedback = "提示：把 2 和 3 合在一起数。".into(),
                _ => self.feedback = "每个星球都有三件包裹，慢慢探索吧。".into(),
            },
            CourierAction::Navigation => {
                if self.tools[0] > 0 {
                    self.tools[0] -= 1;
                    self.feedback = match self.stage {
                        CourierStage::Route(_) if self.fuel >= 2 => {
                            "导航仪建议：燃料充足，安全绕行更稳妥。".into()
                        }
                        CourierStage::Route(_) => "导航仪建议：燃料紧张，可挑战陨石带捷径。".into(),
                        CourierStage::Delivery(slot) => {
                            let q = self.question(slot);
                            format!("导航仪提示：试试「{}」。", q.choices[q.answer])
                        }
                        CourierStage::Event(_) => "导航仪找到安全缝隙：选择 5 颗碎片！".into(),
                        _ => "导航仪已标出当前最短送货路线。".into(),
                    };
                } else {
                    self.feedback = "导航仪次数用完了，可以继续观察地图寻找线索。".into();
                }
            }
            CourierAction::Repair => {
                if self.tools[1] > 0 {
                    self.tools[1] -= 1;
                    if let CourierStage::Event(p) = self.stage {
                        self.planet = Some(p);
                        self.stage = CourierStage::Map;
                        self.feedback = "修理箱帮飞船安全通过了陨石带！".into();
                    } else {
                        self.feedback = "修理箱已准备好，应对航行事件时会派上用场。".into();
                    }
                }
            }
            CourierAction::Candy => {
                if self.tools[2] > 0 && self.fuel < 8 {
                    self.tools[2] -= 1;
                    self.fuel = (self.fuel + 2).min(8);
                    self.feedback = "能量糖补充了燃料！".into();
                }
            }
            CourierAction::Refuel => self.stage = CourierStage::Supply,
            CourierAction::SupplyAnswer(answer) => {
                if answer == 1 {
                    self.fuel = (self.fuel + 3).min(8);
                    self.stage = CourierStage::Map;
                    self.feedback = "补给完成！燃料增加 3 格。".into();
                } else {
                    self.feedback = "再想想：2 + 3 等于多少？".into();
                }
            }
            CourierAction::Back => {
                self.stage = CourierStage::Map;
            }
            CourierAction::FinalAnswer(answer) => {
                if self.total() == 9 {
                    if answer == 1 {
                        self.stage = CourierStage::Complete;
                        self.stars += 3;
                        self.feedback = "终点包裹送达！本章获得 3 颗奖励星星。".into();
                    } else {
                        self.feedback = "看看蓝莓旗帜上的线索，再选择一次。".into();
                    }
                }
            }
            CourierAction::NextChapter => {
                self.chapter += 1;
                self.delivered = [false; 9];
                self.fuel = 8;
                self.tools = [1, 1, 2];
                self.planet = None;
                self.stage = CourierStage::Map;
                self.feedback = "新航线已开启，九个新任务等你完成！".into();
            }
            _ => {}
        }
    }
}

pub(super) fn courier_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    _compact: bool,
    width: f32,
    height: f32,
) {
    let g = &s.courier;
    let sx = (width / 1672.).max(0.25);
    let sy = (height / 940.).max(0.25);
    let size = sx.min(sy);

    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            ImageNode::new(art.courier_map.clone()),
        ))
        .with_children(|root| {
            let chapter_title = if g.chapter == 1 {
                "第一章：蓝莓星航线".to_owned()
            } else {
                format!("第{}章：蓝莓星航线", g.chapter)
            };
            courier_image(root, &art.courier_title, 14. * sx, 0., 506. * sx, 126. * sy);
            // Keep the approved title artwork; let its existing home area
            // return to the library instead of trapping the player here.
            root.spawn((
                Button,
                Action::Library,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(14. * sx),
                    top: Val::Px(0.),
                    width: Val::Px(506. * sx),
                    height: Val::Px(126. * sy),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                ZIndex(5),
            ));

            if g.chapter == 1 {
                courier_image(
                    root,
                    &art.courier_chapter_banner,
                    565. * sx,
                    15. * sy,
                    511. * sx,
                    88. * sy,
                );
            } else {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(576. * sx),
                        top: Val::Px(20. * sy),
                        width: Val::Px(500. * sx),
                        height: Val::Px(76. * sy),
                        padding: UiRect::horizontal(Val::Px(22. * sx)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(4. * size)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.73, 0.92, 1.)),
                    BorderColor(Color::srgb(0.24, 0.61, 0.94)),
                    BorderRadius::all(Val::Px(34. * size)),
                ))
                .with_children(|p| {
                    label(p, art, chapter_title, 26. * size, NAVY);
                });
            }

            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(22. * sx),
                    top: Val::Px(22. * sy),
                    width: Val::Px(292. * sx),
                    height: Val::Px(74. * sy),
                    padding: UiRect::horizontal(Val::Px(14. * sx)),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12. * sx),
                    border: UiRect::all(Val::Px(4. * size)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.04, 0.17, 0.42, 0.95)),
                BorderColor(Color::srgb(0.34, 0.65, 1.)),
                BorderRadius::all(Val::Px(28. * size)),
            ))
            .with_children(|p| {
                let mut parcel = ImageNode::new(art.courier_parcel.clone());
                parcel.image_mode = NodeImageMode::Stretch;
                p.spawn((
                    Node {
                        width: Val::Px(66. * sx),
                        height: Val::Px(62. * sy),
                        ..default()
                    },
                    parcel,
                ));
                label(
                    p,
                    art,
                    format!("送货 {}/9", g.total()),
                    29. * size,
                    Color::WHITE,
                );
            });

            courier_resources(root, art, g, sx, sy, size);

            for (planet, x, y, name) in [
                (0, 409., 256., "月球站"),
                (1, 1055., 220., "蓝莓星"),
                (2, 1048., 657., "糖果星"),
            ] {
                courier_planet(
                    root,
                    art,
                    g,
                    planet,
                    name,
                    x * sx,
                    y * sy,
                    218. * sx,
                    104. * sy,
                    sx,
                    sy,
                );
            }

            courier_station_button(root, art, sx, sy, size);
            courier_panel(root, art, g, sx, sy, size);

            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(468. * sx),
                    bottom: Val::Px(12. * sy),
                    width: Val::Px(738. * sx),
                    height: Val::Px(134. * sy),
                    padding: UiRect::all(Val::Px(10. * size)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(5. * sy),
                    border: UiRect::all(Val::Px(5. * size)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.34, 0.73, 0.97)),
                BorderColor(Color::srgb(0.28, 0.62, 1.)),
                BorderRadius::all(Val::Px(34. * size)),
            ))
            .with_children(|p| {
                p.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(44. * sx),
                        top: Val::Px(-16. * sy),
                        width: Val::Px(610. * sx),
                        height: Val::Px(48. * sy),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(4. * size)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.40, 0.79)),
                    BorderColor(Color::srgb(0.35, 0.69, 1.)),
                    BorderRadius::all(Val::Px(26. * size)),
                ));
                p.spawn(Node {
                    position_type: PositionType::Relative,
                    top: Val::Px(-20. * sy),
                    width: Val::Percent(100.),
                    height: Val::Px(33. * sy),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_shrink: 0.,
                    ..default()
                })
                .with_children(|heading| {
                    if g.total() == 9 {
                        label(
                            heading,
                            art,
                            "本章 9 个送货点 · 完成后挑战终点包裹  终点已开启",
                            19. * size,
                            Color::WHITE,
                        );
                    } else {
                        let mut title = ImageNode::new(art.courier_progress_title.clone());
                        title.image_mode = NodeImageMode::Stretch;
                        heading.spawn((
                            Node {
                                width: Val::Px(460. * sx),
                                height: Val::Px(36. * sy),
                                ..default()
                            },
                            title,
                        ));
                    }
                });
                p.spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Px(62. * sy),
                    position_type: PositionType::Relative,
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(10. * sx),
                            top: Val::Px(0.),
                            width: Val::Px(570. * sx),
                            height: Val::Px(62. * sy),
                            border: UiRect::all(Val::Px(3. * size)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.87, 0.95, 1.)),
                        BorderColor(Color::srgb(0.38, 0.66, 0.94)),
                        BorderRadius::all(Val::Px(20. * size)),
                    ));
                    row.spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(8. * sx),
                        ..default()
                    })
                    .with_children(|items| {
                        for delivered in g.delivered {
                            if delivered {
                                let mut parcel = ImageNode::new(art.courier_parcel_slot.clone());
                                parcel.image_mode = NodeImageMode::Stretch;
                                items.spawn((
                                    Node {
                                        width: Val::Px(50. * sx),
                                        height: Val::Px(47. * sy),
                                        flex_shrink: 0.,
                                        ..default()
                                    },
                                    parcel,
                                ));
                            } else {
                                let mut empty = ImageNode::new(art.courier_empty_slot.clone());
                                empty.image_mode = NodeImageMode::Stretch;
                                items.spawn((
                                    Node {
                                        width: Val::Px(50. * sx),
                                        height: Val::Px(47. * sy),
                                        flex_shrink: 0.,
                                        ..default()
                                    },
                                    empty,
                                ));
                            }
                        }
                        let mut gift = ImageNode::new(art.courier_gift.clone());
                        gift.image_mode = NodeImageMode::Stretch;
                        items.spawn((
                            Node {
                                width: Val::Px(108. * sx),
                                height: Val::Px(94. * sy),
                                ..default()
                            },
                            gift,
                        ));
                    });
                });
            });
        });
}

fn courier_image(
    parent: &mut ChildSpawnerCommands,
    texture: &Handle<Image>,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
) {
    let mut image = ImageNode::new(texture.clone());
    image.image_mode = NodeImageMode::Stretch;
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(left),
            top: Val::Px(top),
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
        image,
    ));
}

fn courier_resources(
    root: &mut ChildSpawnerCommands,
    art: &Art,
    g: &CourierGame,
    sx: f32,
    sy: f32,
    size: f32,
) {
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20. * sx),
            top: Val::Px(136. * sy),
            width: Val::Px(350. * sx),
            height: Val::Px(292. * sy),
            padding: UiRect::all(Val::Px(14. * size)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(7. * sy),
            border: UiRect::all(Val::Px(5. * size)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.91, 0.97, 1., 0.97)),
        BorderColor(Color::srgb(0.33, 0.72, 1.)),
        BorderRadius::all(Val::Px(24. * size)),
    ))
    .with_children(|panel| {
        panel
            .spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(7. * sx),
                top: Val::Px(8. * sy),
                width: Val::Px(25. * sx),
                height: Val::Px(25. * sy),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|star| {
                label(star, art, "★", 19. * size, Color::srgb(0.62, 0.75, 0.91));
            });
        panel
            .spawn(Node {
                width: Val::Percent(100.),
                height: Val::Px(95. * sy),
                align_items: AlignItems::Center,
                column_gap: Val::Px(10. * sx),
                ..default()
            })
            .with_children(|row| {
                let mut fuel = ImageNode::new(art.courier_fuel.clone());
                fuel.image_mode = NodeImageMode::Stretch;
                row.spawn((
                    Node {
                        width: Val::Px(77. * sx),
                        height: Val::Px(80. * sy),
                        ..default()
                    },
                    fuel,
                ));
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(7. * sy),
                    flex_grow: 1.,
                    ..default()
                })
                .with_children(|stats| {
                    stats
                        .spawn(Node {
                            width: Val::Percent(100.),
                            height: Val::Px(31. * sy),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(7. * sx),
                            ..default()
                        })
                        .with_children(|fuel_label| {
                            label(fuel_label, art, "燃料", 22. * size, NAVY);
                            label(
                                fuel_label,
                                art,
                                format!("{}/8", g.fuel),
                                28. * size,
                                Color::srgb(0.08, 0.28, 0.64),
                            );
                        });
                    stats
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Px(22. * sy),
                                padding: UiRect::all(Val::Px(3. * size)),
                                column_gap: Val::Px(3. * sx),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.27, 0.42, 0.62)),
                            BorderRadius::all(Val::Px(8. * size)),
                        ))
                        .with_children(|bar| {
                            for i in 0..8 {
                                bar.spawn((
                                    Node {
                                        width: Val::Percent(12.5),
                                        height: Val::Percent(100.),
                                        flex_grow: if i == 7 { 0. } else { 1. },
                                        ..default()
                                    },
                                    BackgroundColor(if i < g.fuel as usize {
                                        Color::srgb(0.14, 0.82, 0.31)
                                    } else {
                                        Color::srgb(0.55, 0.67, 0.82)
                                    }),
                                    BorderRadius::all(Val::Px(3. * size)),
                                ));
                            }
                        });
                });
            });

        panel
            .spawn(Node {
                width: Val::Percent(100.),
                height: Val::Px(4. * sy),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            })
            .with_children(|divider| {
                for _ in 0..32 {
                    divider.spawn((
                        Node {
                            width: Val::Px(5. * sx),
                            height: Val::Px(2. * sy),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.58, 0.77, 0.94)),
                        BorderRadius::all(Val::Px(1. * size)),
                    ));
                }
            });

        panel
            .spawn(Node {
                width: Val::Percent(100.),
                height: Val::Px(134. * sy),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6. * sx),
                ..default()
            })
            .with_children(|row| {
                courier_tool_button(
                    row,
                    art,
                    &art.courier_navigation,
                    "导航仪",
                    g.tools[0],
                    CourierAction::Navigation,
                    sx,
                    sy,
                    size,
                );
                courier_tool_button(
                    row,
                    art,
                    &art.courier_repair,
                    "修理箱",
                    g.tools[1],
                    CourierAction::Repair,
                    sx,
                    sy,
                    size,
                );
                courier_tool_button(
                    row,
                    art,
                    &art.courier_candy,
                    "能量糖",
                    g.tools[2],
                    CourierAction::Candy,
                    sx,
                    sy,
                    size,
                );
            });
    });
}

fn courier_tool_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    icon: &Handle<Image>,
    name: &str,
    _count: u8,
    action: CourierAction,
    sx: f32,
    sy: f32,
    size: f32,
) {
    parent
        .spawn((
            Button,
            Action::Courier(action),
            Node {
                width: Val::Px(98. * sx),
                height: Val::Px(126. * sy),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(0. * sy),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|button| {
            let mut image = ImageNode::new(icon.clone());
            image.image_mode = NodeImageMode::Stretch;
            button.spawn((
                Node {
                    width: Val::Px(86. * sx),
                    height: Val::Px(86. * sy),
                    ..default()
                },
                image,
            ));
            label(button, art, name, 17. * size, NAVY);
        });
}

fn courier_planet(
    root: &mut ChildSpawnerCommands,
    art: &Art,
    g: &CourierGame,
    planet: usize,
    _name: &str,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
) {
    let delivered = (0..3).filter(|i| g.delivered[planet * 3 + *i]).count();
    let size = sx.min(sy);
    let (name_art, name_width, name_height) = match planet {
        0 => (&art.courier_moon_name, 75., 35.),
        1 => (&art.courier_blueberry_name, 88., 31.),
        _ => (&art.courier_candy_name, 83., 34.),
    };
    root.spawn((
        Button,
        Action::Courier(CourierAction::Planet(planet)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(left),
            top: Val::Px(top),
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
        BackgroundColor(Color::NONE),
    ))
    .with_children(|card| {
        card.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16. * sx),
                top: Val::Px(38. * sy),
                width: Val::Px(186. * sx),
                height: Val::Px(64. * sy),
                border: UiRect::all(Val::Px(2. * size)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.94, 0.98, 1.)),
            BorderColor(Color::srgb(0.28, 0.51, 0.82)),
            BorderRadius::all(Val::Px(17. * size)),
        ));
        card.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(5. * sx),
                top: Val::Px(0.),
                width: Val::Px(208. * sx),
                height: Val::Px(53. * sy),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(8. * sx),
                border: UiRect::all(Val::Px(3. * size)),
                ..default()
            },
            BackgroundColor(Color::WHITE),
            BorderColor(Color::srgb(0.20, 0.43, 0.75)),
            BorderRadius::all(Val::Px(28. * size)),
        ))
        .with_children(|header| {
            let mut name = ImageNode::new(name_art.clone());
            name.image_mode = NodeImageMode::Stretch;
            header.spawn((
                Node {
                    width: Val::Px(name_width * sx),
                    height: Val::Px(name_height * sy),
                    ..default()
                },
                name,
            ));
            label(header, art, format!("{delivered}/3"), 22. * size, NAVY);
        });
        card.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20. * sx),
            top: Val::Px(50. * sy),
            width: Val::Px(178. * sx),
            height: Val::Px(46. * sy),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(5. * sx),
            ..default()
        })
        .with_children(|slots| {
            for i in 0..3 {
                if g.delivered[planet * 3 + i] {
                    let mut parcel = ImageNode::new(art.courier_parcel_slot.clone());
                    parcel.image_mode = NodeImageMode::Stretch;
                    slots.spawn((
                        Node {
                            width: Val::Px(42. * sx),
                            height: Val::Px(38. * sy),
                            ..default()
                        },
                        parcel,
                    ));
                } else {
                    let mut empty = ImageNode::new(art.courier_empty_slot.clone());
                    empty.image_mode = NodeImageMode::Stretch;
                    slots.spawn((
                        Node {
                            width: Val::Px(39. * sx),
                            height: Val::Px(39. * sy),
                            ..default()
                        },
                        empty,
                    ));
                }
            }
        });
    });
}

fn courier_station_button(
    root: &mut ChildSpawnerCommands,
    art: &Art,
    sx: f32,
    sy: f32,
    _size: f32,
) {
    courier_image(
        root,
        &art.courier_station,
        786. * sx,
        414. * sy,
        134. * sx,
        69. * sy,
    );
    root.spawn((
        Button,
        Action::Courier(CourierAction::Refuel),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(794. * sx),
            top: Val::Px(421. * sy),
            width: Val::Px(122. * sx),
            height: Val::Px(54. * sy),
            ..default()
        },
        BackgroundColor(Color::NONE),
    ));
}

fn courier_panel(
    root: &mut ChildSpawnerCommands,
    art: &Art,
    g: &CourierGame,
    sx: f32,
    sy: f32,
    size: f32,
) {
    if matches!(g.stage, CourierStage::Route(_)) {
        courier_image(
            root,
            &art.courier_route_card,
            1318. * sx,
            123. * sy,
            346. * sx,
            548. * sy,
        );
        for (left, top, width, height, action) in [
            (1345., 464., 300., 84., CourierAction::SafeRoute),
            (1345., 558., 300., 86., CourierAction::ChallengeRoute),
        ] {
            root.spawn((
                Button,
                Action::Courier(action),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(left * sx),
                    top: Val::Px(top * sy),
                    width: Val::Px(width * sx),
                    height: Val::Px(height * sy),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ));
        }
        return;
    }

    let panel_width = 336. * sx;
    let panel_height = 538. * sy;
    let inner_width = 302. * sx;

    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16. * sx),
            top: Val::Px(130. * sy),
            width: Val::Px(panel_width),
            height: Val::Px(panel_height),
            padding: UiRect::all(Val::Px(15. * size)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8. * sy),
            overflow: Overflow::clip(),
            border: UiRect::all(Val::Px(5. * size)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.99, 0.98, 1., 0.98)),
        BorderColor(Color::srgb(0.62, 0.51, 0.96)),
        BorderRadius::all(Val::Px(28. * size)),
    ))
    .with_children(|p| {
        let title = match g.stage {
            CourierStage::Route(_) => "陨石带",
            CourierStage::Map if g.planet.is_some() => "星球送货清单",
            CourierStage::Map => "陨石带",
            CourierStage::Event(_) => "陨石带挑战",
            CourierStage::Delivery(_) => "包裹挑战",
            CourierStage::Supply => "补给站",
            CourierStage::Final => "终点包裹",
            CourierStage::Complete => "本章完成！",
        };
        p.spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Px(51. * sy),
                min_height: Val::Px(51. * sy),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.34, 0.27, 0.73)),
            BorderRadius::all(Val::Px(18. * size)),
        ))
        .with_children(|h| label(h, art, title, 25. * size, Color::WHITE));

        match g.stage {
            CourierStage::Route(_) => {
                courier_panel_art(p, art, inner_width, 220. * sy);
                label(p, art, "绕行更安全，穿越可收集星星", 16.5 * size, NAVY);
                button(
                    p,
                    art,
                    "绕行 · 燃料-1",
                    Action::Courier(CourierAction::SafeRoute),
                    inner_width,
                    82. * sy,
                    Color::srgb(0.10, 0.54, 0.92),
                    Color::WHITE,
                );
                button(
                    p,
                    art,
                    "穿越 · 挑战",
                    Action::Courier(CourierAction::ChallengeRoute),
                    inner_width,
                    82. * sy,
                    Color::srgb(0.98, 0.68, 0.12),
                    Color::srgb(0.28, 0.17, 0.02),
                );
            }
            CourierStage::Event(_) => {
                courier_panel_art(p, art, inner_width, 135. * sy);
                label(p, art, "陨石带里有 2 + 3 颗星星碎片。", 17. * size, INK);
                for (i, t) in ["4 颗", "5 颗", "6 颗"].iter().enumerate() {
                    button(
                        p,
                        art,
                        *t,
                        Action::Courier(CourierAction::EventAnswer(i)),
                        inner_width,
                        44. * sy,
                        Color::srgb(0.20, 0.57, 0.87),
                        Color::WHITE,
                    );
                }
                button(
                    p,
                    art,
                    "修理箱：安全通过",
                    Action::Courier(CourierAction::Repair),
                    inner_width,
                    42. * sy,
                    Color::srgb(0.77, 0.37, 0.56),
                    Color::WHITE,
                );
            }
            CourierStage::Delivery(slot) => {
                let q = g.question(slot);
                label(
                    p,
                    art,
                    format!("第{}件包裹", slot % 3 + 1),
                    15. * size,
                    MUTED,
                );
                label(p, art, q.prompt, 19. * size, INK);
                for (i, t) in q.choices.iter().enumerate() {
                    button(
                        p,
                        art,
                        t.clone(),
                        Action::Courier(CourierAction::Answer(i)),
                        inner_width,
                        47. * sy,
                        Color::srgb(0.13, 0.55, 0.89),
                        Color::WHITE,
                    );
                }
                button(
                    p,
                    art,
                    "给我一点提示",
                    Action::Courier(CourierAction::Hint),
                    inner_width,
                    40. * sy,
                    Color::srgb(0.61, 0.48, 0.81),
                    Color::WHITE,
                );
            }
            CourierStage::Supply => {
                label(
                    p,
                    art,
                    "补给站问题：2 + 3 单位燃料是多少？",
                    18. * size,
                    INK,
                );
                for (i, t) in ["4 格", "5 格", "6 格"].iter().enumerate() {
                    button(
                        p,
                        art,
                        *t,
                        Action::Courier(CourierAction::SupplyAnswer(i)),
                        inner_width,
                        48. * sy,
                        Color::srgb(0.16, 0.61, 0.79),
                        Color::WHITE,
                    );
                }
            }
            CourierStage::Final => {
                label(
                    p,
                    art,
                    "三颗星球的包裹都送到了！终点信号指向哪颗星球？",
                    18. * size,
                    INK,
                );
                for (i, t) in ["糖果星", "蓝莓星", "月球站"].iter().enumerate() {
                    button(
                        p,
                        art,
                        *t,
                        Action::Courier(CourierAction::FinalAnswer(i)),
                        inner_width,
                        46. * sy,
                        Color::srgb(0.17, 0.59, 0.85),
                        Color::WHITE,
                    );
                }
            }
            CourierStage::Complete => {
                courier_panel_art(p, art, inner_width, 170. * sy);
                label(
                    p,
                    art,
                    "九件包裹全部送达，终点礼物也找到啦！",
                    18. * size,
                    INK,
                );
                button(
                    p,
                    art,
                    "开启下一章",
                    Action::Courier(CourierAction::NextChapter),
                    inner_width,
                    48. * sy,
                    Color::srgb(0.98, 0.66, 0.14),
                    Color::WHITE,
                );
            }
            CourierStage::Map => {
                if let Some(planet) = g.planet {
                    label(
                        p,
                        art,
                        ["月球站", "蓝莓星", "糖果星"][planet],
                        20. * size,
                        INK,
                    );
                    for i in 0..3 {
                        let slot = planet * 3 + i;
                        if !g.delivered[slot] {
                            button(
                                p,
                                art,
                                format!("包裹 {} · 开始送件", i + 1),
                                Action::Courier(CourierAction::Delivery(slot)),
                                inner_width,
                                45. * sy,
                                Color::srgb(0.13, 0.55, 0.89),
                                Color::WHITE,
                            );
                        } else {
                            label(
                                p,
                                art,
                                format!("包裹 {} · 已送达 ✓", i + 1),
                                15. * size,
                                MUTED,
                            );
                        }
                    }
                    label(p, art, "选地图上的其他星球继续航行。", 14. * size, MUTED);
                } else {
                    courier_panel_art(p, art, inner_width, 205. * sy);
                    label(p, art, "点击地图上的星球，开始规划航线。", 17. * size, INK);
                }
            }
        }

        if !g.feedback.is_empty() && !matches!(g.stage, CourierStage::Route(_)) {
            label(
                p,
                art,
                &g.feedback,
                15. * size,
                Color::srgb(0.55, 0.27, 0.12),
            );
        }

        if !matches!(
            g.stage,
            CourierStage::Map | CourierStage::Route(_) | CourierStage::Complete
        ) {
            button(
                p,
                art,
                "返回星图",
                Action::Courier(CourierAction::Back),
                inner_width,
                38. * sy,
                Color::srgb(0.35, 0.42, 0.55),
                Color::WHITE,
            );
        }
    });
}

fn courier_panel_art(parent: &mut ChildSpawnerCommands, art: &Art, width: f32, height: f32) {
    let mut image = ImageNode::new(art.courier_asteroids.clone());
    image.image_mode = NodeImageMode::Stretch;
    parent.spawn((
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            min_height: Val::Px(height),
            ..default()
        },
        image,
        BorderRadius::all(Val::Px(18.)),
    ));
}
