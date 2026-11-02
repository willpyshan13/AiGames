//! Six authored island graphs expanded into one orthogonal, walkable grid.
use crate::puzzle_game::Bridge;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

pub const COUNT: usize = 6;
pub type Grid = [i32; 2];
#[derive(Clone, Debug, Deserialize)]
pub struct Spec {
    pub n: usize,
    pub title: String,
    pub nodes: BTreeMap<String, Grid>,
    pub edges: Vec<[String; 2]>,
    pub start: String,
    pub goal: String,
    pub wood: usize,
    pub stone: usize,
    pub winds: Vec<usize>,
    pub gem: Option<String>,
    pub required: bool,
    pub tip: String,
}
pub fn specs() -> &'static [Spec] {
    static MAPS: OnceLock<Vec<Spec>> = OnceLock::new();
    MAPS.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/builders/cloud-walk-v3/maps.json"))
            .expect("confirmed maps must parse")
    })
}
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub at: Grid,
    pub gap: bool,
    pub wind: bool,
}
#[derive(Clone, Debug)]
pub struct Map {
    pub spec: Spec,
    pub cells: Vec<Cell>,
    pub islands: Vec<Grid>,
    pub spans: Vec<[usize; 3]>,
    pub start: usize,
    pub goal: usize,
    pub gem: Option<usize>,
}
impl Map {
    pub fn new(stage: usize) -> Self {
        let spec = specs()[stage].clone();
        let mut grid = BTreeMap::<Grid, (bool, bool)>::new();
        for &[x, z] in spec.nodes.values() {
            for dx in -1..=1 {
                for dz in -1..=1 {
                    assert!(grid.insert([x + dx, z + dz], (false, false)).is_none());
                }
            }
        }
        let mut spans = vec![];
        for (edge, [a, b]) in spec.edges.iter().enumerate() {
            let a = spec.nodes[a];
            let b = spec.nodes[b];
            let dx = (b[0] - a[0]).signum();
            let dz = (b[1] - a[1]).signum();
            assert!((dx == 0) ^ (dz == 0));
            assert_eq!((b[0] - a[0]).abs() + (b[1] - a[1]).abs(), 6);
            let cells: [Grid; 3] =
                std::array::from_fn(|i| [a[0] + dx * (i as i32 + 2), a[1] + dz * (i as i32 + 2)]);
            for (i, &at) in cells.iter().enumerate() {
                assert!(grid
                    .insert(at, (true, spec.winds.contains(&edge) && i == 1))
                    .is_none());
            }
            spans.push(cells);
        }
        let cells: Vec<_> = grid
            .into_iter()
            .map(|(at, (gap, wind))| Cell { at, gap, wind })
            .collect();
        let id = |at: Grid| {
            cells
                .iter()
                .position(|c| c.at == at)
                .expect("authored cell must exist")
        };
        let spans = spans.into_iter().map(|p| p.map(id)).collect();
        let start = id(spec.nodes[&spec.start]);
        let goal = id(spec.nodes[&spec.goal]);
        let gem = spec.gem.as_ref().map(|n| id(spec.nodes[n]));
        let islands = spec.nodes.values().copied().collect();
        Self {
            spec,
            cells,
            islands,
            spans,
            start,
            goal,
            gem,
        }
    }
    pub fn id(&self, at: Grid) -> Option<usize> {
        self.cells.iter().position(|c| c.at == at)
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub bridges: BTreeMap<usize, Bridge>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Progress {
    pub version: u8,
    pub stage: usize,
    pub stars: Vec<u8>,
    pub plans: Vec<Plan>,
}
impl Default for Progress {
    fn default() -> Self {
        Self {
            version: 3,
            stage: 0,
            stars: vec![0; COUNT],
            plans: vec![Plan::default(); COUNT],
        }
    }
}
impl Progress {
    pub fn valid(&self) -> bool {
        self.version == 3
            && self.stage < COUNT
            && self.stars.len() == COUNT
            && self.plans.len() == COUNT
            && self.stars.iter().all(|n| *n <= 3)
            && self.plans.iter().enumerate().all(|(i, p)| {
                let map = Map::new(i);
                p.bridges
                    .keys()
                    .all(|id| map.cells.get(*id).is_some_and(|c| c.gap))
                    && p.bridges.values().filter(|b| **b == Bridge::Wood).count() <= map.spec.wood
                    && p.bridges.values().filter(|b| **b == Bridge::Stone).count() <= map.spec.stone
            })
    }
}
pub struct CloudWalk {
    pub stage: usize,
    pub map: Map,
    pub plan: Plan,
    pub progress: Progress,
    pub robot: usize,
    pub walking: bool,
    pub material: Bridge,
    pub erase: bool,
    pub gem_taken: bool,
    pub won: bool,
    pub face_left: bool,
    pub message: String,
    undo: Vec<Plan>,
}
impl Default for CloudWalk {
    fn default() -> Self {
        Self::restore(Progress::default())
    }
}
impl CloudWalk {
    pub fn restore(progress: Progress) -> Self {
        let stage = progress.stage;
        let map = Map::new(stage);
        let robot = map.start;
        let plan = progress.plans[stage].clone();
        Self {
            stage,
            map,
            plan,
            progress,
            robot,
            walking: false,
            material: Bridge::Wood,
            erase: false,
            gem_taken: false,
            won: false,
            face_left: false,
            message: "选建材，点蓝色缺口搭桥；搭好后亲自过桥。".into(),
            undo: vec![],
        }
    }
    pub fn snapshot(&self) -> Progress {
        let mut p = self.progress.clone();
        p.stage = self.stage;
        p.plans[self.stage] = self.plan.clone();
        p
    }
    pub fn choose(&mut self, stage: usize) {
        if stage < COUNT {
            let mut p = self.snapshot();
            p.stage = stage;
            *self = Self::restore(p);
        }
    }
    pub fn remaining(&self, b: Bridge) -> usize {
        let total = if b == Bridge::Wood {
            self.map.spec.wood
        } else {
            self.map.spec.stone
        };
        total.saturating_sub(self.plan.bridges.values().filter(|v| **v == b).count())
    }
    pub fn to_build(&mut self) {
        self.walking = false;
        self.robot = self.map.start;
        self.gem_taken = false;
        self.won = false;
        self.face_left = false;
        self.erase = false;
        self.message = "已回到起点，桥梁保留，可以继续调整。".into();
    }
    pub fn enter(&mut self) {
        self.walking = true;
        self.robot = self.map.start;
        self.gem_taken = false;
        self.won = false;
        self.erase = false;
        self.face_left = false;
        self.message = "点四个方向键移动，也可以用WASD或键盘方向键。".into();
    }
    pub fn place(&mut self, id: usize) -> bool {
        if self.walking {
            self.message = "先返回搭桥，再调整桥块；桥梁会保留。".into();
            return false;
        }
        if !self.map.cells.get(id).is_some_and(|c| c.gap) {
            self.message = "在蓝色缺口搭桥，固定小岛不能拆。".into();
            return false;
        }
        if self.erase {
            if !self.plan.bridges.contains_key(&id) {
                self.message = "这个缺口还没有桥块。".into();
                return false;
            }
            self.undo.push(self.plan.clone());
            self.plan.bridges.remove(&id);
            self.message = "已拆除，材料已退回。".into();
            return true;
        }
        if self.plan.bridges.get(&id) == Some(&self.material) {
            self.message = "这里已经放好，切换拆除可退回材料。".into();
            return false;
        }
        if self.remaining(self.material) == 0 {
            self.message = "建材用完了，撤销或拆回一块再试。".into();
            return false;
        }
        self.undo.push(self.plan.clone());
        self.plan.bridges.insert(id, self.material);
        self.message = if self.map.cells[id].wind && self.material == Bridge::Wood {
            "这个风口要用石桥，木桥无法通过。"
        } else {
            "已接好一格；两种材料的桥面都能相连。"
        }
        .into();
        true
    }
    pub fn undo(&mut self) {
        if !self.walking {
            if let Some(plan) = self.undo.pop() {
                self.plan = plan;
                self.message = "已撤销，材料已退回。".into();
            } else {
                self.message = "还没有需要撤销的搭建。".into();
            }
        }
    }
    pub fn restart(&mut self) {
        self.to_build();
        self.plan = Plan::default();
        self.undo.clear();
        self.message = "本关重新开始，建材已全部退回。".into();
    }
    pub fn passable(&self, id: usize) -> bool {
        let c = self.map.cells[id];
        !c.gap
            || self
                .plan
                .bridges
                .get(&id)
                .is_some_and(|b| !c.wind || *b == Bridge::Stone)
    }
    pub fn move_dir(&mut self, dx: i32, dz: i32) -> bool {
        if !self.walking || self.won {
            return false;
        }
        if dx.abs() + dz.abs() != 1 {
            return false;
        }
        self.face_left = dx - dz < 0;
        let [x, z] = self.map.cells[self.robot].at;
        let Some(next) = self.map.id([x + dx, z + dz]) else {
            self.message = "这里是岛边，没有路；换个方向。".into();
            return false;
        };
        if !self.passable(next) {
            self.message = if self.map.cells[next].wind && self.plan.bridges.contains_key(&next) {
                "木桥挡不住风口，返回搭桥后换成石桥。"
            } else {
                "前面还缺一块桥，返回搭桥后再补。"
            }
            .into();
            return false;
        }
        if next == self.map.goal && self.map.spec.required && !self.gem_taken {
            self.message = "先去星晶岛，再回中枢抵达终点。".into();
            return false;
        }
        self.robot = next;
        self.message = "岔路由你决定，点方向继续走。".into();
        if Some(next) == self.map.gem && !self.gem_taken {
            self.gem_taken = true;
            self.message = "拿到星晶了！继续走到金色终点。".into();
        }
        if next == self.map.goal {
            self.won = true;
            let stars = 2 + u8::from(self.gem_taken || self.map.gem.is_none());
            self.progress.stars[self.stage] = self.progress.stars[self.stage].max(stars);
            self.message = format!("到达终点！获得{stars}颗星；可点上方关卡继续。 ");
        }
        true
    }
    /// One admissible build plan, used by verification, never by the play UI.
    #[cfg(any(test, debug_assertions))]
    pub fn verification_plan(&self) -> Option<Plan> {
        let count = self.map.spans.len();
        for mask in 1usize..(1 << count) {
            let mut floors: BTreeSet<usize> = self
                .map
                .cells
                .iter()
                .enumerate()
                .filter(|(_, c)| !c.gap)
                .map(|(i, _)| i)
                .collect();
            let mut need: Vec<usize> = vec![];
            for (i, span) in self.map.spans.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    floors.extend(span);
                    need.extend(span.iter().copied());
                }
            }
            let mut seen = BTreeSet::from([self.map.start]);
            let mut todo = vec![self.map.start];
            while let Some(id) = todo.pop() {
                let [x, z] = self.map.cells[id].at;
                for [dx, dz] in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
                    if let Some(n) = self.map.id([x + dx, z + dz]) {
                        if floors.contains(&n) && seen.insert(n) {
                            todo.push(n);
                        }
                    }
                }
            }
            if !seen.contains(&self.map.goal) || self.map.gem.is_some_and(|g| !seen.contains(&g)) {
                continue;
            }
            let winds = need.iter().filter(|&&n| self.map.cells[n].wind).count();
            let total = need.len();
            if winds > self.map.spec.stone || total > self.map.spec.wood + self.map.spec.stone {
                continue;
            }
            let mut stone = self.map.spec.stone;
            let mut wood = self.map.spec.wood;
            let mut plan = Plan::default();
            for &n in &need {
                if self.map.cells[n].wind {
                    plan.bridges.insert(n, Bridge::Stone);
                    stone -= 1;
                }
            }
            for &n in &need {
                if plan.bridges.contains_key(&n) {
                    continue;
                }
                let b = if wood > 0 {
                    wood -= 1;
                    Bridge::Wood
                } else if stone > 0 {
                    stone -= 1;
                    Bridge::Stone
                } else {
                    unreachable!()
                };
                plan.bridges.insert(n, b);
            }
            return Some(plan);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    fn route(g: &CloudWalk, to: usize) -> Vec<usize> {
        let mut q = VecDeque::from([g.robot]);
        let mut prev = BTreeMap::from([(g.robot, g.robot)]);
        while let Some(at) = q.pop_front() {
            if at == to {
                let mut path = vec![];
                let mut n = at;
                while n != g.robot {
                    path.push(n);
                    n = prev[&n];
                }
                path.reverse();
                return path;
            }
            let [x, z] = g.map.cells[at].at;
            for [dx, dz] in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
                if let Some(n) = g.map.id([x + dx, z + dz]) {
                    if g.passable(n) && !prev.contains_key(&n) {
                        prev.insert(n, at);
                        q.push_back(n);
                    }
                }
            }
        }
        panic!("no route");
    }
    fn walk(g: &mut CloudWalk, to: usize) {
        for n in route(g, to) {
            let a = g.map.cells[g.robot].at;
            let b = g.map.cells[n].at;
            assert!(g.move_dir(b[0] - a[0], b[1] - a[1]), "{}", g.message);
        }
    }
    fn build(g: &mut CloudWalk) {
        for (n, b) in g.verification_plan().unwrap().bridges {
            g.material = b;
            assert!(g.place(n));
        }
    }
    #[test]
    fn six_maps_have_distinct_layouts_and_exact_three_cell_spans() {
        let mut signatures = BTreeSet::new();
        for i in 0..COUNT {
            let m = Map::new(i);
            assert!(signatures.insert(format!("{:?}:{:?}", m.islands, m.spans)));
            assert_eq!(m.cells.iter().filter(|c| c.gap).count(), m.spans.len() * 3);
            for span in &m.spans {
                for pair in span.windows(2) {
                    let a = m.cells[pair[0]].at;
                    let b = m.cells[pair[1]].at;
                    assert_eq!((a[0] - b[0]).abs() + (a[1] - b[1]).abs(), 1);
                }
            }
        }
    }
    #[test]
    fn all_six_can_be_built_and_walked_without_autoplay() {
        for i in 0..COUNT {
            let mut g = CloudWalk::default();
            g.choose(i);
            build(&mut g);
            assert!(g.snapshot().valid());
            g.enter();
            assert_eq!(g.robot, g.map.start);
            if let Some(gem) = g.map.gem {
                walk(&mut g, gem);
            }
            let goal = g.map.goal;
            walk(&mut g, goal);
            assert!(g.won && g.progress.stars[i] == 3);
        }
    }
    #[test]
    fn missing_bridge_and_wrong_material_do_not_move_robot() {
        let mut g = CloudWalk::default();
        g.enter();
        assert!(g.move_dir(1, 0));
        let before = g.robot;
        assert!(!g.move_dir(1, 0));
        assert_eq!(g.robot, before);
        g.to_build();
        assert_eq!(g.robot, g.map.start);
        let mut g = CloudWalk::default();
        g.choose(3);
        let n = g.map.cells.iter().position(|c| c.wind).unwrap();
        g.material = Bridge::Wood;
        assert!(g.place(n));
        assert!(!g.passable(n));
        g.material = Bridge::Stone;
        assert!(g.place(n));
        assert!(g.passable(n));
    }
    #[test]
    fn build_refunds_and_phase_change_preserve_plan() {
        let mut g = CloudWalk::default();
        let n = g.map.spans[0][0];
        assert!(g.place(n));
        assert_eq!(g.remaining(Bridge::Wood), 2);
        g.undo();
        assert_eq!(g.remaining(Bridge::Wood), 3);
        assert!(g.place(n));
        g.enter();
        assert!(!g.place(n));
        g.to_build();
        assert_eq!(g.plan.bridges.len(), 1);
        g.erase = true;
        assert!(g.place(n));
        assert_eq!(g.remaining(Bridge::Wood), 3);
    }
    #[test]
    fn last_map_requires_crystal_before_goal() {
        let mut g = CloudWalk::default();
        g.choose(5);
        build(&mut g);
        g.enter();
        let goal = g.map.goal;
        let path = route(&g, goal);
        for n in path {
            let a = g.map.cells[g.robot].at;
            let b = g.map.cells[n].at;
            let ok = g.move_dir(b[0] - a[0], b[1] - a[1]);
            if n == goal {
                assert!(!ok && !g.won);
            } else {
                assert!(ok);
            }
        }
        let gem = g.map.gem.unwrap();
        walk(&mut g, gem);
        walk(&mut g, goal);
        assert!(g.won);
    }
    #[test]
    fn optional_crystal_routes_can_be_skipped_and_final_map_has_two_exits() {
        for (stage, edges, expected) in [
            (1, vec![2, 3], 2),
            (4, vec![3, 4, 5], 2),
            (5, vec![0, 1, 3, 5], 3),
        ] {
            let mut g = CloudWalk::default();
            g.choose(stage);
            let ids: Vec<usize> = edges.iter().flat_map(|e| g.map.spans[*e]).collect();
            for id in ids {
                g.material = if g.map.cells[id].wind || g.remaining(Bridge::Wood) == 0 {
                    Bridge::Stone
                } else {
                    Bridge::Wood
                };
                assert!(g.place(id));
            }
            g.enter();
            if g.map.spec.required {
                let gem = g.map.gem.unwrap();
                walk(&mut g, gem);
            }
            let goal = g.map.goal;
            walk(&mut g, goal);
            assert!(g.won);
            assert_eq!(g.progress.stars[stage], expected);
        }
    }
    #[test]
    fn progress_round_trip_is_separate_and_rejects_invalid_plans() {
        let mut g = CloudWalk::default();
        build(&mut g);
        let p: Progress =
            serde_json::from_slice(&serde_json::to_vec(&g.snapshot()).unwrap()).unwrap();
        assert!(p.valid());
        let mut restored = CloudWalk::restore(p);
        assert_eq!(restored.plan, g.plan);
        restored.choose(5);
        restored.choose(0);
        assert_eq!(restored.plan, g.plan);
        let mut p = g.snapshot();
        p.plans[0].bridges.insert(g.map.start, Bridge::Wood);
        assert!(!p.valid());
    }
}
