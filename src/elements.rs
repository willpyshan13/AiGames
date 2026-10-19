use super::*;

#[derive(Clone, Copy)]
pub(super) struct ElementInfo {
    pub(super) number: u8,
    pub(super) symbol: &'static str,
    pub(super) name: &'static str,
    pub(super) column: u8,
    pub(super) period: u8,
    pub(super) family: u8,
}

// The first forty elements keep their real groups and periods. Empty positions
// are intentional: children can see the familiar shape of the periodic table.
pub(super) const ELEMENTS: [ElementInfo; 40] = [
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

pub(super) struct Mission {
    pub(super) title: &'static str,
    pub(super) formula: &'static str,
    pub(super) clue: &'static str,
    pub(super) result: &'static str,
    pub(super) atoms: &'static [u8],
    pub(super) color: Color,
}

const MISSIONS: [Mission; 11] = [
    Mission {
        title: "水",
        formula: "H2O",
        clue: "找 2 个氢和 1 个氧，拼出水分子",
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
    pub(super) observing: bool,
    pub(super) stage: usize,
    pub(super) score: u32,
    pub(super) picked: Vec<u8>,
    pub(super) last_seen: Option<u8>,
    pub(super) discovered: [bool; 40],
    pub(super) complete: bool,
    pub(super) finished: bool,
    pub(super) feedback: &'static str,
    pub(super) flash: f32,
    pub(super) effect_duration: f32,
    pub(super) wrong_flash: f32,
    pub(super) effect_clock: f32,
    redraw: f32,
}

impl ElementGame {
    pub(super) fn new() -> Self {
        Self {
            observing: false,
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

    pub(super) fn mission(&self) -> &'static Mission {
        &MISSIONS[self.stage]
    }

    pub(super) fn remaining(&self, number: u8) -> usize {
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

fn element_tile_color(element: &ElementInfo) -> Color {
    match element.number {
        1 => Color::srgb(0.18, 0.48, 0.89),
        6 | 7 | 15 | 16 | 34 => Color::srgb(0.08, 0.62, 0.48),
        8 => Color::srgb(0.22, 0.72, 0.48),
        _ => family_color(element.family),
    }
}

pub(super) fn family_name(family: u8) -> &'static str {
    match family {
        1 => "稀有气体",
        2 => "碱金属",
        3 => "碱土金属",
        4 => "类金属",
        5 => "卤素",
        6 => "其他金属",
        7 => "过渡金属",
        _ => "非金属",
    }
}

fn atom_color(number: u8) -> Color {
    match number {
        1 => Color::srgb(0.34, 0.72, 1.0),
        6 => Color::srgb(0.34, 0.39, 0.48),
        7 => Color::srgb(0.36, 0.53, 0.96),
        8 => Color::srgb(0.96, 0.29, 0.27),
        11 | 17 => Color::srgb(0.85, 0.66, 0.31),
        _ => family_color(ELEMENTS[usize::from(number - 1)].family),
    }
}

pub(super) fn atomic_mass(number: u8) -> &'static str {
    const MASSES: [&str; 40] = [
        "1.008", "4.003", "6.94", "9.012", "10.81", "12.011", "14.007", "15.999", "18.998",
        "20.180", "22.990", "24.305", "26.982", "28.085", "30.974", "32.06", "35.45", "39.948",
        "39.098", "40.078", "44.956", "47.867", "50.942", "51.996", "54.938", "55.845", "58.933",
        "58.693", "63.546", "65.38", "69.723", "72.630", "74.922", "78.971", "79.904", "83.798",
        "85.468", "87.62", "88.906", "91.224",
    ];
    MASSES[usize::from(number.saturating_sub(1)).min(MASSES.len() - 1)]
}

pub(super) fn electron_shells(number: u8) -> String {
    let n = i16::from(number);
    let shells: Vec<i16> = match number {
        1..=2 => vec![n],
        3..=10 => vec![2, n - 2],
        11..=18 => vec![2, 8, n - 10],
        19..=20 => vec![2, 8, 8, n - 18],
        21..=30 => vec![2, 8, n - 12, 2],
        31..=36 => vec![2, 8, 18, n - 28],
        37..=38 => vec![2, 8, 18, 8, n - 36],
        _ => vec![2, 8, 18, n - 30, 2],
    };
    shells
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn element_fact(number: u8) -> &'static str {
    match number {
        1 => "宇宙中最丰富的\n元素之一",
        6 => "生命的重要组成\n元素",
        8 => "空气中约五分之一\n是氧气",
        11 => "食盐中含有\n钠元素",
        14 => "沙子和石英中\n常见硅",
        20 => "贝壳和粉笔中\n常见钙",
        26 => "铁是常见的\n金属材料",
        29 => "铜能导电，\n也能导热",
        40 => "锆常用于\n耐热材料",
        _ => "点击元素卡片，认识它\n在周期表中的位置",
    }
}

pub(super) fn element_ui(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let game = &s.elements;
    let mission = game.mission();
    let compact = width < 1120. || height < 680.;
    let outer = if compact { 12. } else { 20. };
    let gap = if compact { 8. } else { 14. };
    let header_h = if compact { 46. } else { 58. };
    let mission_h = if compact { 112. } else { 152. };
    let footer_h = 32.;
    let right_w = (width * if compact { 0.25 } else { 0.205 }).clamp(
        if compact { 205. } else { 260. },
        if compact { 270. } else { 330. },
    );
    let table_w = (width - outer * 2. - right_w - gap).max(500.);
    let main_h = (height - outer * 2. - header_h - mission_h - footer_h - gap * 3.).max(260.);
    let label_w = if compact { 25. } else { 32. };
    let cell_gap = if compact { 2. } else { 4. };
    let tile = ((table_w - label_w - cell_gap * 17.) / 18.).min(if compact { 47. } else { 68. });
    let tile_gap = cell_gap;
    let grid_header_h = if compact { 22. } else { 28. };
    // Match the concept's taller board footprint on 16:10 Mac windows while
    // keeping each cell close to square on compact layouts.
    let tile_h = ((main_h - grid_header_h - 36. - tile_gap * 4.) / 5.)
        .min(tile * if compact { 1.0 } else { 1.28 })
        .max(tile.min(24.));
    let cards_h = tile_h * 5. + tile_gap * 4.;
    let grid_top = ((main_h - cards_h - grid_header_h) * 0.5).max(12.);
    let selected_number = game.last_seen.unwrap_or(mission.atoms[0]);
    let selected = &ELEMENTS[usize::from(selected_number - 1)];
    let effect_strength = if game.flash > 0. {
        let progress = if game.effect_duration > 0. {
            1. - game.flash / game.effect_duration
        } else {
            1.
        };
        (std::f32::consts::PI * progress).sin().max(0.)
    } else {
        0.
    };
    let heading = if compact { 20. } else { 28. };
    let body = if compact { 12. } else { 16. };
    let panel_edge = Color::srgb(0.12, 0.38, 0.54);
    let amber = Color::srgb(1., 0.75, 0.25);

    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(outer)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(gap),
                ..default()
            },
            BackgroundColor(Color::srgb(0.018, 0.055, 0.105)),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.element_backdrop.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            // Header: game identity and progress stay visible above the chemistry task.
            root.spawn((Node {
                width: Val::Percent(100.),
                height: Val::Px(header_h),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            },))
                .with_children(|header| {
                    header
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.),
                            ..row()
                        })
                        .with_children(|brand| {
                            brand
                                .spawn((
                                    Node {
                                        width: Val::Px(header_h * 0.72),
                                        height: Val::Px(header_h * 0.72),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.03, 0.20, 0.31)),
                                    BorderColor(Color::srgb(0.10, 0.72, 0.93)),
                                    BorderRadius::all(Val::Px(14.)),
                                ))
                                .with_children(|icon| {
                                    icon.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            width: Val::Percent(74.),
                                            height: Val::Percent(42.),
                                            border: UiRect::all(Val::Px(1.5)),
                                            ..default()
                                        },
                                        BorderColor(Color::srgb(0.18, 0.83, 1.)),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ));
                                    icon.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            width: Val::Percent(42.),
                                            height: Val::Percent(74.),
                                            border: UiRect::all(Val::Px(1.5)),
                                            ..default()
                                        },
                                        BorderColor(Color::srgb(0.18, 0.83, 1.)),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ));
                                    icon.spawn((
                                        Node {
                                            width: Val::Px(10.),
                                            height: Val::Px(10.),
                                            ..default()
                                        },
                                        BackgroundColor(amber),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ));
                                });
                            label(brand, art, "元素小", heading, Color::srgb(0.26, 0.83, 1.));
                            label(brand, art, "实验室", heading, amber);
                        });
                    header.spawn(row()).with_children(|stats| {
                        for text in [
                            format!("挑战 {}/{}", game.stage + 1, MISSIONS.len()),
                            format!("★ 得分 {}", game.score),
                            format!(
                                "已认识 {} 种",
                                game.discovered.iter().filter(|found| **found).count()
                            ),
                        ] {
                            stats
                                .spawn((
                                    Node {
                                        height: Val::Px(header_h * 0.72),
                                        padding: UiRect::horizontal(Val::Px(if compact {
                                            8.
                                        } else {
                                            14.
                                        })),
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.)),
                                        ..row()
                                    },
                                    BackgroundColor(Color::srgb(0.04, 0.13, 0.23)),
                                    BorderColor(panel_edge),
                                    BorderRadius::all(Val::Px(12.)),
                                ))
                                .with_children(|chip| {
                                    label(
                                        chip,
                                        art,
                                        text.clone(),
                                        body,
                                        Color::srgb(0.84, 0.93, 1.),
                                    );
                                });
                        }
                    });
                    button(
                        header,
                        art,
                        "游戏库  ›",
                        Action::Library,
                        if compact { 76. } else { 106. },
                        header_h * 0.76,
                        Color::srgb(0.02, 0.35, 0.53),
                        Color::WHITE,
                    );
                });

            // Mission card: keep the clue, formula and collected atoms together.
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(mission_h),
                    padding: UiRect::all(Val::Px(if compact { 12. } else { 20. })),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(18.),
                    border: UiRect::all(Val::Px(1.5)),
                    ..row()
                },
                BackgroundColor(Color::srgb(0.025, 0.10, 0.18)),
                BorderColor(if game.wrong_flash > 0. {
                    Color::srgb(1., 0.35, 0.35)
                } else {
                    Color::srgb(0.12, 0.68, 0.92)
                }),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|task| {
                task.spawn(Node {
                    width: Val::Percent(32.),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.),
                    ..default()
                })
                .with_children(|copy| {
                    label(
                        copy,
                        art,
                        format!("配方挑战：{}", mission.title),
                        if compact { 18. } else { 25. },
                        Color::WHITE,
                    );
                    label(copy, art, mission.clue, body, Color::srgb(0.73, 0.86, 0.96));
                    copy.spawn(Node {
                        align_items: AlignItems::FlexEnd,
                        column_gap: Val::Px(0.),
                        ..row()
                    })
                    .with_children(|formula| {
                        for character in mission.formula.chars() {
                            let subscript = character.is_ascii_digit();
                            formula
                                .spawn(Node {
                                    margin: UiRect::bottom(Val::Px(if subscript {
                                        if compact { 5. } else { 8. }
                                    } else {
                                        0.
                                    })),
                                    ..default()
                                })
                                .with_children(|piece| {
                                    label(
                                        piece,
                                        art,
                                        character.to_string(),
                                        if subscript {
                                            if compact { 17. } else { 23. }
                                        } else if compact {
                                            28.
                                        } else {
                                            39.
                                        },
                                        mission.color,
                                    )
                                });
                        }
                    });
                });
                task.spawn(Node {
                    width: Val::Percent(66.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(if compact { 4. } else { 8. }),
                    ..row()
                })
                .with_children(|atoms| {
                    for (index, number) in mission.atoms.iter().enumerate() {
                        if index > 0 {
                            label(atoms, art, "+", if compact { 16. } else { 23. }, amber);
                        }
                        let needed = mission.atoms[..=index]
                            .iter()
                            .filter(|candidate| **candidate == *number)
                            .count();
                        let filled = game
                            .picked
                            .iter()
                            .filter(|picked| **picked == *number)
                            .count()
                            >= needed;
                        let element = &ELEMENTS[usize::from(*number - 1)];
                        atoms
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(3.),
                                ..default()
                            })
                            .with_children(|slot| {
                                slot.spawn((
                                    Node {
                                        width: Val::Px(if compact { 40. } else { 58. }),
                                        height: Val::Px(if compact { 40. } else { 58. }),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Val::Px(1.5)),
                                        ..default()
                                    },
                                    BackgroundColor(if filled {
                                        atom_color(*number)
                                    } else {
                                        Color::srgb(0.08, 0.19, 0.29)
                                    }),
                                    BorderColor(if filled { amber } else { panel_edge }),
                                    BorderRadius::all(Val::Percent(50.)),
                                ))
                                .with_children(|bubble| {
                                    label(
                                        bubble,
                                        art,
                                        if filled { element.symbol } else { "?" },
                                        if compact { 17. } else { 23. },
                                        Color::WHITE,
                                    );
                                });
                                label(
                                    slot,
                                    art,
                                    element.name,
                                    if compact { 8. } else { 11. },
                                    Color::srgb(0.78, 0.89, 0.96),
                                );
                            });
                    }
                    label(
                        atoms,
                        art,
                        "→",
                        if compact { 18. } else { 26. },
                        Color::srgb(0.23, 0.78, 0.96),
                    );
                    // A compact molecule model echoes the water illustration in the approved concept.
                    // Its atoms follow the current recipe, so the visual changes with each challenge.
                    atoms
                        .spawn((Node {
                            width: Val::Px(if compact { 58. } else { 78. }),
                            height: Val::Px(if compact { 54. } else { 72. }),
                            position_type: PositionType::Relative,
                            ..default()
                        },))
                        .with_children(|model| {
                            let center_number = mission
                                .atoms
                                .iter()
                                .copied()
                                .find(|number| *number != 1)
                                .unwrap_or(mission.atoms[0]);
                            let center = &ELEMENTS[usize::from(center_number - 1)];
                            model
                                .spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(if compact { 20. } else { 27. }),
                                        top: Val::Px(if compact { 5. } else { 7. }),
                                        width: Val::Px(if compact { 27. } else { 36. }),
                                        height: Val::Px(if compact { 27. } else { 36. }),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Val::Px(1.5)),
                                        ..default()
                                    },
                                    BackgroundColor(atom_color(center_number)),
                                    BorderColor(Color::srgba(1., 1., 1., 0.8)),
                                    BorderRadius::all(Val::Percent(50.)),
                                ))
                                .with_children(|atom| {
                                    label(
                                        atom,
                                        art,
                                        center.symbol,
                                        if compact { 12. } else { 16. },
                                        Color::WHITE,
                                    );
                                });
                            let other_atoms: Vec<_> = mission
                                .atoms
                                .iter()
                                .copied()
                                .filter(|number| *number != center_number)
                                .take(2)
                                .collect();
                            for side in 0..2 {
                                let number =
                                    other_atoms.get(side).copied().unwrap_or(center_number);
                                let element = &ELEMENTS[usize::from(number - 1)];
                                model
                                    .spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(if side == 0 {
                                                1.
                                            } else if compact {
                                                39.
                                            } else {
                                                55.
                                            }),
                                            top: Val::Px(if compact { 29. } else { 38. }),
                                            width: Val::Px(if compact { 22. } else { 29. }),
                                            height: Val::Px(if compact { 22. } else { 29. }),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            border: UiRect::all(Val::Px(1.)),
                                            ..default()
                                        },
                                        BackgroundColor(atom_color(number)),
                                        BorderColor(Color::srgba(1., 1., 1., 0.72)),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ))
                                    .with_children(|atom| {
                                        label(
                                            atom,
                                            art,
                                            element.symbol,
                                            if compact { 9. } else { 12. },
                                            Color::WHITE,
                                        );
                                    });
                            }
                        });
                    if game.complete {
                        button(
                            atoms,
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
                            if compact { 86. } else { 124. },
                            if compact { 38. } else { 48. },
                            Color::srgb(0.10, 0.62, 0.49),
                            Color::WHITE,
                        );
                    }
                });
                if game.flash > 0. {
                    task.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(20.),
                            top: Val::Px(16.),
                            width: Val::Px(12. + effect_strength * 22.),
                            height: Val::Px(12. + effect_strength * 22.),
                            ..default()
                        },
                        BackgroundColor(mission.color.with_alpha(0.3 + effect_strength * 0.7)),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                }
            });

            // Main row: a real periodic-table layout on the left and selected-element dossier on the right.
            root.spawn((Node {
                width: Val::Percent(100.),
                height: Val::Px(main_h),
                column_gap: Val::Px(gap),
                align_items: AlignItems::Stretch,
                ..row()
            },))
                .with_children(|main| {
                    main.spawn((
                        Node {
                            width: Val::Px(table_w),
                            height: Val::Px(main_h),
                            position_type: PositionType::Relative,
                            overflow: Overflow::clip(),
                            border: UiRect::all(Val::Px(1.5)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.018, 0.06, 0.11)),
                        BorderColor(panel_edge),
                        BorderRadius::all(Val::Px(16.)),
                    ))
                    .with_children(|table| {
                        table
                            .spawn(Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(label_w),
                                top: Val::Px(grid_top - grid_header_h),
                                width: Val::Px(tile * 18. + tile_gap * 17.),
                                height: Val::Px(grid_header_h),
                                ..row()
                            })
                            .with_children(|groups| {
                                for group in 1..=18 {
                                    groups
                                        .spawn(Node {
                                            width: Val::Px(tile),
                                            margin: UiRect::right(Val::Px(tile_gap)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        })
                                        .with_children(|cell| {
                                            label(
                                                cell,
                                                art,
                                                group.to_string(),
                                                if compact { 9. } else { 12. },
                                                Color::srgb(0.57, 0.75, 0.86),
                                            );
                                        });
                                }
                            });
                        label_at(
                            table,
                            art,
                            "族 →",
                            Val::Px(4.),
                            Val::Px(grid_top - grid_header_h),
                            if compact { 9. } else { 11. },
                            Color::srgb(0.62, 0.8, 0.91),
                        );
                        for period in 0..5 {
                            let y = grid_top + period as f32 * (tile_h + tile_gap);
                            table
                                .spawn(Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(4.),
                                    top: Val::Px(y + tile_h * 0.28),
                                    ..default()
                                })
                                .with_children(|p| {
                                    label(
                                        p,
                                        art,
                                        (period + 1).to_string(),
                                        if compact { 10. } else { 13. },
                                        Color::srgb(0.66, 0.83, 0.93),
                                    );
                                });
                        }
                        label_at(
                            table,
                            art,
                            "周期",
                            Val::Px(2.),
                            Val::Px(grid_top - grid_header_h + 14.),
                            if compact { 8. } else { 10. },
                            Color::srgb(0.62, 0.8, 0.91),
                        );
                        // Draw the full 18 × 5 scaffold first so the empty positions read as a periodic table.
                        for period in 0..5 {
                            for group in 0..18 {
                                let x = label_w + group as f32 * (tile + tile_gap);
                                let y = grid_top + period as f32 * (tile_h + tile_gap);
                                table.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(x),
                                        top: Val::Px(y),
                                        width: Val::Px(tile),
                                        height: Val::Px(tile_h),
                                        border: UiRect::all(Val::Px(0.6)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.035, 0.16, 0.24, 0.22)),
                                    BorderColor(Color::srgba(0.16, 0.58, 0.75, 0.28)),
                                    BorderRadius::all(Val::Px(if compact { 4. } else { 7. })),
                                ));
                            }
                        }
                        for element in ELEMENTS {
                            let x = label_w + f32::from(element.column) * (tile + tile_gap);
                            let y = grid_top + f32::from(element.period) * (tile_h + tile_gap);
                            let wanted = game.remaining(element.number) > 0 && !game.complete;
                            let selected_card = game.last_seen == Some(element.number);
                            let tile_color = element_tile_color(&element);
                            let border_color = if wanted {
                                amber
                            } else if selected_card {
                                Color::WHITE
                            } else {
                                Color::srgba(0.7, 0.86, 1., 0.17)
                            };
                            table
                                .spawn((
                                    Button,
                                    ButtonTone(tile_color),
                                    Action::ElementPick(element.number),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(x),
                                        top: Val::Px(y),
                                        width: Val::Px(tile),
                                        height: Val::Px(tile_h),
                                        padding: UiRect::all(Val::Px(if compact {
                                            2.
                                        } else {
                                            4.
                                        })),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Val::Px(if wanted {
                                            2.5
                                        } else if selected_card {
                                            1.5
                                        } else {
                                            0.5
                                        })),
                                        ..default()
                                    },
                                    BackgroundColor(if selected_card {
                                        tile_color.with_alpha(0.98)
                                    } else {
                                        tile_color
                                    }),
                                    BorderColor(border_color.with_alpha(if wanted {
                                        1.
                                    } else if selected_card {
                                        0.9
                                    } else {
                                        0.28
                                    })),
                                    BorderRadius::all(Val::Px(if compact { 5. } else { 8. })),
                                ))
                                .with_children(|card| {
                                    label(
                                        card,
                                        art,
                                        element.number.to_string(),
                                        if compact { 8. } else { 10. },
                                        Color::srgba(1., 1., 1., 0.78),
                                    );
                                    label(
                                        card,
                                        art,
                                        element.symbol,
                                        if compact { 15. } else { 22. },
                                        Color::WHITE,
                                    );
                                    label(
                                        card,
                                        art,
                                        element.name,
                                        if compact { 8. } else { 10. },
                                        Color::WHITE,
                                    );
                                    if wanted {
                                        card.spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                right: Val::Px(2.),
                                                top: Val::Px(2.),
                                                width: Val::Px(5.),
                                                height: Val::Px(5.),
                                                ..default()
                                            },
                                            BackgroundColor(amber),
                                            BorderRadius::all(Val::Percent(50.)),
                                        ));
                                    }
                                });
                        }
                    });

                    main.spawn((
                        Node {
                            width: Val::Px(right_w),
                            height: Val::Px(main_h),
                            padding: UiRect::all(Val::Px(if compact { 12. } else { 18. })),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(if compact { 4. } else { 6. }),
                            align_items: AlignItems::Stretch,
                            border: UiRect::all(Val::Px(1.5)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.025, 0.09, 0.15)),
                        BorderColor(Color::srgb(0.13, 0.57, 0.75)),
                        BorderRadius::all(Val::Px(18.)),
                    ))
                    .with_children(|detail| {
                        label(
                            detail,
                            art,
                            "元素档案",
                            if compact { 16. } else { 21. },
                            Color::srgb(0.75, 0.91, 1.),
                        );
                        detail
                            .spawn((Node {
                                height: Val::Px(if compact { 68. } else { 112. }),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..row()
                            },))
                            .with_children(|hero| {
                                hero.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(2.),
                                    ..default()
                                })
                                .with_children(|names| {
                                    label(
                                        names,
                                        art,
                                        selected.name,
                                        if compact { 20. } else { 27. },
                                        Color::WHITE,
                                    );
                                    label(
                                        names,
                                        art,
                                        selected.symbol,
                                        if compact { 42. } else { 60. },
                                        atom_color(selected.number),
                                    );
                                    names
                                        .spawn((
                                            Node {
                                                width: Val::Px(if compact { 76. } else { 94. }),
                                                height: Val::Px(if compact { 20. } else { 24. }),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                column_gap: Val::Px(0.),
                                                border: UiRect::all(Val::Px(1.)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.04, 0.24, 0.27)),
                                            BorderColor(Color::srgb(0.13, 0.72, 0.63)),
                                            BorderRadius::all(Val::Px(12.)),
                                        ))
                                        .with_children(|badge| {
                                            label(
                                                badge,
                                                art,
                                                family_name(selected.family),
                                                if compact { 10. } else { 13. },
                                                Color::srgb(0.40, 0.95, 0.81),
                                            )
                                        });
                                });
                                hero.spawn((
                                    Node {
                                        width: Val::Px(if compact { 48. } else { 64. }),
                                        height: Val::Px(if compact { 48. } else { 64. }),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(family_color(selected.family)),
                                    BorderRadius::all(Val::Px(12.)),
                                ))
                                .with_children(|number| {
                                    label(
                                        number,
                                        art,
                                        format!("{:02}", selected.number),
                                        if compact { 19. } else { 25. },
                                        Color::WHITE,
                                    );
                                });
                            });
                        detail.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Px(1.),
                                ..default()
                            },
                            BackgroundColor(panel_edge),
                        ));
                        for (name, value) in [
                            ("原子序数", selected.number.to_string()),
                            ("相对质量", format!("{} u", atomic_mass(selected.number))),
                            ("电子排布", electron_shells(selected.number)),
                        ] {
                            detail
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    justify_content: JustifyContent::SpaceBetween,
                                    column_gap: Val::Px(6.),
                                    ..row()
                                })
                                .with_children(|row_node| {
                                    row_node
                                        .spawn(Node {
                                            width: Val::Percent(58.),
                                            ..default()
                                        })
                                        .with_children(|key| {
                                            label(
                                                key,
                                                art,
                                                name,
                                                if compact { 10. } else { 12. },
                                                Color::srgb(0.63, 0.77, 0.87),
                                            )
                                        });
                                    row_node
                                        .spawn(Node {
                                            width: Val::Percent(42.),
                                            justify_content: JustifyContent::FlexEnd,
                                            ..row()
                                        })
                                        .with_children(|value_node| {
                                            label(
                                                value_node,
                                                art,
                                                value.clone(),
                                                if compact { 10. } else { 12. },
                                                Color::WHITE,
                                            )
                                        });
                                });
                        }
                        detail
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.),
                                ..default()
                            })
                            .with_children(|fact| {
                                label(
                                    fact,
                                    art,
                                    "你知道吗？",
                                    if compact { 12. } else { 14. },
                                    amber,
                                );
                                label(
                                    fact,
                                    art,
                                    element_fact(selected.number),
                                    if compact { 11. } else { 14. },
                                    Color::srgb(0.84, 0.91, 0.97),
                                );
                            });
                        detail
                            .spawn((
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Px(if compact { 58. } else { 76. }),
                                    position_type: PositionType::Relative,
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.05, 0.23, 0.34)),
                                BorderColor(Color::srgb(0.13, 0.68, 0.84)),
                                BorderRadius::all(Val::Px(12.)),
                            ))
                            .with_children(|illustration| {
                                illustration.spawn((
                                    ImageNode::new(art.element_molecule.clone()),
                                    Node {
                                        width: Val::Px(if compact { 68. } else { 90. }),
                                        height: Val::Px(if compact { 56. } else { 72. }),
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                ));
                            });
                        label(
                            detail,
                            art,
                            "水分子模型  H2O",
                            if compact { 9. } else { 12. },
                            Color::srgb(0.65, 0.84, 0.94),
                        );
                        detail.spawn(Node {
                            flex_grow: 1.,
                            ..default()
                        });
                        label(
                            detail,
                            art,
                            format!(
                                "已认识 {}/40",
                                game.discovered.iter().filter(|found| **found).count()
                            ),
                            body,
                            Color::srgb(0.75, 0.9, 0.98),
                        );
                        detail
                            .spawn((
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Px(if compact { 8. } else { 11. }),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.09, 0.2, 0.28)),
                                BorderRadius::all(Val::Px(8.)),
                            ))
                            .with_children(|bar| {
                                let count = game.discovered.iter().filter(|found| **found).count();
                                bar.spawn((
                                    Node {
                                        width: Val::Percent(count as f32 * 100. / 40.),
                                        height: Val::Percent(100.),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.08, 0.84, 0.72)),
                                    BorderRadius::all(Val::Px(8.)),
                                ));
                            });
                    });
                });

            root.spawn(Node {
                width: Val::Percent(100.),
                height: Val::Px(footer_h),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|footer| {
                footer
                    .spawn((
                        Node {
                            height: Val::Px(if compact { 28. } else { 36. }),
                            padding: UiRect::horizontal(Val::Px(if compact { 14. } else { 24. })),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(10.),
                            border: UiRect::all(Val::Px(1.)),
                            ..row()
                        },
                        BackgroundColor(Color::srgb(0.025, 0.13, 0.21)),
                        BorderColor(Color::srgb(0.08, 0.55, 0.73)),
                        BorderRadius::all(Val::Px(24.)),
                    ))
                    .with_children(|hint| {
                        hint.spawn((
                            Node {
                                width: Val::Px(10.),
                                height: Val::Px(10.),
                                ..default()
                            },
                            BackgroundColor(amber),
                            BorderRadius::all(Val::Percent(50.)),
                        ));
                        label(
                            hint,
                            art,
                            game.feedback,
                            if compact { 10. } else { 14. },
                            if game.wrong_flash > 0. {
                                Color::srgb(1., 0.52, 0.48)
                            } else {
                                Color::srgb(0.86, 0.94, 1.)
                            },
                        );
                    });
            });
        });
}

fn label_at(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    left: Val,
    top: Val,
    size: f32,
    color: Color,
) {
    parent
        .spawn(Node {
            position_type: PositionType::Absolute,
            left,
            top,
            ..default()
        })
        .with_children(|label_node| label(label_node, art, text, size, color));
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
