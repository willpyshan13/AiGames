use super::*;

pub(super) const GARDEN_ROWS: usize = 5;
pub(super) const GARDEN_COLS: usize = 9;
const GARDEN_STEP: f32 = 1. / 120.;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GardenPlantKind {
    Sunflower,
    Pea,
    Nut,
    Frost,
    Chili,
}

impl GardenPlantKind {
    pub(super) const ALL: [Self; 5] = [
        Self::Sunflower,
        Self::Pea,
        Self::Nut,
        Self::Frost,
        Self::Chili,
    ];

    pub(super) fn index(self) -> usize {
        match self {
            Self::Sunflower => 0,
            Self::Pea => 1,
            Self::Nut => 2,
            Self::Frost => 3,
            Self::Chili => 4,
        }
    }

    pub(super) fn cost(self) -> u32 {
        [50, 100, 75, 125, 150][self.index()]
    }

    pub(super) fn cooldown(self) -> f32 {
        [1.5, 1., 3., 2.5, 14.][self.index()]
    }

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Sunflower => "向日葵",
            Self::Pea => "豌豆射手",
            Self::Nut => "坚果卫士",
            Self::Frost => "冰霜花",
            Self::Chili => "火爆辣椒",
        }
    }

    pub(super) fn hint(self) -> &'static str {
        match self {
            Self::Sunflower => "每隔一会儿生出阳光，种在后排最安心",
            Self::Pea => "向右发射豌豆，守住它所在的这条草地",
            Self::Nut => "挡住怪物，种在射手前面保护大家",
            Self::Frost => "冰豌豆让怪物走慢，给射手多一点时间",
            Self::Chili => "一次清空整条草地上的怪物，危急时再用",
        }
    }

    fn health(self) -> f32 {
        match self {
            Self::Nut => 620.,
            _ => 125.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GardenEnemyKind {
    Sprout,
    Runner,
    Bucket,
    Brute,
}

impl GardenEnemyKind {
    pub(super) fn index(self) -> usize {
        match self {
            Self::Sprout => 0,
            Self::Runner => 1,
            Self::Bucket => 2,
            Self::Brute => 3,
        }
    }

    fn health(self) -> f32 {
        [105., 78., 750., 1400.][self.index()]
    }

    fn speed(self) -> f32 {
        [0.205, 0.345, 0.205, 0.17][self.index()]
    }

    fn bite(self) -> f32 {
        [19., 17., 25., 44.][self.index()]
    }

    fn score(self) -> u32 {
        [100, 130, 200, 350][self.index()]
    }

    fn radius(self) -> f32 {
        if self == Self::Brute { 0.34 } else { 0.22 }
    }
}

#[derive(Clone, Debug)]
pub(super) struct GardenPlant {
    pub(super) row: usize,
    pub(super) col: usize,
    pub(super) kind: GardenPlantKind,
    pub(super) hp: f32,
    pub(super) max_hp: f32,
    pub(super) age: f32,
    pub(super) recoil: f32,
    pub(super) flash: f32,
    action_in: f32,
}

#[derive(Clone, Debug)]
pub(super) struct GardenEnemy {
    pub(super) id: u32,
    pub(super) row: usize,
    pub(super) x: f32,
    pub(super) kind: GardenEnemyKind,
    pub(super) hp: f32,
    pub(super) max_hp: f32,
    pub(super) age: f32,
    pub(super) flash: f32,
    pub(super) slow: f32,
    pub(super) biting: bool,
    attack_in: f32,
}

#[derive(Clone, Debug)]
pub(super) struct GardenShot {
    pub(super) row: usize,
    pub(super) x: f32,
    pub(super) frost: bool,
}

#[derive(Clone, Debug)]
pub(super) struct GardenSun {
    pub(super) id: u32,
    pub(super) pos: Vec2,
    pub(super) age: f32,
    pub(super) value: u32,
    target_y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GardenEffectKind {
    Hit,
    Frost,
    Chili,
    Plant,
    Collect,
    Death,
}

impl GardenEffectKind {
    pub(super) fn duration(self) -> f32 {
        match self {
            Self::Hit => 0.42,
            Self::Frost => 0.65,
            Self::Chili => 1.1,
            Self::Plant => 0.6,
            Self::Collect => 0.65,
            Self::Death => 0.9,
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct GardenEffect {
    pub(super) pos: Vec2,
    pub(super) age: f32,
    pub(super) kind: GardenEffectKind,
}

#[derive(Clone, Copy)]
struct GardenSpawn {
    at: f32,
    wave: u8,
    row: usize,
    kind: GardenEnemyKind,
}

pub(super) struct GardenGame {
    pub(super) stage: u8,
    pub(super) score: u32,
    pub(super) sunlight: u32,
    pub(super) selected: GardenPlantKind,
    pub(super) shovel: bool,
    pub(super) hover: Option<(usize, usize)>,
    pub(super) plants: Vec<GardenPlant>,
    pub(super) enemies: Vec<GardenEnemy>,
    pub(super) shots: Vec<GardenShot>,
    pub(super) suns: Vec<GardenSun>,
    pub(super) effects: Vec<GardenEffect>,
    pub(super) elapsed: f32,
    pub(super) cooldowns: [f32; 5],
    pub(super) wave: u8,
    pub(super) total_waves: u8,
    pub(super) progress: f32,
    pub(super) ready: bool,
    pub(super) paused: bool,
    pub(super) over: bool,
    pub(super) won: bool,
    pub(super) guards: [bool; GARDEN_ROWS],
    pub(super) guard_x: [Option<f32>; GARDEN_ROWS],
    pub(super) hint: String,
    stage_start_score: u32,
    accumulator: f32,
    redraw_clock: f32,
    sun_in: f32,
    rng: u32,
    next_id: u32,
    spawn_plan: Vec<GardenSpawn>,
    spawn_cursor: usize,
}

impl GardenGame {
    pub(super) fn new() -> Self {
        Self::for_stage(1, 0)
    }

    fn for_stage(stage: u8, score: u32) -> Self {
        let total_waves = stage + 3;
        let mut spawn_plan = Vec::new();
        let rows = [2, 1, 3, 0, 4, 2, 4, 0, 3, 1];
        for wave in 0..total_waves {
            let count = if stage <= 3 {
                2 + usize::from(stage) + usize::from(wave / 2)
            } else {
                (4 + usize::from(stage - 4) + usize::from(wave / 3)).min(7)
            };
            for i in 0..count {
                let kind = if stage >= 8 && wave >= 2 && i % 4 == 0 {
                    GardenEnemyKind::Brute
                } else if stage >= 6 && wave >= 2 && i % 6 == 0 {
                    GardenEnemyKind::Brute
                } else if stage == 3 && wave >= 2 && i % 5 == 0 {
                    GardenEnemyKind::Brute
                } else if stage >= 5 && wave >= 1 && i % 3 == 0 {
                    GardenEnemyKind::Bucket
                } else if stage >= 2 && wave >= 1 && i % 2 == 0 {
                    GardenEnemyKind::Bucket
                } else if (stage >= 2 || wave >= 2) && i % 3 == 1 {
                    GardenEnemyKind::Runner
                } else {
                    GardenEnemyKind::Sprout
                };
                let wave_spacing = if stage == 1 {
                    15.
                } else if stage <= 3 {
                    12.
                } else if stage >= 8 {
                    8.5
                } else if stage >= 7 {
                    9.5
                } else {
                    10.5
                };
                let enemy_spacing = if stage == 1 {
                    2.7
                } else if stage <= 3 {
                    1.8
                } else {
                    1.35
                };
                spawn_plan.push(GardenSpawn {
                    at: 7. + f32::from(wave) * wave_spacing + i as f32 * enemy_spacing,
                    wave: wave + 1,
                    row: rows[(usize::from(wave) * 3 + i + usize::from(stage.saturating_sub(3)))
                        % rows.len()],
                    kind,
                });
            }
        }
        spawn_plan.sort_by(|a, b| a.at.total_cmp(&b.at));
        Self {
            stage,
            score,
            sunlight: if stage <= 3 {
                300 + u32::from(stage - 1) * 50
            } else {
                400 + u32::from(stage - 3) * 35
            },
            selected: GardenPlantKind::Pea,
            shovel: false,
            hover: None,
            plants: Vec::new(),
            enemies: Vec::new(),
            shots: Vec::new(),
            suns: Vec::new(),
            effects: Vec::new(),
            elapsed: 0.,
            cooldowns: [0.; 5],
            wave: 0,
            total_waves,
            progress: 0.,
            ready: true,
            paused: false,
            over: false,
            won: false,
            guards: [true; GARDEN_ROWS],
            guard_x: [None; GARDEN_ROWS],
            hint: "先种向日葵和射手，再点开始；第一只怪物会走中间那条草地".into(),
            stage_start_score: score,
            accumulator: 0.,
            redraw_clock: 0.,
            sun_in: 3.,
            rng: 0x729c_0b61 ^ u32::from(stage),
            next_id: 1,
            spawn_plan,
            spawn_cursor: 0,
        }
    }

    pub(super) fn reset(&mut self) {
        *self = Self::for_stage(self.stage, self.stage_start_score);
    }

    pub(super) fn next_stage(&mut self) {
        if !self.won {
            return;
        }
        *self = if self.stage < 8 {
            Self::for_stage(self.stage + 1, self.score)
        } else {
            Self::for_stage(1, 0)
        };
    }

    pub(super) fn select(&mut self, index: usize) {
        if let Some(kind) = GardenPlantKind::ALL.get(index).copied() {
            self.selected = kind;
            self.shovel = false;
            self.hint = format!("{}：{}", kind.name(), kind.hint());
        }
    }

    pub(super) fn toggle_shovel(&mut self) {
        self.shovel = !self.shovel;
        self.hint = if self.shovel {
            "点一下植物把它移走，返还一半阳光；再点铲子取消".into()
        } else {
            self.selected.hint().into()
        };
    }

    pub(super) fn toggle_pause(&mut self) {
        if !self.ready && !self.over {
            self.paused = !self.paused;
        }
    }

    pub(super) fn start(&mut self) {
        if self.ready && !self.over {
            self.ready = false;
            self.wave = 1;
            self.hint = "点金色阳光收集能量；怪物从右边来，射手要种在同一条草地".into();
        }
    }

    pub(super) fn plant(&mut self, row: usize, col: usize) -> bool {
        if row >= GARDEN_ROWS || col >= GARDEN_COLS || self.over || self.paused {
            return false;
        }
        let occupied = self
            .plants
            .iter()
            .position(|p| p.row == row && p.col == col);
        if self.shovel {
            let Some(index) = occupied else {
                self.hint = "这格没有植物，点铲子可以取消".into();
                return false;
            };
            let plant = self.plants.remove(index);
            self.sunlight += plant.kind.cost() / 2;
            self.effect(
                Vec2::new(col as f32 + 0.5, row as f32 + 0.5),
                GardenEffectKind::Death,
            );
            self.hint = "腾出一格啦，返还了一半阳光".into();
            return true;
        }
        if occupied.is_some() {
            self.hint = "这一格已经有植物啦，可以用铲子移走".into();
            return false;
        }
        let kind = self.selected;
        if self.cooldowns[kind.index()] > 0.001 {
            self.hint = format!("{}正在休息，等卡片亮起来再种", kind.name());
            return false;
        }
        if self.sunlight < kind.cost() {
            self.hint = "阳光不够啦，点金色阳光收集，或多种一朵向日葵".into();
            return false;
        }
        if kind == GardenPlantKind::Chili
            && !self
                .enemies
                .iter()
                .any(|enemy| enemy.row == row && enemy.hp > 0.)
        {
            self.hint = "这一条草地很安全，等怪物来了再用辣椒，不浪费阳光".into();
            return false;
        }
        self.sunlight -= kind.cost();
        // Preparation is deliberately unhurried: planting cooldowns start only
        // once the child chooses to begin the wave.
        self.cooldowns[kind.index()] = if self.ready { 0. } else { kind.cooldown() };
        let pos = Vec2::new(col as f32 + 0.5, row as f32 + 0.5);
        if kind == GardenPlantKind::Chili {
            for enemy in &mut self.enemies {
                if enemy.row == row {
                    enemy.hp = 0.;
                }
            }
            // The renderer draws this as a single lane-wide sweep. Emitting
            // one effect per cell would stack nine translucent flame layers.
            self.effect(
                Vec2::new(GARDEN_COLS as f32 * 0.5, row as f32 + 0.5),
                GardenEffectKind::Chili,
            );
            self.remove_defeated();
            self.hint = "呼！这一整条草地都安全啦".into();
        } else {
            self.plants.push(GardenPlant {
                row,
                col,
                kind,
                hp: kind.health(),
                max_hp: kind.health(),
                age: 0.,
                recoil: 0.,
                flash: 0.,
                action_in: if kind == GardenPlantKind::Sunflower {
                    4.
                } else {
                    0.15
                },
            });
            self.effect(pos, GardenEffectKind::Plant);
            self.hint = format!("{}准备好啦！{}", kind.name(), kind.hint());
        }
        true
    }

    pub(super) fn collect(&mut self, id: u32) -> bool {
        if self.paused || self.over {
            return false;
        }
        let Some(index) = self.suns.iter().position(|sun| sun.id == id) else {
            return false;
        };
        let sun = self.suns.remove(index);
        self.sunlight += sun.value;
        self.effect(sun.pos, GardenEffectKind::Collect);
        true
    }

    pub(super) fn collect_all(&mut self) -> usize {
        if self.paused || self.over || self.suns.is_empty() {
            return 0;
        }
        let ids: Vec<u32> = self.suns.iter().map(|sun| sun.id).collect();
        let mut collected = 0;
        for id in ids {
            if self.collect(id) {
                collected += 1;
            }
        }
        self.hint = if collected == 1 {
            "收集到 1 份阳光！".into()
        } else {
            format!("一次收集了 {collected} 份阳光！")
        };
        collected
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        if self.paused || !dt.is_finite() || dt <= 0. {
            return false;
        }
        // Bounded catch-up avoids a resumed window suddenly consuming a whole
        // wave, while small fixed steps keep bites and fast shots consistent.
        let dt = dt.min(1.);
        self.accumulator += dt;
        self.redraw_clock += dt;
        while self.accumulator + 0.000001 >= GARDEN_STEP {
            self.accumulator -= GARDEN_STEP;
            self.step(GARDEN_STEP);
        }
        if self.redraw_clock >= 1. / 30. {
            self.redraw_clock %= 1. / 30.;
            true
        } else {
            false
        }
    }

    fn step(&mut self, dt: f32) {
        for plant in &mut self.plants {
            plant.age += dt;
            plant.recoil = (plant.recoil - dt * 5.).max(0.);
            plant.flash = (plant.flash - dt).max(0.);
        }
        for enemy in &mut self.enemies {
            enemy.age += dt;
            enemy.flash = (enemy.flash - dt).max(0.);
        }
        for sun in &mut self.suns {
            sun.age += dt;
            sun.pos.y = (sun.pos.y + dt * 0.65).min(sun.target_y);
        }
        self.suns.retain(|sun| sun.age < 20.);
        for effect in &mut self.effects {
            effect.age += dt;
        }
        self.effects
            .retain(|effect| effect.age < effect.kind.duration());
        for cooldown in &mut self.cooldowns {
            *cooldown = (*cooldown - dt).max(0.);
        }
        if self.ready || self.over {
            return;
        }
        self.elapsed += dt;
        while self.spawn_cursor < self.spawn_plan.len()
            && self.elapsed >= self.spawn_plan[self.spawn_cursor].at
        {
            let spawn = self.spawn_plan[self.spawn_cursor];
            self.wave = self.wave.max(spawn.wave);
            self.spawn_enemy(spawn.row, 9.7, spawn.kind);
            self.spawn_cursor += 1;
        }
        self.progress =
            (self.elapsed / self.spawn_plan.last().map_or(1., |s| s.at) * 0.94).min(0.94);
        self.sun_in -= dt;
        if self.sun_in <= 0. {
            self.sun_in += 4.5;
            let x = 0.75 + self.random() * 7.5;
            let y = 0.65 + self.random() * 3.8;
            self.make_sun(Vec2::new(x, 0.1), y);
        }
        self.plant_actions(dt);
        self.enemy_actions(dt);
        self.shot_actions(dt);
        self.guard_actions(dt);
        self.remove_defeated();
        self.plants.retain(|plant| plant.hp > 0.);
        if self.over {
            return;
        }
        if self.spawn_cursor == self.spawn_plan.len()
            && self.enemies.is_empty()
            && self.guard_x.iter().all(Option::is_none)
        {
            self.won = true;
            self.over = true;
            self.progress = 1.;
            self.score += 500 * u32::from(self.stage);
            self.hint = if self.stage < 8 {
                "守住花园啦！下一关会有新的小怪物来挑战".into()
            } else {
                "八座花园全部守住啦，你是花园小英雄！".into()
            };
        }
    }

    fn plant_actions(&mut self, dt: f32) {
        let mut sunshine = Vec::new();
        for plant in &mut self.plants {
            plant.action_in -= dt;
            match plant.kind {
                GardenPlantKind::Sunflower if plant.action_in <= 0. => {
                    plant.action_in += 9.;
                    plant.recoil = 0.6;
                    sunshine.push(Vec2::new(plant.col as f32 + 0.75, plant.row as f32 + 0.25));
                }
                GardenPlantKind::Pea | GardenPlantKind::Frost => {
                    let ahead = self.enemies.iter().any(|enemy| {
                        enemy.row == plant.row && enemy.hp > 0. && enemy.x > plant.col as f32 + 0.3
                    });
                    if ahead && plant.action_in <= 0. {
                        let frost = plant.kind == GardenPlantKind::Frost;
                        plant.action_in = if frost { 1.35 } else { 1.05 };
                        plant.recoil = 1.;
                        self.shots.push(GardenShot {
                            row: plant.row,
                            x: plant.col as f32 + 0.85,
                            frost,
                        });
                    } else if !ahead {
                        plant.action_in = plant.action_in.max(0.08);
                    }
                }
                _ => {}
            }
        }
        for pos in sunshine {
            self.make_sun(pos, pos.y + 0.35);
        }
    }

    fn enemy_actions(&mut self, dt: f32) {
        let mut bite_effects = Vec::new();
        for enemy in &mut self.enemies {
            if enemy.hp <= 0. {
                continue;
            }
            let speed = enemy.kind.speed() * if enemy.slow > 0. { 0.42 } else { 1. };
            enemy.slow = (enemy.slow - dt).max(0.);
            // The nearest living plant to the left is the only bite target.
            // Clamping movement to its contact point also prevents tunneling.
            let target = self
                .plants
                .iter()
                .enumerate()
                .filter(|(_, plant)| {
                    plant.row == enemy.row
                        && plant.hp > 0.
                        && plant.col as f32 + 0.5 <= enemy.x + 0.15
                })
                .max_by_key(|(_, plant)| plant.col)
                .map(|(index, _)| index);
            if let Some(index) = target {
                // Keep the sprite bodies apart while the attack mouth reaches
                // the plant; centers touching would look like penetration.
                let stop = self.plants[index].col as f32 + 1.25;
                enemy.x = (enemy.x - speed * dt).max(stop.min(enemy.x));
                enemy.biting = enemy.x <= stop + 0.015;
                if enemy.biting {
                    enemy.attack_in -= dt;
                    if enemy.attack_in <= 0. {
                        enemy.attack_in += 0.65;
                        let plant = &mut self.plants[index];
                        plant.hp -= enemy.kind.bite();
                        plant.flash = 0.2;
                        if plant.hp <= 0. {
                            bite_effects.push((
                                Vec2::new(plant.col as f32 + 0.5, plant.row as f32 + 0.5),
                                GardenEffectKind::Death,
                            ));
                        } else {
                            bite_effects.push((
                                Vec2::new(plant.col as f32 + 0.5, plant.row as f32 + 0.5),
                                GardenEffectKind::Hit,
                            ));
                        }
                    }
                } else {
                    enemy.attack_in = 0.3;
                }
            } else {
                enemy.x -= speed * dt;
                enemy.biting = false;
                enemy.attack_in = 0.3;
            }
        }
        for (pos, kind) in bite_effects {
            self.effect(pos, kind);
        }
    }

    fn shot_actions(&mut self, dt: f32) {
        let mut impacts = Vec::new();
        self.shots.retain_mut(|shot| {
            let old_x = shot.x;
            shot.x += 5.8 * dt;
            let target = self
                .enemies
                .iter()
                .enumerate()
                .filter(|(_, enemy)| {
                    enemy.row == shot.row
                        && enemy.hp > 0.
                        && enemy.x + enemy.kind.radius() >= old_x
                        && enemy.x - enemy.kind.radius() <= shot.x
                })
                .min_by(|(_, a), (_, b)| a.x.total_cmp(&b.x))
                .map(|(index, _)| index);
            if let Some(index) = target {
                let enemy = &mut self.enemies[index];
                enemy.hp -= if shot.frost { 23. } else { 36. };
                enemy.flash = 0.18;
                if shot.frost {
                    enemy.slow = 3.5;
                }
                impacts.push((
                    Vec2::new(enemy.x, enemy.row as f32 + 0.5),
                    if shot.frost {
                        GardenEffectKind::Frost
                    } else {
                        GardenEffectKind::Hit
                    },
                ));
                false
            } else {
                shot.x < 10.4
            }
        });
        for (pos, kind) in impacts {
            self.effect(pos, kind);
        }
    }

    fn guard_actions(&mut self, dt: f32) {
        for row in 0..GARDEN_ROWS {
            if self
                .enemies
                .iter()
                .any(|enemy| enemy.row == row && enemy.hp > 0. && enemy.x < 0.05)
            {
                if self.guards[row] {
                    self.guards[row] = false;
                    self.guard_x[row] = Some(-0.2);
                    self.hint = "小推车帮你守住了这一条草地，快补上射手！".into();
                } else if self.guard_x[row].is_none() {
                    self.over = true;
                    self.won = false;
                    self.hint = "有小怪物溜进花园了，再来一次，把射手分到每一条草地".into();
                }
            }
            if let Some(x) = &mut self.guard_x[row] {
                *x += dt * 7.;
                for enemy in &mut self.enemies {
                    if enemy.row == row && enemy.hp > 0. && enemy.x <= *x + 0.38 {
                        enemy.hp = 0.;
                    }
                }
                if *x > 10.4 {
                    self.guard_x[row] = None;
                }
            }
        }
    }

    fn spawn_enemy(&mut self, row: usize, x: f32, kind: GardenEnemyKind) {
        let id = self.take_id();
        self.enemies.push(GardenEnemy {
            id,
            row,
            x,
            kind,
            hp: kind.health(),
            max_hp: kind.health(),
            age: 0.,
            flash: 0.,
            slow: 0.,
            biting: false,
            attack_in: 0.3,
        });
    }

    fn make_sun(&mut self, pos: Vec2, target_y: f32) {
        let id = self.take_id();
        self.suns.push(GardenSun {
            id,
            pos,
            target_y,
            age: 0.,
            value: 25,
        });
    }

    fn effect(&mut self, pos: Vec2, kind: GardenEffectKind) {
        self.effects.push(GardenEffect { pos, age: 0., kind });
    }

    fn remove_defeated(&mut self) {
        let mut deaths = Vec::new();
        self.enemies.retain(|enemy| {
            if enemy.hp > 0. {
                true
            } else {
                self.score += enemy.kind.score();
                deaths.push(Vec2::new(enemy.x, enemy.row as f32 + 0.5));
                false
            }
        });
        for pos in deaths {
            self.effect(pos, GardenEffectKind::Death);
        }
    }

    fn random(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        (self.rng & 0x00ff_ffff) as f32 / 0x0100_0000u32 as f32
    }

    fn take_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // Used only by the screenshot harness: normal play always starts with an
    // empty lawn and a preparation phase.
    pub(super) fn demo(&mut self) {
        *self = Self::for_stage(1, 0);
        self.sunlight = 2000;
        let layout = [
            (0, 0, GardenPlantKind::Sunflower),
            (0, 2, GardenPlantKind::Pea),
            (1, 0, GardenPlantKind::Sunflower),
            (1, 2, GardenPlantKind::Frost),
            (1, 4, GardenPlantKind::Nut),
            (2, 1, GardenPlantKind::Pea),
            (2, 3, GardenPlantKind::Pea),
            (3, 0, GardenPlantKind::Sunflower),
            (3, 2, GardenPlantKind::Pea),
            (3, 5, GardenPlantKind::Nut),
            (4, 1, GardenPlantKind::Frost),
            (4, 3, GardenPlantKind::Pea),
        ];
        for (row, col, kind) in layout {
            self.selected = kind;
            self.plant(row, col);
            if let Some(plant) = self.plants.last_mut() {
                plant.age = 5. + (row + col) as f32 * 0.3;
                plant.action_in = 0.2 + row as f32 * 0.1;
            }
        }
        self.effects.clear();
        self.sunlight = 175;
        self.start();
        self.elapsed = 28.;
        self.wave = 2;
        self.spawn_cursor = self
            .spawn_plan
            .iter()
            .take_while(|spawn| spawn.at <= self.elapsed)
            .count();
        self.spawn_enemy(0, 6.8, GardenEnemyKind::Sprout);
        self.spawn_enemy(0, 8.2, GardenEnemyKind::Bucket);
        self.spawn_enemy(1, 6.5, GardenEnemyKind::Bucket);
        self.spawn_enemy(2, 6.9, GardenEnemyKind::Runner);
        self.spawn_enemy(3, 6.05, GardenEnemyKind::Sprout);
        self.spawn_enemy(3, 8.5, GardenEnemyKind::Brute);
        self.spawn_enemy(4, 7.1, GardenEnemyKind::Runner);
        self.enemies[2].slow = 3.;
        self.make_sun(Vec2::new(1.25, 2.35), 2.35);
        self.make_sun(Vec2::new(4.8, 4.3), 4.3);
        self.selected = GardenPlantKind::Pea;
        self.hover = Some((2, 4));
        self.hint = "点击植物，再点草地种下；点金色阳光收集能量".into();
    }
}

#[cfg(test)]
mod garden_tests {
    use super::*;

    fn running() -> GardenGame {
        let mut game = GardenGame::new();
        game.start();
        game.spawn_plan.clear();
        // Keep one scheduled enemy far in the future so isolated tests don't
        // accidentally finish the level when their test enemy is defeated.
        game.spawn_plan.push(GardenSpawn {
            at: 999.,
            row: 0,
            wave: 1,
            kind: GardenEnemyKind::Sprout,
        });
        game
    }

    #[test]
    fn planting_validates_price_occupancy_bounds_and_cooldown() {
        let mut game = running();
        assert!(game.plant(2, 1));
        assert_eq!(game.sunlight, 200);
        assert!(!game.plant(2, 1));
        assert!(!game.plant(1, 1));
        assert!(!game.plant(5, 0));
        game.advance(1.);
        assert!(game.plant(1, 1));
        game.sunlight = 99;
        game.advance(1.);
        assert!(!game.plant(0, 1));
        assert_eq!(game.sunlight, 99);
    }

    #[test]
    fn preparation_allows_unhurried_repeated_planting() {
        let mut game = GardenGame::new();
        assert!(game.plant(0, 0));
        assert!(game.plant(1, 0));
        game.advance(1.);
        assert_eq!(game.elapsed, 0.);
        assert!(game.enemies.is_empty());
    }

    #[test]
    fn shot_hits_nearest_enemy_in_its_lane_without_tunneling() {
        let mut game = running();
        game.spawn_enemy(0, 2.0, GardenEnemyKind::Sprout);
        game.spawn_enemy(1, 2.5, GardenEnemyKind::Sprout);
        game.spawn_enemy(1, 3.0, GardenEnemyKind::Sprout);
        game.shots.push(GardenShot {
            row: 1,
            x: 1.0,
            frost: false,
        });
        game.advance(0.5);
        assert_eq!(game.enemies[0].hp, 105.);
        assert_eq!(game.enemies[1].hp, 69.);
        assert_eq!(game.enemies[2].hp, 105.);
        assert!(game.shots.is_empty());
    }

    #[test]
    fn frost_projectile_slows_only_the_hit_enemy() {
        let mut game = running();
        game.spawn_enemy(1, 2., GardenEnemyKind::Runner);
        game.spawn_enemy(2, 2., GardenEnemyKind::Runner);
        game.shots.push(GardenShot {
            row: 1,
            x: 1.7,
            frost: true,
        });
        game.advance(0.1);
        assert!(game.enemies[0].slow > 3.);
        assert_eq!(game.enemies[1].slow, 0.);
        let slow_x = game.enemies[0].x;
        let normal_x = game.enemies[1].x;
        game.advance(1.);
        assert!(slow_x - game.enemies[0].x < (normal_x - game.enemies[1].x) * 0.5);
    }

    #[test]
    fn enemy_stops_at_nut_and_bites_it_instead_of_passing_through() {
        let mut game = GardenGame::new();
        game.select(GardenPlantKind::Nut.index());
        assert!(game.plant(2, 4));
        game.start();
        game.spawn_plan[0].at = 999.;
        game.spawn_enemy(2, 5.3, GardenEnemyKind::Sprout);
        for _ in 0..5 {
            game.advance(1.);
        }
        assert!(game.enemies[0].biting);
        assert!((game.enemies[0].x - 5.25).abs() < 0.02);
        assert!(game.plants[0].hp < game.plants[0].max_hp);
        assert!(game.plants[0].hp > 400.);
    }

    #[test]
    fn chili_clears_only_its_lane_and_has_a_cooldown() {
        let mut game = running();
        game.sunlight = 400;
        game.select(GardenPlantKind::Chili.index());
        game.spawn_enemy(1, 7., GardenEnemyKind::Brute);
        game.spawn_enemy(1, 8., GardenEnemyKind::Bucket);
        game.spawn_enemy(2, 7., GardenEnemyKind::Sprout);
        assert!(game.plant(1, 2));
        assert_eq!(game.enemies.len(), 1);
        assert_eq!(game.enemies[0].row, 2);
        assert_eq!(game.sunlight, 250);
        assert!(!game.plant(2, 2));
        assert_eq!(game.score, 550);
    }

    #[test]
    fn pause_freezes_combat_resources_and_input() {
        let mut game = running();
        game.spawn_enemy(2, 6., GardenEnemyKind::Sprout);
        game.make_sun(Vec2::new(1., 1.), 1.);
        let id = game.suns[0].id;
        game.toggle_pause();
        assert!(!game.advance(1.));
        assert!(!game.plant(2, 0));
        assert!(!game.collect(id));
        assert_eq!(game.enemies[0].x, 6.);
        assert_eq!(game.elapsed, 0.);
        assert_eq!(game.sunlight, 300);
    }

    #[test]
    fn each_lane_guard_rescues_once_then_crossing_loses() {
        let mut game = running();
        game.spawn_enemy(3, 0.04, GardenEnemyKind::Sprout);
        game.advance(0.1);
        assert!(!game.guards[3]);
        assert!(game.guard_x[3].is_some());
        assert!(game.enemies.is_empty());
        assert!(!game.over);
        for _ in 0..2 {
            game.advance(1.);
        }
        game.spawn_enemy(3, 0.04, GardenEnemyKind::Sprout);
        game.advance(0.1);
        assert!(game.over);
        assert!(!game.won);
    }

    #[test]
    fn victory_waits_for_scheduled_and_living_enemies() {
        let mut game = running();
        game.spawn_plan.clear();
        game.spawn_enemy(1, 8., GardenEnemyKind::Sprout);
        game.advance(0.1);
        assert!(!game.won);
        game.enemies[0].hp = 0.;
        game.advance(0.1);
        assert!(game.won);
        assert!(game.over);
        assert_eq!(game.progress, 1.);
        assert_eq!(game.score, 600);
        game.next_stage();
        assert_eq!(game.stage, 2);
        assert!(game.ready);
        assert_eq!(game.score, 600);
    }

    #[test]
    fn sunflower_generates_collectible_sun_and_collect_is_idempotent() {
        let mut game = GardenGame::new();
        game.select(GardenPlantKind::Sunflower.index());
        assert!(game.plant(1, 0));
        game.start();
        for _ in 0..5 {
            game.advance(1.);
        }
        let sun = game.suns.iter().find(|sun| sun.pos.x == 0.75).unwrap();
        let id = sun.id;
        assert!(game.collect(id));
        assert!(!game.collect(id));
        assert_eq!(game.sunlight, 275);
    }

    #[test]
    fn destroyed_front_plant_releases_enemy_to_walk_toward_next_plant() {
        let mut game = running();
        game.select(GardenPlantKind::Nut.index());
        assert!(game.plant(2, 4));
        game.plants[0].hp = 1.;
        game.spawn_enemy(2, 5.25, GardenEnemyKind::Sprout);
        game.advance(0.6);
        assert!(game.plants.is_empty());
        assert!(!game.enemies[0].biting);
        assert!(game.enemies[0].x < 5.24);
    }

    #[test]
    fn enemy_bites_front_wall_without_damaging_wall_behind_it() {
        let mut game = GardenGame::new();
        game.select(GardenPlantKind::Nut.index());
        assert!(game.plant(2, 2));
        assert!(game.plant(2, 4));
        game.start();
        game.spawn_enemy(2, 5.3, GardenEnemyKind::Bucket);
        game.advance(1.);
        assert_eq!(game.plants[0].hp, game.plants[0].max_hp);
        assert!(game.plants[1].hp < game.plants[1].max_hp);
        assert!(game.enemies[0].biting);
    }

    #[test]
    fn fixed_steps_match_across_frame_rates() {
        let mut single = running();
        let mut frames = running();
        for game in [&mut single, &mut frames] {
            assert!(game.plant(2, 1));
            game.spawn_enemy(2, 3., GardenEnemyKind::Bucket);
        }
        single.advance(1.);
        for _ in 0..60 {
            frames.advance(1. / 60.);
        }
        assert!((single.elapsed - frames.elapsed).abs() < 0.00001);
        assert!((single.enemies[0].x - frames.enemies[0].x).abs() < 0.00001);
        assert_eq!(single.enemies[0].hp, frames.enemies[0].hp);
        assert_eq!(single.sunlight, frames.sunlight);
        assert_eq!(single.shots.len(), frames.shots.len());
    }

    #[test]
    fn economy_and_steady_reinforcements_can_finish_all_three_stages() {
        let mut game = GardenGame::new();
        for stage in 1..=3 {
            assert_eq!(game.stage, stage);
            // No resource injection: two generators and two initial defenders
            // fit the actual starting budget, even in the first garden.
            game.select(GardenPlantKind::Sunflower.index());
            assert!(game.plant(0, 0));
            assert!(game.plant(4, 0));
            game.select(GardenPlantKind::Pea.index());
            assert!(game.plant(2, 1));
            assert!(game.plant(1, 1));
            game.start();
            for _ in 0..60 * 180 {
                game.advance(1. / 60.);
                let ids: Vec<_> = game.suns.iter().map(|sun| sun.id).collect();
                for id in ids {
                    game.collect(id);
                }
                for row in [3, 0, 4, 2, 1] {
                    if game.sunlight >= GardenPlantKind::Pea.cost()
                        && game.cooldowns[GardenPlantKind::Pea.index()] <= 0.001
                        && !game
                            .plants
                            .iter()
                            .any(|plant| plant.row == row && plant.col == 1)
                    {
                        game.select(GardenPlantKind::Pea.index());
                        game.plant(row, 1);
                        break;
                    }
                }
                // Later gardens teach adding a second defender. A second
                // shooter is a straightforward answer to heavier armor.
                if stage >= 2
                    && game.sunlight >= GardenPlantKind::Pea.cost()
                    && game.cooldowns[GardenPlantKind::Pea.index()] <= 0.001
                    && (0..GARDEN_ROWS).all(|row| {
                        game.plants
                            .iter()
                            .any(|plant| plant.row == row && plant.col == 1)
                    })
                {
                    for row in [1, 4, 2, 3, 0] {
                        if !game
                            .plants
                            .iter()
                            .any(|plant| plant.row == row && plant.col == 2)
                        {
                            game.select(GardenPlantKind::Pea.index());
                            game.plant(row, 2);
                            break;
                        }
                    }
                }
                if game.over {
                    break;
                }
            }
            assert!(
                game.won,
                "garden {stage} is not completable with simple lane coverage: {}",
                game.hint
            );
            assert!(
                game.guards.iter().filter(|guard| **guard).count()
                    >= if stage == 3 { 3 } else { 5 },
                "garden {stage} required an emergency rescue: {:?}, {:.1}s, remaining plants {:?}",
                game.guards,
                game.elapsed,
                game.plants
                    .iter()
                    .map(|plant| (plant.row, plant.col, plant.kind))
                    .collect::<Vec<_>>()
            );
            assert!(game.elapsed < 180.);
            eprintln!(
                "garden {stage}: {:.1}s, {} sunlight, {} score",
                game.elapsed, game.sunlight, game.score
            );
            game.next_stage();
        }
        assert_eq!(game.stage, 1);
        assert!(game.ready);
    }

    #[test]
    fn later_gardens_reward_more_than_one_shooter_per_lane() {
        for stage in [2, 3] {
            let mut game = GardenGame::for_stage(stage, 0);
            game.select(GardenPlantKind::Sunflower.index());
            assert!(game.plant(0, 0));
            assert!(game.plant(4, 0));
            game.select(GardenPlantKind::Pea.index());
            assert!(game.plant(2, 1));
            assert!(game.plant(1, 1));
            game.start();
            for _ in 0..60 * 180 {
                game.advance(1. / 60.);
                let ids: Vec<_> = game.suns.iter().map(|sun| sun.id).collect();
                for id in ids {
                    game.collect(id);
                }
                for row in [3, 0, 4, 2, 1] {
                    if game.sunlight >= GardenPlantKind::Pea.cost()
                        && game.cooldowns[GardenPlantKind::Pea.index()] <= 0.001
                        && !game
                            .plants
                            .iter()
                            .any(|plant| plant.row == row && plant.col == 1)
                    {
                        game.select(GardenPlantKind::Pea.index());
                        game.plant(row, 1);
                        break;
                    }
                }
                if game.over {
                    break;
                }
            }
            let rescues = game.guards.iter().filter(|guard| !**guard).count();
            eprintln!(
                "single-defender garden {stage}: won={}, rescues={rescues}, {:.1}s",
                game.won, game.elapsed
            );
            if stage == 3 {
                assert!(
                    !game.won || rescues > 2,
                    "final garden should reward reinforcements"
                );
            } else {
                assert!(
                    game.elapsed > 85.,
                    "reinforcements should clear armor sooner"
                );
            }
        }
    }

    #[test]
    fn chili_does_not_waste_sunlight_on_an_empty_lane() {
        let mut game = GardenGame::new();
        game.select(GardenPlantKind::Chili.index());
        assert!(!game.plant(2, 3));
        assert_eq!(game.sunlight, 300);
        game.start();
        assert!(!game.plant(2, 3));
        assert_eq!(game.sunlight, 300);
        assert_eq!(game.cooldowns[GardenPlantKind::Chili.index()], 0.);
        assert!(game.effects.is_empty());
    }
}
