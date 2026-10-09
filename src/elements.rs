use super::*;

#[derive(Clone, Copy)]
struct ElementInfo {
    number: u8,
    symbol: &'static str,
    name: &'static str,
    column: u8,
    period: u8,
    family: u8,
}

// The first forty elements keep their real groups and periods. Empty positions
// are intentional: children can see the familiar shape of the periodic table.
const ELEMENTS: [ElementInfo; 40] = [
    ElementInfo {
        number: 1,
        symbol: "H",
        name: "氢",
        column: 0,
        period: 0,
        family: 0,
    },
    ElementInfo {
        number: 2,
        symbol: "He",
        name: "氦",
        column: 17,
        period: 0,
        family: 1,
    },
    ElementInfo {
        number: 3,
        symbol: "Li",
        name: "锂",
        column: 0,
        period: 1,
        family: 2,
    },
    ElementInfo {
        number: 4,
        symbol: "Be",
        name: "铍",
        column: 1,
        period: 1,
        family: 3,
    },
    ElementInfo {
        number: 5,
        symbol: "B",
        name: "硼",
        column: 12,
        period: 1,
        family: 4,
    },
    ElementInfo {
        number: 6,
        symbol: "C",
        name: "碳",
        column: 13,
        period: 1,
        family: 0,
    },
    ElementInfo {
        number: 7,
        symbol: "N",
        name: "氮",
        column: 14,
        period: 1,
        family: 0,
    },
    ElementInfo {
        number: 8,
        symbol: "O",
        name: "氧",
        column: 15,
        period: 1,
        family: 0,
    },
    ElementInfo {
        number: 9,
        symbol: "F",
        name: "氟",
        column: 16,
        period: 1,
        family: 5,
    },
    ElementInfo {
        number: 10,
        symbol: "Ne",
        name: "氖",
        column: 17,
        period: 1,
        family: 1,
    },
    ElementInfo {
        number: 11,
        symbol: "Na",
        name: "钠",
        column: 0,
        period: 2,
        family: 2,
    },
    ElementInfo {
        number: 12,
        symbol: "Mg",
        name: "镁",
        column: 1,
        period: 2,
        family: 3,
    },
    ElementInfo {
        number: 13,
        symbol: "Al",
        name: "铝",
        column: 12,
        period: 2,
        family: 6,
    },
    ElementInfo {
        number: 14,
        symbol: "Si",
        name: "硅",
        column: 13,
        period: 2,
        family: 4,
    },
    ElementInfo {
        number: 15,
        symbol: "P",
        name: "磷",
        column: 14,
        period: 2,
        family: 0,
    },
    ElementInfo {
        number: 16,
        symbol: "S",
        name: "硫",
        column: 15,
        period: 2,
        family: 0,
    },
    ElementInfo {
        number: 17,
        symbol: "Cl",
        name: "氯",
        column: 16,
        period: 2,
        family: 5,
    },
    ElementInfo {
        number: 18,
        symbol: "Ar",
        name: "氩",
        column: 17,
        period: 2,
        family: 1,
    },
    ElementInfo {
        number: 19,
        symbol: "K",
        name: "钾",
        column: 0,
        period: 3,
        family: 2,
    },
    ElementInfo {
        number: 20,
        symbol: "Ca",
        name: "钙",
        column: 1,
        period: 3,
        family: 3,
    },
    ElementInfo {
        number: 21,
        symbol: "Sc",
        name: "钪",
        column: 2,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 22,
        symbol: "Ti",
        name: "钛",
        column: 3,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 23,
        symbol: "V",
        name: "钒",
        column: 4,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 24,
        symbol: "Cr",
        name: "铬",
        column: 5,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 25,
        symbol: "Mn",
        name: "锰",
        column: 6,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 26,
        symbol: "Fe",
        name: "铁",
        column: 7,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 27,
        symbol: "Co",
        name: "钴",
        column: 8,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 28,
        symbol: "Ni",
        name: "镍",
        column: 9,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 29,
        symbol: "Cu",
        name: "铜",
        column: 10,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 30,
        symbol: "Zn",
        name: "锌",
        column: 11,
        period: 3,
        family: 7,
    },
    ElementInfo {
        number: 31,
        symbol: "Ga",
        name: "镓",
        column: 12,
        period: 3,
        family: 6,
    },
    ElementInfo {
        number: 32,
        symbol: "Ge",
        name: "锗",
        column: 13,
        period: 3,
        family: 4,
    },
    ElementInfo {
        number: 33,
        symbol: "As",
        name: "砷",
        column: 14,
        period: 3,
        family: 4,
    },
    ElementInfo {
        number: 34,
        symbol: "Se",
        name: "硒",
        column: 15,
        period: 3,
        family: 0,
    },
    ElementInfo {
        number: 35,
        symbol: "Br",
        name: "溴",
        column: 16,
        period: 3,
        family: 5,
    },
    ElementInfo {
        number: 36,
        symbol: "Kr",
        name: "氪",
        column: 17,
        period: 3,
        family: 1,
    },
    ElementInfo {
        number: 37,
        symbol: "Rb",
        name: "铷",
        column: 0,
        period: 4,
        family: 2,
    },
    ElementInfo {
        number: 38,
        symbol: "Sr",
        name: "锶",
        column: 1,
        period: 4,
        family: 3,
    },
    ElementInfo {
        number: 39,
        symbol: "Y",
        name: "钇",
        column: 2,
        period: 4,
        family: 7,
    },
    ElementInfo {
        number: 40,
        symbol: "Zr",
        name: "锆",
        column: 3,
        period: 4,
        family: 7,
    },
];

struct Mission {
    title: &'static str,
    formula: &'static str,
    clue: &'static str,
    result: &'static str,
    atoms: &'static [u8],
    color: Color,
}

const MISSIONS: [Mission; 11] = [
    Mission {
        title: "做一滴水",
        formula: "H2O",
        clue: "找 2 个氢和 1 个氧",
        result: "水滴诞生啦！水由氢和氧组成。",
        atoms: &[1, 1, 8],
        color: Color::srgb(0.34, 0.75, 1.),
    },
    Mission {
        title: "装一瓶氧气",
        formula: "O2",
        clue: "找 2 个氧",
        result: "氧气泡泡出现啦！",
        atoms: &[8, 8],
        color: Color::srgb(0.43, 0.87, 0.75),
    },
    Mission {
        title: "寻找二氧化碳",
        formula: "CO2",
        clue: "找 1 个碳和 2 个氧",
        result: "拼出了二氧化碳的组成！",
        atoms: &[6, 8, 8],
        color: Color::srgb(0.96, 0.66, 0.44),
    },
    Mission {
        title: "拼一粒食盐",
        formula: "NaCl",
        clue: "找 1 个钠和 1 个氯",
        result: "食盐里的钠和氯找齐啦！",
        atoms: &[11, 17],
        color: Color::srgb(0.93, 0.82, 0.51),
    },
    Mission {
        title: "发现石英",
        formula: "SiO2",
        clue: "找 1 个硅和 2 个氧",
        result: "石英的元素配比完成啦！",
        atoms: &[14, 8, 8],
        color: Color::srgb(0.77, 0.62, 1.),
    },
    Mission {
        title: "认识碳酸钙",
        formula: "CaCO3",
        clue: "找 1 个钙、1 个碳和 3 个氧",
        result: "碳酸钙的元素配比完成啦！",
        atoms: &[20, 6, 8, 8, 8],
        color: Color::srgb(0.90, 0.89, 0.78),
    },
    Mission {
        title: "认识小苏打",
        formula: "NaHCO3",
        clue: "找 1 个钠、1 个氢、1 个碳和 3 个氧",
        result: "小苏打的元素配比完成啦！",
        atoms: &[11, 1, 6, 8, 8, 8],
        color: Color::srgb(0.48, 0.84, 0.93),
    },
    Mission {
        title: "认识氧化铁",
        formula: "Fe2O3",
        clue: "找 2 个铁和 3 个氧",
        result: "你找到了铁元素，氧化铁配比完成！",
        atoms: &[26, 26, 8, 8, 8],
        color: Color::srgb(0.89, 0.48, 0.39),
    },
    Mission {
        title: "认识氧化铜",
        formula: "CuO",
        clue: "找 1 个铜和 1 个氧",
        result: "铜元素也认识啦！",
        atoms: &[29, 8],
        color: Color::srgb(0.89, 0.62, 0.42),
    },
    Mission {
        title: "认识氧化锌",
        formula: "ZnO",
        clue: "找 1 个锌和 1 个氧",
        result: "氧化锌的元素配比完成啦！",
        atoms: &[30, 8],
        color: Color::srgb(0.80, 0.84, 0.91),
    },
    Mission {
        title: "发现氧化锆",
        formula: "ZrO2",
        clue: "找 1 个锆和 2 个氧",
        result: "十一关全完成，你认识了第 40 号元素！",
        atoms: &[40, 8, 8],
        color: Color::srgb(0.48, 0.85, 0.87),
    },
];

pub(super) struct ElementGame {
    pub(super) stage: usize,
    pub(super) score: u32,
    picked: Vec<u8>,
    last_seen: Option<u8>,
    discovered: [bool; 40],
    pub(super) complete: bool,
    pub(super) finished: bool,
    feedback: &'static str,
    flash: f32,
    effect_duration: f32,
    wrong_flash: f32,
    effect_clock: f32,
    redraw: f32,
}

impl ElementGame {
    pub(super) fn new() -> Self {
        Self {
            stage: 0,
            score: 0,
            picked: Vec::new(),
            last_seen: None,
            discovered: [false; 40],
            complete: false,
            finished: false,
            feedback: "点元素卡片，把需要的元素放进收集盘。",
            flash: 0.,
            effect_duration: 0.,
            wrong_flash: 0.,
            effect_clock: 0.,
            redraw: 0.,
        }
    }

    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    fn mission(&self) -> &'static Mission {
        &MISSIONS[self.stage]
    }

    fn remaining(&self, number: u8) -> usize {
        let required = self
            .mission()
            .atoms
            .iter()
            .filter(|item| **item == number)
            .count();
        let picked = self.picked.iter().filter(|item| **item == number).count();
        required.saturating_sub(picked)
    }

    pub(super) fn pick(&mut self, number: u8) -> bool {
        if self.complete || self.finished || !(1..=40).contains(&number) {
            return false;
        }
        self.discovered[usize::from(number - 1)] = true;
        self.last_seen = Some(number);
        if self.remaining(number) == 0 {
            self.feedback = "这张卡不是本关还需要的元素，再看看提示！";
            self.wrong_flash = 2.0;
            return false;
        }
        self.picked.push(number);
        self.score += 10;
        self.effect_duration = 0.72;
        self.flash = self.effect_duration;
        self.effect_clock = 0.;
        self.feedback = "找对了！继续收集。";
        if self.picked.len() == self.mission().atoms.len() {
            self.complete = true;
            self.finished = self.stage == MISSIONS.len() - 1;
            self.score += 80;
            self.effect_duration = 2.2;
            self.flash = self.effect_duration;
            self.feedback = self.mission().result;
        }
        true
    }

    pub(super) fn next(&mut self) {
        if !self.complete || self.finished {
            return;
        }
        self.stage += 1;
        self.picked.clear();
        self.last_seen = None;
        self.complete = false;
        self.feedback = "新挑战来了！看看需要哪些元素。";
        self.flash = 0.;
        self.effect_duration = 0.;
        self.wrong_flash = 0.;
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        if self.flash <= 0. && self.wrong_flash <= 0. {
            return false;
        }
        self.flash = (self.flash - dt).max(0.);
        self.wrong_flash = (self.wrong_flash - dt).max(0.);
        self.effect_clock += dt;
        self.redraw += dt;
        if self.redraw >= 1. / 30. {
            self.redraw = 0.;
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct ElementSave {
    version: u8,
    stage: usize,
    score: u32,
    picked: Vec<u8>,
    discovered: Vec<bool>,
    complete: bool,
    finished: bool,
}

impl ElementSave {
    pub(super) fn from_game(game: &ElementGame) -> Self {
        Self {
            version: 2,
            stage: game.stage,
            score: game.score,
            picked: game.picked.clone(),
            discovered: game.discovered.to_vec(),
            complete: game.complete,
            finished: game.finished,
        }
    }

    fn into_game(self) -> Option<ElementGame> {
        let mission = MISSIONS.get(self.stage)?;
        if !matches!((self.version, self.discovered.len()), (1, 20) | (2, 40))
            || self.picked.len() > mission.atoms.len()
            || self.complete != (self.picked.len() == mission.atoms.len())
            || self.finished
                != (self.complete
                    && self.stage + 1 == if self.version == 1 { 7 } else { MISSIONS.len() })
        {
            return None;
        }
        for number in &self.picked {
            let picked = self.picked.iter().filter(|item| *item == number).count();
            let required = mission.atoms.iter().filter(|item| *item == number).count();
            if picked > required {
                return None;
            }
        }
        let old_game_finished = self.version == 1 && self.finished;
        let mut game = ElementGame::new();
        game.stage = if old_game_finished { 7 } else { self.stage };
        game.score = self.score;
        if !old_game_finished {
            game.picked = self.picked;
            game.complete = self.complete;
            game.finished = self.finished;
        }
        game.discovered[..self.discovered.len()].copy_from_slice(&self.discovered);
        game.last_seen = game.picked.last().copied();
        game.feedback = if game.finished {
            "十一关全部完成，你是元素小侦探！"
        } else if old_game_finished {
            "新挑战已解锁！接着认识更多元素。"
        } else if game.complete {
            "配方完成啦，点下一关继续。"
        } else {
            "欢迎回来，接着收集元素。"
        };
        Some(game)
    }
}

pub(super) fn load_elements() -> ElementGame {
    let path = save_directory().join("elements-v1.json");
    match fs::read(&path) {
        Ok(bytes) => serde_json::from_slice::<ElementSave>(&bytes)
            .ok()
            .and_then(ElementSave::into_game)
            .unwrap_or_else(|| {
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                let _ = fs::rename(
                    &path,
                    path.with_extension(format!("unreadable-{stamp}.json")),
                );
                ElementGame::new()
            }),
        Err(_) => ElementGame::new(),
    }
}

fn family_color(family: u8) -> Color {
    match family {
        1 => Color::srgb(0.47, 0.59, 0.94),
        2 => Color::srgb(0.96, 0.43, 0.46),
        3 => Color::srgb(0.99, 0.67, 0.37),
        4 => Color::srgb(0.69, 0.48, 0.91),
        5 => Color::srgb(0.45, 0.79, 0.51),
        6 => Color::srgb(0.88, 0.67, 0.43),
        7 => Color::srgb(0.32, 0.65, 0.88),
        _ => Color::srgb(0.28, 0.77, 0.79),
    }
}

fn atom_color(number: u8) -> Color {
    ELEMENTS
        .get(usize::from(number.saturating_sub(1)))
        .map(|element| family_color(element.family))
        .unwrap_or(Color::WHITE)
}

pub(super) fn element_ui(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let game = &s.elements;
    let mission = game.mission();
    let small = height < 650.;
    let gap = if small { 3. } else { 4. };
    let tile = ((width - 38. - 17. * gap) / 18.).min(if small { 52. } else { 62. });
    let table_w = tile * 18. + gap * 17.;
    let table_h = tile * 5. + gap * 4.;
    let discovery_w = (tile + gap) * 9.2;
    let discovery_h = (tile + gap) * 2.55;
    let effect_progress = if game.effect_duration > 0. {
        (1. - game.flash / game.effect_duration).clamp(0., 1.)
    } else {
        1.
    };
    let effect_strength = if game.flash > 0. {
        (std::f32::consts::PI * effect_progress).sin().max(0.)
    } else {
        0.
    };
    let heading = if small { 21. } else { 29. };
    let body = if small { 13. } else { 17. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(if small { 9. } else { 18. })),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(if small { 8. } else { 14. }),
                ..default()
            },
            BackgroundColor(Color::srgb(0.045, 0.085, 0.15)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(table_w),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|top| {
                label(top, art, "元素小实验室", heading, Color::WHITE);
                label(
                    top,
                    art,
                    format!(
                        "挑战 {}/{}  ·  {} 分  ·  已认识 {} 种",
                        game.stage + 1,
                        MISSIONS.len(),
                        game.score,
                        game.discovered.iter().filter(|found| **found).count()
                    ),
                    body,
                    Color::srgb(1., 0.82, 0.49),
                );
                button(
                    top,
                    art,
                    "游戏库",
                    Action::Library,
                    if small { 74. } else { 88. },
                    38.,
                    SLATE,
                    Color::WHITE,
                );
            });
            root.spawn((
                Node {
                    width: Val::Px(table_w),
                    min_height: Val::Px(if small { 98. } else { 132. }),
                    border: UiRect::all(Val::Px(2.)),
                    padding: UiRect::all(Val::Px(if small { 11. } else { 17. })),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.),
                    ..row()
                },
                BackgroundColor(Color::srgb(0.09, 0.16, 0.27)),
                BorderColor(if game.wrong_flash > 0. {
                    Color::srgb(1., 0.48, 0.48)
                } else {
                    mission.color
                }),
                BorderRadius::all(Val::Px(16.)),
            ))
            .with_children(|panel| {
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.),
                        ..default()
                    })
                    .with_children(|copy| {
                        label(
                            copy,
                            art,
                            format!("第 {} 关 · {}", game.stage + 1, mission.title),
                            if small { 18. } else { 25. },
                            Color::WHITE,
                        );
                        label(copy, art, mission.clue, body, Color::srgb(0.67, 0.85, 0.94));
                        label(
                            copy,
                            art,
                            mission.formula,
                            if small { 25. } else { 36. },
                            mission.color,
                        );
                    });
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        row_gap: Val::Px(7.),
                        ..default()
                    })
                    .with_children(|right| {
                        right
                            .spawn(Node {
                                column_gap: Val::Px(5.),
                                ..row()
                            })
                            .with_children(|tray| {
                                for (index, number) in mission.atoms.iter().enumerate() {
                                    let filled = game
                                        .picked
                                        .iter()
                                        .filter(|picked| **picked == *number)
                                        .count()
                                        > mission.atoms[..=index]
                                            .iter()
                                            .filter(|candidate| **candidate == *number)
                                            .count()
                                            - 1;
                                    let element = &ELEMENTS[usize::from(number - 1)];
                                    tray.spawn((
                                        Node {
                                            width: Val::Px(if small { 30. } else { 42. }),
                                            height: Val::Px(if small { 30. } else { 42. }),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(if filled {
                                            atom_color(*number)
                                        } else {
                                            Color::srgb(0.18, 0.27, 0.38)
                                        }),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ))
                                    .with_children(|slot| {
                                        label(
                                            slot,
                                            art,
                                            if filled { element.symbol } else { "?" },
                                            if small { 14. } else { 19. },
                                            Color::WHITE,
                                        )
                                    });
                                }
                            });
                        if game.complete {
                            button(
                                right,
                                art,
                                if game.finished {
                                    "再玩一次"
                                } else {
                                    "下一关 →"
                                },
                                if game.finished {
                                    Action::ElementReset
                                } else {
                                    Action::ElementNext
                                },
                                if small { 84. } else { 116. },
                                if small { 31. } else { 38. },
                                Color::srgb(0.19, 0.62, 0.52),
                                Color::WHITE,
                            );
                        } else {
                            label(
                                right,
                                art,
                                format!("已找到 {}/{}", game.picked.len(), mission.atoms.len()),
                                body,
                                Color::srgb(0.71, 0.82, 0.89),
                            );
                        }
                    });
            });
            root.spawn((
                Node {
                    width: Val::Px(table_w),
                    height: Val::Px(table_h),
                    position_type: PositionType::Relative,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.065, 0.12, 0.20)),
                BorderRadius::all(Val::Px(12.)),
            ))
            .with_children(|table| {
                // The table's open center becomes a little discovery window.
                table
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px((tile + gap) * 2.3),
                            top: Val::Px((tile + gap) * 0.32),
                            width: Val::Px(discovery_w),
                            height: Val::Px(discovery_h),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            row_gap: Val::Px(5.),
                            ..default()
                        },
                        BackgroundColor(if game.complete {
                            mission.color.with_alpha(0.13 + effect_strength * 0.12)
                        } else {
                            Color::srgba(
                                0.12 + effect_strength * 0.05,
                                0.23 + effect_strength * 0.07,
                                0.34 + effect_strength * 0.08,
                                0.75,
                            )
                        }),
                        BorderRadius::all(Val::Px(14.)),
                    ))
                    .with_children(|window| {
                        label(
                            window,
                            art,
                            if game.complete {
                                "发现成功！"
                            } else {
                                "元素发现窗"
                            },
                            if small { 16. } else { 23. },
                            if game.complete {
                                mission.color
                            } else {
                                Color::srgb(0.56, 0.77, 0.86)
                            },
                        );
                        label(
                            window,
                            art,
                            if game.complete {
                                mission.formula
                            } else if let Some(number) = game.last_seen {
                                let element = &ELEMENTS[usize::from(number - 1)];
                                element.symbol
                            } else {
                                "点亮卡片，收集配方"
                            },
                            (if small { 23. } else { 32. })
                                * (1. + effect_strength * if game.complete { 0.24 } else { 0.12 }),
                            Color::WHITE,
                        );
                        if game.complete {
                            label(
                                window,
                                art,
                                mission.result,
                                if small { 11. } else { 14. },
                                Color::srgb(0.87, 0.94, 1.),
                            );
                        } else if let Some(number) = game.last_seen {
                            let element = &ELEMENTS[usize::from(number - 1)];
                            label(
                                window,
                                art,
                                format!("{}  ·  第 {} 号元素", element.name, element.number),
                                if small { 12. } else { 16. },
                                Color::srgb(0.87, 0.94, 1.),
                            );
                        }
                        if game.flash > 0. {
                            let scale = if game.complete { 1.35 } else { 1. };
                            let max_diameter = if small { 34. } else { 48. };
                            for ring in 0..3 {
                                let delayed = (effect_progress - ring as f32 * 0.12).max(0.);
                                let diameter = (10. + delayed * max_diameter) * scale;
                                let fade =
                                    ((1. - delayed) * 0.75 * effect_strength).clamp(0., 0.75);
                                window.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px((discovery_w - diameter) * 0.5),
                                        top: Val::Px((discovery_h - diameter) * 0.5),
                                        width: Val::Px(diameter),
                                        height: Val::Px(diameter),
                                        border: UiRect::all(Val::Px(if small { 1.5 } else { 2. })),
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderColor(mission.color.with_alpha(fade)),
                                    BorderRadius::all(Val::Percent(50.)),
                                ));
                            }
                            let spark_count = if game.complete { 16 } else { 10 };
                            for i in 0..spark_count {
                                let seed = i as f32 / spark_count as f32;
                                let angle = seed * std::f32::consts::TAU
                                    + game.effect_clock * if game.complete { 1.8 } else { 2.8 };
                                let radius = 0.18 + effect_progress * 0.34;
                                let x = discovery_w * (0.5 + angle.cos() * radius);
                                let y = discovery_h * (0.5 + angle.sin() * radius * 0.72);
                                let twinkle = (game.effect_clock * 12. + seed * 31.).sin().abs();
                                let diameter = (if small { 3.5 } else { 5. }) + twinkle * 3.;
                                let color = if i % 3 == 0 {
                                    Color::WHITE
                                } else {
                                    mission.color
                                };
                                window.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(x),
                                        top: Val::Px(y),
                                        width: Val::Px(diameter),
                                        height: Val::Px(diameter),
                                        ..default()
                                    },
                                    BackgroundColor(
                                        color.with_alpha(0.35 + effect_strength * 0.65),
                                    ),
                                    BorderRadius::all(Val::Percent(50.)),
                                ));
                            }
                        }
                    });
                for element in ELEMENTS {
                    let x = f32::from(element.column) * (tile + gap);
                    let y = f32::from(element.period) * (tile + gap);
                    let wanted = game.remaining(element.number) > 0
                        && (game.stage < 2 || game.wrong_flash > 0.);
                    let color = family_color(element.family);
                    table
                        .spawn((
                            Button,
                            ButtonTone(color),
                            Action::ElementPick(element.number),
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(x),
                                top: Val::Px(y),
                                width: Val::Px(tile),
                                height: Val::Px(tile),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(if wanted && !game.complete {
                                    2.
                                } else {
                                    0.
                                })),
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderColor(Color::WHITE.with_alpha(if wanted && !game.complete {
                                0.75
                            } else {
                                0.
                            })),
                            BorderRadius::all(Val::Px(if small { 6. } else { 9. })),
                        ))
                        .with_children(|card| {
                            label(
                                card,
                                art,
                                element.number.to_string(),
                                if small { 9. } else { 10. },
                                Color::srgba(1., 1., 1., 0.78),
                            );
                            label(
                                card,
                                art,
                                element.symbol,
                                if small { 17. } else { 23. },
                                Color::WHITE,
                            );
                            label(
                                card,
                                art,
                                element.name,
                                if small { 9. } else { 11. },
                                Color::WHITE,
                            );
                        });
                }
            });
            root.spawn(Node {
                width: Val::Px(table_w),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|foot| {
                label(
                    foot,
                    art,
                    game.feedback,
                    if small { 12. } else { 17. },
                    if game.wrong_flash > 0. {
                        Color::srgb(1., 0.57, 0.56)
                    } else {
                        Color::srgb(0.80, 0.91, 0.96)
                    },
                );
                label(
                    foot,
                    art,
                    "屏幕拼图 · 不是真实化学实验",
                    if small { 10. } else { 13. },
                    Color::srgb(0.53, 0.67, 0.75),
                );
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_forty_keep_periodic_positions_and_numbers() {
        for (index, element) in ELEMENTS.iter().enumerate() {
            assert_eq!(usize::from(element.number), index + 1);
        }
        assert_eq!((ELEMENTS[0].period, ELEMENTS[0].column), (0, 0));
        assert_eq!((ELEMENTS[1].period, ELEMENTS[1].column), (0, 17));
        assert_eq!((ELEMENTS[7].period, ELEMENTS[7].column), (1, 15));
        assert_eq!(
            (
                ELEMENTS[20].symbol,
                ELEMENTS[20].period,
                ELEMENTS[20].column
            ),
            ("Sc", 3, 2)
        );
        assert_eq!(
            (
                ELEMENTS[35].symbol,
                ELEMENTS[35].period,
                ELEMENTS[35].column
            ),
            ("Kr", 3, 17)
        );
        assert_eq!(
            (
                ELEMENTS[36].symbol,
                ELEMENTS[36].period,
                ELEMENTS[36].column
            ),
            ("Rb", 4, 0)
        );
        assert_eq!(
            (
                ELEMENTS[39].symbol,
                ELEMENTS[39].period,
                ELEMENTS[39].column
            ),
            ("Zr", 4, 3)
        );
    }

    #[test]
    fn water_recipe_requires_two_hydrogen_and_one_oxygen() {
        let mut game = ElementGame::new();
        assert!(!game.pick(6));
        assert_eq!(game.score, 0);
        assert!(game.pick(1));
        assert!(game.pick(8));
        assert!(!game.complete);
        assert!(game.pick(1));
        assert!(game.complete);
        assert_eq!(game.score, 110);
        game.next();
        assert_eq!(game.stage, 1);
        assert!(game.picked.is_empty());
    }

    #[test]
    fn all_recipes_can_be_finished_in_any_element_order() {
        let mut game = ElementGame::new();
        for (stage, mission) in MISSIONS.iter().enumerate() {
            for number in mission.atoms.iter().rev() {
                assert!(game.pick(*number));
            }
            assert!(game.complete);
            if stage + 1 < MISSIONS.len() {
                game.next();
            }
        }
        assert!(game.finished);
    }

    #[test]
    fn progress_round_trips_without_resetting_the_stage() {
        let mut game = ElementGame::new();
        game.pick(1);
        game.pick(40);
        let bytes = serde_json::to_vec(&ElementSave::from_game(&game)).unwrap();
        let restored = serde_json::from_slice::<ElementSave>(&bytes)
            .unwrap()
            .into_game()
            .unwrap();
        assert_eq!(restored.stage, 0);
        assert_eq!(restored.picked, vec![1]);
        assert_eq!(restored.remaining(1), 1);
        assert_eq!(restored.score, 10);
        assert!(restored.discovered[39]);
    }

    #[test]
    fn old_finished_save_unlocks_the_new_challenges() {
        let mut old = ElementSave::from_game(&ElementGame::new());
        old.version = 1;
        old.stage = 6;
        old.score = 620;
        old.picked = MISSIONS[6].atoms.to_vec();
        old.discovered.truncate(20);
        old.discovered[19] = true;
        old.complete = true;
        old.finished = true;
        let bytes = serde_json::to_vec(&old).unwrap();
        let restored = serde_json::from_slice::<ElementSave>(&bytes)
            .unwrap()
            .into_game()
            .unwrap();
        assert_eq!(restored.stage, 7);
        assert_eq!(restored.score, 620);
        assert!(!restored.complete);
        assert!(!restored.finished);
        assert!(restored.discovered[19]);
        assert_eq!(restored.discovered.len(), 40);
    }

    #[test]
    fn damaged_progress_cannot_unlock_a_finished_stage() {
        let mut save = ElementSave::from_game(&ElementGame::new());
        save.finished = true;
        assert!(save.into_game().is_none());
    }
}
