//! Bounded bridge and robot puzzles, independent of rendering and legacy saves.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

pub const LEVEL_COUNT: usize = 6;
const CLOUD_NUMBERS: [usize; LEVEL_COUNT] = [1, 2, 4, 3, 5, 6];
const ROBOT_NUMBERS: [usize; LEVEL_COUNT] = [1, 3, 6, 2, 4, 5];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Cloud,
    Robot,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bridge {
    Wood,
    Stone,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    Forward,
    Left,
    Right,
    Wait,
}
impl Op {
    pub fn label(self) -> &'static str {
        match self {
            Self::Forward => "前进",
            Self::Left => "左转",
            Self::Right => "右转",
            Self::Wait => "等待",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub op: Op,
    pub repeats: u8,
}
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub grid: [i32; 2],
    pub point: [f32; 2],
    pub gap: bool,
    pub wind: bool,
}
#[derive(Clone, Debug)]
pub struct Level {
    pub number: usize,
    pub title: &'static str,
    pub cells: Vec<Cell>,
    pub edges: Vec<(usize, usize)>,
    pub start: usize,
    pub goal: usize,
    pub pad: Option<usize>,
    pub gate: Option<usize>,
    pub bonus: Option<usize>,
    pub stock: [usize; 2],
    pub card_limit: usize,
    pub start_facing: usize,
    pub require_bonus: bool,
    pub par_cards: usize,
    pub objective: &'static str,
}
impl Level {
    pub fn index_for_number(kind: Kind, number: usize) -> Option<usize> {
        let numbers = if kind == Kind::Cloud {
            CLOUD_NUMBERS
        } else {
            ROBOT_NUMBERS
        };
        numbers.iter().position(|n| *n == number)
    }
    pub fn new(kind: Kind, index: usize) -> Self {
        assert!(index < LEVEL_COUNT);
        if kind == Kind::Cloud {
            let points = [
                [397., 464.],
                [508., 487.],
                [588., 495.],
                [683., 517.],
                [771., 539.],
                [850., 552.],
                [1145., 575.],
                [434., 385.],
                [487., 364.],
                [543., 339.],
                [600., 311.],
                [653., 282.],
                [745., 276.],
                [815., 285.],
                [905., 310.],
                [952., 325.],
                [1011., 352.],
                [1073., 379.],
                [1130., 402.],
            ];
            let cells = points
                .into_iter()
                .enumerate()
                .map(|(i, point)| Cell {
                    grid: [i as i32, 0],
                    point,
                    gap: matches!(i,2..=4|8..=11|15..=17),
                    wind: match index {
                        2 => i == 3,
                        4 => matches!(i, 2 | 4),
                        5 => i == 10,
                        _ => false,
                    },
                })
                .collect();
            let mut edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
            if index > 0 {
                edges.extend([
                    (0, 7),
                    (7, 8),
                    (8, 9),
                    (9, 10),
                    (10, 11),
                    (11, 12),
                    (12, 13),
                    (13, 14),
                    (14, 15),
                    (15, 16),
                    (16, 17),
                    (17, 18),
                    (18, 6),
                ]);
            }
            Self {
                number: CLOUD_NUMBERS[index],
                title: [
                    "修好第一座桥",
                    "两条都能走",
                    "风口两条路",
                    "精打细算",
                    "风口取舍",
                    "逆风返航",
                ][index],
                cells,
                edges,
                start: if index == 5 { 6 } else { 0 },
                goal: if index == 5 { 0 } else { 6 },
                pad: None,
                gate: None,
                bonus: matches!(index, 1 | 2 | 4 | 5).then_some(13),
                stock: [[3, 0], [7, 0], [7, 1], [2, 1], [6, 1], [6, 1]][index],
                card_limit: 0,
                start_facing: if index == 5 { 2 } else { 0 },
                require_bonus: false,
                par_cards: 0,
                objective: "搭桥抵达终点",
            }
        } else {
            let data = [
                ([0, 0], [392., 232.]),
                ([1, 0], [492., 249.]),
                ([2, 0], [558., 270.]),
                ([3, 0], [626., 296.]),
                ([3, 1], [666., 361.]),
                ([3, 2], [741., 437.]),
                ([4, 2], [833., 462.]),
                ([5, 2], [956., 463.]),
                ([6, 2], [1147., 430.]),
                ([4, 1], [754., 381.]),
                ([5, 1], [842., 402.]),
                ([6, 1], [933., 423.]),
            ];
            let mut cells: Vec<_> = data
                .into_iter()
                .enumerate()
                .map(|(i, (grid, point))| Cell {
                    grid,
                    point,
                    gap: matches!(i, 1 | 2 | 6 | 9 | 10),
                    wind: false,
                })
                .collect();
            if !matches!(index, 1 | 4) {
                cells.truncate(9);
            }
            let mut edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 8),
            ];
            if matches!(index, 1 | 4) {
                edges.extend([(4, 9), (9, 10), (10, 11), (11, 8)]);
            }
            if matches!(index, 0 | 3) {
                cells.truncate(4);
                edges.truncate(3);
            }
            Self {
                number: ROBOT_NUMBERS[index],
                title: [
                    "先走到小岛",
                    "多走一条路",
                    "先开门，再到家",
                    "两张卡过桥",
                    "七卡取星",
                    "转身再出发",
                ][index],
                cells,
                edges,
                start: 0,
                goal: if matches!(index, 0 | 3) { 3 } else { 8 },
                pad: matches!(index, 2 | 5).then_some(5),
                gate: matches!(index, 2 | 5).then_some(7),
                bonus: matches!(index, 1 | 4).then_some(9),
                stock: [[2, 0], [4, 0], [3, 0], [2, 0], [4, 0], [3, 0]][index],
                card_limit: [8, 8, 8, 2, 7, 6][index],
                start_facing: if index == 5 { 2 } else { 0 },
                require_bonus: index == 4,
                par_cards: [1, 5, 5, 2, 7, 6][index],
                objective: [
                    "补好桥，走到小岛。",
                    "拿星晶，再到终点。",
                    "踩下机关，打开星门。",
                    "两张卡以内走到小岛。",
                    "先拿星晶，才能过关。",
                    "先转身，再踩机关。",
                ][index],
            }
        }
    }
    pub fn active(&self, i: usize) -> bool {
        i < self.cells.len()
            && (i == self.start || self.edges.iter().any(|&(a, b)| a == i || b == i))
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Plan {
    pub bridges: BTreeMap<usize, Bridge>,
    pub cards: Vec<Card>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Progress {
    pub version: u8,
    pub kind: Kind,
    pub stage: usize,
    pub stars: Vec<u8>,
    pub plans: Vec<Plan>,
}
impl Progress {
    pub fn new(kind: Kind) -> Self {
        Self {
            version: 1,
            kind,
            stage: 0,
            stars: vec![0; LEVEL_COUNT],
            plans: vec![Plan::default(); LEVEL_COUNT],
        }
    }
    pub fn valid(&self, kind: Kind) -> bool {
        self.version == 1
            && self.kind == kind
            && matches!(self.plans.len(), 3 | LEVEL_COUNT)
            && self.stars.len() == self.plans.len()
            && self.stage < self.plans.len()
            && self.stars.iter().all(|s| *s <= 3)
            && self.plans.iter().enumerate().all(|(i, p)| {
                let l = Level::new(kind, i);
                p.cards.len() <= l.card_limit
                    && p.cards.iter().all(|c| (1..=4).contains(&c.repeats))
                    && p.cards.iter().map(|c| c.repeats as usize).sum::<usize>() <= 64
                    && p.bridges
                        .iter()
                        .all(|(n, _)| l.active(*n) && l.cells[*n].gap)
                    && [Bridge::Wood, Bridge::Stone]
                        .iter()
                        .enumerate()
                        .all(|(j, b)| p.bridges.values().filter(|v| *v == b).count() <= l.stock[j])
            })
    }
}
pub struct Puzzle {
    pub kind: Kind,
    pub stage: usize,
    pub level: Level,
    pub plan: Plan,
    pub progress: Progress,
    pub robot: usize,
    pub facing: usize,
    pub gate_open: bool,
    pub bonus: bool,
    pub won: bool,
    pub running: bool,
    pub started: bool,
    pub pc: usize,
    pub substep: u8,
    pub selected: Option<usize>,
    pub material: Bridge,
    pub erase: bool,
    pub build: bool,
    pub slow: bool,
    pub message: String,
    pub hint: u8,
    undo: Vec<BTreeMap<usize, Bridge>>,
    route: VecDeque<usize>,
    clock: f32,
}
impl Puzzle {
    pub fn new(kind: Kind) -> Self {
        Self::restore(Progress::new(kind))
    }
    pub fn restore(mut progress: Progress) -> Self {
        progress.stars.resize(LEVEL_COUNT, 0);
        progress.plans.resize_with(LEVEL_COUNT, Plan::default);
        let kind = progress.kind;
        let stage = progress.stage;
        let level = Level::new(kind, stage);
        let robot = level.start;
        let facing = level.start_facing;
        Self {
            kind,
            stage,
            level,
            robot,
            plan: progress.plans[stage].clone(),
            progress,
            facing,
            gate_open: false,
            bonus: false,
            won: false,
            running: false,
            started: false,
            pc: 0,
            substep: 0,
            selected: None,
            material: Bridge::Wood,
            erase: false,
            build: kind == Kind::Cloud,
            slow: true,
            message: "观察路线，选择缺口搭桥；撤销会退回材料。".into(),
            hint: 0,
            undo: vec![],
            route: VecDeque::new(),
            clock: 0.,
        }
    }
    pub fn snapshot(&self) -> Progress {
        let mut p = self.progress.clone();
        p.stage = self.stage;
        p.plans[self.stage] = self.plan.clone();
        p
    }
    pub fn choose(&mut self, stage: usize) {
        if stage < LEVEL_COUNT {
            let mut p = self.snapshot();
            p.stage = stage;
            *self = Self::restore(p);
        }
    }
    pub fn remaining(&self, b: Bridge) -> usize {
        self.level.stock[usize::from(b == Bridge::Stone)]
            .saturating_sub(self.plan.bridges.values().filter(|v| **v == b).count())
    }
    pub fn reset_run(&mut self) {
        self.robot = self.level.start;
        self.facing = self.level.start_facing;
        self.gate_open = false;
        self.bonus = false;
        self.won = false;
        self.running = false;
        self.started = false;
        self.pc = 0;
        self.substep = 0;
        self.route.clear();
        self.clock = 0.;
    }
    pub fn place(&mut self, node: usize) -> bool {
        if !self.level.active(node) || !self.level.cells[node].gap {
            self.message = "基础小岛固定，请在蓝色缺口搭桥。".into();
            return false;
        }
        if self.erase {
            if self.plan.bridges.contains_key(&node) {
                self.reset_run();
                self.undo.push(self.plan.bridges.clone());
                self.plan.bridges.remove(&node);
                self.message = "桥块已拆除，材料已退回。".into();
                return true;
            }
            self.message = "这个缺口还没有桥块。".into();
            return false;
        }
        if self.plan.bridges.get(&node) == Some(&self.material) {
            self.message = "这里已经放好了；切换拆除可拿回材料。".into();
            return false;
        }
        if self.remaining(self.material) == 0 {
            self.message = "这种建材用完了，先拆回一块或换条路线。".into();
            return false;
        }
        self.reset_run();
        self.undo.push(self.plan.bridges.clone());
        self.plan.bridges.insert(node, self.material);
        self.message = if self.level.cells[node].wind && self.material == Bridge::Wood {
            "风口需要石桥；木桥无法抵挡这里的风。"
        } else {
            "已搭好一格；点撤销可退回材料。"
        }
        .into();
        true
    }
    pub fn undo(&mut self) {
        if let Some(before) = self.undo.pop() {
            self.reset_run();
            self.plan.bridges = before;
            self.message = "已撤销，材料已退回。".into();
        } else {
            self.message = "还没有需要撤销的搭建。".into();
        }
    }
    pub fn restart(&mut self) {
        self.reset_run();
        self.plan = Plan::default();
        self.undo.clear();
        self.selected = None;
        self.message = "本关已重来，材料全部退回。".into();
    }
    pub fn add(&mut self, op: Op) {
        self.reset_run();
        if let Some(i) = self.selected.take().filter(|i| *i < self.plan.cards.len()) {
            self.plan.cards[i].op = op;
            self.message = "已修改选中的指令。".into();
        } else if self.plan.cards.len() < self.level.card_limit {
            self.plan.cards.push(Card { op, repeats: 1 });
            self.message = "点选卡片，再点重复可调整次数。".into();
        } else {
            self.message = format!(
                "本关最多{}张指令，请点选一张修改，或清空重排。",
                self.level.card_limit
            );
        }
    }
    pub fn repeat(&mut self) {
        let Some(i) = self
            .selected
            .or_else(|| self.plan.cards.len().checked_sub(1))
        else {
            self.message = "先添加或点选一张行动卡。".into();
            return;
        };
        self.reset_run();
        let card = &mut self.plan.cards[i];
        card.repeats = card.repeats % 4 + 1;
        self.message = format!(
            "第{}张：{} ×{}；再次点重复可切换1–4次。",
            i + 1,
            card.op.label(),
            card.repeats
        );
    }
    pub fn clear(&mut self) {
        self.reset_run();
        self.plan.cards.clear();
        self.selected = None;
        self.message = "指令已清空；桥梁保留。".into();
    }
    pub fn path(&self) -> Option<VecDeque<usize>> {
        let mut queue = VecDeque::from([self.robot]);
        let mut prev = BTreeMap::from([(self.robot, self.robot)]);
        while let Some(at) = queue.pop_front() {
            if at == self.level.goal {
                let mut path = VecDeque::new();
                let mut n = at;
                while n != self.robot {
                    path.push_front(n);
                    n = prev[&n];
                }
                return Some(path);
            }
            for &(a, b) in &self.level.edges {
                let next = if a == at {
                    b
                } else if b == at {
                    a
                } else {
                    continue;
                };
                if prev.contains_key(&next)
                    || (self.level.cells[next].gap && !self.plan.bridges.contains_key(&next))
                {
                    continue;
                }
                prev.insert(next, at);
                queue.push_back(next);
            }
        }
        None
    }
    pub fn run(&mut self) {
        if self.running {
            self.running = false;
            self.message = "已暂停；再次点击继续。".into();
            return;
        }
        if self.won {
            self.reset_run();
        }
        if self.kind == Kind::Cloud {
            if !self.started {
                self.reset_run();
                let Some(path) = self.path() else {
                    self.message = "路线还没接通。先把一条完整的路补好。".into();
                    return;
                };
                self.route = path;
                self.started = true;
            }
        } else if self.plan.cards.is_empty() {
            self.message = "先添加指令，再让机器人试走。".into();
            return;
        } else {
            self.started = true;
        }
        self.running = true;
        self.clock = 0.;
        self.message = "正在试走；可随时暂停观察。".into();
    }
    fn arrive(&mut self, next: usize) -> bool {
        if self.level.cells[next].gap
            && (!self.plan.bridges.contains_key(&next)
                || (self.level.cells[next].wind
                    && self.plan.bridges.get(&next) != Some(&Bridge::Stone)))
        {
            self.running = false;
            self.message = if self.level.cells[next].wind {
                "前方是风口，需要石桥。机器人停在风口前。"
            } else {
                "前方缺一块桥。机器人留在出错前的位置。"
            }
            .into();
            return false;
        }
        if self.level.gate == Some(next) && !self.gate_open {
            self.running = false;
            self.message = "星门还没打开，先踩下同图案的机关。".into();
            return false;
        }
        if next == self.level.goal && self.level.require_bonus && !self.bonus {
            self.running = false;
            self.message = "本关必须先拿到星晶，再抵达终点。".into();
            return false;
        }
        if self.kind == Kind::Cloud {
            self.facing = if self.level.cells[next].point[0] < self.level.cells[self.robot].point[0]
            {
                2
            } else {
                0
            };
        }
        self.robot = next;
        if self.level.pad == Some(next) {
            self.gate_open = true;
            self.message = "机关已触发，星门打开了！".into();
        }
        if self.level.bonus == Some(next) {
            self.bonus = true;
        }
        if next == self.level.goal {
            self.won = true;
            self.running = false;
            let used = self.plan.bridges.len();
            let efficient = if self.kind == Kind::Cloud {
                used <= 3
            } else {
                self.plan.cards.len() <= self.level.par_cards
            };
            let stars = 2 + u8::from(if self.level.bonus.is_some() {
                self.bonus
            } else {
                efficient
            });
            self.progress.stars[self.stage] = self.progress.stars[self.stage].max(stars);
            self.message = format!(
                "通关！使用{}块桥{} · {}颗星。",
                used,
                if self.kind == Kind::Robot {
                    format!("、{}张指令", self.plan.cards.len())
                } else {
                    String::new()
                },
                stars
            );
        }
        true
    }
    pub fn step(&mut self) -> bool {
        if self.won {
            self.running = false;
            return false;
        }
        if self.kind == Kind::Cloud {
            let Some(next) = self.route.front().copied() else {
                self.running = false;
                return false;
            };
            if self.arrive(next) {
                self.route.pop_front();
                return true;
            }
            return false;
        }
        let Some(card) = self.plan.cards.get(self.pc).copied() else {
            self.running = false;
            self.message = "指令执行完了；还没到终点，可以选一张卡修改。".into();
            return false;
        };
        self.started = true;
        match card.op {
            Op::Left => self.facing = (self.facing + 3) % 4,
            Op::Right => self.facing = (self.facing + 1) % 4,
            Op::Wait => {}
            Op::Forward => {
                let [x, z] = self.level.cells[self.robot].grid;
                let [dx, dz] = [[1, 0], [0, 1], [-1, 0], [0, -1]][self.facing];
                let Some(next) = self
                    .level
                    .cells
                    .iter()
                    .position(|c| c.grid == [x + dx, z + dz])
                else {
                    self.running = false;
                    self.message = "前方没有路。机器人留在出错前的位置。".into();
                    return false;
                };
                if !self
                    .level
                    .edges
                    .iter()
                    .any(|&(a, b)| (a == self.robot && b == next) || (b == self.robot && a == next))
                {
                    self.running = false;
                    self.message = "这里没有相连的路。".into();
                    return false;
                }
                if !self.arrive(next) {
                    return false;
                }
            }
        }
        self.substep += 1;
        if !self.won {
            self.message = format!(
                "第{}张：{} · {}/{}",
                self.pc + 1,
                card.op.label(),
                self.substep,
                card.repeats
            );
        }
        if self.substep >= card.repeats {
            self.pc += 1;
            self.substep = 0;
        }
        true
    }
    pub fn tick(&mut self, dt: f32) -> bool {
        if !self.running {
            return false;
        }
        self.clock += dt.min(0.1);
        if self.clock >= if self.slow { 0.55 } else { 0.28 } {
            self.clock = 0.;
            return self.step() || !self.running;
        }
        false
    }
    pub fn hint(&mut self) {
        self.hint = (self.hint + 1).min(3);
        let hints = match self.kind {
            Kind::Cloud => match self.stage {
                0 => [
                    "先数一数眼前三格缺口。",
                    "撤销会退回材料，可以放心试。",
                    "把三格缺口用木桥补齐，再点试走。",
                ],
                1 => [
                    "比较两条路线的缺口数量。",
                    "近路省材料，绕路能拿额外星晶。",
                    "近路用木桥3；绕路用木桥7。",
                ],
                2 => [
                    "蓝色风纹的格子需要石桥。",
                    "石桥要留给风口；普通格可以用木桥。",
                    "近路用木桥2、石桥1；绕路用木桥7。",
                ],
                3 => [
                    "木桥只够两格，眼前有三格缺口。",
                    "普通缺口也能用石桥。",
                    "近路用木桥2、石桥1，石桥放任意一格。",
                ],
                4 => [
                    "近路有两处风口，石桥只有一块。",
                    "比较绕路缺口数和你所有的材料。",
                    "绕路用木桥6、石桥1；普通缺口也能放石桥。",
                ],
                _ => [
                    "这次从右岸出发，终点在左岸。",
                    "绕路有一处风口，要给它留一块石桥。",
                    "风纹格用石桥，其余六处绕路缺口用木桥。",
                ],
            },
            Kind::Robot => match self.stage {
                0 => [
                    "机器人朝右，数一数到小岛要走几格。",
                    "连续前进可以放在同一张卡里。",
                    "桥放好后：前进×3。",
                ],
                1 => [
                    "先看路线在哪里转弯。",
                    "上方支路可以拿星晶，最多用8张卡。",
                    "取星路线：前进×3、右转、前进、左转、前进×3、右转、前进。",
                ],
                2 => [
                    "先看圆形机关和星门的位置。",
                    "必须先踩下机关；同一张前进可重复。",
                    "前进×3、右转、前进×2、左转、前进×3。",
                ],
                3 => [
                    "这关只能放两张卡，前进需要三步。",
                    "选前进，再点重复，可以让一张卡执行多次。",
                    "一张前进×3就能到达；也可用前进×2加前进。",
                ],
                4 => [
                    "本关的星晶必须拿到，直路不能直接过关。",
                    "取星支路要用4块桥，连续前进要合并。",
                    "前进×3、右转、前进、左转、前进×3、右转、前进，共7张。",
                ],
                _ => [
                    "机器人朝左，先让它转身。",
                    "一张右转×2或左转×2就能转身。",
                    "右转×2、前进×3、右转、前进×2、左转、前进×3，共6张。",
                ],
            },
        };
        self.message = hints[(self.hint - 1) as usize].into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fill(p: &mut Puzzle, nodes: &[usize], b: Bridge) {
        p.material = b;
        for &i in nodes {
            assert!(p.place(i));
        }
    }
    fn solve(p: &mut Puzzle) {
        p.run();
        for _ in 0..64 {
            if !p.running {
                break;
            }
            p.step();
        }
    }
    fn guide(p: &mut Puzzle) {
        p.plan.cards = vec![Card {
            op: Op::Forward,
            repeats: 3,
        }];
        if p.stage > 0 {
            p.plan.cards.extend([
                Card {
                    op: Op::Right,
                    repeats: 1,
                },
                Card {
                    op: Op::Forward,
                    repeats: 2,
                },
                Card {
                    op: Op::Left,
                    repeats: 1,
                },
                Card {
                    op: Op::Forward,
                    repeats: 3,
                },
            ]);
        }
    }
    #[test]
    fn cloud_all_three_stages_and_both_routes_work() {
        for stage in 0..3 {
            let mut p = Puzzle::new(Kind::Cloud);
            p.choose(stage);
            fill(&mut p, &[2, 4], Bridge::Wood);
            fill(
                &mut p,
                &[3],
                if stage == 2 {
                    Bridge::Stone
                } else {
                    Bridge::Wood
                },
            );
            solve(&mut p);
            assert!(p.won);
            if stage > 0 {
                p.restart();
                fill(&mut p, &[8, 9, 10, 11, 15, 16, 17], Bridge::Wood);
                solve(&mut p);
                assert!(p.won && p.bonus);
            }
        }
    }
    #[test]
    fn stock_is_bounded_and_undo_and_erase_refund() {
        let mut p = Puzzle::new(Kind::Cloud);
        p.choose(0);
        fill(&mut p, &[2, 3, 4], Bridge::Wood);
        assert_eq!(p.remaining(Bridge::Wood), 0);
        p.undo();
        assert_eq!(p.remaining(Bridge::Wood), 1);
        p.erase = true;
        assert!(p.place(2));
        assert_eq!(p.remaining(Bridge::Wood), 2);
        assert!(!p.place(0));
        assert_eq!(p.plan.bridges.len(), 1);
    }
    #[test]
    fn wind_stops_before_unsafe_bridge_and_keeps_work() {
        let mut p = Puzzle::new(Kind::Cloud);
        p.choose(2);
        fill(&mut p, &[2, 3, 4], Bridge::Wood);
        solve(&mut p);
        assert!(!p.won);
        assert_eq!(p.robot, 2);
        assert_eq!(p.plan.bridges.len(), 3);
        assert!(p.message.contains("风口"));
        p.material = Bridge::Stone;
        assert!(p.place(3));
        solve(&mut p);
        assert!(p.won);
    }
    #[test]
    fn robot_all_three_and_alternative_route_work() {
        for i in 0..3 {
            let mut p = Puzzle::new(Kind::Robot);
            p.choose(i);
            fill(
                &mut p,
                if i == 0 { &[1, 2] } else { &[1, 2, 6] },
                Bridge::Wood,
            );
            guide(&mut p);
            solve(&mut p);
            assert!(p.won, "stage {i}: {}", p.message);
        }
        let mut p = Puzzle::new(Kind::Robot);
        p.choose(1);
        fill(&mut p, &[1, 2, 9, 10], Bridge::Wood);
        p.plan.cards = vec![
            Card {
                op: Op::Forward,
                repeats: 3,
            },
            Card {
                op: Op::Right,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 1,
            },
            Card {
                op: Op::Left,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 3,
            },
            Card {
                op: Op::Right,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 1,
            },
        ];
        solve(&mut p);
        assert!(p.won && p.bonus);
    }
    #[test]
    fn failure_does_not_consume_action_and_reset_restores_gate() {
        let mut p = Puzzle::new(Kind::Robot);
        p.choose(2);
        guide(&mut p);
        assert!(!p.step());
        assert_eq!((p.robot, p.pc, p.substep), (0, 0, 0));
        fill(&mut p, &[1, 2, 6], Bridge::Wood);
        guide(&mut p);
        for _ in 0..6 {
            assert!(p.step());
        }
        assert!(p.gate_open);
        p.reset_run();
        assert!(!p.gate_open && p.robot == 0);
        assert_eq!(p.plan.bridges.len(), 3);
        assert_eq!(p.plan.cards.len(), 5);
    }
    #[test]
    fn command_cap_and_save_validation_reject_invalid_data() {
        let mut p = Puzzle::new(Kind::Robot);
        p.choose(2);
        for _ in 0..9 {
            p.add(Op::Wait);
        }
        assert_eq!(p.plan.cards.len(), 8);
        p.repeat();
        assert_eq!(p.plan.cards[7].repeats, 2);
        let saved = p.snapshot();
        assert!(saved.valid(Kind::Robot));
        let restored: Progress =
            serde_json::from_slice(&serde_json::to_vec(&saved).unwrap()).unwrap();
        assert!(restored.valid(Kind::Robot));
        let mut bad = restored;
        bad.plans[2].bridges.insert(99, Bridge::Wood);
        assert!(!bad.valid(Kind::Robot));
    }
    #[test]
    fn all_six_numbers_are_reachable_without_renumbering_old_saves() {
        for kind in [Kind::Cloud, Kind::Robot] {
            let numbers: Vec<_> = (1..=LEVEL_COUNT)
                .map(|n| {
                    let i = Level::index_for_number(kind, n).unwrap();
                    assert_eq!(Level::new(kind, i).number, n);
                    i
                })
                .collect();
            assert_eq!(numbers.len(), LEVEL_COUNT);
            for i in 0..LEVEL_COUNT {
                let l = Level::new(kind, i);
                assert!(l.active(l.start) && l.active(l.goal));
                assert!(!l.cells[l.start].gap && !l.cells[l.goal].gap);
            }
        }
        assert_eq!(
            (0..3)
                .map(|i| Level::new(Kind::Cloud, i).number)
                .collect::<Vec<_>>(),
            [1, 2, 4]
        );
        assert_eq!(
            (0..3)
                .map(|i| Level::new(Kind::Robot, i).number)
                .collect::<Vec<_>>(),
            [1, 3, 6]
        );
    }
    #[test]
    fn new_cloud_levels_have_legal_resource_solutions_and_return_path() {
        for stage in 3..LEVEL_COUNT {
            let mut p = Puzzle::new(Kind::Cloud);
            p.choose(stage);
            if stage == 3 {
                fill(&mut p, &[2, 4], Bridge::Wood);
                fill(&mut p, &[3], Bridge::Stone);
            } else {
                let stone = if stage == 4 { 8 } else { 10 };
                for node in [8, 9, 10, 11, 15, 16, 17] {
                    fill(
                        &mut p,
                        &[node],
                        if node == stone {
                            Bridge::Stone
                        } else {
                            Bridge::Wood
                        },
                    );
                }
            }
            assert!(p.snapshot().valid(Kind::Cloud));
            solve(&mut p);
            assert!(p.won, "{}: {}", p.level.title, p.message);
            assert_eq!(p.robot, p.level.goal);
            if stage > 3 {
                assert!(p.bonus);
            }
            assert_eq!(p.progress.stars[stage], 3);
        }
        let mut p = Puzzle::new(Kind::Cloud);
        p.choose(4);
        fill(&mut p, &[2, 3], Bridge::Wood);
        fill(&mut p, &[4], Bridge::Stone);
        solve(&mut p);
        assert!(!p.won && p.message.contains("风口"));
    }
    #[test]
    fn two_card_challenge_accepts_compressed_program_and_rejects_third_card() {
        let mut p = Puzzle::new(Kind::Robot);
        p.choose(3);
        fill(&mut p, &[1, 2], Bridge::Wood);
        p.add(Op::Forward);
        p.add(Op::Forward);
        p.add(Op::Forward);
        assert_eq!(p.plan.cards.len(), 2);
        assert!(p.message.contains("最多2张"));
        p.plan.cards[0].repeats = 2;
        solve(&mut p);
        assert!(p.won && p.progress.stars[3] == 3);
        p.clear();
        p.plan.cards = vec![Card {
            op: Op::Forward,
            repeats: 3,
        }];
        solve(&mut p);
        assert!(p.won);
    }
    #[test]
    fn mandatory_crystal_blocks_direct_finish_but_branch_passes() {
        let mut p = Puzzle::new(Kind::Robot);
        p.choose(4);
        fill(&mut p, &[1, 2, 6], Bridge::Wood);
        guide(&mut p);
        solve(&mut p);
        assert!(!p.won && !p.bonus);
        assert_eq!(p.robot, 7);
        assert!(p.message.contains("必须先拿到星晶"));
        p.erase = true;
        assert!(p.place(6));
        p.erase = false;
        fill(&mut p, &[9, 10], Bridge::Wood);
        p.plan.cards = vec![
            Card {
                op: Op::Forward,
                repeats: 3,
            },
            Card {
                op: Op::Right,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 1,
            },
            Card {
                op: Op::Left,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 3,
            },
            Card {
                op: Op::Right,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 1,
            },
        ];
        assert!(p.snapshot().valid(Kind::Robot));
        solve(&mut p);
        assert!(p.won && p.bonus && p.progress.stars[4] == 3);
    }
    #[test]
    fn turn_around_level_starts_left_and_replays_after_reset() {
        let mut p = Puzzle::new(Kind::Robot);
        p.choose(5);
        fill(&mut p, &[1, 2, 6], Bridge::Wood);
        assert_eq!(p.facing, 2);
        let route = vec![
            Card {
                op: Op::Right,
                repeats: 2,
            },
            Card {
                op: Op::Forward,
                repeats: 3,
            },
            Card {
                op: Op::Right,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 2,
            },
            Card {
                op: Op::Left,
                repeats: 1,
            },
            Card {
                op: Op::Forward,
                repeats: 3,
            },
        ];
        p.plan.cards = route;
        solve(&mut p);
        assert!(p.won && p.progress.stars[5] == 3);
        p.reset_run();
        assert_eq!(p.facing, 2);
        assert!(!p.gate_open);
        p.plan.cards[0].op = Op::Left;
        solve(&mut p);
        assert!(p.won);
    }
    #[test]
    fn three_level_saves_expand_without_losing_old_plans_or_stars() {
        for kind in [Kind::Cloud, Kind::Robot] {
            let mut old = Progress::new(kind);
            old.stage = 2;
            old.stars = vec![1, 2, 3];
            old.plans.truncate(3);
            old.plans[2].bridges.insert(
                if kind == Kind::Cloud { 3 } else { 1 },
                if kind == Kind::Cloud {
                    Bridge::Stone
                } else {
                    Bridge::Wood
                },
            );
            let original = serde_json::to_value(&old.plans).unwrap();
            assert!(old.valid(kind));
            let decoded: Progress =
                serde_json::from_slice(&serde_json::to_vec(&old).unwrap()).unwrap();
            let mut p = Puzzle::restore(decoded);
            assert_eq!(p.progress.stars, [1, 2, 3, 0, 0, 0]);
            assert_eq!(p.snapshot().plans.len(), 6);
            assert!(p.snapshot().valid(kind));
            for i in 3..6 {
                p.choose(i);
            }
            p.choose(2);
            let saved = p.snapshot();
            assert_eq!(serde_json::to_value(&saved.plans[..3]).unwrap(), original);
            assert_eq!(&saved.stars[..3], &[1, 2, 3]);
        }
    }
    #[test]
    fn save_validation_enforces_new_stage_limits_and_lengths() {
        let mut save = Progress::new(Kind::Robot);
        save.plans[3].cards = vec![
            Card {
                op: Op::Wait,
                repeats: 1
            };
            3
        ];
        assert!(!save.valid(Kind::Robot));
        save.plans[3].cards.clear();
        save.stage = 6;
        assert!(!save.valid(Kind::Robot));
        save.stage = 0;
        save.stars.truncate(3);
        assert!(!save.valid(Kind::Robot));
    }
}
