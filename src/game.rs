//! Deterministic game rules, independent of rendering and platform APIs.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Material {
    Grass,
    Wood,
    Stone,
    Crystal,
    Brick,
    Sand,
    Snow,
    Glass,
    Lamp,
    Leaf,
}
impl Material {
    pub fn label(self) -> &'static str {
        match self {
            Self::Grass => "草地",
            Self::Wood => "木头",
            Self::Stone => "石头",
            Self::Crystal => "水晶",
            Self::Brick => "砖块",
            Self::Sand => "沙土",
            Self::Snow => "雪块",
            Self::Glass => "玻璃",
            Self::Lamp => "灯笼",
            Self::Leaf => "树叶",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Instruction {
    Place,
    Forward,
    Left,
    Right,
    Back,
    Jump,
    Wait,
}
impl Instruction {
    pub fn label(self) -> &'static str {
        match self {
            Self::Place => "放木块",
            Self::Forward => "前进一步",
            Self::Left => "向左转",
            Self::Right => "向右转",
            Self::Back => "后退一步",
            Self::Jump => "跳过一格",
            Self::Wait => "停留一下",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub pos: [i32; 3],
    pub material: Material,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct World {
    pub blocks: Vec<Block>,
    pub robot: [i32; 2],
    /// East, south, west, north.
    pub facing: usize,
    pub goal: [i32; 2],
    /// Star crystals are collected when the robot lands on their tile.
    #[serde(default)]
    pub crystals: Vec<[i32; 2]>,
    /// A blank construction board with no robot, flag, or task rules.
    #[serde(default)]
    pub free_build: bool,
}
impl World {
    /// Number of grounded bridge blocks laid across the fifth level's gap.
    pub fn bridge_progress(&self) -> usize {
        (-2..=2)
            .filter(|x| {
                self.blocks
                    .iter()
                    .any(|block| block.pos[0] == *x && block.pos[1] == 0)
            })
            .count()
    }

    /// Find a safe walking route from the robot to the goal over the built floor.
    pub fn path_to_goal(&self) -> Option<Vec<[i32; 2]>> {
        let start = self.robot;
        let goal = self.goal;
        let mut queue = VecDeque::from([start]);
        let mut previous = BTreeMap::<[i32; 2], [i32; 2]>::new();
        previous.insert(start, start);
        while let Some([x, z]) = queue.pop_front() {
            if [x, z] == goal {
                let mut path = vec![goal];
                let mut at = goal;
                while at != start {
                    at = *previous.get(&at)?;
                    path.push(at);
                }
                path.reverse();
                return Some(path);
            }
            for next in [[x + 1, z], [x, z + 1], [x - 1, z], [x, z - 1]] {
                if previous.contains_key(&next)
                    || self.material([next[0], 0, next[1]]).is_none()
                    || self.material([next[0], 1, next[1]]).is_some()
                {
                    continue;
                }
                previous.insert(next, [x, z]);
                queue.push_back(next);
            }
        }
        None
    }

    pub fn material(&self, pos: [i32; 3]) -> Option<Material> {
        self.blocks
            .iter()
            .find(|b| b.pos == pos)
            .map(|b| b.material)
    }
    pub fn put(&mut self, pos: [i32; 3], material: Material) -> bool {
        if !self.accepts(pos) || self.material(pos).is_some() || self.blocks.len() >= 1800 {
            return false;
        }
        self.blocks.push(Block { pos, material });
        true
    }
    pub fn in_bounds(p: [i32; 3]) -> bool {
        (-6..=6).contains(&p[0]) && (0..=8).contains(&p[1]) && (-3..=3).contains(&p[2])
    }
    fn accepts(&self, p: [i32; 3]) -> bool {
        if self.free_build {
            (-8..=8).contains(&p[0]) && (0..=12).contains(&p[1]) && (-5..=5).contains(&p[2])
        } else {
            Self::in_bounds(p)
        }
    }
    pub fn edit(
        &mut self,
        pos: [i32; 3],
        material: Material,
        erase: bool,
    ) -> Result<(), &'static str> {
        if !self.accepts(pos) {
            return Err(if self.free_build {
                "这里超出搭建范围了"
            } else {
                "这里超出小岛范围了"
            });
        }
        if !self.free_build
            && pos[1] == 0
            && ([pos[0], pos[2]] == self.robot || [pos[0], pos[2]] == self.goal)
        {
            return Err("给机器人和小旗留一点空间吧");
        }
        if !self.free_build
            && !erase
            && pos[1] > 0
            && ([pos[0], pos[2]] == self.robot || [pos[0], pos[2]] == self.goal)
        {
            return Err("给机器人和小旗留一点空间吧");
        }
        if erase {
            if self.material(pos).is_none() {
                return Err("这里还没有方块");
            }
            self.blocks.retain(|b| b.pos != pos);
        } else {
            // Magnetic blocks can attach to any face, leaving empty space below.
            let connected = [
                [1, 0, 0],
                [-1, 0, 0],
                [0, 1, 0],
                [0, -1, 0],
                [0, 0, 1],
                [0, 0, -1],
            ]
            .iter()
            .any(|d| {
                self.material([pos[0] + d[0], pos[1] + d[1], pos[2] + d[2]])
                    .is_some()
            });
            if pos[1] > 0 && !connected {
                return Err("贴着已有方块的任意一面搭建吧");
            }
            if !self.put(pos, material) {
                return Err("这里已经有方块，或达到搭建上限了");
            }
        }
        Ok(())
    }
    pub fn execute(&mut self, op: Instruction) -> Result<(), &'static str> {
        let directions = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        let d = directions[self.facing % 4];
        let next = [self.robot[0] + d[0], self.robot[1] + d[1]];
        let back = [self.robot[0] - d[0], self.robot[1] - d[1]];
        match op {
            Instruction::Left => self.facing = (self.facing + 3) % 4,
            Instruction::Right => self.facing = (self.facing + 1) % 4,
            Instruction::Place => {
                if !Self::in_bounds([next[0], 0, next[1]]) {
                    return Err("前面超出小岛了，试试转弯");
                }
                if self.material([next[0], 0, next[1]]).is_some() {
                    return Err("前面已经有方块了，试试前进");
                }
                self.put([next[0], 0, next[1]], Material::Wood);
            }
            Instruction::Forward => {
                if self.material([next[0], 0, next[1]]).is_none() {
                    return Err("前面是缺口！先放木块，再前进");
                }
                if self.material([next[0], 1, next[1]]).is_some() {
                    return Err("前面有挡路的方块，试试绕开它");
                }
                self.robot = next;
            }
            Instruction::Back => {
                if self.material([back[0], 0, back[1]]).is_none() {
                    return Err("后面是缺口，不能后退");
                }
                if self.material([back[0], 1, back[1]]).is_some() {
                    return Err("后面的方块挡住了，不能后退");
                }
                self.robot = back;
            }
            Instruction::Jump => {
                let landing = [self.robot[0] + d[0] * 2, self.robot[1] + d[1] * 2];
                if !Self::in_bounds([landing[0], 0, landing[1]])
                    || self.material([landing[0], 0, landing[1]]).is_none()
                    || self.material([landing[0], 1, landing[1]]).is_some()
                {
                    return Err("前面没有合适的落脚点，跳不过去");
                }
                self.robot = landing;
            }
            Instruction::Wait => {}
        }
        if matches!(
            op,
            Instruction::Forward | Instruction::Back | Instruction::Jump
        ) {
            self.crystals.retain(|crystal| *crystal != self.robot);
        }
        Ok(())
    }
    pub fn won(&self) -> bool {
        self.robot == self.goal && self.crystals.is_empty()
    }
    pub fn validate(&self) -> bool {
        self.blocks.len() <= 1800
            && self.facing < 4
            && (self.free_build
                || (Self::in_bounds([self.robot[0], 0, self.robot[1]])
                    && Self::in_bounds([self.goal[0], 0, self.goal[1]])
                    && self.material([self.robot[0], 0, self.robot[1]]).is_some()
                    && self.material([self.robot[0], 1, self.robot[1]]).is_none()
                    && self.material([self.goal[0], 0, self.goal[1]]).is_some()))
            && self
                .crystals
                .iter()
                .all(|crystal| Self::in_bounds([crystal[0], 0, crystal[1]]))
            && self.blocks.iter().all(|b| self.accepts(b.pos))
            && self
                .blocks
                .iter()
                .map(|b| b.pos)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                == self.blocks.len()
    }
}

/// A roomy, flat board for making anything without a mission or a finish line.
pub fn free_build_world() -> World {
    let mut world = World {
        blocks: vec![],
        robot: [0, 0],
        facing: 0,
        goal: [0, 0],
        crystals: vec![],
        free_build: true,
    };
    for x in -8_i32..=8 {
        for z in -5_i32..=5 {
            world.put([x, 0, z], Material::Grass);
        }
    }
    world
}

pub const LEVEL_NAMES: [&str; 6] = [
    "新手村 · 找到小旗",
    "森林边 · 修好小路",
    "风车岛 · 学会转弯",
    "云桥谷 · 跨过小河",
    "矿车站 · 重复铺路",
    "我的家园 · 自由建造",
];
pub const LEVEL_HINTS: [&str; 6] = [
    "先收集路上的星晶，再抵达小旗。点「单步」看路线。",
    "修好木桥，拿到星晶后到对岸的小旗。",
    "先左转，沿着小路收集星晶。",
    "把「放木块、前进」重复 5 次，沿桥收集两颗星晶。",
    "用重复铺路，拿到矿车站中间的星晶。",
    "创作挑战：用至少 3 种材料搭 10 块，解锁建造大师徽章。",
];

pub const LEVEL_CRYSTALS: [usize; 6] = [1, 1, 1, 2, 1, 0];
pub const LEVEL_STAR_PAR: [usize; 6] = [2, 3, 3, 11, 7, 0];

pub fn level(index: usize) -> World {
    let mut w = World {
        blocks: vec![],
        robot: [-3, 0],
        facing: 0,
        goal: [3, 0],
        crystals: vec![],
        free_build: false,
    };
    for x in -6_i32..=6 {
        for z in -3_i32..=3 {
            if x.abs() >= 3 && !(x.abs() == 6 && z.abs() == 3) {
                w.put([x, 0, z], Material::Grass);
            }
        }
    }
    match index {
        0 => {
            w.robot = [-5, 0];
            w.goal = [-3, 0];
            w.crystals = vec![[-4, 0]];
        }
        1 => {
            w.robot = [-4, 0];
            w.goal = [-2, 0];
            w.blocks.retain(|b| b.pos != [-3, 0, 0]);
            w.put([-2, 0, 0], Material::Grass);
            w.crystals = vec![[-2, 0]];
        }
        2 => {
            w.robot = [-4, 1];
            w.goal = [-4, -1];
            w.crystals = vec![[-4, 0]];
        }
        3 => w.crystals = vec![[-1, 0], [1, 0]],
        4 => {
            w.robot = [-2, 0];
            w.goal = [2, 0];
            w.put([-2, 0, 0], Material::Grass);
            w.put([2, 0, 0], Material::Grass);
            w.crystals = vec![[0, 0]];
        }
        _ => {}
    }
    w
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub world: World,
    pub program: Vec<Instruction>,
    pub repeat: usize,
    #[serde(default)]
    pub finish_forward: bool,
    pub level: usize,
    pub completed: Vec<usize>,
    #[serde(default)]
    pub stars: [u8; 6],
}
impl Default for Work {
    fn default() -> Self {
        Self {
            world: level(0),
            program: vec![Instruction::Forward, Instruction::Forward],
            repeat: 1,
            finish_forward: false,
            level: 0,
            completed: vec![],
            stars: [0; 6],
        }
    }
}
impl Work {
    pub fn valid(&self) -> bool {
        self.world.validate()
            && self.program.len() <= 8
            && (1..=8).contains(&self.repeat)
            && self.level < 6
            && self.completed.iter().all(|i| *i < 6)
            && self.stars.iter().all(|stars| *stars <= 3)
    }
    pub fn choose_level(&mut self, index: usize) {
        self.level = index.min(5);
        self.world = level(self.level);
        self.repeat = 1;
        self.finish_forward = matches!(self.level, 3 | 4);
        self.program = match self.level {
            0 => vec![Instruction::Forward, Instruction::Forward],
            1 => vec![
                Instruction::Place,
                Instruction::Forward,
                Instruction::Forward,
            ],
            2 => vec![
                Instruction::Left,
                Instruction::Forward,
                Instruction::Forward,
            ],
            _ => vec![Instruction::Place, Instruction::Forward],
        };
    }

    /// Loads a working starter route for the teaching levels. It lets a new
    /// player watch a solution first, then change one card at a time.
    pub fn load_guide_route(&mut self) -> bool {
        let (program, repeat, finish_forward) = match self.level {
            0 => (vec![Instruction::Forward, Instruction::Forward], 1, false),
            1 => (
                vec![
                    Instruction::Place,
                    Instruction::Forward,
                    Instruction::Forward,
                ],
                1,
                false,
            ),
            2 => (
                vec![
                    Instruction::Left,
                    Instruction::Forward,
                    Instruction::Forward,
                ],
                1,
                false,
            ),
            3 => (vec![Instruction::Place, Instruction::Forward], 5, true),
            4 => (vec![Instruction::Place, Instruction::Forward], 3, true),
            _ => return false,
        };
        self.program = program;
        self.repeat = repeat;
        self.finish_forward = finish_forward;
        true
    }

    pub fn creative_progress(&self) -> (usize, usize) {
        let blocks = self
            .world
            .blocks
            .iter()
            .filter(|block| block.pos[1] > 0)
            .count();
        let materials = self
            .world
            .blocks
            .iter()
            .filter(|block| block.pos[1] > 0)
            .map(|block| block.material)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        (blocks, materials)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    pub version: u32,
    pub active: usize,
    pub profiles: BTreeMap<usize, Work>,
    /// One no-goal construction board for each local profile.
    #[serde(default)]
    pub free_builds: BTreeMap<usize, World>,
}
impl Default for Save {
    fn default() -> Self {
        Self {
            version: 1,
            active: 0,
            profiles: BTreeMap::from([
                (0, {
                    let mut work = Work::default();
                    work.choose_level(3);
                    work.repeat = 5;
                    work
                }),
                (1, {
                    let mut w = Work::default();
                    w.choose_level(3);
                    w.repeat = 5;
                    w
                }),
            ]),
            free_builds: BTreeMap::from([(0, free_build_world()), (1, free_build_world())]),
        }
    }
}
impl Save {
    pub fn validate(&self) -> bool {
        self.version == 1
            && self.active < 2
            && self.profiles.len() == 2
            && (0..2).all(|i| self.profiles.get(&i).is_some_and(Work::valid))
            && self.free_builds.values().all(World::validate)
    }
}

/// Execution is bounded and retains a snapshot for deterministic reset.
pub struct Runner {
    pub before: World,
    pub ops: Vec<Instruction>,
    pub pc: usize,
    pub running: bool,
    pub faulted: bool,
}
impl Runner {
    pub fn new(work: &Work) -> Self {
        let mut ops: Vec<_> = work
            .program
            .iter()
            .copied()
            .cycle()
            .take(work.program.len().min(16) * work.repeat.min(8))
            .collect();
        if work.finish_forward && !ops.is_empty() {
            ops.push(Instruction::Forward);
        }
        Self {
            before: work.world.clone(),
            ops,
            pc: 0,
            running: false,
            faulted: false,
        }
    }
    pub fn step(&mut self, world: &mut World) -> Result<bool, &'static str> {
        if self.faulted {
            return Err("先点「复位」，再修改指令试试");
        }
        let Some(op) = self.ops.get(self.pc) else {
            self.running = false;
            return Ok(true);
        };
        if let Err(e) = world.execute(*op) {
            self.running = false;
            self.faulted = true;
            return Err(e);
        }
        self.pc += 1;
        let done = self.pc == self.ops.len() || world.won();
        if done {
            self.running = false;
        }
        Ok(done)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hollow_structure_survives_edit_and_save() {
        let mut w = level(5);
        w.edit([0, 0, 2], Material::Wood, false).unwrap();
        w.edit([0, 1, 2], Material::Wood, false).unwrap();
        w.edit([0, 2, 2], Material::Wood, false).unwrap();
        // Extend a roof with no blocks underneath, then attach below it.
        w.edit([1, 2, 2], Material::Wood, false).unwrap();
        w.edit([2, 2, 2], Material::Wood, false).unwrap();
        w.edit([2, 1, 2], Material::Wood, false).unwrap();
        w.edit([0, 1, 2], Material::Wood, true).unwrap();
        assert_eq!(w.material([1, 1, 2]), None);
        assert_eq!(w.material([0, 2, 2]), Some(Material::Wood));
        let restored: World = serde_json::from_str(&serde_json::to_string(&w).unwrap()).unwrap();
        assert_eq!(restored, w);
        assert!(restored.validate());
        let before = w.clone();
        assert!(w.edit([6, 4, 3], Material::Wood, false).is_err());
        assert_eq!(w, before);
    }

    #[test]
    fn bridge_loop_then_final_step_reaches_flag() {
        let mut work = Work::default();
        work.choose_level(3);
        work.repeat = 5;
        let mut r = Runner::new(&work);
        while r.pc < r.ops.len() {
            r.step(&mut work.world).unwrap();
        }
        assert_eq!(work.world.robot, [3, 0]);
        assert!(work.world.won());
        assert_eq!(
            work.world
                .blocks
                .iter()
                .filter(|b| b.material == Material::Wood)
                .count(),
            5
        );
    }
    #[test]
    fn guide_route_reaches_every_teaching_level_goal() {
        for level in 0..5 {
            let mut work = Work::default();
            work.choose_level(level);
            assert!(work.load_guide_route());
            let mut runner = Runner::new(&work);
            for _ in 0..32 {
                if work.world.won() {
                    break;
                }
                runner.step(&mut work.world).unwrap();
            }
            assert!(work.world.won());
        }
    }
    #[test]
    fn reaching_flag_requires_collecting_star_crystals() {
        let mut w = level(0);
        w.robot = w.goal;
        assert!(!w.won());
        w.crystals.clear();
        assert!(w.won());
    }
    #[test]
    fn old_world_save_without_crystals_still_loads() {
        let old = r#"{"blocks":[],"robot":[0,0],"facing":0,"goal":[0,0]}"#;
        let restored: World = serde_json::from_str(old).unwrap();
        assert!(restored.crystals.is_empty());
        assert!(!restored.free_build);
    }
    #[test]
    fn free_build_board_has_no_robot_or_flag_protection() {
        let mut world = free_build_world();
        assert!(world.free_build);
        assert!(world.validate());
        world.edit([0, 1, 0], Material::Crystal, false).unwrap();
        world.edit([0, 1, 0], Material::Crystal, true).unwrap();
        world.edit([8, 1, 5], Material::Lamp, false).unwrap();
        assert!(world.edit([9, 0, 0], Material::Wood, false).is_err());
        assert!(world.validate());
    }
    #[test]
    fn legacy_saves_gain_a_free_build_board_without_becoming_invalid() {
        let mut save = Save::default();
        save.free_builds.clear();
        let legacy_json = serde_json::to_string(&save).unwrap();
        let restored: Save = serde_json::from_str(&legacy_json).unwrap();
        assert!(restored.validate());
        assert!(restored.free_builds.is_empty());
    }
    #[test]
    fn failure_does_not_move_or_skip_instruction() {
        let mut w = level(3);
        let before = w.clone();
        let work = Work {
            world: w.clone(),
            program: vec![Instruction::Forward],
            ..Default::default()
        };
        let mut r = Runner::new(&work);
        assert!(r.step(&mut w).is_err());
        assert_eq!(w, before);
        assert_eq!(r.pc, 0);
        assert!(r.faulted);
    }
    #[test]
    fn simple_levels_have_valid_solutions() {
        for i in 0..3 {
            let mut w = Work::default();
            w.choose_level(i);
            let mut r = Runner::new(&w);
            for _ in 0..r.ops.len() {
                r.step(&mut w.world).unwrap();
            }
            assert!(w.world.won(), "level {i}");
        }
    }
    #[test]
    fn invalid_saves_rejected_and_profiles_independent() {
        let mut s = Save::default();
        assert!(s.validate());
        s.profiles.get_mut(&0).unwrap().choose_level(2);
        assert_eq!(s.profiles[&1].level, 3);
        let decoded: Save = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert!(decoded.validate());
        s.profiles.get_mut(&0).unwrap().world.facing = 9;
        assert!(!s.validate());
    }
    #[test]
    fn cannot_overwrite_bridge_or_block_robot() {
        let mut w = level(3);
        w.execute(Instruction::Place).unwrap();
        assert!(w.execute(Instruction::Place).is_err());
        assert!(w.edit([-3, 1, 0], Material::Stone, false).is_err());
    }

    #[test]
    fn erase_allows_upper_layers_above_robot_and_goal() {
        let mut w = level(5);
        w.edit([1, 0, 0], Material::Wood, false).unwrap();
        w.edit([1, 1, 0], Material::Wood, false).unwrap();
        w.edit([1, 1, 0], Material::Wood, true).unwrap();
        assert_eq!(w.material([1, 1, 0]), None);
    }
    #[test]
    fn execution_is_bounded_and_snapshot_is_unchanged() {
        let work = Work {
            repeat: 100000,
            program: vec![Instruction::Left; 8],
            ..Default::default()
        };
        let mut r = Runner::new(&work);
        assert_eq!(r.ops.len(), 64);
        let before = r.before.clone();
        let mut world = work.world.clone();
        r.step(&mut world).unwrap();
        assert_eq!(r.before, before);
    }
}
