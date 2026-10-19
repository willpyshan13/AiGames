#[path = "fruit_arcade.rs"]
mod fruit_arcade;
use fruit_arcade::*;
#[path = "fishing.rs"]
mod fishing;
use fishing::*;
#[path = "honey.rs"]
mod honey;
use honey::*;
#[path = "moon_festival.rs"]
mod moon_festival;
use moon_festival::*;
#[path = "slots.rs"]
mod slots;
use slots::*;
#[path = "fortune.rs"]
mod fortune;
use fortune::*;
#[path = "play_ui.rs"]
mod play_ui;
use play_ui::*;
#[path = "storybook.rs"]
mod storybook;
use storybook::*;
#[path = "retro.rs"]
mod retro;
use retro::*;
#[path = "sokoban.rs"]
mod sokoban;
use sokoban::*;
#[path = "breakout.rs"]
mod breakout;
use breakout::*;
#[path = "elements.rs"]
mod elements;
use elements::*;
#[path = "light_path.rs"]
mod light_path;
use light_path::*;
#[path = "garden_game.rs"]
mod garden_game;
use garden_game::*;
#[path = "garden_ui.rs"]
mod garden_ui;
use garden_ui::*;
#[path = "courier.rs"]
mod courier;
use courier::*;
#[cfg(debug_assertions)]
#[path = "qa.rs"]
mod qa;

use crate::game::{self, Instruction, Material, Runner, Save, Work};
use bevy::{
    audio::Volume,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::{ScalingMode, Viewport},
    window::PrimaryWindow,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const INK: Color = Color::srgb(0.15, 0.23, 0.25);
const MUTED: Color = Color::srgb(0.40, 0.47, 0.46);
const TEAL: Color = Color::srgb(0.12, 0.46, 0.42);
const PAPER: Color = Color::srgb(0.99, 0.98, 0.94);
const EDGE: Color = Color::srgb(0.85, 0.87, 0.80);
const NAVY: Color = Color::srgb(0.10, 0.19, 0.23);
const SLATE: Color = Color::srgb(0.17, 0.27, 0.31);
const MINT: Color = Color::srgb(0.66, 0.88, 0.73);
const SOFT: Color = Color::srgb(0.69, 0.77, 0.78);
const GARDEN_PAPER: Color = Color::srgb(0.97, 0.92, 0.77);
const GARDEN_GREEN: Color = Color::srgb(0.37, 0.53, 0.22);
const GARDEN_WOOD: Color = Color::srgb(0.35, 0.23, 0.12);
const GARDEN_WOOD_EDGE: Color = Color::srgb(0.78, 0.59, 0.31);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameMode {
    Island,
    CloudBridge,
    CrystalGarden,
    FreeBuild,
    Ball2048,
    Tetris,
    Math,
    Literacy,
    Firefighters,
    Detective,
    SpaceFarm,
    MusicFactory,
    PixelAssault,
    MushroomRun,
    TankBattle,
    SkyRaid,
    BubbleIsland,
    BubbleBomber,
    FruitNinja,
    Fortune,
    Slots,
    FruitArcade,
    Fishing,
    HoneyHunt,
    MoonFestival,
    Snake,
    Minesweeper,
    Classic2048,
    Klotski,
    Sokoban,
    Sudoku,
    Breakout,
    ElementLab,
    LightPath,
    GardenDefense,
    SpaceCourier,
}

impl GameMode {
    fn title(self) -> &'static str {
        match self {
            Self::Island => "磁力小岛",
            Self::CloudBridge => "云桥工程师",
            Self::CrystalGarden => "小小动物餐厅",
            Self::FreeBuild => "自由搭建工坊",
            Self::Ball2048 => "2048球球游戏",
            Self::Tetris => "方块消消乐",
            Self::Math => "数学游乐岛",
            Self::Literacy => "识字小岛",
            Self::Firefighters => "小小消防队",
            Self::Detective => "神秘博物馆侦探",
            Self::SpaceFarm => "太空种植园",
            Self::MusicFactory => "彩虹钢琴派对",
            Self::PixelAssault => "像素突击队",
            Self::MushroomRun => "蘑菇跳跳岛",
            Self::TankBattle => "迷你坦克大战",
            Self::SkyRaid => "星际飞行队",
            Self::BubbleIsland => "彩虹泡泡岛",
            Self::BubbleBomber => "泡泡爆破王",
            Self::FruitNinja => "水果忍者派对",
            Self::Fortune => "口袋大富翁",
            Self::Slots => "水果转转乐",
            Self::FruitArcade => "经典水果机",
            Self::Fishing => "捕鱼达人",
            Self::HoneyHunt => "小熊寻蜜",
            Self::MoonFestival => "中秋博饼会",
            Self::Snake => "贪吃蛇",
            Self::Minesweeper => "扫雷",
            Self::Classic2048 => "经典2048",
            Self::Klotski => "华容道",
            Self::Sokoban => "推箱子",
            Self::Sudoku => "数独乐园",
            Self::Breakout => "霓虹打砖块",
            Self::ElementLab => "元素小实验室",
            Self::LightPath => "光路小侦探",
            Self::GardenDefense => "花园守卫",
            Self::SpaceCourier => "星球快递员",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::Island => "选材料搭桥，让机器人走到星星身边",
            Self::CloudBridge => "在云海之间搭一座会走的桥",
            Self::CrystalGarden => "装修餐厅，准备动物们喜欢的美食",
            Self::FreeBuild => "没有任务和终点，想搭什么就搭什么",
            Self::Ball2048 => "从 2 开始滚动，合并数字，躲开陷阱",
            Self::Tetris => "竖屏下落方块，拼满一行就会消除",
            Self::Math => "一年级冒险，从数数到竞赛思维",
            Self::Literacy => "每天一小关，认识汉字好朋友",
            Self::Firefighters => "规划救援路线，帮助方块城的小动物",
            Self::Detective => "收集线索，找回走失的展品",
            Self::SpaceFarm => "调节光水温，培育星球植物",
            Self::MusicFactory => "追着彩色音符弹琴，连击演奏一首歌",
            Self::PixelAssault => "横版冲刺、跳跃和射击，守住能量森林",
            Self::MushroomRun => "横版跑跳、踩小怪、收集金币，抵达终点旗帜",
            Self::TankBattle => "在砖墙迷宫里移动、开火，守住蓝色基地",
            Self::SkyRaid => "驾驶战机穿越星云，击落来袭的敌机",
            Self::BubbleIsland => "瞄准发射彩色泡泡，连成三个就会爆开",
            Self::BubbleBomber => "放下水泡、炸开砖块，在迷宫里追击小怪",
            Self::FruitNinja => "划开飞起的水果，躲开危险炸弹",
            Self::Fortune => "掷骰子逛小镇，买地盖房，和小狐狸比资产",
            Self::Slots => "水果转轮依次停下，赢取免费虚拟积分",
            Self::FruitArcade => "复古跑灯水果机，灯光绕圈逐渐停下",
            Self::Fishing => "瞄准游动的鱼群，撒网收获金币",
            Self::HoneyHunt => "摇摆抓钩，收集草地里的甜甜蜂蜜",
            Self::MoonFestival => "摇动六颗骰子，收集月兔福礼",
            Self::Snake => "吃掉果实变长，别撞到自己和边界",
            Self::Minesweeper => "找出安全格，标记藏起来的地雷",
            Self::Classic2048 => "滑动数字格，合并出更大的数字",
            Self::Klotski => "移动棋子，为曹操打开通往出口的路",
            Self::Sokoban => "把每个箱子推到发光的目标格",
            Self::Sudoku => "4×4 入门与 9×9 经典，填数字挑战自己",
            Self::Breakout => "拖动挡板反弹小球，连击砖块收集道具",
            Self::ElementLab => "点亮元素周期表，拼出生活里的小配方",
            Self::LightPath => "旋转镜片，让光束真正反射到水晶",
            Self::GardenDefense => "收集阳光、种下伙伴，守住五条花园战线",
            Self::SpaceCourier => "规划航线、送完九件包裹、破解终点谜题",
        }
    }

    fn intro(self) -> &'static str {
        match self {
            Self::Island => "完成岛上的小任务，获得星星和新奖励。",
            Self::CloudBridge => "用放置和前进，越过云海中的缺口。",
            Self::CrystalGarden => "给小动物布置餐桌、厨房和食材区。",
            Self::FreeBuild => "挑一种方块，轻点搭建。你的作品会自动保存在这个档案里。",
            Self::Ball2048 => "左右换道，让相同数字的球碰到一起。",
            Self::Tetris => "左右移动、旋转和快速下落，把一整行填满就能消除。",
            Self::Math => "点亮五颗数学星球，每一步都有新发现。",
            Self::Literacy => "看图、认字、组词，一起点亮识字小路。",
            Self::Firefighters => "先判断情况，再选合适的救援工具。",
            Self::Detective => "仔细观察，线索会告诉你答案。",
            Self::SpaceFarm => "让植物得到刚刚好的照顾。",
            Self::MusicFactory => "让音符落到琴键上，弹出你的第一首歌。",
            Self::PixelAssault => "向前突击，跳过障碍，击退机械小怪。",
            Self::MushroomRun => "左右移动、看准时机跳跃。踩过小怪，收集金币，跑到终点旗帜。",
            Self::TankBattle => "绕开砖墙和炮弹，消灭所有敌方坦克。",
            Self::SkyRaid => "左右移动，发射能量弹，躲开陨石与敌机。",
            Self::BubbleIsland => "点战场瞄准，凑齐三个同色泡泡，让整串掉下来。",
            Self::BubbleBomber => "移动躲开水泡，等它爆开十字水柱，清空迷宫里的小怪。",
            Self::FruitNinja => "手指划过水果就能切开，别碰到带火花的炸弹。",
            Self::Fortune => "骰子滚动，角色跳格，盖起属于你的彩色小镇。",
            Self::Slots => "转动三列水果，中间一行相同图案越多奖励越多。",
            Self::FruitArcade => "选择水果投入虚拟积分，跑灯停稳后结算奖励。",
            Self::Fishing => "点击水面发射渔网，网住鱼群赚取金币。",
            Self::HoneyHunt => "等抓钩对准蜂蜜后轻点画面。抓到蜂蜜会自动拉回小熊身边。",
            Self::MoonFestival => "摇一摇六颗骰子，收下属于你的中秋福气。",
            Self::Snake => "控制小蛇吃果实，撞墙或撞到自己就结束。",
            Self::Minesweeper => "点击安全格查看数字提示；切换标记模式后，点击方块插旗。",
            Self::Classic2048 => "向任意方向滑动，让相同数字合并。",
            Self::Klotski => "先点选棋子，再用方向键移动；让曹操走到最下方出口。",
            Self::Sokoban => "用方向键推箱子，箱子只能推、不能拉。",
            Self::Sudoku => "点选空格，再点数字填入。每行、每列和每个小方宫都不能有重复数字。",
            Self::Breakout => "左右拖动挡板接球，打碎砖块，接住掉落的道具。",
            Self::ElementLab => "在元素周期表中找到需要的元素，拼出水、氧气等小配方。",
            Self::LightPath => "点镜片旋转方向，观察光线如何反射；让它绕过障碍照亮水晶。",
            Self::GardenDefense => {
                "先点植物卡片，再点草地种下。向日葵产生阳光，豌豆和冰花自动射击，坚果挡住怪物，辣椒清理整条战线。点阳光补充能量，准备好后开始进攻。"
            }
            Self::SpaceCourier => "选择星球和航线，管理燃料，完成九个送货点与终点挑战。",
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum BallKind {
    Number(u64),
    Trap,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct BallObstacle {
    lane: i8,
    kind: BallKind,
}

#[derive(Serialize, Deserialize)]
struct BallGame {
    #[serde(default)]
    rules_version: u8,
    number: u64,
    score: u64,
    lane: i8,
    next: usize,
    timer: f32,
    travel: f32,
    effect: f32,
    #[serde(default)]
    shields: u8,
    running: bool,
    failed: bool,
    finished: bool,
    track: Vec<BallObstacle>,
    #[serde(skip)]
    pointer_x: Option<f32>,
}

impl BallGame {
    fn new() -> Self {
        let mut game = Self {
            rules_version: 3,
            number: 2,
            score: 0,
            lane: 1,
            next: 0,
            timer: 0.,
            travel: 0.,
            effect: 0.,
            shields: 2,
            running: false,
            failed: false,
            finished: false,
            track: Vec::new(),
            pointer_x: None,
        };
        game.ensure_track();
        game
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn interval(&self) -> f32 {
        1.55 / (1. + (self.number.max(2) as f32).log2() * 0.075)
    }

    fn ensure_track(&mut self) {
        while self.next + 12 >= self.track.len() {
            let wave = (self.track.len() / 3) as i8;
            let good_lane = (wave + 1) % 3;
            let trap_lane = (wave + 2) % 3;
            let decoy_lane = 3 - good_lane - trap_lane;
            let decoy = if wave % 4 == 0 && self.number > 2 {
                self.number / 2
            } else {
                self.number.saturating_mul(4)
            };
            // The opening is a practice stretch: every lane is safe for the
            // first four gates, so players can learn to look for a match.
            let third_kind = if wave < 4 {
                BallKind::Number(self.number.saturating_mul(8).max(4))
            } else {
                BallKind::Trap
            };
            self.track.extend([
                BallObstacle {
                    lane: good_lane,
                    kind: BallKind::Number(self.number),
                },
                BallObstacle {
                    lane: decoy_lane,
                    kind: BallKind::Number(decoy.max(2)),
                },
                BallObstacle {
                    lane: trap_lane,
                    kind: third_kind,
                },
            ]);
        }
    }

    fn advance(&mut self, dt: f32) -> Option<String> {
        if !self.running {
            return None;
        }
        self.timer += dt;
        let interval = self.interval();
        self.travel = (self.timer / interval).clamp(0., 1.);
        self.effect = (self.effect - dt).max(0.);
        // Resolve the hit while the obstacle is still overlapping the player.
        // Waiting for the full travel interval made the visual collision feel
        // disconnected from the score and number update.
        if self.travel < 0.96 {
            return None;
        }
        self.timer = 0.;
        self.travel = 0.;
        self.ensure_track();
        let gate = &self.track[self.next..self.next + 3];
        self.next += 3;
        let chosen = gate
            .iter()
            .find(|obstacle| obstacle.lane == self.lane)
            .copied();
        let result = match chosen.map(|obstacle| obstacle.kind) {
            Some(BallKind::Trap) => {
                if self.shields > 0 {
                    self.shields -= 1;
                    self.lane = 1;
                    self.effect = 0.55;
                    format!("护盾挡住了陷阱！还剩 {} 次保护，继续滚动。", self.shields)
                } else {
                    self.running = false;
                    self.failed = true;
                    self.effect = 0.8;
                    "护盾用完后撞到陷阱了！点再来一次重新滚动。".to_string()
                }
            }
            Some(BallKind::Number(value)) if value == self.number => {
                self.number = value.saturating_mul(2);
                self.score += self.number;
                self.effect = 0.45;
                format!("合并成功！{} 球变成了 {}。", value, self.number)
            }
            Some(BallKind::Number(value)) => {
                format!("前方是 {} 球，继续找相同数字。", value)
            }
            None => "顺利躲开这一闸，继续向前滚！".to_string(),
        };
        // Rebuild future gates from the number we have now. This keeps a
        // missed number available in the next gate instead of dead-ending.
        self.track.truncate(self.next);
        self.ensure_track();
        if self.next >= self.track.len() && !self.failed {
            self.running = false;
            self.finished = true;
        }
        Some(result)
    }
}

const TETRIS_WIDTH: usize = 10;
const TETRIS_HEIGHT: usize = 20;
const TETRIS_LINES_PER_LEVEL: u32 = 20;

#[derive(Clone, Copy)]
struct TetrisGame {
    board: [u8; TETRIS_WIDTH * TETRIS_HEIGHT],
    kind: u8,
    rotation: u8,
    x: i8,
    y: i8,
    next: u8,
    score: u32,
    best_score: u32,
    lines: u32,
    level: u32,
    drop_clock: f32,
    feedback: &'static str,
    failed: bool,
}

impl TetrisGame {
    fn new() -> Self {
        Self {
            board: [0; TETRIS_WIDTH * TETRIS_HEIGHT],
            kind: 0,
            rotation: 0,
            x: 3,
            y: 0,
            next: 1,
            score: 0,
            best_score: 0,
            lines: 0,
            level: 1,
            drop_clock: 0.75,
            feedback: "左右移动，旋转方块，拼满整行！",
            failed: false,
        }
    }

    fn reset(&mut self) {
        let best_score = self.best_score.max(self.score);
        *self = Self::new();
        self.best_score = best_score;
    }

    fn base_cells(kind: u8) -> [(i8, i8); 4] {
        match kind % 7 {
            0 => [(0, 1), (1, 1), (2, 1), (3, 1)], // I
            1 => [(1, 0), (2, 0), (1, 1), (2, 1)], // O
            2 => [(1, 0), (0, 1), (1, 1), (2, 1)], // T
            3 => [(1, 0), (2, 0), (0, 1), (1, 1)], // S
            4 => [(0, 0), (1, 0), (1, 1), (2, 1)], // Z
            5 => [(0, 0), (0, 1), (1, 1), (2, 1)], // J
            _ => [(2, 0), (0, 1), (1, 1), (2, 1)], // L
        }
    }

    fn cells(kind: u8, rotation: u8) -> [(i8, i8); 4] {
        let mut cells = Self::base_cells(kind);
        if kind % 7 == 1 {
            return cells;
        }
        for _ in 0..rotation % 4 {
            for cell in &mut cells {
                *cell = (3 - cell.1, cell.0);
            }
        }
        cells
    }

    fn valid(&self, x: i8, y: i8, rotation: u8) -> bool {
        Self::cells(self.kind, rotation)
            .into_iter()
            .all(|(cx, cy)| {
                let px = x + cx;
                let py = y + cy;
                px >= 0
                    && px < TETRIS_WIDTH as i8
                    && py >= 0
                    && py < TETRIS_HEIGHT as i8
                    && self.board[py as usize * TETRIS_WIDTH + px as usize] == 0
            })
    }

    fn move_by(&mut self, dx: i8, dy: i8) -> bool {
        if !self.failed && self.valid(self.x + dx, self.y + dy, self.rotation) {
            self.x += dx;
            self.y += dy;
            true
        } else {
            false
        }
    }

    fn rotate(&mut self) {
        if self.failed {
            return;
        }
        let next = (self.rotation + 1) % 4;
        for kick in [0, -1, 1, -2, 2] {
            if self.valid(self.x + kick, self.y, next) {
                self.x += kick;
                self.rotation = next;
                self.feedback = "转好了！继续拼一整行。";
                return;
            }
        }
    }

    fn clear_lines(&mut self) -> u32 {
        let mut cleared = 0;
        let mut next_board = [0; TETRIS_WIDTH * TETRIS_HEIGHT];
        let mut target = TETRIS_HEIGHT as i8 - 1;
        for source in (0..TETRIS_HEIGHT).rev() {
            let start = source * TETRIS_WIDTH;
            if self.board[start..start + TETRIS_WIDTH]
                .iter()
                .all(|cell| *cell != 0)
            {
                cleared += 1;
            } else {
                let destination = target as usize * TETRIS_WIDTH;
                next_board[destination..destination + TETRIS_WIDTH]
                    .copy_from_slice(&self.board[start..start + TETRIS_WIDTH]);
                target -= 1;
            }
        }
        self.board = next_board;
        cleared
    }

    fn lock_piece(&mut self) {
        for (cx, cy) in Self::cells(self.kind, self.rotation) {
            let px = self.x + cx;
            let py = self.y + cy;
            if px >= 0 && px < TETRIS_WIDTH as i8 && py >= 0 && py < TETRIS_HEIGHT as i8 {
                self.board[py as usize * TETRIS_WIDTH + px as usize] = self.kind + 1;
            }
        }
        let cleared = self.clear_lines();
        if cleared > 0 {
            self.lines += cleared;
            self.level = self.lines / TETRIS_LINES_PER_LEVEL + 1;
            self.score += match cleared {
                1 => 100,
                2 => 300,
                3 => 500,
                _ => 800,
            } * self.level;
            self.best_score = self.best_score.max(self.score);
            self.feedback = match cleared {
                1 => "消掉一行！",
                2 => "双消！真漂亮。",
                3 => "三消连锁！",
                _ => "四连消！太厉害了！",
            };
        }
        self.kind = self.next;
        self.next = (self.next + 3) % 7;
        self.rotation = 0;
        self.x = 3;
        self.y = 0;
        self.drop_clock = (0.76 - (self.level.saturating_sub(1) as f32 * 0.055)).max(0.24);
        if !self.valid(self.x, self.y, self.rotation) {
            self.failed = true;
            self.feedback = "方块堆到顶了，点重新开始！";
        }
    }

    fn soft_drop(&mut self) {
        if self.move_by(0, 1) {
        } else if !self.failed {
            self.lock_piece();
        }
    }

    fn hard_drop(&mut self) {
        if self.failed {
            return;
        }
        while self.move_by(0, 1) {}
        self.lock_piece();
    }

    fn advance(&mut self, dt: f32) -> bool {
        if self.failed {
            return false;
        }
        self.drop_clock -= dt;
        if self.drop_clock > 0. {
            return false;
        }
        self.drop_clock = (0.76 - (self.level.saturating_sub(1) as f32 * 0.055)).max(0.24);
        self.soft_drop();
        true
    }

    fn cell(&self, x: usize, y: usize) -> u8 {
        let active = Self::cells(self.kind, self.rotation)
            .into_iter()
            .any(|(cx, cy)| self.x + cx == x as i8 && self.y + cy == y as i8);
        if active {
            self.kind + 1
        } else {
            self.board[y * TETRIS_WIDTH + x]
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TetrisSave {
    version: u8,
    board: Vec<u8>,
    kind: u8,
    rotation: u8,
    x: i8,
    y: i8,
    next: u8,
    score: u32,
    best_score: u32,
    lines: u32,
    level: u32,
    drop_clock: f32,
    failed: bool,
}

impl TetrisSave {
    fn from_game(game: &TetrisGame) -> Self {
        Self {
            version: 1,
            board: game.board.to_vec(),
            kind: game.kind,
            rotation: game.rotation,
            x: game.x,
            y: game.y,
            next: game.next,
            score: game.score,
            best_score: game.best_score,
            lines: game.lines,
            level: game.level,
            drop_clock: game.drop_clock,
            failed: game.failed,
        }
    }

    fn into_game(self) -> Option<TetrisGame> {
        if self.version != 1
            || self.board.len() != TETRIS_WIDTH * TETRIS_HEIGHT
            || self.board.iter().any(|cell| *cell > 7)
            || self.kind >= 7
            || self.next >= 7
            || self.rotation >= 4
            || !(-3..=9).contains(&self.x)
            || !(0..=19).contains(&self.y)
            || self.lines > 100_000
            || self.level != self.lines / TETRIS_LINES_PER_LEVEL + 1
            || !self.drop_clock.is_finite()
            || !(0. ..=1.).contains(&self.drop_clock)
        {
            return None;
        }
        let mut game = TetrisGame::new();
        game.board = self.board.try_into().ok()?;
        game.kind = self.kind;
        game.rotation = self.rotation;
        game.x = self.x;
        game.y = self.y;
        game.next = self.next;
        game.score = self.score;
        game.best_score = self.best_score.max(self.score);
        game.lines = self.lines;
        game.level = self.level;
        game.drop_clock = self.drop_clock;
        game.failed = self.failed;
        if !game.failed && !game.valid(game.x, game.y, game.rotation) {
            return None;
        }
        game.feedback = if game.failed {
            "方块堆到顶了，点重新开始！"
        } else {
            "欢迎回来，接着上一局玩！"
        };
        Some(game)
    }
}

fn load_tetris() -> TetrisGame {
    let path = save_directory().join("tetris-v1.json");
    match fs::read(&path) {
        Ok(bytes) => serde_json::from_slice::<TetrisSave>(&bytes)
            .ok()
            .and_then(TetrisSave::into_game)
            .unwrap_or_else(|| {
                // Preserve an unreadable save rather than replacing it on the next move.
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                let _ = fs::rename(
                    &path,
                    path.with_extension(format!("unreadable-{stamp}.json")),
                );
                TetrisGame::new()
            }),
        Err(_) => TetrisGame::new(),
    }
}

const PIANO_NOTES: [usize; 24] = [
    0, 2, 4, 2, 1, 3, 5, 3, 0, 2, 4, 6, 5, 3, 1, 0, 1, 3, 5, 6, 4, 2, 1, 0,
];
const PIANO_NAMES: [&str; 7] = ["Do", "Re", "Mi", "Fa", "Sol", "La", "Si"];

struct PianoGame {
    step: usize,
    score: usize,
    combo: usize,
    fall: f32,
    flash: f32,
    last_key: Option<usize>,
    feedback: String,
}

impl PianoGame {
    fn new() -> Self {
        Self {
            step: 0,
            score: 0,
            combo: 0,
            fall: 0.,
            flash: 0.,
            last_key: None,
            feedback: "练习模式：按亮起的琴键，音符会等你。".into(),
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn target(&self) -> usize {
        PIANO_NOTES[self.step % PIANO_NOTES.len()]
    }

    fn hit(&mut self, key: usize) {
        self.last_key = Some(key);
        self.flash = 0.28;
        if key != self.target() {
            self.feedback = format!(
                "这个是 {}，看准落下的 {} 音符。",
                PIANO_NAMES[key],
                PIANO_NAMES[self.target()]
            );
            return;
        }
        let timing = if self.fall > 1.12 {
            "完美！"
        } else if self.fall > 0.72 {
            "很好！"
        } else {
            "弹对啦！"
        };
        self.combo += 1;
        self.score += 10 + self.combo.min(10);
        self.step += 1;
        self.fall = 0.;
        self.feedback = format!("{} {} 连击", timing, self.combo);
    }

    fn advance(&mut self, dt: f32) -> bool {
        self.fall = (self.fall + dt).min(3.2);
        let was_flashing = self.flash > 0.;
        self.flash = (self.flash - dt).max(0.);
        was_flashing && self.flash == 0.
    }
}

#[derive(Clone)]
struct AssaultEnemy {
    lane: i8,
    x: f32,
    hp: u8,
    heavy: bool,
}

#[derive(Clone)]
struct AssaultBullet {
    lane: i8,
    x: f32,
}

#[derive(Clone)]
struct AssaultSpark {
    x: f32,
    lane: i8,
    life: f32,
    large: bool,
}

struct AssaultGame {
    lane: i8,
    player_x: f32,
    camera_x: f32,
    moving: bool,
    distance: f32,
    stage: u8,
    score: u32,
    combo: u32,
    lives: u8,
    jump: f32,
    cooldown: f32,
    spawn_clock: f32,
    redraw: f32,
    wave: u32,
    enemies: Vec<AssaultEnemy>,
    bullets: Vec<AssaultBullet>,
    sparks: Vec<AssaultSpark>,
    muzzle: f32,
    shake: f32,
    run_phase: f32,
    feedback: String,
    failed: bool,
    finished: bool,
}

impl AssaultGame {
    fn new() -> Self {
        let mut game = Self {
            lane: 1,
            player_x: 0.18,
            camera_x: 0.,
            moving: false,
            distance: 0.,
            stage: 0,
            score: 0,
            combo: 0,
            lives: 3,
            jump: 0.,
            cooldown: 0.,
            spawn_clock: 0.55,
            redraw: 0.,
            wave: 0,
            enemies: vec![],
            bullets: vec![],
            sparks: vec![],
            muzzle: 0.,
            shake: 0.,
            run_phase: 0.,
            feedback: "向前冲！按射击清理前方的机械小怪。".into(),
            failed: false,
            finished: false,
        };
        game.spawn_enemy();
        game
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn spawn_enemy(&mut self) {
        let heavy = self.wave > 0 && self.wave % (5 - self.stage.min(2)) as u32 == 0;
        self.enemies.push(AssaultEnemy {
            lane: 1,
            x: self.player_x + 0.78,
            hp: if heavy { 2 } else { 1 },
            heavy,
        });
        self.wave += 1;
    }

    fn move_player(&mut self, direction: f32, dt: f32) {
        if self.failed || self.finished || direction == 0. {
            return;
        }
        self.player_x = (self.player_x + direction.clamp(-1., 1.) * dt * 0.42).max(0.);
        self.camera_x = (self.player_x - 0.24).max(0.);
        self.distance = self.player_x * 100.;
        self.moving = true;
        self.run_phase += dt * 10.;
        let next_stage = (self.distance / 260.).floor().min(2.) as u8;
        if next_stage > self.stage {
            self.stage = next_stage;
            self.enemies.clear();
            self.bullets.clear();
            self.sparks.push(AssaultSpark {
                x: self.player_x + 0.35,
                lane: 1,
                life: 0.7,
                large: true,
            });
            self.feedback = match self.stage {
                1 => "第 2 关：悬崖石桥！炮台开始出场。".into(),
                _ => "第 3 关：夜光营地！最后的机械小队来了。".into(),
            };
        }
        if self.distance >= 780. {
            self.distance = 780.;
            self.player_x = 7.8;
            self.camera_x = self.player_x - 0.24;
            self.finished = true;
            self.enemies.clear();
            self.bullets.clear();
            self.sparks.push(AssaultSpark {
                x: self.player_x + 0.28,
                lane: 1,
                life: 0.8,
                large: true,
            });
            self.feedback = "三关全部完成！玩具突击队成功守住能量森林！".into();
        }
    }

    fn stop_player(&mut self) {
        self.moving = false;
    }

    fn jump(&mut self) {
        if !self.failed && !self.finished && self.jump <= 0. {
            self.jump = 0.58;
            self.feedback = "跳跃！可以越过贴近的敌人。".into();
        }
    }

    fn fire(&mut self) {
        if self.failed || self.finished || self.cooldown > 0. {
            return;
        }
        self.bullets.push(AssaultBullet {
            lane: self.lane,
            x: self.player_x + 0.10,
        });
        self.cooldown = 0.19;
        self.muzzle = 0.12;
        self.feedback = "哒哒哒！能量弹已经发射。".into();
    }

    fn advance(&mut self, dt: f32) -> bool {
        if self.failed || self.finished {
            return false;
        }
        let speed = 0.23 + (self.distance / 700.).min(0.15) + self.stage as f32 * 0.035;
        self.jump = (self.jump - dt).max(0.);
        self.cooldown = (self.cooldown - dt).max(0.);
        self.muzzle = (self.muzzle - dt).max(0.);
        self.shake = (self.shake - dt).max(0.);
        if self.moving {
            self.run_phase += dt * 10.;
        }
        for spark in &mut self.sparks {
            spark.life -= dt;
            spark.x += if spark.large { dt * 0.03 } else { dt * 0.12 };
        }
        self.sparks.retain(|spark| spark.life > 0.);
        self.spawn_clock -= dt;
        if self.spawn_clock <= 0. {
            self.spawn_enemy();
            self.spawn_clock = (1.35 - self.distance / 900. - self.stage as f32 * 0.12).max(0.52);
        }
        for bullet in &mut self.bullets {
            bullet.x += dt * 1.35;
        }
        for enemy in &mut self.enemies {
            enemy.x -= dt * speed;
        }

        let mut used_bullets = vec![false; self.bullets.len()];
        let mut defeated = vec![];
        for (enemy_index, enemy) in self.enemies.iter_mut().enumerate() {
            for (bullet_index, bullet) in self.bullets.iter().enumerate() {
                if !used_bullets[bullet_index]
                    && bullet.lane == enemy.lane
                    && (bullet.x - enemy.x).abs() < 0.065
                {
                    used_bullets[bullet_index] = true;
                    enemy.hp = enemy.hp.saturating_sub(1);
                    self.sparks.push(AssaultSpark {
                        x: enemy.x,
                        lane: enemy.lane,
                        life: 0.20,
                        large: false,
                    });
                    if enemy.hp == 0 {
                        defeated.push(enemy_index);
                    }
                    break;
                }
            }
        }
        self.bullets = self
            .bullets
            .drain(..)
            .enumerate()
            .filter_map(|(index, bullet)| {
                (!used_bullets[index] && bullet.x < 1.1).then_some(bullet)
            })
            .collect();
        defeated.sort_unstable();
        defeated.dedup();
        for index in defeated.into_iter().rev() {
            let enemy = self.enemies.remove(index);
            self.sparks.push(AssaultSpark {
                x: enemy.x,
                lane: enemy.lane,
                life: 0.46,
                large: true,
            });
            self.shake = 0.16;
            self.combo += 1;
            self.score += if enemy.heavy { 40 } else { 20 } + self.combo.min(10) * 2;
            self.feedback = format!("命中！{} 连击，能量 +{}。", self.combo, self.score);
        }

        let mut hit = false;
        self.enemies.retain(|enemy| {
            if enemy.x < self.player_x + 0.04 {
                if enemy.lane == self.lane && self.jump <= 0. {
                    hit = true;
                }
                false
            } else {
                true
            }
        });
        if hit {
            self.shake = 0.22;
            self.sparks.push(AssaultSpark {
                x: self.player_x,
                lane: self.lane,
                life: 0.38,
                large: true,
            });
            self.lives = self.lives.saturating_sub(1);
            self.combo = 0;
            self.feedback = if self.lives == 0 {
                self.failed = true;
                "能量护盾耗尽！点重新出发再试一次。".into()
            } else {
                format!("被撞到了！还剩 {} 格护盾，换道或跳起来。", self.lives)
            };
        }
        self.redraw += dt;
        if self.redraw > 0.04 {
            self.redraw = 0.;
            true
        } else {
            hit
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PlatformThingKind {
    Coin,
    Walker,
    Block,
}

#[derive(Clone)]
struct PlatformThing {
    x: f32,
    y: f32,
    kind: PlatformThingKind,
}

// A compact, original side-scrolling course. The course only moves while the
// player is walking, so the controls feel like a proper platform game instead
// of an auto-runner with a cosmetic left/right button.
struct PlatformGame {
    player_x: f32,
    player_y: f32,
    velocity_y: f32,
    distance: f32,
    walk_phase: f32,
    walk_timer: f32,
    walk_direction: i8,
    score: u32,
    coins: u32,
    lives: u8,
    jump_buffer: f32,
    invulnerable: f32,
    spawn_clock: f32,
    seed: u32,
    redraw: f32,
    things: Vec<PlatformThing>,
    feedback: String,
    failed: bool,
    finished: bool,
}

impl PlatformGame {
    // The progress meter is expressed in metres, while the stage uses a
    // compact 0..1-ish screen coordinate system.
    fn camera_x(&self) -> f32 {
        self.distance * 0.015
    }

    fn new() -> Self {
        Self {
            player_x: 0.18,
            player_y: 0.,
            velocity_y: 0.,
            distance: 0.,
            walk_phase: 0.,
            walk_timer: 0.,
            walk_direction: 1,
            score: 0,
            coins: 0,
            lives: 3,
            jump_buffer: 0.,
            invulnerable: 0.,
            spawn_clock: 0.65,
            seed: 0,
            redraw: 0.,
            things: vec![],
            feedback: "左右移动，点跳跃越过小怪和砖块！".into(),
            failed: false,
            finished: false,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn move_player(&mut self, direction: f32, dt: f32) -> bool {
        if self.failed || self.finished || dt <= 0. || direction.abs() < f32::EPSILON {
            return false;
        }
        let direction = direction.signum();
        let speed = if direction > 0. { 14. } else { 10. };
        let before = self.distance;
        self.distance = (self.distance + direction * dt * speed).clamp(0., 300.);
        if (self.distance - before).abs() < f32::EPSILON {
            return false;
        }
        self.walk_direction = direction as i8;
        self.walk_timer = 0.14;
        true
    }

    fn jump(&mut self) {
        if self.failed || self.finished {
            return;
        }
        if self.player_y <= 0.015 {
            self.velocity_y = 1.34;
            self.jump_buffer = 0.;
            self.feedback = "跳！看准落点继续向前。".into();
        } else if self.velocity_y <= 0. && self.player_y < 0.18 {
            // A slightly early tap carries into the landing, which feels much
            // better on touch screens than requiring a perfect frame.
            self.jump_buffer = 0.14;
            self.feedback = "已记住跳跃，落地就会马上起跳。".into();
        }
    }

    fn spawn(&mut self) {
        let kind = if self.distance < 50. {
            PlatformThingKind::Coin
        } else if self.distance < 105. {
            match self.seed % 4 {
                0 | 1 | 3 => PlatformThingKind::Coin,
                _ => PlatformThingKind::Block,
            }
        } else {
            match self.seed % 6 {
                0 | 1 | 4 => PlatformThingKind::Coin,
                2 | 5 => PlatformThingKind::Walker,
                _ => PlatformThingKind::Block,
            }
        };
        let y = match kind {
            PlatformThingKind::Coin => {
                if self.seed % 3 == 0 {
                    0.14
                } else {
                    0.43
                }
            }
            PlatformThingKind::Walker => 0.,
            PlatformThingKind::Block => 0.12,
        };
        // Things live in course coordinates. They advance across the stage
        // only while the player walks.
        self.things.push(PlatformThing {
            x: self.camera_x() + 1.12,
            y,
            kind,
        });
        self.seed += 1;
    }

    fn lose_life(&mut self, message: &str) {
        self.lives = self.lives.saturating_sub(1);
        self.player_x = 0.18;
        self.player_y = 0.;
        self.velocity_y = 0.;
        self.jump_buffer = 0.;
        self.invulnerable = 1.15;
        if self.lives == 0 {
            self.failed = true;
            self.feedback = "冒险暂停啦，点重新开始再闯一次！".into();
        } else {
            self.feedback = format!("{} 还剩 {} 颗爱心。", message, self.lives);
        }
    }

    fn advance(&mut self, dt: f32) -> bool {
        if self.failed || self.finished {
            return false;
        }
        self.walk_timer = (self.walk_timer - dt).max(0.);
        if self.walk_timer > 0. {
            self.walk_phase += dt * 18.;
        }
        self.invulnerable = (self.invulnerable - dt).max(0.);
        self.jump_buffer = (self.jump_buffer - dt).max(0.);
        self.velocity_y -= dt * 2.85;
        self.player_y = (self.player_y + self.velocity_y * dt).max(0.);
        if self.player_y <= 0. {
            self.velocity_y = 0.;
            if self.jump_buffer > 0. {
                self.velocity_y = 1.34;
                self.jump_buffer = 0.;
                self.feedback = "连跳成功！".into();
            }
        }
        self.spawn_clock -= dt;
        if self.spawn_clock <= 0. {
            self.spawn();
            self.spawn_clock = if self.distance < 105. {
                1.22
            } else {
                (1.12 - self.distance / 1200.).max(0.68)
            };
        }
        let mut danger = false;
        let mut earned = 0;
        let camera_x = self.camera_x();
        let player_x = self.player_x;
        self.things.retain(|thing| {
            let screen_x = thing.x - camera_x;
            let close = (screen_x - player_x).abs() < 0.085;
            match thing.kind {
                PlatformThingKind::Coin
                    if close && (self.player_y + 0.08 - thing.y).abs() < 0.20 =>
                {
                    earned += 1;
                    false
                }
                PlatformThingKind::Walker if close => {
                    if self.player_y > 0.14 && self.velocity_y <= 0.38 {
                        earned += 2;
                        false
                    } else {
                        danger = true;
                        false
                    }
                }
                PlatformThingKind::Block if close && self.player_y < 0.18 => {
                    danger = true;
                    false
                }
                _ => screen_x > -0.12,
            }
        });
        if earned > 0 {
            self.coins += earned;
            self.score += earned * 10;
            self.feedback = if earned >= 2 {
                "踩中小怪！获得 2 枚金币。".into()
            } else {
                "叮！收集到一枚金币。".into()
            };
        }
        if danger && self.invulnerable <= 0. {
            self.lose_life("碰到障碍啦！");
        }
        if self.distance >= 300. {
            self.finished = true;
            self.feedback = format!("到达终点旗帜！本关收集了 {} 枚金币。", self.coins);
            return true;
        }
        self.redraw += dt;
        if self.redraw >= 0.04 {
            self.redraw = 0.;
            true
        } else {
            danger || earned > 0
        }
    }
}

#[derive(Clone)]
struct TankEnemy {
    fire_clock: f32,
    warning: f32,
    x: i8,
    y: i8,
    direction: (i8, i8),
}

#[derive(Clone)]
struct TankBullet {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    friendly: bool,
}

const TANK_MAP_COUNT: u8 = 12;

struct TankGame {
    reserves: usize,
    defeated: usize,
    spawn_clock: f32,
    invulnerable: f32,
    player: (i8, i8),
    direction: (i8, i8),
    walls: Vec<(i8, i8)>,
    steel: Vec<(i8, i8)>,
    bushes: Vec<(i8, i8)>,
    map: u8,
    enemies: Vec<TankEnemy>,
    bullets: Vec<TankBullet>,
    lives: u8,
    score: u32,
    wave: u32,
    cooldown: f32,
    enemy_clock: f32,
    patrol_step: u32,
    redraw: f32,
    feedback: String,
    failed: bool,
}

impl TankGame {
    fn new() -> Self {
        let mut game = Self {
            reserves: 0,
            defeated: 0,
            spawn_clock: 0.,
            invulnerable: 1.5,
            player: (3, 6),
            direction: (0, -1),
            walls: vec![],
            steel: vec![],
            bushes: vec![],
            map: 0,
            enemies: vec![],
            bullets: vec![],
            lives: 3,
            score: 0,
            wave: 1,
            cooldown: 0.,
            enemy_clock: 0.95,
            patrol_step: 0,
            redraw: 0.,
            feedback: "守住蓝色基地！用方向键移动，开火击退红坦克。".into(),
            failed: false,
        };
        game.load_map(0);
        game.spawn_wave();
        game
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn level_total(&self) -> usize {
        [10, 12, 14, 14, 16, 16, 18, 20, 20, 22, 24, 26][self.map as usize]
    }

    fn active_limit(&self) -> usize {
        [2, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5][self.map as usize]
    }

    fn spawn_wave(&mut self) {
        self.enemies.clear();
        self.reserves = self.level_total();
        self.defeated = 0;
        self.spawn_clock = 2.;
        for _ in 0..self.active_limit() {
            self.spawn_reinforcement();
        }
    }

    fn spawn_reinforcement(&mut self) {
        if self.reserves == 0 || self.enemies.len() >= self.active_limit() {
            return;
        }
        let positions = [(0, 0), (3, 0), (6, 0), (0, 2), (6, 2)];
        if let Some((x, y)) = positions.into_iter().find(|&(x, y)| {
            !self.blocked(x, y)
                && (x - self.player.0).abs() + (y - self.player.1).abs() > 1
                && !self
                    .enemies
                    .iter()
                    .any(|enemy| (enemy.x, enemy.y) == (x, y))
        }) {
            self.enemies.push(TankEnemy {
                fire_clock: 1.4 + self.enemies.len() as f32 * 0.65,
                warning: 0.,
                x,
                y,
                direction: (0, 1),
            });
            self.reserves -= 1;
        }
    }

    fn blocked(&self, x: i8, y: i8) -> bool {
        !(0..=6).contains(&x)
            || !(0..=6).contains(&y)
            || self.walls.contains(&(x, y))
            || self.steel.contains(&(x, y))
    }

    fn load_map(&mut self, map: u8) {
        self.map = map % TANK_MAP_COUNT;
        let (walls, steel, bushes) = match self.map {
            0 => (
                vec![
                    (1, 1),
                    (2, 1),
                    (4, 1),
                    (5, 1),
                    (1, 3),
                    (2, 3),
                    (4, 3),
                    (5, 3),
                    (1, 5),
                    (5, 5),
                ],
                vec![(3, 1), (3, 3)],
                vec![(0, 2), (1, 2), (5, 4), (6, 4)],
            ),
            1 => (
                vec![
                    (0, 1),
                    (0, 2),
                    (2, 1),
                    (4, 1),
                    (6, 2),
                    (1, 4),
                    (2, 4),
                    (3, 4),
                    (5, 4),
                    (6, 4),
                ],
                vec![(2, 0), (2, 2), (4, 2), (5, 2)],
                vec![(1, 3), (2, 3), (4, 3), (5, 3)],
            ),
            2 => (
                vec![
                    (1, 0),
                    (5, 0),
                    (1, 2),
                    (2, 2),
                    (4, 2),
                    (5, 2),
                    (0, 4),
                    (6, 4),
                    (1, 5),
                    (5, 5),
                ],
                vec![(3, 1), (3, 2), (3, 3)],
                vec![(0, 1), (6, 1), (1, 3), (5, 3)],
            ),
            3 => (
                vec![
                    (1, 1),
                    (1, 2),
                    (1, 4),
                    (1, 5),
                    (5, 1),
                    (5, 2),
                    (5, 4),
                    (5, 5),
                ],
                vec![(3, 2), (3, 4)],
                vec![(0, 3), (1, 3), (5, 3), (6, 3)],
            ),
            4 => (
                vec![
                    (1, 1),
                    (2, 1),
                    (4, 1),
                    (5, 1),
                    (1, 5),
                    (2, 5),
                    (4, 5),
                    (5, 5),
                ],
                vec![(2, 3), (4, 3)],
                vec![(3, 1), (3, 2), (3, 4), (3, 5)],
            ),
            5 => (
                vec![(1, 1), (5, 1), (1, 3), (5, 3), (1, 5), (5, 5), (3, 3)],
                vec![(2, 2), (4, 2), (2, 4), (4, 4)],
                vec![(0, 3), (0, 4), (6, 3), (6, 4)],
            ),
            6 => (
                vec![
                    (1, 1),
                    (2, 1),
                    (3, 1),
                    (3, 3),
                    (4, 3),
                    (5, 3),
                    (1, 5),
                    (2, 5),
                ],
                vec![(4, 1), (2, 3), (3, 5)],
                vec![(0, 2), (1, 2), (5, 4), (6, 4)],
            ),
            7 => (
                vec![(1, 1), (5, 1), (1, 5), (5, 5), (3, 2), (3, 4)],
                vec![(2, 2), (4, 2), (2, 4), (4, 4)],
                vec![(2, 3), (3, 3), (4, 3), (0, 3), (6, 3)],
            ),
            8 => (
                vec![(1, 1), (5, 1), (2, 2), (4, 2), (2, 4), (4, 4), (1, 5), (5, 5)],
                vec![(3, 2), (3, 4)],
                vec![(1, 3), (5, 3)],
            ),
            9 => (
                vec![(2, 1), (4, 1), (2, 2), (4, 2), (2, 4), (4, 4), (2, 5), (4, 5)],
                vec![(0, 1), (6, 1), (0, 5), (6, 5)],
                vec![(0, 3), (6, 3)],
            ),
            10 => (
                vec![(1, 1), (2, 1), (4, 1), (5, 1), (1, 5), (5, 5)],
                vec![(3, 2), (1, 3), (5, 3), (3, 4)],
                vec![(1, 2), (5, 2), (1, 4), (5, 4)],
            ),
            11 => (
                vec![(1, 1), (5, 1), (1, 5), (5, 5)],
                vec![(2, 1), (4, 1), (2, 3), (4, 3), (2, 5), (4, 5)],
                vec![(1, 2), (5, 2), (1, 4), (5, 4)],
            ),
            _ => unreachable!("map index is reduced modulo TANK_MAP_COUNT"),
        };
        self.walls = walls;
        self.steel = steel;
        self.bushes = bushes;
        self.player = (3, 6);
        self.direction = (0, -1);
        self.bullets.clear();
        self.invulnerable = 1.5;
    }

    fn map_name(&self) -> &'static str {
        match self.map {
            0 => "红砖峡谷",
            1 => "灌木通道",
            2 => "钢墙要塞",
            3 => "双路突围",
            4 => "十字街区",
            5 => "环城包抄",
            6 => "曲线走廊",
            7 => "星形堡垒",
            8 => "风车岔路",
            9 => "曲径夹击",
            10 => "双翼包围",
            11 => "花环堡垒",
            _ => unreachable!("map index is reduced modulo TANK_MAP_COUNT"),
        }
    }

    fn retry_level(&mut self) {
        let wave = self.wave;
        let score = self.score;
        *self = Self::new();
        self.wave = wave;
        self.score = score;
        self.load_map(((wave - 1) % u32::from(TANK_MAP_COUNT)) as u8);
        self.spawn_wave();
        self.feedback = format!("重新挑战{}，护甲已补满！", self.map_name());
    }

    fn move_player(&mut self, dx: i8, dy: i8) {
        if self.failed {
            return;
        }
        self.direction = (dx, dy);
        let (x, y) = (self.player.0 + dx, self.player.1 + dy);
        if !self.blocked(x, y)
            && !self
                .enemies
                .iter()
                .any(|enemy| (enemy.x, enemy.y) == (x, y))
        {
            self.player = (x, y);
            self.feedback = "履带前进，注意红坦克的炮口。".into();
        }
    }

    fn fire(&mut self) {
        if self.failed || self.cooldown > 0. {
            return;
        }
        self.bullets.push(TankBullet {
            x: self.player.0 as f32 + 0.5,
            y: self.player.1 as f32 + 0.5,
            dx: self.direction.0 as f32,
            dy: self.direction.1 as f32,
            friendly: true,
        });
        self.cooldown = 0.28;
        self.feedback = "砰！炮弹已经出膛。".into();
    }

    fn advance(&mut self, dt: f32) -> bool {
        if self.failed {
            return false;
        }
        let dt = dt.clamp(0., 0.4);
        self.invulnerable = (self.invulnerable - dt).max(0.);
        self.cooldown = (self.cooldown - dt).max(0.);
        self.enemy_clock -= dt;
        for bullet in &mut self.bullets {
            bullet.x += bullet.dx * dt * if bullet.friendly { 3.1 } else { 2.0 };
            bullet.y += bullet.dy * dt * if bullet.friendly { 3.1 } else { 2.0 };
        }
        let mut removed = vec![false; self.bullets.len()];
        let mut defeated = vec![];
        let mut player_hit = false;
        for (bullet_index, bullet) in self.bullets.iter().enumerate() {
            let cell = (bullet.x.floor() as i8, bullet.y.floor() as i8);
            if self.blocked(cell.0, cell.1) {
                removed[bullet_index] = true;
                continue;
            }
            if bullet.friendly {
                if let Some(enemy_index) = self
                    .enemies
                    .iter()
                    .position(|enemy| (enemy.x, enemy.y) == cell)
                {
                    removed[bullet_index] = true;
                    defeated.push(enemy_index);
                }
            } else if cell == self.player {
                removed[bullet_index] = true;
                player_hit = true;
            }
        }
        defeated.sort_unstable();
        defeated.dedup();
        for index in defeated.into_iter().rev() {
            self.enemies.remove(index);
            self.defeated += 1;
            self.score += 50;
            self.feedback = "命中红坦克！继续清理战场。".into();
        }
        self.bullets = self
            .bullets
            .drain(..)
            .enumerate()
            .filter_map(|(index, bullet)| {
                (!removed[index]
                    && (-0.2..7.2).contains(&bullet.x)
                    && (-0.2..7.2).contains(&bullet.y))
                .then_some(bullet)
            })
            .collect();
        if player_hit && self.invulnerable <= 0. {
            self.invulnerable = 2.5;
            self.lives = self.lives.saturating_sub(1);
            if self.lives == 0 {
                self.failed = true;
                self.feedback = "护甲耗尽！点「重试本关」继续挑战。".into();
            } else {
                self.player = (3, 6);
                self.feedback = format!("挨了一炮！还剩 {} 格护甲。", self.lives);
            }
        }
        // Fire on an independent rhythm. Warning locks the barrel and position;
        // stepping into a firing lane never triggers an instant aimed shot.
        for enemy in &mut self.enemies {
            if enemy.warning > 0. {
                enemy.warning = (enemy.warning - dt).max(0.);
                if enemy.warning == 0. {
                    self.bullets.push(TankBullet {
                        x: enemy.x as f32 + 0.5,
                        y: enemy.y as f32 + 0.5,
                        dx: enemy.direction.0 as f32,
                        dy: enemy.direction.1 as f32,
                        friendly: false,
                    });
                    enemy.fire_clock = 2.8;
                }
            } else {
                enemy.fire_clock -= dt;
                if enemy.fire_clock <= 0. {
                    enemy.warning = 0.9;
                }
            }
        }
        if self.enemy_clock <= 0. {
            self.enemy_clock = 0.95;
            self.patrol_step = self.patrol_step.wrapping_add(1);
            let walls = self.walls.clone();
            let steel = self.steel.clone();
            let mut positions: Vec<_> = self
                .enemies
                .iter()
                .map(|enemy| (enemy.x, enemy.y))
                .collect();
            for (index, enemy) in self.enemies.iter_mut().enumerate() {
                if enemy.warning > 0. || enemy.fire_clock >= 2.8 {
                    continue;
                }
                let choices: Vec<_> = [(0, -1), (1, 0), (0, 1), (-1, 0)]
                    .into_iter()
                    .filter(|&(dx, dy)| {
                        let next = (enemy.x + dx, enemy.y + dy);
                        (0..=6).contains(&next.0)
                            && (0..=6).contains(&next.1)
                            && !walls.contains(&next)
                            && !steel.contains(&next)
                            && next != self.player
                            && positions
                                .iter()
                                .enumerate()
                                .all(|(other, &position)| other == index || position != next)
                    })
                    .collect();
                if !choices.is_empty() {
                    let direction = choices
                        [(self.patrol_step as usize + self.wave as usize + index) % choices.len()];
                    enemy.direction = direction;
                    enemy.x += direction.0;
                    enemy.y += direction.1;
                    positions[index] = (enemy.x, enemy.y);
                }
            }
        }
        if self.failed {
            return true;
        }
        if self.reserves > 0 && self.enemies.len() < self.active_limit() {
            self.spawn_clock -= dt;
            if self.spawn_clock <= 0. {
                self.spawn_reinforcement();
                self.spawn_clock = 2.;
            }
        } else {
            self.spawn_clock = 2.;
        }
        if self.enemies.is_empty() && self.reserves == 0 {
            self.wave += 1;
            self.load_map(((self.wave - 1) % u32::from(TANK_MAP_COUNT)) as u8);
            self.lives = (self.lives + 1).min(3);
            self.cooldown = 0.;
            self.enemy_clock = 1.5;
            self.spawn_wave();
            self.feedback = format!(
                "进入{}：第 {} 关，补充一格护甲！",
                self.map_name(),
                self.wave
            );
        }
        self.redraw += dt;
        if self.redraw >= 0.05 {
            self.redraw = 0.;
            true
        } else {
            player_hit
        }
    }
}

#[derive(Clone)]
struct SkyEnemy {
    x: f32,
    y: f32,
    hp: u8,
    heavy: bool,
}

#[derive(Clone)]
struct SkyShot {
    x: f32,
    y: f32,
    dy: f32,
    friendly: bool,
}

#[derive(Clone)]
struct SkyRock {
    x: f32,
    y: f32,
    size: f32,
}

#[derive(Clone)]
struct SkyBoss {
    x: f32,
    y: f32,
    hp: u16,
    max_hp: u16,
    phase: f32,
}

struct SkyGame {
    x: f32,
    score: u32,
    lives: u8,
    combo: u32,
    stage: u32,
    cooldown: f32,
    spawn_clock: f32,
    enemy_fire: f32,
    redraw: f32,
    star_scroll: f32,
    wave: u32,
    enemies: Vec<SkyEnemy>,
    shots: Vec<SkyShot>,
    rocks: Vec<SkyRock>,
    boss: Option<SkyBoss>,
    boss_round: u32,
    next_boss_score: u32,
    feedback: String,
    failed: bool,
}

impl SkyGame {
    fn new() -> Self {
        let mut game = Self {
            x: 0.5,
            score: 0,
            lives: 3,
            combo: 0,
            stage: 1,
            cooldown: 0.,
            spawn_clock: 0.35,
            enemy_fire: 1.1,
            redraw: 0.,
            star_scroll: 0.,
            wave: 0,
            enemies: vec![],
            shots: vec![],
            rocks: vec![
                SkyRock {
                    x: 0.18,
                    y: 0.22,
                    size: 0.07,
                },
                SkyRock {
                    x: 0.76,
                    y: 0.48,
                    size: 0.10,
                },
                SkyRock {
                    x: 0.42,
                    y: 0.66,
                    size: 0.06,
                },
            ],
            boss: None,
            boss_round: 0,
            next_boss_score: 300,
            feedback: "星云前方有敌机！按住战机左右侧移动，能量炮会自动开火。".into(),
            failed: false,
        };
        game.spawn_enemy();
        game
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn move_x(&mut self, delta: f32) {
        if !self.failed {
            self.x = (self.x + delta).clamp(0.08, 0.92);
        }
    }

    fn fire(&mut self) {
        if self.failed || self.cooldown > 0. {
            return;
        }
        self.shots.extend([
            SkyShot {
                x: self.x - 0.025,
                y: 0.71,
                dy: -0.92,
                friendly: true,
            },
            SkyShot {
                x: self.x + 0.025,
                y: 0.71,
                dy: -0.92,
                friendly: true,
            },
        ]);
        self.cooldown = 0.16;
        self.feedback = "双发能量弹！".into();
    }

    fn spawn_enemy(&mut self) {
        let heavy = self.wave > 0 && self.wave % 6 == 0;
        let x = 0.12 + ((self.wave * 37 % 73) as f32 / 100.);
        self.enemies.push(SkyEnemy {
            x,
            y: -0.08,
            hp: if heavy { 2 } else { 1 },
            heavy,
        });
        if self.wave % 3 == 2 {
            self.rocks.push(SkyRock {
                x: 0.12 + ((self.wave * 23 % 75) as f32 / 100.),
                y: -0.12,
                size: if self.wave % 2 == 0 { 0.075 } else { 0.10 },
            });
        }
        self.wave += 1;
    }

    fn maybe_spawn_boss(&mut self) {
        if self.boss.is_some() || self.score < self.next_boss_score {
            return;
        }
        self.boss_round += 1;
        let hp = 14 + self.boss_round as u16 * 4;
        self.boss = Some(SkyBoss {
            x: 0.5,
            y: 0.24,
            hp,
            max_hp: hp,
            phase: 0.,
        });
        // Each victory creates a much longer runway before the next mothership.
        self.next_boss_score += 800;
        self.feedback = format!("警报！第 {} 艘首领母舰正在接近。", self.boss_round);
    }

    fn lose_life(&mut self, message: &'static str) {
        self.lives = self.lives.saturating_sub(1);
        self.combo = 0;
        if self.lives == 0 {
            self.failed = true;
            self.feedback = "战机受损坠落！点重新起飞继续。".into();
        } else {
            self.feedback = format!("{} 还剩 {} 架战机。", message, self.lives);
        }
    }

    fn advance(&mut self, dt: f32) -> bool {
        if self.failed {
            return false;
        }
        self.star_scroll = (self.star_scroll + dt * 0.16) % 1.;
        self.cooldown = (self.cooldown - dt).max(0.);
        // Firing is automatic: steering is the only control needed in play.
        if self.cooldown <= f32::EPSILON {
            self.fire();
        }
        self.spawn_clock -= dt;
        if self.spawn_clock <= 0. && self.boss.is_none() {
            self.spawn_enemy();
            self.spawn_clock = (1.0 - self.stage as f32 * 0.035).max(0.48);
        }
        for enemy in &mut self.enemies {
            enemy.y += dt * (0.16 + self.stage as f32 * 0.012);
        }
        for rock in &mut self.rocks {
            rock.y += dt * 0.045;
            if rock.y > 1.14 {
                rock.y = -0.10;
            }
        }
        if let Some(boss) = &mut self.boss {
            boss.phase += dt * (0.95 + self.boss_round as f32 * 0.12);
            boss.x = 0.5 + boss.phase.sin() * 0.29;
        }
        for shot in &mut self.shots {
            shot.y += shot.dy * dt;
        }
        self.enemy_fire -= dt;
        if self.enemy_fire <= 0. {
            self.enemy_fire = (1.15 - self.stage as f32 * 0.025).max(0.58);
            if let Some(boss) = &self.boss {
                for offset in [-0.10, 0., 0.10] {
                    self.shots.push(SkyShot {
                        x: boss.x + offset,
                        y: boss.y + 0.11,
                        dy: 0.48,
                        friendly: false,
                    });
                }
            } else {
                for enemy in self
                    .enemies
                    .iter()
                    .filter(|enemy| enemy.y > 0.08 && enemy.y < 0.74)
                {
                    self.shots.push(SkyShot {
                        x: enemy.x,
                        y: enemy.y + 0.06,
                        dy: 0.42,
                        friendly: false,
                    });
                }
            }
        }

        let mut remove_shots = vec![false; self.shots.len()];
        let mut defeated = vec![];
        let mut boss_defeated = false;
        let mut player_hit = false;
        for (shot_index, shot) in self.shots.iter().enumerate() {
            if shot.y < -0.08 || shot.y > 1.08 {
                remove_shots[shot_index] = true;
                continue;
            }
            if shot.friendly {
                if let Some(boss) = &mut self.boss
                    && (boss.x - shot.x).abs() < 0.14
                    && (boss.y - shot.y).abs() < 0.10
                {
                    remove_shots[shot_index] = true;
                    boss.hp = boss.hp.saturating_sub(1);
                    boss_defeated = boss.hp == 0;
                } else if let Some(enemy_index) = self.enemies.iter().position(|enemy| {
                    (enemy.x - shot.x).abs() < 0.065 && (enemy.y - shot.y).abs() < 0.075
                }) {
                    remove_shots[shot_index] = true;
                    if self.enemies[enemy_index].hp > 0 {
                        self.enemies[enemy_index].hp -= 1;
                        if self.enemies[enemy_index].hp == 0 {
                            defeated.push(enemy_index);
                        }
                    }
                }
            } else if (shot.x - self.x).abs() < 0.065 && shot.y > 0.76 {
                remove_shots[shot_index] = true;
                player_hit = true;
            }
        }
        defeated.sort_unstable();
        defeated.dedup();
        for index in defeated.into_iter().rev() {
            let enemy = self.enemies.remove(index);
            self.combo += 1;
            self.score += if enemy.heavy { 80 } else { 30 } + self.combo.min(10) * 3;
            self.feedback = format!("敌机击落！{} 连击。", self.combo);
        }
        if boss_defeated {
            let boss = self
                .boss
                .take()
                .expect("boss should exist while taking damage");
            let reward = 420 + self.boss_round * 90;
            self.score += reward;
            self.combo += 5;
            self.feedback = format!("首领母舰击毁！获得 {} 分。", reward);
            // A cleared boss releases the remaining airspace for the next wave.
            self.enemies.clear();
            self.shots.retain(|shot| shot.friendly);
            let _ = boss;
        }
        self.shots = self
            .shots
            .drain(..)
            .enumerate()
            .filter_map(|(index, shot)| (!remove_shots[index]).then_some(shot))
            .collect();
        self.enemies.retain(|enemy| {
            if enemy.y > 1.04 {
                if (enemy.x - self.x).abs() < 0.13 {
                    player_hit = true;
                }
                false
            } else {
                true
            }
        });
        if self
            .rocks
            .iter()
            .any(|rock| (rock.x - self.x).abs() < rock.size && (rock.y - 0.84).abs() < rock.size)
        {
            player_hit = true;
        }
        if player_hit {
            self.lose_life("撞上敌方火力！");
            self.rocks.retain(|rock| (rock.y - 0.84).abs() > rock.size);
        }
        let next_stage = self.score / 300 + 1;
        if next_stage > self.stage {
            self.stage = next_stage;
            self.feedback = format!("进入第 {} 星区，敌机速度提升！", self.stage);
        }
        self.maybe_spawn_boss();
        self.redraw += dt;
        if self.redraw >= 0.04 {
            self.redraw = 0.;
            true
        } else {
            player_hit
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BubbleKind {
    Coral,
    Gold,
    Mint,
    Violet,
    Rainbow,
    Bomb,
}

#[derive(Clone)]
struct BubbleCell {
    col: i8,
    row: i8,
    kind: BubbleKind,
}

#[derive(Clone)]
struct BubbleShot {
    x: f32,
    y: f32,
    dx: f32,
    kind: BubbleKind,
}

#[derive(Clone)]
struct BubblePop {
    position: Vec2,
    kind: BubbleKind,
    life: f32,
}

#[derive(Component)]
struct BubbleBoard;

struct BubbleGame {
    aim: Vec2,
    pops: Vec<BubblePop>,
    cells: Vec<BubbleCell>,
    shot: Option<BubbleShot>,
    next: BubbleKind,
    score: u32,
    level: u32,
    misses: u8,
    shot_count: u32,
    redraw: f32,
    feedback: String,
    failed: bool,
}

impl BubbleGame {
    fn new() -> Self {
        let mut game = Self {
            aim: Vec2::new(0.5, 0.2),
            pops: vec![],
            cells: vec![],
            shot: None,
            next: BubbleKind::Coral,
            score: 0,
            level: 1,
            misses: 0,
            shot_count: 0,
            redraw: 0.,
            feedback: "按住棋盘瞄准，松手发射；连到两个同色泡泡就能消除。".into(),
            failed: false,
        };
        game.seed_board();
        game
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn palette(index: u32) -> BubbleKind {
        match index % 4 {
            0 => BubbleKind::Coral,
            1 => BubbleKind::Gold,
            2 => BubbleKind::Mint,
            _ => BubbleKind::Violet,
        }
    }

    fn position(col: i8, row: i8) -> Vec2 {
        Vec2::new(
            0.12 + col as f32 * 0.12 + if row % 2 != 0 { 0.06 } else { 0. },
            0.12 + row as f32 * 0.105,
        )
    }

    fn seed_board(&mut self) {
        self.cells.clear();
        // The concept screen opens with a substantial canopy across the top,
        // so seed five staggered rows instead of leaving most of the board
        // empty on the first turn.
        for row in 0..if self.level == 1 { 5 } else { 6 } {
            for col in 0..7 {
                self.cells.push(BubbleCell {
                    col,
                    row,
                    kind: Self::palette(((col / 2 + row) % 4) as u32),
                });
            }
        }
    }

    fn kind_after_shot(&self) -> BubbleKind {
        if self.shot_count > 0 && self.shot_count % 6 == 0 {
            return BubbleKind::Rainbow;
        }
        let mut kinds = Vec::new();
        for cell in &self.cells {
            if !kinds.contains(&cell.kind) {
                kinds.push(cell.kind);
            }
        }
        if kinds.is_empty() {
            BubbleKind::Coral
        } else {
            kinds[self.shot_count as usize % kinds.len()]
        }
    }

    fn shot_toward(&self, target: Vec2) -> BubbleShot {
        let vertical = (0.89 - target.y).max(0.16);
        BubbleShot {
            x: 0.5,
            y: 0.89,
            dx: ((target.x - 0.5) / vertical * 0.65).clamp(-1.3, 1.3),
            kind: self.next,
        }
    }

    fn move_shot(shot: &mut BubbleShot, dt: f32) {
        shot.x += shot.dx * dt;
        shot.y -= 0.65 * dt;
        if shot.x < 0.06 {
            shot.x = 0.12 - shot.x;
            shot.dx = shot.dx.abs();
        }
        if shot.x > 0.94 {
            shot.x = 1.88 - shot.x;
            shot.dx = -shot.dx.abs();
        }
    }

    fn contact(&self, shot: &BubbleShot) -> bool {
        shot.y <= 0.12
            || self.cells.iter().any(|cell| {
                Self::position(cell.col, cell.row).distance(Vec2::new(shot.x, shot.y)) <= 0.114
            })
    }

    fn trajectory(&self) -> Vec<Vec2> {
        let mut shot = self.shot_toward(self.aim);
        let mut dots = Vec::new();
        for i in 0..180 {
            Self::move_shot(&mut shot, 1. / 120.);
            if i % 6 == 0 {
                dots.push(Vec2::new(shot.x, shot.y));
            }
            if self.contact(&shot) {
                break;
            }
        }
        dots
    }

    fn recommended_aim(&self) -> Option<Vec2> {
        let mut fallback = None;
        for i in 0..81 {
            let dx = -1.3 + i as f32 * 2.6 / 80.;
            let target = Vec2::new(0.5 + dx * 0.16 / 0.65, 0.73);
            let mut shot = self.shot_toward(target);
            for _ in 0..180 {
                Self::move_shot(&mut shot, 1. / 120.);
                if self.contact(&shot) {
                    if let Some(index) = self.cells.iter().position(|cell| {
                        Self::position(cell.col, cell.row).distance(Vec2::new(shot.x, shot.y))
                            <= 0.114
                    }) {
                        if self.cells[index].kind == self.next
                            || matches!(self.next, BubbleKind::Rainbow | BubbleKind::Bomb)
                        {
                            if self.same_cluster(index).len() >= 2 {
                                return Some(target);
                            }
                            fallback = Some(target);
                        }
                    }
                    break;
                }
            }
        }
        fallback
    }

    fn shoot(&mut self, target: Vec2) {
        if self.failed || self.shot.is_some() {
            return;
        }
        self.aim = target;
        self.shot = Some(self.shot_toward(target));
        self.shot_count += 1;
    }

    fn vacant_nearest(&self, point: Vec2) -> (i8, i8) {
        let mut best = (3, 7);
        let mut distance = f32::INFINITY;
        for row in 0..13 {
            for col in 0..7 {
                if self
                    .cells
                    .iter()
                    .any(|cell| cell.col == col && cell.row == row)
                {
                    continue;
                }
                let p = Self::position(col, row);
                let d = p.distance_squared(point);
                if d < distance {
                    distance = d;
                    best = (col, row);
                }
            }
        }
        best
    }

    fn touching(&self, index: usize) -> Vec<usize> {
        let origin = Self::position(self.cells[index].col, self.cells[index].row);
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(other, cell)| {
                (other != index && Self::position(cell.col, cell.row).distance(origin) < 0.137)
                    .then_some(other)
            })
            .collect()
    }

    fn same_cluster(&self, start: usize) -> Vec<usize> {
        let kind = self.cells[start].kind;
        let mut seen = vec![start];
        let mut cursor = 0;
        while cursor < seen.len() {
            let index = seen[cursor];
            for next in self.touching(index) {
                if self.cells[next].kind == kind && !seen.contains(&next) {
                    seen.push(next);
                }
            }
            cursor += 1;
        }
        seen
    }

    fn remove_indexes(&mut self, indexes: &[usize]) -> usize {
        let mut sorted = indexes.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        for index in sorted.iter().rev() {
            let cell = self.cells.remove(*index);
            self.pops.push(BubblePop {
                position: Self::position(cell.col, cell.row),
                kind: cell.kind,
                life: 0.65,
            });
        }
        sorted.len()
    }

    fn drop_unanchored(&mut self) -> usize {
        let mut anchored: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| (cell.row == 0).then_some(index))
            .collect();
        let mut cursor = 0;
        while cursor < anchored.len() {
            let index = anchored[cursor];
            for next in self.touching(index) {
                if !anchored.contains(&next) {
                    anchored.push(next);
                }
            }
            cursor += 1;
        }
        let dropped: Vec<_> = (0..self.cells.len())
            .filter(|index| !anchored.contains(index))
            .collect();
        self.remove_indexes(&dropped)
    }

    fn settle(&mut self, shot: BubbleShot) {
        let (col, row) = self.vacant_nearest(Vec2::new(shot.x, shot.y));
        let mut kind = shot.kind;
        if kind == BubbleKind::Rainbow {
            if let Some(neighbor) = self.cells.iter().find(|cell| {
                Self::position(cell.col, cell.row).distance(Self::position(col, row)) < 0.137
                    && matches!(
                        cell.kind,
                        BubbleKind::Coral
                            | BubbleKind::Gold
                            | BubbleKind::Mint
                            | BubbleKind::Violet
                    )
            }) {
                kind = neighbor.kind;
            } else {
                kind = BubbleKind::Coral;
            }
        }
        self.cells.push(BubbleCell { col, row, kind });
        let placed = self.cells.len() - 1;
        let removed = if shot.kind == BubbleKind::Bomb {
            let center = Self::position(col, row);
            let bombed: Vec<_> = self
                .cells
                .iter()
                .enumerate()
                .filter_map(|(index, cell)| {
                    (Self::position(cell.col, cell.row).distance(center) < 0.20).then_some(index)
                })
                .collect();
            self.remove_indexes(&bombed)
        } else {
            let cluster = self.same_cluster(placed);
            if cluster.len() >= 3 {
                self.remove_indexes(&cluster)
            } else {
                0
            }
        };
        if removed > 0 {
            let dropped = self.drop_unanchored();
            self.score += removed as u32 * 12 + dropped as u32 * 18;
            self.misses = 0;
            self.feedback = if dropped > 0 {
                format!("爆开 {} 个，掉落 {} 个！", removed, dropped)
            } else {
                format!("连消 {} 个泡泡！", removed)
            };
        } else {
            self.misses = (self.misses + 1).min(8);
            self.feedback = if self.level == 1 {
                "慢慢瞄准，第一关不会下压。".into()
            } else {
                format!("再连一个同色泡泡！距离下移还有 {} 次", 8 - self.misses)
            };
            if self.misses >= 8 && self.level > 1 {
                self.misses = 0;
                for cell in &mut self.cells {
                    cell.row += 1;
                }
                for col in 0..7 {
                    if col != 2 {
                        self.cells.push(BubbleCell {
                            col,
                            row: 0,
                            kind: Self::palette(self.shot_count + col as u32),
                        });
                    }
                }
                self.feedback = "泡泡墙下压了，小心！".into();
            }
        }
        if self.cells.is_empty() {
            self.level += 1;
            self.seed_board();
            self.feedback = format!("第 {} 关！新的彩虹泡泡来了。", self.level);
        }
        self.next = self.kind_after_shot();
        if self.cells.iter().any(|cell| cell.row >= 7) {
            self.failed = true;
            self.feedback = "泡泡压到底了，重新挑战吧！".into();
        }
    }

    fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.min(0.05);
        let had_pops = !self.pops.is_empty();
        for pop in &mut self.pops {
            pop.life -= dt;
        }
        self.pops.retain(|pop| pop.life > 0.);
        if self.shot.is_none() && self.pops.is_empty() {
            return had_pops;
        }
        // Small substeps keep the visible aiming guide and live collisions identical.
        let steps = (dt / (1. / 120.)).ceil().max(1.) as usize;
        let mut settled = false;
        for _ in 0..steps {
            if let Some(mut shot) = self.shot.take() {
                Self::move_shot(&mut shot, dt / steps as f32);
                if self.contact(&shot) {
                    self.settle(shot);
                    settled = true;
                    break;
                }
                self.shot = Some(shot);
            }
        }
        self.redraw += dt;
        if self.redraw >= 1. / 60. || settled {
            self.redraw = 0.;
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
struct BomberBoard {
    first_column: i8,
    visible_columns: i8,
}

const BOMBER_COLUMNS: i8 = 17;
const BOMBER_ROWS: i8 = 9;
const BOMBER_MAX_ROUNDS: u32 = 10;

#[derive(Clone)]
struct BomberEnemy {
    visual: Vec2,
    x: i8,
    y: i8,
    step: u8,
    kind: u8,
}
#[derive(Clone)]
struct WaterBomb {
    x: i8,
    y: i8,
    fuse: f32,
    range: i8,
}
#[derive(Clone)]
struct WaterBlast {
    x: i8,
    y: i8,
    life: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BomberPickupKind {
    Range,
    Capacity,
    Heart,
}

#[derive(Clone)]
struct BomberPickup {
    x: i8,
    y: i8,
    kind: BomberPickupKind,
}

struct BomberGame {
    visual_player: Vec2,
    invulnerable: f32,
    started: bool,
    player: (i8, i8),
    bricks: Vec<(i8, i8)>,
    bushes: Vec<(i8, i8)>,
    pots: Vec<(i8, i8)>,
    enemies: Vec<BomberEnemy>,
    bombs: Vec<WaterBomb>,
    blasts: Vec<WaterBlast>,
    pickups: Vec<BomberPickup>,
    bomb_capacity: usize,
    blast_range: i8,
    lives: u8,
    score: u32,
    round: u32,
    remaining: f32,
    paused: bool,
    won: bool,
    enemy_clock: f32,
    redraw: f32,
    feedback: String,
    failed: bool,
}

impl BomberGame {
    fn new() -> Self {
        let mut game = Self {
            player: (2, 8),
            visual_player: Vec2::new(2., 8.),
            invulnerable: 0.,
            started: false,
            bricks: vec![],
            enemies: vec![],
            bombs: vec![],
            blasts: vec![],
            pickups: vec![],
            bomb_capacity: 2,
            blast_range: 2,
            lives: 3,
            score: 0,
            round: 1,
            remaining: 156.,
            paused: false,
            won: false,
            bushes: vec![],
            pots: vec![],
            enemy_clock: 1.2,
            redraw: 0.,
            feedback: "击败小怪闯关！炸开花盆找强化水泡，连锁爆破有额外奖励。".into(),
            failed: false,
        };
        game.load_round();
        game
    }
    fn reset(&mut self) {
        *self = Self::new();
    }
    fn solid(&self, x: i8, y: i8) -> bool {
        !(0..BOMBER_COLUMNS).contains(&x)
            || !(0..BOMBER_ROWS).contains(&y)
            || (x % 2 == 1 && y % 2 == 1)
            || self.bricks.contains(&(x, y))
            || self.bushes.contains(&(x, y))
            || self.pots.contains(&(x, y))
    }
    fn load_round(&mut self) {
        self.bushes.clear();
        self.pots.clear();
        self.bricks.clear();
        for y in 0..BOMBER_ROWS {
            for x in 0..BOMBER_COLUMNS {
                let reserved = (x - 2).abs() + (y - 8).abs() <= 2
                    || (x - 15).abs() + y <= 2
                    || (x - 14).abs() + (y - 4).abs() <= 1
                    || (x - 16).abs() + (y - 8).abs() <= 1;
                let density = (2 + ((self.round.saturating_sub(1) / 3) as i16)).min(5);
                let scattered = (x as i16 * 7 + y as i16 * 11 + (self.round % 9) as i16 * 3)
                    .rem_euclid(9)
                    < density;
                if scattered && !reserved && !(x % 2 == 1 && y % 2 == 1) {
                    self.bricks.push((x, y));
                }
            }
        }
        self.bushes = [
            (5, 2),
            (6, 5),
            (2, 6),
            (11, 2),
            (12, 6),
            (8, 4),
            (8, 7),
            (3, 2),
            (14, 6),
            (9, 0),
        ]
        .into_iter()
        .take(6 + ((self.round.saturating_sub(1) / 2) as usize).min(4))
        .filter(|(x, y)| !self.solid(*x, *y) && (*x, *y) != (2, 8))
        .collect();
        self.pots = [
            (3, 4),
            (6, 7),
            (10, 4),
            (13, 5),
            (4, 1),
            (9, 5),
            (15, 6),
            (7, 2),
        ]
        .into_iter()
        .take(4 + ((self.round.saturating_sub(1) / 2) as usize).min(4))
        .filter(|(x, y)| !self.solid(*x, *y) && (*x, *y) != (2, 8))
        .collect();
        self.player = (2, 8);
        self.visual_player = Vec2::new(2., 8.);
        self.invulnerable = 2.;
        self.enemy_clock = 1.2;
        self.remaining = (156. - self.round.saturating_sub(1) as f32 * 5.).max(105.);
        self.paused = false;
        self.bombs.clear();
        self.blasts.clear();
        self.pickups.clear();
        let mut spawn_points: Vec<_> = (0..BOMBER_ROWS)
            .flat_map(|y| (0..BOMBER_COLUMNS).map(move |x| (x, y)))
            .filter(|(x, y)| {
                (x - self.player.0).abs() + (y - self.player.1).abs() >= 7 && !self.solid(*x, *y)
            })
            .collect();
        spawn_points.sort_by_key(|(x, y)| {
            (i32::from(*x) * 13 + i32::from(*y) * 7 + self.round as i32 * 11).rem_euclid(31)
        });
        self.enemies = spawn_points
            .into_iter()
            .take((6 + self.round as usize).min(14))
            .enumerate()
            .map(|(step, (x, y))| BomberEnemy {
                visual: Vec2::new(x as f32, y as f32),
                x,
                y,
                step: step as u8,
                kind: (step % 7) as u8,
            })
            .collect();
    }
    fn move_player(&mut self, dx: i8, dy: i8) {
        if self.failed || self.paused || self.won {
            return;
        }
        self.started = true;
        let next = (self.player.0 + dx, self.player.1 + dy);
        if !self.solid(next.0, next.1) && !self.bombs.iter().any(|bomb| (bomb.x, bomb.y) == next) {
            self.player = next;
            self.collect_pickup();
        }
    }
    fn collect_pickup(&mut self) {
        let Some(index) = self
            .pickups
            .iter()
            .position(|pickup| (pickup.x, pickup.y) == self.player)
        else {
            return;
        };
        let pickup = self.pickups.remove(index);
        match pickup.kind {
            BomberPickupKind::Range if self.blast_range < 4 => {
                self.blast_range += 1;
                self.feedback = format!("爆破范围提升到 {} 格！", self.blast_range);
            }
            BomberPickupKind::Capacity if self.bomb_capacity < 4 => {
                self.bomb_capacity += 1;
                self.feedback = format!("水泡容量提升到 {} 颗！", self.bomb_capacity);
            }
            BomberPickupKind::Heart if self.lives < 3 => {
                self.lives += 1;
                self.feedback = "捡到爱心水泡，恢复 1 条命！".into();
            }
            _ => {
                self.score += 100;
                self.feedback = "道具已满，转化为 100 分！".into();
            }
        }
    }
    fn place_bomb(&mut self) {
        if self.failed
            || self.paused
            || self.won
            || self.bombs.len() >= self.bomb_capacity
            || self
                .bombs
                .iter()
                .any(|bomb| (bomb.x, bomb.y) == self.player)
        {
            return;
        }
        self.started = true;
        self.bombs.push(WaterBomb {
            x: self.player.0,
            y: self.player.1,
            fuse: 2.2,
            range: self.blast_range,
        });
        self.feedback = "水泡 2 秒后爆开！引爆另一颗水泡可触发连锁爆破。".into();
    }
    fn blast_cells(&self, bomb: &WaterBomb) -> Vec<(i8, i8)> {
        let mut cells = vec![(bomb.x, bomb.y)];
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for distance in 1..=bomb.range {
                let point = (bomb.x + dx * distance, bomb.y + dy * distance);
                if !(0..BOMBER_COLUMNS).contains(&point.0)
                    || !(0..BOMBER_ROWS).contains(&point.1)
                    || (point.0 % 2 == 1 && point.1 % 2 == 1)
                {
                    break;
                }
                cells.push(point);
                if self.bricks.contains(&point)
                    || self.pots.contains(&point)
                    || self.bushes.contains(&point)
                {
                    break;
                }
            }
        }
        cells
    }
    fn explode(&mut self, bomb: WaterBomb) {
        let cells = self.blast_cells(&bomb);
        // Blasts can trigger nearby bombs, setting up quick chain reactions.
        for chained in &mut self.bombs {
            if cells.contains(&(chained.x, chained.y)) {
                chained.fuse = 0.;
            }
        }
        let destroyed: Vec<_> = cells
            .iter()
            .copied()
            .filter(|cell| self.bricks.contains(cell))
            .collect();
        self.bricks.retain(|brick| !destroyed.contains(brick));
        let destroyed_pots: Vec<_> = cells
            .iter()
            .copied()
            .filter(|cell| self.pots.contains(cell))
            .collect();
        let pots_destroyed = destroyed_pots.len();
        self.pots.retain(|pot| !cells.contains(pot));
        for (x, y) in destroyed_pots {
            if (i16::from(x) * 3 + i16::from(y) + self.round as i16).rem_euclid(2) == 0 {
                let kind = match self.round % 3 {
                    1 => BomberPickupKind::Range,
                    2 => BomberPickupKind::Capacity,
                    _ => BomberPickupKind::Heart,
                };
                self.pickups.push(BomberPickup { x, y, kind });
            }
        }
        let defeated: Vec<_> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(index, enemy)| cells.contains(&(enemy.x, enemy.y)).then_some(index))
            .collect();
        for index in defeated.iter().rev() {
            self.enemies.remove(*index);
        }
        self.score +=
            destroyed.len() as u32 * 8 + pots_destroyed as u32 * 20 + defeated.len() as u32 * 60;
        self.blasts.extend(
            cells
                .into_iter()
                .map(|(x, y)| WaterBlast { x, y, life: 0.46 }),
        );
        if !defeated.is_empty() {
            self.feedback = format!("水花击中 {} 个小怪！", defeated.len());
        }
    }
    fn lose_life(&mut self) {
        if self.invulnerable > 0. || self.failed {
            return;
        }
        self.invulnerable = 2.5;
        self.lives = self.lives.saturating_sub(1);
        self.player = (2, 8);
        self.visual_player = Vec2::new(2., 8.);
        if self.lives == 0 {
            self.failed = true;
            self.feedback = "被水柱卷走啦！重新挑战吧。".into();
        } else {
            self.feedback = format!("小心水柱！还剩 {} 条命。", self.lives);
        }
    }
    fn advance(&mut self, dt: f32) -> bool {
        if self.failed || self.paused || self.won || !self.started {
            return false;
        }
        let dt = dt.min(0.05);
        self.remaining = (self.remaining - dt).max(0.);
        if self.remaining == 0. {
            self.failed = true;
            self.feedback = "时间到！再试一次，争取拿到三星。".into();
            return true;
        }
        self.invulnerable = (self.invulnerable - dt).max(0.);
        let target = Vec2::new(self.player.0 as f32, self.player.1 as f32);
        self.visual_player = self.visual_player.lerp(target, (dt * 15.).min(1.));
        for enemy in &mut self.enemies {
            enemy.visual = enemy
                .visual
                .lerp(Vec2::new(enemy.x as f32, enemy.y as f32), (dt * 9.).min(1.));
        }
        for bomb in &mut self.bombs {
            bomb.fuse -= dt;
        }
        let mut detonations = 0;
        while let Some(index) = self.bombs.iter().position(|bomb| bomb.fuse <= 0.) {
            let bomb = self.bombs.remove(index);
            self.explode(bomb);
            detonations += 1;
        }
        if detonations > 1 {
            let bonus = (detonations as u32 - 1) * 100;
            self.score += bonus;
            self.feedback = format!("连锁爆破 ×{}！额外奖励 +{} 分", detonations, bonus);
        }
        self.collect_pickup();
        for blast in &mut self.blasts {
            blast.life -= dt;
        }
        self.blasts.retain(|blast| blast.life > 0.);
        if self
            .blasts
            .iter()
            .any(|blast| (blast.x, blast.y) == self.player)
        {
            self.lose_life();
        }
        self.enemy_clock -= dt;
        if self.enemy_clock <= 0. {
            self.enemy_clock = (1.15 - self.round as f32 * 0.065).max(0.48);
            let bricks = self.bricks.clone();
            let bushes = self.bushes.clone();
            let pots = self.pots.clone();
            let bombs: Vec<_> = self.bombs.iter().map(|bomb| (bomb.x, bomb.y)).collect();
            for enemy in &mut self.enemies {
                let dx = (self.player.0 - enemy.x).signum();
                let dy = (self.player.1 - enemy.y).signum();
                let options = if enemy.step % 2 == 0 {
                    [(dx, 0), (0, dy)]
                } else {
                    [(0, dy), (dx, 0)]
                };
                for (mx, my) in options {
                    let point = (enemy.x + mx, enemy.y + my);
                    if (mx != 0 || my != 0)
                        && (0..BOMBER_COLUMNS).contains(&point.0)
                        && (0..BOMBER_ROWS).contains(&point.1)
                        && !(point.0 % 2 == 1 && point.1 % 2 == 1)
                        && !bricks.contains(&point)
                        && !bushes.contains(&point)
                        && !pots.contains(&point)
                        && !bombs.contains(&point)
                    {
                        enemy.x = point.0;
                        enemy.y = point.1;
                        break;
                    }
                }
                enemy.step = enemy.step.wrapping_add(1);
            }
        }
        if self
            .enemies
            .iter()
            .any(|enemy| (enemy.x, enemy.y) == self.player)
        {
            self.lose_life();
        }
        if self.enemies.is_empty() && !self.failed {
            if self.round >= BOMBER_MAX_ROUNDS {
                self.won = true;
                self.feedback = "太棒了！三星闯关成功！".into();
            } else {
                self.round += 1;
                self.load_round();
                self.feedback = format!("第 {} 关，迷宫里的小怪更多了！", self.round);
            }
        }
        self.redraw += dt;
        if self.redraw >= 1. / 60. {
            self.redraw = 0.;
            true
        } else {
            detonations > 0
        }
    }
}

#[derive(Clone)]
struct Fruit {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    kind: u8,
    bomb: bool,
}

#[derive(Clone)]
struct FruitSlice {
    x: f32,
    y: f32,
    kind: u8,
    life: f32,
    /// The two halves fly apart perpendicular to the player's stroke. Keeping
    /// this with the effect makes the cut readable even after the fruit body
    /// has been removed from the board.
    split: Vec2,
    /// The swipe direction itself is drawn as a dotted blade trail.
    slash: Vec2,
    /// Only the first fruit in a stroke carries the floating score label.
    score: u32,
}

struct FruitGame {
    fruits: Vec<Fruit>,
    slices: Vec<FruitSlice>,
    score: u32,
    combo: u32,
    best_combo: u32,
    combo_clock: f32,
    lives: u8,
    spawn_clock: f32,
    wave: u32,
    stage: u32,
    stage_progress: u32,
    stage_target: u32,
    redraw: f32,
    hit_flash: f32,
    feedback: String,
    failed: bool,
}

impl FruitGame {
    fn new() -> Self {
        Self {
            fruits: vec![],
            slices: vec![],
            score: 0,
            combo: 0,
            best_combo: 0,
            combo_clock: 0.,
            lives: 3,
            spawn_clock: 0.15,
            wave: 0,
            stage: 1,
            stage_progress: 0,
            stage_target: 8,
            redraw: 0.,
            hit_flash: 0.,
            feedback: "划过飞起来的水果！漏切不扣命，注意避开炸弹。".into(),
            failed: false,
        }
    }
    fn reset(&mut self) {
        *self = Self::new();
    }
    fn spawn(&mut self) {
        // Fruit launches in readable fans, so one smooth stroke can make a
        // satisfying multi-cut instead of asking the player to chase dots.
        let fever = self.wave > 0 && self.wave % 6 == 5;
        // Larger, staggered fans make each launch feel like a juicy wave,
        // while keeping enough horizontal spread for deliberate swipes.
        let count = if fever {
            7
        } else {
            4 + (self.wave % 3) as usize
        };
        let center = 0.28 + ((self.wave * 19) % 42) as f32 / 100.;
        for index in 0..count {
            let seed = self.wave * 31 + index as u32 * 17;
            let offset = index as f32 - (count.saturating_sub(1) as f32 / 2.);
            const FRUIT_KINDS: [u8; 9] = [0, 1, 2, 3, 4, 6, 7, 8, 9];
            self.fruits.push(Fruit {
                x: (center + offset * 0.072).clamp(0.08, 0.92),
                y: 1.08 + (index as f32 * 0.025),
                vx: offset * 0.048 + ((seed % 5) as f32 - 2.) * 0.010,
                vy: -0.98 - (seed % 10) as f32 * 0.015,
                // The gold fruit appears in every sixth launch and gives a
                // small, obvious reason to vary a stroke's path.
                kind: if fever && index == count / 2 {
                    5
                } else {
                    FRUIT_KINDS[(seed as usize + self.wave as usize) % FRUIT_KINDS.len()]
                },
                // Bombs enter only after the player has had several safe
                // launches, and never interrupt a gold-fruit fever launch.
                bomb: !fever && self.wave >= 8 && index + 1 == count && self.wave % 4 == 0,
            });
        }
        self.wave += 1;
        if fever {
            self.feedback = "金色连斩波！一笔切掉整串水果。".into();
        }
    }
    fn lose_life(&mut self, message: &str) {
        self.lives = self.lives.saturating_sub(1);
        self.combo = 0;
        if self.lives == 0 {
            self.failed = true;
            self.feedback = "忍者被炸到了！点重新开始。".into();
        } else {
            self.feedback = format!("{} 还剩 {} 颗爱心。", message, self.lives);
        }
    }
    fn slice_between(&mut self, from: Vec2, to: Vec2) {
        if self.failed {
            return;
        }
        let mut cut = 0;
        let mut gold = 0;
        let mut rare_bonus = 0;
        let mut bomb = false;
        let mut slices = vec![];
        let stroke = to - from;
        let split = if stroke.length_squared() > 0.000_1 {
            Vec2::new(-stroke.y, stroke.x).normalize()
        } else {
            Vec2::Y
        };
        let slash = if stroke.length_squared() > 0.000_1 {
            stroke.normalize()
        } else {
            Vec2::X
        };
        self.fruits.retain(|fruit| {
            let point = Vec2::new(fruit.x, fruit.y);
            let t = if stroke.length_squared() > 0. {
                ((point - from).dot(stroke) / stroke.length_squared()).clamp(0., 1.)
            } else {
                0.
            };
            let distance = point.distance(from + stroke * t);
            // Fruit should feel generous to hit. Bombs deliberately require a
            // more central strike so a line that merely brushes past one does
            // not turn a successful cut into an unfair loss.
            let hit_radius = if fruit.bomb { 0.045 } else { 0.095 };
            if distance < hit_radius {
                if fruit.bomb {
                    bomb = true;
                } else {
                    cut += 1;
                    gold += u32::from(fruit.kind == 5);
                    rare_bonus += match fruit.kind {
                        6 => 4, // strawberry
                        7 => 5, // banana bunch
                        8 => 6, // peach
                        9 => 8, // pineapple
                        _ => 0,
                    };
                    slices.push(FruitSlice {
                        x: fruit.x,
                        y: fruit.y,
                        kind: fruit.kind,
                        // A slightly longer effect gives the player time to
                        // register a successful cut without slowing the game.
                        life: 0.82,
                        split,
                        slash,
                        score: 0,
                    });
                }
                false
            } else {
                true
            }
        });
        if bomb {
            self.lose_life("切到了炸弹！");
        }
        if cut > 0 && !bomb {
            self.combo = if self.combo_clock > 0. {
                self.combo + cut
            } else {
                cut
            };
            self.combo_clock = 1.15;
            self.best_combo = self.best_combo.max(self.combo);
            let points = cut * 12 + self.combo.min(15) * 3 + gold * 25 + rare_bonus;
            self.score += points;
            self.stage_progress += cut as u32;
            self.hit_flash = 0.16;
            if let Some(effect) = slices.first_mut() {
                effect.score = points;
            }
            self.feedback = if gold > 0 {
                format!("金色加分！+{} · {} 连击", points, self.combo)
            } else if cut >= 2 {
                format!("一刀 {} 个！+{} · {} 连击", cut, points, self.combo)
            } else {
                format!("漂亮！+{} · {} 连击", points, self.combo)
            };
            if self.stage_progress >= self.stage_target {
                let bonus = self.stage * 100;
                self.score += bonus;
                self.stage += 1;
                self.stage_progress = 0;
                self.stage_target = 8 + (self.stage - 1) * 3;
                self.feedback =
                    format!("挑战完成！额外奖励 +{} 分，进入第 {} 轮", bonus, self.stage);
            }
        }
        self.slices.extend(slices);
    }
    fn advance(&mut self, dt: f32) -> bool {
        if self.failed {
            return false;
        }
        let mut changed = false;
        self.combo_clock = (self.combo_clock - dt).max(0.);
        if self.combo > 0 && self.combo_clock == 0. {
            self.combo = 0;
            changed = true;
        }
        for slice in &mut self.slices {
            slice.life -= dt;
        }
        self.slices.retain(|slice| slice.life > 0.);
        self.hit_flash = (self.hit_flash - dt).max(0.);
        self.spawn_clock -= dt;
        if self.spawn_clock <= 0. {
            self.spawn();
            self.spawn_clock = (1.18 - self.wave as f32 * 0.018).max(0.68);
            changed = true;
        }
        for fruit in &mut self.fruits {
            fruit.x += fruit.vx * dt;
            fruit.y += fruit.vy * dt;
            fruit.vy += dt * 0.78;
        }
        self.fruits
            .retain(|fruit| fruit.y <= 1.28 && fruit.x >= -0.10 && fruit.x <= 1.10);
        self.redraw += dt;
        if self.redraw >= 0.04 {
            self.redraw = 0.;
            true
        } else {
            changed || !self.slices.is_empty() || self.hit_flash > 0.
        }
    }
}

#[derive(Resource)]
struct Session {
    mode: GameMode,
    help_open: bool,
    math_tools: bool,
    library_open: bool,
    magnet_modes_open: bool,
    ball: BallGame,
    tetris: TetrisGame,
    math: crate::math::MathGame,
    literacy: crate::literacy::Literacy,
    piano: PianoGame,
    assault: AssaultGame,
    platform: PlatformGame,
    tank: TankGame,
    sky: SkyGame,
    bubble: BubbleGame,
    bomber: BomberGame,
    fruit: FruitGame,
    fortune: FortuneGame,
    slots: SlotsGame,
    arcade: ArcadeGame,
    fishing: FishingGame,
    honey: HoneyGame,
    moon: MoonFestivalGame,
    snake: SnakeGame,
    minesweeper: MinesweeperGame,
    classic_2048: Classic2048Game,
    klotski: KlotskiGame,
    sokoban: SokobanGame,
    sudoku: SudokuGame,
    breakout: BreakoutGame,
    elements: ElementGame,
    light_path: LightPathGame,
    garden: GardenGame,
    courier: CourierGame,
    mission_step: usize,
    mission_done: bool,
    mission_wrong: Option<usize>,
    cheer: f32,
    mission_stars: [usize; 4],
    mission_feedback: String,
    save: Save,
    cloud_bridge_campaign: CloudBridgeCampaign,
    work: Work,
    runner: Option<Runner>,
    material: Material,
    erase: bool,
    build: bool,
    island_3d_view: bool,
    island_menu_open: bool,
    editor_open: bool,
    undo: Vec<game::World>,
    message: String,
    ui_dirty: bool,
    world_dirty: bool,
    clock: f32,
    save_path: PathBuf,
    yaw: f32,
    pitch: f32,
    zoom: f32,
}

#[derive(Clone, Serialize, Deserialize)]
struct CloudBridgeCampaign {
    completed: Vec<usize>,
    stars: [u8; 6],
    unlocked: usize,
}

impl Default for CloudBridgeCampaign {
    fn default() -> Self {
        Self {
            completed: vec![],
            stars: [0; 6],
            // The cloud-bridge game opens at the fourth island, as shown in
            // its approved screen. Earlier islands remain replayable.
            unlocked: 3,
        }
    }
}

impl CloudBridgeCampaign {
    fn load() -> Self {
        fs::read(save_directory().join("cloudbridge-v1.json"))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Self>(&bytes).ok())
            .filter(|campaign| {
                campaign.unlocked < 6
                    && campaign.completed.iter().all(|level| *level < 6)
                    && campaign.stars.iter().all(|stars| *stars <= 3)
            })
            .unwrap_or_default()
    }

    fn record(&mut self, level: usize, stars: u8) {
        if level >= 6 {
            return;
        }
        if !self.completed.contains(&level) {
            self.completed.push(level);
        }
        self.stars[level] = self.stars[level].max(stars);
        self.unlocked = self.unlocked.max((level + 1).min(5));
        let path = save_directory().join("cloudbridge-v1.json");
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(bytes) = serde_json::to_vec_pretty(self) {
            let temp = path.with_extension("tmp");
            if fs::write(&temp, bytes).is_ok() {
                let _ = fs::rename(temp, path);
            }
        }
    }
}

impl Session {
    fn new() -> Self {
        let mut save_path = save_directory().join("island-v1.json");
        let (save, warning) = match fs::read(&save_path) {
            Ok(bytes) => match serde_json::from_slice::<Save>(&bytes) {
                Ok(mut save) if save.validate() => {
                    // Move a never-played first profile into the redesigned
                    // bridge-building challenge, while keeping any real player
                    // progress and custom construction untouched.
                    let untouched_first_profile = save.profiles.get(&0).is_some_and(|work| {
                        work.level == 0
                            && work.completed.is_empty()
                            && work.stars == [0; 6]
                            && work.world == game::level(0)
                            && work.program == [Instruction::Forward, Instruction::Forward]
                            && work.repeat == 1
                            && !work.finish_forward
                    });
                    if untouched_first_profile {
                        let mut work = save.profiles[&0].clone();
                        work.choose_level(3);
                        work.repeat = 5;
                        save.profiles.insert(0, work);
                    }
                    (save, None)
                }
                _ => {
                    // A damaged/unsupported save must never be overwritten by autosave.
                    let stamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis();
                    let backup = save_path.with_extension(format!("unreadable-{stamp}.json"));
                    if fs::rename(&save_path, backup).is_err() {
                        save_path = save_path.with_file_name(format!("recovered-{stamp}.json"));
                    }
                    (
                        Save::default(),
                        Some("存档无法读取。原文件已保留，本次先进入新小岛。"),
                    )
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Save::default(), None),
            Err(_) => {
                save_path = save_path.with_file_name("island-recovery.json");
                (
                    Save::default(),
                    Some("暂时无法读取本地存档，本次另存恢复文件。"),
                )
            }
        };
        let work = save.profiles[&save.active].clone();
        let cloud_bridge_campaign = CloudBridgeCampaign::load();
        let mut session = Self {
            mode: GameMode::Island,
            help_open: false,
            math_tools: false,
            library_open: std::env::var_os("MAGNET_START_LIBRARY").is_some(),
            magnet_modes_open: false,
            ball: BallGame::new(),
            tetris: load_tetris(),
            math: crate::math::MathGame::default(),
            literacy: crate::literacy::Literacy::default(),
            piano: PianoGame::new(),
            assault: AssaultGame::new(),
            platform: PlatformGame::new(),
            tank: TankGame::new(),
            sky: SkyGame::new(),
            bubble: BubbleGame::new(),
            bomber: BomberGame::new(),
            fruit: FruitGame::new(),
            fortune: FortuneGame::new(),
            slots: SlotsGame::new(),
            arcade: ArcadeGame::new(),
            fishing: FishingGame::new(),
            honey: HoneyGame::new(),
            moon: MoonFestivalGame::new(),
            snake: SnakeGame::new(),
            minesweeper: MinesweeperGame::new(),
            classic_2048: Classic2048Game::new(),
            klotski: KlotskiGame::new(),
            sokoban: SokobanGame::new(),
            sudoku: load_sudoku(),
            breakout: BreakoutGame::new(),
            elements: load_elements(),
            light_path: LightPathGame::new(),
            garden: GardenGame::new(),
            courier: CourierGame::load(),
            mission_step: 0,
            mission_done: false,
            mission_wrong: None,
            cheer: 0.,
            mission_stars: [0; 4],
            mission_feedback: String::new(),
            message: warning.unwrap_or(game::LEVEL_HINTS[work.level]).to_owned(),
            save,
            cloud_bridge_campaign,
            work,
            runner: None,
            material: Material::Wood,
            erase: false,
            build: true,
            island_3d_view: false,
            island_menu_open: false,
            editor_open: std::env::var_os("MAGNET_CAPTURE").is_some()
                && std::env::var("MAGNET_PREVIEW_MODE").as_deref() == Ok("code"),
            undo: vec![],
            ui_dirty: true,
            world_dirty: true,
            clock: 0.,
            save_path,
            yaw: 0.68,
            pitch: 0.762,
            zoom: 16.0,
        };
        session.island_3d_view = !island_base_intact(&session.work);
        if let Some(mode) = match std::env::var("MAGNET_START_GAME").as_deref() {
            Ok("fire") => Some(GameMode::Firefighters),
            Ok("detective") => Some(GameMode::Detective),
            Ok("farm") => Some(GameMode::SpaceFarm),
            Ok("cloudbridge") => Some(GameMode::CloudBridge),
            Ok("restaurant") => Some(GameMode::CrystalGarden),
            _ => None,
        } {
            select_game(&mut session, mode);
        }
        if session.mode == GameMode::CloudBridge
            && std::env::var_os("MAGNET_CAPTURE").is_some()
            && std::env::var_os("MAGNET_CLOUDBRIDGE_DEMO").is_some()
        {
            for x in -2..=2 {
                session.work.world.put([x, 0, 0], Material::Wood);
            }
            session.world_dirty = true;
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("ball") {
            select_game(&mut session, GameMode::Ball2048);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("breakout") {
            select_game(&mut session, GameMode::Breakout);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("garden") {
            select_game(&mut session, GameMode::GardenDefense);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("elements") {
            select_game(&mut session, GameMode::ElementLab);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("lightpath") {
            select_game(&mut session, GameMode::LightPath);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("tetris") {
            select_game(&mut session, GameMode::Tetris);
        }
        if let Some(mode) = match std::env::var("MAGNET_START_GAME").as_deref() {
            Ok("snake") => Some(GameMode::Snake),
            Ok("mines") => Some(GameMode::Minesweeper),
            Ok("classic2048") => Some(GameMode::Classic2048),
            Ok("klotski") => Some(GameMode::Klotski),
            Ok("sokoban") => Some(GameMode::Sokoban),
            Ok("sudoku") => Some(GameMode::Sudoku),
            _ => None,
        } {
            select_game(&mut session, mode);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("math") {
            select_game(&mut session, GameMode::Math);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("arcade") {
            select_game(&mut session, GameMode::FruitArcade);
            if std::env::var_os("MAGNET_CAPTURE").is_some()
                && std::env::var_os("MAGNET_ARCADE_DEMO").is_some()
            {
                for i in 0..5 {
                    session.arcade.add(i);
                }
                session.arcade.begin(4);
            }
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("slots") {
            select_game(&mut session, GameMode::Slots);
            if std::env::var_os("MAGNET_CAPTURE").is_some() {
                if let Ok(scene) = std::env::var("MAGNET_SLOTS_SCENE") {
                    session.slots.preview(&scene);
                }
            }
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("fortune") {
            select_game(&mut session, GameMode::Fortune);
            if std::env::var_os("MAGNET_CAPTURE").is_some() {
                if let Ok(scene) = std::env::var("MAGNET_FORTUNE_SCENE") {
                    session.fortune.prepare_preview(&scene);
                }
            }
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("piano") {
            select_game(&mut session, GameMode::MusicFactory);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("shooter") {
            select_game(&mut session, GameMode::PixelAssault);
        }
        if matches!(
            std::env::var("MAGNET_START_GAME").as_deref(),
            Ok("mushroom") | Ok("platform")
        ) {
            select_game(&mut session, GameMode::MushroomRun);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("tank") {
            select_game(&mut session, GameMode::TankBattle);
            if let Ok(map) = std::env::var("MAGNET_TANK_MAP")
                .unwrap_or_default()
                .parse::<u8>()
            {
                session.tank.wave = u32::from(map % TANK_MAP_COUNT) + 1;
                session.tank.load_map(map);
                session.tank.spawn_wave();
            }
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("sky") {
            select_game(&mut session, GameMode::SkyRaid);
            if std::env::var_os("MAGNET_SKY_BOSS").is_some() {
                session.sky.score = session.sky.next_boss_score;
                session.sky.maybe_spawn_boss();
            }
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("bubble") {
            select_game(&mut session, GameMode::BubbleIsland);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("bomber") {
            select_game(&mut session, GameMode::BubbleBomber);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("fruit") {
            select_game(&mut session, GameMode::FruitNinja);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("fishing") {
            select_game(&mut session, GameMode::Fishing);
        }
        if matches!(
            std::env::var("MAGNET_START_GAME").as_deref(),
            Ok("honey") | Ok("bear")
        ) {
            select_game(&mut session, GameMode::HoneyHunt);
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("courier") {
            select_game(&mut session, GameMode::SpaceCourier);
        }
        if matches!(
            std::env::var("MAGNET_START_GAME").as_deref(),
            Ok("moon") | Ok("bobing")
        ) {
            select_game(&mut session, GameMode::MoonFestival);
        }
        if matches!(
            std::env::var("MAGNET_START_GAME").as_deref(),
            Ok("build") | Ok("freebuild")
        ) {
            select_game(&mut session, GameMode::FreeBuild);
        }
        if let Ok(bytes) = fs::read(save_directory().join("literacy-v1.json")) {
            if let Ok(progress) = serde_json::from_slice(&bytes) {
                session.literacy.progress = progress;
            }
        }
        if let Ok(bytes) = fs::read(save_directory().join("math-v1.json")) {
            if let Ok(math) = serde_json::from_slice(&bytes) {
                session.math = math;
            }
        }
        if let Ok(bytes) = fs::read(save_directory().join("ball-v1.json")) {
            if let Ok(ball) = serde_json::from_slice::<BallGame>(&bytes) {
                if ball.rules_version == 3 {
                    session.ball = ball;
                } else {
                    let number = ball.number.max(2);
                    let score = ball.score;
                    session.ball.reset();
                    session.ball.number = number;
                    session.ball.score = score;
                }
            }
        }
        if session.mode == GameMode::Ball2048 {
            session.ball.running = true;
        }
        if std::env::var("MAGNET_START_GAME").as_deref() == Ok("literacy") {
            select_game(&mut session, GameMode::Literacy);
            if std::env::var_os("MAGNET_CAPTURE").is_some()
                && std::env::var_os("MAGNET_LITERACY_LESSON").is_some()
            {
                session.literacy.start(0);
                session.literacy.stage = 2;
            }
        }
        if std::env::var_os("MAGNET_CAPTURE").is_some()
            && std::env::var_os("MAGNET_PREVIEW_SUCCESS").is_some()
        {
            if mission_index(session.mode).is_some() {
                session.mission_step = std::env::var("MAGNET_PREVIEW_SUCCESS")
                    .ok()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(1)
                    .min(3);
                session.mission_done = true;
                session.mission_feedback = mission_question(session.mode, session.mission_step)
                    .success
                    .into();
            }
            if session.mode == GameMode::Math {
                let answer = session.math.question().answer;
                session.math.placed = answer;
                session.math.answer(answer);
            }
            if session.mode == GameMode::Literacy {
                session.literacy.start(0);
                session.literacy.next();
                session.literacy.selected = Some(0);
                session.literacy.check();
            }
        }
        if std::env::var_os("MAGNET_CAPTURE").is_some() {
            if let Ok(step) = std::env::var("MAGNET_PREVIEW_MISSION_STEP") {
                if mission_index(session.mode).is_some() {
                    session.mission_step = step.parse::<usize>().unwrap_or(0).min(3);
                    session.mission_done = false;
                    session.mission_feedback.clear();
                }
            }
        }
        if std::env::var_os("MAGNET_CAPTURE").is_some()
            && std::env::var_os("MAGNET_PREVIEW_HELP").is_some()
        {
            session.help_open = true;
        }
        session
    }
    fn persist(&mut self) {
        if self.mode == GameMode::CrystalGarden {
            let path = save_directory().join("restaurant-v1.json");
            let result = (|| -> Result<(), Box<dyn std::error::Error>> {
                fs::create_dir_all(save_directory())?;
                let bytes = serde_json::to_vec_pretty(&self.work)?;
                let temp = path.with_extension("tmp");
                fs::write(&temp, bytes)?;
                fs::rename(temp, path)?;
                Ok(())
            })();
            if let Err(error) = result {
                self.message = format!("餐厅暂未保存：{error}");
            }
            return;
        }

        if !matches!(self.mode, GameMode::Island | GameMode::FreeBuild) {
            return;
        }
        // Save the editable starting world, never a transient half-executed program.
        let mut work = self.work.clone();
        if let Some(r) = &self.runner {
            work.world = r.before.clone();
        }
        if self.mode == GameMode::FreeBuild {
            self.save.free_builds.insert(self.save.active, work.world);
        } else {
            self.save.profiles.insert(self.save.active, work);
        }
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            fs::create_dir_all(self.save_path.parent().unwrap())?;
            let bytes = serde_json::to_vec_pretty(&self.save)?;
            let temp = self.save_path.with_extension("tmp");
            fs::write(&temp, bytes)?;
            fs::rename(temp, &self.save_path)?;
            Ok(())
        })();
        if let Err(e) = result {
            self.message = format!("保存失败，作品仍在当前游戏中：{e}");
        }
    }
    fn persist_sidecars(&self) {
        let dir = save_directory();
        let _ = fs::create_dir_all(&dir);
        if let Ok(bytes) = serde_json::to_vec_pretty(&self.math) {
            let _ = fs::write(dir.join("math-v1.json"), bytes);
        }
        if let Ok(bytes) = serde_json::to_vec_pretty(&self.literacy.progress) {
            let _ = fs::write(dir.join("literacy-v1.json"), bytes);
        }
        if let Ok(bytes) = serde_json::to_vec_pretty(&self.ball) {
            let _ = fs::write(dir.join("ball-v1.json"), bytes);
        }
    }
    fn persist_tetris(&self) {
        let path = save_directory().join("tetris-v1.json");
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            fs::create_dir_all(path.parent().ok_or("missing save directory")?)?;
            let bytes = serde_json::to_vec(&TetrisSave::from_game(&self.tetris))?;
            let temp = path.with_extension("tmp");
            fs::write(&temp, bytes)?;
            fs::rename(temp, path)?;
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("Could not save Tetris: {error}");
        }
    }
    fn persist_elements(&self) {
        let path = save_directory().join("elements-v1.json");
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            fs::create_dir_all(path.parent().ok_or("missing save directory")?)?;
            let bytes = serde_json::to_vec(&ElementSave::from_game(&self.elements))?;
            let temp = path.with_extension("tmp");
            fs::write(&temp, bytes)?;
            fs::rename(temp, path)?;
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("Could not save elements game: {error}");
        }
    }
    fn persist_sudoku(&self) {
        let path = save_directory().join("sudoku-v1.json");
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            fs::create_dir_all(path.parent().ok_or("missing save directory")?)?;
            let bytes = serde_json::to_vec(&SudokuSave::from_game(&self.sudoku))?;
            let temp = path.with_extension("tmp");
            fs::write(&temp, bytes)?;
            fs::rename(temp, path)?;
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("Could not save Sudoku: {error}");
        }
    }
    fn reset_run(&mut self) {
        if let Some(r) = self.runner.take() {
            self.work.world = r.before;
            self.world_dirty = true;
        }
        self.clock = 0.;
    }
    fn edit_program(&mut self) {
        self.reset_run();
        self.ui_dirty = true;
    }
    fn level_complete(&self) -> bool {
        if self.mode == GameMode::CloudBridge && self.work.level == 5 {
            let (blocks, materials) = self.work.creative_progress();
            blocks >= 10 && materials >= 3
        } else {
            self.work.world.won()
        }
    }
    fn step(&mut self) {
        if self.work.program.is_empty() {
            self.message = "先添加一张指令卡片吧".into();
            return;
        }
        let (planned_steps, result) = {
            let runner = self.runner.get_or_insert_with(|| Runner::new(&self.work));
            let planned_steps = runner.ops.len();
            (planned_steps, runner.step(&mut self.work.world))
        };
        match result {
            Ok(done) => {
                self.world_dirty = true;
                if self.level_complete() {
                    let level = self.work.level;
                    let earned = if level == 5 || planned_steps <= game::LEVEL_STAR_PAR[level] {
                        3
                    } else {
                        2
                    };
                    self.work.stars[level] = self.work.stars[level].max(earned);
                    self.message = format!(
                        "任务完成！获得 {} 颗星，{}。",
                        earned,
                        if self.work.level < 5 {
                            "下一座岛正在等你"
                        } else {
                            "你的方块家园又变漂亮了"
                        }
                    );
                    if !self.work.completed.contains(&level) {
                        self.work.completed.push(level);
                    }
                    if self.mode == GameMode::CloudBridge {
                        self.cloud_bridge_campaign.record(level, earned);
                        self.work.completed = self.cloud_bridge_campaign.completed.clone();
                        self.work.stars = self.cloud_bridge_campaign.stars;
                    }
                    self.persist();
                } else if done {
                    self.message =
                        "指令执行完了。看看和小旗还差几步？可以点「示范路线」看看做法。".into();
                } else {
                    let runner = self
                        .runner
                        .as_ref()
                        .expect("runner stays available after a successful step");
                    self.message = format!(
                        "正在执行：{} · 第 {} / {} 步",
                        runner.ops[runner.pc - 1].label(),
                        runner.pc,
                        runner.ops.len()
                    );
                }
            }
            Err(e) => {
                self.reset_run();
                self.editor_open = true;
                self.build = false;
                self.message = format!("{e} 已自动回到起点，改一张卡片再试，或点「示范路线」。");
            }
        }
        self.ui_dirty = true;
    }
}

fn island_base_intact(work: &Work) -> bool {
    game::level(work.level)
        .blocks
        .iter()
        .filter(|block| block.pos[1] == 0 && block.material == Material::Grass)
        .all(|block| work.world.material(block.pos) == Some(Material::Grass))
}

fn free_build_base_intact(work: &Work) -> bool {
    game::free_build_world()
        .blocks
        .iter()
        .all(|block| work.world.material(block.pos) == Some(Material::Grass))
}

fn select_game(s: &mut Session, mode: GameMode) {
    s.help_open = false;
    s.magnet_modes_open = false;
    s.cheer = 0.;
    if s.mode != mode && mission_index(mode).is_some() {
        s.mission_step = 0;
        s.mission_done = false;
        s.mission_wrong = None;
        s.mission_feedback.clear();
    }
    s.reset_run();
    s.mode = mode;
    s.library_open = false;
    s.runner = None;
    s.undo.clear();
    s.erase = false;
    s.editor_open = false;
    s.yaw = 0.68;
    s.pitch = 0.762;
    s.zoom = if mode == GameMode::CloudBridge {
        12.0
    } else {
        16.0
    };
    s.island_3d_view = false;
    if mode == GameMode::Island {
        s.work = s.save.profiles[&s.save.active].clone();
        s.island_3d_view = !island_base_intact(&s.work);
        s.build = true;
        s.message = game::LEVEL_HINTS[s.work.level].into();
    } else {
        s.work = match mode {
            GameMode::CloudBridge => {
                let mut work = Work::default();
                work.choose_level(s.cloud_bridge_campaign.unlocked.min(5));
                work.completed = s.cloud_bridge_campaign.completed.clone();
                work.stars = s.cloud_bridge_campaign.stars;
                work
            }
            GameMode::CrystalGarden => load_restaurant_work(),
            GameMode::FreeBuild => Work {
                world: s
                    .save
                    .free_builds
                    .get(&s.save.active)
                    .cloned()
                    .unwrap_or_else(game::free_build_world),
                program: vec![],
                repeat: 1,
                finish_forward: false,
                level: 5,
                completed: vec![],
                stars: [0; 6],
            },
            GameMode::Ball2048
            | GameMode::Tetris
            | GameMode::Math
            | GameMode::Literacy
            | GameMode::Firefighters
            | GameMode::Detective
            | GameMode::SpaceFarm
            | GameMode::MusicFactory
            | GameMode::PixelAssault
            | GameMode::MushroomRun
            | GameMode::TankBattle
            | GameMode::SkyRaid
            | GameMode::BubbleIsland
            | GameMode::BubbleBomber
            | GameMode::FruitNinja
            | GameMode::Fortune
            | GameMode::Slots
            | GameMode::FruitArcade
            | GameMode::Fishing
            | GameMode::HoneyHunt
            | GameMode::MoonFestival
            | GameMode::Snake
            | GameMode::Minesweeper
            | GameMode::Classic2048
            | GameMode::Klotski
            | GameMode::Sokoban
            | GameMode::Sudoku => Work::default(),
            GameMode::Breakout => Work::default(),
            GameMode::ElementLab => Work::default(),
            GameMode::LightPath => Work::default(),
            GameMode::GardenDefense | GameMode::SpaceCourier => Work::default(),
            GameMode::Island => unreachable!(),
        };
        if mode == GameMode::Ball2048 {
            s.ball.reset();
            s.ball.running = true;
        }
        if mode == GameMode::MusicFactory {
            s.piano.reset();
        }
        if mode == GameMode::PixelAssault {
            s.assault.reset();
        }
        if mode == GameMode::MushroomRun {
            s.platform.reset();
        }
        if mode == GameMode::TankBattle {
            s.tank.reset();
        }
        if mode == GameMode::SkyRaid {
            s.sky.reset();
        }
        if mode == GameMode::BubbleIsland {
            s.bubble.reset();
        }
        if mode == GameMode::BubbleBomber {
            s.bomber.reset();
        }
        if mode == GameMode::FruitArcade {
            s.arcade = ArcadeGame::new();
        }
        if mode == GameMode::Slots {
            s.slots = SlotsGame::new();
        }
        if mode == GameMode::Fortune {
            s.fortune = FortuneGame::new();
        }
        if mode == GameMode::GardenDefense {
            s.garden.reset();
        }
        if mode == GameMode::FruitNinja {
            s.fruit.reset();
        }
        if mode == GameMode::Fishing {
            s.fishing.reset();
        }
        if mode == GameMode::HoneyHunt {
            s.honey.reset();
        }
        if mode == GameMode::MoonFestival {
            s.moon.reset();
        }
        if mode == GameMode::Snake {
            s.snake.reset();
        }
        if mode == GameMode::Minesweeper {
            s.minesweeper.reset();
        }
        if mode == GameMode::Classic2048 {
            s.classic_2048.reset();
        }
        if mode == GameMode::Klotski {
            s.klotski.reset();
        }
        if mode == GameMode::Sokoban {
            s.sokoban.reset();
        }
        if mode == GameMode::Breakout {
            s.breakout.reset();
        }
        s.build = matches!(mode, GameMode::CrystalGarden | GameMode::FreeBuild)
            || (mode == GameMode::CloudBridge && s.work.level == 5);
        s.message = mode.intro().into();
        if mode == GameMode::CloudBridge {
            s.message = if s.work.level == 5 {
                "终章挑战：搭建至少 10 块积木，并使用 3 种材料。".into()
            } else {
                game::LEVEL_HINTS[s.work.level].into()
            };
        }
        if mode == GameMode::FreeBuild {
            s.zoom = 22.0;
        }
        if mode == GameMode::CrystalGarden {
            s.message = "欢迎来到小小动物餐厅！先搭出厨房和客人的座位吧。".into();
        }
    }
    s.world_dirty = true;
    s.ui_dirty = true;
}

fn save_directory() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        bevy::window::ANDROID_APP
            .get()
            .and_then(|app| app.internal_data_path())
            .unwrap_or_else(|| PathBuf::from("."))
    }
    #[cfg(not(target_os = "android"))]
    {
        std::env::var_os("MAGNET_SAVE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("saves"))
    }
}
fn asset_directory() -> String {
    #[cfg(target_os = "android")]
    {
        "assets".into()
    }
    #[cfg(not(target_os = "android"))]
    {
        let local = std::env::current_dir().unwrap_or_default().join("assets");
        if local.is_dir() {
            return local.to_string_lossy().into_owned();
        }
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("assets")))
            .unwrap_or_else(|| PathBuf::from("assets"))
            .to_string_lossy()
            .into_owned()
    }
}
#[derive(Component)]
struct WorldPiece;
#[derive(Component)]
struct BuilderBackdrop;
#[derive(Component)]
struct BuilderBackdropCamera;
#[derive(Component)]
struct IslandPlate;
#[derive(Component)]
struct IslandGoalSprite;
#[derive(Component)]
struct BuilderGhost;
#[derive(Component)]
struct Robot;
#[derive(Component)]
struct IslandRobotBillboard;
#[derive(Component)]
struct GameCamera;
#[derive(Component)]
struct UiRoot;
#[derive(Component)]
struct LibraryCards;
#[derive(Component)]
struct ButtonTone(Color);
#[derive(Component, Clone, Copy)]
struct BallObstacleScene(usize, f32);
#[derive(Component)]
struct BallPlayer(f32);
#[derive(Component)]
struct CrystalSparkle(f32);
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Help,
    CloseHelp,
    MathTools,
    BubbleAssist,
    BubbleFire,
    MissionHint,
    MissionChoice(usize),
    MissionNext,
    PianoKey(usize),
    PianoReset,
    AssaultLeft,
    AssaultRight,
    AssaultJump,
    AssaultFire,
    AssaultReset,
    PlatformLeft,
    PlatformRight,
    PlatformJump,
    PlatformReset,
    SkyMove(i8),
    TankReset,
    TankRetry,
    FortuneRoll,
    FortuneBuy,
    FortuneSkip,
    FortuneReset,
    FortuneCenter,
    ArcadeSpin,
    ArcadeBet(usize),
    ArcadeClear,
    ArcadeRefill,
    SlotsSpin,
    SlotsBet(u32),
    SlotsRefill,
    SkyReset,
    BubbleReset,
    BomberMove(i8, i8),
    BomberPlace,
    BomberReset,
    BomberTogglePause,
    FruitReset,
    FishingReset,
    HoneyLaunch,
    HoneyReset,
    MoonShake,
    MoonReset,
    Courier(CourierAction),
    MagnetModes,
    MagnetModesClose,
    #[cfg(target_os = "macos")]
    LiteracySpeak,
    LiteracyStart(usize),
    LiteracySelect(usize),
    LiteracyCheck,
    LiteracyNext,
    LiteracyMap,
    MathPlace(i32),
    MathCheck,
    MathLevels,
    MathResume,
    MathLevel(usize),
    MathAnswer(i32),
    MathHint,
    MathNext,
    Library,
    SelectGame(GameMode),
    BallLeft,
    BallRight,
    BallStart,
    BallReset,
    BreakoutToggle,
    BreakoutLaunch,
    BreakoutReset,
    ElementPick(u8),
    ElementObserve,
    ElementNext,
    ElementReset,
    TetrisLeft,
    TetrisRight,
    TetrisRotate,
    TetrisDown,
    TetrisDrop,
    TetrisReset,
    BuilderTest,
    RestaurantCheck,
    RestaurantReset,
    SnakeMove(i8, i8),
    SnakeReset,
    SnakeToggle,
    MineReveal(usize),
    MineToggleFlag,
    MineReset,
    ClassicReset,
    ClassicMove(i8, i8),
    KlotskiSelect(usize),
    KlotskiMove(i8, i8),
    KlotskiReset,
    SokobanMove(i8, i8),
    SokobanReset,
    SokobanUndo,
    SokobanChoose,
    SokobanLevel(usize),
    SokobanNext,
    SudokuSelect(usize),
    SudokuDigit(u8),
    SudokuClear,
    SudokuHint,
    SudokuReset,
    SudokuSize(usize),
    SudokuNew,
    LightRotate(usize),
    LightReset,
    LightNext,
    GardenSelect(usize),
    GardenStart,
    GardenPause,
    GardenReset,
    GardenNext,
    GardenShovel,
    GardenCollectAll,
    Material(Material),
    LibraryReturn,
    Erase,
    Build,
    Code,
    Run,
    Step,
    Reset,
    Add(Instruction),
    Remove(usize),
    Clear,
    GuideRoute,
    Repeat,
    Save,
    Profile(usize),
    Next,
    Previous,
    CloudLevel(usize),
    Undo,
    ClearFreeBuild,
    Rotate,
    ResetView,
    ZoomIn,
    ZoomOut,
    ToggleIslandView,
    ToggleIslandMenu,
    Finish,
    ToggleEditor,
}
#[derive(Resource, Default)]
struct PressGate(bool);
#[derive(Resource, Default)]
struct Layout {
    compact: bool,
    width: f32,
    height: f32,
}
#[derive(Resource, Default)]
struct Gesture {
    touch: Option<u64>,
    start: Option<Vec2>,
    last: Vec2,
    moved: bool,
    world: bool,
    pinch: Option<f32>,
}
impl Gesture {
    fn finish_tap(&mut self, released: Option<Vec2>, scale: f32) -> Option<Vec2> {
        let start = self.start.take()?;
        let point = released.unwrap_or(self.last);
        let tapped = self.world
            && !self.moved
            && self.pinch.is_none()
            && point.distance(start) <= drag_slop(self.touch.is_some(), scale);
        self.touch = None;
        tapped.then_some(point)
    }
}

#[derive(Component)]
struct WorldInputBlocker;

#[derive(Resource, Default)]
struct LibraryDrag {
    start: Option<Vec2>,
    last: Vec2,
    dragging: bool,
    touch: Option<u64>,
    in_list: bool,
    pending_game: Option<GameMode>,
}
impl LibraryDrag {
    fn begin(&mut self, point: Vec2, touch: Option<u64>, in_list: bool) {
        *self = Self {
            start: Some(point),
            last: point,
            touch,
            in_list,
            ..default()
        };
    }
    fn move_to(&mut self, point: Vec2, scale: f32) -> f32 {
        let Some(start) = self.start else {
            return 0.;
        };
        let was_dragging = self.dragging;
        if point.distance(start) / scale.max(0.01) >= 8. {
            self.dragging = true;
            self.pending_game = None;
        }
        let previous = if was_dragging { self.last } else { start };
        self.last = point;
        if self.dragging && self.in_list {
            (previous.y - point.y) / scale.max(0.01)
        } else {
            0.
        }
    }
    fn finish(&mut self, released: Option<Vec2>, scale: f32) -> Option<GameMode> {
        if let Some(point) = released {
            self.move_to(point, scale);
        }
        let selected = if released.is_some() && !self.dragging {
            self.pending_game
        } else {
            None
        };
        *self = Self::default();
        selected
    }
}
#[derive(Resource, Default)]
struct TankHold {
    direction: Option<(i8, i8)>,
    repeat_in: f32,
    pointer: Option<Option<u64>>,
    center: Vec2,
    radius: f32,
    knob: Vec2,
    fire_in: f32,
}
#[derive(Component)]
struct TankStick;
#[derive(Component)]
struct TankKnob;
#[derive(Component)]
struct TankFirePad;
fn tank_stick_direction(delta: Vec2, radius: f32) -> Option<(i8, i8)> {
    if delta.length() < radius * 0.20 {
        return None;
    }
    if delta.x.abs() > delta.y.abs() {
        Some((if delta.x > 0. { 1 } else { -1 }, 0))
    } else {
        Some((0, if delta.y > 0. { 1 } else { -1 }))
    }
}
#[derive(Resource, Default)]
struct SkyHold {
    repeat_in: f32,
}
#[derive(Resource, Default)]
struct BubbleHold(bool);
#[derive(Resource, Default)]
struct BomberHold {
    direction: Option<(i8, i8)>,
    remaining: f32,
    touch_id: Option<u64>,
    mouse_active: bool,
}
#[derive(Component)]
struct BomberJoystick;
#[derive(Component)]
struct BomberJoystickKnob;
#[derive(Component)]
struct BomberTouchPlace;
#[derive(Resource, Default)]
struct TetrisHold {
    active: bool,
    repeat_in: f32,
}
#[derive(Resource, Default)]
struct FruitSlash {
    last: Option<Vec2>,
}
#[derive(Resource)]
struct Art {
    storybook: StorybookArt,
    restaurant_grass: Handle<StandardMaterial>,
    cube: Handle<Mesh>,
    round: Handle<Mesh>,
    grass: Handle<StandardMaterial>,
    island_grass: Handle<StandardMaterial>,
    island_cliff: Handle<StandardMaterial>,
    island_wood: Handle<StandardMaterial>,
    island_ghost: Handle<StandardMaterial>,
    island_crystal: Handle<StandardMaterial>,
    wood: Handle<StandardMaterial>,
    stone: Handle<StandardMaterial>,
    crystal: Handle<StandardMaterial>,
    brick: Handle<StandardMaterial>,
    sand: Handle<StandardMaterial>,
    snow: Handle<StandardMaterial>,
    glass: Handle<StandardMaterial>,
    lamp: Handle<StandardMaterial>,
    leaf: Handle<StandardMaterial>,
    soil: Handle<StandardMaterial>,
    yellow: Handle<StandardMaterial>,
    dark: Handle<StandardMaterial>,
    eye: Handle<StandardMaterial>,
    red: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
    font: Handle<Font>,
    lightpath_level_plates: Handle<Image>,
    lightpath_objects: Handle<Image>,
    mission_fire_approved: Handle<Image>,
    mission_fire_steps: [Handle<Image>; 3],
    mission_detective_approved: Handle<Image>,
    mission_detective_steps: [Handle<Image>; 3],
    mission_farm_approved: Handle<Image>,
    mission_farm_steps: [Handle<Image>; 3],
    mission_pause_overlays: [Handle<Image>; 3],
    mission_success_overlays: [Handle<Image>; 3],
    honey_story: Handle<Image>,
    honey_claw: Handle<Image>,
    honey_jar: Handle<Image>,
    courier_map: Handle<Image>,
    courier_title: Handle<Image>,
    courier_chapter_banner: Handle<Image>,
    courier_route_card: Handle<Image>,
    courier_station: Handle<Image>,
    courier_moon_name: Handle<Image>,
    courier_blueberry_name: Handle<Image>,
    courier_candy_name: Handle<Image>,
    courier_progress_title: Handle<Image>,
    courier_fuel: Handle<Image>,
    courier_navigation: Handle<Image>,
    courier_repair: Handle<Image>,
    courier_candy: Handle<Image>,
    courier_asteroids: Handle<Image>,
    courier_parcel: Handle<Image>,
    courier_parcel_slot: Handle<Image>,
    courier_empty_slot: Handle<Image>,
    courier_gift: Handle<Image>,
    bubble_story: Handle<Image>,
    sky_backdrop: Handle<Image>,
    sky_player: Handle<Image>,
    sky_enemy: Handle<Image>,
    sky_boss: Handle<Image>,
    sky_asteroid: Handle<Image>,
    fruit_paper: Handle<Image>,
    fruit_sprites: Handle<Image>,
    fruit_extras: Handle<Image>,
    bomber_tiles: Handle<Image>,
    bomber_backdrop: Handle<Image>,
    bomber_monsters: Handle<Image>,
    bomber_extra_monsters: Handle<Image>,
    bomber_action_bubble: Handle<Image>,
    element_backdrop: Handle<Image>,
    element_molecule: Handle<Image>,
    fishing_aquarium: Handle<Image>,
    fishing_fish: Handle<Image>,
    fortune_board: Handle<Image>,
    assault_forest: Handle<Image>,
    assault_hero: Handle<Image>,
    garden_backdrop: Handle<Image>,
    garden_plants: Handle<Image>,
    garden_enemies: Handle<Image>,
    garden_props: Handle<Image>,
    builder_backdrop: Handle<Image>,
    cloud_bridge_backdrop: Handle<Image>,
    island_landscape: Handle<Image>,
    island_free_build: Handle<Image>,
    island_goal_star: Handle<Image>,
    island_star_empty: Handle<Image>,
    island_materials: Handle<Image>,
    tank_sprites: Handle<Image>,
    tank_concept: Handle<Image>,
    tank_mobile_background_wide: Handle<Image>,
    tank_mobile_background_regular: Handle<Image>,
    placement_preview: Handle<StandardMaterial>,
    snake_reference: Handle<Image>,
    minesweeper_plate: Handle<Image>,
    minesweeper_atlas: Handle<Image>,
    sokoban_atlas: Handle<Image>,
}

// Small synthesized WAV clips keep the app completely offline and avoid
// shipping music that would distract from the games. They are intentionally
// short: a click confirms an action, a chime celebrates progress, and a low
// tone warns of danger.
#[derive(Resource)]
struct SoundBank {
    tap: Handle<AudioSource>,
    start: Handle<AudioSource>,
    success: Handle<AudioSource>,
    math_success_2: Handle<AudioSource>,
    math_success_4: Handle<AudioSource>,
    math_success_6: Handle<AudioSource>,
    danger: Handle<AudioSource>,
    slice: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct GameSoundState {
    initialized: bool,
    mode: Option<GameMode>,
    score: u64,
    lives: u8,
    failed: bool,
    lane: i8,
    positive: bool,
    negative: bool,
}

fn synthesized_wav(notes: &[(f32, f32)], gain: f32) -> AudioSource {
    const SAMPLE_RATE: u32 = 22_050;
    let samples = notes
        .iter()
        .flat_map(|&(frequency, duration)| {
            let count = (duration * SAMPLE_RATE as f32).round() as usize;
            (0..count).map(move |index| {
                let time = index as f32 / SAMPLE_RATE as f32;
                let fade = (time / 0.012).min(1.) * ((duration - time) / 0.035).clamp(0., 1.);
                let wave = (std::f32::consts::TAU * frequency * time).sin()
                    + (std::f32::consts::TAU * frequency * 2.01 * time).sin() * 0.16;
                (wave * fade * gain * i16::MAX as f32) as i16
            })
        })
        .collect::<Vec<_>>();
    let data_size = (samples.len() * std::mem::size_of::<i16>()) as u32;
    let mut bytes = Vec::with_capacity(44 + data_size as usize);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    bytes.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    AudioSource {
        bytes: bytes.into(),
    }
}

fn play_sound(commands: &mut Commands, source: Handle<AudioSource>, volume: f32) {
    commands.spawn((
        AudioPlayer::new(source),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)),
    ));
}

fn button_sounds(
    buttons: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    sounds: Res<SoundBank>,
    mut commands: Commands,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        play_sound(&mut commands, sounds.tap.clone(), 0.20);
    }
}

fn game_sounds(
    s: Res<Session>,
    sounds: Res<SoundBank>,
    mut state: ResMut<GameSoundState>,
    mut commands: Commands,
) {
    let (score, lives, failed, lane, positive, negative) = match s.mode {
        GameMode::Ball2048 => (
            s.ball.score,
            0,
            s.ball.failed,
            s.ball.lane,
            s.ball.finished,
            false,
        ),
        GameMode::Tetris => (
            u64::from(s.tetris.lines),
            0,
            s.tetris.failed,
            0,
            false,
            false,
        ),
        GameMode::Math => (
            u64::from(s.math.correct_total),
            0,
            false,
            0,
            s.math.solved,
            s.math.wrong.is_some(),
        ),
        GameMode::Literacy => (
            0,
            0,
            false,
            0,
            s.literacy.checked && s.literacy.correct,
            s.literacy.checked && !s.literacy.correct,
        ),
        GameMode::MusicFactory => (
            s.piano.score as u64,
            0,
            false,
            0,
            false,
            s.piano.last_key.is_some_and(|key| key != s.piano.target()),
        ),
        GameMode::PixelAssault => (
            u64::from(s.assault.score),
            s.assault.lives,
            s.assault.failed,
            0,
            false,
            false,
        ),
        GameMode::MushroomRun => (
            u64::from(s.platform.score),
            s.platform.lives,
            s.platform.failed,
            0,
            s.platform.finished,
            false,
        ),
        GameMode::TankBattle => (
            u64::from(s.tank.score),
            s.tank.lives,
            s.tank.failed,
            0,
            false,
            false,
        ),
        GameMode::Breakout => (
            u64::from(s.breakout.score),
            s.breakout.lives,
            s.breakout.over,
            0,
            s.breakout.won,
            false,
        ),
        GameMode::GardenDefense => (
            s.garden.score as u64,
            0,
            s.garden.over,
            0,
            s.garden.won,
            false,
        ),
        GameMode::Snake => (u64::from(s.snake.score), 0, s.snake.over, 0, false, false),
        GameMode::ElementLab => (
            u64::from(s.elements.score),
            0,
            false,
            0,
            s.elements.finished,
            false,
        ),
        GameMode::SkyRaid => (
            u64::from(s.sky.score),
            s.sky.lives,
            s.sky.failed,
            0,
            false,
            false,
        ),
        GameMode::BubbleIsland => (
            u64::from(s.bubble.score),
            0,
            s.bubble.failed,
            0,
            false,
            false,
        ),
        GameMode::BubbleBomber => (
            u64::from(s.bomber.score),
            s.bomber.lives,
            s.bomber.failed,
            0,
            false,
            false,
        ),
        GameMode::FruitNinja => (
            u64::from(s.fruit.score),
            s.fruit.lives,
            s.fruit.failed,
            0,
            false,
            false,
        ),
        GameMode::Fishing => (u64::from(s.fishing.score), 0, false, 0, false, false),
        GameMode::HoneyHunt => (u64::from(s.honey.score), 0, false, 0, false, false),
        GameMode::MoonFestival => (u64::from(s.moon.score), 0, false, 0, false, false),
        mode if mission_index(mode).is_some() => {
            (0, 0, false, 0, s.mission_done, s.mission_wrong.is_some())
        }
        _ => (0, 0, false, 0, false, false),
    };
    if !state.initialized {
        state.initialized = true;
    } else if state.mode != Some(s.mode) {
        play_sound(&mut commands, sounds.start.clone(), 0.24);
    } else if (!state.failed && failed)
        || lives < state.lives
        || (!state.negative && negative && s.mode != GameMode::Math)
    {
        play_sound(&mut commands, sounds.danger.clone(), 0.30);
    } else if score > state.score || (!state.positive && positive) {
        let source = if s.mode == GameMode::Math {
            if s.math.correct_total >= 6 {
                sounds.math_success_6.clone()
            } else if s.math.correct_total >= 4 {
                sounds.math_success_4.clone()
            } else if s.math.correct_total >= 2 {
                sounds.math_success_2.clone()
            } else {
                sounds.success.clone()
            }
        } else if s.mode == GameMode::FruitNinja {
            sounds.slice.clone()
        } else {
            sounds.success.clone()
        };
        let volume = if s.mode == GameMode::Math {
            (0.22 + s.math.correct_total as f32 * 0.025).min(0.38)
        } else {
            0.26
        };
        play_sound(&mut commands, source, volume);
    } else if s.mode == GameMode::Ball2048 && lane != state.lane {
        play_sound(&mut commands, sounds.tap.clone(), 0.18);
    }
    *state = GameSoundState {
        initialized: true,
        mode: Some(s.mode),
        score,
        lives,
        failed,
        lane,
        positive,
        negative,
    };
}

impl Art {
    fn material(&self, m: Material) -> Handle<StandardMaterial> {
        match m {
            Material::Grass => self.grass.clone(),
            Material::Wood => self.wood.clone(),
            Material::Stone => self.stone.clone(),
            Material::Crystal => self.crystal.clone(),
            Material::Brick => self.brick.clone(),
            Material::Sand => self.sand.clone(),
            Material::Snow => self.snow.clone(),
            Material::Glass => self.glass.clone(),
            Material::Lamp => self.lamp.clone(),
            Material::Leaf => self.leaf.clone(),
        }
    }
}
pub fn run() {
    // These games are built around a tall play field.  Start in the same
    // orientation that they will request after choosing them from the library,
    // so a preview never flashes the wrong layout first.
    let portrait_game = matches!(
        std::env::var("MAGNET_START_GAME").as_deref(),
        Ok("sky") | Ok("ball") | Ok("tetris") | Ok("moon") | Ok("bobing") | Ok("snake")
    );
    let tank_game = matches!(std::env::var("MAGNET_START_GAME").as_deref(), Ok("tank"));
    let mission_game = matches!(
        std::env::var("MAGNET_START_GAME").as_deref(),
        Ok("fire") | Ok("detective") | Ok("farm")
    );
    let mut app = App::new();
    app
        .insert_resource(ClearColor(Color::srgb(0.79, 0.88, 0.91)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 550.,
            ..default()
        })
        .insert_resource(Session::new())
        .init_resource::<PressGate>()
        .init_resource::<Gesture>()
        .init_resource::<LibraryDrag>()
        .init_resource::<TankHold>()
        .init_resource::<SkyHold>()
        .init_resource::<BubbleHold>()
        .init_resource::<ClassicSwipe>()
        .init_resource::<BomberHold>()
        .init_resource::<TetrisHold>()
        .init_resource::<FruitSlash>()
        .init_resource::<Layout>()
        .init_resource::<GameSoundState>()
        .init_resource::<BreakoutPointer>()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: asset_directory(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "小游戏中心".into(),
                        resolution: if std::env::var_os("MAGNET_PHONE_PREVIEW").is_some() {
                            if portrait_game {
                                (540_f32, 960_f32).into()
                            } else {
                                (960_f32, 540_f32).into()
                            }
                        } else if portrait_game {
                            (560_f32, 900_f32).into()
                        } else if tank_game {
                            (1280_f32, 720_f32).into()
                        } else if mission_game {
                            (1280_f32, 720_f32).into()
                        } else {
                            (1280_f32, 800_f32).into()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::render::texture::ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (button_feedback, sokoban_feedback, button_sounds).chain(),
                animate_math,
                math_island_input,
                animate_math_island,
                library_scroll_input,
                (
                    piano_key_input,
                    tetris_key_input,
                    retro_key_input,
                    assault_key_input,
                    platform_key_input,
                    tank_key_input,
                    breakout_key_input,
                    garden_key_input,
                    courier_key_input,
                )
                    .chain(),
                (
                    sky_key_input,
                    bubble_screen_input,
                    (
                        bomber_key_input,
                        bomber_hold_input,
                        bomber_joystick_input,
                        bomber_board_input,
                    )
                        .chain(),
                    fruit_screen_input,
                    fishing_screen_input,
                    honey_screen_input,
                )
                    .chain(),
                (ball_key_input, ball_screen_input).chain(),
                (
                    mine_board_input,
                    classic_swipe_input,
                    breakout_pointer_input,
                    garden_input,
                    buttons,
                    tetris_hold_input,
                    tank_touch_hold,
                    platform_touch_hold,
                    assault_touch_hold,
                )
                    .chain(),
                update_builder_preview,
                world_input,
                (tick, game_sounds).chain(),
                (sync_game_orientation, layout_camera).chain(),
                rebuild_world,
                animate_robot,
                sync_island_robot_billboard,
                animate_crystals,
                animate_ball_scene,
                rebuild_ui,
                (
                    animate_tank_stick,
                    animate_play_ui,
                    fortune_camera,
                    animate_fortune,
                    animate_slots,
                    animate_arcade,
                    capture_preview,
                )
                    .chain(),
            )
                .chain(),
        )
        .add_systems(
            PreUpdate,
            storybook::smoke_input
                .after(bevy::input::InputSystem)
                .before(bevy::ui::UiSystem::Focus),
        );
    #[cfg(debug_assertions)]
    app.add_plugins(qa::QaPlugin);
    app.run();
}

fn move_ball_lane(ball: &mut BallGame, direction: i8) -> bool {
    let next = (ball.lane + direction).clamp(0, 2);
    if next == ball.lane {
        false
    } else {
        ball.lane = next;
        true
    }
}

fn garden_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.mode != GameMode::GardenDefense || s.library_open || s.help_open {
        return;
    }
    let mut changed = false;
    for (index, key) in [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ]
    .into_iter()
    .enumerate()
    {
        if keys.just_pressed(key) {
            s.garden.select(index);
            changed = true;
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        if s.garden.ready {
            s.garden.start();
        } else {
            s.garden.toggle_pause();
        }
        changed = true;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        s.garden.toggle_pause();
        changed = true;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        s.garden.reset();
        changed = true;
    }
    if keys.just_pressed(KeyCode::KeyS) {
        s.garden.toggle_shovel();
        changed = true;
    }
    if keys.just_pressed(KeyCode::KeyC) {
        changed |= s.garden.collect_all() > 0;
    }
    if keys.just_pressed(KeyCode::Escape) && s.garden.shovel {
        s.garden.toggle_shovel();
        changed = true;
    }
    if changed {
        s.ui_dirty = true;
    }
}

fn courier_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.mode != GameMode::SpaceCourier || s.library_open || s.help_open {
        return;
    }
    if keys.just_pressed(KeyCode::Escape) {
        s.reset_run();
        s.library_open = true;
        s.build = false;
        s.editor_open = false;
        s.message = "选择一个想玩的小游戏吧".into();
        s.ui_dirty = true;
    }
}

fn ball_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open
        || s.help_open
        || s.mode != GameMode::Ball2048
        || s.ball.failed
        || s.ball.finished
    {
        return;
    }
    let direction = if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        Some(-1)
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        Some(1)
    } else {
        None
    };
    if let Some(direction) = direction {
        if move_ball_lane(&mut s.ball, direction) {
            s.message = if direction < 0 {
                "向左换道。"
            } else {
                "向右换道。"
            }
            .into();
        }
        s.ui_dirty = true;
    }
}

fn ball_screen_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    layout: Res<Layout>,
    mut s: ResMut<Session>,
) {
    if s.library_open {
        return;
    }
    if s.mode != GameMode::Math
        && (s.mode != GameMode::Ball2048 || s.ball.failed || s.ball.finished)
    {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let active = mouse.pressed(MouseButton::Left) || touches.iter().next().is_some();
    if !active {
        s.ball.pointer_x = None;
        return;
    }
    let position = touches
        .iter()
        .next()
        .map(|touch| touch.position())
        .or_else(|| window.cursor_position());
    let Some(position) = position else {
        return;
    };
    if matches!(s.mode, GameMode::Math | GameMode::Literacy) {
        return;
    }
    // Keep the header and large portrait controls out of the gesture field.
    // A tap on a UI action should never also steer the ball behind it.
    let controls_height = if layout.compact { 176. } else { 130. };
    if position.y < if layout.compact { 104. } else { 0. }
        || position.y > window.height() - controls_height
    {
        return;
    }
    // A press on either side still works, while dragging directly across the
    // ball changes lane as soon as the finger/mouse moves enough.
    let center = window.width() * 0.5;
    let previous_x = s.ball.pointer_x.replace(position.x);
    let delta = previous_x.map(|old| position.x - old).unwrap_or(0.);
    let direction = if delta.abs() >= 14. {
        Some(if delta < 0. { -1 } else { 1 })
    } else if previous_x.is_none() {
        let dead_zone = (window.width() * 0.12).max(48.0);
        if position.x < center - dead_zone {
            Some(-1)
        } else if position.x > center + dead_zone {
            Some(1)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(direction) = direction {
        if move_ball_lane(&mut s.ball, direction) {
            s.message = if direction < 0 {
                "向左换道。"
            } else {
                "向右换道。"
            }
            .into();
            s.ui_dirty = true;
        }
    }
}

fn image_asset_path(png_path: &str) -> String {
    #[cfg(target_os = "android")]
    {
        png_path
            .strip_suffix(".png")
            .map(|stem| format!("{stem}.webp"))
            .unwrap_or_else(|| png_path.to_owned())
    }
    #[cfg(not(target_os = "android"))]
    {
        png_path.to_owned()
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut audio_sources: ResMut<Assets<AudioSource>>,
) {
    fn clay(materials: &mut Assets<StandardMaterial>, color: Color) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.9,
            ..default()
        })
    }
    let grass = clay(&mut materials, Color::srgb(0.57, 0.76, 0.49));
    let wood = clay(&mut materials, Color::srgb(0.80, 0.59, 0.36));
    let stone = clay(&mut materials, Color::srgb(0.62, 0.70, 0.75));
    let crystal = clay(&mut materials, Color::srgb(0.48, 0.77, 0.79));
    let soil = clay(&mut materials, Color::srgb(0.59, 0.46, 0.34));
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.35, 0.78, 0.95, 0.38),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.1,
        perceptual_roughness: 0.1,
        ..default()
    });
    let lamp = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.62, 0.15),
        emissive: LinearRgba::new(1.0, 0.25, 0.03, 1.0),
        ..default()
    });
    let art = Art {
        storybook: load_storybook(&assets),
        restaurant_grass: materials.add(StandardMaterial {
            base_color_texture: Some(assets.load_with_settings(
                image_asset_path("storybook/restaurant-grass.png"),
                |s: &mut bevy::image::ImageLoaderSettings| {
                    s.sampler = bevy::image::ImageSampler::linear()
                },
            )),
            perceptual_roughness: 0.95,
            ..default()
        }),
        cube: meshes.add(Cuboid::new(1., 1., 1.)),
        round: meshes.add(Sphere::new(1.).mesh().uv(16, 12)),
        grass,
        island_grass: materials.add(StandardMaterial {
            base_color_texture: Some(assets.load(image_asset_path("builders/island-grass-v2.png"))),
            perceptual_roughness: 0.9,
            ..default()
        }),
        island_cliff: materials.add(StandardMaterial {
            base_color_texture: Some(assets.load(image_asset_path("builders/island-cliff-v2.png"))),
            perceptual_roughness: 0.9,
            ..default()
        }),
        island_wood: materials.add(StandardMaterial {
            base_color_texture: Some(assets.load(image_asset_path("builders/island-wood-v3.png"))),
            perceptual_roughness: 0.83,
            ..default()
        }),
        island_ghost: materials.add(StandardMaterial {
            base_color: Color::srgba(0.70, 0.97, 1.0, 0.78),
            emissive: LinearRgba::new(0.20, 0.65, 0.85, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        island_crystal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.92, 1.0),
            emissive: LinearRgba::new(0.10, 0.48, 0.67, 1.0),
            perceptual_roughness: 0.12,
            ..default()
        }),
        wood,
        stone,
        crystal,
        brick: clay(&mut materials, Color::srgb(0.70, 0.25, 0.18)),
        sand: clay(&mut materials, Color::srgb(0.86, 0.72, 0.42)),
        snow: clay(&mut materials, Color::srgb(0.91, 0.96, 1.0)),
        glass,
        lamp,
        leaf: clay(&mut materials, Color::srgb(0.22, 0.55, 0.22)),
        soil,
        yellow: materials.add(Color::srgb(1., 0.65, 0.13)),
        dark: materials.add(Color::srgb(0.075, 0.16, 0.19)),
        eye: materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.95, 0.96),
            emissive: LinearRgba::new(0.2, 0.7, 0.8, 1.),
            ..default()
        }),
        red: materials.add(Color::srgb(0.93, 0.29, 0.23)),
        water: materials.add(StandardMaterial {
            base_color: Color::srgb(0.79, 0.88, 0.91),
            unlit: true,
            perceptual_roughness: 0.38,
            ..default()
        }),
        font: assets.load("fonts/NotoSansCJKsc-Medium.otf"),
        mission_fire_approved: assets.load(image_asset_path("learning/fire-approved-v2.png")),
        mission_fire_steps: [
            assets.load(image_asset_path("learning/fire-step2-v2.png")),
            assets.load(image_asset_path("learning/fire-step3-v2.png")),
            assets.load(image_asset_path("learning/fire-step4-v2.png")),
        ],
        mission_detective_approved: assets
            .load(image_asset_path("learning/detective-approved-v2.png")),
        mission_detective_steps: [
            assets.load(image_asset_path("learning/detective-step2-v2.png")),
            assets.load(image_asset_path("learning/detective-step3-v2.png")),
            assets.load(image_asset_path("learning/detective-step4-v2.png")),
        ],
        mission_farm_approved: assets.load(image_asset_path("learning/farm-approved-v2.png")),
        mission_farm_steps: [
            assets.load(image_asset_path("learning/farm-step2-v2.png")),
            assets.load(image_asset_path("learning/farm-step3-v2.png")),
            assets.load(image_asset_path("learning/farm-step4-v2.png")),
        ],
        mission_pause_overlays: [
            assets.load(image_asset_path("learning/fire-pause-overlay-v2.png")),
            assets.load(image_asset_path("learning/detective-pause-overlay-v2.png")),
            assets.load(image_asset_path("learning/farm-pause-overlay-v2.png")),
        ],
        mission_success_overlays: [
            assets.load(image_asset_path("learning/fire-success-overlay-v2.png")),
            assets.load(image_asset_path(
                "learning/detective-success-overlay-v2.png",
            )),
            assets.load(image_asset_path("learning/farm-success-overlay-v2.png")),
        ],
        honey_story: assets.load(image_asset_path("honey/honey-hunt-felt-empty-v3.png")),
        honey_claw: assets.load(image_asset_path("honey/crane-claw-v1.png")),
        honey_jar: assets.load(image_asset_path("honey/honey-jar-v1.png")),
        courier_map: assets.load(image_asset_path("courier/space-map-v1.png")),
        courier_title: assets.load(image_asset_path("courier/title-logo-v1.png")),
        courier_chapter_banner: assets.load(image_asset_path("courier/chapter-banner-v1.png")),
        courier_route_card: assets.load(image_asset_path("courier/route-card-v1.png")),
        courier_station: assets.load(image_asset_path("courier/station-button-v1.png")),
        courier_moon_name: assets.load(image_asset_path("courier/moon-name-v1.png")),
        courier_blueberry_name: assets.load(image_asset_path("courier/blueberry-name-v1.png")),
        courier_candy_name: assets.load(image_asset_path("courier/candy-name-v1.png")),
        courier_progress_title: assets.load(image_asset_path("courier/progress-title-v1.png")),
        courier_fuel: assets.load(image_asset_path("courier/fuel-icon-v1.png")),
        courier_navigation: assets.load(image_asset_path("courier/navigation-icon-v1.png")),
        courier_repair: assets.load(image_asset_path("courier/repair-icon-v1.png")),
        courier_candy: assets.load(image_asset_path("courier/candy-icon-v1.png")),
        courier_asteroids: assets.load(image_asset_path("courier/asteroid-card-v1.png")),
        courier_parcel: assets.load(image_asset_path("courier/parcel-icon-v1.png")),
        courier_parcel_slot: assets.load(image_asset_path("courier/parcel-slot-v1.png")),
        courier_empty_slot: assets.load(image_asset_path("courier/empty-slot-v1.png")),
        courier_gift: assets.load(image_asset_path("courier/gift-v1.png")),
        lightpath_level_plates: assets.load_with_settings(
            image_asset_path("lightpath/level-screen-plates-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        lightpath_objects: assets.load_with_settings(
            image_asset_path("lightpath/optical-objects-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        bubble_story: assets.load(image_asset_path("bubble/storybook-sky-v1.png")),
        sky_backdrop: assets.load(image_asset_path("sky/nebula-field-v1.png")),
        sky_player: assets.load(image_asset_path("sky/player-fighter-v1.png")),
        sky_enemy: assets.load(image_asset_path("sky/enemy-fighter-v1.png")),
        sky_boss: assets.load(image_asset_path("sky/boss-mothership-v1.png")),
        sky_asteroid: assets.load(image_asset_path("sky/asteroid-v1.png")),
        fruit_paper: assets.load(image_asset_path("fruit/washi-paper-v1.png")),
        fruit_sprites: assets.load(image_asset_path("fruit/fruit-sprites-v1.png")),
        fruit_extras: assets.load(image_asset_path("fruit/fruit-extras-v1.png")),
        bomber_tiles: assets.load(image_asset_path("bomber/maze-atlas-v1.png")),
        bomber_backdrop: assets.load(image_asset_path("bomber/arena-backdrop-v1.png")),
        bomber_monsters: assets.load(image_asset_path("bomber/monster-atlas-v1.png")),
        bomber_extra_monsters: assets
            .load(image_asset_path("bomber/monster-atlas-v2-additions.png")),
        bomber_action_bubble: assets.load(image_asset_path("bomber/action-bubble-v2.png")),
        element_backdrop: assets.load(image_asset_path("elements/chemistry-lab-backdrop-v1.png")),
        element_molecule: assets.load(image_asset_path("elements/water-molecule-v1.png")),
        fishing_aquarium: assets.load(image_asset_path("fishing/aquarium-v1.png")),
        fishing_fish: assets.load(image_asset_path("fishing/fish-atlas-v1.png")),
        fortune_board: assets.load(image_asset_path("fortune/board-loop-v4.png")),
        assault_forest: assets.load(image_asset_path("assault/toy-forest-v1.png")),
        assault_hero: assets.load(image_asset_path("assault/explorer-hero-v1.png")),
        garden_backdrop: assets.load_with_settings(
            image_asset_path("garden/backdrop-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        garden_plants: assets.load_with_settings(
            image_asset_path("garden/plants-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        garden_enemies: assets.load_with_settings(
            image_asset_path("garden/enemies-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        garden_props: assets.load_with_settings(
            image_asset_path("garden/props-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        builder_backdrop: assets.load_with_settings(
            image_asset_path("builders/island-sky-v4.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        cloud_bridge_backdrop: assets.load_with_settings(
            image_asset_path("builders/cloud-bridge-sky-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        island_landscape: assets.load(image_asset_path("builders/island-landscape-v4.png")),
        island_free_build: assets.load(image_asset_path("builders/island-free-build-v1.png")),
        island_goal_star: assets.load(image_asset_path("builders/island-goal-star-v1.png")),
        island_star_empty: assets.load(image_asset_path("builders/island-star-empty-v1.png")),
        island_materials: assets.load(image_asset_path("builders/island-materials-v2.png")),
        tank_sprites: assets.load(image_asset_path("tank/tank-sprites-v4.png")),
        tank_concept: assets.load_with_settings(
            image_asset_path("tank/concept-reference.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        tank_mobile_background_wide: assets.load_with_settings(
            image_asset_path("tank/mobile-forest-20x9-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        tank_mobile_background_regular: assets.load_with_settings(
            image_asset_path("tank/mobile-forest-16x9-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        placement_preview: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.83, 0.25, 0.46),
            emissive: LinearRgba::new(0.45, 0.28, 0.02, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        snake_reference: assets.load(image_asset_path("snake/snake-reference-v1.png")),
        minesweeper_plate: assets.load_with_settings(
            image_asset_path("minesweeper/approved-plate-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        minesweeper_atlas: assets.load_with_settings(
            image_asset_path("minesweeper/approved-atlas-v1.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
        sokoban_atlas: assets.load_with_settings(
            image_asset_path("sokoban/approved-atlas.png"),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear();
            },
        ),
    };
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: 16.8,
            },
            ..OrthographicProjection::default_3d()
        }),
        Tonemapping::None,
        Transform::from_xyz(12., 16., 17.).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
    let builder_backdrop_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 0,
                clear_color: ClearColorConfig::Custom(Color::srgb(0.49, 0.78, 0.96)),
                ..default()
            },
            bevy::render::view::RenderLayers::layer(1),
            BuilderBackdropCamera,
        ))
        .id();
    let mut builder_background = ImageNode::new(art.builder_backdrop.clone());
    builder_background.image_mode = NodeImageMode::Stretch;
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        builder_background,
        UiTargetCamera(builder_backdrop_camera),
        bevy::render::view::RenderLayers::layer(1),
        Visibility::Visible,
        BuilderBackdrop,
    ));
    let mut island_plate = ImageNode::new(art.island_landscape.clone());
    island_plate.image_mode = NodeImageMode::Stretch;
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(125.),
            top: Val::Px(215.),
            width: Val::Px(760.),
            height: Val::Px(475.),
            ..default()
        },
        island_plate,
        UiTargetCamera(builder_backdrop_camera),
        bevy::render::view::RenderLayers::layer(1),
        Visibility::Visible,
        IslandPlate,
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(640.),
            top: Val::Px(420.),
            width: Val::Px(95.),
            height: Val::Px(95.),
            display: Display::None,
            ..default()
        },
        ImageNode::new(art.island_goal_star.clone()),
        UiTargetCamera(builder_backdrop_camera),
        bevy::render::view::RenderLayers::layer(1),
        IslandGoalSprite,
    ));
    commands.spawn((
        Mesh3d(art.cube.clone()),
        MeshMaterial3d(art.placement_preview.clone()),
        Transform::from_scale(Vec3::splat(1.015)),
        Visibility::Hidden,
        BuilderGhost,
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 4500.,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-4., 12., 8.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands
        .spawn((
            Transform::from_scale(Vec3::splat(1.28)),
            Visibility::default(),
            Robot,
        ))
        .with_children(|p| {
            let parts = [
                (
                    Vec3::new(0., 0.46, 0.),
                    Vec3::new(0.52, 0.52, 0.50),
                    art.snow.clone(),
                ),
                (
                    Vec3::new(0., 0.96, 0.),
                    Vec3::new(0.70, 0.53, 0.67),
                    art.yellow.clone(),
                ),
                (
                    Vec3::new(0.358, 0.97, 0.),
                    Vec3::new(0.018, 0.35, 0.48),
                    art.dark.clone(),
                ),
                (
                    Vec3::new(0.372, 1., -0.13),
                    Vec3::new(0.024, 0.16, 0.075),
                    art.eye.clone(),
                ),
                (
                    Vec3::new(0.372, 1., 0.13),
                    Vec3::new(0.024, 0.16, 0.075),
                    art.eye.clone(),
                ),
                (
                    Vec3::new(0., 1.33, 0.),
                    Vec3::new(0.05, 0.23, 0.05),
                    art.dark.clone(),
                ),
                (
                    Vec3::new(0., 1.46, 0.),
                    Vec3::splat(0.13),
                    art.yellow.clone(),
                ),
                (
                    Vec3::new(0.08, 0.1, -0.20),
                    Vec3::new(0.46, 0.18, 0.20),
                    art.dark.clone(),
                ),
                (
                    Vec3::new(0.08, 0.1, 0.20),
                    Vec3::new(0.46, 0.18, 0.20),
                    art.dark.clone(),
                ),
                (
                    Vec3::new(0., 0.5, -0.36),
                    Vec3::new(0.20, 0.34, 0.18),
                    art.yellow.clone(),
                ),
                (
                    Vec3::new(0., 0.5, 0.36),
                    Vec3::new(0.20, 0.34, 0.18),
                    art.yellow.clone(),
                ),
            ];
            for (pos, scale, mat) in parts {
                p.spawn((
                    Mesh3d(art.cube.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_translation(pos).with_scale(scale),
                ));
            }
        });
    commands.spawn((
        IslandRobotBillboard,
        Mesh3d(meshes.add(Rectangle::new(1., 1.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(assets.load(image_asset_path("builders/island-robot-v1.png"))),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
        Transform::from_scale(Vec3::new(2.8, 2.8, 1.)),
        Visibility::Hidden,
    ));
    commands.insert_resource(art);
    commands.insert_resource(SoundBank {
        tap: audio_sources.add(synthesized_wav(&[(660., 0.045)], 0.24)),
        start: audio_sources.add(synthesized_wav(&[(392., 0.055), (523., 0.10)], 0.22)),
        success: audio_sources.add(synthesized_wav(
            &[(523., 0.055), (659., 0.065), (784., 0.12)],
            0.20,
        )),
        math_success_2: audio_sources.add(synthesized_wav(
            &[(523., 0.05), (659., 0.05), (784., 0.06), (988., 0.13)],
            0.19,
        )),
        math_success_4: audio_sources.add(synthesized_wav(
            &[
                (523., 0.05),
                (659., 0.05),
                (784., 0.05),
                (988., 0.06),
                (1175., 0.16),
            ],
            0.19,
        )),
        math_success_6: audio_sources.add(synthesized_wav(
            &[
                (523., 0.05),
                (659., 0.05),
                (784., 0.05),
                (988., 0.05),
                (1175., 0.05),
                (1568., 0.18),
            ],
            0.19,
        )),
        danger: audio_sources.add(synthesized_wav(&[(180., 0.16), (130., 0.20)], 0.22)),
        slice: audio_sources.add(synthesized_wav(&[(300., 0.035), (620., 0.075)], 0.20)),
    });
}
fn layout_camera(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<(&mut Camera, &mut Transform, &mut Projection), With<GameCamera>>,
    mut backdrop_camera: Query<&mut Camera, (With<BuilderBackdropCamera>, Without<GameCamera>)>,
    mut backdrop_visibility: Query<(&mut Visibility, &mut ImageNode), With<BuilderBackdrop>>,
    mut island_plate: Query<&mut Node, (With<IslandPlate>, Without<IslandGoalSprite>)>,
    mut island_goal_sprite: Query<&mut Node, (With<IslandGoalSprite>, Without<IslandPlate>)>,
    mut ui_scale: ResMut<UiScale>,
    mut layout: ResMut<Layout>,
    s: Res<Session>,
    art: Res<Art>,
    mut last_physical_size: Local<Option<UVec2>>,
    mut resize_cooldown: Local<u8>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let physical_size = UVec2::new(
        window.resolution.physical_width(),
        window.resolution.physical_height(),
    );
    if *last_physical_size != Some(physical_size) {
        *last_physical_size = Some(physical_size);
        // The surface color texture may still belong to the previous window
        // size while Bevy has already recreated the depth texture. Suppress
        // this camera briefly so those attachments are never mixed in a pass.
        *resize_cooldown = 3;
    }
    let show_builder_backdrop = matches!(
        s.mode,
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild | GameMode::CrystalGarden
    );
    if let Ok(mut camera) = backdrop_camera.single_mut() {
        camera.is_active = show_builder_backdrop;
    }
    if let Ok((mut visibility, mut image)) = backdrop_visibility.single_mut() {
        *visibility = if show_builder_backdrop {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        image.image = if s.mode == GameMode::CloudBridge {
            art.cloud_bridge_backdrop.clone()
        } else {
            art.builder_backdrop.clone()
        };
    }
    // Portrait phones are tall, so height alone cannot identify their compact
    // touch layout.  A narrow viewport needs the same treatment as a short one.
    let compact = cfg!(target_os = "android") || window.height() < 650. || window.width() < 650.;
    if let Ok(mut plate) = island_plate.single_mut() {
        if s.mode == GameMode::FreeBuild && compact {
            plate.left = Val::Px(85.);
            plate.top = Val::Px(72.);
            plate.width = Val::Px(585.);
            plate.height = Val::Px(360.);
        } else if s.mode == GameMode::FreeBuild {
            plate.left = Val::Px(100.);
            plate.top = Val::Px(114.);
            plate.width = Val::Px(790.);
            plate.height = Val::Px(505.);
        } else if compact {
            plate.left = Val::Px(60.);
            plate.top = Val::Px(70.);
            plate.width = Val::Px(585.);
            plate.height = Val::Px(366.);
        } else {
            plate.left = Val::Px(125.);
            plate.top = Val::Px(215.);
            plate.width = Val::Px(760.);
            plate.height = Val::Px(475.);
        }
    }
    if let Ok(mut goal) = island_goal_sprite.single_mut() {
        let illustrated_goal = s.mode == GameMode::Island
            && !s.island_3d_view
            && island_base_intact(&s.work)
            && s.work.level == 3;
        goal.display = if illustrated_goal {
            Display::Flex
        } else {
            Display::None
        };
        if compact {
            goal.left = Val::Px(457.);
            goal.top = Val::Px(228.);
            goal.width = Val::Px(75.);
            goal.height = Val::Px(75.);
        } else {
            goal.left = Val::Px(640.);
            goal.top = Val::Px(420.);
            goal.width = Val::Px(95.);
            goal.height = Val::Px(95.);
        }
    }
    if layout.compact != compact
        || layout.width != window.width()
        || layout.height != window.height()
    {
        *layout = Layout {
            compact,
            width: window.width(),
            height: window.height(),
        };
    }
    let scale = if compact && window.height() > window.width() {
        (window.width() / 540.).min(window.height() / 960.)
    } else if compact {
        (window.width() / 960.).min(window.height() / 540.)
    } else {
        (window.width() / 1280.).min(window.height() / 760.)
    }
    .max(0.2);
    ui_scale.0 = scale;
    let Ok((mut camera, mut transform, mut projection)) = cameras.single_mut() else {
        return;
    };
    camera.is_active = *resize_cooldown == 0;
    *resize_cooldown = resize_cooldown.saturating_sub(1);
    if s.mode == GameMode::CrystalGarden {
        let k = (window.width() / 1672.).min(window.height() / 941.);
        let offset =
            (Vec2::new(window.width(), window.height()) - Vec2::new(1672., 941.) * k) * 0.5;
        let dpi = window.scale_factor();
        camera.viewport = Some(Viewport {
            physical_position: ((offset + Vec2::new(355., 125.) * k) * dpi).as_uvec2(),
            physical_size: (Vec2::new(962., 655.) * k * dpi).as_uvec2(),
            ..default()
        });
        camera.clear_color = ClearColorConfig::None;
        let mut p = OrthographicProjection::default_3d();
        p.scaling_mode = ScalingMode::AutoMin {
            min_width: s.zoom * 1.1,
            min_height: s.zoom * 0.85,
        };
        *projection = Projection::Orthographic(p);
        let radius = 30.4;
        let horizontal = radius * s.pitch.cos();
        *transform = Transform::from_xyz(
            s.yaw.sin() * horizontal,
            radius * s.pitch.sin(),
            s.yaw.cos() * horizontal,
        )
        .looking_at(Vec3::ZERO, Vec3::Y);
        return;
    }
    if s.mode == GameMode::Literacy {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.89, 0.96, 0.97));
        let mut p = OrthographicProjection::default_3d();
        p.scaling_mode = ScalingMode::FixedHorizontal {
            viewport_width: 19.,
        };
        *projection = Projection::Orthographic(p);
        *transform = Transform::from_xyz(0., 5., 16.).looking_at(Vec3::new(4.3, 0.5, 0.), Vec3::Y);
        return;
    }
    if s.mode == GameMode::Math {
        let origin = Vec2::new(16., 108.) * scale;
        let size = Vec2::new(
            window.width() - 32. * scale,
            (window.height() - 304. * scale).max(100.),
        );
        camera.viewport = Some(Viewport {
            physical_position: (origin * window.scale_factor()).as_uvec2(),
            physical_size: (size * window.scale_factor()).as_uvec2(),
            ..default()
        });
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(1.0, 0.97, 0.91));
        let mut p = OrthographicProjection::default_3d();
        p.scaling_mode = ScalingMode::AutoMin {
            min_width: 20.,
            min_height: 10.,
        };
        let kick = if s.cheer > 0. {
            (s.cheer * 34.).sin() * s.cheer * (0.025 + s.math.correct_total.min(6) as f32 * 0.003)
        } else {
            0.
        };
        *projection = Projection::Orthographic(p);
        *transform = Transform::from_xyz(s.yaw.sin() * 18. + kick, 15., s.yaw.cos() * 18.)
            .looking_at(Vec3::new(0., 0., 0.), Vec3::Y);
        return;
    }
    if s.mode == GameMode::Ball2048 {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.72, 0.88, 0.96));
        if !matches!(*projection, Projection::Perspective(_)) {
            *projection = Projection::Perspective(PerspectiveProjection {
                fov: 52_f32.to_radians(),
                near: 0.1,
                far: 240.,
                ..default()
            });
        }
        let lane_x = if s.mode == GameMode::Math {
            0.
        } else {
            (s.ball.lane as f32 - 1.) * 2.5
        };
        let shake = if s.ball.failed {
            (s.ball.effect * 50.).sin() * s.ball.effect * 0.22
        } else {
            0.
        };
        *transform = Transform::from_xyz(lane_x + shake, 6.2, 12.5)
            .looking_at(Vec3::new(lane_x, 0.3, -12.), Vec3::Y);
        return;
    }
    if s.mode == GameMode::Tetris {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.08, 0.10, 0.22));
        return;
    }
    if s.mode == GameMode::PixelAssault {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.05, 0.08, 0.16));
        return;
    }
    if s.mode == GameMode::MushroomRun {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.40, 0.76, 0.94));
        return;
    }
    if s.mode == GameMode::TankBattle {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.08, 0.12, 0.16));
        return;
    }
    if matches!(
        s.mode,
        GameMode::Breakout | GameMode::ElementLab | GameMode::LightPath | GameMode::GardenDefense
    ) {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.05, 0.08, 0.16));
        return;
    }
    if s.mode == GameMode::SkyRaid {
        camera.viewport = None;
        // The flight game is drawn entirely in the UI so it can use every pixel
        // of a narrow phone screen instead of inheriting the 3D island viewport.
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.70, 0.77, 0.80));
        return;
    }
    if s.mode == GameMode::BubbleIsland {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.52, 0.78, 0.86));
        return;
    }
    if s.mode == GameMode::BubbleBomber {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.30, 0.70, 0.78));
        return;
    }
    if matches!(
        s.mode,
        GameMode::Fortune
            | GameMode::Slots
            | GameMode::FruitArcade
            | GameMode::Fishing
            | GameMode::HoneyHunt
            | GameMode::MoonFestival
            | GameMode::Snake
            | GameMode::Minesweeper
            | GameMode::Classic2048
            | GameMode::Klotski
            | GameMode::Sokoban
            | GameMode::Sudoku
            | GameMode::LightPath
    ) {
        camera.viewport = None;
        return;
    }
    if s.mode == GameMode::FruitNinja {
        camera.viewport = None;
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.08, 0.18, 0.24));
        return;
    }
    camera.clear_color = ClearColorConfig::Default;
    if !matches!(*projection, Projection::Orthographic(_)) {
        *projection = Projection::Orthographic(OrthographicProjection::default_3d());
    }
    let dpi = window.scale_factor();
    let island_desktop = s.mode == GameMode::Island && !compact;
    let island_compact = s.mode == GameMode::Island && compact;
    let cloud_bridge = s.mode == GameMode::CloudBridge;
    let origin = if cloud_bridge && !compact {
        Vec2::new(18., 94.) * scale
    } else if island_desktop {
        Vec2::new(18., 166.) * scale
    } else if island_compact {
        Vec2::new(8., 85.) * scale
    } else if s.mode == GameMode::FreeBuild && !compact {
        Vec2::new(60., 60.) * scale
    } else if s.mode == GameMode::FreeBuild {
        Vec2::new(8., 48.) * scale
    } else if compact {
        Vec2::new(16., 116.) * scale
    } else {
        Vec2::new(142., 150.) * scale
    };
    let size = Vec2::new(
        (window.width()
            - if cloud_bridge && !compact {
                186. * scale
            } else if island_desktop {
                340. * scale
            } else if island_compact {
                226. * scale
            } else if compact {
                if s.build { 288. } else { 32. }
            } else {
                500.
            } * scale)
            .max(100.),
        (window.height()
            - if cloud_bridge && !compact {
                224. * scale
            } else if island_desktop {
                266. * scale
            } else if island_compact {
                204. * scale
            } else if compact {
                if s.build {
                    132.
                } else if s.editor_open {
                    384.
                } else {
                    192.
                }
            } else {
                300.
            } * scale)
            .max(100.),
    );
    camera.viewport = Some(Viewport {
        physical_position: (origin * dpi).as_uvec2(),
        physical_size: (size * dpi).as_uvec2(),
        ..default()
    });
    let radius = 30.4;
    let horizontal = radius * s.pitch.cos();
    *transform = Transform::from_xyz(
        s.yaw.sin() * horizontal,
        radius * s.pitch.sin(),
        s.yaw.cos() * horizontal,
    )
    .looking_at(Vec3::ZERO, Vec3::Y);
    if let Projection::Orthographic(p) = &mut *projection {
        p.scaling_mode = ScalingMode::AutoMin {
            min_width: s.zoom
                * if island_desktop {
                    0.90
                } else if island_compact {
                    1.18
                } else {
                    1.25
                },
            min_height: s.zoom * 0.85,
        };
    }
}
fn cube(
    commands: &mut Commands,
    art: &Art,
    pos: Vec3,
    scale: Vec3,
    material: Handle<StandardMaterial>,
) {
    commands.spawn((
        WorldPiece,
        Mesh3d(art.cube.clone()),
        MeshMaterial3d(material),
        Transform::from_translation(pos).with_scale(scale),
    ));
}

fn island_bridge_guide(commands: &mut Commands, art: &Art, x: f32) {
    let edge = 0.43;
    let thin = 0.025;
    for y in [-edge, edge] {
        for z in [-edge, edge] {
            cube(
                commands,
                art,
                Vec3::new(x, y, z),
                Vec3::new(edge * 2., thin, thin),
                art.island_ghost.clone(),
            );
        }
    }
    for x_offset in [-edge, edge] {
        for z in [-edge, edge] {
            cube(
                commands,
                art,
                Vec3::new(x + x_offset, 0., z),
                Vec3::new(thin, edge * 2., thin),
                art.island_ghost.clone(),
            );
        }
    }
    for x_offset in [-edge, edge] {
        for y in [-edge, edge] {
            cube(
                commands,
                art,
                Vec3::new(x + x_offset, y, 0.),
                Vec3::new(thin, thin, edge * 2.),
                art.island_ghost.clone(),
            );
        }
    }
}
fn rebuild_world(
    mut commands: Commands,
    mut s: ResMut<Session>,
    art: Res<Art>,
    entities: Query<Entity, With<WorldPiece>>,
    mut robots: Query<&mut Visibility, (With<Robot>, Without<IslandPlate>)>,
    mut island_plate: Query<(&mut Visibility, &mut ImageNode), (With<IslandPlate>, Without<Robot>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !s.world_dirty {
        return;
    }
    s.world_dirty = false;
    let illustrated_island =
        s.mode == GameMode::Island && !s.island_3d_view && island_base_intact(&s.work);
    if let Ok((mut visibility, mut image)) = island_plate.single_mut() {
        *visibility = if illustrated_island || (s.mode == GameMode::FreeBuild && !s.island_3d_view)
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        image.image = if s.mode == GameMode::FreeBuild {
            art.island_free_build.clone()
        } else {
            art.island_landscape.clone()
        };
    }
    for e in &entities {
        commands.entity(e).despawn();
    }
    for mut visibility in &mut robots {
        *visibility = if illustrated_island
            || (s.mode == GameMode::CloudBridge && s.work.level == 5)
            || matches!(
                s.mode,
                GameMode::Ball2048
                    | GameMode::Tetris
                    | GameMode::Math
                    | GameMode::Literacy
                    | GameMode::PixelAssault
                    | GameMode::MushroomRun
                    | GameMode::TankBattle
                    | GameMode::Breakout
                    | GameMode::ElementLab
                    | GameMode::LightPath
                    | GameMode::GardenDefense
                    | GameMode::SkyRaid
                    | GameMode::BubbleIsland
                    | GameMode::BubbleBomber
                    | GameMode::FruitNinja
                    | GameMode::Fortune
                    | GameMode::Slots
                    | GameMode::FruitArcade
                    | GameMode::Fishing
                    | GameMode::HoneyHunt
                    | GameMode::MoonFestival
                    | GameMode::Snake
                    | GameMode::Minesweeper
                    | GameMode::Classic2048
                    | GameMode::Klotski
                    | GameMode::Sokoban
                    | GameMode::Sudoku
                    | GameMode::FreeBuild
                    | GameMode::CrystalGarden
            ) {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
    if s.mode == GameMode::Literacy {
        spawn_literacy_scene(&mut commands, &s.literacy, &mut meshes, &mut materials);
        return;
    }
    if s.mode == GameMode::Math {
        spawn_math_scene(&mut commands, &s.math, &mut meshes, &mut materials);
        return;
    }
    if s.mode == GameMode::Ball2048 {
        spawn_ball_scene(&mut commands, &s.ball, &mut meshes, &mut materials);
        return;
    }
    if s.mode == GameMode::Tetris {
        return;
    }
    if s.mode == GameMode::PixelAssault {
        return;
    }
    if s.mode == GameMode::MushroomRun {
        return;
    }
    if s.mode == GameMode::TankBattle {
        return;
    }
    if matches!(
        s.mode,
        GameMode::Breakout | GameMode::ElementLab | GameMode::LightPath | GameMode::GardenDefense
    ) {
        return;
    }
    if s.mode == GameMode::SkyRaid {
        return;
    }
    if s.mode == GameMode::BubbleIsland {
        return;
    }
    if s.mode == GameMode::BubbleBomber {
        return;
    }
    if matches!(
        s.mode,
        GameMode::Fortune
            | GameMode::Slots
            | GameMode::FruitArcade
            | GameMode::Fishing
            | GameMode::HoneyHunt
            | GameMode::MoonFestival
            | GameMode::Snake
            | GameMode::Minesweeper
            | GameMode::Classic2048
            | GameMode::Klotski
            | GameMode::Sokoban
            | GameMode::Sudoku
    ) {
        return;
    }
    if s.mode == GameMode::FruitNinja {
        return;
    }
    if !matches!(
        s.mode,
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild | GameMode::CrystalGarden
    ) {
        cube(
            &mut commands,
            &art,
            Vec3::new(0., -0.67, 0.),
            Vec3::new(200., 0.10, 200.),
            art.water.clone(),
        );
    }
    for b in &s.work.world.blocks {
        let [x, y, z] = b.pos;
        if (illustrated_island || (s.mode == GameMode::FreeBuild && !s.island_3d_view))
            && y == 0
            && b.material == Material::Grass
        {
            continue;
        }
        if b.material == Material::Grass {
            cube(
                &mut commands,
                &art,
                Vec3::new(x as f32, y as f32 - 0.05, z as f32),
                Vec3::new(0.995, 0.84, 0.995),
                if matches!(
                    s.mode,
                    GameMode::Island | GameMode::CloudBridge | GameMode::CrystalGarden
                ) {
                    art.island_cliff.clone()
                } else {
                    art.soil.clone()
                },
            );
            if s.mode == GameMode::CloudBridge && y == 0 {
                cube(
                    &mut commands,
                    &art,
                    Vec3::new(x as f32, y as f32 - 0.78, z as f32),
                    Vec3::new(0.98, 0.62, 0.98),
                    art.island_cliff.clone(),
                );
            }
            cube(
                &mut commands,
                &art,
                Vec3::new(x as f32, y as f32 + 0.40, z as f32),
                Vec3::new(1.0, 0.18, 1.0),
                if s.mode == GameMode::CrystalGarden {
                    art.restaurant_grass.clone()
                } else if s.mode == GameMode::Island {
                    art.island_grass.clone()
                } else if s.mode == GameMode::CloudBridge {
                    art.grass.clone()
                } else {
                    art.grass.clone()
                },
            );
        } else {
            cube(
                &mut commands,
                &art,
                Vec3::new(x as f32, y as f32, z as f32),
                Vec3::splat(0.94),
                if matches!(
                    s.mode,
                    GameMode::Island | GameMode::CloudBridge | GameMode::CrystalGarden
                ) && b.material == Material::Wood
                {
                    art.island_wood.clone()
                } else {
                    art.material(b.material)
                },
            );
        }
    }
    if s.mode == GameMode::Island && s.work.level == 3 {
        // Pale magnetic landing guides show the five missing bridge positions.
        // They are scenery only; the player still has to place real blocks.
        for x in -2..=2 {
            if s.work.world.material([x, 0, 0]).is_none() {
                island_bridge_guide(&mut commands, &art, x as f32);
            }
        }
    }
    for (index, [x, z]) in s.work.world.crystals.iter().copied().enumerate() {
        commands.spawn((
            WorldPiece,
            CrystalSparkle(index as f32 * 1.7),
            Mesh3d(art.cube.clone()),
            MeshMaterial3d(if s.mode == GameMode::Island {
                art.island_crystal.clone()
            } else {
                art.crystal.clone()
            }),
            Transform::from_xyz(x as f32, 1.18, z as f32)
                .with_rotation(Quat::from_rotation_z(0.55))
                .with_scale(Vec3::new(0.42, 0.70, 0.42)),
        ));
    }
    if !matches!(s.mode, GameMode::FreeBuild | GameMode::CrystalGarden)
        && !(s.mode == GameMode::CloudBridge && s.work.level == 5)
    {
        if !matches!(
            s.mode,
            GameMode::Island | GameMode::CloudBridge | GameMode::CrystalGarden
        ) {
            spawn_island_people(&mut commands, &art, &mut meshes, &mut materials);
        }
        let [gx, gz] = s.work.world.goal;
        if s.mode == GameMode::Island {
            if !(illustrated_island && s.work.level == 3) {
                // Five gold rays make the live 3D destination read as the star in
                // the design, from the playable isometric camera.
                for ray in 0..5 {
                    let angle = ray as f32 * std::f32::consts::TAU / 5.;
                    commands.spawn((
                        WorldPiece,
                        Mesh3d(art.cube.clone()),
                        MeshMaterial3d(art.lamp.clone()),
                        Transform::from_xyz(
                            gx as f32 + angle.sin() * 0.28,
                            1.38 + angle.cos() * 0.28,
                            gz as f32,
                        )
                        .with_rotation(Quat::from_rotation_z(-angle))
                        .with_scale(Vec3::new(0.18, 0.52, 0.16)),
                    ));
                }
                commands.spawn((
                    WorldPiece,
                    CrystalSparkle(2.4),
                    Mesh3d(art.cube.clone()),
                    MeshMaterial3d(art.lamp.clone()),
                    Transform::from_xyz(gx as f32, 1.38, gz as f32)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4))
                        .with_scale(Vec3::new(0.42, 0.42, 0.18)),
                ));
            }
            // Keep the destination clear during bridge challenges; the cottage
            // belongs only in the final free-building home level.
            if s.work.level == 5 {
                spawn_island_cottage(&mut commands, &art);
            }
        } else if s.mode == GameMode::CloudBridge {
            for ray in 0..5 {
                let angle = ray as f32 * std::f32::consts::TAU / 5.;
                commands.spawn((
                    WorldPiece,
                    Mesh3d(art.cube.clone()),
                    MeshMaterial3d(art.lamp.clone()),
                    Transform::from_xyz(
                        gx as f32 + angle.sin() * 0.27,
                        1.40 + angle.cos() * 0.27,
                        gz as f32,
                    )
                    .with_rotation(Quat::from_rotation_z(-angle))
                    .with_scale(Vec3::new(0.16, 0.48, 0.15)),
                ));
            }
            commands.spawn((
                WorldPiece,
                CrystalSparkle(2.4),
                Mesh3d(art.cube.clone()),
                MeshMaterial3d(art.lamp.clone()),
                Transform::from_xyz(gx as f32, 1.40, gz as f32)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4))
                    .with_scale(Vec3::new(0.42, 0.42, 0.18)),
            ));
        } else {
            cube(
                &mut commands,
                &art,
                Vec3::new(gx as f32, 1.25, gz as f32),
                Vec3::new(0.075, 1.5, 0.075),
                art.wood.clone(),
            );
            cube(
                &mut commands,
                &art,
                Vec3::new(gx as f32 + 0.30, 1.80, gz as f32),
                Vec3::new(0.58, 0.35, 0.055),
                art.red.clone(),
            );
        }
        for (x, z) in if s.mode == GameMode::Island {
            if s.island_3d_view {
                vec![(-5., -2.), (5., -2.)]
            } else {
                vec![]
            }
        } else {
            vec![(5., -2.), (-5., -2.)]
        } {
            cube(
                &mut commands,
                &art,
                Vec3::new(x, 0.94, z),
                Vec3::new(0.30, 0.9, 0.30),
                art.wood.clone(),
            );
            if s.mode == GameMode::Island {
                for (dx, dy, dz, size) in [
                    (0., 1.68, 0., 0.74),
                    (-0.48, 1.55, 0., 0.46),
                    (0.43, 1.58, 0.05, 0.48),
                    (0., 1.89, -0.31, 0.42),
                ] {
                    commands.spawn((
                        WorldPiece,
                        Mesh3d(art.round.clone()),
                        MeshMaterial3d(art.leaf.clone()),
                        Transform::from_xyz(x + dx, dy, z + dz).with_scale(Vec3::new(
                            size,
                            size * 0.8,
                            size,
                        )),
                    ));
                }
            } else {
                cube(
                    &mut commands,
                    &art,
                    Vec3::new(x, 1.55, z),
                    Vec3::new(1.15, 0.65, 1.10),
                    art.leaf.clone(),
                );
                cube(
                    &mut commands,
                    &art,
                    Vec3::new(x, 2., z),
                    Vec3::new(0.70, 0.40, 0.70),
                    art.grass.clone(),
                );
            }
        }
    }
}

fn spawn_island_cottage(commands: &mut Commands, art: &Art) {
    let x = 4.7;
    let z = 1.05;
    // Warm little voxel cottage: cream walls, timber door, blue windows and a
    // two-piece terracotta roof, placed on the goal island beside the star.
    cube(
        commands,
        art,
        Vec3::new(x, 0.98, z),
        Vec3::new(1.55, 0.98, 1.28),
        art.snow.clone(),
    );
    cube(
        commands,
        art,
        Vec3::new(x, 0.55, z + 0.67),
        Vec3::new(0.36, 0.74, 0.10),
        art.wood.clone(),
    );
    for wx in [x - 0.48, x + 0.48] {
        cube(
            commands,
            art,
            Vec3::new(wx, 1.12, z + 0.66),
            Vec3::new(0.31, 0.32, 0.08),
            art.glass.clone(),
        );
        cube(
            commands,
            art,
            Vec3::new(wx, 1.12, z + 0.66),
            Vec3::new(0.38, 0.045, 0.10),
            art.wood.clone(),
        );
        cube(
            commands,
            art,
            Vec3::new(wx, 1.12, z + 0.66),
            Vec3::new(0.045, 0.37, 0.10),
            art.wood.clone(),
        );
    }
    for (roof_x, tilt) in [(x - 0.40, 0.55), (x + 0.40, -0.55)] {
        commands.spawn((
            WorldPiece,
            Mesh3d(art.cube.clone()),
            MeshMaterial3d(art.brick.clone()),
            Transform::from_xyz(roof_x, 1.72, z)
                .with_rotation(Quat::from_rotation_z(tilt))
                .with_scale(Vec3::new(0.96, 0.18, 1.48)),
        ));
    }
    cube(
        commands,
        art,
        Vec3::new(x + 0.45, 2.02, z - 0.28),
        Vec3::new(0.22, 0.58, 0.22),
        art.brick.clone(),
    );
}

fn spawn_island_people(
    commands: &mut Commands,
    art: &Art,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let sphere = meshes.add(Sphere::new(0.5).mesh().uv(16, 12));
    let skin = materials.add(Color::srgb(1.0, 0.72, 0.55));
    let shirt = materials.add(Color::srgb(0.25, 0.55, 0.85));
    let dress = materials.add(Color::srgb(0.93, 0.40, 0.58));
    let hair = materials.add(Color::srgb(0.20, 0.12, 0.08));
    for (index, (x, z, outfit)) in [
        (-4.0_f32, -2.7_f32, shirt.clone()),
        (3.1_f32, -1.9_f32, dress.clone()),
        (5.0_f32, 2.8_f32, shirt.clone()),
    ]
    .into_iter()
    .enumerate()
    {
        let bob = if index == 1 { 0.08 } else { 0.0 };
        commands.spawn((
            WorldPiece,
            Mesh3d(sphere.clone()),
            MeshMaterial3d(outfit),
            Transform::from_xyz(x, 0.72 + bob, z).with_scale(Vec3::new(0.42, 0.62, 0.32)),
        ));
        commands.spawn((
            WorldPiece,
            Mesh3d(sphere.clone()),
            MeshMaterial3d(skin.clone()),
            Transform::from_xyz(x, 1.48 + bob, z).with_scale(Vec3::splat(0.42)),
        ));
        commands.spawn((
            WorldPiece,
            Mesh3d(sphere.clone()),
            MeshMaterial3d(hair.clone()),
            Transform::from_xyz(x, 1.77 + bob, z - 0.02).with_scale(Vec3::new(0.43, 0.18, 0.42)),
        ));
        for side in [-1.0_f32, 1.0] {
            commands.spawn((
                WorldPiece,
                Mesh3d(art.cube.clone()),
                MeshMaterial3d(art.dark.clone()),
                Transform::from_xyz(x + side * 0.17, 0.18, z)
                    .with_scale(Vec3::new(0.14, 0.35, 0.16)),
            ));
        }
    }
}

fn ball_radius(value: u64) -> f32 {
    (0.68 + (value.max(2) as f32).log2() * 0.065).min(1.48)
}

fn spawn_digit_segments(
    parent: &mut ChildSpawnerCommands,
    value: &str,
    cube: &Handle<Mesh>,
    white: &Handle<StandardMaterial>,
    radius: f32,
) {
    let patterns = [
        ('0', "abcdef"),
        ('1', "bc"),
        ('2', "abdeg"),
        ('3', "abcdg"),
        ('4', "bcfg"),
        ('5', "acdfg"),
        ('6', "acdefg"),
        ('7', "abc"),
        ('8', "abcdefg"),
        ('9', "abcdfg"),
    ];
    let digits: Vec<char> = value.chars().collect();
    let spacing = 0.48_f32.min(1.65 / digits.len().max(1) as f32);
    let start = -spacing * (digits.len() as f32 - 1.) * 0.5;
    for (index, digit) in digits.iter().enumerate() {
        let Some((_, segments)) = patterns.iter().find(|(ch, _)| ch == digit) else {
            continue;
        };
        let x = start + index as f32 * spacing;
        for segment in segments.chars() {
            let (offset, scale) = match segment {
                'a' => (
                    Vec3::new(x, 0.29, radius * 1.02),
                    Vec3::new(0.34, 0.075, 0.06),
                ),
                'b' => (
                    Vec3::new(x + 0.18, 0.14, radius * 1.02),
                    Vec3::new(0.075, 0.29, 0.06),
                ),
                'c' => (
                    Vec3::new(x + 0.18, -0.14, radius * 1.02),
                    Vec3::new(0.075, 0.29, 0.06),
                ),
                'd' => (
                    Vec3::new(x, -0.29, radius * 1.02),
                    Vec3::new(0.34, 0.075, 0.06),
                ),
                'e' => (
                    Vec3::new(x - 0.18, -0.14, radius * 1.02),
                    Vec3::new(0.075, 0.29, 0.06),
                ),
                'f' => (
                    Vec3::new(x - 0.18, 0.14, radius * 1.02),
                    Vec3::new(0.075, 0.29, 0.06),
                ),
                'g' => (
                    Vec3::new(x, 0., radius * 1.02),
                    Vec3::new(0.34, 0.075, 0.06),
                ),
                _ => continue,
            };
            parent.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(white.clone()),
                Transform::from_translation(offset)
                    .with_rotation(Quat::from_rotation_x(-0.42))
                    .with_scale(scale),
            ));
        }
    }
}

fn spawn_ball_scene(
    commands: &mut Commands,
    ball: &BallGame,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let road = materials.add(StandardMaterial {
        base_color: Color::srgb(0.98, 0.87, 0.65),
        perceptual_roughness: 0.82,
        ..default()
    });
    let lane_line = materials.add(Color::srgb(1.0, 0.98, 0.88));
    let road_edge = materials.add(Color::srgb(0.95, 0.63, 0.27));
    let lane_materials = [
        materials.add(Color::srgb(0.97, 0.58, 0.50)),
        materials.add(Color::srgb(1.0, 0.84, 0.51)),
        materials.add(Color::srgb(0.36, 0.69, 0.95)),
    ];
    let turf = materials.add(Color::srgb(0.55, 0.82, 0.55));
    let ball_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.98, 0.54, 0.12),
        emissive: LinearRgba::new(0.12, 0.035, 0.005, 1.),
        ..default()
    });
    let number_mats = [
        materials.add(Color::srgb(0.26, 0.68, 0.92)),
        materials.add(Color::srgb(0.42, 0.78, 0.55)),
        materials.add(Color::srgb(0.60, 0.42, 0.84)),
    ];
    let trap_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.88, 0.20, 0.18),
        emissive: LinearRgba::new(0.22, 0.015, 0.01, 1.),
        ..default()
    });
    let cube = meshes.add(Cuboid::new(1., 1., 1.));
    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(0.7, 0.7, 0.7, 1.),
        unlit: true,
        ..default()
    });
    let road_piece =
        |commands: &mut Commands, pos: Vec3, scale: Vec3, material: Handle<StandardMaterial>| {
            commands.spawn((
                WorldPiece,
                Mesh3d(cube.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(pos).with_scale(scale),
            ));
        };
    road_piece(
        commands,
        Vec3::new(0., -0.72, -28.),
        Vec3::new(9.5, 0.35, 78.),
        road.clone(),
    );
    for (x, material) in [-2.5, 0., 2.5].into_iter().zip(lane_materials) {
        road_piece(
            commands,
            Vec3::new(x, -0.515, -28.),
            Vec3::new(2.42, 0.035, 78.),
            material,
        );
    }
    for x in [-7.8, 7.8] {
        road_piece(
            commands,
            Vec3::new(x, -0.88, -28.),
            Vec3::new(5.8, 0.16, 78.),
            turf.clone(),
        );
    }
    for x in [-1.25, 1.25] {
        road_piece(
            commands,
            Vec3::new(x, -0.45, -28.),
            Vec3::new(0.06, 0.05, 78.),
            lane_line.clone(),
        );
    }
    road_piece(
        commands,
        Vec3::new(-4.8, -0.25, -28.),
        Vec3::new(0.18, 0.40, 78.),
        road_edge.clone(),
    );
    road_piece(
        commands,
        Vec3::new(4.8, -0.25, -28.),
        Vec3::new(0.18, 0.40, 78.),
        road_edge.clone(),
    );
    // The sky-bridge concept uses chunky candy-colored guard rails, little
    // pennants and floating castle isles. Keep them outside the three lanes
    // so these details enrich the scene without changing collision gameplay.
    let rail_gold = materials.add(Color::srgb(0.98, 0.70, 0.27));
    let rail_cream = materials.add(Color::srgb(1.0, 0.92, 0.72));
    let rail_coral = materials.add(Color::srgb(0.97, 0.47, 0.38));
    let rail_blue = materials.add(Color::srgb(0.33, 0.65, 0.93));
    let rail_mint = materials.add(Color::srgb(0.39, 0.78, 0.64));
    let side_rail = materials.add(Color::srgb(0.96, 0.81, 0.57));
    for side in [-1.0_f32, 1.0] {
        road_piece(
            commands,
            Vec3::new(side * 4.62, 0.18, -28.),
            Vec3::new(0.18, 0.22, 78.),
            side_rail.clone(),
        );
        for index in 0..18 {
            let z = 9.0 - index as f32 * 4.35;
            let post_color = match index % 3 {
                0 => rail_coral.clone(),
                1 => rail_blue.clone(),
                _ => rail_mint.clone(),
            };
            road_piece(
                commands,
                Vec3::new(side * 4.62, -0.08, z),
                Vec3::new(0.30, 0.83, 0.34),
                post_color,
            );
            road_piece(
                commands,
                Vec3::new(side * 4.62, 0.40, z),
                Vec3::new(0.48, 0.13, 0.48),
                rail_gold.clone(),
            );
        }
    }
    let island_rock = materials.add(Color::srgb(0.43, 0.59, 0.72));
    let island_grass = materials.add(Color::srgb(0.43, 0.76, 0.49));
    let tower_wall = rail_cream.clone();
    let tower_roof = materials.add(Color::srgb(0.90, 0.35, 0.27));
    let rock_mesh = meshes.add(Sphere::new(1.).mesh().uv(20, 12));
    let grass_mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let tower_mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let roof_mesh = meshes.add(Sphere::new(0.5).mesh().uv(12, 8));
    for (index, z) in [-17.0_f32, -39.0, -61.0].into_iter().enumerate() {
        for side in [-1.0_f32, 1.0] {
            let x = side * (9.0 + (index % 2) as f32 * 1.8);
            commands.spawn((
                WorldPiece,
                Mesh3d(rock_mesh.clone()),
                MeshMaterial3d(island_rock.clone()),
                Transform::from_xyz(x, -0.30, z).with_scale(Vec3::new(3.0, 0.95, 4.2)),
            ));
            commands.spawn((
                WorldPiece,
                Mesh3d(grass_mesh.clone()),
                MeshMaterial3d(island_grass.clone()),
                Transform::from_xyz(x, 0.13, z).with_scale(Vec3::new(3.3, 0.25, 4.4)),
            ));
            commands.spawn((
                WorldPiece,
                Mesh3d(tower_mesh.clone()),
                MeshMaterial3d(tower_wall.clone()),
                Transform::from_xyz(x, 1.10, z).with_scale(Vec3::new(0.78, 1.75, 0.78)),
            ));
            commands.spawn((
                WorldPiece,
                Mesh3d(roof_mesh.clone()),
                MeshMaterial3d(tower_roof.clone()),
                Transform::from_xyz(x, 2.18, z).with_scale(Vec3::new(1.1, 0.72, 1.1)),
            ));
            road_piece(
                commands,
                Vec3::new(x + side * 0.72, 1.38, z - 0.08),
                Vec3::new(0.08, 1.15, 0.08),
                rail_gold.clone(),
            );
            road_piece(
                commands,
                Vec3::new(x + side * 0.92, 1.72, z - 0.08),
                Vec3::new(0.52, 0.30, 0.06),
                if index % 2 == 0 {
                    rail_coral.clone()
                } else {
                    rail_blue.clone()
                },
            );
            road_piece(
                commands,
                Vec3::new(x, 0.29, z),
                Vec3::new(1.1, 0.08, 1.1),
                rail_gold.clone(),
            );
        }
    }
    let player_x = (ball.lane as f32 - 1.) * 2.5;
    let player_radius = ball_radius(ball.number);
    let player_sphere = meshes.add(Sphere::new(player_radius).mesh().uv(24, 16));
    commands
        .spawn((
            WorldPiece,
            BallPlayer(player_radius),
            Mesh3d(player_sphere),
            MeshMaterial3d(ball_mat),
            Transform::from_xyz(player_x, player_radius, 3.0),
        ))
        .with_children(|p| {
            spawn_digit_segments(p, &ball.number.to_string(), &cube, &white, player_radius);
        });
    for (offset, obstacle) in ball.track.iter().skip(ball.next).take(8).enumerate() {
        let x = (obstacle.lane as f32 - 1.) * 2.5;
        let z = -3.5 - (offset / 3) as f32 * 7.0 + ball.travel * 7.0;
        let (material, text, radius) = match obstacle.kind {
            BallKind::Number(value) => (
                number_mats[obstacle.lane as usize].clone(),
                value.to_string(),
                ball_radius(value),
            ),
            BallKind::Trap => (trap_mat.clone(), "".into(), 0.78),
        };
        let obstacle_sphere = meshes.add(Sphere::new(radius).mesh().uv(24, 16));
        commands
            .spawn((
                WorldPiece,
                BallObstacleScene(offset, radius),
                Mesh3d(obstacle_sphere),
                MeshMaterial3d(material),
                Transform::from_xyz(x, radius, z),
            ))
            .with_children(|p| {
                if text.is_empty() {
                    p.spawn((
                        Mesh3d(cube.clone()),
                        MeshMaterial3d(white.clone()),
                        Transform::from_xyz(0., 0., radius * 1.02)
                            .with_rotation(Quat::from_rotation_x(-0.42))
                            .with_scale(Vec3::new(0.10, 0.45, 0.04)),
                    ));
                } else {
                    spawn_digit_segments(p, &text, &cube, &white, radius);
                }
            });
    }
}

fn animate_ball_scene(
    time: Res<Time>,
    s: Res<Session>,
    mut player: Query<
        (&mut Transform, &BallPlayer),
        (With<BallPlayer>, Without<BallObstacleScene>),
    >,
    mut obstacles: Query<(&mut Transform, &BallObstacleScene), Without<BallPlayer>>,
) {
    if s.mode != GameMode::Ball2048 {
        return;
    }
    let t = time.elapsed_secs();
    let px = (s.ball.lane as f32 - 1.) * 2.5;
    for (mut transform, player_ball) in &mut player {
        transform.translation.x = px;
        transform.translation.y = player_ball.0
            + if s.ball.running {
                (t * 9.).sin().abs() * 0.10
            } else {
                0.
            };
        transform.rotation = Quat::from_rotation_x(-t * 3.0 * if s.ball.running { 1. } else { 0. });
    }
    for (mut transform, obstacle) in &mut obstacles {
        transform.translation.z = -3.5 - (obstacle.0 / 3) as f32 * 7.0 + s.ball.travel * 7.0;
        transform.translation.y = obstacle.1 + (t * 4. + obstacle.0 as f32).sin() * 0.04;
    }
}

fn animate_robot(s: Res<Session>, time: Res<Time>, mut robots: Query<&mut Transform, With<Robot>>) {
    let Ok(mut t) = robots.single_mut() else {
        return;
    };
    let target = Vec3::new(
        s.work.world.robot[0] as f32,
        0.50,
        s.work.world.robot[1] as f32,
    );
    let alpha = (time.delta_secs() * 12.).min(1.);
    t.translation = t.translation.lerp(target, alpha);
    t.rotation = t.rotation.slerp(
        Quat::from_rotation_y(-(s.work.world.facing as f32) * std::f32::consts::FRAC_PI_2),
        alpha,
    );
}

fn sync_island_robot_billboard(
    s: Res<Session>,
    robots: Query<&Transform, (With<Robot>, Without<IslandRobotBillboard>)>,
    cameras: Query<&Transform, (With<GameCamera>, Without<IslandRobotBillboard>)>,
    mut billboards: Query<(&mut Transform, &mut Visibility), With<IslandRobotBillboard>>,
) {
    let Ok((mut billboard, mut visibility)) = billboards.single_mut() else {
        return;
    };
    let illustrated =
        s.mode == GameMode::Island && !s.island_3d_view && island_base_intact(&s.work);
    *visibility = if illustrated {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    if !illustrated {
        return;
    }
    let (Ok(robot), Ok(camera)) = (robots.single(), cameras.single()) else {
        return;
    };
    billboard.translation = robot.translation + Vec3::new(0., 0.62, 0.);
    billboard.rotation = camera.rotation;
    billboard.scale = Vec3::new(2.8, 2.8, 1.);
}

fn animate_crystals(time: Res<Time>, mut crystals: Query<(&mut Transform, &CrystalSparkle)>) {
    for (mut transform, sparkle) in &mut crystals {
        transform.translation.y = 1.18 + (time.elapsed_secs() * 3. + sparkle.0).sin() * 0.12;
        transform.rotate_y(time.delta_secs() * 1.8);
    }
}

fn tick(time: Res<Time>, mut s: ResMut<Session>) {
    // The library owns its scroll state.  Letting the game underneath keep
    // animating marked the UI dirty and rebuilt the card list every few frames,
    // which snapped a phone swipe back to the top on release.
    if s.library_open || s.help_open {
        return;
    }
    s.cheer = (s.cheer - time.delta_secs()).max(0.);
    if s.mode == GameMode::Math || s.mode == GameMode::Literacy || mission_index(s.mode).is_some() {
        return;
    }
    if s.mode == GameMode::FruitArcade {
        if s.arcade.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::Slots {
        if s.slots.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::Fortune {
        if s.fortune.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::MusicFactory {
        if s.piano.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::PixelAssault {
        if s.assault.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::MushroomRun {
        if s.platform.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::TankBattle {
        if s.tank.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::Breakout {
        if s.breakout.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::ElementLab {
        if s.elements.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::LightPath {
        if s.light_path.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::GardenDefense {
        if s.garden.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::SkyRaid {
        if s.sky.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::BubbleIsland {
        if s.bubble.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::BubbleBomber {
        if s.bomber.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::FruitNinja {
        if s.fruit.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::Fishing {
        if s.fishing.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::HoneyHunt {
        if s.honey.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::MoonFestival {
        if s.moon.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::Snake {
        if s.snake.advance(time.delta_secs()) {
            s.ui_dirty = true;
        }
        return;
    }
    if s.mode == GameMode::Tetris {
        if s.tetris.advance(time.delta_secs()) {
            s.ui_dirty = true;
            s.persist_tetris();
        }
        return;
    }
    if s.mode == GameMode::Ball2048 && s.ball.running {
        if let Some(message) = s.ball.advance(time.delta_secs()) {
            s.message = message;
            s.ui_dirty = true;
            s.world_dirty = true;
            s.persist_sidecars();
        }
        return;
    }
    if s.runner.as_ref().is_some_and(|r| r.running) {
        s.clock += time.delta_secs();
        if s.clock >= 0.8 {
            s.clock = 0.;
            s.step();
        }
    }
}

fn piano_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.help_open || s.library_open || s.mode != GameMode::MusicFactory {
        return;
    }
    for (key, note) in [
        (KeyCode::KeyA, 0),
        (KeyCode::KeyS, 1),
        (KeyCode::KeyD, 2),
        (KeyCode::KeyF, 3),
        (KeyCode::KeyG, 4),
        (KeyCode::KeyH, 5),
        (KeyCode::KeyJ, 6),
    ] {
        if keys.just_pressed(key) {
            s.piano.hit(note);
            s.ui_dirty = true;
            break;
        }
    }
}

fn tetris_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open || s.help_open || s.mode != GameMode::Tetris {
        return;
    }
    let mut changed = false;
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        changed = s.tetris.move_by(-1, 0);
    }
    if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        changed |= s.tetris.move_by(1, 0);
    }
    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
        s.tetris.rotate();
        changed = true;
    }
    if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
        s.tetris.soft_drop();
        changed = true;
    }
    if keys.just_pressed(KeyCode::Space) {
        s.tetris.hard_drop();
        changed = true;
    }
    if changed {
        s.ui_dirty = true;
        s.persist_tetris();
    }
}

// The UI is rebuilt after each move, so track the physical touch rather than
// relying on a button's Interaction state. This keeps a held ↓ button falling
// smoothly even as the board refreshes underneath the finger.
fn tetris_hold_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    controls: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut hold: ResMut<TetrisHold>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::Tetris || s.tetris.failed {
        *hold = TetrisHold::default();
        return;
    }
    let mut falling = keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown);
    if let Ok(window) = windows.single() {
        let physical = |point: Vec2| point * window.scale_factor();
        let held_points: Vec<Vec2> = touches
            .iter()
            .map(|touch| physical(touch.position()))
            .chain(
                mouse
                    .pressed(MouseButton::Left)
                    .then(|| window.cursor_position())
                    .flatten()
                    .map(physical),
            )
            .collect();
        for (action, node, transform) in &controls {
            if matches!(action, Action::TetrisDown) {
                let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
                falling |= held_points.iter().any(|point| rect.contains(*point));
            }
        }
    }
    if !falling {
        *hold = TetrisHold::default();
        return;
    }
    if !hold.active {
        // The tap action supplies the first step. Begin repeat shortly after,
        // then fall fast enough to make holding feel distinct from a normal drop.
        hold.active = true;
        hold.repeat_in = 0.22;
        return;
    }
    hold.repeat_in -= time.delta_secs();
    if hold.repeat_in <= 0. {
        s.tetris.soft_drop();
        hold.repeat_in = 0.055;
        s.ui_dirty = true;
        s.persist_tetris();
    }
}

fn assault_key_input(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open || s.mode != GameMode::PixelAssault {
        return;
    }
    let left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    let direction = if left == right {
        0.
    } else if left {
        -1.
    } else {
        1.
    };
    if direction == 0. {
        s.assault.stop_player();
    } else {
        s.assault.move_player(direction, time.delta_secs());
    }
    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
        s.assault.jump();
    }
    if keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::KeyJ) {
        s.assault.fire();
    }
    if direction != 0. || keys.get_just_pressed().next().is_some() {
        s.ui_dirty = true;
    }
}

fn platform_key_input(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open || s.help_open || s.mode != GameMode::MushroomRun {
        return;
    }
    let mut changed = false;
    let left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    if left != right {
        changed |= s
            .platform
            .move_player(if right { 1. } else { -1. }, time.delta_secs());
    }
    if keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::Space)
    {
        s.platform.jump();
        changed = true;
    }
    if changed {
        s.ui_dirty = true;
    }
}

// Do not depend on `Interaction::Pressed` here. The game redraws its UI while
// walking, and Bevy correctly changes a rebuilt held button into `Hovered`.
// Raw hit-testing keeps each finger working through those redraws, including a
// second finger used to jump while the first keeps walking.
fn platform_touch_hold(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    controls: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::MushroomRun {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let physical = |point: Vec2| point * window.scale_factor();
    let held_points: Vec<Vec2> = touches
        .iter()
        .map(|touch| physical(touch.position()))
        .chain(
            mouse
                .pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
                .map(physical),
        )
        .collect();
    let pressed_points: Vec<Vec2> = touches
        .iter_just_pressed()
        .map(|touch| physical(touch.position()))
        .chain(
            mouse
                .just_pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
                .map(physical),
        )
        .collect();
    let mut left = false;
    let mut right = false;
    let mut jump = false;
    for (action, node, transform) in &controls {
        let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
        match action {
            Action::PlatformLeft => left |= held_points.iter().any(|point| rect.contains(*point)),
            Action::PlatformRight => right |= held_points.iter().any(|point| rect.contains(*point)),
            Action::PlatformJump => {
                jump |= pressed_points.iter().any(|point| rect.contains(*point))
            }
            _ => {}
        }
    }
    let mut changed = false;
    if left != right {
        changed |= s
            .platform
            .move_player(if right { 1. } else { -1. }, time.delta_secs());
    }
    if jump {
        s.platform.jump();
        changed = true;
    }
    if changed {
        s.ui_dirty = true;
    }
}

fn assault_touch_hold(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    controls: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.mode != GameMode::PixelAssault {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let physical = |point: Vec2| point * window.scale_factor();
    let held_points: Vec<Vec2> = touches
        .iter()
        .map(|touch| physical(touch.position()))
        .chain(
            mouse
                .pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
                .map(physical),
        )
        .collect();
    let mut left = false;
    let mut right = false;
    let mut fire = false;
    for (action, node, transform) in &controls {
        let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
        let held = held_points.iter().any(|point| rect.contains(*point));
        match action {
            Action::AssaultLeft => left |= held,
            Action::AssaultRight => right |= held,
            Action::AssaultFire => fire |= held,
            _ => {}
        }
    }
    let direction = if left == right {
        0.
    } else if left {
        -1.
    } else {
        1.
    };
    if direction == 0. {
        s.assault.stop_player();
    } else {
        s.assault.move_player(direction, time.delta_secs());
    }
    if fire {
        s.assault.fire();
    }
    if direction != 0. || fire {
        s.ui_dirty = true;
    }
}

fn tank_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open || s.mode != GameMode::TankBattle {
        return;
    }
    let mut moved = false;
    for (key, dx, dy) in [
        (KeyCode::KeyW, 0, -1),
        (KeyCode::ArrowUp, 0, -1),
        (KeyCode::KeyS, 0, 1),
        (KeyCode::ArrowDown, 0, 1),
        (KeyCode::KeyA, -1, 0),
        (KeyCode::ArrowLeft, -1, 0),
        (KeyCode::KeyD, 1, 0),
        (KeyCode::ArrowRight, 1, 0),
    ] {
        if keys.just_pressed(key) {
            s.tank.move_player(dx, dy);
            moved = true;
            break;
        }
    }
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::KeyJ) {
        s.tank.fire();
        moved = true;
    }
    if moved {
        s.ui_dirty = true;
    }
}

// Capture the finger, not a rebuilt UI button. A second finger can fire.
fn tank_touch_hold(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    controls: Query<(
        &ComputedNode,
        &GlobalTransform,
        Option<&TankStick>,
        Option<&TankFirePad>,
    )>,
    mut hold: ResMut<TankHold>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::TankBattle || s.tank.failed {
        *hold = TankHold::default();
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let physical = |point: Vec2| point * window.scale_factor();
    let mut firing = false;
    for (node, transform, stick, fire) in &controls {
        if stick.is_none() && fire.is_none() {
            continue;
        }
        let center = transform.translation().truncate();
        let radius = node.size().x.min(node.size().y) / 2.;
        let inside = |point: Vec2| physical(point).distance(center) <= radius;
        if stick.is_some() && hold.pointer.is_none() {
            let touch = touches.iter_just_pressed().find(|t| inside(t.position()));
            let pointer = touch.map(|t| Some(t.id())).or_else(|| {
                (mouse.just_pressed(MouseButton::Left)
                    && window.cursor_position().is_some_and(inside))
                .then_some(None)
            });
            if let Some(pointer) = pointer {
                hold.pointer = Some(pointer);
                hold.center = center;
                hold.radius = radius;
            }
        }
        if fire.is_some() {
            firing = touches
                .iter()
                .any(|t| hold.pointer != Some(Some(t.id())) && inside(t.position()))
                || (hold.pointer != Some(None)
                    && mouse.pressed(MouseButton::Left)
                    && window.cursor_position().is_some_and(inside));
        }
    }
    let point = match hold.pointer {
        Some(Some(id)) => touches.get_pressed(id).map(|t| physical(t.position())),
        Some(None) if mouse.pressed(MouseButton::Left) => window.cursor_position().map(physical),
        _ => None,
    };
    let direction = if let Some(point) = point {
        let delta = point - hold.center;
        hold.knob = delta.clamp_length_max(hold.radius * 0.6) / hold.radius.max(1.);
        tank_stick_direction(delta, hold.radius)
    } else {
        hold.pointer = None;
        hold.knob = Vec2::ZERO;
        None
    };
    if let Some(direction) = direction {
        hold.repeat_in -= time.delta_secs();
        if hold.direction != Some(direction) || hold.repeat_in <= 0. {
            s.tank.move_player(direction.0, direction.1);
            hold.repeat_in = 0.18;
            s.ui_dirty = true;
        }
    } else {
        hold.repeat_in = 0.;
    }
    hold.direction = direction;
    hold.fire_in = (hold.fire_in - time.delta_secs()).max(0.);
    if firing && hold.fire_in <= 0. {
        s.tank.fire();
        hold.fire_in = 0.30;
        s.ui_dirty = true;
    }
    if !firing {
        hold.fire_in = 0.;
    }
}
fn animate_tank_stick(hold: Res<TankHold>, mut knobs: Query<&mut Node, With<TankKnob>>) {
    for mut node in &mut knobs {
        node.left = Val::Percent(32. + hold.knob.x * 50.);
        node.top = Val::Percent(32. + hold.knob.y * 50.);
    }
}

fn sky_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.library_open || s.mode != GameMode::SkyRaid {
        return;
    }
    let mut changed = false;
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        s.sky.move_x(-0.08);
        changed = true;
    }
    if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        s.sky.move_x(0.08);
        changed = true;
    }
    if changed {
        s.ui_dirty = true;
    }
}

// Touching the left or right side of the ship is intentionally the steering
// control.  Holding a side repeats movement, which makes a phone feel closer to
// an arcade slider than two detached on-screen buttons.
fn sky_screen_input(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut hold: ResMut<SkyHold>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.mode != GameMode::SkyRaid || s.sky.failed {
        hold.repeat_in = 0.;
        return;
    }
    let active = mouse.pressed(MouseButton::Left) || touches.iter().next().is_some();
    if !active {
        hold.repeat_in = 0.;
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let position = touches
        .iter()
        .next()
        .map(|touch| touch.position())
        .or_else(|| window.cursor_position());
    let Some(position) = position else {
        return;
    };
    // Ignore the upper half, where an accidental touch is more likely to be
    // looking at an enemy.  The lower play area surrounds the player's ship.
    if position.y < window.height() * 0.58 {
        hold.repeat_in = 0.;
        return;
    }
    let player_x = s.sky.x * window.width();
    let dead_zone = (window.width() * 0.045).max(18.);
    let direction = if position.x < player_x - dead_zone {
        -1.
    } else if position.x > player_x + dead_zone {
        1.
    } else {
        hold.repeat_in = 0.;
        return;
    };
    hold.repeat_in -= time.delta_secs();
    if hold.repeat_in <= 0. {
        s.sky.move_x(direction * 0.052);
        hold.repeat_in = 0.085;
        s.ui_dirty = true;
    }
}

fn bubble_screen_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    boards: Query<(&ComputedNode, &GlobalTransform), With<BubbleBoard>>,
    mut hold: ResMut<BubbleHold>,
    mut s: ResMut<Session>,
) {
    if s.help_open || s.library_open || s.mode != GameMode::BubbleIsland || s.bubble.failed {
        hold.0 = false;
        return;
    }
    if touches.iter_just_canceled().next().is_some() {
        hold.0 = false;
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((node, transform)) = boards.single() else {
        return;
    };
    let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
    let active = mouse.pressed(MouseButton::Left) || touches.iter().next().is_some();
    let point = touches
        .iter()
        .next()
        .map(|t| t.position())
        .or_else(|| window.cursor_position());
    if active {
        if let Some(point) = point {
            let physical = point * window.scale_factor();
            let began = mouse.just_pressed(MouseButton::Left)
                || touches.iter_just_pressed().next().is_some();
            if began {
                hold.0 = rect.contains(physical) && s.bubble.shot.is_none();
            }
            if hold.0 {
                s.bubble.aim = ((physical - rect.min) / rect.size())
                    .clamp(Vec2::new(0.06, 0.06), Vec2::new(0.94, 0.73));
                s.ui_dirty = true;
            }
        }
    } else if hold.0 {
        hold.0 = false;
        let aim = s.bubble.aim;
        s.bubble.shoot(aim);
        s.ui_dirty = true;
    }
}

fn bomber_key_input(keys: Res<ButtonInput<KeyCode>>, mut s: ResMut<Session>) {
    if s.help_open || s.library_open || s.mode != GameMode::BubbleBomber {
        return;
    }
    let mut changed = false;
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyP) {
        if !s.bomber.failed && !s.bomber.won {
            s.bomber.paused = !s.bomber.paused;
        }
        s.ui_dirty = true;
        return;
    }
    for (key, dx, dy) in [
        (KeyCode::KeyW, 0, -1),
        (KeyCode::ArrowUp, 0, -1),
        (KeyCode::KeyS, 0, 1),
        (KeyCode::ArrowDown, 0, 1),
        (KeyCode::KeyA, -1, 0),
        (KeyCode::ArrowLeft, -1, 0),
        (KeyCode::KeyD, 1, 0),
        (KeyCode::ArrowRight, 1, 0),
    ] {
        if keys.just_pressed(key) {
            s.bomber.move_player(dx, dy);
            changed = true;
            break;
        }
    }
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::KeyJ) {
        s.bomber.place_bomb();
        changed = true;
    }
    if changed {
        s.ui_dirty = true;
    }
}

fn bomber_hold_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    controls: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut hold: ResMut<BomberHold>,
    pads: Query<(), With<BomberJoystick>>,
    mut s: ResMut<Session>,
) {
    if s.help_open
        || s.library_open
        || s.mode != GameMode::BubbleBomber
        || s.bomber.failed
        || s.bomber.paused
        || s.bomber.won
    {
        *hold = BomberHold::default();
        return;
    }
    if !pads.is_empty() {
        return;
    }
    let mut direction = [
        (KeyCode::ArrowUp, KeyCode::KeyW, (0, -1)),
        (KeyCode::ArrowDown, KeyCode::KeyS, (0, 1)),
        (KeyCode::ArrowLeft, KeyCode::KeyA, (-1, 0)),
        (KeyCode::ArrowRight, KeyCode::KeyD, (1, 0)),
    ]
    .into_iter()
    .find_map(|(a, b, delta)| (keys.pressed(a) || keys.pressed(b)).then_some(delta));
    if let Ok(window) = windows.single() {
        let point = touches.iter().next().map(|t| t.position()).or_else(|| {
            mouse
                .pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
        });
        if let Some(point) = point {
            for (action, node, transform) in &controls {
                if let Action::BomberMove(dx, dy) = action {
                    if Rect::from_center_size(transform.translation().truncate(), node.size())
                        .contains(point * window.scale_factor())
                    {
                        direction = Some((*dx, *dy));
                        break;
                    }
                }
            }
        }
    }
    if direction != hold.direction {
        hold.direction = direction;
        hold.remaining = 0.22;
    } else if let Some((dx, dy)) = direction {
        hold.remaining -= time.delta_secs();
        if hold.remaining <= 0. {
            hold.remaining = 0.16;
            s.bomber.move_player(dx, dy);
            s.ui_dirty = true;
        }
    }
}

fn bomber_joystick_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pads: Query<(&ComputedNode, &GlobalTransform), With<BomberJoystick>>,
    actions: Query<(&ComputedNode, &GlobalTransform), With<BomberTouchPlace>>,
    mut knobs: Query<&mut Node, With<BomberJoystickKnob>>,
    mut hold: ResMut<BomberHold>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::BubbleBomber
        || s.library_open
        || s.help_open
        || s.bomber.failed
        || s.bomber.paused
        || s.bomber.won
    {
        *hold = BomberHold::default();
        for mut knob in &mut knobs {
            knob.left = Val::Percent(27.);
            knob.top = Val::Percent(27.);
        }
        return;
    }
    let (Ok(window), Ok((node, transform))) = (windows.single(), pads.single()) else {
        return;
    };
    let factor = window.scale_factor();
    let center = transform.translation().truncate();
    let radius = node.size().x * 0.5;
    if hold
        .touch_id
        .is_some_and(|id| touches.get_pressed(id).is_none())
    {
        hold.touch_id = None;
    }
    if hold.touch_id.is_none() {
        hold.touch_id = touches
            .iter_just_pressed()
            .find(|t| (t.position() * factor - center).length() <= radius)
            .map(|t| t.id());
    }
    if mouse.just_pressed(MouseButton::Left) && touches.iter().next().is_none() {
        hold.mouse_active = window
            .cursor_position()
            .is_some_and(|p| (p * factor - center).length() <= radius);
    }
    if !mouse.pressed(MouseButton::Left) {
        hold.mouse_active = false;
    }
    let point = hold
        .touch_id
        .and_then(|id| touches.get_pressed(id))
        .map(|t| t.position())
        .or_else(|| {
            if hold.mouse_active {
                window.cursor_position()
            } else {
                None
            }
        });
    let offset = point.map(|p| p * factor - center).unwrap_or(Vec2::ZERO);
    let visual = offset.clamp_length_max(radius * 0.54) / radius.max(1.) * 50.;
    for mut knob in &mut knobs {
        knob.left = Val::Percent(27. + visual.x);
        knob.top = Val::Percent(27. + visual.y);
    }
    let direction = bomber_joystick_direction(offset, radius).or_else(|| {
        [
            (KeyCode::ArrowUp, KeyCode::KeyW, (0, -1)),
            (KeyCode::ArrowDown, KeyCode::KeyS, (0, 1)),
            (KeyCode::ArrowLeft, KeyCode::KeyA, (-1, 0)),
            (KeyCode::ArrowRight, KeyCode::KeyD, (1, 0)),
        ]
        .into_iter()
        .find_map(|(a, b, d)| (keys.pressed(a) || keys.pressed(b)).then_some(d))
    });
    if direction != hold.direction {
        hold.direction = direction;
        hold.remaining = 0.;
    }
    if let Some((dx, dy)) = direction {
        hold.remaining -= time.delta_secs();
        if hold.remaining <= 0. {
            // Keyboard's initial step is handled by bomber_key_input.
            let keyboard_initial = point.is_none() && keys.get_just_pressed().next().is_some();
            if !keyboard_initial {
                s.bomber.move_player(dx, dy);
                s.ui_dirty = true;
            }
            hold.remaining = 0.16;
        }
    }
    for (node, transform) in &actions {
        let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
        let touch_press = touches
            .iter_just_pressed()
            .any(|t| Some(t.id()) != hold.touch_id && rect.contains(t.position() * factor));
        let mouse_press = touches.iter().next().is_none()
            && mouse.just_pressed(MouseButton::Left)
            && window
                .cursor_position()
                .is_some_and(|p| rect.contains(p * factor));
        if touch_press || mouse_press {
            s.bomber.place_bomb();
            s.ui_dirty = true;
        }
    }
}

fn bomber_board_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    boards: Query<(&BomberBoard, &ComputedNode, &GlobalTransform)>,
    controls: Query<(&ComputedNode, &GlobalTransform), With<Button>>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::BubbleBomber
        || s.library_open
        || s.help_open
        || s.bomber.failed
        || s.bomber.paused
        || s.bomber.won
    {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let point = touches
        .iter_just_pressed()
        .next()
        .map(|t| t.position())
        .or_else(|| {
            mouse
                .just_pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
        });
    let Some(point) = point else {
        return;
    };
    let Ok((board, node, transform)) = boards.single() else {
        return;
    };
    let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
    let point = point * window.scale_factor();
    if controls.iter().any(|(node, transform)| {
        Rect::from_center_size(transform.translation().truncate(), node.size()).contains(point)
    }) {
        return;
    }
    if !rect.contains(point) {
        return;
    }
    let grid = ((point - rect.min) / rect.size())
        * Vec2::new(board.visible_columns as f32, BOMBER_ROWS as f32);
    let target = (
        board.first_column + grid.x.floor() as i8,
        grid.y.floor() as i8,
    );
    let delta = (target.0 - s.bomber.player.0, target.1 - s.bomber.player.1);
    if delta.0.abs() + delta.1.abs() == 1 {
        let before = s.bomber.player;
        s.bomber.move_player(delta.0, delta.1);
        if before == s.bomber.player {
            s.bomber.feedback = "这里被挡住了，点亮着的空格走。".into();
        } else {
            s.bomber.feedback = "走到了！继续点身边的亮格，或按住方向键。".into();
        }
    } else {
        s.bomber.feedback = "点身边上、下、左、右的亮格，就能走一步。".into();
    }
    s.ui_dirty = true;
}

fn fruit_screen_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut slash: ResMut<FruitSlash>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::FruitNinja || s.fruit.failed {
        slash.last = None;
        return;
    }
    let active = mouse.pressed(MouseButton::Left) || touches.iter().next().is_some();
    if !active {
        slash.last = None;
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let point = touches
        .iter()
        .next()
        .map(|touch| touch.position())
        .or_else(|| window.cursor_position());
    let Some(point) = point else {
        return;
    };
    let current = Vec2::new(point.x / window.width(), point.y / window.height());
    if let Some(last) = slash.last {
        s.fruit.slice_between(last, current);
    }
    slash.last = Some(current);
    s.ui_dirty = true;
}

fn fishing_screen_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::Fishing {
        return;
    }
    let point = touches
        .iter_just_pressed()
        .next()
        .map(|touch| touch.position())
        .or_else(|| {
            mouse
                .just_pressed(MouseButton::Left)
                .then(|| windows.single().ok()?.cursor_position())
                .flatten()
        });
    let Some(point) = point else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    s.fishing.fire_at(Vec2::new(
        point.x / window.width(),
        point.y / window.height(),
    ));
    s.ui_dirty = true;
}

fn honey_screen_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut s: ResMut<Session>,
) {
    if s.library_open || s.help_open || s.mode != GameMode::HoneyHunt {
        return;
    }
    let tapped = touches.iter_just_pressed().next().is_some()
        || (mouse.just_pressed(MouseButton::Left)
            && windows
                .single()
                .ok()
                .and_then(|window| window.cursor_position())
                .is_some());
    if tapped {
        s.honey.launch();
        s.ui_dirty = true;
    }
}

fn button_feedback(
    mut buttons: Query<(&Interaction, &ButtonTone, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, tone, mut background) in &mut buttons {
        let c = tone.0.to_srgba();
        let (factor, lift) = match interaction {
            Interaction::Pressed => (0.88, 0.),
            Interaction::Hovered => (0.92, 0.08),
            Interaction::None => (1., 0.),
        };
        background.0 = Color::srgba(
            c.red * factor + lift,
            c.green * factor + lift,
            c.blue * factor + lift,
            c.alpha,
        );
    }
}

fn buttons(
    query: Query<(&Interaction, &Action), (Changed<Interaction>, With<Button>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut gate: ResMut<PressGate>,
    mut library_drag: ResMut<LibraryDrag>,
    mut s: ResMut<Session>,
) {
    if !mouse.pressed(MouseButton::Left) && touches.iter().next().is_none() {
        gate.0 = false;
    }
    if gate.0 {
        return;
    }
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        gate.0 = true;
        if let Action::SelectGame(mode) = *action {
            if s.library_open {
                if library_drag.start.is_some() && !library_drag.dragging {
                    library_drag.pending_game = Some(mode);
                }
                // A press is only a candidate tap. Never rebuild the scrolling
                // list here: rebuilding destroys its ScrollPosition and hit state.
                continue;
            }
        }
        if s.help_open && !matches!(action, Action::CloseHelp | Action::Library) {
            continue;
        }
        match *action {
            Action::Help => s.help_open = true,
            Action::CloseHelp => s.help_open = false,
            Action::MathTools => s.math_tools = !s.math_tools,
            Action::MissionHint => {
                let task = mission_question(s.mode, s.mission_step);
                s.mission_feedback =
                    format!("{} 试试「{}」。", task.hint, task.choices[task.correct]);
            }
            Action::BubbleAssist => {
                if s.bubble.shot.is_none() {
                    if let Some(aim) = s.bubble.recommended_aim() {
                        s.bubble.aim = aim;
                        s.bubble.feedback = "瞄好啦！点「发射」试试这条路线。".into();
                    } else {
                        s.bubble.feedback = "暂时没有直接同色路线，试着利用墙壁反弹。".into();
                    }
                }
            }
            Action::BubbleFire => {
                let aim = s.bubble.aim;
                s.bubble.shoot(aim);
            }
            Action::MissionChoice(choice) => {
                if !s.mission_done && mission_index(s.mode).is_some() {
                    let task = mission_question(s.mode, s.mission_step);
                    if choice == task.correct {
                        let i = mission_index(s.mode).unwrap();
                        s.mission_stars[i] = s.mission_stars[i].max(s.mission_step + 1);
                        s.mission_done = true;
                        s.mission_wrong = None;
                        s.cheer = 1.4;
                        s.mission_feedback = task.success.into();
                    } else {
                        s.mission_wrong = Some(choice);
                        s.mission_feedback = task.hint.into();
                    }
                }
            }
            Action::MissionNext => {
                if s.mission_done {
                    s.mission_step = (s.mission_step + 1) % 4;
                    s.mission_done = false;
                    s.mission_wrong = None;
                    s.mission_feedback.clear();
                }
            }
            Action::PianoKey(key) => {
                if s.mode == GameMode::MusicFactory {
                    s.piano.hit(key);
                }
            }
            Action::PianoReset => {
                if s.mode == GameMode::MusicFactory {
                    s.piano.reset();
                }
            }
            Action::AssaultLeft => {
                if s.mode == GameMode::PixelAssault {
                    s.assault.move_player(-1., 0.35);
                }
            }
            Action::AssaultRight => {
                if s.mode == GameMode::PixelAssault {
                    s.assault.move_player(1., 0.35);
                }
            }
            Action::AssaultJump => {
                if s.mode == GameMode::PixelAssault {
                    s.assault.jump();
                }
            }
            Action::AssaultFire => {
                if s.mode == GameMode::PixelAssault {
                    s.assault.fire();
                }
            }
            Action::AssaultReset => {
                if s.mode == GameMode::PixelAssault {
                    s.assault.reset();
                }
            }
            // Directional platform controls are processed from raw pointers in
            // `platform_touch_hold`, so holding through a UI redraw keeps
            // moving instead of being reduced to a one-frame tap.
            Action::PlatformLeft | Action::PlatformRight => {}
            Action::PlatformJump => {
                if s.mode == GameMode::MushroomRun {
                    s.platform.jump();
                }
            }
            Action::PlatformReset => {
                if s.mode == GameMode::MushroomRun {
                    s.platform.reset();
                }
            }
            Action::ArcadeSpin => s.arcade.spin(),
            Action::ArcadeBet(i) => s.arcade.add(i),
            Action::ArcadeClear => s.arcade.clear(),
            Action::ArcadeRefill => s.arcade.refill(),
            Action::SlotsSpin => s.slots.spin(),
            Action::SlotsBet(bet) => s.slots.set_bet(bet),
            Action::SlotsRefill => s.slots.refill(),
            Action::FortuneRoll => s.fortune.roll(),
            Action::FortuneBuy => s.fortune.choose(true),
            Action::FortuneSkip => s.fortune.choose(false),
            Action::FortuneCenter => s.fortune.recenter(),
            Action::FortuneReset => s.fortune = FortuneGame::new(),
            Action::TankRetry => s.tank.retry_level(),
            Action::TankReset => {
                if s.mode == GameMode::TankBattle {
                    s.tank.reset();
                }
            }
            Action::SkyMove(dx) => {
                if s.mode == GameMode::SkyRaid {
                    s.sky.move_x(dx as f32 * 0.08);
                }
            }
            Action::SkyReset => {
                if s.mode == GameMode::SkyRaid {
                    s.sky.reset();
                }
            }
            Action::BubbleReset => {
                if s.mode == GameMode::BubbleIsland {
                    s.bubble.reset();
                }
            }
            Action::BomberMove(dx, dy) => {
                if s.mode == GameMode::BubbleBomber {
                    s.bomber.move_player(dx, dy);
                }
            }
            Action::BomberPlace => {
                if s.mode == GameMode::BubbleBomber {
                    s.bomber.place_bomb();
                }
            }
            Action::BomberReset => {
                if s.mode == GameMode::BubbleBomber {
                    s.bomber.reset();
                }
            }
            Action::BomberTogglePause => {
                if s.mode == GameMode::BubbleBomber && !s.bomber.failed && !s.bomber.won {
                    s.bomber.paused = !s.bomber.paused;
                }
            }
            Action::FruitReset => {
                if s.mode == GameMode::FruitNinja {
                    s.fruit.reset();
                }
            }
            Action::FishingReset => {
                if s.mode == GameMode::Fishing {
                    s.fishing.reset();
                }
            }
            Action::HoneyLaunch => {
                if s.mode == GameMode::HoneyHunt {
                    s.honey.launch();
                }
            }
            Action::HoneyReset => {
                if s.mode == GameMode::HoneyHunt {
                    s.honey.reset();
                }
            }
            Action::MoonShake => {
                if s.mode == GameMode::MoonFestival {
                    s.moon.shake();
                }
            }
            Action::MoonReset => {
                if s.mode == GameMode::MoonFestival {
                    s.moon.reset();
                }
            }
            Action::Courier(courier_action) => {
                if s.mode == GameMode::SpaceCourier {
                    s.courier.act(courier_action);
                    s.courier.persist();
                }
            }
            #[cfg(target_os = "macos")]
            Action::LiteracySpeak => {
                let word = s.literacy.word();
                let utterance = format!("{}。{}。", word.character, word.phrase);
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("/usr/bin/say")
                        .args(["-v", "Tingting", "-r", "140", &utterance])
                        .status();
                });
            }
            Action::LiteracyStart(unit) => {
                s.literacy.start(unit);
                s.world_dirty = true;
            }
            Action::LiteracySelect(value) => {
                let was_correct = s.literacy.checked && s.literacy.correct;
                s.literacy.select(value);
                if s.literacy.correct && !was_correct {
                    s.cheer = 1.4;
                }
                s.world_dirty = true;
            }
            Action::LiteracyCheck => {
                s.literacy.check();
                s.world_dirty = true;
            }
            Action::LiteracyNext => {
                s.literacy.next();
                s.world_dirty = true;
                if s.literacy.finished {
                    let path = save_directory().join("literacy-v1.json");
                    if let Ok(bytes) = serde_json::to_vec(&s.literacy.progress) {
                        if fs::create_dir_all(save_directory())
                            .and_then(|_| fs::write(path, bytes))
                            .is_err()
                        {
                            s.message = "学习进度暂时无法保存。".into();
                        }
                    }
                }
            }
            Action::LiteracyMap => {
                s.literacy.map = true;
                s.world_dirty = true;
            }
            Action::MathPlace(delta) => {
                s.math.place(delta);
                s.world_dirty = true;
            }
            Action::MathCheck => {
                let value = s.math.placed;
                s.math.answer(value);
                if s.math.solved {
                    s.cheer = 1.4;
                } else {
                    s.math.hinted = true;
                }
                s.world_dirty = true;
            }
            Action::MathLevels => s.math.choosing_level = true,
            Action::MathResume => s.math.choosing_level = false,
            Action::MathLevel(level) => {
                s.math.choose_level(level);
                s.world_dirty = true;
            }
            Action::MathAnswer(value) => {
                if !s.math.solved {
                    s.math.placed = value;
                    s.math.answer(value);
                    if s.math.solved {
                        s.cheer = 1.4;
                    } else {
                        s.math.hinted = true;
                    }
                    s.world_dirty = true;
                }
            }
            Action::MathHint => s.math.hinted = true,
            Action::MathNext => {
                s.math.next();
                s.world_dirty = true;
            }
            Action::MagnetModes => {
                s.help_open = false;
                s.library_open = false;
                s.magnet_modes_open = true;
            }
            Action::MagnetModesClose => {
                s.magnet_modes_open = false;
                s.library_open = true;
            }
            Action::Library => {
                s.help_open = false;
                s.magnet_modes_open = false;
                s.reset_run();
                s.library_open = true;
                s.build = false;
                s.editor_open = false;
                s.message = "选择一个想玩的小游戏吧".into();
            }
            Action::LibraryReturn => {
                s.library_open = false;
                s.magnet_modes_open = false;
                // Opening the library stops the runner and clears build mode.
                // Returning to a building game must restore editable input.
                if matches!(
                    s.mode,
                    GameMode::Island
                        | GameMode::CloudBridge
                        | GameMode::FreeBuild
                        | GameMode::CrystalGarden
                ) {
                    s.build = true;
                }
            }
            Action::SelectGame(mode) => select_game(&mut s, mode),
            Action::BallLeft => {
                if s.mode == GameMode::Ball2048 && move_ball_lane(&mut s.ball, -1) {
                    s.message = "向左换道。".into();
                }
            }
            Action::BallRight => {
                if s.mode == GameMode::Ball2048 && move_ball_lane(&mut s.ball, 1) {
                    s.message = "向右换道。".into();
                }
            }
            Action::BallStart => {
                if s.mode == GameMode::Ball2048 {
                    if s.ball.failed || s.ball.finished {
                        s.ball.reset();
                    }
                    s.ball.running = !s.ball.running;
                    s.message = if s.ball.running {
                        "球开始滚动！优先找相同数字，前四关没有陷阱。".into()
                    } else {
                        "已暂停，准备好后继续。".into()
                    };
                }
            }
            Action::BallReset => {
                if s.mode == GameMode::Ball2048 {
                    s.ball.reset();
                    s.message = "新的 2 球已经准备好。".into();
                }
            }
            Action::BreakoutLaunch => {
                if s.mode == GameMode::Breakout {
                    s.breakout.launch();
                }
            }
            Action::BreakoutToggle => {
                if s.mode == GameMode::Breakout {
                    if s.breakout.ready {
                        s.breakout.launch();
                    } else if !s.breakout.over && !s.breakout.won {
                        s.breakout.paused = !s.breakout.paused;
                    }
                }
            }
            Action::BreakoutReset => {
                if s.mode == GameMode::Breakout {
                    s.breakout.reset();
                }
            }
            Action::ElementObserve => {
                if s.mode == GameMode::ElementLab {
                    s.elements.observing = !s.elements.observing;
                }
            }
            Action::ElementPick(number) => {
                if s.mode == GameMode::ElementLab {
                    s.elements.pick(number);
                }
            }
            Action::ElementNext => {
                if s.mode == GameMode::ElementLab {
                    s.elements.next();
                }
            }
            Action::ElementReset => {
                if s.mode == GameMode::ElementLab {
                    s.elements.reset();
                }
            }
            Action::TetrisLeft => {
                if s.mode == GameMode::Tetris {
                    s.tetris.move_by(-1, 0);
                }
            }
            Action::TetrisRight => {
                if s.mode == GameMode::Tetris {
                    s.tetris.move_by(1, 0);
                }
            }
            Action::TetrisRotate => {
                if s.mode == GameMode::Tetris {
                    s.tetris.rotate();
                }
            }
            Action::TetrisDown => {
                if s.mode == GameMode::Tetris {
                    s.tetris.soft_drop();
                }
            }
            Action::TetrisDrop => {
                if s.mode == GameMode::Tetris {
                    s.tetris.hard_drop();
                }
            }
            Action::TetrisReset => {
                if s.mode == GameMode::Tetris {
                    s.tetris.reset();
                }
            }
            Action::SnakeMove(dx, dy) => {
                if s.mode == GameMode::Snake && s.snake.turn(dx, dy) {
                    s.ui_dirty = true;
                }
            }
            Action::SnakeReset => {
                if s.mode == GameMode::Snake {
                    s.snake.reset();
                }
            }
            Action::SnakeToggle => {
                if s.mode == GameMode::Snake {
                    s.snake.toggle();
                }
            }
            Action::MineReveal(cell) => {
                if s.mode == GameMode::Minesweeper {
                    s.minesweeper.reveal(cell);
                }
            }
            Action::MineToggleFlag => {
                if s.mode == GameMode::Minesweeper {
                    s.minesweeper.flag_mode = !s.minesweeper.flag_mode;
                }
            }
            Action::MineReset => {
                if s.mode == GameMode::Minesweeper {
                    s.minesweeper.reset();
                }
            }
            Action::ClassicMove(dx, dy) => {
                if s.mode == GameMode::Classic2048 {
                    s.classic_2048.move_board(dx, dy);
                }
            }
            Action::ClassicReset => {
                if s.mode == GameMode::Classic2048 {
                    s.classic_2048.reset();
                }
            }
            Action::KlotskiSelect(index) => {
                if s.mode == GameMode::Klotski {
                    s.klotski.selected = index;
                }
            }
            Action::KlotskiMove(dx, dy) => {
                if s.mode == GameMode::Klotski && s.klotski.move_selected(dx, dy) {
                    s.ui_dirty = true;
                }
            }
            Action::KlotskiReset => {
                if s.mode == GameMode::Klotski {
                    s.klotski.reset();
                }
            }
            Action::SokobanMove(dx, dy) => {
                if s.mode == GameMode::Sokoban && s.sokoban.move_player(dx, dy) {
                    s.ui_dirty = true;
                }
            }
            Action::SokobanReset => {
                if s.mode == GameMode::Sokoban {
                    s.sokoban.reset();
                }
            }
            Action::SokobanUndo => {
                if s.mode == GameMode::Sokoban {
                    s.sokoban.undo();
                }
            }
            Action::SokobanChoose => {
                if s.mode == GameMode::Sokoban {
                    s.sokoban.choosing = !s.sokoban.choosing;
                }
            }
            Action::SokobanLevel(level) => {
                if s.mode == GameMode::Sokoban {
                    s.sokoban.select_level(level);
                }
            }
            Action::SokobanNext => {
                if s.mode == GameMode::Sokoban {
                    s.sokoban.next();
                }
            }
            Action::SudokuSelect(index) => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.select(index);
                }
            }
            Action::SudokuDigit(value) => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.set_selected(value);
                }
            }
            Action::SudokuClear => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.clear_selected();
                }
            }
            Action::SudokuHint => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.hint();
                }
            }
            Action::SudokuReset => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.reset();
                }
            }
            Action::SudokuSize(size) => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.set_size(size);
                }
            }
            Action::SudokuNew => {
                if s.mode == GameMode::Sudoku {
                    s.sudoku.new_puzzle();
                }
            }
            Action::LightRotate(index) => {
                if s.mode == GameMode::LightPath {
                    s.light_path.rotate(index);
                }
            }
            Action::LightReset => {
                if s.mode == GameMode::LightPath {
                    s.light_path.reset();
                }
            }
            Action::LightNext => {
                if s.mode == GameMode::LightPath {
                    s.light_path.next_level();
                }
            }
            Action::GardenSelect(index) => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.select(index);
                }
            }
            Action::GardenStart => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.start();
                }
            }
            Action::GardenPause => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.toggle_pause();
                }
            }
            Action::GardenReset => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.reset();
                }
            }
            Action::GardenNext => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.next_stage();
                }
            }
            Action::GardenShovel => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.toggle_shovel();
                }
            }
            Action::GardenCollectAll => {
                if s.mode == GameMode::GardenDefense {
                    s.garden.collect_all();
                }
            }
            Action::Material(m) => {
                s.reset_run();
                s.material = m;
                s.erase = false;
                s.build = true;
                s.message = format!(
                    "已选择{}：点顶面向上搭，点侧面横着搭，下面可以留空。",
                    m.label()
                );
            }
            Action::Erase => {
                s.reset_run();
                s.erase = !s.erase;
                s.build = true;
                s.message = if s.erase {
                    "拆除模式：点击要拿走的方块。可以撤销。".into()
                } else {
                    format!("已回到搭建模式，当前材料：{}。", s.material.label())
                };
            }
            Action::ToggleEditor => s.editor_open = !s.editor_open,
            Action::Build => {
                s.reset_run();
                s.build = true;
                s.erase = false;
                s.message = "点击放方块，上下拖动俯仰，左右拖动旋转，双指缩放。".into();
            }
            Action::RestaurantCheck => {
                if s.mode == GameMode::CrystalGarden {
                    let (blocks, kinds) = s.work.creative_progress();
                    s.message = if blocks >= 10 && kinds >= 3 {
                        "布置完成！餐厅已经自动保存在这台设备。".into()
                    } else {
                        format!(
                            "餐厅布置：已搭 {blocks}/10 块，使用 {kinds}/3 种材料。继续装饰吧！"
                        )
                    };
                }
            }
            Action::RestaurantReset => {
                if s.mode == GameMode::CrystalGarden {
                    let before = s.work.world.clone();
                    s.undo.push(before);
                    s.work = restaurant_starter();
                    s.world_dirty = true;
                    s.build = true;
                    s.erase = false;
                    s.message = "餐厅已重置，可以点撤销恢复。".into();
                }
            }
            Action::BuilderTest => {
                s.reset_run();
                if let Some(path) = s.work.world.path_to_goal() {
                    let mut facing = s.work.world.facing;
                    let mut route = Vec::new();
                    for pair in path.windows(2) {
                        let delta = [pair[1][0] - pair[0][0], pair[1][1] - pair[0][1]];
                        let target = match delta {
                            [1, 0] => 0,
                            [0, 1] => 1,
                            [-1, 0] => 2,
                            [0, -1] => 3,
                            _ => continue,
                        };
                        match (target + 4 - facing) % 4 {
                            1 => {
                                route.push(Instruction::Right);
                                facing = (facing + 1) % 4;
                            }
                            2 => {
                                route.extend([Instruction::Right, Instruction::Right]);
                                facing = (facing + 2) % 4;
                            }
                            3 => {
                                route.push(Instruction::Left);
                                facing = (facing + 3) % 4;
                            }
                            _ => {}
                        }
                        route.push(Instruction::Forward);
                    }
                    s.work.program = route;
                    s.work.repeat = 1;
                    s.work.finish_forward = false;
                    s.editor_open = false;
                    s.build = false;
                    s.runner = Some(Runner::new(&s.work));
                    s.runner.as_mut().unwrap().running = true;
                    s.clock = 0.65;
                    s.message = "路线已找到！机器人正在测试你搭的桥。".into();
                } else {
                    s.build = true;
                    let progress = s.work.world.bridge_progress();
                    s.message = if s.work.level == 3 {
                        format!("桥还没接通两岸：当前铺好 {progress} / 5 格，继续搭建后再测试。")
                    } else {
                        "路线还没连通。用磁力积木接起缺口，再来测试路线。".into()
                    };
                }
            }
            Action::Code => {
                if s.mode == GameMode::FreeBuild {
                    s.build = true;
                    s.message = "自由工坊只管搭建：选材料后轻点方块，就能继续创作。".into();
                } else {
                    s.editor_open = true;
                    s.build = false;
                    s.message = game::LEVEL_HINTS[s.work.level].into();
                }
            }
            Action::Run => {
                s.editor_open = false;
                s.build = false;
                if s.runner.as_ref().is_some_and(|r| r.running) {
                    s.runner.as_mut().unwrap().running = false;
                    s.message = "已暂停，可以点「单步」观察下一步。".into();
                } else if s.work.program.is_empty() {
                    s.message = "先添加一张指令卡片吧".into();
                } else if s
                    .runner
                    .as_ref()
                    .is_some_and(|r| r.faulted || r.pc >= r.ops.len() || s.level_complete())
                {
                    s.message = "这次运行已结束。点「复位」后再试一次。".into();
                } else {
                    if s.runner.is_none() {
                        s.runner = Some(Runner::new(&s.work));
                    }
                    s.runner.as_mut().unwrap().running = true;
                    s.clock = 0.65;
                }
            }
            Action::Step => {
                s.build = false;
                if let Some(r) = &mut s.runner {
                    r.running = false;
                }
                s.step();
            }
            Action::Finish => {
                s.edit_program();
                s.work.finish_forward = !s.work.finish_forward;
            }
            Action::Reset => {
                if s.mode == GameMode::Island && s.build {
                    s.reset_run();
                    let before = s.work.world.clone();
                    let original = game::level(s.work.level);
                    if before != original {
                        if s.undo.len() >= 40 {
                            s.undo.remove(0);
                        }
                        s.undo.push(before);
                        s.work.world = original;
                        s.world_dirty = true;
                        s.persist();
                        s.message = "小岛已恢复到关卡初始状态，可用「撤销」找回刚才的搭建。".into();
                    } else {
                        s.message = "小岛已经是初始状态了。".into();
                    }
                } else {
                    s.reset_run();
                    s.message = "机器人已回到测试前的位置，可以继续搭建。".into();
                }
                if matches!(s.mode, GameMode::Island | GameMode::CloudBridge) {
                    s.build = true;
                }
            }
            Action::Add(op) => {
                s.edit_program();
                let limit = if s.mode == GameMode::Island { 8 } else { 16 };
                if s.work.program.len() < limit {
                    s.work.program.push(op);
                } else {
                    s.message = format!("最多放 {limit} 张卡片，试试用重复减少指令。");
                }
            }
            Action::Remove(i) => {
                s.edit_program();
                if i < s.work.program.len() {
                    s.work.program.remove(i);
                }
            }
            Action::Clear => {
                s.edit_program();
                s.work.program.clear();
                s.message = "从下面选一张指令卡片开始吧。".into();
            }
            Action::GuideRoute => {
                s.edit_program();
                s.editor_open = true;
                s.build = false;
                s.message = if s.work.load_guide_route() {
                    "示范路线已放好：先点「单步」看每一步，再改成自己的方案。".into()
                } else {
                    "这一关是自由建造，没有固定路线。试试用 3 种材料搭 10 块。".into()
                };
            }
            Action::Repeat => {
                s.edit_program();
                s.work.repeat = s.work.repeat % 8 + 1;
            }
            Action::Save => {
                s.message = if matches!(s.mode, GameMode::Island | GameMode::FreeBuild) {
                    "已保存到这台设备，下次打开可以继续。".into()
                } else {
                    "小游戏随时可以重玩，小岛作品仍保存在原来的档案里。".into()
                };
            }
            Action::Profile(i) => {
                s.persist();
                let return_to_free_build = s.mode == GameMode::FreeBuild;
                if s.mode != GameMode::Island && !return_to_free_build {
                    select_game(&mut s, GameMode::Island);
                }
                s.save.active = i;
                if return_to_free_build {
                    select_game(&mut s, GameMode::FreeBuild);
                    s.message = format!(
                        "欢迎回来，{}！继续搭你的自由作品吧。",
                        if i == 0 { "小芽" } else { "小树" }
                    );
                } else {
                    s.work = s.save.profiles[&i].clone();
                    s.island_3d_view = !island_base_intact(&s.work);
                    s.runner = None;
                    s.undo.clear();
                    s.world_dirty = true;
                    s.message = format!(
                        "欢迎回来，{}！{}",
                        if i == 0 { "小芽" } else { "小树" },
                        game::LEVEL_HINTS[s.work.level]
                    );
                }
            }
            Action::Next | Action::Previous => {
                if s.mode != GameMode::Island && s.mode != GameMode::CloudBridge {
                    s.message = "水晶花园可以自由搭建，先试试材料盒吧。".into();
                    s.ui_dirty = true;
                    break;
                }
                if s.mode == GameMode::CloudBridge {
                    if matches!(action, Action::Next)
                        && s.work.level < 5
                        && !s.cloud_bridge_campaign.completed.contains(&s.work.level)
                    {
                        s.message = "先完成本关目标，下一座岛就会解锁。".into();
                        s.ui_dirty = true;
                        break;
                    }
                    if s.work.level == 5
                        && matches!(action, Action::Next)
                        && !s.cloud_bridge_campaign.completed.contains(&5)
                    {
                        s.message = "先完成自由搭建挑战，再从第一座岛重新冒险。".into();
                        s.ui_dirty = true;
                        break;
                    }
                }
                s.reset_run();
                let index = if matches!(action, Action::Next) {
                    (s.work.level + 1) % 6
                } else {
                    (s.work.level + 5) % 6
                };
                if s.mode == GameMode::CloudBridge && index > s.cloud_bridge_campaign.unlocked {
                    s.message = "这座岛还没解锁，先完成前一关吧。".into();
                    s.ui_dirty = true;
                    break;
                }
                s.work.choose_level(index);
                if s.mode == GameMode::CloudBridge {
                    s.work.completed = s.cloud_bridge_campaign.completed.clone();
                    s.work.stars = s.cloud_bridge_campaign.stars;
                    s.build = index == 5;
                }
                if s.mode == GameMode::Island {
                    s.island_3d_view = false;
                    s.yaw = 0.68;
                    s.pitch = 0.762;
                    s.zoom = 16.;
                }
                s.build = true;
                s.undo.clear();
                s.world_dirty = true;
                s.message = game::LEVEL_HINTS[index].into();
            }
            Action::CloudLevel(index) => {
                if s.mode == GameMode::CloudBridge {
                    if index > 5 || index > s.cloud_bridge_campaign.unlocked {
                        s.message = "这座岛还没解锁，先完成当前目标吧。".into();
                    } else {
                        s.reset_run();
                        s.work.choose_level(index);
                        s.work.completed = s.cloud_bridge_campaign.completed.clone();
                        s.work.stars = s.cloud_bridge_campaign.stars;
                        s.build = index == 5;
                        s.undo.clear();
                        s.message = if index == 5 {
                            "终章挑战：搭建至少 10 块积木，并使用 3 种材料。".into()
                        } else {
                            game::LEVEL_HINTS[index].into()
                        };
                        s.world_dirty = true;
                    }
                }
            }
            Action::Undo => {
                s.reset_run();
                if let Some(w) = s.undo.pop() {
                    s.work.world = w;
                    s.world_dirty = true;
                    s.message = "已撤销上一次搭建。".into();
                } else {
                    s.message = "还没有需要撤销的搭建。".into();
                }
            }
            Action::ClearFreeBuild => {
                if s.mode == GameMode::FreeBuild {
                    let empty = game::free_build_world();
                    if s.work.world != empty {
                        if s.undo.len() >= 40 {
                            s.undo.remove(0);
                        }
                        let previous = s.work.world.clone();
                        s.undo.push(previous);
                        s.work.world = empty;
                        s.island_3d_view = false;
                        s.yaw = 0.68;
                        s.pitch = 0.762;
                        s.zoom = 22.;
                        s.world_dirty = true;
                        s.persist();
                        s.message = "作品已清空，可点撤销找回。".into();
                    } else {
                        s.message = "现在就是空白小岛。".into();
                    }
                }
            }
            Action::ResetView => {
                s.yaw = 0.68;
                s.pitch = 0.762;
                s.zoom = 16.;
                s.message = "视角已回正，可以继续搭建。".into();
            }
            Action::ToggleIslandView => {
                let base_intact = if s.mode == GameMode::FreeBuild {
                    free_build_base_intact(&s.work)
                } else {
                    island_base_intact(&s.work)
                };
                if s.island_3d_view && !base_intact {
                    s.message = "地形已修改，继续使用可旋转的立体视角。".into();
                } else {
                    s.island_3d_view = !s.island_3d_view;
                    s.yaw = 0.68;
                    s.pitch = 0.762;
                    s.zoom = if s.mode == GameMode::FreeBuild {
                        22.
                    } else {
                        16.
                    };
                    s.world_dirty = true;
                    s.message = if s.island_3d_view {
                        "已切换到可旋转的立体搭建视角。".into()
                    } else {
                        "已回到插画场景视角。".into()
                    };
                }
            }
            Action::ToggleIslandMenu => s.island_menu_open = !s.island_menu_open,
            Action::Rotate => {
                if s.mode == GameMode::FreeBuild {
                    s.island_3d_view = true;
                    s.world_dirty = true;
                }
                s.yaw += std::f32::consts::FRAC_PI_2;
            }
            Action::ZoomIn => s.zoom = (s.zoom - 1.5).max(8.),
            Action::ZoomOut => s.zoom = (s.zoom + 1.5).min(30.),
        }
        s.ui_dirty = true;
        s.persist();
        s.persist_sidecars();
        if s.mode == GameMode::Tetris {
            s.persist_tetris();
        }
        if s.mode == GameMode::ElementLab {
            s.persist_elements();
        }
        if s.mode == GameMode::Sudoku {
            s.persist_sudoku();
        }
        break;
    }
}
fn world_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut wheels: EventReader<MouseWheel>,
    mut g: ResMut<Gesture>,
    mut s: ResMut<Session>,
    scale: Res<UiScale>,
    blockers: Query<(&ComputedNode, &GlobalTransform), Or<(With<WorldInputBlocker>, With<Button>)>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    if !matches!(
        s.mode,
        GameMode::Island | GameMode::CloudBridge | GameMode::CrystalGarden | GameMode::FreeBuild
    ) || s.library_open
    {
        *g = Gesture::default();
        wheels.clear();
        return;
    }
    let blocked = |p: Vec2| {
        let physical = p * window.scale_factor();
        blockers.iter().any(|(node, transform)| {
            Rect::from_center_size(transform.translation().truncate(), node.size())
                .contains(physical)
        })
    };
    if g.touch
        .is_some_and(|id| touches.iter_just_canceled().any(|t| t.id() == id))
    {
        *g = Gesture::default();
        return;
    }
    let ts: Vec<_> = touches.iter().collect();
    let position = ts
        .first()
        .map(|t| t.position())
        .or_else(|| window.cursor_position());
    let in_world = |p: Vec2| {
        if blocked(p) {
            return false;
        }
        cameras
            .single()
            .ok()
            .and_then(|(camera, _)| camera.logical_viewport_rect())
            .is_some_and(|rect| rect.contains(p))
    };
    if ts.len() >= 2 {
        if g.pinch.is_none() {
            g.world = in_world(ts[0].start_position()) && in_world(ts[1].start_position());
        }
        let distance = ts[0].position().distance(ts[1].position());
        if let Some(old) = g.pinch.filter(|_| {
            (s.mode != GameMode::Island || s.island_3d_view)
                && g.world
                && !blocked(ts[0].position())
                && !blocked(ts[1].position())
        }) {
            s.zoom = (s.zoom * old / distance.max(1.)).clamp(8., 30.);
        }
        g.pinch = Some(distance);
        g.moved = true;
        return;
    } else if g.pinch.take().is_some() {
        s.ui_dirty = true;
        *g = Gesture::default();
        return;
    }
    for wheel in wheels.read() {
        if (s.mode != GameMode::Island || s.island_3d_view) && position.is_some_and(in_world) {
            let d = match wheel.unit {
                MouseScrollUnit::Line => wheel.y,
                MouseScrollUnit::Pixel => wheel.y * 0.025,
            };
            s.zoom = (s.zoom - d).clamp(8., 30.);
            s.ui_dirty = true;
        }
    }
    let down = mouse.pressed(MouseButton::Left) || !ts.is_empty();
    if down {
        if let Some(p) = position {
            if let Some(start) = g.start {
                if p.distance(start) > drag_slop(g.touch.is_some(), scale.0) {
                    g.moved = true;
                }
                if g.moved && g.world && (s.mode != GameMode::Island || s.island_3d_view) {
                    s.yaw -= (p.x - g.last.x) * 0.006;
                    // Stop short of the poles to keep the camera upright.
                    s.pitch = (s.pitch + (p.y - g.last.y) * 0.006).clamp(0.20, 1.30);
                }
                g.last = p;
            } else if mouse.just_pressed(MouseButton::Left)
                || touches.iter_just_pressed().next().is_some()
            {
                g.touch = ts.first().map(|t| t.id());
                g.start = Some(p);
                g.last = p;
                g.moved = false;
                g.world = in_world(p);
            }
        }
        return;
    }
    let released = g.touch.and_then(|id| {
        touches
            .iter_just_released()
            .find(|t| t.id() == id)
            .map(|t| t.position())
    });
    let Some(p) = g.finish_tap(released, scale.0) else {
        return;
    };
    if !in_world(p) || !s.build {
        return;
    }
    let Ok((camera, transform)) = cameras.single() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, p) else {
        return;
    };
    let mut best: Option<(f32, [i32; 3], Vec3)> = None;
    for b in &s.work.world.blocks {
        let center = Vec3::new(b.pos[0] as f32, b.pos[1] as f32, b.pos[2] as f32);
        if let Some((t, n)) = ray_box(ray.origin, *ray.direction, center) {
            if best.is_none_or(|(old, _, _)| t < old) {
                best = Some((t, b.pos, n));
            }
        }
    }
    let pos = if let Some((_, cell, normal)) = best {
        if s.erase {
            cell
        } else {
            [
                cell[0] + normal.x as i32,
                cell[1] + normal.y as i32,
                cell[2] + normal.z as i32,
            ]
        }
    } else {
        if s.erase || ray.direction.y.abs() < 0.001 {
            return;
        }
        let t = (-0.50 - ray.origin.y) / ray.direction.y;
        if t <= 0. {
            return;
        }
        let hit = ray.origin + *ray.direction * t;
        [hit.x.round() as i32, 0, hit.z.round() as i32]
    };
    let before = s.work.world.clone();
    let material = s.material;
    let erase = s.erase;
    let erased_illustrated_ground = matches!(s.mode, GameMode::Island | GameMode::FreeBuild)
        && !s.island_3d_view
        && erase
        && pos[1] == 0
        && before.material(pos) == Some(Material::Grass);
    match s.work.world.edit(pos, material, erase) {
        Ok(()) => {
            if erased_illustrated_ground {
                s.island_3d_view = true;
            }
            if s.undo.len() >= 40 {
                s.undo.remove(0);
            }
            s.undo.push(before);
            s.world_dirty = true;
            let (built, kinds) = s.work.creative_progress();
            let creative_complete = matches!(s.mode, GameMode::Island | GameMode::CloudBridge)
                && s.work.level == 5
                && built >= 10
                && kinds >= 3;
            if creative_complete {
                s.work.stars[5] = 3;
                if !s.work.completed.contains(&5) {
                    s.work.completed.push(5);
                }
                if s.mode == GameMode::CloudBridge {
                    s.cloud_bridge_campaign.record(5, 3);
                    s.work.completed = s.cloud_bridge_campaign.completed.clone();
                    s.work.stars = s.cloud_bridge_campaign.stars;
                }
            }
            s.message = if creative_complete {
                if s.mode == GameMode::CloudBridge {
                    "六座岛全部点亮！自由搭建挑战完成，可以重新冒险或继续创作。".into()
                } else {
                    "建造大师徽章解锁！这座作品使用了 3 种材料和至少 10 块积木。".into()
                }
            } else if erase {
                "拿走了一块。随时可以撤销。".into()
            } else if s.mode == GameMode::Island && s.work.level == 3 {
                format!(
                    "磁吸咔嗒！桥面已铺 {}/5 格，积木吸附成功。",
                    s.work.world.bridge_progress()
                )
            } else if s.mode == GameMode::Island && s.work.level == 5 {
                format!(
                    "咔嗒！已搭 {} 块，用了 {} 种材料（目标 10 块 / 3 种）。",
                    built, kinds
                )
            } else {
                format!("咔嗒！放好了一块{}。", material.label())
            };
            s.persist();
        }
        Err(e) => s.message = e.into(),
    }
    s.ui_dirty = true;
}
fn drag_slop(touch: bool, ui_scale: f32) -> f32 {
    if touch { (12. * ui_scale).max(8.) } else { 5. }
}

fn ray_box(origin: Vec3, direction: Vec3, center: Vec3) -> Option<(f32, Vec3)> {
    let lo = center - Vec3::splat(0.49);
    let hi = center + Vec3::splat(0.49);
    let mut near = 0.;
    let mut far = f32::INFINITY;
    let mut normal = Vec3::Y;
    for i in 0..3 {
        if direction[i].abs() < 0.00001 {
            if origin[i] < lo[i] || origin[i] > hi[i] {
                return None;
            }
            continue;
        }
        let a = (lo[i] - origin[i]) / direction[i];
        let b = (hi[i] - origin[i]) / direction[i];
        let n = a.min(b);
        if n > near {
            near = n;
            normal = Vec3::ZERO;
            normal[i] = -direction[i].signum();
        }
        far = far.min(a.max(b));
        if near > far {
            return None;
        }
    }
    if far < 0. { None } else { Some((near, normal)) }
}

fn update_builder_preview(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    touches: Res<Touches>,
    s: Res<Session>,
    mut preview: Query<(&mut Transform, &mut Visibility), With<BuilderGhost>>,
) {
    let Ok((mut transform, mut visibility)) = preview.single_mut() else {
        return;
    };
    if !matches!(s.mode, GameMode::Island | GameMode::FreeBuild) || !s.build || s.erase {
        *visibility = Visibility::Hidden;
        return;
    }
    let Ok(window) = windows.single() else {
        *visibility = Visibility::Hidden;
        return;
    };
    let position = touches
        .iter()
        .next()
        .map(|touch| touch.position())
        .or_else(|| window.cursor_position());
    let (Ok((camera, camera_transform)), Some(position)) = (cameras.single(), position) else {
        *visibility = Visibility::Hidden;
        return;
    };
    if !camera
        .logical_viewport_rect()
        .is_some_and(|viewport| viewport.contains(position))
    {
        *visibility = Visibility::Hidden;
        return;
    }
    let Ok(ray) = camera.viewport_to_world(camera_transform, position) else {
        *visibility = Visibility::Hidden;
        return;
    };
    let mut best: Option<(f32, [i32; 3], Vec3)> = None;
    for block in &s.work.world.blocks {
        let center = Vec3::new(
            block.pos[0] as f32,
            block.pos[1] as f32,
            block.pos[2] as f32,
        );
        if let Some((distance, normal)) = ray_box(ray.origin, *ray.direction, center) {
            if best.is_none_or(|(old, _, _)| distance < old) {
                best = Some((distance, block.pos, normal));
            }
        }
    }
    let target = if let Some((_, cell, normal)) = best {
        [
            cell[0] + normal.x.round() as i32,
            cell[1] + normal.y.round() as i32,
            cell[2] + normal.z.round() as i32,
        ]
    } else if ray.direction.y.abs() > 0.001 {
        let distance = (-0.50 - ray.origin.y) / ray.direction.y;
        if distance > 0. {
            let point = ray.origin + *ray.direction * distance;
            [point.x.round() as i32, 0, point.z.round() as i32]
        } else {
            *visibility = Visibility::Hidden;
            return;
        }
    } else {
        *visibility = Visibility::Hidden;
        return;
    };
    let connected = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ]
    .iter()
    .any(|delta| {
        s.work
            .world
            .material([
                target[0] + delta[0],
                target[1] + delta[1],
                target[2] + delta[2],
            ])
            .is_some()
    });
    let character_tile = target[1] == 0
        && ([target[0], target[2]] == s.work.world.robot
            || [target[0], target[2]] == s.work.world.goal);
    let valid = game::World::in_bounds(target)
        && s.work.world.material(target).is_none()
        && (target[1] == 0 || connected)
        && !character_tile;
    if valid {
        transform.translation = Vec3::new(target[0] as f32, target[1] as f32, target[2] as f32);
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}

fn capture_preview(
    mut commands: Commands,
    mut frame: Local<u32>,
    mut lightpath_solved_frame: Local<Option<u32>>,
    mut exit: EventWriter<bevy::app::AppExit>,
    mut session: ResMut<Session>,
) {
    #[cfg(debug_assertions)]
    if qa::enabled() {
        return;
    }
    let Ok(path) = std::env::var("MAGNET_CAPTURE") else {
        return;
    };
    *frame += 1;
    if *frame == 5 {
        if let Ok(state) = std::env::var("MAGNET_BOOK_STATE") {
            match state.as_str() {
                "literacy-intro" => session.literacy.start(0),
                "literacy-correct" => {
                    session.literacy.start(0);
                    session.literacy.next();
                    let i = session.literacy.word_index();
                    session.literacy.select(i);
                }
                "literacy-finished" => {
                    session.literacy.start(0);
                    session.literacy.finished = true;
                    session.literacy.progress.completed[0] = true;
                    session.literacy.progress.xp = 48;
                }
                "math-map" => session.math.choosing_level = true,
                "math-tools" => {
                    session.math_tools = true;
                    session.math.placed = 5;
                }
                "sudoku-four" => session.sudoku.set_size(4),
                "restaurant-demo" => {
                    for x in -3..=0 {
                        for z in -2..=1 {
                            session.work.world.put([x, 1, z], Material::Wood);
                        }
                    }
                    for x in -3..=0 {
                        for y in 2..=3 {
                            session.work.world.put(
                                [x, y, -2],
                                if y == 3 {
                                    Material::Glass
                                } else {
                                    Material::Stone
                                },
                            );
                        }
                    }
                    for z in -1..=1 {
                        for y in 2..=3 {
                            session.work.world.put(
                                [-3, y, z],
                                if y == 3 {
                                    Material::Glass
                                } else {
                                    Material::Wood
                                },
                            );
                        }
                    }
                    for x in -3..=0 {
                        for z in -2..=1 {
                            if x == -3 || z == -2 {
                                session.work.world.put([x, 4, z], Material::Wood);
                            }
                        }
                    }
                    session.work.world.put([2, 1, 0], Material::Wood);
                    session.work.world.put([2, 2, 0], Material::Wood);
                    session.work.world.put([2, 1, 2], Material::Stone);
                    session.work.world.put([3, 1, 0], Material::Stone);
                }
                _ => {}
            }
            session.ui_dirty = true;
            session.world_dirty = true;
        }
    }
    if *frame == 5 && session.mode == GameMode::Minesweeper {
        if let Ok(state) = std::env::var("MAGNET_CAPTURE_MINES_STATE") {
            let g = &mut session.minesweeper;
            match state.as_str() {
                "playing" | "flag" => {
                    for cell in [19, 20, 26, 28, 34, 36, 42, 43, 44] {
                        g.reveal(cell);
                    }
                    g.toggle_flag(27);
                    g.flag_mode = state == "flag";
                }
                "win" => {
                    for cell in 0..64 {
                        if !g.mines[cell] {
                            g.reveal(cell);
                        }
                    }
                }
                "lost" => {
                    g.reveal(19);
                    g.reveal(27);
                }
                _ => {}
            }
            session.ui_dirty = true;
        }
    }

    #[cfg(debug_assertions)]
    if *frame == 2 && session.mode == GameMode::Sokoban {
        if let Ok(state) = std::env::var("MAGNET_CAPTURE_SOKOBAN_STATE") {
            session.sokoban.capture_demo(&state);
            session.ui_dirty = true;
        }
    }
    if *frame == 2
        && session.mode == GameMode::GardenDefense
        && std::env::var_os("MAGNET_CAPTURE_GARDEN_DEMO").is_some()
    {
        session.garden.demo();
        session.ui_dirty = true;
    }
    // Exercise the same landscape-to-portrait transition as a library tap.
    if *frame == 30 && std::env::var_os("MAGNET_CAPTURE_SWITCH_TO_BALL").is_some() {
        select_game(&mut session, GameMode::Ball2048);
    }
    if *frame == 30 && std::env::var_os("MAGNET_CAPTURE_BREAKOUT_PLAY").is_some() {
        session.breakout.launch();
        session.ui_dirty = true;
    }
    if std::env::var_os("MAGNET_CAPTURE_ELEMENT_DEMO").is_some() {
        if *frame == 30 || *frame == 34 {
            session.elements.pick(1);
            session.ui_dirty = true;
        }
        if *frame == 38 {
            session.elements.pick(8);
            session.persist_elements();
            session.ui_dirty = true;
        }
    }
    if std::env::var_os("MAGNET_CAPTURE_LIGHTPATH_DEMO").is_some() {
        if *frame == 30 {
            let level = std::env::var("MAGNET_CAPTURE_LIGHTPATH_LEVEL")
                .ok()
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(1);
            session.light_path.capture_demo(level.saturating_sub(1));
            session.ui_dirty = true;
        }
        if lightpath_solved_frame.is_none() && session.light_path.is_solved() {
            *lightpath_solved_frame = Some(*frame);
            commands
                .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(bevy::render::view::screenshot::save_to_disk(format!(
                    "{path}.solved.png"
                )));
        }
    }
    if std::env::var_os("MAGNET_CAPTURE_SNAKE_DEMO").is_some() {
        if *frame == 30 {
            session.snake.turn(1, 0);
            session.ui_dirty = true;
        }
        if *frame == 66 {
            session.snake.turn(0, -1);
            session.ui_dirty = true;
        }
    }
    if *frame == 100 && std::env::var_os("MAGNET_CAPTURE_SWITCH_BACK").is_some() {
        session.library_open = true;
        session.ui_dirty = true;
    }
    if *frame == 30 && std::env::var_os("MAGNET_CAPTURE_MOTION").is_some() {
        commands
            .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(format!(
                "{path}.early.png"
            )));
    }
    if *frame == 90 {
        commands
            .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(path.clone()));
    }
    if *frame == 130 && std::env::var_os("MAGNET_STORYBOOK_SMOKE").is_some() {
        commands
            .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(format!(
                "{path}.verified.png"
            )));
    }
    if *frame == 135 && std::env::var_os("MAGNET_CAPTURE_SWITCH_BACK").is_some() {
        commands
            .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(format!(
                "{path}.back.png"
            )));
    }
    let sequence = std::env::var("MAGNET_CAPTURE_SEQUENCE").ok();
    if let Some(directory) = &sequence {
        if *frame >= 12
            && *frame
                <= if std::env::var_os("MAGNET_CAPTURE_LONG").is_some() {
                    900
                } else {
                    420
                }
            && *frame % 6 == 0
        {
            commands
                .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(bevy::render::view::screenshot::save_to_disk(format!(
                    "{directory}/frame-{:04}.png",
                    *frame
                )));
        }
    }
    if std::env::var_os("MAGNET_CAPTURE_LIGHTPATH_DEMO").is_some() && sequence.is_none() {
        // Wait for the real animation to finish, regardless of rendering frame rate.
        if lightpath_solved_frame.is_some_and(|solved| *frame >= solved + 15) || *frame > 1800 {
            exit.write(bevy::app::AppExit::Success);
        }
        return;
    }
    if *frame
        == if sequence.is_some() {
            if std::env::var_os("MAGNET_CAPTURE_LONG").is_some() {
                930
            } else {
                450
            }
        } else {
            150
        }
    {
        exit.write(bevy::app::AppExit::Success);
    }
}
fn label(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    size: f32,
    color: Color,
) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font: art.font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
    ));
}
fn button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    action: Action,
    width: f32,
    height: f32,
    background: Color,
    color: Color,
) {
    let font_size = match action {
        Action::SnakeMove(_, _) | Action::KlotskiMove(_, _) | Action::SokobanMove(_, _) => 28.,
        Action::SudokuSelect(_) | Action::SudokuDigit(_) => 22.,
        Action::BomberMove(_, _)
        | Action::SkyMove(_)
        | Action::PlatformLeft
        | Action::PlatformRight
        | Action::PlatformJump
        | Action::AssaultLeft
        | Action::AssaultRight
        | Action::AssaultJump
        | Action::TetrisLeft
        | Action::TetrisRight
        | Action::TetrisDown => 26.,
        Action::BallLeft | Action::BallRight => 22.,
        _ => 16.,
    };
    parent
        .spawn((
            Button,
            ButtonTone(background),
            action,
            Node {
                width: if matches!(action, Action::PianoKey(_)) {
                    Val::Percent(100. / 7.)
                } else {
                    Val::Px(width)
                },
                height: Val::Px(height),
                min_height: Val::Px(height),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: if matches!(action, Action::PianoKey(_)) {
                    UiRect::all(Val::Px(2.))
                } else {
                    UiRect::ZERO
                },
                flex_shrink: 0.,
                padding: UiRect::horizontal(Val::Px(6.)),
                ..default()
            },
            BackgroundColor(background),
            BorderColor(EDGE),
            BorderRadius::all(Val::Px(12.)),
        ))
        .with_children(|p| label(p, art, text, font_size, color));
}

fn tetris_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    action: Action,
    width: f32,
    height: f32,
    background: Color,
    border: Color,
) {
    let font_size = match action {
        Action::TetrisLeft | Action::TetrisRight | Action::TetrisDown => 28.,
        _ => 16.,
    };
    parent
        .spawn((
            Button,
            ButtonTone(background),
            action,
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                min_height: Val::Px(height),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.)),
                flex_shrink: 0.,
                padding: UiRect::horizontal(Val::Px(6.)),
                ..default()
            },
            BackgroundColor(background),
            BorderColor(border),
            BorderRadius::all(Val::Px(16.)),
        ))
        .with_children(|button| label(button, art, text, font_size, Color::WHITE));
}

// Every self-contained game gets the same escape route.  It is a small,
// high-layer node instead of a full-screen overlay, so it never blocks a
// game's own touch controls.
fn game_library_shortcut(commands: &mut Commands, art: &Art, compact: bool) {
    commands
        .spawn((
            UiRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(if compact { 12. } else { 20. }),
                top: Val::Px(if compact { 10. } else { 16. }),
                ..default()
            },
            ZIndex(20),
        ))
        .with_children(|root| {
            button(
                root,
                art,
                "← 游戏库",
                Action::Library,
                if compact { 82. } else { 96. },
                if compact { 34. } else { 38. },
                Color::srgba(0.08, 0.16, 0.21, 0.92),
                Color::WHITE,
            );
        });
}
fn row() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.),
        align_items: AlignItems::Center,
        ..default()
    }
}
fn panel(left: Val, right: Val, top: Val, bottom: Val, width: Val) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left,
        right,
        top,
        bottom,
        width,
        padding: UiRect::all(Val::Px(18.)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.),
        border: UiRect::all(Val::Px(1.)),
        ..default()
    }
}
fn rebuild_ui(
    mut commands: Commands,
    mut s: ResMut<Session>,
    art: Res<Art>,
    roots: Query<Entity, With<UiRoot>>,
    layout: Res<Layout>,
    ui_scale: Res<UiScale>,
) {
    if !s.ui_dirty && !layout.is_changed() {
        return;
    }
    s.ui_dirty = false;
    for root in &roots {
        commands.entity(root).despawn();
    }
    if s.library_open {
        library_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.magnet_modes_open {
        magnet_modes_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.help_open {
        if mission_index(s.mode).is_some() {
            mission_pause_panel(
                &mut commands,
                &s,
                &art,
                layout.width / ui_scale.0,
                layout.height / ui_scale.0,
            );
        } else {
            help_panel(&mut commands, &s, &art);
        }
    }
    if storybook_ui(
        &mut commands,
        &s,
        &art,
        layout.width / ui_scale.0,
        layout.height / ui_scale.0,
    ) {
        return;
    }
    if s.mode == GameMode::FruitArcade {
        arcade_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Slots {
        slots_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Fortune {
        fortune_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Literacy {
        literacy_ui(&mut commands, &s, &art);
        return;
    }
    if s.mode == GameMode::MusicFactory {
        piano_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::Tetris {
        tetris_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::PixelAssault {
        assault_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::MushroomRun {
        platform_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::TankBattle {
        tank_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::SkyRaid {
        sky_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            layout.width / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::BubbleIsland {
        bubble_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            (layout.height / ui_scale.0 - 174.).min(layout.width / ui_scale.0 - 48.),
        );
        return;
    }
    if s.mode == GameMode::BubbleBomber {
        bomber_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            layout.width > layout.height,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::FruitNinja {
        fruit_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::Fishing {
        fishing_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::HoneyHunt {
        honey_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::MoonFestival {
        moon_festival_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::SpaceCourier {
        courier_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Snake {
        snake_ui(
            &mut commands,
            &s,
            &art,
            layout.compact,
            layout.width,
            layout.height,
        );
        return;
    }
    if s.mode == GameMode::Minesweeper {
        minesweeper_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Classic2048 {
        classic_2048_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Klotski {
        klotski_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::Sokoban {
        sokoban_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Sudoku {
        sudoku_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::LightPath {
        light_path_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::GardenDefense {
        garden_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if mission_index(s.mode).is_some() {
        mission_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::Math {
        math_ui(&mut commands, &s, &art);
        return;
    }
    if s.mode == GameMode::Ball2048 {
        ball_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::Breakout {
        breakout_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::ElementLab {
        element_ui(
            &mut commands,
            &s,
            &art,
            layout.width / ui_scale.0,
            layout.height / ui_scale.0,
        );
        return;
    }
    if s.mode == GameMode::FreeBuild {
        free_build_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if s.mode == GameMode::Island && layout.compact {
        island_compact_ui(&mut commands, &s, &art);
        return;
    }
    if s.mode == GameMode::CloudBridge {
        cloud_bridge_ui(&mut commands, &s, &art, layout.compact);
        return;
    }
    if layout.compact {
        mobile_ui(&mut commands, &s, &art);
        return;
    }
    magnet_ui(&mut commands, &s, &art);
}

fn cloud_bridge_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let ink = Color::srgb(0.18, 0.25, 0.31);
    let muted = Color::srgb(0.38, 0.49, 0.58);
    let paper = Color::srgba(1.0, 0.98, 0.91, 0.97);
    let soft = Color::srgb(0.88, 0.94, 0.98);
    let blue = Color::srgb(0.20, 0.54, 0.91);
    let green = Color::srgb(0.09, 0.73, 0.31);
    let running = s.runner.as_ref().is_some_and(|runner| runner.running);
    let rail_width = if compact { 104. } else { 126. };
    let card_width = if compact { 84. } else { 102. };
    let card_height = if compact { 82. } else { 94. };
    let dock_height = if compact { 172. } else { 176. };
    let right_gap = if compact { 12. } else { 18. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.),
                    right: Val::Px(16.),
                    top: Val::Px(12.),
                    height: Val::Px(58.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
            ))
            .with_children(|header| {
                header.spawn(row()).with_children(|left| {
                    button(left, art, "←", Action::Library, 52., 52., paper, ink);
                    left.spawn((
                        Node {
                            height: Val::Px(52.),
                            padding: UiRect::horizontal(Val::Px(14.)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.),
                            ..row()
                        },
                        BackgroundColor(paper),
                        BorderRadius::all(Val::Px(18.)),
                    ))
                    .with_children(|brand| {
                        let mut icon = ImageNode::new(art.island_materials.clone());
                        icon.rect = Some(Rect::new(0., 0., 627., 627.));
                        brand.spawn((
                            Node {
                                width: Val::Px(34.),
                                height: Val::Px(34.),
                                ..default()
                            },
                            icon,
                        ));
                        label(brand, art, "云桥工程师", 24., ink);
                    });
                });
                header.spawn(row()).with_children(|right| {
                    button(right, art, "‹", Action::Previous, 42., 48., paper, ink);
                    right
                        .spawn((
                            Node {
                                height: Val::Px(48.),
                                padding: UiRect::horizontal(Val::Px(18.)),
                                align_items: AlignItems::Center,
                                ..row()
                            },
                            BackgroundColor(paper),
                            BorderRadius::all(Val::Px(18.)),
                        ))
                        .with_children(|level| {
                            let name =
                                ["新手村", "森林边", "风车岛", "云桥谷", "矿车站", "家园挑战"]
                                    [s.work.level.min(5)];
                            label(
                                level,
                                art,
                                format!("第 {} 关 · {name}", s.work.level + 1),
                                20.,
                                ink,
                            );
                        });
                    button(right, art, "›", Action::Next, 42., 48., paper, ink);
                    button(right, art, "?  怎么玩", Action::Help, 128., 48., paper, ink);
                });
            });

            let stage_title = format!("冒险进度 {}/6 · 过关继续下一岛", s.work.level + 1);
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(if compact { 25. } else { 31. }),
                    right: Val::Percent(if compact { 25. } else { 31. }),
                    top: Val::Px(if compact { 64. } else { 8. }),
                    height: Val::Px(if compact { 94. } else { 96. }),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(5.),
                    ..default()
                },
            ))
            .with_children(|progress| {
                label(
                    progress,
                    art,
                    stage_title,
                    if compact { 14. } else { 17. },
                    Color::WHITE,
                );
                progress
                    .spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Px(42.),
                        align_items: AlignItems::Center,
                        ..row()
                    })
                    .with_children(|track| {
                        for stage in 0..6 {
                            if stage > 0 {
                                track.spawn((
                                    Node {
                                        flex_grow: 1.,
                                        height: Val::Px(4.),
                                        ..default()
                                    },
                                    BackgroundColor(if stage <= s.work.level {
                                        Color::srgb(0.31, 0.82, 0.44)
                                    } else {
                                        Color::srgba(0.88, 0.96, 1.0, 0.72)
                                    }),
                                ));
                            }
                            let is_current = stage == s.work.level;
                            let is_open = stage <= s.cloud_bridge_campaign.unlocked;
                            let fill = if is_current {
                                Color::srgb(0.13, 0.75, 0.36)
                            } else if stage < s.work.level {
                                Color::srgb(0.12, 0.73, 0.36)
                            } else if is_open {
                                Color::srgb(0.23, 0.61, 0.88)
                            } else {
                                Color::srgba(0.44, 0.62, 0.75, 0.88)
                            };
                            let edge = if is_current {
                                Color::srgb(1.0, 0.86, 0.22)
                            } else {
                                Color::WHITE
                            };
                            track
                                .spawn((
                                    Button,
                                    ButtonTone(fill),
                                    Action::CloudLevel(stage),
                                    Node {
                                        width: Val::Px(if compact { 30. } else { 36. }),
                                        height: Val::Px(if compact { 30. } else { 36. }),
                                        min_width: Val::Px(if compact { 30. } else { 36. }),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Val::Px(if is_current {
                                            3.
                                        } else {
                                            2.
                                        })),
                                        flex_shrink: 0.,
                                        ..default()
                                    },
                                    BackgroundColor(fill),
                                    BorderColor(edge),
                                    BorderRadius::all(Val::Percent(50.)),
                                ))
                                .with_children(|node| {
                                    label(
                                        node,
                                        art,
                                        if stage == 5 {
                                            "家".to_owned()
                                        } else {
                                            (stage + 1).to_string()
                                        },
                                        if compact { 13. } else { 15. },
                                        Color::WHITE,
                                    );
                                });
                        }
                    });
            });

            let objective = if s.work.level == 5 {
                let (built, kinds) = s.work.creative_progress();
                format!("终章挑战：搭建 {built}/10 块积木 · 使用 {kinds}/3 种材料")
            } else {
                let total = game::LEVEL_CRYSTALS[s.work.level];
                let collected = total.saturating_sub(s.work.world.crystals.len());
                format!("本关目标：收集星晶 {collected}/{total} → 抵达星星")
            };
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(if compact { 156. } else { 108. }),
                    left: Val::Percent(50.),
                    margin: UiRect::left(Val::Px(if compact { -175. } else { -235. })),
                    width: Val::Px(if compact { 350. } else { 470. }),
                    max_width: Val::Percent(68.),
                    height: Val::Px(if compact { 48. } else { 62. }),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.),
                    ..default()
                },
                BackgroundColor(paper),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|mission| {
                let star = ImageNode::new(art.island_goal_star.clone());
                mission.spawn((
                    Node {
                        width: Val::Px(34.),
                        height: Val::Px(34.),
                        ..default()
                    },
                    star,
                ));
                label(
                    mission,
                    art,
                    objective,
                    if compact { 14. } else { 18. },
                    ink,
                );
            });

            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(right_gap),
                    top: Val::Px(88.),
                    bottom: Val::Px(dock_height + 24.),
                    width: Val::Px(rail_width),
                    padding: UiRect::all(Val::Px(if compact { 7. } else { 10. })),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(paper),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|rail| {
                label(rail, art, "材料", if compact { 15. } else { 18. }, ink);
                for material in [Material::Wood, Material::Grass, Material::Stone] {
                    island_material_card(
                        rail,
                        art,
                        material,
                        s.material == material && !s.erase,
                        card_width,
                        card_height,
                    );
                }
                rail.spawn(Node {
                    flex_grow: 1.,
                    ..default()
                });
                button(
                    rail,
                    art,
                    if s.build { "编程" } else { "搭建" },
                    if s.build { Action::Code } else { Action::Build },
                    card_width,
                    38.,
                    soft,
                    ink,
                );
            });

            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.),
                    right: Val::Px(rail_width + right_gap + 12.),
                    bottom: Val::Px(12.),
                    height: Val::Px(dock_height),
                    padding: UiRect::all(Val::Px(if compact { 9. } else { 13. })),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(paper),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|dock| {
                dock.spawn(Node {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                })
                .with_children(|heading| {
                    label(
                        heading,
                        art,
                        if s.build {
                            "搭建云桥"
                        } else {
                            "编排机器人的行动"
                        },
                        if compact { 17. } else { 20. },
                        ink,
                    );
                    label(
                        heading,
                        art,
                        if s.build {
                            "选材料后点空位搭桥"
                        } else if s.work.program.is_empty() {
                            "选择行动卡片，规划过桥路线"
                        } else {
                            "点已放入的卡片可移除"
                        },
                        if compact { 12. } else { 14. },
                        muted,
                    );
                    if s.build {
                        button(heading, art, "拆除", Action::Erase, 70., 36., soft, ink);
                        button(heading, art, "撤销", Action::Undo, 70., 36., soft, ink);
                    } else {
                        button(heading, art, "清空", Action::Clear, 70., 36., soft, ink);
                    }
                });
                dock.spawn(Node {
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(7.),
                    overflow: Overflow::clip_x(),
                    ..row()
                })
                .with_children(|controls| {
                    if s.build {
                        button(
                            controls,
                            art,
                            "切换编程模式",
                            Action::Code,
                            170.,
                            if compact { 58. } else { 70. },
                            soft,
                            ink,
                        );
                        label(controls, art, &s.message, 14., muted);
                    } else {
                        for (index, instruction) in s.work.program.iter().copied().enumerate() {
                            let (icon, title, color, accent) = match instruction {
                                Instruction::Place => (
                                    "+",
                                    "放木块",
                                    Color::srgb(1.0, 0.92, 0.75),
                                    Color::srgb(0.76, 0.42, 0.12),
                                ),
                                Instruction::Forward => {
                                    ("↑", "前进", Color::srgb(0.82, 0.92, 1.0), blue)
                                }
                                Instruction::Left => {
                                    ("↶", "左转", Color::srgb(0.83, 0.96, 0.86), green)
                                }
                                Instruction::Right => (
                                    "↷",
                                    "右转",
                                    Color::srgb(1.0, 0.86, 0.86),
                                    Color::srgb(0.84, 0.22, 0.25),
                                ),
                                Instruction::Back => (
                                    "↓",
                                    "后退",
                                    Color::srgb(0.90, 0.88, 1.0),
                                    Color::srgb(0.42, 0.38, 0.78),
                                ),
                                Instruction::Jump => (
                                    "↟",
                                    "跳跃",
                                    Color::srgb(1.0, 0.88, 0.63),
                                    Color::srgb(0.82, 0.51, 0.10),
                                ),
                                Instruction::Wait => {
                                    ("Ⅱ", "停留", Color::srgb(0.90, 0.93, 0.95), muted)
                                }
                            };
                            button(
                                controls,
                                art,
                                format!("{icon} {title} ×"),
                                Action::Remove(index),
                                if compact { 88. } else { 106. },
                                if compact { 56. } else { 68. },
                                color,
                                accent,
                            );
                        }
                        if s.work.program.is_empty() {
                            for _ in 0..3 {
                                controls
                                    .spawn((
                                        Node {
                                            width: Val::Px(if compact { 70. } else { 92. }),
                                            height: Val::Px(if compact { 56. } else { 68. }),
                                            border: UiRect::all(Val::Px(1.)),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BorderColor(Color::srgb(0.86, 0.87, 0.84)),
                                        BorderRadius::all(Val::Px(14.)),
                                        BackgroundColor(Color::srgb(0.98, 0.96, 0.90)),
                                    ))
                                    .with_children(|slot| label(slot, art, "+", 27., muted));
                            }
                        }
                        for (instruction, icon, title, color) in [
                            (
                                Instruction::Place,
                                "+",
                                "放木块",
                                Color::srgb(1.0, 0.92, 0.75),
                            ),
                            (
                                Instruction::Forward,
                                "↑",
                                "前进",
                                Color::srgb(0.82, 0.92, 1.0),
                            ),
                            (
                                Instruction::Left,
                                "↶",
                                "左转",
                                Color::srgb(0.83, 0.96, 0.86),
                            ),
                            (
                                Instruction::Right,
                                "↷",
                                "右转",
                                Color::srgb(1.0, 0.86, 0.86),
                            ),
                            (Instruction::Back, "↓", "后退", Color::srgb(0.90, 0.88, 1.0)),
                            (Instruction::Jump, "↟", "跳跃", Color::srgb(1.0, 0.88, 0.63)),
                            (
                                Instruction::Wait,
                                "Ⅱ",
                                "停留",
                                Color::srgb(0.90, 0.93, 0.95),
                            ),
                        ] {
                            button(
                                controls,
                                art,
                                format!("{icon} {title}"),
                                Action::Add(instruction),
                                if compact { 72. } else { 82. },
                                if compact { 56. } else { 68. },
                                color,
                                ink,
                            );
                        }
                        controls.spawn(Node {
                            flex_grow: 1.,
                            ..default()
                        });
                        island_run_button(
                            controls,
                            art,
                            running,
                            if compact { 120. } else { 154. },
                            if compact { 58. } else { 70. },
                        );
                    }
                });
            });
        });
}

fn magnet_ui(commands: &mut Commands, s: &Session, art: &Art) {
    if s.mode == GameMode::Island {
        island_builder_ui(commands, s, art);
        return;
    }
    let free_build = s.mode == GameMode::FreeBuild;
    let restaurant = s.mode == GameMode::CrystalGarden;
    let cloud_bridge = s.mode == GameMode::CloudBridge;
    let (
        frame,
        frame_edge,
        panel,
        panel_edge,
        accent,
        accent_text,
        soft_panel,
        heading,
        body,
        muted,
        tray_bg,
    ) = if cloud_bridge {
        (
            Color::srgb(0.035, 0.10, 0.22),
            Color::srgb(0.10, 0.72, 0.91),
            Color::srgb(0.06, 0.15, 0.29),
            Color::srgb(0.13, 0.55, 0.72),
            Color::srgb(0.18, 0.82, 0.94),
            Color::srgb(0.02, 0.12, 0.19),
            Color::srgb(0.10, 0.22, 0.35),
            Color::srgb(0.89, 0.96, 1.0),
            Color::srgb(0.70, 0.84, 0.93),
            Color::srgb(0.49, 0.69, 0.81),
            Color::srgb(0.055, 0.14, 0.26),
        )
    } else if restaurant {
        (
            Color::srgb(0.34, 0.22, 0.15),
            Color::srgb(0.67, 0.42, 0.25),
            Color::srgb(1.0, 0.96, 0.87),
            Color::srgb(0.83, 0.68, 0.48),
            Color::srgb(0.78, 0.39, 0.25),
            Color::WHITE,
            Color::srgb(0.92, 0.87, 0.73),
            Color::srgb(0.25, 0.20, 0.15),
            Color::srgb(0.38, 0.30, 0.22),
            Color::srgb(0.48, 0.42, 0.34),
            Color::srgb(0.91, 0.91, 0.78),
        )
    } else if free_build {
        (
            Color::srgb(0.88, 0.89, 0.87),
            Color::srgb(0.69, 0.73, 0.72),
            Color::srgb(0.98, 0.98, 0.96),
            Color::srgb(0.78, 0.81, 0.79),
            Color::srgb(0.18, 0.50, 0.49),
            Color::WHITE,
            Color::srgb(0.91, 0.93, 0.91),
            Color::srgb(0.18, 0.22, 0.22),
            Color::srgb(0.34, 0.39, 0.38),
            Color::srgb(0.48, 0.54, 0.52),
            Color::srgb(0.94, 0.95, 0.93),
        )
    } else {
        (
            Color::srgb(0.14, 0.40, 0.65),
            Color::srgb(0.30, 0.68, 0.87),
            Color::srgb(0.95, 0.98, 0.88),
            Color::srgb(0.48, 0.74, 0.56),
            Color::srgb(0.97, 0.72, 0.23),
            Color::srgb(0.22, 0.29, 0.22),
            Color::srgb(0.84, 0.94, 0.76),
            Color::srgb(0.14, 0.27, 0.24),
            Color::srgb(0.27, 0.39, 0.34),
            Color::srgb(0.40, 0.53, 0.45),
            Color::srgb(0.83, 0.94, 0.77),
        )
    };
    let active = s.runner.as_ref().and_then(|runner| {
        let instruction = if runner.faulted {
            runner.pc
        } else {
            runner.pc.saturating_sub(1)
        };
        (instruction < s.work.program.len() * s.work.repeat).then_some(instruction)
    });
    let total_stars: u8 = s.work.stars.iter().sum();
    let objective = if free_build {
        "自由搭建，没有关卡限制；作品会自动保存".to_owned()
    } else if restaurant || s.work.level == 5 {
        let (built, kinds) = s.work.creative_progress();
        if restaurant {
            format!("餐厅布置：搭建 {built} / 10 块 · 使用 {kinds} / 3 种材料")
        } else {
            format!("创作目标：搭建 {built} / 10 块 · 使用 {kinds} / 3 种材料")
        }
    } else {
        let total = game::LEVEL_CRYSTALS[s.work.level];
        format!(
            "收集星晶 {} / {}，再带机器人到达小旗",
            total.saturating_sub(s.work.world.crystals.len()),
            total
        )
    };

    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            // Each builder has a distinct visual language while sharing the same controls.
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.),
                    right: Val::Px(16.),
                    top: Val::Px(10.),
                    height: Val::Px(78.),
                    ..default()
                },
                BackgroundColor(frame),
                BorderColor(frame_edge),
                BorderRadius::all(Val::Px(22.)),
            ));
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.),
                    right: Val::Px(16.),
                    bottom: Val::Px(10.),
                    height: Val::Px(70.),
                    padding: UiRect::horizontal(Val::Px(16.)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                BackgroundColor(frame),
                BorderColor(frame_edge),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|footer| {
                footer
                    .spawn(Node {
                        width: Val::Px(390.),
                        max_width: Val::Percent(32.),
                        flex_shrink: 1.,
                        ..default()
                    })
                    .with_children(|p| {
                        label(p, art, &s.message, 15., body);
                    });
                footer.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "转视角",
                        Action::Rotate,
                        82.,
                        40.,
                        soft_panel,
                        heading,
                    );
                    button(
                        p,
                        art,
                        "视角回正",
                        Action::ResetView,
                        92.,
                        40.,
                        soft_panel,
                        heading,
                    );
                    button(
                        p,
                        art,
                        "放大 +",
                        Action::ZoomIn,
                        72.,
                        40.,
                        soft_panel,
                        heading,
                    );
                    label(
                        p,
                        art,
                        format!("{}%", (1600. / s.zoom).round() as i32),
                        14.,
                        accent,
                    );
                    button(
                        p,
                        art,
                        "缩小 −",
                        Action::ZoomOut,
                        72.,
                        40.,
                        soft_panel,
                        heading,
                    );
                });
                footer.spawn(row()).with_children(|p| {
                    if !free_build {
                        button(
                            p,
                            art,
                            "玩法",
                            Action::MagnetModes,
                            72.,
                            40.,
                            accent,
                            accent_text,
                        );
                        button(
                            p,
                            art,
                            "上一关",
                            Action::Previous,
                            88.,
                            40.,
                            soft_panel,
                            heading,
                        );
                        button(
                            p,
                            art,
                            "下一关 →",
                            Action::Next,
                            104.,
                            40.,
                            accent,
                            accent_text,
                        );
                    } else {
                        label(p, art, "自由创作 · 自动保存到当前档案", 14., body);
                    }
                });
            });

            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(26.),
                    right: Val::Px(26.),
                    top: Val::Px(18.),
                    height: Val::Px(60.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
            ))
            .with_children(|header| {
                header
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(2.),
                        ..default()
                    })
                    .with_children(|p| {
                        label(p, art, s.mode.title(), 29., heading);
                        label(p, art, s.mode.subtitle(), 12., body);
                    });
                header.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "小芽 · 6岁",
                        Action::Profile(0),
                        112.,
                        42.,
                        if s.save.active == 0 {
                            accent
                        } else {
                            soft_panel
                        },
                        if s.save.active == 0 {
                            accent_text
                        } else {
                            heading
                        },
                    );
                    button(
                        p,
                        art,
                        "小树 · 9岁",
                        Action::Profile(1),
                        112.,
                        42.,
                        if s.save.active == 1 {
                            accent
                        } else {
                            soft_panel
                        },
                        if s.save.active == 1 {
                            accent_text
                        } else {
                            heading
                        },
                    );
                    button(
                        p,
                        art,
                        "保存作品",
                        Action::Save,
                        108.,
                        42.,
                        soft_panel,
                        heading,
                    );
                    button(p, art, "游戏库", Action::Library, 92., 42., panel, body);
                });
            });

            // Left seed tray: each material remains directly selectable in build mode.
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    top: Val::Px(108.),
                    bottom: Val::Px(92.),
                    width: Val::Px(304.),
                    padding: UiRect::all(Val::Px(14.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(tray_bg),
                BorderColor(accent),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|tray| {
                label(
                    tray,
                    art,
                    if restaurant {
                        "餐厅装饰材料"
                    } else if cloud_bridge {
                        "桥梁工程材料"
                    } else if free_build {
                        "创作材料"
                    } else {
                        "岛屿积木材料"
                    },
                    23.,
                    heading,
                );
                label(
                    tray,
                    art,
                    if restaurant {
                        "选取材料，布置厨房与客席"
                    } else if cloud_bridge {
                        "搭建桥面与支撑结构"
                    } else {
                        "选择材料，再点场景放置"
                    },
                    13.,
                    muted,
                );
                let materials = build_materials();
                for pair in materials.chunks(2) {
                    tray.spawn(row()).with_children(|cards| {
                        for &(material, color) in pair {
                            let selected = s.material == material && !s.erase;
                            button(
                                cards,
                                art,
                                format!(
                                    "{} {}",
                                    if selected { "✓" } else { "◆" },
                                    material.label()
                                ),
                                Action::Material(material),
                                126.,
                                48.,
                                if selected { accent } else { color },
                                if selected { accent_text } else { heading },
                            );
                        }
                    });
                }
                tray.spawn(Node {
                    flex_grow: 1.,
                    min_height: Val::Px(4.),
                    ..default()
                });
                tray.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        if s.erase { "拆除中" } else { "拆除" },
                        Action::Erase,
                        126.,
                        42.,
                        if s.erase {
                            Color::srgb(1., 0.75, 0.65)
                        } else {
                            soft_panel
                        },
                        heading,
                    );
                    button(p, art, "撤销", Action::Undo, 126., 42., soft_panel, heading);
                });
            });

            // A compact mission card floats above the open 3D play space.
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(31.),
                    top: Val::Px(108.),
                    width: Val::Percent(36.),
                    padding: UiRect::axes(Val::Px(18.), Val::Px(12.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderColor(panel_edge),
                BorderRadius::all(Val::Px(20.)),
            ))
            .with_children(|card| {
                if free_build {
                    label(card, art, "自由创作", 24., heading);
                } else {
                    label(
                        card,
                        art,
                        format!(
                            "关卡 {} · {}",
                            s.work.level + 1,
                            game::LEVEL_NAMES[s.work.level]
                        ),
                        24.,
                        heading,
                    );
                }
                label(card, art, objective.clone(), 15., accent);
                if !free_build && !restaurant {
                    label(
                        card,
                        art,
                        format!("星星 {total_stars} / 18     探索 {}/6", s.work.level + 1),
                        13.,
                        muted,
                    );
                }
            });

            // Right seed packet becomes the coding deck; every existing action is kept.
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(18.),
                    top: Val::Px(108.),
                    bottom: Val::Px(92.),
                    width: Val::Px(356.),
                    padding: UiRect::all(Val::Px(14.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(7.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderColor(panel_edge),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|deck| {
                if free_build {
                    label(deck, art, "搭建工具", 23., heading);
                    label(deck, art, "没有终点，尽情组合方块", 14., muted);
                    button(
                        deck,
                        art,
                        "作品已自动保存",
                        Action::Save,
                        322.,
                        42.,
                        accent,
                        accent_text,
                    );
                    deck.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            "转视角",
                            Action::Rotate,
                            154.,
                            42.,
                            soft_panel,
                            heading,
                        );
                        button(
                            p,
                            art,
                            "视角回正",
                            Action::ResetView,
                            154.,
                            42.,
                            soft_panel,
                            heading,
                        );
                    });
                    deck.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            "放大 +",
                            Action::ZoomIn,
                            100.,
                            42.,
                            soft_panel,
                            heading,
                        );
                        button(
                            p,
                            art,
                            "缩小 −",
                            Action::ZoomOut,
                            100.,
                            42.,
                            soft_panel,
                            heading,
                        );
                        label(
                            p,
                            art,
                            format!("{}%", (1600. / s.zoom).round() as i32),
                            14.,
                            accent,
                        );
                    });
                    label(deck, art, &s.message, 15., muted);
                    return;
                }

                if restaurant {
                    label(deck, art, "餐厅布置台", 23., heading);
                    label(deck, art, "用木块安排座位，用绿植点亮空间", 14., muted);
                    button(
                        deck,
                        art,
                        "保存餐厅布置",
                        Action::Save,
                        322.,
                        42.,
                        accent,
                        accent_text,
                    );
                    deck.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            "转视角",
                            Action::Rotate,
                            154.,
                            42.,
                            soft_panel,
                            heading,
                        );
                        button(
                            p,
                            art,
                            "视角回正",
                            Action::ResetView,
                            154.,
                            42.,
                            soft_panel,
                            heading,
                        );
                    });
                    deck.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            "放大 +",
                            Action::ZoomIn,
                            100.,
                            42.,
                            soft_panel,
                            heading,
                        );
                        button(
                            p,
                            art,
                            "缩小 −",
                            Action::ZoomOut,
                            100.,
                            42.,
                            soft_panel,
                            heading,
                        );
                        label(
                            p,
                            art,
                            format!("{}%", (1600. / s.zoom).round() as i32),
                            14.,
                            accent,
                        );
                    });
                    label(deck, art, objective, 15., accent);
                    label(deck, art, &s.message, 14., muted);
                    return;
                }

                deck.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "搭建",
                        Action::Build,
                        154.,
                        40.,
                        if s.build { accent } else { soft_panel },
                        if s.build { accent_text } else { heading },
                    );
                    button(
                        p,
                        art,
                        "编程",
                        Action::Code,
                        154.,
                        40.,
                        if !s.build { accent } else { soft_panel },
                        if !s.build { accent_text } else { heading },
                    );
                });
                deck.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                })
                .with_children(|p| {
                    label(p, art, "我的指令", 21., heading);
                    button(
                        p,
                        art,
                        "示范路线",
                        Action::GuideRoute,
                        104.,
                        34.,
                        accent,
                        accent_text,
                    );
                    button(p, art, "清空", Action::Clear, 58., 34., soft_panel, heading);
                });
                button(
                    deck,
                    art,
                    format!("重复 {} 次   ＋", s.work.repeat),
                    Action::Repeat,
                    322.,
                    38.,
                    accent,
                    accent_text,
                );
                button(
                    deck,
                    art,
                    format!(
                        "循环后再前进：{}",
                        if s.work.finish_forward { "开" } else { "关" }
                    ),
                    Action::Finish,
                    322.,
                    32.,
                    soft_panel,
                    heading,
                );
                if s.work.program.is_empty() {
                    label(deck, art, "从下方选指令卡，编排机器人的路线", 13., muted);
                } else {
                    for pair_start in (0..s.work.program.len()).step_by(2) {
                        deck.spawn(row()).with_children(|cards| {
                            for index in pair_start..(pair_start + 2).min(s.work.program.len()) {
                                let instruction = s.work.program[index];
                                let is_current = active.is_some_and(|n| {
                                    !s.work.program.is_empty() && n % s.work.program.len() == index
                                });
                                let color = if is_current {
                                    Color::srgb(1., 0.80, 0.40)
                                } else {
                                    match instruction {
                                        Instruction::Place => Color::srgb(0.73, 0.85, 0.94),
                                        Instruction::Forward | Instruction::Back => {
                                            Color::srgb(0.79, 0.88, 0.66)
                                        }
                                        Instruction::Jump => Color::srgb(0.98, 0.78, 0.40),
                                        Instruction::Wait => Color::srgb(0.84, 0.84, 0.90),
                                        _ => Color::srgb(0.87, 0.80, 0.95),
                                    }
                                };
                                button(
                                    cards,
                                    art,
                                    format!("{}  {}  ×", index + 1, instruction.label()),
                                    Action::Remove(index),
                                    154.,
                                    34.,
                                    color,
                                    heading,
                                );
                            }
                        });
                    }
                }
                for ops in [
                    [Instruction::Place, Instruction::Forward],
                    [Instruction::Left, Instruction::Right],
                    [Instruction::Back, Instruction::Jump],
                    [Instruction::Wait, Instruction::Wait],
                ] {
                    deck.spawn(row()).with_children(|p| {
                        for instruction in ops {
                            button(
                                p,
                                art,
                                format!("＋ {}", instruction.label()),
                                Action::Add(instruction),
                                154.,
                                35.,
                                soft_panel,
                                heading,
                            );
                        }
                    });
                }
                deck.spawn(Node {
                    flex_grow: 1.,
                    min_height: Val::Px(0.),
                    ..default()
                });
                let running = s.runner.as_ref().is_some_and(|runner| runner.running);
                button(
                    deck,
                    art,
                    if running {
                        "Ⅱ  暂停"
                    } else {
                        "▶  运行"
                    },
                    Action::Run,
                    322.,
                    50.,
                    accent,
                    accent_text,
                );
                deck.spawn(row()).with_children(|p| {
                    button(p, art, "单步", Action::Step, 154., 40., soft_panel, heading);
                    button(
                        p,
                        art,
                        "复位",
                        Action::Reset,
                        154.,
                        40.,
                        soft_panel,
                        heading,
                    );
                });
            });
        });
}

fn island_score_stars(parent: &mut ChildSpawnerCommands, art: &Art, earned: u8, size: f32) {
    for index in 0..3 {
        parent.spawn((
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                ..default()
            },
            ImageNode::new(if index < earned {
                art.island_goal_star.clone()
            } else {
                art.island_star_empty.clone()
            }),
        ));
    }
}

fn island_material_card(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    material: Material,
    selected: bool,
    width: f32,
    height: f32,
) {
    let index = match material {
        Material::Grass => 0,
        Material::Wood => 1,
        Material::Stone => 2,
        Material::Crystal => 3,
        _ => return,
    };
    let x = (index % 2) as f32 * 627.;
    let y = (index / 2) as f32 * 627.;
    let mut icon = ImageNode::new(art.island_materials.clone());
    icon.rect = Some(Rect::new(x, y, x + 627., y + 627.));
    icon.image_mode = NodeImageMode::Stretch;
    let tone = if selected {
        Color::srgb(0.76, 0.89, 1.)
    } else {
        Color::WHITE
    };
    parent
        .spawn((
            Button,
            ButtonTone(tone),
            Action::Material(material),
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                min_height: Val::Px(height),
                flex_shrink: 0.,
                border: UiRect::all(Val::Px(if selected { 3. } else { 1. })),
                padding: UiRect::all(Val::Px(3.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(tone),
            BorderColor(if selected {
                Color::srgb(0.11, 0.51, 0.98)
            } else {
                Color::srgb(0.87, 0.90, 0.91)
            }),
            BorderRadius::all(Val::Px(15.)),
        ))
        .with_children(|card| {
            card.spawn((
                Node {
                    width: Val::Px(height - 28.),
                    height: Val::Px(height - 28.),
                    ..default()
                },
                icon,
            ));
            label(
                card,
                art,
                material.label(),
                if height < 70. { 12. } else { 16. },
                Color::srgb(0.05, 0.20, 0.30),
            );
        });
}

fn island_command_card(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    instruction: Instruction,
    icon: &str,
    title: &str,
    width: f32,
    height: f32,
    background: Color,
    accent: Color,
) {
    parent
        .spawn((
            Button,
            ButtonTone(background),
            Action::Add(instruction),
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                min_height: Val::Px(height),
                flex_shrink: 0.,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(2.),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            BackgroundColor(background),
            BorderColor(accent),
            BorderRadius::all(Val::Px(17.)),
        ))
        .with_children(|card| {
            if instruction == Instruction::Place {
                let mut image = ImageNode::new(art.island_materials.clone());
                image.rect = Some(Rect::new(627., 0., 1254., 627.));
                image.image_mode = NodeImageMode::Stretch;
                let size = if height < 70. { 27. } else { 36. };
                card.spawn((
                    Node {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        ..default()
                    },
                    image,
                ));
            } else {
                label(
                    card,
                    art,
                    icon,
                    if height < 70. { 28. } else { 36. },
                    accent,
                );
            }
            label(
                card,
                art,
                title,
                if height < 70. { 13. } else { 16. },
                Color::srgb(0.06, 0.20, 0.28),
            );
        });
}

fn island_run_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    running: bool,
    width: f32,
    height: f32,
) {
    let green = Color::srgb(0.03, 0.72, 0.41);
    parent
        .spawn((
            Button,
            ButtonTone(green),
            Action::Run,
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                min_height: Val::Px(height),
                flex_shrink: 0.,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(green),
            BorderRadius::all(Val::Px(height * 0.42)),
        ))
        .with_children(|button| {
            label(
                button,
                art,
                if running {
                    "Ⅱ  暂停"
                } else {
                    "▶  运行"
                },
                if height < 70. { 19. } else { 24. },
                Color::WHITE,
            );
        });
}

fn free_build_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let ink = Color::srgb(0.23, 0.12, 0.07);
    let panel = Color::srgba(1.0, 0.985, 0.945, 0.96);
    let soft = Color::srgb(0.96, 0.91, 0.82);
    let tray_width = if compact { 206. } else { 294. };
    let card_width = if compact { 86. } else { 126. };
    let card_height = if compact { 70. } else { 120. };
    let dock_height = if compact { 68. } else { 112. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    top: Val::Px(14.),
                    height: Val::Px(if compact { 44. } else { 54. }),
                    align_items: AlignItems::Center,
                    ..row()
                },
            ))
            .with_children(|header| {
                button(
                    header,
                    art,
                    "←",
                    Action::Library,
                    if compact { 44. } else { 54. },
                    if compact { 44. } else { 54. },
                    panel,
                    ink,
                );
                header
                    .spawn((
                        Node {
                            height: Val::Px(if compact { 44. } else { 54. }),
                            padding: UiRect::horizontal(Val::Px(12.)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(9.),
                            ..row()
                        },
                        BackgroundColor(panel),
                        BorderRadius::all(Val::Px(18.)),
                    ))
                    .with_children(|brand| {
                        let mut icon = ImageNode::new(art.island_materials.clone());
                        icon.rect = Some(Rect::new(0., 0., 627., 627.));
                        brand.spawn((
                            Node {
                                width: Val::Px(if compact { 27. } else { 36. }),
                                height: Val::Px(if compact { 27. } else { 36. }),
                                ..default()
                            },
                            icon,
                        ));
                        label(
                            brand,
                            art,
                            "磁力小岛  自由搭建",
                            if compact { 18. } else { 24. },
                            ink,
                        );
                    });
                if s.island_3d_view {
                    button(
                        header,
                        art,
                        "插画视角",
                        Action::ToggleIslandView,
                        if compact { 92. } else { 120. },
                        if compact { 44. } else { 54. },
                        panel,
                        ink,
                    );
                }
            });
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(18.),
                    top: Val::Px(if compact { 68. } else { 105. }),
                    width: Val::Px(tray_width),
                    height: Val::Px(if compact { 229. } else { 326. }),
                    padding: UiRect::all(Val::Px(if compact { 8. } else { 14. })),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(7.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|tray| {
                label(tray, art, "选择材料", if compact { 15. } else { 19. }, ink);
                for pair in build_materials()[..4].chunks(2) {
                    tray.spawn(row()).with_children(|cards| {
                        for &(material, _) in pair {
                            island_material_card(
                                cards,
                                art,
                                material,
                                s.material == material && !s.erase,
                                card_width,
                                card_height,
                            );
                        }
                    });
                }
            });
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    right: Val::Px(18.),
                    bottom: Val::Px(10.),
                    height: Val::Px(dock_height),
                    padding: UiRect::all(Val::Px(if compact { 7. } else { 14. })),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|dock| {
                if !compact {
                    label(dock, art, "自由搭建 · 作品自动保存", 17., ink);
                }
                dock.spawn(Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                })
                .with_children(|tools| {
                    let w = if compact { 163. } else { 225. };
                    let h = if compact { 45. } else { 66. };
                    button(
                        tools,
                        art,
                        "搭建",
                        Action::Build,
                        w,
                        h,
                        if !s.erase {
                            Color::srgb(1., 0.92, 0.75)
                        } else {
                            soft
                        },
                        ink,
                    );
                    button(
                        tools,
                        art,
                        "拆除",
                        Action::Erase,
                        w,
                        h,
                        if s.erase {
                            Color::srgb(1., 0.82, 0.82)
                        } else {
                            soft
                        },
                        ink,
                    );
                    button(
                        tools,
                        art,
                        "撤销",
                        Action::Undo,
                        w,
                        h,
                        Color::srgb(0.82, 0.92, 1.),
                        ink,
                    );
                    button(
                        tools,
                        art,
                        "旋转",
                        Action::Rotate,
                        w,
                        h,
                        Color::srgb(0.83, 0.96, 0.86),
                        ink,
                    );
                    button(
                        tools,
                        art,
                        "清空作品",
                        Action::ClearFreeBuild,
                        w,
                        h,
                        Color::srgb(0.91, 0.85, 1.),
                        ink,
                    );
                });
            });
        });
}

fn island_builder_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let ink = Color::srgb(0.23, 0.12, 0.07);
    let muted = Color::srgb(0.48, 0.37, 0.29);
    let panel = Color::srgba(1.0, 0.985, 0.945, 0.96);
    let soft = Color::srgb(0.96, 0.91, 0.82);
    let running = s.runner.as_ref().is_some_and(|runner| runner.running);
    let primary = &build_materials()[..4];
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            // The approved design uses separate title and star plaques above the scene.
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    right: Val::Px(18.),
                    top: Val::Px(14.),
                    height: Val::Px(58.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                },
            ))
            .with_children(|header| {
                header.spawn(row()).with_children(|left| {
                    button(left, art, "←", Action::Library, 54., 54., panel, ink);
                    left.spawn((
                        Node {
                            height: Val::Px(54.),
                            padding: UiRect::horizontal(Val::Px(12.)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.),
                            ..row()
                        },
                        BackgroundColor(panel),
                        BorderRadius::all(Val::Px(18.)),
                    ))
                    .with_children(|brand| {
                        let mut icon = ImageNode::new(art.island_materials.clone());
                        icon.rect = Some(Rect::new(0., 0., 627., 627.));
                        brand.spawn((
                            Node {
                                width: Val::Px(40.),
                                height: Val::Px(40.),
                                ..default()
                            },
                            icon,
                        ));
                        label(brand, art, "磁力小岛", 25., ink);
                        label(
                            brand,
                            art,
                            format!(
                                "  {}  {}/6",
                                game::LEVEL_NAMES[s.work.level]
                                    .split('·')
                                    .next()
                                    .unwrap_or("关卡")
                                    .trim(),
                                s.work.level + 1
                            ),
                            16.,
                            muted,
                        );
                    });
                });
                header.spawn(row()).with_children(|right| {
                    right
                        .spawn((
                            Node {
                                height: Val::Px(54.),
                                padding: UiRect::horizontal(Val::Px(18.)),
                                align_items: AlignItems::Center,
                                ..row()
                            },
                            BackgroundColor(panel),
                            BorderRadius::all(Val::Px(18.)),
                        ))
                        .with_children(|score| {
                            island_score_stars(score, art, s.work.stars[s.work.level], 36.);
                        });
                    button(
                        right,
                        art,
                        "...",
                        Action::ToggleIslandMenu,
                        66.,
                        54.,
                        panel,
                        ink,
                    );
                });
            });
            if s.island_menu_open {
                root.spawn((
                    WorldInputBlocker,
                    GlobalZIndex(30),
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(18.),
                        top: Val::Px(78.),
                        width: Val::Px(180.),
                        padding: UiRect::all(Val::Px(10.)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(7.),
                        ..default()
                    },
                    BackgroundColor(panel),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|menu| {
                    button(menu, art, "下一关", Action::Next, 160., 40., soft, ink);
                    button(menu, art, "保存进度", Action::Save, 160., 40., soft, ink);
                    button(
                        menu,
                        art,
                        if s.island_3d_view {
                            "插画视角"
                        } else {
                            "立体视角"
                        },
                        Action::ToggleIslandView,
                        160.,
                        40.,
                        soft,
                        ink,
                    );
                });
            }

            // The four bridge materials are the large, direct choices from the concept.
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(18.),
                    top: Val::Px(104.),
                    width: Val::Px(294.),
                    height: Val::Px(if s.island_3d_view { 522. } else { 326. }),
                    padding: UiRect::all(Val::Px(14.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|tray| {
                label(tray, art, "选择材料 · 搭建磁力桥", 19., ink);
                for pair in primary.chunks(2) {
                    tray.spawn(row()).with_children(|cards| {
                        for &(material, _) in pair {
                            let selected = s.material == material && !s.erase;
                            island_material_card(
                                cards,
                                art,
                                material,
                                selected,
                                126.,
                                if s.island_3d_view { 92. } else { 120. },
                            );
                        }
                    });
                }
                if s.island_3d_view {
                    label(tray, art, "点岛上的方块表面放置", 13., muted);
                    for pair in build_materials()[4..].chunks(3) {
                        tray.spawn(row()).with_children(|cards| {
                            for &(material, color) in pair {
                                let selected = s.material == material && !s.erase;
                                button(
                                    cards,
                                    art,
                                    material.label(),
                                    Action::Material(material),
                                    80.,
                                    34.,
                                    if selected { soft } else { color },
                                    ink,
                                );
                            }
                        });
                    }
                    tray.spawn(Node {
                        flex_grow: 1.,
                        ..default()
                    });
                    tray.spawn(row()).with_children(|actions| {
                        button(
                            actions,
                            art,
                            if s.erase { "✓ 拆除" } else { "拆除" },
                            Action::Erase,
                            126.,
                            42.,
                            soft,
                            ink,
                        );
                        button(actions, art, "缩放 +", Action::ZoomIn, 64., 42., soft, ink);
                        button(actions, art, "−", Action::ZoomOut, 48., 42., soft, ink);
                    });
                    tray.spawn(row()).with_children(|tools| {
                        button(
                            tools,
                            art,
                            format!("重复 {}", s.work.repeat),
                            Action::Repeat,
                            80.,
                            30.,
                            soft,
                            ink,
                        );
                        button(tools, art, "单步", Action::Step, 56., 30., soft, ink);
                        button(tools, art, "复位", Action::Reset, 56., 30., soft, ink);
                    });
                    tray.spawn(row()).with_children(|tools| {
                        button(tools, art, "搭建", Action::Build, 80., 30., soft, ink);
                        button(tools, art, "检查", Action::BuilderTest, 80., 30., soft, ink);
                    });
                }
            });

            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    right: Val::Px(18.),
                    bottom: Val::Px(12.),
                    height: Val::Px(139.),
                    padding: UiRect::all(Val::Px(14.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|dock| {
                dock.spawn(row()).with_children(|heading| {
                    label(heading, art, "编写行动 · 让机器人到达星星", 19., ink);
                    if !s.work.program.is_empty() {
                        label(
                            heading,
                            art,
                            format!("  ·  已编排 {} 步  ·  {}", s.work.program.len(), s.message),
                            13.,
                            muted,
                        );
                    }
                });
                dock.spawn(row()).with_children(|controls| {
                    for (op, icon, title, color, accent) in [
                        (
                            Instruction::Place,
                            "+",
                            "放木块",
                            Color::srgb(1.0, 0.92, 0.75),
                            Color::srgb(0.76, 0.42, 0.12),
                        ),
                        (
                            Instruction::Forward,
                            "↑",
                            "前进",
                            Color::srgb(0.82, 0.92, 1.0),
                            Color::srgb(0.12, 0.49, 0.88),
                        ),
                        (
                            Instruction::Left,
                            "←",
                            "左转",
                            Color::srgb(0.83, 0.96, 0.86),
                            Color::srgb(0.09, 0.58, 0.31),
                        ),
                        (
                            Instruction::Right,
                            "→",
                            "右转",
                            Color::srgb(1.0, 0.86, 0.86),
                            Color::srgb(0.84, 0.22, 0.25),
                        ),
                    ] {
                        island_command_card(
                            controls, art, op, icon, title, 185., 76., color, accent,
                        );
                    }
                    controls.spawn((
                        Node {
                            width: Val::Px(20.),
                            height: Val::Px(54.),
                            border: UiRect::left(Val::Px(1.)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.75, 0.82, 0.85)),
                    ));
                    button(controls, art, "↶  撤销", Action::Undo, 108., 76., soft, ink);
                    controls.spawn(Node {
                        flex_grow: 1.,
                        ..default()
                    });
                    island_run_button(controls, art, running, 190., 76.);
                });
            });
        });
}

fn island_compact_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let ink = Color::srgb(0.23, 0.12, 0.07);
    let panel = Color::srgba(1.0, 0.985, 0.945, 0.96);
    let soft = Color::srgb(0.96, 0.91, 0.82);
    let running = s.runner.as_ref().is_some_and(|runner| runner.running);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(8.),
                    right: Val::Px(8.),
                    top: Val::Px(6.),
                    height: Val::Px(52.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
            ))
            .with_children(|bar| {
                bar.spawn(row()).with_children(|p| {
                    button(p, art, "←", Action::Library, 44., 48., panel, ink);
                    p.spawn((
                        Node {
                            height: Val::Px(48.),
                            padding: UiRect::horizontal(Val::Px(9.)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(7.),
                            ..row()
                        },
                        BackgroundColor(panel),
                        BorderRadius::all(Val::Px(16.)),
                    ))
                    .with_children(|brand| {
                        let mut icon = ImageNode::new(art.island_materials.clone());
                        icon.rect = Some(Rect::new(0., 0., 627., 627.));
                        brand.spawn((
                            Node {
                                width: Val::Px(30.),
                                height: Val::Px(30.),
                                ..default()
                            },
                            icon,
                        ));
                        label(brand, art, "磁力小岛", 20., ink);
                        label(brand, art, format!("{}/6", s.work.level + 1), 14., ink);
                    });
                });
                bar.spawn(row()).with_children(|p| {
                    p.spawn((
                        Node {
                            height: Val::Px(48.),
                            padding: UiRect::horizontal(Val::Px(10.)),
                            align_items: AlignItems::Center,
                            ..row()
                        },
                        BackgroundColor(panel),
                        BorderRadius::all(Val::Px(16.)),
                    ))
                    .with_children(|score| {
                        island_score_stars(score, art, s.work.stars[s.work.level], 28.);
                    });
                    button(
                        p,
                        art,
                        "...",
                        Action::ToggleIslandMenu,
                        58.,
                        48.,
                        panel,
                        ink,
                    );
                });
            });
            if s.island_menu_open {
                root.spawn((
                    WorldInputBlocker,
                    GlobalZIndex(30),
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(8.),
                        top: Val::Px(64.),
                        width: Val::Px(146.),
                        padding: UiRect::all(Val::Px(8.)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.),
                        ..default()
                    },
                    BackgroundColor(panel),
                    BorderRadius::all(Val::Px(16.)),
                ))
                .with_children(|menu| {
                    button(menu, art, "下一关", Action::Next, 130., 34., soft, ink);
                    button(menu, art, "保存进度", Action::Save, 130., 34., soft, ink);
                    button(
                        menu,
                        art,
                        if s.island_3d_view {
                            "插画视角"
                        } else {
                            "立体视角"
                        },
                        Action::ToggleIslandView,
                        130.,
                        34.,
                        soft,
                        ink,
                    );
                });
            }
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(8.),
                    top: Val::Px(72.),
                    width: Val::Px(206.),
                    height: Val::Px(if s.island_3d_view { 448. } else { 255. }),
                    padding: UiRect::all(Val::Px(8.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderRadius::all(Val::Px(17.)),
            ))
            .with_children(|tray| {
                label(tray, art, "搭建磁力桥", 16., ink);
                for pair in build_materials()[..4].chunks(2) {
                    tray.spawn(row()).with_children(|cards| {
                        for &(material, _) in pair {
                            let selected = s.material == material && !s.erase;
                            island_material_card(
                                cards,
                                art,
                                material,
                                selected,
                                90.,
                                if s.island_3d_view { 56. } else { 91. },
                            );
                        }
                    });
                }
                if s.island_3d_view {
                    for pair in build_materials()[4..].chunks(3) {
                        tray.spawn(row()).with_children(|cards| {
                            for &(material, color) in pair {
                                let selected = s.material == material && !s.erase;
                                button(
                                    cards,
                                    art,
                                    material.label(),
                                    Action::Material(material),
                                    58.,
                                    30.,
                                    if selected { soft } else { color },
                                    ink,
                                );
                            }
                        });
                    }
                    tray.spawn(Node {
                        flex_grow: 1.,
                        ..default()
                    });
                    tray.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            if s.erase { "✓ 拆除" } else { "拆除" },
                            Action::Erase,
                            90.,
                            36.,
                            soft,
                            ink,
                        );
                        button(p, art, "缩放 +", Action::ZoomIn, 90., 36., soft, ink);
                    });
                    tray.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            format!("重复 {}", s.work.repeat),
                            Action::Repeat,
                            62.,
                            29.,
                            soft,
                            ink,
                        );
                        button(p, art, "单步", Action::Step, 55., 29., soft, ink);
                        button(p, art, "复位", Action::Reset, 55., 29., soft, ink);
                    });
                    tray.spawn(row()).with_children(|p| {
                        button(p, art, "搭建", Action::Build, 62., 29., soft, ink);
                        button(p, art, "检查", Action::BuilderTest, 80., 29., soft, ink);
                    });
                }
            });
            root.spawn((
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(8.),
                    right: Val::Px(8.),
                    bottom: Val::Px(6.),
                    height: Val::Px(112.),
                    padding: UiRect::all(Val::Px(8.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.),
                    ..default()
                },
                BackgroundColor(panel),
                BorderRadius::all(Val::Px(17.)),
            ))
            .with_children(|dock| {
                label(
                    dock,
                    art,
                    format!(
                        "编写行动 · 让机器人到达星星{}",
                        if s.work.program.is_empty() {
                            String::new()
                        } else {
                            format!("  ·  已编排 {} 步", s.work.program.len())
                        }
                    ),
                    16.,
                    ink,
                );
                dock.spawn(row()).with_children(|p| {
                    for (op, icon, title, color, accent) in [
                        (
                            Instruction::Place,
                            "+",
                            "放木块",
                            Color::srgb(1., 0.92, 0.75),
                            Color::srgb(0.76, 0.42, 0.12),
                        ),
                        (
                            Instruction::Forward,
                            "↑",
                            "前进",
                            Color::srgb(0.82, 0.92, 1.),
                            Color::srgb(0.12, 0.49, 0.88),
                        ),
                        (
                            Instruction::Left,
                            "←",
                            "左转",
                            Color::srgb(0.83, 0.96, 0.86),
                            Color::srgb(0.09, 0.58, 0.31),
                        ),
                        (
                            Instruction::Right,
                            "→",
                            "右转",
                            Color::srgb(1., 0.86, 0.86),
                            Color::srgb(0.84, 0.22, 0.25),
                        ),
                    ] {
                        island_command_card(p, art, op, icon, title, 105., 62., color, accent);
                    }
                    p.spawn(Node {
                        flex_grow: 1.,
                        ..default()
                    });
                    button(p, art, "撤销", Action::Undo, 68., 62., soft, ink);
                    island_run_button(p, art, running, 112., 62.);
                });
            });
        });
}

fn build_materials() -> [(Material, Color); 10] {
    [
        (Material::Grass, Color::srgb(0.76, 0.85, 0.59)),
        (Material::Wood, Color::srgb(0.91, 0.73, 0.46)),
        (Material::Stone, Color::srgb(0.78, 0.82, 0.81)),
        (Material::Crystal, Color::srgb(0.62, 0.83, 0.87)),
        (Material::Brick, Color::srgb(0.78, 0.43, 0.32)),
        (Material::Sand, Color::srgb(0.92, 0.78, 0.48)),
        (Material::Snow, Color::srgb(0.91, 0.96, 1.0)),
        (Material::Glass, Color::srgb(0.55, 0.84, 0.96)),
        (Material::Lamp, Color::srgb(1.0, 0.62, 0.20)),
        (Material::Leaf, Color::srgb(0.35, 0.68, 0.28)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ball_eats_matching_ball_and_doubles_immediately() {
        let mut ball = BallGame::new();
        ball.running = true;
        let before_speed = ball.interval();
        let message = ball
            .advance(before_speed * 0.97)
            .expect("matching ball should collide");
        assert_eq!(ball.number, 4);
        assert_eq!(ball.score, 4);
        assert!(message.contains("变成了 4"));
        assert!(ball.interval() < before_speed);
    }

    #[test]
    fn opening_ball_gates_are_safe_before_traps_appear() {
        let ball = BallGame::new();
        for (index, gate) in ball.track.chunks_exact(3).take(5).enumerate() {
            assert_eq!(
                gate.iter()
                    .map(|item| item.lane)
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                3
            );
            let traps = gate
                .iter()
                .filter(|item| matches!(item.kind, BallKind::Trap))
                .count();
            assert_eq!(traps, usize::from(index >= 4));
            assert_eq!(
                gate.iter()
                    .filter(|item| matches!(item.kind, BallKind::Number(2)))
                    .count(),
                1
            );
        }
    }

    #[test]
    fn ball_trap_uses_a_shield_before_ending_the_run() {
        let mut ball = BallGame::new();
        ball.running = true;
        ball.lane = 2;
        ball.track = vec![
            BallObstacle {
                lane: 0,
                kind: BallKind::Number(8),
            },
            BallObstacle {
                lane: 1,
                kind: BallKind::Number(8),
            },
            BallObstacle {
                lane: 2,
                kind: BallKind::Trap,
            },
        ];
        let message = ball.advance(ball.interval()).expect("trap should resolve");
        assert!(message.contains("护盾"));
        assert!(!ball.failed);
        assert!(ball.running);
        assert_eq!(ball.shields, 1);
        assert_eq!(ball.lane, 1);
    }

    #[test]
    fn missing_a_gate_keeps_generating_the_current_number() {
        let mut ball = BallGame::new();
        ball.running = true;
        ball.lane = 0; // first gate's decoy lane
        let _ = ball.advance(ball.interval() * 0.95);
        assert_eq!(ball.number, 2);
        assert!(
            ball.track[ball.next..ball.next + 3]
                .iter()
                .any(|item| matches!(item.kind, BallKind::Number(2)))
        );
    }

    #[test]
    fn piano_correct_key_advances_song_and_builds_combo() {
        let mut piano = PianoGame::new();
        assert_eq!(piano.target(), 0);
        piano.fall = 1.2;
        piano.hit(0);
        assert_eq!(piano.step, 1);
        assert_eq!(piano.combo, 1);
        assert!(piano.score >= 11);
        assert_eq!(piano.fall, 0.);
    }

    #[test]
    fn piano_waits_for_child_in_practice_mode() {
        let mut piano = PianoGame::new();
        piano.advance(20.);
        assert_eq!(piano.step, 0);
        assert_eq!(piano.fall, 3.2);
        piano.hit(2);
        assert_eq!(piano.step, 0);
        piano.hit(0);
        assert_eq!(piano.step, 1);
    }

    #[test]
    fn assault_bullet_defeats_enemy_on_matching_lane() {
        let mut assault = AssaultGame::new();
        assault.enemies = vec![AssaultEnemy {
            lane: 1,
            x: 0.42,
            hp: 1,
            heavy: false,
        }];
        assault.lane = 1;
        assault.fire();
        assault.bullets[0].x = 0.42;
        assault.advance(0.);
        assert!(assault.enemies.is_empty());
        assert_eq!(assault.combo, 1);
        assert!(assault.score >= 20);
    }

    #[test]
    fn assault_progresses_through_three_stages_and_finishes() {
        let mut assault = AssaultGame::new();
        assault.move_player(1., 6.2);
        assert_eq!(assault.stage, 1);
        assert!(!assault.finished);

        assault.move_player(1., 6.2);
        assert_eq!(assault.stage, 2);
        assert!(!assault.finished);

        assault.move_player(1., 6.2);
        assert!(assault.finished);
        assert_eq!(assault.distance, 780.);
        let bullets_before = assault.bullets.len();
        assault.fire();
        assault.advance(1.);
        assert_eq!(assault.bullets.len(), bullets_before);
    }

    #[test]
    fn assault_camera_tracks_the_player_after_the_follow_point() {
        let mut assault = AssaultGame::new();
        assault.move_player(1., 0.5);
        let screen_x_after_first_move = assault.player_x - assault.camera_x;
        assault.move_player(1., 0.5);
        let screen_x_after_second_move = assault.player_x - assault.camera_x;
        assert!(assault.player_x > 0.24);
        assert!((screen_x_after_first_move - screen_x_after_second_move).abs() < 0.001);
        assert!(assault.distance > 0.);
    }

    #[test]
    fn platform_course_rewards_coins_and_a_safe_stomp() {
        let mut platform = PlatformGame::new();
        platform.things = vec![PlatformThing {
            x: platform.player_x,
            y: 0.14,
            kind: PlatformThingKind::Coin,
        }];
        platform.advance(0.);
        assert_eq!((platform.coins, platform.score), (1, 10));

        platform.player_y = 0.42;
        platform.velocity_y = 0.;
        platform.things = vec![PlatformThing {
            x: platform.player_x,
            y: 0.,
            kind: PlatformThingKind::Walker,
        }];
        platform.advance(0.);
        assert_eq!((platform.coins, platform.score, platform.lives), (3, 30, 3));
    }

    #[test]
    fn platform_opening_only_spawns_safe_coins() {
        let mut platform = PlatformGame::new();
        for _ in 0..4 {
            platform.spawn();
        }
        assert!(
            platform
                .things
                .iter()
                .all(|thing| thing.kind == PlatformThingKind::Coin)
        );
    }

    #[test]
    fn platform_only_advances_while_the_player_walks() {
        let mut platform = PlatformGame::new();
        platform.advance(1.);
        assert_eq!(platform.distance, 0.);
        assert!(platform.move_player(1., 0.5));
        assert!(platform.distance > 0.);
        assert!(platform.walk_timer > 0.);
    }

    #[test]
    fn platform_late_jump_tap_is_buffered_until_landing() {
        let mut platform = PlatformGame::new();
        platform.player_y = 0.02;
        platform.velocity_y = -0.3;
        platform.jump();
        assert!(platform.jump_buffer > 0.);
        platform.advance(0.1);
        assert!(platform.velocity_y > 0.);
    }

    #[test]
    fn platform_respawn_protection_prevents_an_immediate_second_hit() {
        let mut platform = PlatformGame::new();
        platform.things = vec![PlatformThing {
            x: platform.player_x,
            y: 0.,
            kind: PlatformThingKind::Walker,
        }];
        platform.advance(0.);
        assert_eq!(platform.lives, 2);
        assert!(platform.invulnerable > 0.);
        platform.things = vec![PlatformThing {
            x: platform.player_x,
            y: 0.,
            kind: PlatformThingKind::Walker,
        }];
        platform.advance(0.);
        assert_eq!(platform.lives, 2);
    }

    #[test]
    fn tetris_locking_a_completed_row_clears_and_scores() {
        let mut tetris = TetrisGame::new();
        for x in 0..6 {
            tetris.board[(TETRIS_HEIGHT - 1) * TETRIS_WIDTH + x] = 2;
        }
        tetris.kind = 0; // Horizontal I piece fills columns 6 through 9.
        tetris.x = 6;
        tetris.y = 18;
        assert!(tetris.valid(tetris.x, tetris.y, tetris.rotation));
        tetris.lock_piece();
        assert_eq!((tetris.lines, tetris.level, tetris.score), (1, 1, 100));
        assert!(
            tetris.board[(TETRIS_HEIGHT - 1) * TETRIS_WIDTH..]
                .iter()
                .all(|cell| *cell == 0)
        );
    }

    #[test]
    fn tetris_drops_do_not_award_points_without_a_line_clear() {
        let mut tetris = TetrisGame::new();
        tetris.soft_drop();
        assert_eq!(tetris.score, 0);
        tetris.hard_drop();
        assert_eq!(tetris.score, 0);
    }

    #[test]
    fn tetris_save_restores_board_and_best_score_survives_restart() {
        let mut tetris = TetrisGame::new();
        tetris.hard_drop();
        tetris.score = 300;
        tetris.best_score = 300;
        tetris.lines = 2;
        let bytes = serde_json::to_vec(&TetrisSave::from_game(&tetris)).unwrap();
        let restored = serde_json::from_slice::<TetrisSave>(&bytes)
            .unwrap()
            .into_game()
            .unwrap();
        assert_eq!(restored.board, tetris.board);
        assert_eq!(
            (restored.score, restored.lines, restored.next),
            (300, 2, tetris.next)
        );
        let mut restarted = restored;
        restarted.reset();
        assert_eq!(restarted.score, 0);
        assert_eq!(restarted.best_score, 300);
        assert!(restarted.board.iter().all(|cell| *cell == 0));
    }

    #[test]
    fn tetris_rejects_a_damaged_save() {
        let mut save = TetrisSave::from_game(&TetrisGame::new());
        save.board[0] = 99;
        assert!(save.into_game().is_none());
    }

    #[test]
    fn tank_shell_destroys_enemy_and_starts_next_wave() {
        let mut tank = TankGame::new();
        tank.player = (3, 6);
        tank.direction = (0, -1);
        tank.enemies = vec![TankEnemy {
            fire_clock: 1.4,
            warning: 0.,
            x: 3,
            y: 5,
            direction: (0, 1),
        }];
        tank.reserves = 0;
        tank.fire();
        tank.advance(0.4);
        assert_eq!(tank.score, 50);
        assert_eq!(tank.wave, 2);
        assert!(!tank.enemies.is_empty());
    }

    #[test]
    fn all_tank_levels_have_reachable_enemies_and_clear_spawn() {
        let mut names = std::collections::HashSet::new();
        for map in 0..TANK_MAP_COUNT {
            let mut tank = TankGame::new();
            tank.load_map(map);
            tank.spawn_wave();
            assert!(names.insert(tank.map_name()));
            assert!(!tank.blocked(3, 6));
            assert!(!tank.enemies.is_empty());
            let mut visited = std::collections::HashSet::from([tank.player]);
            let mut pending = vec![tank.player];
            while let Some((x, y)) = pending.pop() {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next = (x + dx, y + dy);
                    if !tank.blocked(next.0, next.1) && visited.insert(next) {
                        pending.push(next);
                    }
                }
            }
            for enemy in &tank.enemies {
                assert!(
                    visited.contains(&(enemy.x, enemy.y)),
                    "unreachable enemy in map {map}"
                );
            }
            for bush in &tank.bushes {
                assert!(!tank.blocked(bush.0, bush.1));
            }
        }
    }

    #[test]
    fn tank_progression_visits_all_twelve_maps_and_retry_keeps_level() {
        let mut tank = TankGame::new();
        for wave in 2..=13 {
            tank.lives = 2;
            tank.enemies.clear();
            tank.reserves = 0;
            tank.advance(0.01);
            assert_eq!(tank.wave, wave);
            assert_eq!(tank.map, ((wave - 1) % u32::from(TANK_MAP_COUNT)) as u8);
            assert_eq!(tank.lives, 3);
            assert!(!tank.enemies.is_empty());
            assert!(tank.bullets.is_empty());
        }
        tank.wave = 7;
        tank.score = 350;
        tank.failed = true;
        tank.lives = 0;
        tank.retry_level();
        assert_eq!(
            (tank.wave, tank.map, tank.score, tank.lives),
            (7, 6, 350, 3)
        );
        assert!(!tank.failed);
        assert_eq!(tank.enemies.len(), 4);

        tank.wave = 12;
        tank.score = 1200;
        tank.failed = true;
        tank.lives = 0;
        tank.retry_level();
        assert_eq!((tank.wave, tank.map, tank.score, tank.lives), (12, 11, 1200, 3));
        assert_eq!((tank.enemies.len(), tank.reserves), (5, 21));
        assert!(!tank.failed);
    }

    #[test]
    fn new_tank_maps_match_the_approved_grids_and_enemy_counts() {
        let layouts = [
            [".......", ".#...#.", "..#S#..", ".B...B.", "..#S#..", ".#...#.", "......."],
            [".......", "S.#.#.S", "..#.#..", "B.....B", "..#.#..", "S.#.#.S", "......."],
            [".......", ".##.##.", ".B.S.B.", ".S...S.", ".B.S.B.", ".#...#.", "......."],
            [".......", ".#S.S#.", ".B...B.", "..S.S..", ".B...B.", ".#S.S#.", "......."],
        ];
        let names = ["风车岔路", "曲径夹击", "双翼包围", "花环堡垒"];
        let totals = [20, 22, 24, 26];
        let active = [4, 4, 5, 5];
        for (index, rows) in layouts.iter().enumerate() {
            let mut tank = TankGame::new();
            tank.load_map((index + 8) as u8);
            tank.spawn_wave();
            assert_eq!(tank.map_name(), names[index]);
            assert_eq!(tank.level_total(), totals[index]);
            assert_eq!(tank.active_limit(), active[index]);
            assert_eq!(tank.enemies.len(), active[index]);
            for (y, row) in rows.iter().enumerate() {
                for (x, tile) in row.bytes().enumerate() {
                    let cell = (x as i8, y as i8);
                    let actual = if tank.walls.contains(&cell) {
                        b'#'
                    } else if tank.steel.contains(&cell) {
                        b'S'
                    } else if tank.bushes.contains(&cell) {
                        b'B'
                    } else {
                        b'.'
                    };
                    assert_eq!(actual, tile, "map {} at {cell:?}", index + 9);
                }
            }
        }
    }

    #[test]
    fn tank_reserves_refill_without_early_level_completion() {
        let mut tank = TankGame::new();
        assert_eq!(
            (tank.level_total(), tank.enemies.len(), tank.reserves),
            (10, 2, 8)
        );
        tank.enemies.clear();
        tank.advance(0.4);
        assert_eq!(tank.wave, 1);
        assert!(tank.enemies.is_empty());
        for _ in 0..5 {
            tank.advance(0.4);
        }
        assert_eq!((tank.wave, tank.enemies.len(), tank.reserves), (1, 1, 7));
        for _ in 0..30 {
            tank.advance(0.1);
        }
        assert_eq!(tank.enemies.len(), 2);
        assert_eq!(tank.reserves, 6);
        tank.failed = true;
        tank.retry_level();
        assert_eq!(
            (tank.defeated, tank.enemies.len(), tank.reserves),
            (0, 2, 8)
        );
    }

    #[test]
    fn tank_reinforcements_wait_when_spawn_points_are_occupied() {
        let mut tank = TankGame::new();
        tank.load_map(7);
        tank.spawn_wave();
        tank.enemies.truncate(3);
        tank.player = (0, 2);
        tank.walls.push((6, 2));
        let reserves = tank.reserves;
        tank.spawn_reinforcement();
        assert_eq!(tank.reserves, reserves);
        tank.player = (3, 6);
        tank.spawn_reinforcement();
        assert_eq!(tank.reserves, reserves - 1);
    }

    #[test]
    fn tank_maps_include_passable_bushes_and_solid_steel() {
        let mut tank = TankGame::new();
        tank.load_map(1);
        let bush = tank.bushes[0];
        let steel = tank.steel[0];
        assert!(!tank.blocked(bush.0, bush.1));
        assert!(tank.blocked(steel.0, steel.1));
        tank.load_map(2);
        assert_eq!(tank.map_name(), "钢墙要塞");
    }

    #[test]
    fn tank_enemy_warns_and_fires_without_player_in_its_lane() {
        let mut tank = TankGame::new();
        tank.enemies.truncate(1);
        tank.enemy_clock = 10.;
        tank.enemies[0].fire_clock = 0.;
        tank.advance(0.);
        assert_eq!(tank.enemies[0].warning, 0.9);
        assert!(tank.bullets.is_empty());
        let position = (tank.enemies[0].x, tank.enemies[0].y);
        let direction = tank.enemies[0].direction;
        // Move into the lane during warning: no tracking or surprise shot.
        tank.player = (0, 4);
        tank.enemy_clock = 0.;
        tank.advance(0.4);
        assert_eq!((tank.enemies[0].x, tank.enemies[0].y), position);
        assert_eq!(tank.enemies[0].direction, direction);
        assert!(tank.bullets.is_empty());
        tank.player = (3, 6);
        tank.advance(0.4);
        tank.advance(0.11);
        assert_eq!(tank.bullets.len(), 1);
        assert!(!tank.bullets[0].friendly);
        assert_eq!(tank.bullets[0].dx, direction.0 as f32);
        assert_eq!(tank.bullets[0].dy, direction.1 as f32);
    }

    #[test]
    fn tank_hit_grants_protection_against_followup_shells() {
        let mut tank = TankGame::new();
        tank.invulnerable = 0.;
        for expected_lives in [2, 2] {
            tank.bullets.push(TankBullet {
                x: 3.5,
                y: 6.5,
                dx: 0.,
                dy: 1.,
                friendly: false,
            });
            tank.advance(0.);
            assert_eq!(tank.lives, expected_lives);
            assert!(tank.invulnerable > 0.);
        }
    }

    #[test]
    fn sky_energy_shot_defeats_enemy_and_awards_score() {
        let mut sky = SkyGame::new();
        sky.enemies = vec![SkyEnemy {
            x: 0.5,
            y: 0.5,
            hp: 1,
            heavy: false,
        }];
        sky.shots = vec![SkyShot {
            x: 0.5,
            y: 0.5,
            dy: -0.92,
            friendly: true,
        }];
        sky.advance(0.);
        assert!(sky.enemies.is_empty());
        assert!(sky.score >= 30);
        assert_eq!(sky.combo, 1);
    }

    #[test]
    fn sky_ship_fires_automatically_without_a_button_press() {
        let mut sky = SkyGame::new();
        sky.shots.clear();
        sky.advance(0.);
        assert_eq!(
            sky.shots.iter().filter(|shot| shot.friendly).count(),
            2,
            "the ship should immediately emit its paired energy shots"
        );
    }

    #[test]
    fn sky_boss_arrives_at_score_threshold_and_can_be_destroyed() {
        let mut sky = SkyGame::new();
        sky.score = sky.next_boss_score;
        sky.advance(0.);
        assert!(sky.boss.is_some());
        sky.boss = Some(SkyBoss {
            x: 0.5,
            y: 0.5,
            hp: 1,
            max_hp: 1,
            phase: 0.,
        });
        sky.shots = vec![SkyShot {
            x: 0.5,
            y: 0.5,
            dy: -0.92,
            friendly: true,
        }];
        sky.advance(0.);
        assert!(sky.boss.is_none());
        assert!(sky.score >= 720);
    }

    #[test]
    fn beginner_bubble_assist_finds_a_real_scoring_shot() {
        let mut g = BubbleGame::new();
        let aim = g
            .recommended_aim()
            .expect("beginner board should offer a matching route");
        assert!(g.shot.is_none(), "assistance must not fire automatically");
        g.shoot(aim);
        for _ in 0..240 {
            g.advance(1. / 120.);
        }
        assert!(g.score > 0, "suggested route must actually remove bubbles");
        assert!(!g.failed);
    }

    #[test]
    fn bubble_aim_line_and_flight_use_identical_geometry() {
        let mut g = BubbleGame::new();
        g.cells.clear();
        g.aim = Vec2::new(0.85, 0.35);
        let dots = g.trajectory();
        let mut shot = g.shot_toward(g.aim);
        for i in 0..=12 {
            BubbleGame::move_shot(&mut shot, 1. / 120.);
            if i % 6 == 0 {
                assert!(dots[i / 6].distance(Vec2::new(shot.x, shot.y)) < 0.0001);
            }
        }
    }

    #[test]
    fn bubble_ammo_uses_remaining_colors_and_practice_does_not_descend() {
        let mut g = BubbleGame::new();
        g.cells = vec![BubbleCell {
            col: 0,
            row: 0,
            kind: BubbleKind::Mint,
        }];
        g.shot_count = 1;
        assert!(g.kind_after_shot() == BubbleKind::Mint);
        g.misses = 7;
        g.settle(BubbleShot {
            x: 0.8,
            y: 0.1,
            dx: 0.,
            kind: BubbleKind::Coral,
        });
        assert!(g.cells.iter().any(|cell| cell.col == 0 && cell.row == 0));
    }

    #[test]
    fn bomber_waits_then_protects_player_from_repeated_damage() {
        let mut g = BomberGame::new();
        let enemy = (g.enemies[0].x, g.enemies[0].y);
        for _ in 0..100 {
            g.advance(0.02);
        }
        assert_eq!((g.enemies[0].x, g.enemies[0].y), enemy);
        g.invulnerable = 0.;
        g.lose_life();
        assert_eq!(g.lives, 2);
        for _ in 0..60 {
            g.lose_life();
        }
        assert_eq!(g.lives, 2);
        assert!(g.invulnerable > 2.);
    }

    #[test]
    fn bomber_warning_matches_blast_and_movement_is_interpolated() {
        let mut g = BomberGame::new();
        g.bricks = vec![(2, 2)];
        let bomb = WaterBomb {
            x: 0,
            y: 2,
            fuse: 3.,
            range: 3,
        };
        let warning = g.blast_cells(&bomb);
        assert!(warning.contains(&(2, 2)));
        assert!(!warning.contains(&(3, 2)));
        g.explode(bomb);
        let actual: Vec<_> = g.blasts.iter().map(|b| (b.x, b.y)).collect();
        assert_eq!(warning, actual);
        g.move_player(1, 0);
        g.advance(0.02);
        assert_eq!(g.player, (3, 8));
        assert!(g.visual_player.x > 2. && g.visual_player.x < 3.);
        g.blasts.clear();
        g.enemy_clock = 10.;
        g.place_bomb();
        assert_eq!(g.bombs[0].fuse, 2.2);
        for _ in 0..40 {
            g.advance(0.05);
        }
        assert_eq!(g.bombs.len(), 1, "the water bubble must not burst early");
        assert!(g.blasts.is_empty());
        for _ in 0..5 {
            g.advance(0.05);
        }
        assert!(g.bombs.is_empty());
        assert!(g.blasts.iter().any(|blast| (blast.x, blast.y) == (3, 8)));
    }

    #[test]
    fn bubble_three_matching_colors_pop_and_score() {
        let mut bubble = BubbleGame::new();
        bubble.cells = vec![
            BubbleCell {
                col: 2,
                row: 0,
                kind: BubbleKind::Coral,
            },
            BubbleCell {
                col: 3,
                row: 0,
                kind: BubbleKind::Coral,
            },
        ];
        bubble.settle(BubbleShot {
            x: 0.42,
            y: 0.225,
            dx: 0.,
            kind: BubbleKind::Coral,
        });
        assert!(bubble.score >= 36);
        assert_eq!(bubble.level, 2, "clearing the cluster opens the next level");
        assert!(!bubble.cells.is_empty(), "the next level should be seeded");
    }

    #[test]
    fn bomber_water_bubble_breaks_bricks_and_defeats_enemy() {
        let mut bomber = BomberGame::new();
        bomber.bricks = vec![(2, 1)];
        bomber.enemies = vec![BomberEnemy {
            visual: Vec2::ZERO,
            x: 4,
            y: 2,
            step: 0,
            kind: 0,
        }];
        bomber.explode(WaterBomb {
            x: 2,
            y: 2,
            fuse: 0.,
            range: 2,
        });
        assert!(!bomber.bricks.contains(&(2, 1)));
        assert!(bomber.enemies.is_empty());
        assert!(bomber.score >= 68);
        assert!(!bomber.blasts.is_empty());
    }

    #[test]
    fn fruit_slash_cuts_fruit_but_leaves_distant_bomb() {
        let mut fruit = FruitGame::new();
        fruit.fruits = vec![
            Fruit {
                x: 0.5,
                y: 0.5,
                vx: 0.,
                vy: 0.,
                kind: 0,
                bomb: false,
            },
            Fruit {
                x: 0.9,
                y: 0.9,
                vx: 0.,
                vy: 0.,
                kind: 1,
                bomb: true,
            },
        ];
        fruit.slice_between(Vec2::new(0.3, 0.5), Vec2::new(0.7, 0.5));
        assert_eq!(fruit.fruits.len(), 1);
        assert!(fruit.fruits[0].bomb);
        assert!(fruit.score > 0);
        assert_eq!(fruit.lives, 3);
        assert_eq!(fruit.slices.len(), 1);
        assert!(fruit.slices[0].score > 0);
        assert!(fruit.slices[0].split.y > 0.99);
        assert!(fruit.slices[0].slash.x > 0.99);
        assert!((fruit.slices[0].life - 0.82).abs() < f32::EPSILON);
        assert!(fruit.hit_flash > 0.);
    }

    #[test]
    fn fruit_slash_grazing_a_bomb_keeps_the_successful_cut_safe() {
        let mut fruit = FruitGame::new();
        fruit.fruits = vec![
            Fruit {
                x: 0.50,
                y: 0.50,
                vx: 0.,
                vy: 0.,
                kind: 0,
                bomb: false,
            },
            Fruit {
                x: 0.50,
                y: 0.55,
                vx: 0.,
                vy: 0.,
                kind: 1,
                bomb: true,
            },
        ];
        fruit.slice_between(Vec2::new(0.30, 0.50), Vec2::new(0.70, 0.50));
        assert_eq!(fruit.fruits.len(), 1);
        assert!(fruit.fruits[0].bomb);
        assert!(fruit.score > 0);
        assert_eq!(fruit.lives, 3);
    }

    #[test]
    fn fruit_chain_cut_rewards_a_short_lived_combo_and_slice_effects() {
        let mut fruit = FruitGame::new();
        fruit.spawn_clock = 99.;
        fruit.fruits = vec![
            Fruit {
                x: 0.42,
                y: 0.50,
                vx: 0.,
                vy: 0.,
                kind: 0,
                bomb: false,
            },
            Fruit {
                x: 0.58,
                y: 0.50,
                vx: 0.,
                vy: 0.,
                kind: 5,
                bomb: false,
            },
        ];
        fruit.slice_between(Vec2::new(0.25, 0.50), Vec2::new(0.75, 0.50));
        assert!(fruit.fruits.is_empty());
        assert_eq!(fruit.combo, 2);
        assert_eq!(fruit.best_combo, 2);
        assert_eq!(fruit.slices.len(), 2);
        assert!(
            fruit.score >= 55,
            "gold fruit and a two-fruit cut should feel rewarding"
        );
        fruit.advance(1.2);
        assert_eq!(fruit.combo, 0);
    }

    #[test]
    fn fruit_bombs_do_not_appear_in_the_first_eight_waves() {
        let mut fruit = FruitGame::new();
        for _ in 0..8 {
            fruit.spawn();
        }
        assert!(fruit.fruits.iter().all(|fruit| !fruit.bomb));
        fruit.spawn();
        assert!(fruit.fruits.iter().any(|fruit| fruit.bomb));
    }

    #[test]
    fn touch_tap_tolerates_jitter_and_uses_release_location() {
        let mut gesture = Gesture {
            start: Some(Vec2::new(100., 100.)),
            last: Vec2::new(102., 101.),
            touch: Some(1),
            world: true,
            ..default()
        };
        let released = Vec2::new(106., 103.);
        assert_eq!(gesture.finish_tap(Some(released), 1.), Some(released));
        assert_eq!(gesture.finish_tap(Some(released), 1.), None);
    }

    #[test]
    fn drag_pinch_and_toolbar_gestures_never_place_blocks() {
        for (moved, world, pinch) in [
            (true, true, None),
            (false, false, None),
            (false, true, Some(50.)),
        ] {
            let mut gesture = Gesture {
                start: Some(Vec2::ZERO),
                last: Vec2::ZERO,
                touch: Some(1),
                moved,
                world,
                pinch,
            };
            assert_eq!(gesture.finish_tap(Some(Vec2::ZERO), 1.), None);
        }
    }

    #[test]
    fn large_final_touch_motion_is_not_a_tap() {
        let mut gesture = Gesture {
            start: Some(Vec2::ZERO),
            touch: Some(1),
            world: true,
            ..default()
        };
        // A release can contain motion that did not arrive in a held frame.
        assert_eq!(gesture.finish_tap(Some(Vec2::new(30., 0.)), 1.), None);
    }

    #[test]
    fn ray_hits_top_and_misses_parallel_box() {
        let (t, n) = ray_box(Vec3::new(0., 5., 0.), Vec3::NEG_Y, Vec3::ZERO).unwrap();
        assert!((t - 4.51).abs() < 0.001);
        assert_eq!(n, Vec3::Y);
        assert!(ray_box(Vec3::new(2., 5., 0.), Vec3::NEG_Y, Vec3::ZERO).is_none());
    }
}

#[derive(Component)]
struct MathTraveler;

#[derive(Component)]
struct MathBob {
    base: Vec3,
    phase: f32,
    rabbit: bool,
}

fn spawn_math_scene(
    commands: &mut Commands,
    game: &crate::math::MathGame,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let block = meshes.add(Cuboid::new(1., 1., 1.));
    let sphere = meshes.add(Sphere::new(1.).mesh().uv(20, 12));
    let grass = materials.add(Color::srgb(0.43, 0.76, 0.56));
    let soil = materials.add(Color::srgb(0.73, 0.57, 0.40));
    let cream = materials.add(Color::srgb(1., 0.96, 0.85));
    let white = materials.add(Color::srgb(1., 0.99, 0.95));
    let pink = materials.add(Color::srgb(0.98, 0.60, 0.68));
    let black = materials.add(Color::srgb(0.13, 0.17, 0.23));
    let orange = materials.add(Color::srgb(1., 0.48, 0.15));
    let green = materials.add(Color::srgb(0.16, 0.52, 0.35));
    let violet = materials.add(Color::srgb(0.53, 0.43, 0.83));
    let gold = materials.add(Color::srgb(1., 0.76, 0.22));
    let mut part = |pos: Vec3, scale: Vec3, mat: Handle<StandardMaterial>, round: bool| {
        let mut entity = commands.spawn((
            WorldPiece,
            Mesh3d(if round { sphere.clone() } else { block.clone() }),
            MeshMaterial3d(mat),
            Transform::from_translation(pos).with_scale(scale),
        ));
        if pos.y >= 0.6 && pos.y < 1.5 && pos.x.abs() < 4. && pos.z >= 0. && pos.z < 2.5 {
            entity.insert(DropIn::new(pos, ((pos.x + 3.) * 0.03).max(0.)));
        }
    };
    part(
        Vec3::new(0., -0.85, 0.),
        Vec3::new(8.3, 0.5, 5.8),
        soil.clone(),
        true,
    );
    part(
        Vec3::new(0., -0.35, 0.),
        Vec3::new(8.4, 0.6, 5.9),
        grass.clone(),
        true,
    );
    part(
        Vec3::new(0., 0.18, 1.),
        Vec3::new(8., 0.22, 5.),
        cream.clone(),
        false,
    );
    // Five small destinations gradually receive a reward house.
    for i in 0..5 {
        let x = -5.6 + i as f32 * 2.7;
        part(
            Vec3::new(x, 0.4, -3.4),
            Vec3::new(0.6, 0.6, 0.6),
            green.clone(),
            true,
        );
        part(
            Vec3::new(x, 0.1, -3.4),
            Vec3::new(0.15, 0.8, 0.15),
            soil.clone(),
            false,
        );
        if game.stars[i] > 0 {
            part(
                Vec3::new(x, 0.7, -2.5),
                Vec3::new(1.1, 1., 0.9),
                if i % 2 == 0 {
                    pink.clone()
                } else {
                    violet.clone()
                },
                false,
            );
            part(
                Vec3::new(x, 1.35, -2.5),
                Vec3::new(0.9, 0.45, 0.8),
                gold.clone(),
                true,
            );
            part(
                Vec3::new(x, 0.55, -1.99),
                Vec3::new(0.3, 0.5, 0.06),
                white.clone(),
                false,
            );
        }
    }
    for i in 0..game.placed {
        let units = if game.level == 0 { 1 } else { 10 };
        let display_count = if units == 1 {
            game.placed
        } else {
            game.placed / 10 + game.placed % 10
        };
        if i >= display_count {
            break;
        }
        let pos = Vec3::new(
            -2.8 + (i % 8) as f32 * 0.8,
            0.62 + (i / 24) as f32 * 0.6,
            (i / 8 % 3) as f32 * 0.95,
        );
        let ten = units == 10 && i < game.placed / 10;
        part(
            pos,
            if ten {
                Vec3::new(0.55, 0.22, 0.65)
            } else {
                Vec3::new(0.23, 0.38, 0.23)
            },
            if ten {
                gold.clone()
            } else if game.level == 0 {
                orange.clone()
            } else {
                violet.clone()
            },
            game.level == 0,
        );
        if game.level == 0 {
            part(
                pos + Vec3::Y * 0.4,
                Vec3::new(0.08, 0.2, 0.08),
                green.clone(),
                false,
            );
        }
    }
    drop(part);
    // Clickable supply and tray use broad hit targets above their visible surfaces.
    commands.spawn((
        WorldPiece,
        Mesh3d(block.clone()),
        MeshMaterial3d(orange.clone()),
        Transform::from_xyz(-5.3, 0.7, 1.).with_scale(Vec3::new(1.5, 0.6, 1.5)),
    ));
    for i in 0..3 {
        let base = Vec3::new(-7. + i as f32 * 6., 3.5, -5.);
        commands.spawn((
            WorldPiece,
            MathBob {
                base,
                phase: i as f32,
                rabbit: false,
            },
            Mesh3d(sphere.clone()),
            MeshMaterial3d(white.clone()),
            Transform::from_translation(base).with_scale(Vec3::new(1.7, 0.45, 0.6)),
        ));
    }
    let base = Vec3::new(5.1, 0.65, 1.);
    commands
        .spawn((
            WorldPiece,
            MathBob {
                base,
                phase: 0.,
                rabbit: true,
            },
            Transform::from_translation(base),
            Visibility::default(),
        ))
        .with_children(|p| {
            for (pos, scale, mat) in [
                (Vec3::ZERO, Vec3::new(0.65, 0.8, 0.55), white.clone()),
                (Vec3::Y * 0.9, Vec3::splat(0.62), white.clone()),
                (
                    Vec3::new(-0.3, 1.7, 0.),
                    Vec3::new(0.18, 0.65, 0.19),
                    white.clone(),
                ),
                (
                    Vec3::new(0.3, 1.7, 0.),
                    Vec3::new(0.18, 0.65, 0.19),
                    white.clone(),
                ),
                (
                    Vec3::new(-0.3, 1.7, 0.15),
                    Vec3::new(0.08, 0.48, 0.05),
                    pink.clone(),
                ),
                (
                    Vec3::new(0.3, 1.7, 0.15),
                    Vec3::new(0.08, 0.48, 0.05),
                    pink.clone(),
                ),
                (
                    Vec3::new(-0.22, 1., 0.55),
                    Vec3::splat(0.075),
                    black.clone(),
                ),
                (Vec3::new(0.22, 1., 0.55), Vec3::splat(0.075), black.clone()),
                (Vec3::new(0., 0.78, 0.60), Vec3::splat(0.08), pink.clone()),
            ] {
                p.spawn((
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_translation(pos).with_scale(scale),
                ));
            }
        });
    if game.solved {
        let burst_count = (10 + game.correct_total as usize * 8).min(66);
        let burst_colors = [
            gold.clone(),
            pink.clone(),
            orange.clone(),
            violet.clone(),
            green.clone(),
        ];
        for i in 0..burst_count {
            let angle = i as f32 * std::f32::consts::TAU / burst_count as f32;
            let ring = 3.6 + (i % 3) as f32 * 0.45;
            let base = Vec3::new(
                angle.cos() * ring,
                1.7 + (i % 5) as f32 * 0.34,
                angle.sin() * 2.8,
            );
            commands.spawn((
                WorldPiece,
                MathBob {
                    base,
                    phase: i as f32 * 0.47,
                    rabbit: false,
                },
                Mesh3d(if i % 3 == 0 {
                    block.clone()
                } else {
                    sphere.clone()
                }),
                MeshMaterial3d(burst_colors[i % burst_colors.len()].clone()),
                Transform::from_translation(base).with_scale(Vec3::splat(if i % 3 == 0 {
                    0.22
                } else {
                    0.16
                })),
            ));
        }
        if game.correct_total >= 4 {
            for i in 0..9 {
                let base = Vec3::new(-6.4 + i as f32 * 1.6, 4.2, -4.8);
                commands.spawn((
                    WorldPiece,
                    MathBob {
                        base,
                        phase: i as f32 * 0.38,
                        rabbit: false,
                    },
                    Mesh3d(block.clone()),
                    MeshMaterial3d(burst_colors[i % burst_colors.len()].clone()),
                    Transform::from_translation(base).with_scale(Vec3::new(0.32, 0.42, 0.12)),
                ));
            }
        }
    }
}

fn animate_math_island(
    time: Res<Time>,
    s: Res<Session>,
    mut objects: Query<(&mut Transform, &MathBob)>,
) {
    if !matches!(s.mode, GameMode::Math | GameMode::Literacy) {
        return;
    }
    let math_energy = (s.math.correct_total.min(6) as f32) * 0.12;
    let celebrating = if s.mode == GameMode::Literacy {
        s.literacy.finished || (s.literacy.checked && s.literacy.correct)
    } else {
        s.math.solved
    };
    for (mut t, bob) in &mut objects {
        let phase = time.elapsed_secs()
            * if bob.rabbit && celebrating {
                7. + s.math.correct_total.min(6) as f32 * 0.8
            } else {
                1.8 + math_energy
            }
            + bob.phase;
        t.translation.y = bob.base.y
            + if bob.rabbit && celebrating {
                phase.sin().abs() * (0.7 + math_energy)
            } else {
                phase.sin() * 0.16
            };
        if bob.rabbit {
            t.rotation = Quat::from_rotation_z(if celebrating { phase.sin() * 0.12 } else { 0. });
        }
    }
}

fn math_island_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut s: ResMut<Session>,
    blockers: Query<(&ComputedNode, &GlobalTransform), Or<(With<Button>, With<WorldInputBlocker>)>>,
) {
    if s.mode != GameMode::Math
        || s.help_open
        || !s.math_tools
        || s.library_open
        || s.math.choosing_level
        || s.math.solved
    {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let position = touches
        .iter_just_pressed()
        .next()
        .map(|t| t.position())
        .or_else(|| {
            if mouse.just_pressed(MouseButton::Left) {
                window.cursor_position()
            } else {
                None
            }
        });
    let Some(position) = position else {
        return;
    };
    if blockers.iter().any(|(node, transform)| {
        Rect::from_center_size(transform.translation().truncate(), node.size())
            .contains(position * window.scale_factor())
    }) {
        return;
    }
    if position.y < window.height() * 0.22 || position.y > window.height() * 0.76 {
        return;
    }
    let Ok((camera, transform)) = cameras.single() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, position) else {
        return;
    };
    if ray.direction.y.abs() < 0.001 {
        return;
    }
    let point = ray.origin + ray.direction * ((0.7 - ray.origin.y) / ray.direction.y);
    let delta = if (point.x + 5.3).abs() < 1.3 && (point.z - 1.).abs() < 1.5 {
        1
    } else if point.x.abs() < 4. && (point.z - 1.).abs() < 2.5 {
        -1
    } else {
        0
    };
    if delta != 0 {
        s.math.place(delta);
        s.ui_dirty = true;
        s.world_dirty = true;
    }
}

fn math_answer_button(parent: &mut ChildSpawnerCommands, art: &Art, value: i32, fill: Color) {
    let ink = Color::srgb(0.10, 0.11, 0.30);
    parent
        .spawn((
            Node {
                width: Val::Px(88.),
                height: Val::Px(66.),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            BackgroundColor(ink),
            BorderRadius::all(Val::Px(17.)),
        ))
        .with_children(|shadow| {
            shadow
                .spawn((
                    Button,
                    ButtonTone(fill),
                    Action::MathAnswer(value),
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(60.),
                        min_height: Val::Px(60.),
                        border: UiRect::all(Val::Px(3.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(fill),
                    BorderColor(ink),
                    BorderRadius::all(Val::Px(16.)),
                ))
                .with_children(|button| label(button, art, value.to_string(), 27., ink));
        });
}

fn math_ui(commands: &mut Commands, s: &Session, art: &Art) {
    if s.math.choosing_level {
        math_levels_ui(commands, s, art);
        return;
    }
    let g = &s.math;
    let stage = (g.correct_total / 2).min(3) as usize;
    let stage_accent = [
        Color::srgb(0.23, 0.42, 0.96),
        Color::srgb(0.96, 0.36, 0.65),
        Color::srgb(0.97, 0.62, 0.10),
        Color::srgb(0.62, 0.37, 0.96),
    ][stage];
    let stage_glow = [
        Color::srgb(1.0, 0.97, 0.91),
        Color::srgb(0.86, 0.91, 1.0),
        Color::srgb(1.0, 0.92, 0.72),
        Color::srgb(1.0, 0.84, 0.93),
    ][stage];
    let celebration = ["安静练习", "灯光亮起", "音乐加入", "庆典时刻"][stage];
    let decorations = ["", "✦", "✦  ♪  ✦", "✦  ♪  ★  ♫  ✦"][stage];
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                max_width: Val::Px(460.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_self: AlignSelf::Center,
                padding: UiRect::all(Val::Px(14.)),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                WorldInputBlocker,
                Node {
                    padding: UiRect::all(Val::Px(12.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BackgroundColor(stage_glow),
                BorderColor(Color::srgb(0.10, 0.11, 0.30)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|p| {
                p.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                })
                .with_children(|p| {
                    label(p, art, "数学游乐岛", 23., stage_accent);
                    label(
                        p,
                        art,
                        format!("{} · 第 {}/8 题", crate::math::LEVELS[g.level], g.index + 1),
                        14.,
                        Color::srgb(0.10, 0.11, 0.30),
                    );
                    label(
                        p,
                        art,
                        format!("★ {}", g.stars.iter().sum::<usize>()),
                        17.,
                        stage_accent,
                    );
                });
                p.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "选难度",
                        Action::MathLevels,
                        86.,
                        36.,
                        PAPER,
                        stage_accent,
                    );
                    button(
                        p,
                        art,
                        "怎么玩",
                        Action::Help,
                        86.,
                        36.,
                        PAPER,
                        stage_accent,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        86.,
                        36.,
                        PAPER,
                        stage_accent,
                    );
                });
                label(
                    p,
                    art,
                    g.question().prompt.replace('\n', " "),
                    26.,
                    Color::srgb(0.10, 0.11, 0.30),
                );
                p.spawn(row()).with_children(|p| {
                    label(
                        p,
                        art,
                        format!("答对 {} 题", g.correct_total),
                        15.,
                        stage_accent,
                    );
                    label(p, art, celebration, 15., Color::srgb(0.10, 0.11, 0.30));
                    if !decorations.is_empty() {
                        label(p, art, decorations, 17., stage_accent);
                    }
                });
                p.spawn(row()).with_children(|p| {
                    label(p, art, "本关", 13., Color::srgb(0.10, 0.11, 0.30));
                    for i in 0..8 {
                        let complete = i < g.index;
                        let current = i == g.index;
                        p.spawn((
                            Node {
                                width: Val::Px(17.),
                                height: Val::Px(17.),
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            BackgroundColor(if complete {
                                Color::srgb(1.0, 0.82, 0.22)
                            } else if current {
                                stage_accent
                            } else {
                                Color::WHITE
                            }),
                            BorderColor(Color::srgb(0.10, 0.11, 0.30)),
                            BorderRadius::all(Val::Px(50.)),
                        ));
                    }
                });
            });
            root.spawn((
                WorldInputBlocker,
                Node {
                    padding: UiRect::all(Val::Px(12.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(7.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BackgroundColor(stage_glow),
                BorderColor(Color::srgb(0.10, 0.11, 0.30)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|p| {
                label(
                    p,
                    art,
                    if g.solved {
                        format!("答对啦！{}", g.question().explanation)
                    } else if g.hinted {
                        g.question().hint.into()
                    } else if g.wrong.is_some() {
                        format!("再试一次！舞台已经成长到第 {} 阶段。", stage + 1)
                    } else {
                        "看题目，点下面的数字。拿不准就点「给我提示」。".into()
                    },
                    16.,
                    stage_accent,
                );
                p.spawn(row()).with_children(|p| {
                    for (index, value) in g.choices().into_iter().enumerate() {
                        let color = if g.solved && value == g.question().answer {
                            Color::srgb(1.0, 0.82, 0.22)
                        } else if g.wrong == Some(value) {
                            Color::srgb(0.78, 0.67, 1.0)
                        } else {
                            [
                                Color::srgb(1.0, 0.49, 0.69),
                                Color::srgb(0.43, 0.65, 1.0),
                                Color::srgb(1.0, 0.82, 0.22),
                                Color::srgb(0.35, 0.86, 0.68),
                            ][index]
                        };
                        math_answer_button(p, art, value, color);
                    }
                });
                p.spawn(row()).with_children(|p| {
                    if g.solved {
                        button(
                            p,
                            art,
                            "答对了，下一题 →",
                            Action::MathNext,
                            220.,
                            52.,
                            stage_accent,
                            Color::WHITE,
                        );
                    } else {
                        button(
                            p,
                            art,
                            "给我提示",
                            Action::MathHint,
                            120.,
                            48.,
                            PAPER,
                            stage_accent,
                        );
                        button(
                            p,
                            art,
                            if s.math_tools {
                                "收起实物工具"
                            } else {
                                "我想摆实物"
                            },
                            Action::MathTools,
                            150.,
                            48.,
                            PAPER,
                            INK,
                        );
                    }
                });
                if s.math_tools && !g.solved {
                    p.spawn(row()).with_children(|p| {
                        button(
                            p,
                            art,
                            "拿回 1",
                            Action::MathPlace(-1),
                            85.,
                            48.,
                            PAPER,
                            INK,
                        );
                        button(p, art, "放入 1", Action::MathPlace(1), 85., 48., MINT, INK);
                        if g.level > 0 {
                            button(p, art, "+10", Action::MathPlace(10), 58., 48., MINT, INK);
                            button(p, art, "−10", Action::MathPlace(-10), 58., 48., PAPER, INK);
                        }
                        label(p, art, format!("已摆 {}", g.placed), 23., INK);
                        button(
                            p,
                            art,
                            if g.solved { "下一关" } else { "摆好啦" },
                            if g.solved {
                                Action::MathNext
                            } else {
                                Action::MathCheck
                            },
                            100.,
                            48.,
                            TEAL,
                            Color::WHITE,
                        );
                        button(p, art, "提示", Action::MathHint, 60., 38., PAPER, TEAL);
                        button(p, art, "转一转", Action::Rotate, 78., 44., PAPER, TEAL);
                    });
                }
                label(
                    p,
                    art,
                    if g.level == 0 {
                        "每根胡萝卜代表 1 · 答对解锁岛上小屋"
                    } else {
                        "紫色小积木代表 1 · 金色积木代表 10 · 答对建造奖励小屋"
                    },
                    13.,
                    MUTED,
                );
            });
        });
}

fn spawn_literacy_scene(
    commands: &mut Commands,
    game: &crate::literacy::Literacy,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let sphere = meshes.add(Sphere::new(1.).mesh().uv(28, 18));
    let cube = meshes.add(Cuboid::new(1., 1., 1.));
    let mint = materials.add(Color::srgb(0.29, 0.72, 0.58));
    let white = materials.add(Color::srgb(1., 0.99, 0.94));
    let dark = materials.add(Color::srgb(0.12, 0.22, 0.25));
    let pink = materials.add(Color::srgb(1., 0.60, 0.61));
    let gold = materials.add(Color::srgb(1., 0.72, 0.13));
    let blue = materials.add(Color::srgb(0.24, 0.62, 0.94));
    let brown = materials.add(Color::srgb(0.56, 0.35, 0.20));
    let red = materials.add(Color::srgb(1., 0.34, 0.16));
    let gray = materials.add(Color::srgb(0.51, 0.61, 0.65));
    let mut part = |pos: Vec3, size: Vec3, mat: Handle<StandardMaterial>, round: bool| {
        let mut entity = commands.spawn((
            WorldPiece,
            Mesh3d(if round { sphere.clone() } else { cube.clone() }),
            MeshMaterial3d(mat),
            Transform::from_translation(pos).with_scale(size),
        ));
        if pos.y > 0.5 {
            entity.insert(MathBob {
                base: pos,
                phase: 0.,
                rabbit: false,
            });
        }
    };
    part(
        Vec3::new(0., -2.0, 0.),
        Vec3::new(3.6, 0.25, 2.1),
        mint.clone(),
        true,
    );
    if !game.map && !game.finished {
        let center = Vec3::new(0., 1.6, 0.);
        match game.word_index() {
            0 => {
                part(center, Vec3::splat(0.95), gold.clone(), true);
                for i in 0..12 {
                    let a = i as f32 * std::f32::consts::TAU / 12.;
                    part(
                        center + Vec3::new(a.cos() * 1.35, a.sin() * 1.35, 0.),
                        Vec3::splat(0.14),
                        gold.clone(),
                        true,
                    );
                }
            }
            1 => {
                for i in 0..18 {
                    let a = 0.8 + i as f32 * 4.7 / 17.;
                    part(
                        center + Vec3::new(a.cos(), a.sin(), 0.),
                        Vec3::splat(0.24),
                        gold.clone(),
                        true,
                    );
                }
            }
            2 => {
                for (x, h) in [(-1.1, 1.3), (0., 2.0), (1.1, 1.1)] {
                    for j in 0..5 {
                        let t = j as f32 / 5.;
                        part(
                            center + Vec3::new(x, t * h - 0.8, 0.),
                            Vec3::new(1. - t * 0.8, h / 5., 0.8),
                            mint.clone(),
                            false,
                        );
                    }
                }
            }
            3 => {
                for row in 0..3 {
                    for i in 0..12 {
                        let x = i as f32 * 0.25 - 1.5;
                        part(
                            center
                                + Vec3::new(x, (x * 3.).sin() * 0.14 + row as f32 * 0.5 - 0.5, 0.),
                            Vec3::new(0.2, 0.12, 0.15),
                            blue.clone(),
                            true,
                        );
                    }
                }
            }
            4 => {
                part(
                    center - Vec3::Y * 0.6,
                    Vec3::new(0.35, 1.5, 0.35),
                    brown.clone(),
                    false,
                );
                for x in [-0.65, 0., 0.65] {
                    part(
                        center + Vec3::new(x, 0.45, 0.),
                        Vec3::new(0.8, 0.9, 0.6),
                        mint.clone(),
                        true,
                    );
                }
            }
            5 => {
                for (x, h) in [(-0.65, 0.8), (0., 1.25), (0.65, 0.7)] {
                    part(
                        center + Vec3::X * x,
                        Vec3::new(0.45, h, 0.3),
                        red.clone(),
                        true,
                    );
                }
                part(
                    center - Vec3::Y * 0.4,
                    Vec3::new(0.5, 0.65, 0.4),
                    gold.clone(),
                    true,
                );
            }
            6 => {
                part(
                    center - Vec3::Y * 0.55,
                    Vec3::new(2.6, 0.65, 1.4),
                    brown.clone(),
                    false,
                );
                part(
                    center + Vec3::Y * 0.2,
                    Vec3::new(0.10, 0.9, 0.10),
                    mint.clone(),
                    false,
                );
                part(
                    center + Vec3::new(0.25, 0.6, 0.),
                    Vec3::new(0.45, 0.2, 0.18),
                    mint.clone(),
                    true,
                );
            }
            7 => {
                for (x, y, r) in [(-0.7, -0.3, 0.6), (0.5, 0., 0.9), (0., -0.6, 0.5)] {
                    part(
                        center + Vec3::new(x, y, 0.),
                        Vec3::new(r, r * 0.7, r * 0.6),
                        gray.clone(),
                        true,
                    );
                }
            }
            8 => {
                part(center, Vec3::new(1.25, 0.67, 0.25), pink.clone(), true);
                part(
                    center + Vec3::Z * 0.2,
                    Vec3::new(0.95, 0.35, 0.15),
                    dark.clone(),
                    true,
                );
            }
            9 => {
                part(center, Vec3::new(1.3, 0.65, 0.3), white.clone(), true);
                part(
                    center + Vec3::Z * 0.3,
                    Vec3::new(0.4, 0.5, 0.15),
                    blue.clone(),
                    true,
                );
                part(
                    center + Vec3::Z * 0.45,
                    Vec3::new(0.22, 0.3, 0.1),
                    dark.clone(),
                    true,
                );
            }
            10 => {
                part(
                    center - Vec3::Y * 0.25,
                    Vec3::new(0.7, 0.65, 0.25),
                    pink.clone(),
                    true,
                );
                for i in 0..4 {
                    part(
                        center + Vec3::new(-0.5 + i as f32 * 0.33, 0.5, 0.),
                        Vec3::new(0.14, 0.7 - (i as f32 - 1.5).abs() * 0.13, 0.17),
                        pink.clone(),
                        true,
                    );
                }
                part(
                    center + Vec3::new(-0.85, -0.1, 0.),
                    Vec3::new(0.3, 0.16, 0.2),
                    pink.clone(),
                    true,
                );
            }
            _ => {
                part(
                    center - Vec3::Y * 0.2,
                    Vec3::new(0.5, 0.85, 0.23),
                    pink.clone(),
                    true,
                );
                for i in 0..5 {
                    part(
                        center + Vec3::new(-0.45 + i as f32 * 0.23, 0.7 - i as f32 * 0.07, 0.),
                        Vec3::splat(0.19 - i as f32 * 0.016),
                        pink.clone(),
                        true,
                    );
                }
            }
        }
    }
    drop(part);
    let base = Vec3::new(0., -1.0, 1.);
    commands
        .spawn((
            WorldPiece,
            MathBob {
                base,
                phase: 0.,
                rabbit: true,
            },
            Transform::from_translation(base),
            Visibility::default(),
        ))
        .with_children(|p| {
            for (pos, size, mat) in [
                (Vec3::ZERO, Vec3::new(0.72, 0.75, 0.48), mint.clone()),
                (
                    Vec3::new(-0.38, 0.67, 0.),
                    Vec3::new(0.22, 0.38, 0.22),
                    mint.clone(),
                ),
                (
                    Vec3::new(0.38, 0.67, 0.),
                    Vec3::new(0.22, 0.38, 0.22),
                    mint.clone(),
                ),
                (
                    Vec3::new(0., -0.15, 0.4),
                    Vec3::new(0.49, 0.4, 0.14),
                    white.clone(),
                ),
                (
                    Vec3::new(-0.24, 0.25, 0.44),
                    Vec3::splat(0.08),
                    dark.clone(),
                ),
                (Vec3::new(0.24, 0.25, 0.44), Vec3::splat(0.08), dark.clone()),
                (
                    Vec3::new(0., 0.06, 0.53),
                    Vec3::new(0.12, 0.07, 0.06),
                    pink.clone(),
                ),
                (
                    Vec3::new(-0.5, -0.65, 0.),
                    Vec3::new(0.3, 0.16, 0.28),
                    mint.clone(),
                ),
                (
                    Vec3::new(0.5, -0.65, 0.),
                    Vec3::new(0.3, 0.16, 0.28),
                    mint.clone(),
                ),
            ] {
                p.spawn((
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_translation(pos).with_scale(size),
                ));
            }
        });
}

fn literacy_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let g = &s.literacy;
    let green = Color::srgb(0.24, 0.61, 0.36);
    let light = Color::srgb(0.91, 0.96, 0.91);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(18.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                ..row()
            })
            .with_children(|p| {
                label(p, art, "识字小岛", 28., green);
                label(
                    p,
                    art,
                    format!(
                        "{} XP  ·  已完成 {}/3",
                        g.progress.xp,
                        g.progress.completed.iter().filter(|&&x| x).count()
                    ),
                    18.,
                    INK,
                );
                button(
                    p,
                    art,
                    "学习地图",
                    Action::LiteracyMap,
                    100.,
                    36.,
                    Color::WHITE,
                    green,
                );
                button(p, art, "怎么玩", Action::Help, 90., 44., PAPER, TEAL);
                button(
                    p,
                    art,
                    "游戏库",
                    Action::Library,
                    80.,
                    36.,
                    Color::WHITE,
                    green,
                );
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(44.),
                    right: Val::Px(18.),
                    top: Val::Px(76.),
                    bottom: Val::Px(76.),
                    padding: UiRect::all(Val::Px(20.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(7.),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|p| {
                if g.map {
                    label(p, art, "跟小芽一起，每天认识几个字", 22., INK);
                    label(p, art, "每站 4 个字 · 看图 / 拼音 / 组词", 15., MUTED);
                    for (i, name) in crate::literacy::UNITS.iter().enumerate() {
                        let words = crate::literacy::WORDS[i * 4..i * 4 + 4]
                            .iter()
                            .map(|w| w.character)
                            .collect::<Vec<_>>()
                            .join("  ");
                        button(
                            p,
                            art,
                            format!(
                                "{}  {}    {}",
                                if g.progress.completed[i] {
                                    "★"
                                } else {
                                    "○"
                                },
                                name,
                                words
                            ),
                            Action::LiteracyStart(i),
                            390.,
                            56.,
                            if g.progress.completed[i] {
                                light
                            } else {
                                green
                            },
                            if g.progress.completed[i] {
                                green
                            } else {
                                Color::WHITE
                            },
                        );
                    }
                } else if g.finished {
                    label(p, art, "★ 这一站完成啦！", 31., green);
                    label(p, art, "4 个汉字朋友，全部练习完成", 21., INK);
                    label(p, art, "做错的题也已经重新练过了", 16., MUTED);
                    label(p, art, "首次完成每站可获得 40 XP", 16., green);
                    button(
                        p,
                        art,
                        "回到学习地图",
                        Action::LiteracyMap,
                        260.,
                        48.,
                        green,
                        Color::WHITE,
                    );
                } else {
                    label(
                        p,
                        art,
                        format!(
                            "{}  ·  {}  ·  连对 {} 题",
                            crate::literacy::UNITS[g.unit],
                            if g.reviewing {
                                "错题复习".into()
                            } else {
                                format!("第 {}/4 个字", g.card + 1)
                            },
                            g.combo
                        ),
                        15.,
                        MUTED,
                    );
                    if g.stage == 0 {
                        label(p, art, "认识一个新朋友", 22., INK);
                        label(p, art, g.word().character, 64., green);
                        label(p, art, g.word().pinyin, 27., INK);
                        #[cfg(target_os = "macos")]
                        button(
                            p,
                            art,
                            "听一听读音",
                            Action::LiteracySpeak,
                            160.,
                            32.,
                            light,
                            green,
                        );
                        label(p, art, g.word().meaning, 17., MUTED);
                        label(p, art, format!("组词：{}", g.word().phrase), 21., INK);
                    } else {
                        label(
                            p,
                            art,
                            match g.stage {
                                1 => "看看左边的图，选出对应的字",
                                2 => "读一读拼音，选出对应的字",
                                _ => "选一个字，把词语补完整",
                            },
                            22.,
                            INK,
                        );
                        if g.stage == 2 {
                            label(p, art, g.word().pinyin, 46., green);
                        }
                        if g.stage == 3 {
                            label(p, art, g.word().blank, 21., INK);
                        }
                        p.spawn(Node {
                            flex_wrap: FlexWrap::Wrap,
                            width: Val::Px(360.),
                            column_gap: Val::Px(12.),
                            row_gap: Val::Px(7.),
                            ..row()
                        })
                        .with_children(|p| {
                            for value in g.options() {
                                let chosen = g.selected == Some(value);
                                let good = g.checked && value == g.word_index();
                                let bad = g.checked && chosen && !g.correct;
                                let bg = if good {
                                    light
                                } else if bad {
                                    Color::srgb(1., 0.88, 0.84)
                                } else if chosen {
                                    Color::srgb(0.84, 0.93, 1.)
                                } else {
                                    Color::srgb(0.96, 0.97, 0.96)
                                };
                                p.spawn((
                                    Button,
                                    ButtonTone(bg),
                                    Action::LiteracySelect(value),
                                    Node {
                                        width: Val::Px(170.),
                                        height: Val::Px(65.),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(bg),
                                    BorderRadius::all(Val::Px(14.)),
                                ))
                                .with_children(|p| {
                                    label(
                                        p,
                                        art,
                                        crate::literacy::WORDS[value].character,
                                        36.,
                                        if good { green } else { INK },
                                    );
                                });
                            }
                        });
                        if g.checked {
                            label(
                                p,
                                art,
                                if g.correct {
                                    "答对了！小芽为你跳个舞。".into()
                                } else {
                                    format!(
                                        "这是「{}」，读 {}。直接点正确的字，再试一次。",
                                        g.word().character,
                                        g.word().pinyin
                                    )
                                },
                                16.,
                                if g.correct {
                                    green
                                } else {
                                    Color::srgb(0.72, 0.31, 0.22)
                                },
                            );
                        }
                    }
                }
            });
            root.spawn(Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                ..row()
            })
            .with_children(|p| {
                label(
                    p,
                    art,
                    if g.map {
                        "小芽陪你学 · 不计时，放心试"
                    } else if g.reviewing {
                        "再见一次，就记得更牢"
                    } else {
                        "看一看 · 认一认 · 用一用"
                    },
                    17.,
                    green,
                );
                if !g.map && !g.finished {
                    if g.stage == 0 || g.checked {
                        button(
                            p,
                            art,
                            if g.stage == 0 {
                                "认识啦，去练习"
                            } else {
                                if g.correct {
                                    "答对了，继续 →"
                                } else {
                                    "先跳过，稍后练"
                                }
                            },
                            Action::LiteracyNext,
                            190.,
                            46.,
                            green,
                            Color::WHITE,
                        );
                    } else if g.selected.is_some() {
                        button(
                            p,
                            art,
                            "检查答案",
                            Action::LiteracyCheck,
                            190.,
                            46.,
                            green,
                            Color::WHITE,
                        );
                    } else {
                        label(p, art, "轻点汉字就能作答", 18., MUTED);
                    }
                }
            });
        });
}

fn animate_math(
    time: Res<Time>,
    s: Res<Session>,
    mut travelers: Query<&mut Node, With<MathTraveler>>,
) {
    for mut node in &mut travelers {
        let target = 4. + (s.math.index as f32 + if s.math.solved { 1. } else { 0. }) * 10.5;
        let current = if let Val::Percent(value) = node.left {
            value
        } else {
            target
        };
        node.left = Val::Percent(current + (target - current) * (time.delta_secs() * 6.).min(1.));
        node.top = Val::Px(2. + (time.elapsed_secs() * 3.).sin() * 3.);
    }
}

fn math_levels_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let game = &s.math;
    let question = game.question();
    let purple = Color::srgb(0.40, 0.34, 0.79);
    let gold = Color::srgb(1., 0.79, 0.32);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::axes(Val::Px(32.), Val::Px(16.)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.10, 0.23)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(960.),
                justify_content: JustifyContent::SpaceBetween,
                ..row()
            })
            .with_children(|p| {
                label(p, art, "数学星球  /  一年级", 27., Color::WHITE);
                label(
                    p,
                    art,
                    format!("已点亮 {} / 40 颗星", game.stars.iter().sum::<usize>()),
                    17.,
                    gold,
                );
                button(
                    p,
                    art,
                    "游戏库",
                    Action::Library,
                    90.,
                    36.,
                    SLATE,
                    Color::WHITE,
                );
            });
            root.spawn(row()).with_children(|p| {
                for (i, name) in crate::math::LEVELS.iter().enumerate() {
                    button(
                        p,
                        art,
                        format!("{}  {} / 8", name, game.stars[i]),
                        Action::MathLevel(i),
                        150.,
                        38.,
                        if i == game.level { purple } else { SLATE },
                        Color::WHITE,
                    );
                }
            });
            root.spawn((
                Node {
                    width: Val::Percent(90.),
                    max_width: Val::Px(800.),
                    height: Val::Px(42.),
                    min_height: Val::Px(42.),
                    justify_content: JustifyContent::SpaceAround,
                    ..row()
                },
                BackgroundColor(Color::srgb(0.13, 0.17, 0.32)),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|p| {
                for i in 0..8 {
                    label(
                        p,
                        art,
                        if i < game.stars[game.level] {
                            "★".to_owned()
                        } else {
                            format!("{}", i + 1)
                        },
                        21.,
                        if i < game.stars[game.level] {
                            gold
                        } else {
                            SOFT
                        },
                    );
                }
                p.spawn((
                    MathTraveler,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(4. + game.index as f32 * 10.5),
                        top: Val::Px(2.),
                        ..default()
                    },
                ))
                .with_children(|p| {
                    label(p, art, "◆", 28., MINT);
                });
            });
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(960.),
                    padding: UiRect::all(Val::Px(18.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.),
                    ..default()
                },
                BackgroundColor(PAPER),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|p| {
                label(
                    p,
                    art,
                    format!(
                        "{}星球  ·  第 {} / 8 关",
                        crate::math::LEVELS[game.level],
                        game.index + 1
                    ),
                    15.,
                    purple,
                );
                label(p, art, question.prompt, 26., INK);
                p.spawn(row()).with_children(|p| {
                    for value in game.choices() {
                        let color = if game.solved && value == question.answer {
                            TEAL
                        } else if game.wrong == Some(value) {
                            Color::srgb(0.73, 0.38, 0.32)
                        } else {
                            purple
                        };
                        button(
                            p,
                            art,
                            value.to_string(),
                            Action::MathAnswer(value),
                            130.,
                            48.,
                            color,
                            Color::WHITE,
                        );
                    }
                });
                let feedback = if game.solved {
                    format!("★ 答对了！ {}", question.explanation)
                } else if game.hinted {
                    format!("小提示：{}", question.hint)
                } else if game.wrong.is_some() {
                    "差一点，再想想！可以点下面的提示帮帮忙。".into()
                } else {
                    "慢慢想，不用抢时间。选出你的答案吧！".into()
                };
                label(
                    p,
                    art,
                    feedback,
                    17.,
                    if game.solved { TEAL } else { MUTED },
                );
                p.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "给我一个提示",
                        Action::MathHint,
                        148.,
                        38.,
                        Color::srgb(0.87, 0.89, 0.95),
                        INK,
                    );
                    if game.solved {
                        button(
                            p,
                            art,
                            if game.index == 7 {
                                if game.level == 4 {
                                    "完成冒险 · 再练一轮"
                                } else {
                                    "星球完成 · 下一站"
                                }
                            } else {
                                "收集星星 · 下一题"
                            },
                            Action::MathNext,
                            200.,
                            38.,
                            TEAL,
                            Color::WHITE,
                        );
                    }
                });
            });
            label(
                root,
                art,
                "数感启蒙 → 加减运算 → 生活应用 → 规律推理 → 竞赛思维",
                14.,
                SOFT,
            );
        });
}

fn ball_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    if compact {
        ball_portrait_ui(commands, s, art);
        return;
    }
    let upcoming = s
        .ball
        .track
        .iter()
        .skip(s.ball.next)
        .take(5)
        .map(|item| match item.kind {
            BallKind::Number(value) => value.to_string(),
            BallKind::Trap => "陷阱".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(" · ");
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(24.),
                    right: Val::Px(24.),
                    top: Val::Px(18.),
                    height: Val::Px(54.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                BackgroundColor(Color::srgba(0.05, 0.12, 0.16, 0.78)),
                BorderRadius::all(Val::Px(16.)),
            ))
            .with_children(|p| {
                p.spawn(row()).with_children(|p| {
                    label(p, art, "2048 球球", 26., Color::WHITE);
                    label(
                        p,
                        art,
                        format!(
                            "当前 {}  ·  得分 {}  ·  护盾 {}  ·  前方 {}",
                            s.ball.number, s.ball.score, s.ball.shields, upcoming
                        ),
                        17.,
                        MINT,
                    );
                });
                p.spawn(row()).with_children(|p| {
                    label(
                        p,
                        art,
                        if s.ball.failed {
                            "撞到陷阱"
                        } else if s.ball.finished {
                            "完成"
                        } else {
                            "向前滚动中"
                        },
                        16.,
                        Color::WHITE,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        100.,
                        42.,
                        SLATE,
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(24.),
                    right: Val::Px(24.),
                    bottom: Val::Px(18.),
                    padding: UiRect::all(Val::Px(12.)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                BackgroundColor(Color::srgba(0.05, 0.12, 0.16, 0.86)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|p| {
                p.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "← 左道",
                        Action::BallLeft,
                        112.,
                        46.,
                        SLATE,
                        Color::WHITE,
                    );
                    button(
                        p,
                        art,
                        "右道 →",
                        Action::BallRight,
                        112.,
                        46.,
                        SLATE,
                        Color::WHITE,
                    );
                });
                label(p, art, &s.message, 16., Color::WHITE);
                p.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        if s.ball.running {
                            "Ⅱ 暂停"
                        } else {
                            "▶ 开始"
                        },
                        Action::BallStart,
                        132.,
                        46.,
                        MINT,
                        NAVY,
                    );
                    button(
                        p,
                        art,
                        "再来一次",
                        Action::BallReset,
                        120.,
                        46.,
                        Color::srgb(0.92, 0.80, 0.62),
                        NAVY,
                    );
                });
            });
        });
}

// 2048 is played one-handed on a tall screen: the board stays clear in the
// middle, while both lanes get a generous, thumb-sized target at the bottom.
// Swiping across the track remains available for players who prefer gestures.
fn ball_portrait_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let upcoming = s
        .ball
        .track
        .iter()
        .skip(s.ball.next)
        .take(3)
        .map(|item| match item.kind {
            BallKind::Number(value) => value.to_string(),
            BallKind::Trap => "陷阱".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(" · ");
    let status = if s.ball.failed {
        "撞到陷阱"
    } else if s.ball.finished {
        "已完成"
    } else if s.ball.running {
        "滚动中"
    } else {
        "已暂停"
    };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(14.),
                    right: Val::Px(14.),
                    top: Val::Px(12.),
                    height: Val::Px(104.),
                    padding: UiRect::axes(Val::Px(12.), Val::Px(8.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.98, 0.96, 0.86, 0.92)),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|header| {
                header
                    .spawn(Node {
                        width: Val::Percent(100.),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..row()
                    })
                    .with_children(|line| {
                        line.spawn(row()).with_children(|p| {
                            label(p, art, "2048 球球", 25., Color::srgb(0.13, 0.35, 0.38));
                            label(p, art, status, 15., Color::srgb(0.20, 0.54, 0.47));
                        });
                        button(
                            line,
                            art,
                            "游戏库",
                            Action::Library,
                            84.,
                            34.,
                            Color::srgb(0.25, 0.55, 0.58),
                            Color::WHITE,
                        );
                    });
                header
                    .spawn(Node {
                        width: Val::Percent(100.),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..row()
                    })
                    .with_children(|line| {
                        label(
                            line,
                            art,
                            format!(
                                "当前 {}   得分 {}   护盾 {}",
                                s.ball.number, s.ball.score, s.ball.shields
                            ),
                            18.,
                            Color::srgb(0.10, 0.38, 0.44),
                        );
                        label(
                            line,
                            art,
                            format!("前方 {upcoming}"),
                            14.,
                            Color::srgb(0.37, 0.48, 0.45),
                        );
                    });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(14.),
                    right: Val::Px(14.),
                    bottom: Val::Px(12.),
                    height: Val::Px(164.),
                    padding: UiRect::all(Val::Px(8.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.98, 0.95, 0.82, 0.94)),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|controls| {
                controls
                    .spawn(Node {
                        width: Val::Percent(100.),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..row()
                    })
                    .with_children(|line| {
                        label(
                            line,
                            art,
                            "滑动或点按换道",
                            14.,
                            Color::srgb(0.25, 0.43, 0.42),
                        );
                        line.spawn(row()).with_children(|p| {
                            button(
                                p,
                                art,
                                if s.ball.running { "暂停" } else { "开始" },
                                Action::BallStart,
                                70.,
                                34.,
                                Color::srgb(0.99, 0.73, 0.24),
                                Color::srgb(0.19, 0.34, 0.35),
                            );
                            button(
                                p,
                                art,
                                "重来",
                                Action::BallReset,
                                62.,
                                34.,
                                Color::srgb(0.83, 0.87, 0.77),
                                Color::srgb(0.19, 0.34, 0.35),
                            );
                        });
                    });
                controls
                    .spawn(Node {
                        width: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(10.),
                        ..row()
                    })
                    .with_children(|line| {
                        button(
                            line,
                            art,
                            "←  左道",
                            Action::BallLeft,
                            220.,
                            78.,
                            Color::srgb(0.20, 0.56, 0.79),
                            Color::WHITE,
                        );
                        button(
                            line,
                            art,
                            "右道  →",
                            Action::BallRight,
                            220.,
                            78.,
                            Color::srgb(0.95, 0.56, 0.22),
                            Color::WHITE,
                        );
                    });
            });
        });
}

fn mission_index(mode: GameMode) -> Option<usize> {
    match mode {
        GameMode::Firefighters => Some(0),
        GameMode::Detective => Some(1),
        GameMode::SpaceFarm => Some(2),
        _ => None,
    }
}
fn tetris_color(cell: u8) -> Color {
    match cell {
        1 => Color::srgb(0.20, 0.82, 0.94),
        2 => Color::srgb(0.96, 0.78, 0.20),
        3 => Color::srgb(0.72, 0.35, 0.94),
        4 => Color::srgb(0.30, 0.82, 0.47),
        5 => Color::srgb(0.94, 0.32, 0.40),
        6 => Color::srgb(0.28, 0.44, 0.91),
        7 => Color::srgb(0.94, 0.54, 0.20),
        _ => Color::srgb(0.10, 0.13, 0.25),
    }
}

fn tetris_stat(parent: &mut ChildSpawnerCommands, art: &Art, title: &str, value: String) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.),
                padding: UiRect::axes(Val::Px(10.), Val::Px(9.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                ..row()
            },
            BackgroundColor(Color::srgb(0.035, 0.065, 0.14)),
            BorderColor(Color::srgb(0.17, 0.37, 0.72)),
            BorderRadius::all(Val::Px(12.)),
        ))
        .with_children(|card| {
            label(card, art, title, 14., Color::srgb(0.64, 0.78, 0.98));
            label(card, art, value, 22., Color::srgb(0.42, 0.85, 1.));
        });
}

fn tetris_control_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    icon: &str,
    caption: &str,
    action: Action,
    button_size: f32,
    background: Color,
    edge: Color,
) {
    parent
        .spawn(Node {
            width: Val::Percent(20.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.),
            ..default()
        })
        .with_children(|control| {
            control
                .spawn((
                    Button,
                    ButtonTone(background),
                    action,
                    Node {
                        width: Val::Px(button_size),
                        height: Val::Px(button_size),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(2.)),
                        flex_shrink: 0.,
                        ..default()
                    },
                    BackgroundColor(background),
                    BorderColor(edge),
                    BorderRadius::all(Val::Percent(50.)),
                ))
                .with_children(|button| label(button, art, icon, 27., Color::WHITE));
            label(control, art, caption, 13., Color::srgb(0.83, 0.90, 1.));
        });
}

fn tetris_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    compact: bool,
    width: f32,
    height: f32,
) {
    let g = &s.tetris;
    // Desktop is a view-only game surface: let the well consume almost the
    // entire available height instead of keeping phone-sized 32px cells in a
    // large fullscreen window.
    let content_width = if compact {
        (width - 20.).max(320.)
    } else {
        980.
    };
    let cell = if compact {
        // The portrait preview has a compact control row; use that saved space
        // to make the playfield substantially larger.
        (((height - 190.) / TETRIS_HEIGHT as f32).min((content_width - 160.) / TETRIS_WIDTH as f32))
            .floor()
            .clamp(22., 36.)
    } else {
        ((height - 160.) / TETRIS_HEIGHT as f32)
            .min((width - 250.) / TETRIS_WIDTH as f32)
            .floor()
            .clamp(32., 64.)
    };
    let board_width = cell * TETRIS_WIDTH as f32;
    let board_height = cell * TETRIS_HEIGHT as f32;
    let side_width = if compact {
        (content_width - board_width - 12.).clamp(100., 140.)
    } else {
        188.
    };
    let preview_cell = if compact {
        cell.min((side_width - 8.) / 4.)
    } else {
        cell
    };
    let mut ghost_y = g.y;
    while !g.failed && g.valid(g.x, ghost_y + 1, g.rotation) {
        ghost_y += 1;
    }
    let ghost_cells = TetrisGame::cells(g.kind, g.rotation).map(|(x, y)| (g.x + x, ghost_y + y));
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(if compact { 8. } else { 22. })),
                border: UiRect::all(Val::Px(if compact { 2. } else { 5. })),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(if compact { 8. } else { 10. }),
                ..default()
            },
            BackgroundColor(Color::srgb(0.025, 0.035, 0.12)),
            BorderColor(Color::srgb(0.12, 0.39, 0.87)),
            BorderRadius::all(Val::Px(18.)),
        ))
        .with_children(|root| {
            if compact {
                let mut backdrop = ImageNode::new(art.sky_backdrop.clone());
                backdrop.image_mode = NodeImageMode::Stretch;
                root.spawn((
                    backdrop,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                ));
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.008, 0.018, 0.06, 0.70)),
                ));
            }
            if compact {
                for (x, y, symbol, color) in [
                    (16., 148., "✦", Color::srgb(0.30, 0.71, 1.)),
                    (content_width - 28., 320., "✧", Color::srgb(0.55, 0.39, 1.)),
                    (20., height - 190., "·", Color::srgb(0.24, 0.79, 0.94)),
                    (
                        content_width - 26.,
                        height - 230.,
                        "✦",
                        Color::srgb(1., 0.69, 0.25),
                    ),
                ] {
                    root.spawn((
                        Text::new(symbol),
                        TextFont {
                            font: art.font.clone(),
                            font_size: 18.,
                            ..default()
                        },
                        TextColor(color),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(x),
                            top: Val::Px(y),
                            ..default()
                        },
                    ));
                }
            }
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(content_width),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|top| {
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "方块消消乐", 31., Color::srgb(1., 0.77, 0.25));
                    if !compact {
                        label(
                            p,
                            art,
                            "CLASSIC BLOCKS  ·  拼满一行消除",
                            14.,
                            Color::srgb(0.60, 0.61, 0.61),
                        );
                    }
                });
                button(
                    top,
                    art,
                    "‹ 返回游戏库",
                    Action::Library,
                    128.,
                    42.,
                    Color::srgb(0.08, 0.19, 0.43),
                    Color::WHITE,
                );
            });
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(content_width),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                column_gap: Val::Px(if compact { 12. } else { 32. }),
                ..row()
            })
            .with_children(|area| {
                area.spawn((
                    Node {
                        width: Val::Px(board_width),
                        height: Val::Px(board_height),
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(if compact { 3. } else { 5. })),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.012, 0.025, 0.075)),
                    BorderColor(Color::srgb(0.18, 0.58, 1.)),
                    BorderRadius::all(Val::Px(9.)),
                ))
                .with_children(|board| {
                    for y in 0..TETRIS_HEIGHT {
                        board
                            .spawn(Node {
                                width: Val::Px(board_width),
                                height: Val::Px(cell),
                                column_gap: Val::Px(if compact { 1.5 } else { 0. }),
                                ..row()
                            })
                            .with_children(|line| {
                                for x in 0..TETRIS_WIDTH {
                                    let value = g.cell(x, y);
                                    let ghost = value == 0
                                        && !g.failed
                                        && ghost_cells.contains(&(x as i8, y as i8));
                                    line.spawn((
                                        Node {
                                            width: Val::Px(cell - if compact { 1.5 } else { 0. }),
                                            height: Val::Px(cell - if compact { 1.5 } else { 0. }),
                                            border: UiRect::all(Val::Px(if value == 0 {
                                                1.
                                            } else {
                                                2.
                                            })),
                                            ..default()
                                        },
                                        BackgroundColor(if ghost {
                                            Color::srgba(0.31, 0.53, 0.91, 0.18)
                                        } else {
                                            tetris_color(value)
                                        }),
                                        BorderColor(if value == 0 {
                                            if ghost {
                                                Color::srgb(0.40, 0.70, 1.)
                                            } else {
                                                Color::srgb(0.10, 0.19, 0.36)
                                            }
                                        } else {
                                            Color::srgb(0.74, 0.89, 1.)
                                        }),
                                        BorderRadius::all(Val::Px(if compact { 5. } else { 2. })),
                                    ));
                                    if value > 0 {
                                        line.spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Px(x as f32 * cell + 3.),
                                                top: Val::Px(3.),
                                                width: Val::Px((cell - 8.).max(4.)),
                                                height: Val::Px((cell * 0.12).max(3.)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgba(1., 1., 1., 0.35)),
                                            BorderRadius::all(Val::Px(5.)),
                                        ));
                                    }
                                }
                            });
                    }
                });
                area.spawn(Node {
                    width: Val::Px(side_width),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(if compact { 8. } else { 16. }),
                    ..default()
                })
                .with_children(|side| {
                    tetris_stat(side, art, "得分", g.score.to_string());
                    tetris_stat(side, art, "最高", g.best_score.to_string());
                    tetris_stat(side, art, "消除", g.lines.to_string());
                    tetris_stat(side, art, "等级", g.level.to_string());
                    label(side, art, "NEXT  下一个", 16., Color::srgb(0.48, 0.86, 1.));
                    side.spawn((
                        Node {
                            width: Val::Px(preview_cell * 4.),
                            height: Val::Px(preview_cell * 4.),
                            flex_direction: FlexDirection::Column,
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.025, 0.045, 0.105)),
                        BorderColor(Color::srgb(0.20, 0.46, 0.84)),
                        BorderRadius::all(Val::Px(10.)),
                    ))
                    .with_children(|preview| {
                        let next_cells = TetrisGame::cells(g.next, 0);
                        for y in 0..4_i8 {
                            preview
                                .spawn(Node {
                                    width: Val::Px(preview_cell * 4.),
                                    height: Val::Px(preview_cell),
                                    ..row()
                                })
                                .with_children(|line| {
                                    for x in 0..4_i8 {
                                        let filled = next_cells.contains(&(x, y));
                                        line.spawn((
                                            Node {
                                                width: Val::Px(preview_cell),
                                                height: Val::Px(preview_cell),
                                                border: UiRect::all(Val::Px(1.)),
                                                ..default()
                                            },
                                            BackgroundColor(if filled {
                                                tetris_color(g.next + 1)
                                            } else {
                                                Color::srgb(0.025, 0.045, 0.105)
                                            }),
                                            BorderColor(Color::srgb(0.18, 0.20, 0.20)),
                                        ));
                                    }
                                });
                        }
                    });
                    if compact {
                        tetris_button(
                            side,
                            art,
                            "重来",
                            Action::TetrisReset,
                            side_width,
                            36.,
                            Color::srgb(0.08, 0.19, 0.43),
                            Color::srgb(0.26, 0.49, 0.82),
                        );
                    }
                });
            });
            if !compact {
                label(root, art, g.feedback, 15., Color::srgb(0.91, 0.71, 0.39));
            }
            if compact {
                root.spawn((Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(content_width),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..row()
                },))
                    .with_children(|controls| {
                        let control_size = ((content_width - 64.) / 5.).min(76.);
                        tetris_control_button(
                            controls,
                            art,
                            "←",
                            "左移",
                            Action::TetrisLeft,
                            control_size,
                            Color::srgb(0.02, 0.66, 0.83),
                            Color::srgb(0.28, 0.92, 1.),
                        );
                        tetris_control_button(
                            controls,
                            art,
                            "→",
                            "右移",
                            Action::TetrisRight,
                            control_size,
                            Color::srgb(0.10, 0.40, 0.89),
                            Color::srgb(0.40, 0.68, 1.),
                        );
                        tetris_control_button(
                            controls,
                            art,
                            "转",
                            "旋转",
                            Action::TetrisRotate,
                            control_size,
                            Color::srgb(0.62, 0.18, 0.87),
                            Color::srgb(0.83, 0.50, 1.),
                        );
                        tetris_control_button(
                            controls,
                            art,
                            "↓",
                            "快速下落",
                            Action::TetrisDown,
                            control_size,
                            Color::srgb(0.03, 0.68, 0.43),
                            Color::srgb(0.31, 0.96, 0.70),
                        );
                        tetris_control_button(
                            controls,
                            art,
                            "↓↓",
                            "直接落下",
                            Action::TetrisDrop,
                            control_size,
                            Color::srgb(0.98, 0.57, 0.06),
                            Color::srgb(1., 0.82, 0.27),
                        );
                    });
            } else {
                root.spawn(Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(content_width),
                    justify_content: JustifyContent::Center,
                    ..row()
                })
                .with_children(|controls| {
                    button(
                        controls,
                        art,
                        "重新开始",
                        Action::TetrisReset,
                        110.,
                        42.,
                        SLATE,
                        Color::WHITE,
                    )
                });
            }
            if !compact {
                label(
                    root,
                    art,
                    "键盘：← → 移动 · ↑ 旋转 · 长按 ↓ 连续下落 · 空格快速下落",
                    13.,
                    Color::srgb(0.53, 0.54, 0.53),
                );
            }
            if g.failed {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(290.),
                        height: Val::Px(150.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.04, 0.05, 0.12, 0.94)),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|panel| {
                    label(panel, art, "方块堆满啦", 24., Color::WHITE);
                    label(
                        panel,
                        art,
                        format!("得分 {} · 消除 {} 行", g.score, g.lines),
                        16.,
                        SOFT,
                    );
                    button(
                        panel,
                        art,
                        "重新开始",
                        Action::TetrisReset,
                        130.,
                        46.,
                        Color::srgb(0.31, 0.63, 0.84),
                        Color::WHITE,
                    );
                });
            }
        });
}

fn piano_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.piano;
    let key_colors = [
        Color::srgb(0.94, 0.35, 0.35),
        Color::srgb(0.96, 0.62, 0.25),
        Color::srgb(0.95, 0.82, 0.27),
        Color::srgb(0.35, 0.73, 0.45),
        Color::srgb(0.25, 0.62, 0.88),
        Color::srgb(0.48, 0.38, 0.83),
        Color::srgb(0.74, 0.37, 0.78),
    ];
    let note = g.target();
    let note_top = 8. + (g.fall / 3.2).clamp(0., 1.) * 66.;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(if compact { 14. } else { 24. })),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.08, 0.18)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(820.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|p| {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "彩虹钢琴派对", 28., Color::WHITE);
                    label(p, art, "看音符落下，弹出小动物乐队的歌", 15., SOFT);
                });
                label(
                    p,
                    art,
                    format!(
                        "第 {} / {} 个音符  ·  {} 连击",
                        g.step % PIANO_NOTES.len() + 1,
                        PIANO_NOTES.len(),
                        g.combo
                    ),
                    17.,
                    Color::srgb(1.0, 0.80, 0.34),
                );
                button(p, art, "怎么玩", Action::Help, 90., 44., PAPER, TEAL);
                button(
                    p,
                    art,
                    "游戏库",
                    Action::Library,
                    82.,
                    38.,
                    SLATE,
                    Color::WHITE,
                );
            });
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(820.),
                    flex_grow: 1.,
                    min_height: Val::Px(0.),
                    padding: UiRect::all(Val::Px(if compact { 14. } else { 20. })),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.13, 0.16, 0.33)),
                BorderRadius::all(Val::Px(26.)),
            ))
            .with_children(|p| {
                p.spawn(Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                })
                .with_children(|p| {
                    label(p, art, "演奏《彩虹小步舞》", 20., Color::WHITE);
                    label(
                        p,
                        art,
                        format!("得分 {}  ·  已弹 {} 个音符", g.score, g.step),
                        16.,
                        SOFT,
                    );
                });
                p.spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(if compact { 190. } else { 235. }),
                        min_height: Val::Px(if compact { 190. } else { 235. }),
                        position_type: PositionType::Relative,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.08, 0.10, 0.24)),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|stage| {
                    for lane in 0..7 {
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(lane as f32 * 100. / 7.),
                                width: Val::Percent(100. / 7.),
                                height: Val::Percent(100.),
                                border: UiRect::right(Val::Px(1.)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.14, 0.17, 0.34, 0.45)),
                            BorderColor(Color::srgba(1., 1., 1., 0.12)),
                        ));
                    }
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            bottom: Val::Px(28.),
                            height: Val::Px(4.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1., 0.82, 0.28)),
                    ));
                    stage
                        .spawn((
                            PianoNote,
                            Button,
                            Action::PianoKey(note),
                            ButtonTone(key_colors[note]),
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(note as f32 * 100. / 7. + 1.2),
                                top: Val::Percent(note_top),
                                width: Val::Percent(100. / 7. - 2.4),
                                height: Val::Px(if compact { 48. } else { 58. }),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(key_colors[note]),
                            BorderRadius::all(Val::Px(14.)),
                        ))
                        .with_children(|p| label(p, art, PIANO_NAMES[note], 22., Color::WHITE));
                });
                label(p, art, &g.feedback, 18., Color::srgb(1., 0.87, 0.54));
                p.spawn(Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(0.),
                    ..row()
                })
                .with_children(|keys| {
                    for key in 0..7 {
                        let pressed = g.last_key == Some(key) && g.flash > 0.;
                        let background = if pressed || key == note {
                            key_colors[key]
                        } else {
                            PAPER
                        };
                        button(
                            keys,
                            art,
                            format!(
                                "{} {}\n{}",
                                key + 1,
                                ["哆", "来", "咪", "发", "嗦", "拉", "西"][key],
                                PIANO_NAMES[key]
                            ),
                            Action::PianoKey(key),
                            if compact { 86. } else { 98. },
                            if compact { 76. } else { 88. },
                            background,
                            if pressed || key == note {
                                Color::WHITE
                            } else {
                                INK
                            },
                        );
                    }
                });
                label(p, art, "也能使用键盘：A  S  D  F  G  H  J", 14., SOFT);
            });
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(820.),
                justify_content: JustifyContent::SpaceBetween,
                ..row()
            })
            .with_children(|p| {
                label(p, art, "点亮起的琴键，或直接点落下的音符", 15., SOFT);
                button(
                    p,
                    art,
                    "重新开始",
                    Action::PianoReset,
                    104.,
                    38.,
                    Color::srgb(0.32, 0.38, 0.66),
                    Color::WHITE,
                );
            });
        });
}

fn platform_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.platform;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(if compact { 14. } else { 24. })),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.28, 0.67, 0.88)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(900.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|p| {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "蘑菇跳跳岛", 28., Color::WHITE);
                    label(
                        p,
                        art,
                        "原创横版跑跳闯关 · 踩怪、金币、终点旗帜",
                        15.,
                        Color::srgb(0.91, 0.97, 1.),
                    );
                });
                p.spawn(row()).with_children(|p| {
                    label(
                        p,
                        art,
                        format!("{}m / 300m", g.distance.min(300.).round()),
                        17.,
                        Color::srgb(1., 0.91, 0.38),
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        82.,
                        38.,
                        SLATE,
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(900.),
                    flex_grow: 1.,
                    min_height: Val::Px(if compact { 250. } else { 320. }),
                    padding: UiRect::all(Val::Px(if compact { 12. } else { 18. })),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.30, 0.52)),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|p| {
                p.spawn(Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                })
                .with_children(|p| {
                    label(
                        p,
                        art,
                        format!("金币 {}  ·  得分 {}", g.coins, g.score),
                        18.,
                        Color::srgb(1., 0.87, 0.30),
                    );
                    label(
                        p,
                        art,
                        if g.invulnerable > 0. {
                            format!("爱心 {}/3 · 护盾中", g.lives)
                        } else {
                            format!("爱心 {}/3", g.lives)
                        },
                        17.,
                        Color::srgb(1., 0.78, 0.80),
                    );
                });
                p.spawn((
                    Node {
                        width: Val::Percent(100.),
                        flex_grow: 1.,
                        min_height: Val::Px(if compact { 200. } else { 265. }),
                        position_type: PositionType::Relative,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.39, 0.76, 0.94)),
                    BorderRadius::all(Val::Px(16.)),
                ))
                .with_children(|stage| {
                    for (left, top, width) in [(10., 13., 74.), (42., 25., 104.), (76., 9., 64.)] {
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(left),
                                top: Val::Percent(top),
                                width: Val::Px(width),
                                height: Val::Px(16.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(1., 1., 1., 0.72)),
                            BorderRadius::all(Val::Px(12.)),
                        ));
                    }
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            bottom: Val::Px(0.),
                            height: Val::Percent(21.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.30, 0.64, 0.30)),
                    ));
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            bottom: Val::Percent(18.),
                            height: Val::Px(10.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.62, 0.40, 0.20)),
                    ));
                    // The flag gives a clear visual destination throughout the run.
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Percent(4.),
                            bottom: Val::Percent(21.),
                            width: Val::Px(5.),
                            height: Val::Percent(48.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.93, 0.94, 0.88)),
                    ));
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Percent(4.5),
                            bottom: Val::Percent(55.),
                            width: Val::Px(42.),
                            height: Val::Px(25.),
                            ..default()
                        },
                        BackgroundColor(if g.finished {
                            Color::srgb(1., 0.76, 0.20)
                        } else {
                            Color::srgb(0.90, 0.27, 0.25)
                        }),
                        BorderRadius::all(Val::Px(4.)),
                    ));
                    for thing in &g.things {
                        let bottom = 22. + thing.y * 60.;
                        let (width, height, color) = match thing.kind {
                            PlatformThingKind::Coin => (
                                if compact { 24. } else { 28. },
                                if compact { 24. } else { 28. },
                                Color::srgb(1., 0.79, 0.17),
                            ),
                            PlatformThingKind::Walker => (
                                if compact { 34. } else { 40. },
                                if compact { 30. } else { 36. },
                                Color::srgb(0.53, 0.30, 0.18),
                            ),
                            PlatformThingKind::Block => (
                                if compact { 34. } else { 40. },
                                if compact { 34. } else { 40. },
                                Color::srgb(0.82, 0.42, 0.16),
                            ),
                        };
                        stage
                            .spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent((thing.x - g.camera_x()) * 100.),
                                    bottom: Val::Percent(bottom),
                                    width: Val::Px(width),
                                    height: Val::Px(height),
                                    ..default()
                                },
                                BackgroundColor(color),
                                BorderRadius::all(Val::Px(
                                    if matches!(
                                        thing.kind,
                                        PlatformThingKind::Coin | PlatformThingKind::Walker
                                    ) {
                                        width / 2.
                                    } else {
                                        4.
                                    },
                                )),
                            ))
                            .with_children(|piece| {
                                if matches!(thing.kind, PlatformThingKind::Walker) {
                                    for left in [9., 22.] {
                                        piece.spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Px(left),
                                                top: Val::Px(9.),
                                                width: Val::Px(5.),
                                                height: Val::Px(6.),
                                                ..default()
                                            },
                                            BackgroundColor(Color::WHITE),
                                            BorderRadius::all(Val::Px(4.)),
                                        ));
                                    }
                                }
                                if matches!(thing.kind, PlatformThingKind::Block) {
                                    piece.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(7.),
                                            top: Val::Px(7.),
                                            width: Val::Px(width - 14.),
                                            height: Val::Px(4.),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(1., 0.72, 0.30)),
                                    ));
                                }
                            });
                    }
                    let walking = g.walk_timer > 0.;
                    let stride = if walking { g.walk_phase.sin() * 5. } else { 0. };
                    let bob = if walking {
                        g.walk_phase.sin().abs() * 1.8
                    } else {
                        0.
                    };
                    let hero_bottom = 22. + g.player_y * 60. + bob;
                    stage
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(g.player_x * 100.),
                            bottom: Val::Percent(hero_bottom),
                            width: Val::Px(if compact { 38. } else { 46. }),
                            height: Val::Px(if compact { 50. } else { 60. }),
                            ..default()
                        },))
                        .with_children(|hero| {
                            hero.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(2.),
                                    top: Val::Px(0.),
                                    width: Val::Percent(92.),
                                    height: Val::Px(17.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.86, 0.19, 0.16)),
                                BorderRadius::all(Val::Px(6.)),
                            ));
                            hero.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(8.),
                                    top: Val::Px(13.),
                                    width: Val::Percent(70.),
                                    height: Val::Px(21.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1., 0.76, 0.52)),
                                BorderRadius::all(Val::Px(7.)),
                            ));
                            hero.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(5.),
                                    bottom: Val::Px(7.),
                                    width: Val::Percent(78.),
                                    height: Val::Px(23.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.19, 0.38, 0.82)),
                                BorderRadius::all(Val::Px(5.)),
                            ));
                            // Separate animated legs make walking immediately
                            // visible, even while the camera keeps the hero in a
                            // comfortable on-screen position.
                            hero.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(10. + stride),
                                    bottom: Val::Px(0.),
                                    width: Val::Px(9.),
                                    height: Val::Px(14.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.10, 0.22, 0.48)),
                                BorderRadius::all(Val::Px(3.)),
                            ));
                            hero.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(27. - stride),
                                    bottom: Val::Px(0.),
                                    width: Val::Px(9.),
                                    height: Val::Px(14.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.10, 0.22, 0.48)),
                                BorderRadius::all(Val::Px(3.)),
                            ));
                            hero.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(if g.walk_direction >= 0 { 29. } else { 7. }),
                                    top: Val::Px(18.),
                                    width: Val::Px(5.),
                                    height: Val::Px(5.),
                                    ..default()
                                },
                                BackgroundColor(INK),
                                BorderRadius::all(Val::Px(4.)),
                            ));
                        });
                });
                label(p, art, &g.feedback, 17., Color::srgb(1., 0.89, 0.50));
            });
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(900.),
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(18.),
                ..row()
            })
            .with_children(|controls| {
                // Keep the movement cluster shaped like a gamepad: jump is
                // always above left/right rather than moving between games.
                controls
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(5.),
                        ..default()
                    })
                    .with_children(|pad| {
                        button(
                            pad,
                            art,
                            "跳",
                            Action::PlatformJump,
                            if compact { 104. } else { 92. },
                            if compact { 62. } else { 54. },
                            Color::srgb(1., 0.68, 0.20),
                            INK,
                        );
                        pad.spawn(row()).with_children(|line| {
                            button(
                                line,
                                art,
                                "←",
                                Action::PlatformLeft,
                                if compact { 104. } else { 92. },
                                if compact { 62. } else { 54. },
                                Color::srgb(0.19, 0.38, 0.66),
                                Color::WHITE,
                            );
                            button(
                                line,
                                art,
                                "→",
                                Action::PlatformRight,
                                if compact { 104. } else { 92. },
                                if compact { 62. } else { 54. },
                                Color::srgb(0.19, 0.38, 0.66),
                                Color::WHITE,
                            );
                        });
                    });
                button(
                    controls,
                    art,
                    "重来",
                    Action::PlatformReset,
                    82.,
                    if compact { 62. } else { 54. },
                    SLATE,
                    Color::WHITE,
                );
            });
            label(
                root,
                art,
                if compact {
                    "按住 ← / → 连续移动 · 跳跃可提前一点按"
                } else {
                    "键盘：A / D 或 ← / → 移动 · W、↑ 或空格跳跃"
                },
                14.,
                Color::srgb(0.92, 0.98, 1.),
            );
        });
}

fn assault_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.assault;
    let (stage_name, stage_hint, stage_tint, ground_color) = match g.stage {
        0 => (
            "第一关 · 晨雾绿谷",
            "熟悉跳台和连发节奏",
            Color::srgba(0.08, 0.28, 0.18, 0.10),
            Color::srgb(0.13, 0.42, 0.27),
        ),
        1 => (
            "第二关 · 悬崖石桥",
            "精英炮台需要两发能量弹",
            Color::srgba(0.55, 0.25, 0.08, 0.18),
            Color::srgb(0.36, 0.27, 0.20),
        ),
        _ => (
            "第三关 · 夜光营地",
            "敌人更快，稳住连击",
            Color::srgba(0.20, 0.08, 0.48, 0.28),
            Color::srgb(0.15, 0.25, 0.38),
        ),
    };
    // The three tactical heights are presented as one continuous landscape:
    // floor plus two short ledges, rather than the old board-game lanes.
    let lane_top = |lane: i8| match lane {
        0 => 25.,
        1 => 48.,
        _ => 67.,
    };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(if compact { 14. } else { 24. })),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.08, 0.16)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(860.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|p| {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "像素突击队", 28., Color::WHITE);
                    label(p, art, stage_name, 15., SOFT);
                });
                label(
                    p,
                    art,
                    format!("路程 {}m  ·  连击 {}", g.distance.round(), g.combo),
                    17.,
                    Color::srgb(1., 0.80, 0.30),
                );
                button(
                    p,
                    art,
                    "游戏库",
                    Action::Library,
                    82.,
                    38.,
                    SLATE,
                    Color::WHITE,
                );
            });
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    max_width: Val::Px(860.),
                    flex_grow: 1.,
                    min_height: Val::Px(0.),
                    padding: UiRect::all(Val::Px(if compact { 14. } else { 20. })),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.11, 0.16, 0.30)),
                BorderRadius::all(Val::Px(26.)),
            ))
            .with_children(|p| {
                p.spawn(Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    ..row()
                })
                .with_children(|p| {
                    label(p, art, stage_name, 20., Color::WHITE);
                    label(
                        p,
                        art,
                        format!("得分 {}  ·  护盾 {}/3", g.score, g.lives),
                        17.,
                        SOFT,
                    );
                });
                p.spawn((
                    Node {
                        width: Val::Percent(100.),
                        flex_grow: 1.,
                        min_height: Val::Px(if compact { 185. } else { 240. }),
                        position_type: PositionType::Relative,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.08, 0.12, 0.24)),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|stage| {
                    let camera_x = g.camera_x;
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(-((camera_x * 9.).rem_euclid(30.))),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                            width: Val::Percent(132.),
                            ..default()
                        },
                        ImageNode::new(art.assault_forest.clone()),
                    ));
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        BackgroundColor(stage_tint),
                    ));
                    // Dark foreground silhouette, short ledges and the main ground make
                    // the height choices read as a scrolling platform battlefield.
                    for (left, top, width) in [
                        (0., 76., 100.),
                        (35., 57., 24.),
                        (71., 40., 19.),
                        (4., 35., 20.),
                    ] {
                        let screen_left = (left - camera_x * 26.).rem_euclid(130.) - 15.;
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(screen_left),
                                top: Val::Percent(top),
                                width: Val::Percent(width),
                                height: Val::Percent(8.),
                                ..default()
                            },
                            BackgroundColor(ground_color),
                            BorderRadius::all(Val::Px(4.)),
                            BorderColor(Color::srgb(0.46, 0.86, 0.38)),
                        ));
                    }
                    for tree_x in [28., 52., 76., 95.] {
                        let screen_x = (tree_x - camera_x * 14.).rem_euclid(112.);
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(screen_x),
                                bottom: Val::Percent(16.),
                                width: Val::Px(28.),
                                height: Val::Px(55.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.04, 0.17, 0.17, 0.45)),
                            BorderRadius::all(Val::Px(18.)),
                        ));
                    }
                    // The hero artwork is a full pixel silhouette, while its
                    // position is driven every frame: a small run bob, jump arc
                    // and recoil kick give separate readable movement states.
                    let run_bob = (g.run_phase * 2.).sin() * 1.2;
                    stage.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(
                                ((g.player_x - camera_x) * 100.
                                    + if g.muzzle > 0. { -0.7 } else { 0. }
                                    + if g.shake > 0. { 1.2 } else { 0. })
                                .clamp(0., 92.),
                            ),
                            top: Val::Percent((lane_top(g.lane) - g.jump * 34. + run_bob).max(1.)),
                            width: Val::Px(if compact { 80. } else { 98. }),
                            height: Val::Px(if compact { 80. } else { 98. }),
                            ..default()
                        },
                        ImageNode::new(art.assault_hero.clone()),
                    ));
                    for bullet in &g.bullets {
                        // Long translucent trail followed by a bright core reads as a
                        // fast projectile even during a low-refresh UI redraw.
                        let bullet_x = bullet.x - camera_x;
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent((bullet_x * 100. - 5.).max(0.)),
                                top: Val::Percent(lane_top(bullet.lane) + 17.),
                                width: Val::Percent(6.),
                                height: Val::Px(3.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.25, 0.95, 1., 0.38)),
                            BorderRadius::all(Val::Px(4.)),
                        ));
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent((bullet_x * 100.).max(0.)),
                                top: Val::Percent(lane_top(bullet.lane) + 15.),
                                width: Val::Px(18.),
                                height: Val::Px(7.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.66, 0.98, 1.)),
                            BorderRadius::all(Val::Px(5.)),
                        ));
                    }
                    for spark in &g.sparks {
                        let size = if spark.large {
                            44. * spark.life / 0.46
                        } else {
                            20. * spark.life / 0.20
                        };
                        stage.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent((spark.x - camera_x) * 100. - 2.),
                                top: Val::Percent(lane_top(spark.lane) + 8.),
                                width: Val::Px(size.max(6.)),
                                height: Val::Px(size.max(6.)),
                                ..default()
                            },
                            BackgroundColor(if spark.large {
                                Color::srgba(1., 0.45, 0.12, 0.82)
                            } else {
                                Color::srgba(1., 0.94, 0.36, 0.95)
                            }),
                            BorderRadius::all(Val::Percent(50.)),
                        ));
                    }
                    for enemy in &g.enemies {
                        stage
                            .spawn((Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent((enemy.x - camera_x) * 100.),
                                top: Val::Percent(lane_top(enemy.lane)),
                                width: Val::Px(if enemy.heavy { 64. } else { 50. }),
                                height: Val::Px(if enemy.heavy { 64. } else { 50. }),
                                ..default()
                            },))
                            .with_children(|p| assault_enemy_sprite(p, enemy.heavy));
                    }
                    if g.finished {
                        stage
                            .spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(20.),
                                    right: Val::Percent(20.),
                                    top: Val::Percent(28.),
                                    height: Val::Percent(34.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.02, 0.06, 0.13, 0.88)),
                                BorderRadius::all(Val::Px(18.)),
                            ))
                            .with_children(|panel| {
                                label(
                                    panel,
                                    art,
                                    "🎉 三关通关！",
                                    32.,
                                    Color::srgb(1., 0.83, 0.30),
                                )
                            });
                    }
                });
                label(
                    p,
                    art,
                    format!("{}  ·  {}", stage_hint, g.feedback),
                    17.,
                    Color::srgb(1., 0.84, 0.46),
                );
                p.spawn(Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(8.),
                    ..row()
                })
                .with_children(|controls| {
                    let size = if compact { 76. } else { 66. };
                    controls
                        .spawn(Node {
                            column_gap: Val::Px(6.),
                            align_items: AlignItems::Center,
                            ..row()
                        })
                        .with_children(|pad| {
                            button(
                                pad,
                                art,
                                "←",
                                Action::AssaultLeft,
                                size,
                                size,
                                Color::srgb(0.25, 0.35, 0.57),
                                Color::WHITE,
                            );
                            button(
                                pad,
                                art,
                                "跳",
                                Action::AssaultJump,
                                size,
                                size,
                                Color::srgb(0.95, 0.66, 0.26),
                                INK,
                            );
                            button(
                                pad,
                                art,
                                "→",
                                Action::AssaultRight,
                                size,
                                size,
                                Color::srgb(0.25, 0.35, 0.57),
                                Color::WHITE,
                            );
                        });
                    button(
                        controls,
                        art,
                        "开火\n射击",
                        Action::AssaultFire,
                        if compact { 112. } else { 100. },
                        if compact { 92. } else { 82. },
                        Color::srgb(0.30, 0.77, 0.62),
                        NAVY,
                    );
                });
                label(
                    p,
                    art,
                    if compact {
                        "手机：点 ← / → 移动 · 跳跃躲避 · 按住开火"
                    } else {
                        "键盘：按住 A / D 或 ← / → 移动 · W 跳跃 · 空格射击"
                    },
                    14.,
                    SOFT,
                );
            });
            root.spawn(Node {
                width: Val::Percent(100.),
                max_width: Val::Px(860.),
                justify_content: JustifyContent::SpaceBetween,
                ..row()
            })
            .with_children(|p| {
                label(
                    p,
                    art,
                    format!("关卡进度：{} / 3  ·  {}", g.stage + 1, stage_hint),
                    15.,
                    SOFT,
                );
                button(
                    p,
                    art,
                    "重新出发",
                    Action::AssaultReset,
                    104.,
                    38.,
                    Color::srgb(0.34, 0.42, 0.72),
                    Color::WHITE,
                );
            });
        });
}

fn assault_enemy_sprite(parent: &mut ChildSpawnerCommands, heavy: bool) {
    let part = |parent: &mut ChildSpawnerCommands, left, top, width, height, color| {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                width: Val::Px(width),
                height: Val::Px(height),
                ..default()
            },
            BackgroundColor(color),
            BorderRadius::all(Val::Px(3.)),
        ));
    };
    let shell = if heavy {
        Color::srgb(0.66, 0.27, 0.61)
    } else {
        Color::srgb(0.85, 0.29, 0.28)
    };
    let dark = Color::srgb(0.16, 0.16, 0.25);
    let metal = Color::srgb(0.48, 0.55, 0.62);
    // A squat robot head with one glowing eye.
    part(parent, 11., 8., if heavy { 34. } else { 28. }, 16., dark);
    part(
        parent,
        16.,
        13.,
        if heavy { 20. } else { 15. },
        5.,
        Color::srgb(1., 0.80, 0.25),
    );
    part(
        parent,
        8.,
        23.,
        if heavy { 42. } else { 34. },
        if heavy { 25. } else { 20. },
        shell,
    );
    part(parent, 15., 28., if heavy { 27. } else { 20. }, 6., metal);
    // Legs make the small enemy look like it is running toward the player.
    part(parent, 11., if heavy { 47. } else { 41. }, 9., 10., dark);
    part(
        parent,
        if heavy { 37. } else { 31. },
        if heavy { 47. } else { 41. },
        9.,
        10.,
        dark,
    );
    if heavy {
        // Purple turret: a large barrel makes its two-hit behavior clear.
        part(parent, 39., 15., 23., 10., metal);
        part(parent, 55., 17., 10., 6., dark);
        part(parent, 19., 2., 8., 8., shell);
    } else {
        part(parent, 3., 29., 8., 7., dark);
        part(parent, 39., 29., 8., 7., dark);
        part(parent, 22., 2., 4., 8., metal);
    }
}

// Touch controls keep the same visual language as the flight HUD.
fn sky_control_button(parent: &mut ChildSpawnerCommands, art: &Art, direction: i8, compact: bool) {
    let cyan = Color::srgb(0.64, 0.94, 1.0);
    let size = if compact { 88. } else { 104. };
    parent
        .spawn((
            Button,
            ButtonTone(Color::srgb(0.08, 0.30, 0.62)),
            Action::SkyMove(direction),
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                min_height: Val::Px(size),
                border: UiRect::all(Val::Px(3.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.30, 0.62)),
            BorderColor(cyan),
            BorderRadius::all(Val::Percent(50.)),
        ))
        .with_children(|p| label(p, art, if direction < 0 { "←" } else { "→" }, 42., cyan));
}

fn sky_secondary_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    action: Action,
    symbol: &str,
    caption: &str,
    compact: bool,
) {
    let size = if compact { 66. } else { 76. };
    let bg = Color::srgb(0.035, 0.20, 0.42);
    parent
        .spawn((
            Button,
            ButtonTone(bg),
            action,
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                min_height: Val::Px(size),
                border: UiRect::all(Val::Px(2.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(bg),
            BorderColor(Color::srgb(0.49, 0.84, 1.0)),
            BorderRadius::all(Val::Percent(50.)),
        ))
        .with_children(|p| {
            label(
                p,
                art,
                symbol,
                if compact { 25. } else { 30. },
                Color::WHITE,
            );
            label(
                p,
                art,
                caption,
                if compact { 11. } else { 13. },
                Color::srgb(0.72, 0.90, 1.0),
            );
        });
}

fn sky_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool, width: f32) {
    let g = &s.sky;
    let field_top = if compact { 88. } else { 106. };
    let field_bottom = if compact { 144. } else { 158. };
    let plane_size = (width * 0.24).clamp(90., 158.);
    let enemy_size = (width * 0.16).clamp(58., 104.);
    let cyan = Color::srgb(0.29, 0.88, 1.0);
    let coral = Color::srgb(1.0, 0.35, 0.36);
    let navy = Color::srgb(0.015, 0.055, 0.17);
    let soft = Color::srgb(0.69, 0.85, 0.98);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(navy),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.sky_backdrop.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(field_top),
                    bottom: Val::Px(field_bottom),
                    left: Val::Px(if compact { 10. } else { 32. }),
                    right: Val::Px(if compact { 10. } else { 32. }),
                    border: UiRect::horizontal(Val::Px(1.)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.01, 0.03, 0.10, 0.08)),
                BorderColor(Color::srgba(0.31, 0.73, 1.0, 0.35)),
            ))
            .with_children(|field| {
                // The background plate supplies the nebula; faint guides stay behind gameplay.
                for x in [25., 50., 75.] {
                    field.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(x),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                            width: Val::Px(1.),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.55, 0.82, 1.0, 0.08)),
                    ));
                }
                // Two star sizes drift at different speeds with the play field.
                for (x, y, size) in [
                    (7., 9., 3.),
                    (18., 28., 2.),
                    (29., 14., 4.),
                    (38., 43., 2.),
                    (54., 7., 3.),
                    (66., 26., 2.),
                    (78., 13., 4.),
                    (91., 36., 2.),
                    (10., 72., 2.),
                    (23., 58., 3.),
                    (47., 68., 2.),
                    (61., 51., 3.),
                    (74., 77., 2.),
                    (88., 62., 3.),
                ] {
                    field.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(x),
                            top: Val::Percent((y + g.star_scroll * (75. + size * 8.)) % 100.),
                            width: Val::Px(size),
                            height: Val::Px(size),
                            ..default()
                        },
                        BackgroundColor(if size >= 3. {
                            Color::srgba(0.50, 0.93, 1.0, 0.92)
                        } else {
                            Color::srgba(0.95, 0.96, 1.0, 0.72)
                        }),
                        BorderRadius::all(Val::Px(size)),
                    ));
                }
                for (x, y) in [(13., 18.), (83., 27.), (24., 55.), (72., 80.), (46., 38.)] {
                    field.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(x),
                            top: Val::Percent((y + g.star_scroll * 120.) % 100.),
                            width: Val::Px(2.),
                            height: Val::Px(28.),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.52, 0.88, 1.0, 0.24)),
                        BorderRadius::all(Val::Px(2.)),
                    ));
                }
                for rock in &g.rocks {
                    let size = rock.size * width * 1.5;
                    field.spawn((
                        ImageNode::new(art.sky_asteroid.clone()),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(rock.x * 100.),
                            margin: UiRect::left(Val::Px(-size * 0.5)),
                            top: Val::Percent(rock.y * 100.),
                            width: Val::Px(size),
                            height: Val::Px(size),
                            ..default()
                        },
                    ));
                }
                if let Some(boss) = &g.boss {
                    let boss_width = (width * 0.68).clamp(220., 390.);
                    field.spawn((
                        ImageNode::new(art.sky_boss.clone()),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(boss.x * 100.),
                            margin: UiRect::left(Val::Px(-boss_width * 0.5)),
                            top: Val::Percent(boss.y * 100.),
                            width: Val::Px(boss_width),
                            height: Val::Px(boss_width * 0.52),
                            ..default()
                        },
                    ));
                }
                for enemy in &g.enemies {
                    let size = if enemy.heavy {
                        enemy_size * 1.18
                    } else {
                        enemy_size
                    };
                    field.spawn((
                        ImageNode::new(art.sky_enemy.clone()),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(enemy.x * 100.),
                            margin: UiRect::left(Val::Px(-size * 0.5)),
                            top: Val::Percent(enemy.y * 100.),
                            width: Val::Px(size),
                            height: Val::Px(size),
                            ..default()
                        },
                    ));
                }
                for shot in &g.shots {
                    field
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(shot.x * 100.),
                                margin: UiRect::left(Val::Px(-7.)),
                                top: Val::Percent(shot.y * 100.),
                                width: Val::Px(14.),
                                height: Val::Px(if shot.friendly { 29. } else { 23. }),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(if shot.friendly {
                                Color::srgba(0.14, 0.88, 1.0, 0.28)
                            } else {
                                Color::srgba(1.0, 0.28, 0.34, 0.26)
                            }),
                            BorderRadius::all(Val::Px(8.)),
                        ))
                        .with_children(|glow| {
                            glow.spawn((
                                Node {
                                    width: Val::Px(if shot.friendly { 6. } else { 7. }),
                                    height: Val::Px(if shot.friendly { 23. } else { 17. }),
                                    ..default()
                                },
                                BackgroundColor(if shot.friendly { cyan } else { coral }),
                                BorderRadius::all(Val::Px(5.)),
                            ));
                        });
                }
                field.spawn((
                    ImageNode::new(art.sky_player.clone()),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(g.x * 100.),
                        margin: UiRect::left(Val::Px(-plane_size * 0.5)),
                        top: Val::Percent(69.),
                        width: Val::Px(plane_size),
                        height: Val::Px(plane_size * 1.35),
                        ..default()
                    },
                ));
            });

            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(if compact { 8. } else { 24. }),
                    right: Val::Px(if compact { 8. } else { 24. }),
                    top: Val::Px(if compact { 6. } else { 12. }),
                    height: Val::Px(if compact { 76. } else { 88. }),
                    padding: UiRect::horizontal(Val::Px(if compact { 9. } else { 18. })),
                    border: UiRect::all(Val::Px(2.)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                BackgroundColor(Color::srgba(0.02, 0.12, 0.31, 0.92)),
                BorderColor(cyan),
                BorderRadius::all(Val::Px(22.)),
            ))
            .with_children(|top| {
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(1.),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(Node {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(5.),
                        ..row()
                    })
                    .with_children(|brand| {
                        label(brand, art, "◉", if compact { 25. } else { 31. }, cyan);
                        label(
                            brand,
                            art,
                            "星际飞行队",
                            if compact { 20. } else { 27. },
                            Color::WHITE,
                        );
                    });
                    label(
                        p,
                        art,
                        format!("第 {} 星区", g.stage),
                        if compact { 12. } else { 15. },
                        soft,
                    );
                });
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(if compact { 8. } else { 14. }), Val::Px(6.)),
                    ..default()
                })
                .insert((
                    BackgroundColor(Color::srgba(0.03, 0.23, 0.49, 0.85)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|p| {
                    label(p, art, "得分", if compact { 10. } else { 13. }, soft);
                    label(
                        p,
                        art,
                        format!("{:05}", g.score),
                        if compact { 20. } else { 26. },
                        cyan,
                    );
                });
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(4.),
                    padding: UiRect::axes(Val::Px(if compact { 7. } else { 12. }), Val::Px(6.)),
                    ..default()
                })
                .insert((
                    BackgroundColor(Color::srgba(0.03, 0.23, 0.49, 0.85)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|p| {
                    label(
                        p,
                        art,
                        format!("连击 ×{}", g.combo),
                        if compact { 12. } else { 16. },
                        Color::srgb(1., 0.69, 0.35),
                    );
                    p.spawn(row()).with_children(|shields| {
                        for alive in 0..3 {
                            shields.spawn((
                                Node {
                                    width: Val::Px(if compact { 14. } else { 20. }),
                                    height: Val::Px(if compact { 12. } else { 16. }),
                                    margin: UiRect::left(Val::Px(3.)),
                                    ..default()
                                },
                                BackgroundColor(if alive < g.lives as usize {
                                    cyan
                                } else {
                                    Color::srgb(0.21, 0.29, 0.42)
                                }),
                                BorderRadius::all(Val::Px(5.)),
                            ));
                        }
                    });
                });
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(if compact { 12. } else { 34. }),
                top: Val::Px(field_top + 8.),
                ..row()
            })
            .with_children(|nav| {
                button(
                    nav,
                    art,
                    "← 返回游戏库",
                    Action::Library,
                    if compact { 118. } else { 142. },
                    if compact { 36. } else { 42. },
                    Color::srgb(0.02, 0.20, 0.43),
                    Color::srgb(0.78, 0.95, 1.),
                );
            });
            if let Some(boss) = &g.boss {
                let health = boss.hp as f32 / boss.max_hp as f32 * 100.;
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(field_top + 50.),
                        left: Val::Percent(20.),
                        width: Val::Percent(60.),
                        padding: UiRect::all(Val::Px(5.)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(2.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.16, 0.04, 0.18, 0.90)),
                    BorderRadius::all(Val::Px(10.)),
                ))
                .with_children(|panel| {
                    label(
                        panel,
                        art,
                        format!("BOSS 来袭 · 首领母舰  {}", boss.hp),
                        if compact { 12. } else { 15. },
                        coral,
                    );
                    panel
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Px(if compact { 8. } else { 11. }),
                                padding: UiRect::all(Val::Px(1.)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.18, 0.32)),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_children(|bar| {
                            bar.spawn((
                                Node {
                                    width: Val::Percent(health),
                                    height: Val::Percent(100.),
                                    ..default()
                                },
                                BackgroundColor(coral),
                                BorderRadius::all(Val::Px(4.)),
                            ));
                        });
                });
            }
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    bottom: Val::Px(0.),
                    height: Val::Px(if compact { 144. } else { 158. }),
                    border: UiRect::top(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.13, 0.34, 0.94)),
                BorderColor(Color::srgba(0.35, 0.79, 1.0, 0.82)),
            ));
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(14.),
                right: Val::Px(14.),
                bottom: Val::Px(7.),
                justify_content: JustifyContent::Center,
                ..row()
            })
            .with_children(|hint| {
                label(
                    hint,
                    art,
                    "左右移动 · 自动开火",
                    if compact { 10. } else { 13. },
                    soft,
                );
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                right: Val::Px(0.),
                bottom: Val::Px(if compact { 34. } else { 42. }),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(if compact { 6. } else { 12. }),
                ..row()
            })
            .with_children(|pad| {
                sky_secondary_button(pad, art, Action::SkyReset, "R", "重来", compact);
                sky_control_button(pad, art, -1, compact);
                sky_control_button(pad, art, 1, compact);
                sky_secondary_button(pad, art, Action::Help, "Ⅱ", "暂停", compact);
            });
            if g.failed {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(if compact { 300. } else { 370. }),
                        height: Val::Px(150.),
                        padding: UiRect::all(Val::Px(18.)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.13, 0.18, 0.91)),
                    BorderRadius::all(Val::Px(16.)),
                ))
                .with_children(|panel| {
                    label(panel, art, "战机受损", 24., Color::WHITE);
                    label(panel, art, format!("本次得分 {}", g.score), 15., SOFT);
                    button(
                        panel,
                        art,
                        "重新起飞",
                        Action::SkyReset,
                        136.,
                        44.,
                        Color::srgb(0.29, 0.60, 0.83),
                        Color::WHITE,
                    );
                });
            }
        });
}

fn bubble_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool, side: f32) {
    let g = &s.bubble;
    let field_top = if compact { 62. } else { 76. };
    let size = side * 0.114;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.80, 0.82, 0.94)),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.bubble_story.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            // A large uninterrupted board keeps the aiming gesture precise on a phone.
            root.spawn((
                BubbleBoard,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(field_top),
                    width: Val::Px(side),
                    height: Val::Px(side),
                    left: Val::Percent(50.),
                    margin: UiRect::left(Val::Px(-side / 2.)),
                    overflow: Overflow::clip(),
                    border: UiRect::ZERO,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.70, 0.81, 0.94, 0.74)),
                BorderColor(Color::srgb(0.98, 0.94, 0.85)),
                BorderRadius::all(Val::Px(20.)),
            ))
            .with_children(|board| {
                // Water-like dots make gaps in the cluster feel alive rather than blank.
                for (x, y) in [
                    (8., 17.),
                    (24., 36.),
                    (43., 11.),
                    (68., 30.),
                    (86., 19.),
                    (15., 65.),
                    (55., 54.),
                    (80., 73.),
                    (37., 82.),
                ] {
                    board.spawn((
                        SceneMotion::bob(Vec2::new(x * side / 100., y * side / 100.)),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(x),
                            top: Val::Percent(y),
                            width: Val::Px(4.),
                            height: Val::Px(4.),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1., 0.98, 0.93, 0.55)),
                        BorderRadius::all(Val::Px(4.)),
                    ));
                }
                if g.shot.is_none() && !g.failed {
                    for point in g.trajectory() {
                        board.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(point.x * 100.),
                                top: Val::Percent(point.y * 100.),
                                width: Val::Px(5.),
                                height: Val::Px(5.),
                                margin: UiRect::all(Val::Px(-2.5)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(1., 1., 1., 0.7)),
                            BorderRadius::all(Val::Px(3.)),
                        ));
                    }
                    board
                        .spawn(Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.),
                            top: Val::Percent(89.),
                            margin: UiRect {
                                left: Val::Px(-size / 2.),
                                top: Val::Px(-size / 2.),
                                ..default()
                            },
                            width: Val::Px(size),
                            height: Val::Px(size),
                            ..default()
                        })
                        .with_children(|p| bubble_sprite(p, art, g.next, size));
                }
                for pop in &g.pops {
                    let age = 0.65 - pop.life;
                    for i in 0..3 {
                        let angle = i as f32 * std::f32::consts::TAU / 5.;
                        let pos =
                            pop.position * side + Vec2::new(angle.cos(), angle.sin()) * age * 70.;
                        board.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(pos.x),
                                top: Val::Px(pos.y + age * age * 45.),
                                width: Val::Px(8. * pop.life / 0.65),
                                height: Val::Px(8. * pop.life / 0.65),
                                ..default()
                            },
                            BackgroundColor(bubble_color(pop.kind).with_alpha(pop.life / 0.65)),
                            BorderRadius::all(Val::Px(5.)),
                        ));
                    }
                }
                for cell in &g.cells {
                    let point = BubbleGame::position(cell.col, cell.row);
                    board
                        .spawn(Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(point.x * 100.),
                            top: Val::Percent(point.y * 100.),
                            margin: UiRect {
                                left: Val::Px(-size / 2.),
                                top: Val::Px(-size / 2.),
                                ..default()
                            },
                            width: Val::Px(size),
                            height: Val::Px(size),
                            ..default()
                        })
                        .with_children(|bubble| bubble_sprite(bubble, art, cell.kind, size));
                }
                if let Some(shot) = &g.shot {
                    board
                        .spawn(Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(shot.x * 100.),
                            top: Val::Percent(shot.y * 100.),
                            margin: UiRect {
                                left: Val::Px(-size / 2.),
                                top: Val::Px(-size / 2.),
                                ..default()
                            },
                            width: Val::Px(size),
                            height: Val::Px(size),
                            ..default()
                        })
                        .with_children(|bubble| bubble_sprite(bubble, art, shot.kind, size));
                }
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(if compact { 18. } else { 46. }),
                right: Val::Px(if compact { 18. } else { 46. }),
                top: Val::Px(if compact { 12. } else { 18. }),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|top| {
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|brand| {
                    label(
                        brand,
                        art,
                        "彩虹泡泡岛",
                        if compact { 20. } else { 30. },
                        Color::srgb(0.33, 0.25, 0.50),
                    );
                    label(
                        brand,
                        art,
                        format!("第 {} 关 · 泡泡分 {:04}", g.level, g.score),
                        if compact { 12. } else { 16. },
                        Color::srgb(0.25, 0.33, 0.50),
                    );
                });
                top.spawn(row()).with_children(|next| {
                    label(
                        next,
                        art,
                        "下一颗",
                        if compact { 12. } else { 16. },
                        Color::srgb(0.29, 0.27, 0.46),
                    );
                    next.spawn(Node {
                        width: Val::Px(if compact { 30. } else { 40. }),
                        height: Val::Px(if compact { 30. } else { 40. }),
                        ..default()
                    })
                    .with_children(|bubble| {
                        bubble_sprite(bubble, art, g.next, if compact { 30. } else { 40. })
                    });
                });
                top.spawn(row()).with_children(|nav| {
                    button(
                        nav,
                        art,
                        "怎么玩",
                        Action::Help,
                        if compact { 62. } else { 90. },
                        42.,
                        PAPER,
                        Color::srgb(0.41, 0.32, 0.59),
                    );
                    button(
                        nav,
                        art,
                        "游戏库",
                        Action::Library,
                        if compact { 66. } else { 90. },
                        42.,
                        PAPER,
                        Color::srgb(0.41, 0.32, 0.59),
                    );
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.),
                    margin: UiRect::left(Val::Px(if compact { -42. } else { -48. })),
                    bottom: Val::Px(if compact { 26. } else { 34. }),
                    width: Val::Px(if compact { 84. } else { 96. }),
                    height: Val::Px(if compact { 78. } else { 90. }),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.54, 0.39, 0.67)),
                BorderRadius::all(Val::Px(28.)),
            ))
            .with_children(|cannon| {
                cannon.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(40.),
                        top: Val::Px(-20.),
                        width: Val::Percent(20.),
                        height: Val::Px(40.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.91, 0.93, 0.90)),
                    BorderRadius::all(Val::Px(8.)),
                ));
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(18.),
                right: Val::Px(18.),
                bottom: Val::Px(4.),
                justify_content: JustifyContent::Center,
                ..row()
            })
            .with_children(|hint| {
                label(
                    hint,
                    art,
                    &g.feedback,
                    if compact { 11. } else { 13. },
                    Color::srgb(0.08, 0.26, 0.34),
                )
            });
            if !compact {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(22.),
                        top: Val::Px(100.),
                        width: Val::Px(if compact { 150. } else { 230. }),
                        padding: UiRect::all(Val::Px(12.)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.99, 0.96, 0.88, 0.87)),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|p| {
                    label(
                        p,
                        art,
                        "按住瞄准
松手发射",
                        24.,
                        Color::srgb(0.27, 0.24, 0.42),
                    );
                    label(
                        p,
                        art,
                        "看白色虚线
连到两个同色泡泡",
                        16.,
                        Color::srgb(0.27, 0.24, 0.42),
                    );
                    label(
                        p,
                        art,
                        if g.level == 1 {
                            "第 1 关：泡泡墙不会下压"
                        } else {
                            "连续 8 次未消除才会下压"
                        },
                        13.,
                        Color::srgb(0.27, 0.24, 0.42),
                    );
                    button(
                        p,
                        art,
                        "帮我瞄准",
                        Action::BubbleAssist,
                        160.,
                        52.,
                        PAPER,
                        NAVY,
                    );
                    button(p, art, "发射 →", Action::BubbleFire, 160., 60., MINT, NAVY);
                    button(
                        p,
                        art,
                        "重新开始",
                        Action::BubbleReset,
                        120.,
                        44.,
                        PAPER,
                        NAVY,
                    );
                });
            }
            if g.failed {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(if compact { 300. } else { 360. }),
                        height: Val::Px(212.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.18, 0.29, 0.93)),
                    BorderRadius::all(Val::Px(20.)),
                ))
                .with_children(|panel| {
                    label(panel, art, "泡泡压到底啦", 24., Color::WHITE);
                    label(panel, art, format!("本次泡泡分 {}", g.score), 15., SOFT);
                    button(
                        panel,
                        art,
                        "重新挑战",
                        Action::BubbleReset,
                        136.,
                        44.,
                        Color::srgb(0.94, 0.45, 0.25),
                        Color::WHITE,
                    );
                    button(
                        panel,
                        art,
                        "回到游戏库",
                        Action::Library,
                        136.,
                        42.,
                        PAPER,
                        NAVY,
                    );
                });
            }
        });
}

fn fruit_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let g = &s.fruit;
    let size = if compact { 84. } else { 150. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.94, 0.88, 0.75)),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.fruit_paper.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            for (x, y, w) in [
                (4., 18., 22.),
                (62., 12., 16.),
                (75., 72., 19.),
                (20., 82., 14.),
            ] {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        width: Val::Percent(w),
                        height: Val::Px(2.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.19, 0.18, 0.15, 0.20)),
                ));
            }
            if g.hit_flash > 0. {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1., 0.84, 0.40, (g.hit_flash / 0.16) * 0.14)),
                ));
            }
            for fruit in &g.fruits {
                root.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(fruit.x * 100.),
                    top: Val::Percent(fruit.y * 100.),
                    width: Val::Px(size),
                    height: Val::Px(size),
                    ..default()
                })
                .with_children(|node| fruit_sprite(node, art, fruit.kind, fruit.bomb, size));
            }
            for slice in &g.slices {
                let fade = (slice.life / 0.82).clamp(0., 1.);
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(slice.x * 100.),
                        top: Val::Percent(slice.y * 100.),
                        width: Val::Px(size * 1.55),
                        height: Val::Px(size * 1.55),
                        overflow: Overflow::visible(),
                        ..default()
                    },
                    // Effects should sit above remaining fruit, but underneath
                    // the score and navigation controls drawn later.
                    ZIndex(3),
                ))
                .with_children(|node| {
                    fruit_slice_sprite(node, slice, size, fade);
                    if slice.score > 0 {
                        node.spawn(Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(size * 0.30),
                            top: Val::Px(-size * (0.12 + (1. - fade) * 0.32)),
                            ..default()
                        })
                        .with_children(|score| {
                            label(
                                score,
                                art,
                                format!("+{}", slice.score),
                                size * 0.34,
                                Color::srgba(1., 0.92, 0.42, fade),
                            );
                        });
                    }
                });
            }
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(if compact { 22. } else { 42. }),
                right: Val::Px(if compact { 22. } else { 42. }),
                top: Val::Px(if compact { 16. } else { 22. }),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|top| {
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(
                        p,
                        art,
                        "水果忍者派对",
                        if compact { 21. } else { 27. },
                        Color::srgb(0.21, 0.18, 0.16),
                    );
                    label(
                        p,
                        art,
                        format!(
                            "第 {} 轮 · 目标 {}/{} · 得分 {:04} · {} 连击 · 最佳 {}",
                            g.stage,
                            g.stage_progress,
                            g.stage_target,
                            g.score,
                            g.combo,
                            g.best_combo
                        ),
                        if compact { 13. } else { 16. },
                        Color::srgb(0.70, 0.15, 0.11),
                    );
                });
                top.spawn(row()).with_children(|p| {
                    label(
                        p,
                        art,
                        "爱心",
                        if compact { 13. } else { 16. },
                        Color::srgb(0.21, 0.18, 0.16),
                    );
                    for i in 0..3 {
                        label(
                            p,
                            art,
                            if i < g.lives as usize { "♥" } else { "·" },
                            if compact { 22. } else { 27. },
                            if i < g.lives as usize {
                                Color::srgb(0.72, 0.15, 0.13)
                            } else {
                                Color::srgb(0.63, 0.59, 0.52)
                            },
                        );
                    }
                    button(
                        p,
                        art,
                        "?",
                        Action::Help,
                        38.,
                        36.,
                        Color::srgb(0.21, 0.19, 0.17),
                        Color::WHITE,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        if compact { 70. } else { 86. },
                        36.,
                        Color::srgb(0.21, 0.19, 0.17),
                        Color::WHITE,
                    );
                });
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.),
                top: Val::Px(if compact { 72. } else { 84. }),
                max_width: Val::Percent(70.),
                justify_self: JustifySelf::Center,
                ..default()
            })
            .with_children(|p| {
                label(
                    p,
                    art,
                    &g.feedback,
                    if compact { 14. } else { 17. },
                    Color::srgb(0.68, 0.13, 0.11),
                );
            });
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.),
                right: Val::Px(20.),
                bottom: Val::Px(18.),
                justify_content: JustifyContent::Center,
                ..row()
            })
            .with_children(|p| {
                label(
                    p,
                    art,
                    "漏切不扣命 · 一笔切一串更高分 · 金色水果 +25 · 不要碰炸弹",
                    if compact { 13. } else { 16. },
                    Color::srgb(0.24, 0.21, 0.18),
                );
            });
            if g.failed {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(310.),
                        height: Val::Px(160.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.02, 0.06, 0.10, 0.92)),
                    BorderRadius::all(Val::Px(20.)),
                ))
                .with_children(|panel| {
                    label(panel, art, "忍者休息一下", 24., Color::WHITE);
                    label(panel, art, format!("本次得分 {}", g.score), 15., SOFT);
                    button(
                        panel,
                        art,
                        "重新切水果",
                        Action::FruitReset,
                        140.,
                        44.,
                        Color::srgb(0.82, 0.33, 0.25),
                        Color::WHITE,
                    );
                });
            }
        });
}

fn fruit_color(kind: u8) -> Color {
    let colors = [
        Color::srgb(0.95, 0.28, 0.28),
        Color::srgb(1., 0.70, 0.18),
        Color::srgb(0.28, 0.78, 0.40),
        Color::srgb(0.72, 0.35, 0.80),
        Color::srgb(0.95, 0.52, 0.18),
    ];
    match kind {
        5 => Color::srgb(1.0, 0.78, 0.18),
        6 => Color::srgb(0.91, 0.20, 0.25),
        7 => Color::srgb(0.98, 0.76, 0.20),
        8 => Color::srgb(0.98, 0.47, 0.56),
        9 => Color::srgb(0.95, 0.65, 0.20),
        _ => colors[kind as usize % colors.len()],
    }
}

fn fruit_sprite(parent: &mut ChildSpawnerCommands, art: &Art, kind: u8, bomb: bool, _size: f32) {
    if (6..=9).contains(&kind) && !bomb {
        let cell_w = 647.5;
        let cell_h = 607.;
        let index = (kind - 6) as usize;
        let column = index % 2;
        let row = index / 2;
        let inset = 10.;
        let mut image = ImageNode::new(art.fruit_extras.clone());
        image.rect = Some(Rect::from_corners(
            Vec2::new(column as f32 * cell_w + inset, row as f32 * cell_h + inset),
            Vec2::new(
                (column + 1) as f32 * cell_w - inset,
                (row + 1) as f32 * cell_h - inset,
            ),
        ));
        parent.spawn((
            image,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ));
        return;
    }
    let index = if bomb { 6 } else { kind.min(5) as usize };
    let column = index % 4;
    let row = index / 4;
    let cell = 443.5;
    let inset = 8.;
    let mut image = ImageNode::new(art.fruit_sprites.clone());
    image.rect = Some(Rect::from_corners(
        Vec2::new(column as f32 * cell + inset, row as f32 * cell + inset),
        Vec2::new(
            (column + 1) as f32 * cell - inset,
            (row + 1) as f32 * cell - inset,
        ),
    ));
    parent.spawn((
        image,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
    ));
}

fn bomber_atlas_image(art: &Art, index: usize) -> ImageNode {
    let cell = 443.5;
    let inset = 8.;
    let column = index % 4;
    let row = index / 4;
    let mut image = ImageNode::new(art.bomber_tiles.clone());
    image.rect = Some(Rect::from_corners(
        Vec2::new(column as f32 * cell + inset, row as f32 * cell + inset),
        Vec2::new(
            (column + 1) as f32 * cell - inset,
            (row + 1) as f32 * cell - inset,
        ),
    ));
    image
}

fn bomber_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    compact: bool,
    _wide: bool,
    width: f32,
    height: f32,
) {
    if compact || _wide {
        bomber_ui_compact(commands, s, art, true, _wide, width, height);
    } else {
        bomber_ui_desktop(commands, s, art, width, height);
    }
}

fn bomber_ui_compact(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    compact: bool,
    _wide: bool,
    width: f32,
    height: f32,
) {
    let g = &s.bomber;
    let landscape = width > height;
    let visible_columns = if landscape { BOMBER_COLUMNS } else { 7 };
    let first_column = if landscape {
        0
    } else {
        (g.player.0 - visible_columns / 2).clamp(0, BOMBER_COLUMNS - visible_columns)
    };
    let dpad_size = 58.;
    let dpad_height = 54.;
    let cell = if landscape {
        (height * 0.60 / BOMBER_ROWS as f32).min(width * 0.6375 / BOMBER_COLUMNS as f32)
    } else {
        ((height - 210.) / BOMBER_ROWS as f32)
            .min((width - 24.) / visible_columns as f32)
            .floor()
            .clamp(34., 68.)
    };
    let board = cell * visible_columns as f32;
    let board_left = (width - board - 6.) * 0.5;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.055, 0.22, 0.34)),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.bomber_backdrop.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            if landscape {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0., 0.078, 0.137, 0.33)),
                ));
            }
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(if compact { 12. } else { 26. }),
                right: Val::Px(if compact { 12. } else { 26. }),
                top: Val::Px(if landscape { 11. } else { 8. }),
                height: Val::Px(if landscape { 51. } else { 48. }),
                padding: UiRect::horizontal(Val::Px(if landscape { 15. } else { 10. })),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .insert((
                BackgroundColor(Color::srgba(0.035, 0.137, 0.216, 0.98)),
                BorderRadius::all(Val::Px(18.)),
            ))
            .with_children(|top| {
                top.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(15.),
                    ..row()
                })
                .with_children(|brand| {
                    label(
                        brand,
                        art,
                        "泡泡爆破王",
                        if compact { 20. } else { 29. },
                        Color::WHITE,
                    );
                });
                top.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(33.9),
                    column_gap: Val::Px(8.),
                    ..row()
                })
                .with_children(|stats| {
                    label(
                        stats,
                        art,
                        format!("第 {} 关", g.round),
                        if compact { 16. } else { 21. },
                        Color::WHITE,
                    );
                    label(
                        stats,
                        art,
                        format!("爆破分 {:04}", g.score),
                        if compact { 13. } else { 17. },
                        Color::srgb(0.56, 0.91, 1.),
                    );
                });
                top.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(61.2),
                    ..row()
                })
                .with_children(|p| {
                    for i in 0..5 {
                        label(
                            p,
                            art,
                            if i < g.lives as usize { "♥" } else { "♡" },
                            if compact { 18. } else { 24. },
                            if i < g.lives as usize {
                                Color::srgb(1., 0.30, 0.34)
                            } else {
                                Color::srgb(0.55, 0.68, 0.74)
                            },
                        );
                    }
                });
                top.spawn(Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(8.),
                    ..row()
                })
                .with_children(|nav| {
                    button(
                        nav,
                        art,
                        "暂停",
                        Action::Help,
                        60.,
                        40.,
                        if landscape {
                            Color::NONE
                        } else {
                            Color::srgb(0.12, 0.35, 0.55)
                        },
                        Color::WHITE,
                    );
                    button(
                        nav,
                        art,
                        "游戏库",
                        Action::Library,
                        if compact { 70. } else { 88. },
                        40.,
                        if landscape {
                            Color::NONE
                        } else {
                            Color::srgb(0.12, 0.35, 0.55)
                        },
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(if landscape { height * 0.1444 } else { 66. }),
                    left: Val::Px(board_left),
                    width: Val::Px(board + 6.),
                    height: Val::Px(cell * BOMBER_ROWS as f32 + 6.),
                    border: UiRect::all(Val::Px(3.)),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip(),
                    ..default()
                },
                BomberBoard {
                    first_column,
                    visible_columns,
                },
                BackgroundColor(Color::srgb(0.97, 0.86, 0.63)),
                BorderColor(Color::srgb(0.10, 0.22, 0.34)),
                BorderRadius::all(Val::Px(14.)),
            ))
            .with_children(|grid| {
                for y in 0..BOMBER_ROWS {
                    grid.spawn(Node {
                        width: Val::Px(board),
                        height: Val::Px(cell),
                        flex_shrink: 0.,
                        column_gap: Val::Px(0.),
                        ..row()
                    })
                    .with_children(|line| {
                        for local_x in 0..visible_columns {
                            let x = first_column + local_x;
                            let brick = g.bricks.contains(&(x, y));
                            let pillar = x % 2 == 1 && y % 2 == 1;
                            line.spawn((
                                Node {
                                    width: Val::Px(cell),
                                    height: Val::Px(cell),
                                    flex_shrink: 0.,
                                    border: UiRect::all(Val::Px(1.)),
                                    ..default()
                                },
                                BackgroundColor(if brick {
                                    Color::srgb(0.94, 0.43, 0.32)
                                } else if pillar {
                                    Color::srgb(0.76, 0.78, 0.77)
                                } else if (x - g.player.0).abs() + (y - g.player.1).abs() == 1
                                    && !g.bombs.iter().any(|b| (b.x, b.y) == (x, y))
                                {
                                    Color::srgb(0.80, 0.92, 0.68)
                                } else {
                                    Color::srgb(0.98, 0.89, 0.72)
                                }),
                                BorderColor(Color::srgba(0.18, 0.36, 0.48, 0.30)),
                            ))
                            .with_children(|tile| {
                                let art_index = if brick {
                                    if (x + y) % 3 == 0 { 3 } else { 1 }
                                } else if pillar {
                                    2
                                } else {
                                    0
                                };
                                tile.spawn((
                                    bomber_atlas_image(art, art_index),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        ..default()
                                    },
                                ));
                                if g.bushes.contains(&(x, y)) {
                                    tile.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(cell * 0.08),
                                            top: Val::Px(cell * 0.12),
                                            width: Val::Px(cell * 0.84),
                                            height: Val::Px(cell * 0.76),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.12, 0.38, 0.19)),
                                        BorderRadius::all(Val::Px(cell * 0.34)),
                                    ))
                                    .with_children(|bush| {
                                        for (dx, dy, shade) in [
                                            (0.19, 0.22, Color::srgb(0.18, 0.53, 0.23)),
                                            (0.43, 0.08, Color::srgb(0.24, 0.61, 0.26)),
                                            (0.65, 0.24, Color::srgb(0.14, 0.46, 0.20)),
                                            (0.35, 0.44, Color::srgb(0.28, 0.64, 0.28)),
                                            (0.61, 0.52, Color::srgb(0.19, 0.52, 0.22)),
                                        ] {
                                            bush.spawn((
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    left: Val::Percent(dx * 100.),
                                                    top: Val::Percent(dy * 100.),
                                                    width: Val::Percent(34.),
                                                    height: Val::Percent(43.),
                                                    ..default()
                                                },
                                                BackgroundColor(shade),
                                                BorderRadius::all(Val::Percent(50.)),
                                            ));
                                        }
                                        bush.spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(40.),
                                                top: Val::Percent(26.),
                                                width: Val::Px(cell * 0.12),
                                                height: Val::Px(cell * 0.12),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(1., 0.82, 0.22)),
                                            BorderRadius::all(Val::Percent(50.)),
                                        ));
                                    });
                                } else if g.pots.contains(&(x, y)) {
                                    tile.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(cell * 0.20),
                                            top: Val::Px(cell * 0.15),
                                            width: Val::Px(cell * 0.60),
                                            height: Val::Px(cell * 0.72),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.54, 0.25, 0.12)),
                                        BorderColor(Color::srgb(0.90, 0.59, 0.29)),
                                        BorderRadius::all(Val::Px(cell * 0.22)),
                                    ))
                                    .with_children(|pot| {
                                        pot.spawn((
                                            Node {
                                                width: Val::Percent(55.),
                                                height: Val::Percent(55.),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.70, 0.42, 0.22)),
                                            BorderRadius::all(Val::Percent(50.)),
                                        ));
                                    });
                                }
                            });
                        }
                    });
                }
                for pickup in &g.pickups {
                    if (first_column..first_column + visible_columns).contains(&pickup.x) {
                        bomber_pickup_marker(
                            grid,
                            art,
                            pickup,
                            (pickup.x - first_column) as f32 * cell + cell * 0.22,
                            pickup.y as f32 * cell + cell * 0.22,
                            cell * 0.56,
                        );
                    }
                }
                for bomb in &g.bombs {
                    for (x, y) in g.blast_cells(bomb) {
                        grid.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px((x - first_column) as f32 * cell + 3.),
                                top: Val::Px(y as f32 * cell + 3.),
                                width: Val::Px(cell - 6.),
                                height: Val::Px(cell - 6.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(
                                0.16,
                                0.72,
                                1.,
                                0.20 + (bomb.fuse * 7.).sin().abs() * 0.22,
                            )),
                            BorderRadius::all(Val::Px(6.)),
                        ));
                    }
                }
                for blast in &g.blasts {
                    grid.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px((blast.x - first_column) as f32 * cell + cell * 0.13),
                            top: Val::Px(blast.y as f32 * cell + cell * 0.13),
                            width: Val::Px(cell * (1. - blast.life * 0.5)),
                            height: Val::Px(cell * (1. - blast.life * 0.5)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.26, 0.90, 1., (blast.life / 0.46).min(1.))),
                        BorderRadius::all(Val::Px(12.)),
                    ));
                }
                for bomb in &g.bombs {
                    let pulse = 1. + (bomb.fuse * 9.).sin() * 0.08;
                    grid.spawn(Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px((bomb.x - first_column) as f32 * cell + cell * 0.16),
                        top: Val::Px(bomb.y as f32 * cell + cell * 0.16),
                        width: Val::Px(cell * 0.68),
                        height: Val::Px(cell * 0.68),
                        ..default()
                    })
                    .with_children(|node| {
                        node.spawn((
                            ImageNode::new(art.bomber_action_bubble.clone()),
                            Node {
                                width: Val::Px(cell * 0.68 * pulse),
                                height: Val::Px(cell * 0.68 * pulse),
                                ..default()
                            },
                        ));
                        node.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(9.),
                                top: Val::Px(6.),
                                ..default()
                            },
                            GlobalZIndex(3),
                        ))
                        .with_children(|p| {
                            label(p, art, format!("{}", bomb.fuse.ceil() as i32), 20., NAVY)
                        });
                    });
                }
                grid.spawn((
                    SceneMotion::bob(Vec2::new(
                        (g.visual_player.x - first_column as f32) * cell + cell * 0.14,
                        g.visual_player.y * cell + cell * 0.14,
                    )),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(
                            (g.visual_player.x - first_column as f32) * cell + cell * 0.14,
                        ),
                        top: Val::Px(g.visual_player.y * cell + cell * 0.14),
                        width: Val::Px(cell * 0.72),
                        height: Val::Px(cell * 0.72),
                        ..default()
                    },
                ))
                .with_children(|node| {
                    if g.invulnerable > 0. {
                        node.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(-5.),
                                top: Val::Px(-5.),
                                width: Val::Px(cell * 0.9),
                                height: Val::Px(cell * 0.9),
                                border: UiRect::all(Val::Px(3.)),
                                ..default()
                            },
                            BorderColor(Color::srgb(0.80, 0.98, 1.)),
                            BorderRadius::all(Val::Px(cell)),
                        ));
                    }
                    node.spawn((
                        bomber_atlas_image(art, 4),
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                    ));
                });
                for enemy in &g.enemies {
                    grid.spawn((
                        SceneMotion::bob(Vec2::new(
                            (enemy.visual.x - first_column as f32) * cell + cell * 0.16,
                            enemy.visual.y * cell + cell * 0.16,
                        )),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(
                                (enemy.visual.x - first_column as f32) * cell + cell * 0.16,
                            ),
                            top: Val::Px(enemy.visual.y * cell + cell * 0.16),
                            width: Val::Px(cell * 0.68),
                            height: Val::Px(cell * 0.68),
                            ..default()
                        },
                    ))
                    .with_children(|node| {
                        node.spawn((
                            bomber_monster_image(art, enemy.kind),
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                        ));
                    });
                }
            });
            if landscape {
                bomber_landscape_controls(root, art, width, height);
                root.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(board_left),
                    right: Val::Px(board_left),
                    top: Val::Px(height * 0.776),
                    height: Val::Px(height * 0.165),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::horizontal(Val::Px(12.)),
                    ..default()
                })
                .insert((
                    BackgroundColor(Color::srgb(0.035, 0.137, 0.216)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|hint| {
                    label(hint, art, &g.feedback, 13., Color::WHITE);
                });
            } else {
                root.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(if compact { 14. } else { 42. }),
                    right: Val::Px(if compact { 14. } else { 42. }),
                    bottom: Val::Px(if compact { 16. } else { 26. }),
                    padding: UiRect::horizontal(Val::Px(if compact { 8. } else { 16. })),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(4.),
                    ..row()
                })
                .insert((
                    BackgroundColor(Color::srgba(0.025, 0.13, 0.23, 0.52)),
                    BorderRadius::all(Val::Px(28.)),
                    BorderColor(Color::srgb(0.25, 0.72, 0.91)),
                ))
                .with_children(|controls| {
                    controls
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(3.),
                            ..default()
                        })
                        .with_children(|pad| {
                            button(
                                pad,
                                art,
                                "↑",
                                Action::BomberMove(0, -1),
                                dpad_size,
                                dpad_height,
                                Color::srgb(0.13, 0.38, 0.65),
                                Color::WHITE,
                            );
                            pad.spawn(row()).with_children(|row| {
                                button(
                                    row,
                                    art,
                                    "←",
                                    Action::BomberMove(-1, 0),
                                    dpad_size,
                                    dpad_height,
                                    Color::srgb(0.13, 0.38, 0.65),
                                    Color::WHITE,
                                );
                                button(
                                    row,
                                    art,
                                    "↓",
                                    Action::BomberMove(0, 1),
                                    dpad_size,
                                    dpad_height,
                                    Color::srgb(0.13, 0.38, 0.65),
                                    Color::WHITE,
                                );
                                button(
                                    row,
                                    art,
                                    "→",
                                    Action::BomberMove(1, 0),
                                    dpad_size,
                                    dpad_height,
                                    Color::srgb(0.13, 0.38, 0.65),
                                    Color::WHITE,
                                );
                            });
                        });
                    bomber_place_button(
                        controls,
                        art,
                        if compact { 104. } else { 142. },
                        if compact { 66. } else { 88. },
                        if compact { 13. } else { 17. },
                    );
                });
                root.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(if compact { 96. } else { 225. }),
                    right: Val::Px(if compact { 96. } else { 225. }),
                    bottom: Val::Px(if compact { 8. } else { 10. }),
                    justify_content: JustifyContent::Center,
                    ..row()
                })
                .with_children(|hint| {
                    label(
                        hint,
                        art,
                        &g.feedback,
                        if compact { 11. } else { 14. },
                        Color::WHITE,
                    );
                });
            }
            if g.failed {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(300.),
                        height: Val::Px(206.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.05, 0.12, 0.20, 0.92)),
                    BorderRadius::all(Val::Px(18.)),
                ))
                .with_children(|panel| {
                    label(panel, art, "被水泡困住啦", 24., Color::WHITE);
                    label(panel, art, format!("本次得分 {}", g.score), 15., SOFT);
                    button(
                        panel,
                        art,
                        "重新闯关",
                        Action::BomberReset,
                        136.,
                        44.,
                        Color::srgb(0.26, 0.62, 0.88),
                        Color::WHITE,
                    );
                    button(
                        panel,
                        art,
                        "回到游戏库",
                        Action::Library,
                        136.,
                        42.,
                        if landscape {
                            Color::NONE
                        } else {
                            Color::srgb(0.12, 0.35, 0.55)
                        },
                        Color::WHITE,
                    );
                });
            }
        });
}

fn bomber_landscape_controls(root: &mut ChildSpawnerCommands, art: &Art, width: f32, height: f32) {
    let panel_w = width * 0.152;
    for right in [false, true] {
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: if right {
                    Val::Auto
                } else {
                    Val::Px(width * 0.0146)
                },
                right: if right {
                    Val::Px(width * 0.0146)
                } else {
                    Val::Auto
                },
                top: Val::Px(height * 0.5185),
                width: Val::Px(panel_w),
                height: Val::Px(height * 0.4222),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.035, 0.137, 0.216)),
            BorderRadius::all(Val::Px(15.)),
        ))
        .with_children(|panel| {
            if right {
                panel
                    .spawn((
                        BomberTouchPlace,
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(12.),
                            ..default()
                        },
                    ))
                    .with_children(|action| {
                        action.spawn((
                            ImageNode::new(art.bomber_action_bubble.clone()),
                            Node {
                                width: Val::Px(panel_w * 0.72),
                                height: Val::Px(panel_w * 0.72),
                                ..default()
                            },
                        ));
                        label(action, art, "放置泡泡", 15., Color::WHITE);
                        label(action, art, "支持边走边放", 12., Color::WHITE);
                    });
            } else {
                panel
                    .spawn(Node {
                        margin: UiRect::top(Val::Px(16.)),
                        ..default()
                    })
                    .with_children(|title| {
                        label(title, art, "拖动摇杆", 15., Color::WHITE);
                    });
                panel
                    .spawn((
                        BomberJoystick,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(panel_w * 0.116),
                            top: Val::Px(height * 0.12),
                            width: Val::Px(panel_w * 0.768),
                            height: Val::Px(panel_w * 0.768),
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.09, 0.247, 0.35)),
                        BorderColor(Color::srgb(0.51, 0.80, 0.89)),
                        BorderRadius::all(Val::Percent(50.)),
                    ))
                    .with_children(|pad| {
                        for (symbol, x, y) in [
                            ("↑", 43., 0.),
                            ("↓", 43., 76.),
                            ("←", 3., 38.),
                            ("→", 80., 38.),
                        ] {
                            pad.spawn(Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(x),
                                top: Val::Percent(y),
                                ..default()
                            })
                            .with_children(|arrow| {
                                label(arrow, art, symbol, 15., Color::WHITE);
                            });
                        }
                        pad.spawn((
                            BomberJoystickKnob,
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(46.),
                                height: Val::Percent(46.),
                                left: Val::Percent(27.),
                                top: Val::Percent(27.),
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.247, 0.604, 0.753)),
                            BorderColor(Color::srgb(0.78, 0.95, 1.)),
                            BorderRadius::all(Val::Percent(50.)),
                        ));
                    });
                panel
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(16.),
                        ..default()
                    })
                    .with_children(|hint| {
                        label(hint, art, "松手即停止", 12., Color::WHITE);
                    });
            }
        });
    }
}

fn bomber_joystick_direction(offset: Vec2, radius: f32) -> Option<(i8, i8)> {
    if offset.length() < radius * 0.18 {
        return None;
    }
    if offset.x.abs() > offset.y.abs() {
        Some((if offset.x > 0. { 1 } else { -1 }, 0))
    } else {
        Some((0, if offset.y > 0. { 1 } else { -1 }))
    }
}

fn bomber_monster_image(art: &Art, kind: u8) -> ImageNode {
    let (texture, cell, column) = if kind < 4 {
        (art.bomber_monsters.clone(), 543., kind as usize)
    } else {
        (art.bomber_extra_monsters.clone(), 724., (kind - 4) as usize)
    };
    let inset = 8.;
    let mut image = ImageNode::new(texture);
    image.rect = Some(Rect::from_corners(
        Vec2::new(column as f32 * cell + inset, inset),
        Vec2::new((column + 1) as f32 * cell - inset, 724. - inset),
    ));
    image
}

fn bomber_desktop_direction(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    text: &str,
    action: Action,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
) {
    parent
        .spawn((
            Button,
            action,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                width: Val::Px(width),
                height: Val::Px(height),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.40, 0.62, 0.72)),
            BorderColor(Color::srgba(1., 1., 1., 0.78)),
            BorderRadius::all(Val::Percent(50.)),
        ))
        .with_children(|arrow| label(arrow, art, text, height * 0.40, Color::WHITE));
}

fn bomber_place_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    width: f32,
    height: f32,
    font_size: f32,
) {
    let bubble_size = width.min(height);
    parent
        .spawn((
            Button,
            Action::BomberPlace,
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            ZIndex(2),
        ))
        .with_children(|button| {
            button.spawn((
                ImageNode::new(art.bomber_action_bubble.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(bubble_size),
                    height: Val::Px(bubble_size),
                    left: Val::Px((width - bubble_size) * 0.5),
                    top: Val::Px((height - bubble_size) * 0.5),
                    ..default()
                },
            ));
            button
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(width * 0.04),
                        right: Val::Px(width * 0.04),
                        bottom: Val::Px(0.),
                        height: Val::Px((font_size * 1.65).max(22.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.015, 0.075, 0.14, 0.91)),
                    BorderRadius::all(Val::Px(font_size)),
                ))
                .with_children(|pill| {
                    label(pill, art, "放置泡泡", font_size, Color::WHITE);
                });
        });
}

fn bomber_pickup_marker(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    pickup: &BomberPickup,
    left: f32,
    top: f32,
    size: f32,
) {
    let (symbol, color) = match pickup.kind {
        BomberPickupKind::Range => ("↗", Color::srgb(0.05, 0.62, 0.95)),
        BomberPickupKind::Capacity => ("＋", Color::srgb(0.54, 0.30, 0.88)),
        BomberPickupKind::Heart => ("♥", Color::srgb(0.93, 0.25, 0.38)),
    };
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                width: Val::Px(size),
                height: Val::Px(size),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1., 0.91, 0.48)),
            BorderColor(Color::WHITE),
            BorderRadius::all(Val::Percent(50.)),
        ))
        .with_children(|node| label(node, art, symbol, size * 0.58, color));
}

fn bomber_ui_desktop(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let g = &s.bomber;
    let sx = width / 1672.;
    let sy = height / 941.;
    // Reserve a clear HUD strip, then fit all nine rows into the remaining
    // play area so the top row stays visible without shrinking the board.
    let board_top = 100. * sy;
    let board_height = (height - board_top).max(1.);
    let cell_w = width / BOMBER_COLUMNS as f32;
    let cell_h = board_height / BOMBER_ROWS as f32;
    let rating = if g.remaining >= 100. {
        3
    } else if g.remaining >= 45. {
        2
    } else {
        1
    };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .with_children(|root| {
            // Full-bleed arena art: controls and the live board float over the whole scene.
            root.spawn((
                ImageNode::new(art.bomber_backdrop.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(board_top),
                    width: Val::Percent(100.),
                    height: Val::Px(board_height),
                    ..default()
                },
                BomberBoard {
                    first_column: 0,
                    visible_columns: BOMBER_COLUMNS,
                },
            ))
            .with_children(|grid| {
                // Fixed stone pillars and destructible objects are laid over the arena floor.
                for y in 0..BOMBER_ROWS {
                    for x in 0..BOMBER_COLUMNS {
                        let left = x as f32 * cell_w;
                        let top = y as f32 * cell_h;
                        if g.bushes.contains(&(x, y)) {
                            grid.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(left + cell_w * 0.08),
                                    top: Val::Px(top + cell_h * 0.12),
                                    width: Val::Px(cell_w * 0.84),
                                    height: Val::Px(cell_h * 0.76),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.12, 0.38, 0.19)),
                                BorderRadius::all(Val::Px(cell_h * 0.34)),
                            ))
                            .with_children(|bush| {
                                for (dx, dy, shade) in [
                                    (0.19, 0.22, Color::srgb(0.18, 0.53, 0.23)),
                                    (0.43, 0.08, Color::srgb(0.24, 0.61, 0.26)),
                                    (0.65, 0.24, Color::srgb(0.14, 0.46, 0.20)),
                                    (0.35, 0.44, Color::srgb(0.28, 0.64, 0.28)),
                                    (0.61, 0.52, Color::srgb(0.19, 0.52, 0.22)),
                                ] {
                                    bush.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Percent(dx * 100.),
                                            top: Val::Percent(dy * 100.),
                                            width: Val::Percent(34.),
                                            height: Val::Percent(43.),
                                            ..default()
                                        },
                                        BackgroundColor(shade),
                                        BorderRadius::all(Val::Percent(50.)),
                                    ));
                                }
                                bush.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(40.),
                                        top: Val::Percent(26.),
                                        width: Val::Px(7. * sx),
                                        height: Val::Px(7. * sy),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(1., 0.82, 0.22)),
                                    BorderRadius::all(Val::Percent(50.)),
                                ));
                            });
                        } else if g.pots.contains(&(x, y)) {
                            grid.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(left + cell_w * 0.20),
                                    top: Val::Px(top + cell_h * 0.15),
                                    width: Val::Px(cell_w * 0.60),
                                    height: Val::Px(cell_h * 0.72),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.54, 0.25, 0.12)),
                                BorderColor(Color::srgb(0.90, 0.59, 0.29)),
                                BorderRadius::all(Val::Px(cell_h * 0.22)),
                            ))
                            .with_children(|pot| {
                                pot.spawn((
                                    Node {
                                        width: Val::Percent(55.),
                                        height: Val::Percent(55.),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.70, 0.42, 0.22)),
                                    BorderRadius::all(Val::Percent(50.)),
                                ));
                            });
                        } else if g.bricks.contains(&(x, y)) {
                            grid.spawn((
                                bomber_atlas_image(art, if (x + y) % 4 == 0 { 3 } else { 1 }),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(left),
                                    top: Val::Px(top),
                                    width: Val::Px(cell_w),
                                    height: Val::Px(cell_h),
                                    ..default()
                                },
                            ));
                        } else if x % 2 == 1 && y % 2 == 1 {
                            grid.spawn((
                                bomber_atlas_image(art, 2),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(left),
                                    top: Val::Px(top),
                                    width: Val::Px(cell_w),
                                    height: Val::Px(cell_h),
                                    ..default()
                                },
                            ));
                        }
                    }
                }
                for pickup in &g.pickups {
                    bomber_pickup_marker(
                        grid,
                        art,
                        pickup,
                        pickup.x as f32 * cell_w + cell_w * 0.22,
                        pickup.y as f32 * cell_h + cell_h * 0.22,
                        cell_w.min(cell_h) * 0.56,
                    );
                }
                for bomb in &g.bombs {
                    for (x, y) in g.blast_cells(bomb) {
                        grid.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(x as f32 * cell_w),
                                top: Val::Px(y as f32 * cell_h),
                                width: Val::Px(cell_w),
                                height: Val::Px(cell_h),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(
                                0.10,
                                0.73,
                                1.,
                                0.24 + (bomb.fuse * 7.).sin().abs() * 0.20,
                            )),
                            BorderRadius::all(Val::Px(14. * sx)),
                        ));
                    }
                }
                for blast in &g.blasts {
                    grid.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(blast.x as f32 * cell_w + cell_w * 0.08),
                            top: Val::Px(blast.y as f32 * cell_h + cell_h * 0.08),
                            width: Val::Px(cell_w * 0.84),
                            height: Val::Px(cell_h * 0.84),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.28, 0.89, 1., (blast.life / 0.46).min(1.))),
                        BorderRadius::all(Val::Px(22. * sx)),
                    ));
                }
                for bomb in &g.bombs {
                    let pulse = 1. + (bomb.fuse * 9.).sin() * 0.08;
                    grid.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(
                                bomb.x as f32 * cell_w + cell_w * (0.16 - (pulse - 1.) * 0.3),
                            ),
                            top: Val::Px(
                                bomb.y as f32 * cell_h + cell_h * (0.16 - (pulse - 1.) * 0.3),
                            ),
                            width: Val::Px(cell_w * 0.68 * pulse),
                            height: Val::Px(cell_h * 0.68 * pulse),
                            ..default()
                        },
                        ImageNode::new(art.bomber_action_bubble.clone()),
                    ));
                }
                grid.spawn((
                    SceneMotion::bob(Vec2::new(
                        g.visual_player.x * cell_w + cell_w * 0.14,
                        g.visual_player.y * cell_h + cell_h * 0.12,
                    )),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(g.visual_player.x * cell_w + cell_w * 0.14),
                        top: Val::Px(g.visual_player.y * cell_h + cell_h * 0.12),
                        width: Val::Px(cell_w * 0.72),
                        height: Val::Px(cell_h * 0.76),
                        ..default()
                    },
                    bomber_atlas_image(art, 4),
                ));
                for enemy in &g.enemies {
                    grid.spawn((
                        SceneMotion::bob(Vec2::new(
                            enemy.visual.x * cell_w + cell_w * 0.16,
                            enemy.visual.y * cell_h + cell_h * 0.12,
                        )),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(enemy.visual.x * cell_w + cell_w * 0.16),
                            top: Val::Px(enemy.visual.y * cell_h + cell_h * 0.12),
                            width: Val::Px(cell_w * 0.68),
                            height: Val::Px(cell_h * 0.76),
                            ..default()
                        },
                        bomber_monster_image(art, enemy.kind),
                    ));
                }
            });

            let hud = Color::srgba(0.025, 0.10, 0.20, 0.91);
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(506. * sx),
                    top: Val::Px(12. * sy),
                    width: Val::Px(370. * sx),
                    height: Val::Px(82. * sy),
                    padding: UiRect::horizontal(Val::Px(18. * sx)),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(16. * sx),
                    ..row()
                },
                BackgroundColor(hud),
                BorderRadius::all(Val::Px(22. * sy)),
                ZIndex(6),
            ))
            .with_children(|stats| {
                stats.spawn((
                    bomber_atlas_image(art, 4),
                    Node {
                        width: Val::Px(58. * sx),
                        height: Val::Px(62. * sy),
                        ..default()
                    },
                ));
                stats.spawn(row()).with_children(|hearts| {
                    for i in 0..3 {
                        label(
                            hearts,
                            art,
                            if i < g.lives as usize { "♥" } else { "♡" },
                            34. * sy,
                            if i < g.lives as usize {
                                Color::srgb(1., 0.28, 0.35)
                            } else {
                                Color::srgb(0.48, 0.54, 0.62)
                            },
                        );
                    }
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(925. * sx),
                    top: Val::Px(12. * sy),
                    width: Val::Px(258. * sx),
                    height: Val::Px(82. * sy),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(12. * sx),
                    ..row()
                },
                BackgroundColor(hud),
                BorderRadius::all(Val::Px(22. * sy)),
                ZIndex(6),
            ))
            .with_children(|timer| {
                label(timer, art, "◴", 42. * sy, Color::WHITE);
                label(
                    timer,
                    art,
                    format!(
                        "{:02}:{:02}",
                        (g.remaining as u32) / 60,
                        (g.remaining as u32) % 60
                    ),
                    38. * sy,
                    Color::WHITE,
                );
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(1220. * sx),
                    top: Val::Px(12. * sy),
                    width: Val::Px(205. * sx),
                    height: Val::Px(82. * sy),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(2. * sy),
                    ..default()
                },
                BackgroundColor(hud),
                BorderRadius::all(Val::Px(22. * sy)),
                ZIndex(6),
            ))
            .with_children(|level| {
                label(
                    level,
                    art,
                    format!(
                        "关卡 {}-{}",
                        g.round.min(BOMBER_MAX_ROUNDS),
                        BOMBER_MAX_ROUNDS
                    ),
                    27. * sy,
                    Color::WHITE,
                );
                level.spawn(row()).with_children(|stars| {
                    for i in 0..3 {
                        label(
                            stars,
                            art,
                            "★",
                            30. * sy,
                            if i < rating {
                                Color::srgb(1., 0.77, 0.11)
                            } else {
                                Color::srgb(0.25, 0.33, 0.47)
                            },
                        );
                    }
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(24. * sx),
                    top: Val::Px(10. * sy),
                    ..row()
                },
                ZIndex(7),
            ))
            .with_children(|pause| bomber_ui_pause_button(pause, art, g, sx, sy));

            // Floating left thumb pad and right water-bubble action.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20. * sx),
                    bottom: Val::Px(35. * sy),
                    width: Val::Px(270. * sx),
                    height: Val::Px(260. * sy),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.03, 0.18, 0.28, 0.18)),
                BorderColor(Color::srgba(1., 1., 1., 0.38)),
                BorderRadius::all(Val::Percent(50.)),
                ZIndex(8),
            ))
            .with_children(|pad| {
                bomber_desktop_direction(
                    pad,
                    art,
                    "▲",
                    Action::BomberMove(0, -1),
                    102. * sx,
                    17. * sy,
                    66. * sx,
                    66. * sy,
                );
                bomber_desktop_direction(
                    pad,
                    art,
                    "◀",
                    Action::BomberMove(-1, 0),
                    17. * sx,
                    97. * sy,
                    66. * sx,
                    66. * sy,
                );
                bomber_desktop_direction(
                    pad,
                    art,
                    "▶",
                    Action::BomberMove(1, 0),
                    187. * sx,
                    97. * sy,
                    66. * sx,
                    66. * sy,
                );
                bomber_desktop_direction(
                    pad,
                    art,
                    "▼",
                    Action::BomberMove(0, 1),
                    102. * sx,
                    177. * sy,
                    66. * sx,
                    66. * sy,
                );
                pad.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(100. * sx),
                        top: Val::Px(95. * sy),
                        width: Val::Px(70. * sx),
                        height: Val::Px(70. * sy),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.82, 0.92, 0.96, 0.60)),
                    BorderColor(Color::srgba(1., 1., 1., 0.7)),
                    BorderRadius::all(Val::Percent(50.)),
                ));
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(25. * sx),
                    bottom: Val::Px(28. * sy),
                    ..default()
                },
                ZIndex(8),
            ))
            .with_children(|action| {
                bomber_place_button(action, art, 220. * sx, 250. * sy, 23. * sy)
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(310. * sx),
                    right: Val::Px(300. * sx),
                    bottom: Val::Px(20. * sy),
                    justify_content: JustifyContent::Center,
                    ..row()
                },
                ZIndex(9),
            ))
            .with_children(|hint| label(hint, art, &g.feedback, 20. * sy, Color::WHITE));

            if g.paused || g.failed || g.won {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(
                        0.01,
                        0.06,
                        0.12,
                        if g.paused { 0.56 } else { 0.74 },
                    )),
                    ZIndex(20),
                ))
                .with_children(|modal| {
                    modal
                        .spawn((
                            Node {
                                width: Val::Px(420. * sx),
                                height: Val::Px(330. * sy),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(16. * sy),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.025, 0.13, 0.24, 0.96)),
                            BorderColor(Color::srgb(0.25, 0.72, 0.95)),
                            BorderRadius::all(Val::Px(28. * sx)),
                        ))
                        .with_children(|card| {
                            label(
                                card,
                                art,
                                if g.won {
                                    "闯关成功！"
                                } else if g.failed {
                                    "挑战结束"
                                } else {
                                    "游戏暂停"
                                },
                                38. * sy,
                                Color::WHITE,
                            );
                            label(
                                card,
                                art,
                                format!("得分 {} · 本关 {} 星", g.score, rating),
                                22. * sy,
                                Color::srgb(0.67, 0.9, 1.),
                            );
                            if g.failed || g.won {
                                button(
                                    card,
                                    art,
                                    "再玩一次",
                                    Action::BomberReset,
                                    172. * sx,
                                    58. * sy,
                                    Color::srgb(0.02, 0.58, 0.92),
                                    Color::WHITE,
                                );
                            } else {
                                button(
                                    card,
                                    art,
                                    "继续游戏",
                                    Action::BomberTogglePause,
                                    172. * sx,
                                    58. * sy,
                                    Color::srgb(0.02, 0.58, 0.92),
                                    Color::WHITE,
                                );
                            }
                            button(
                                card,
                                art,
                                "回到游戏库",
                                Action::Library,
                                172. * sx,
                                54. * sy,
                                Color::srgb(0.12, 0.35, 0.55),
                                Color::WHITE,
                            );
                        });
                });
            }
        });
}

fn bomber_ui_pause_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    game: &BomberGame,
    sx: f32,
    sy: f32,
) {
    button(
        parent,
        art,
        if game.paused { "▶" } else { "Ⅱ" },
        Action::BomberTogglePause,
        88. * sx,
        82. * sy,
        Color::srgb(0.02, 0.50, 0.91),
        Color::WHITE,
    );
}

fn bubble_color(kind: BubbleKind) -> Color {
    match kind {
        BubbleKind::Coral => Color::srgb(0.94, 0.31, 0.35),
        BubbleKind::Gold => Color::srgb(1.00, 0.70, 0.18),
        BubbleKind::Mint => Color::srgb(0.23, 0.79, 0.61),
        BubbleKind::Violet => Color::srgb(0.55, 0.40, 0.91),
        BubbleKind::Rainbow => Color::srgb(0.96, 0.48, 0.72),
        BubbleKind::Bomb => Color::srgb(0.26, 0.28, 0.40),
    }
}

fn fruit_slice_sprite(parent: &mut ChildSpawnerCommands, slice: &FruitSlice, size: f32, fade: f32) {
    let rgba = fruit_color(slice.kind).to_srgba();
    let juice = Color::srgba(rgba.red, rgba.green, rgba.blue, fade);
    let cut_face = Color::srgba(1., 0.86, 0.57, fade * 0.96);
    let cut_edge = Color::srgba(
        (rgba.red + 1.) * 0.5,
        (rgba.green + 1.) * 0.5,
        (rgba.blue + 1.) * 0.5,
        fade,
    );
    let progress = 1. - fade;
    let split_distance = size * (0.16 + progress * 0.66);
    let half_size = size * 0.64;
    let center = size * 0.52;

    // A growing translucent splash makes the first frame of a cut pop even
    // when the player's stroke is fast.
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(center - size * (0.35 + progress * 0.18)),
            top: Val::Px(center - size * (0.35 + progress * 0.18)),
            width: Val::Px(size * (0.70 + progress * 0.36)),
            height: Val::Px(size * (0.70 + progress * 0.36)),
            ..default()
        },
        BackgroundColor(Color::srgba(rgba.red, rgba.green, rgba.blue, fade * 0.30)),
        BorderRadius::all(Val::Px(size)),
    ));

    // Dot the blade along the exact swipe vector. This reads as a diagonal
    // swipe too, while avoiding UI transform/rotation artefacts on Android.
    for i in 0..9 {
        let t = i as f32 / 8. - 0.5;
        let offset = slice.slash * (t * size * 1.25);
        let dot = if i == 4 { 8. } else { 4.5 } * (0.82 + fade * 0.18);
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(center + offset.x - dot * 0.5),
                top: Val::Px(center + offset.y - dot * 0.5),
                width: Val::Px(dot),
                height: Val::Px(dot),
                ..default()
            },
            BackgroundColor(Color::srgba(1., 0.99, 0.91, fade)),
            BorderRadius::all(Val::Px(dot)),
        ));
    }

    // The pale stripe is the exposed fruit flesh. It sits on the side of each
    // half facing the other half, so the player sees a genuine cut surface
    // rather than two whole fruit icons drifting apart.
    let horizontal_cut = slice.split.y.abs() >= slice.split.x.abs();
    for side in [-1., 1.] {
        let offset = slice.split * (split_distance * side);
        let lower_or_right = if horizontal_cut {
            offset.y > 0.
        } else {
            offset.x > 0.
        };
        parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(center - half_size * 0.5 + offset.x),
                    top: Val::Px(center - half_size * 0.42 + offset.y),
                    width: Val::Px(half_size),
                    height: Val::Px(half_size * 0.72),
                    border: UiRect::all(Val::Px(2.)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(juice),
                BorderColor(cut_edge),
                BorderRadius::all(Val::Px(size)),
            ))
            .with_children(|half| {
                half.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(if horizontal_cut {
                            12.
                        } else if lower_or_right {
                            0.
                        } else {
                            82.
                        }),
                        top: Val::Percent(if horizontal_cut {
                            if lower_or_right { 0. } else { 82. }
                        } else {
                            12.
                        }),
                        width: Val::Percent(if horizontal_cut { 76. } else { 18. }),
                        height: Val::Percent(if horizontal_cut { 18. } else { 76. }),
                        ..default()
                    },
                    BackgroundColor(cut_face),
                    BorderRadius::all(Val::Px(5.)),
                ));
                half.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(45.),
                        top: Val::Percent(43.),
                        width: Val::Percent(12.),
                        height: Val::Percent(16.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.39, 0.16, 0.10, fade * 0.78)),
                    BorderRadius::all(Val::Px(6.)),
                ));
            });
    }

    // A few asymmetric drops are cheap to draw and keep the impact visible
    // around the moving halves instead of hiding it at their center.
    let tangent = Vec2::new(slice.split.y, -slice.split.x);
    for (normal, sideways, diameter) in [
        (0.72, -0.34, 0.15),
        (-0.66, 0.42, 0.12),
        (0.30, 0.70, 0.10),
        (-0.25, -0.74, 0.09),
    ] {
        let offset = slice.split * (normal * size * (0.58 + progress * 0.52))
            + tangent * (sideways * size * (0.58 + progress * 0.52));
        let dot = size * diameter * (1. - progress * 0.22);
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(center + offset.x - dot * 0.5),
                top: Val::Px(center + offset.y - dot * 0.5),
                width: Val::Px(dot),
                height: Val::Px(dot),
                ..default()
            },
            BackgroundColor(cut_edge),
            BorderRadius::all(Val::Px(dot)),
        ));
    }
}

fn bubble_sprite(parent: &mut ChildSpawnerCommands, art: &Art, kind: BubbleKind, size: f32) {
    let color = bubble_color(kind);
    parent
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
            BorderColor(Color::srgba(1., 1., 1., 0.65)),
            BorderRadius::all(Val::Px(size)),
        ))
        .with_children(|body| {
            body.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(18.),
                    top: Val::Percent(14.),
                    width: Val::Percent(30.),
                    height: Val::Percent(23.),
                    ..default()
                },
                BackgroundColor(Color::srgba(1., 1., 1., 0.56)),
                BorderRadius::all(Val::Px(size)),
            ));
            if kind == BubbleKind::Rainbow {
                body.spawn((
                    Node {
                        width: Val::Percent(46.),
                        height: Val::Percent(16.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.45, 0.80, 0.95)),
                    BorderRadius::all(Val::Px(5.)),
                ));
            } else if kind == BubbleKind::Bomb {
                label(body, art, "★", size * 0.45, Color::srgb(1., 0.80, 0.25));
            }
        });
}

fn tank_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    compact: bool,
    width: f32,
    height: f32,
) {
    if compact {
        tank_mobile_ui(commands, &s.tank, art, width, height);
    } else {
        tank_wide_ui(commands, &s.tank, art, width, height);
    }
}

fn tank_mobile_ui(commands: &mut Commands, g: &TankGame, art: &Art, width: f32, height: f32) {
    let extra_wide = width / height > 2.0;
    let board_side = height * if extra_wide { 0.86 } else { 0.765 };
    let cell = board_side / 7.0;
    let board_left = (width - board_side) * 0.5;
    let board_top = height * if extra_wide { 0.092 } else { 0.105 };
    let stick_side = height * if extra_wide { 0.34 } else { 0.29 };
    let fire_side = height * if extra_wide { 0.29 } else { 0.26 };
    let background = if extra_wide {
        art.tank_mobile_background_wide.clone()
    } else {
        art.tank_mobile_background_regular.clone()
    };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Relative,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.09, 0.22, 0.10)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ImageNode {
                    image_mode: NodeImageMode::Stretch,
                    ..ImageNode::new(background)
                },
            ));

            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(board_left),
                    top: Val::Px(board_top),
                    width: Val::Px(board_side),
                    height: Val::Px(board_side),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.35, 0.14)),
                BorderColor(Color::srgba(0.08, 0.20, 0.10, 0.8)),
            ))
            .with_children(|board| {
                for y in 0..7_i8 {
                    board
                        .spawn(Node {
                            width: Val::Px(board_side),
                            height: Val::Px(cell),
                            min_height: Val::Px(cell),
                            column_gap: Val::Px(0.),
                            ..row()
                        })
                        .with_children(|row_node| {
                            for x in 0..7_i8 {
                                let wall = g.walls.contains(&(x, y));
                                let steel = g.steel.contains(&(x, y));
                                let bush = g.bushes.contains(&(x, y));
                                let ground = if x == 3 || (x == 1 || x == 5) && y % 3 == 0 {
                                    1
                                } else {
                                    0
                                };
                                row_node
                                    .spawn((
                                        Node {
                                            width: Val::Px(cell),
                                            height: Val::Px(cell),
                                            min_width: Val::Px(cell),
                                            min_height: Val::Px(cell),
                                            border: UiRect::all(Val::Px(0.5)),
                                            ..default()
                                        },
                                        tank_concept_tile(
                                            art,
                                            ground,
                                            x as usize * 7 + y as usize * 11,
                                        ),
                                        BorderColor(Color::srgba(0.16, 0.31, 0.13, 0.65)),
                                    ))
                                    .with_children(|tile| {
                                        let obstacle = if wall {
                                            Some(2)
                                        } else if steel {
                                            Some(3)
                                        } else if bush {
                                            Some(4)
                                        } else {
                                            None
                                        };
                                        if let Some(index) = obstacle {
                                            tile.spawn((
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    left: Val::Px(cell * 0.04),
                                                    top: Val::Px(cell * 0.04),
                                                    width: Val::Px(cell * 0.92),
                                                    height: Val::Px(cell * 0.92),
                                                    ..default()
                                                },
                                                tank_concept_tile(art, index, (x + y) as usize),
                                            ));
                                        }
                                    });
                            }
                        });
                }
                let draw_tank = |parent: &mut ChildSpawnerCommands,
                                 x: i8,
                                 y: i8,
                                 friendly: bool,
                                 tint: Color,
                                 direction: (i8, i8)| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px((x as f32 - 0.125) * cell),
                            top: Val::Px((y as f32 - 0.125) * cell),
                            width: Val::Px(cell * 1.25),
                            height: Val::Px(cell * 1.25),
                            ..default()
                        },
                        Transform::from_rotation(Quat::from_rotation_z(match direction {
                            (0, -1) => 0.0,
                            (1, 0) => -std::f32::consts::FRAC_PI_2,
                            (0, 1) => std::f32::consts::PI,
                            (-1, 0) => std::f32::consts::FRAC_PI_2,
                            _ => 0.0,
                        })),
                        tank_sprite_image(art, friendly, tint),
                    ));
                };
                draw_tank(
                    board,
                    g.player.0,
                    g.player.1,
                    true,
                    if g.invulnerable > 0.0 {
                        Color::srgb(0.85, 0.97, 1.0)
                    } else {
                        Color::WHITE
                    },
                    g.direction,
                );
                for enemy in &g.enemies {
                    draw_tank(
                        board,
                        enemy.x,
                        enemy.y,
                        false,
                        if enemy.warning > 0.0 {
                            Color::srgb(1.0, 0.90, 0.62)
                        } else {
                            Color::WHITE
                        },
                        enemy.direction,
                    );
                }
                for bullet in &g.bullets {
                    board.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px((bullet.x - 0.10) * cell),
                            top: Val::Px((bullet.y - 0.10) * cell),
                            width: Val::Px(cell * 0.20),
                            height: Val::Px(cell * 0.20),
                            ..default()
                        },
                        BackgroundColor(if bullet.friendly {
                            Color::srgb(1.0, 0.84, 0.20)
                        } else {
                            Color::srgb(1.0, 0.39, 0.15)
                        }),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                }
            });

            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(width * 0.018),
                right: Val::Px(width * 0.018),
                top: Val::Px(height * 0.012),
                height: Val::Px(height * 0.085),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..row()
            })
            .with_children(|hud| {
                tank_hud_stat(
                    hud,
                    art,
                    "★",
                    "战区",
                    format!("第 {} 关", g.wave),
                    Color::WHITE,
                );
                tank_hud_stat(
                    hud,
                    art,
                    "分",
                    "得分",
                    format!("{:05}", g.score),
                    Color::WHITE,
                );
                tank_hud_stat(hud, art, "盾", "护甲", g.lives.to_string(), Color::WHITE);
                tank_hud_stat(
                    hud,
                    art,
                    "敌",
                    "敌军",
                    (g.enemies.len() + g.reserves).to_string(),
                    Color::WHITE,
                );
                button(
                    hud,
                    art,
                    "重开",
                    Action::TankReset,
                    74.,
                    38.,
                    Color::srgb(0.03, 0.17, 0.19),
                    Color::WHITE,
                );
                button(
                    hud,
                    art,
                    "游戏库",
                    Action::Library,
                    88.,
                    38.,
                    Color::srgb(0.07, 0.23, 0.53),
                    Color::WHITE,
                );
            });

            root.spawn((
                TankStick,
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(width * 0.028),
                    bottom: Val::Px(height * if extra_wide { 0.08 } else { 0.09 }),
                    width: Val::Px(stick_side),
                    height: Val::Px(stick_side),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.01, 0.13, 0.18, 0.72)),
                BorderColor(Color::srgb(0.37, 0.82, 0.96)),
                BorderRadius::all(Val::Percent(50.)),
            ))
            .with_children(|stick| {
                for (glyph, left, top) in [
                    ("▲", 45., 5.),
                    ("▼", 45., 79.),
                    ("◀", 9., 42.),
                    ("▶", 78., 42.),
                ] {
                    stick
                        .spawn(Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(left),
                            top: Val::Percent(top),
                            ..default()
                        })
                        .with_children(|arrow| {
                            label(arrow, art, glyph, 17., Color::srgb(0.75, 0.96, 1.0))
                        });
                }
                stick.spawn((
                    TankKnob,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(32.),
                        top: Val::Percent(32.),
                        width: Val::Percent(36.),
                        height: Val::Percent(36.),
                        border: UiRect::all(Val::Px(3.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.29, 0.74, 0.86)),
                    BorderColor(Color::srgb(0.79, 0.98, 1.0)),
                    BorderRadius::all(Val::Percent(50.)),
                ));
            });

            root.spawn((
                TankFirePad,
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(width * 0.025),
                    bottom: Val::Px(height * if extra_wide { 0.09 } else { 0.10 }),
                    width: Val::Px(fire_side),
                    height: Val::Px(fire_side),
                    border: UiRect::all(Val::Px(4.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.91, 0.17, 0.07)),
                BorderColor(Color::srgb(1.0, 0.75, 0.25)),
                BorderRadius::all(Val::Percent(50.)),
            ))
            .with_children(|fire| {
                fire.spawn((
                    Node {
                        width: Val::Px(fire_side * 0.31),
                        height: Val::Px(fire_side * 0.31),
                        border: UiRect::all(Val::Px(3.)),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Percent(50.)),
                ))
                .with_children(|target| {
                    target.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(45.),
                            top: Val::Percent(-20.),
                            width: Val::Percent(10.),
                            height: Val::Percent(140.),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        BorderRadius::all(Val::Px(3.)),
                    ));
                    target.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(-20.),
                            top: Val::Percent(45.),
                            width: Val::Percent(140.),
                            height: Val::Percent(10.),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        BorderRadius::all(Val::Px(3.)),
                    ));
                });
                label(fire, art, "开火", 24., Color::WHITE);
            });

            if g.failed {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.02, 0.05, 0.04, 0.86)),
                    ZIndex(5),
                ))
                .with_children(|panel| {
                    label(panel, art, "基地失守", 32., Color::srgb(1.0, 0.67, 0.27));
                    label(
                        panel,
                        art,
                        "可以重试当前关卡，护甲会补满",
                        18.,
                        Color::WHITE,
                    );
                    panel.spawn(row()).with_children(|actions| {
                        button(
                            actions,
                            art,
                            "重试本关",
                            Action::TankRetry,
                            118.,
                            46.,
                            Color::srgb(0.30, 0.74, 0.60),
                            NAVY,
                        );
                        button(
                            actions,
                            art,
                            "返回游戏库",
                            Action::Library,
                            128.,
                            46.,
                            SLATE,
                            Color::WHITE,
                        );
                    });
                });
            }
        });
}
fn tank_concept_tile(art: &Art, index: usize, variation: usize) -> ImageNode {
    // Source rectangles are taken from unobstructed cells of the approved
    // concept image. This keeps the live arena on the same palette and texture.
    let (left, top, right, bottom) = match index {
        0 => match variation % 5 {
            0 => (222., 299., 337., 414.),
            1 => (340., 191., 449., 297.),
            2 => (667., 541., 769., 655.),
            3 => (898., 300., 1002., 413.),
            _ => (1220., 542., 1325., 653.),
        },
        1 => match variation % 3 {
            0 => (561., 541., 663., 655.),
            1 => (563., 192., 665., 295.),
            _ => (1006., 542., 1108., 655.),
        },
        2 => match variation % 3 {
            0 => (452., 191., 557., 298.),
            1 => (451., 542., 557., 655.),
            _ => (1114., 542., 1218., 654.),
        },
        3 => (773., 290., 895., 417.),
        4 => match variation % 3 {
            0 => (330., 418., 445., 541.),
            1 => (450., 418., 561., 541.),
            _ => (1220., 418., 1330., 541.),
        },
        _ => (222., 299., 337., 414.),
    };
    let mut image = ImageNode::new(art.tank_concept.clone());
    image.image_mode = NodeImageMode::Stretch;
    image.rect = Some(Rect::from_corners(
        Vec2::new(left, top),
        Vec2::new(right, bottom),
    ));
    image
}

fn tank_sprite_image(art: &Art, friendly: bool, tint: Color) -> ImageNode {
    let cell = 887.;
    let column = usize::from(!friendly);
    let mut image = ImageNode::new(art.tank_sprites.clone());
    image.image_mode = NodeImageMode::Stretch;
    image.color = tint;
    image.rect = Some(Rect::from_corners(
        Vec2::new(column as f32 * cell, 0.),
        Vec2::new((column + 1) as f32 * cell, cell),
    ));
    image
}

fn tank_hud_stat(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    _icon: &str,
    caption: &str,
    value: impl Into<String>,
    _accent: Color,
) {
    let value = value.into();
    let shield_count = value.parse::<usize>().unwrap_or(0).min(3);
    let is_level_badge = caption == "战区";
    let is_score = caption == "得分";
    let is_armor = caption == "护甲";
    let is_enemy = caption == "敌军";
    parent
        .spawn((
            Node {
                width: if is_level_badge {
                    Val::Px(170.)
                } else if is_score {
                    Val::Px(135.)
                } else {
                    Val::Auto
                },
                padding: UiRect::axes(Val::Px(if is_level_badge { 6. } else { 8. }), Val::Px(5.)),
                column_gap: Val::Px(8.),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                ..row()
            },
            BackgroundColor(Color::srgb(0.02, 0.09, 0.08)),
            BorderColor(if is_level_badge {
                Color::srgba(0.75, 0.55, 0.13, 0.7)
            } else {
                Color::srgba(0.31, 0.55, 0.38, 0.55)
            }),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_children(|stat| {
            if is_level_badge || is_armor {
                let (rect, icon_width) = if is_level_badge {
                    (
                        Rect::from_corners(Vec2::new(41., 8.), Vec2::new(117., 50.)),
                        58.,
                    )
                } else {
                    (
                        Rect::from_corners(Vec2::new(896., 11.), Vec2::new(931., 50.)),
                        26.,
                    )
                };
                stat.spawn((
                    Node {
                        width: Val::Px(icon_width),
                        height: Val::Px(32.),
                        ..default()
                    },
                    ImageNode {
                        rect: Some(rect),
                        image_mode: NodeImageMode::Stretch,
                        ..ImageNode::new(art.tank_concept.clone())
                    },
                ));
            }
            if !is_level_badge {
                label(stat, art, caption, 12., Color::srgb(0.82, 0.88, 0.81));
            }
            label(
                stat,
                art,
                value,
                if is_level_badge { 20. } else { 18. },
                if is_score || is_enemy {
                    Color::srgb(1., 0.84, 0.33)
                } else {
                    Color::WHITE
                },
            );
            if is_armor {
                for _ in 0..shield_count {
                    label(stat, art, "◆", 13., Color::srgb(0.28, 0.89, 0.96));
                }
            }
        });
}

fn tank_wide_ui(commands: &mut Commands, g: &TankGame, art: &Art, width: f32, height: f32) {
    // The concept uses small ground tiles around larger, spaced gameplay objects.
    // Keep the 7 x 7 collision map, but draw it on a denser 15 x 8 field.
    let columns = 15_i8;
    let rows = 8_i8;
    let tile_gap = 0.;
    let cell_w = (width - (columns as f32 - 1.) * tile_gap) / columns as f32;
    let cell_h = (height - (rows as f32 - 1.) * tile_gap) / rows as f32;
    let step_w = cell_w + tile_gap;
    let step_h = cell_h + tile_gap;
    let control = (height * 0.15).clamp(96., 112.);
    let tank_side = cell_h * 1.85;
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Relative,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.30, 0.16)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(tile_gap),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.20, 0.36, 0.20)),
            ))
            .with_children(|arena| {
                for y in 0..rows {
                    arena
                        .spawn(Node {
                            width: Val::Percent(100.),
                            height: Val::Px(cell_h),
                            min_height: Val::Px(cell_h),
                            column_gap: Val::Px(tile_gap),
                            ..row()
                        })
                        .with_children(|row_node| {
                            for x in 0..columns {
                                let is_border_decor = x == 0 || x == columns - 1;
                                let mapped_y = |ly: i8| ly;
                                let mapped_x = |lx: i8| ((2.5 + lx as f32 * 1.45).round()) as i8;
                                let wall = !is_border_decor
                                    && g.walls
                                        .iter()
                                        .any(|&(lx, ly)| x == mapped_x(lx) && y == mapped_y(ly));
                                let steel = !is_border_decor
                                    && g.steel
                                        .iter()
                                        .any(|&(lx, ly)| x == mapped_x(lx) && y == mapped_y(ly));
                                let bush = !is_border_decor
                                    && g.bushes.iter().any(|&(lx, ly)| {
                                        let bx = match lx {
                                            0 => 2,
                                            1 => 3,
                                            5 => 11,
                                            6 => 12,
                                            _ => mapped_x(lx),
                                        };
                                        x == bx && y == mapped_y(ly)
                                    });
                                let obstacle_index = if wall {
                                    Some(2)
                                } else if steel {
                                    Some(3)
                                } else if bush {
                                    Some(4)
                                } else {
                                    None
                                };
                                let tile_index = if is_border_decor {
                                    if (x + y) % 2 == 0 { 5 } else { 4 }
                                } else if obstacle_index.is_none() && (x == 5 || x == 9) {
                                    1
                                } else {
                                    0
                                };
                                let mut tile_image = tank_concept_tile(
                                    art,
                                    tile_index,
                                    x as usize * 7 + y as usize * 11,
                                );
                                tile_image.image_mode = NodeImageMode::Stretch;
                                row_node
                                    .spawn((
                                        Node {
                                            width: Val::Px(cell_w),
                                            height: Val::Px(cell_h),
                                            min_width: Val::Px(cell_w),
                                            border: UiRect::all(Val::Px(1.)),
                                            ..default()
                                        },
                                        tile_image,
                                        BackgroundColor(Color::WHITE),
                                        BorderColor(Color::srgb(0.24, 0.37, 0.16)),
                                        BorderRadius::all(Val::Px(2.)),
                                    ))
                                    .with_children(|cell| {
                                        if let Some(obstacle_index) = obstacle_index {
                                            let scale = if bush { 1.0 } else { 0.94 };
                                            let mut obstacle_image = tank_concept_tile(
                                                art,
                                                obstacle_index,
                                                x as usize + y as usize,
                                            );
                                            obstacle_image.image_mode = NodeImageMode::Stretch;
                                            cell.spawn((
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    left: Val::Px(cell_w * (1. - scale) * 0.5),
                                                    top: Val::Px(cell_h * 0.04),
                                                    width: Val::Px(cell_w * scale),
                                                    height: Val::Px(cell_h * 0.92),
                                                    ..default()
                                                },
                                                obstacle_image,
                                            ));
                                        }
                                    });
                            }
                        });
                }
                let side_width = width * 210. / 1672.;
                for (left, right, right_side) in [(0., 210., false), (1462., 1672., true)] {
                    let mut side_image = ImageNode::new(art.tank_concept.clone());
                    side_image.image_mode = NodeImageMode::Stretch;
                    side_image.rect = Some(Rect::from_corners(
                        Vec2::new(left, 0.),
                        Vec2::new(right, 941.),
                    ));
                    arena.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: if right_side { Val::Auto } else { Val::Px(0.) },
                            right: if right_side { Val::Px(0.) } else { Val::Auto },
                            top: Val::Px(0.),
                            width: Val::Px(side_width),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        side_image,
                    ));
                }
                let spawn_tank = |parent: &mut ChildSpawnerCommands,
                                  x: f32,
                                  y: f32,
                                  friendly: bool,
                                  color: Color,
                                  direction: (i8, i8)| {
                    parent
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(x + (cell_w - tank_side) * 0.5),
                                top: Val::Px(y + (cell_h - tank_side) * 0.5),
                                width: Val::Px(tank_side),
                                height: Val::Px(tank_side),
                                ..default()
                            },
                            Transform::from_rotation(Quat::from_rotation_z(match direction {
                                (0, -1) => 0.,
                                (1, 0) => -std::f32::consts::FRAC_PI_2,
                                (0, 1) => std::f32::consts::PI,
                                (-1, 0) => std::f32::consts::FRAC_PI_2,
                                _ => 0.,
                            })),
                        ))
                        .insert(tank_sprite_image(art, friendly, color));
                };
                spawn_tank(
                    arena,
                    (2.5 + g.player.0 as f32 * 1.45) * step_w,
                    (0.6 + g.player.1 as f32 * 5.8 / 6.) * step_h,
                    true,
                    if g.invulnerable > 0. {
                        Color::srgb(0.88, 0.97, 1.)
                    } else {
                        Color::WHITE
                    },
                    g.direction,
                );
                for enemy in &g.enemies {
                    spawn_tank(
                        arena,
                        (2.5 + enemy.x as f32 * 1.45) * step_w,
                        (0.6 + enemy.y as f32 * 5.8 / 6.) * step_h,
                        false,
                        if enemy.warning > 0. {
                            Color::srgb(1., 0.90, 0.67)
                        } else {
                            Color::WHITE
                        },
                        enemy.direction,
                    );
                }
                for bullet in &g.bullets {
                    arena.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(((2.5 + bullet.x * 1.45) * step_w) - cell_h * 0.08),
                            top: Val::Px((0.6 + bullet.y * 5.8 / 6.) * step_h - cell_h * 0.08),
                            width: Val::Px(cell_h * 0.16),
                            height: Val::Px(cell_h * 0.16),
                            ..default()
                        },
                        BackgroundColor(if bullet.friendly {
                            Color::srgb(1., 0.82, 0.25)
                        } else {
                            Color::srgb(1., 0.43, 0.28)
                        }),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                }
                if g.failed {
                    arena
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.),
                                right: Val::Px(0.),
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(12.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.02, 0.04, 0.06, 0.88)),
                            ZIndex(5),
                        ))
                        .with_children(|panel| {
                            label(panel, art, "基地失守", 34., Color::srgb(1., 0.65, 0.30));
                            label(
                                panel,
                                art,
                                "可以重试当前关卡，护甲会补满",
                                18.,
                                Color::WHITE,
                            );
                            panel.spawn(row()).with_children(|actions| {
                                button(
                                    actions,
                                    art,
                                    "重试本关",
                                    Action::TankRetry,
                                    118.,
                                    46.,
                                    Color::srgb(0.30, 0.74, 0.60),
                                    NAVY,
                                );
                                button(
                                    actions,
                                    art,
                                    "返回游戏库",
                                    Action::Library,
                                    128.,
                                    46.,
                                    SLATE,
                                    Color::WHITE,
                                );
                            });
                        });
                }
            });

            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(0.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|hud| {
                tank_hud_stat(
                    hud,
                    art,
                    "★",
                    "战区",
                    format!("第 {} 关", g.wave),
                    Color::srgb(0.83, 0.57, 0.16),
                );
                hud.spawn((Node {
                    padding: UiRect::axes(Val::Px(8.), Val::Px(5.)),
                    column_gap: Val::Px(10.),
                    ..row()
                },))
                    .with_children(|stats| {
                        tank_hud_stat(
                            stats,
                            art,
                            "分",
                            "得分",
                            format!("{:05}", g.score),
                            Color::srgb(0.27, 0.59, 0.52),
                        );
                        tank_hud_stat(
                            stats,
                            art,
                            "盾",
                            "护甲",
                            g.lives.to_string(),
                            Color::srgb(0.24, 0.53, 0.72),
                        );
                    });
                hud.spawn(row()).with_children(|actions| {
                    button(
                        actions,
                        art,
                        "重新开始",
                        Action::TankReset,
                        100.,
                        45.,
                        Color::srgb(0.13, 0.31, 0.57),
                        Color::WHITE,
                    );
                    button(
                        actions,
                        art,
                        "返回游戏库",
                        Action::Library,
                        116.,
                        45.,
                        Color::srgb(0.06, 0.15, 0.16),
                        Color::WHITE,
                    );
                });
            });

            // The right border already contains the concept's enemy badge.
            // Cover only its sample number and draw the live count in its place.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(13.),
                    top: Val::Px(52.),
                    width: Val::Px(45.),
                    height: Val::Px(42.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.02, 0.09, 0.08)),
                BorderRadius::all(Val::Px(6.)),
            ))
            .with_children(|enemy_count| {
                label(
                    enemy_count,
                    art,
                    format!("{:02}", g.enemies.len() + g.reserves as usize),
                    18.,
                    Color::srgb(1., 0.84, 0.33),
                );
            });

            root.spawn((
                TankStick,
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(14.),
                    bottom: Val::Px(14.),
                    width: Val::Px(control),
                    height: Val::Px(control),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BorderRadius::all(Val::Percent(50.)),
                ImageNode {
                    rect: Some(Rect::from_corners(
                        Vec2::new(17., 798.),
                        Vec2::new(138., 925.),
                    )),
                    image_mode: NodeImageMode::Stretch,
                    ..ImageNode::new(art.tank_concept.clone())
                },
            ));
            root.spawn((
                TankFirePad,
                WorldInputBlocker,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(9.),
                    bottom: Val::Px(13.),
                    width: Val::Px(control),
                    height: Val::Px(control),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BorderRadius::all(Val::Percent(50.)),
                ImageNode {
                    rect: Some(Rect::from_corners(
                        Vec2::new(1540., 796.),
                        Vec2::new(1661., 922.),
                    )),
                    image_mode: NodeImageMode::Stretch,
                    ..ImageNode::new(art.tank_concept.clone())
                },
            ));
        });
}

// The library keeps its title and return button visible while the card collection
// scrolls in the space between them.  Wheel input works on desktop and vertical
// finger drags work on a phone.
fn library_scroll_input(
    mut wheels: EventReader<MouseWheel>,
    touches: Res<Touches>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    scale: Res<UiScale>,
    mut drag: ResMut<LibraryDrag>,
    mut s: ResMut<Session>,
    mut cards: Query<(&mut ScrollPosition, &ComputedNode, &GlobalTransform), With<LibraryCards>>,
    game_buttons: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
) {
    if !s.library_open {
        *drag = LibraryDrag::default();
        wheels.clear();
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((mut scroll, node, transform)) = cards.single_mut() else {
        return;
    };
    let max_scroll =
        ((node.content_size().y - node.size().y) * node.inverse_scale_factor()).max(0.);
    let inside = |point: Vec2| {
        let physical = point * window.scale_factor();
        let center = transform.translation().truncate();
        let half = node.size() / 2.;
        (physical.x - center.x).abs() <= half.x && (physical.y - center.y).abs() <= half.y
    };
    for wheel in wheels.read() {
        if window.cursor_position().is_some_and(inside) {
            let distance = match wheel.unit {
                MouseScrollUnit::Line => wheel.y * 42.,
                MouseScrollUnit::Pixel => wheel.y / scale.0,
            };
            scroll.offset_y = (scroll.offset_y - distance).clamp(0., max_scroll);
        }
    }
    if drag.start.is_none() {
        if let Some(touch) = touches.iter_just_pressed().next() {
            drag.begin(touch.position(), Some(touch.id()), inside(touch.position()));
        } else if mouse.just_pressed(MouseButton::Left) {
            if let Some(point) = window.cursor_position() {
                drag.begin(point, None, inside(point));
            }
        }
    }
    if drag.start.is_none() {
        return;
    }
    let (point, released) = if let Some(id) = drag.touch {
        (
            touches.get_pressed(id).map(|t| t.position()),
            touches.get_released(id).map(|t| t.position()),
        )
    } else {
        (
            if mouse.pressed(MouseButton::Left) {
                window.cursor_position()
            } else {
                None
            },
            if mouse.just_released(MouseButton::Left) {
                window.cursor_position()
            } else {
                None
            },
        )
    };
    if let Some(point) = point {
        scroll.offset_y = (scroll.offset_y + drag.move_to(point, scale.0)).clamp(0., max_scroll);
    } else {
        if let Some(point) = released {
            scroll.offset_y =
                (scroll.offset_y + drag.move_to(point, scale.0)).clamp(0., max_scroll);
        }
        let raw_tap = released
            .filter(|point| {
                drag.in_list
                    && drag
                        .start
                        .is_some_and(|start| point.distance(start) / scale.0.max(0.01) < 8.)
            })
            .and_then(|point| {
                let physical = point * window.scale_factor();
                game_buttons.iter().find_map(|(action, node, transform)| {
                    let Action::SelectGame(mode) = action else {
                        return None;
                    };
                    Rect::from_center_size(transform.translation().truncate(), node.size())
                        .contains(physical)
                        .then_some(*mode)
                })
            });
        if let Some(mode) = drag.finish(released, scale.0).or(raw_tap) {
            if mode == GameMode::Island {
                s.library_open = false;
                s.magnet_modes_open = true;
                s.ui_dirty = true;
            } else {
                select_game(&mut s, mode);
            }
        }
    }
}

fn magnet_modes_ui(commands: &mut Commands, _s: &Session, art: &Art, compact: bool) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(if compact { 13. } else { 18. }),
                padding: UiRect::all(Val::Px(if compact { 22. } else { 36. })),
                ..default()
            },
            BackgroundColor(Color::srgba(0.07, 0.15, 0.19, 0.97)),
        ))
        .with_children(|root| {
            label(
                root,
                art,
                "磁力方块探险家",
                if compact { 28. } else { 34. },
                Color::WHITE,
            );
            label(
                root,
                art,
                "选择一种玩法，所有内容都属于同一个方块世界",
                if compact { 14. } else { 17. },
                SOFT,
            );
            for (mode, description, color) in [
                (
                    GameMode::Island,
                    "小小建造师 · 六关编程闯关",
                    Color::srgb(0.28, 0.62, 0.59),
                ),
                (
                    GameMode::CloudBridge,
                    "云桥工程师 · 搭桥穿越云海",
                    Color::srgb(0.35, 0.53, 0.77),
                ),
                (
                    GameMode::CrystalGarden,
                    "小小动物餐厅 · 自由装修餐厅",
                    Color::srgb(0.59, 0.40, 0.70),
                ),
                (
                    GameMode::FreeBuild,
                    "自由搭建工坊 · 保存你的作品",
                    Color::srgb(0.80, 0.50, 0.24),
                ),
            ] {
                button(
                    root,
                    art,
                    description,
                    Action::SelectGame(mode),
                    if compact { 330. } else { 400. },
                    if compact { 54. } else { 60. },
                    color,
                    Color::WHITE,
                );
            }
            button(
                root,
                art,
                "← 返回游戏库",
                Action::MagnetModesClose,
                142.,
                42.,
                SLATE,
                Color::WHITE,
            );
        });
}

fn library_icon(parent: &mut ChildSpawnerCommands, art: &Art, mode: GameMode) {
    parent
        .spawn((
            Node {
                width: Val::Px(72.),
                height: Val::Px(72.),
                flex_shrink: 0.,
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(Color::srgba(1., 1., 1., 0.22)),
            BorderRadius::all(Val::Px(16.)),
        ))
        .with_children(|icon| {
            let block = |icon: &mut ChildSpawnerCommands,
                         x: f32,
                         y: f32,
                         w: f32,
                         h: f32,
                         radius: f32,
                         color: Color| {
                icon.spawn((
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
            };
            let white = Color::WHITE;
            let pale = Color::srgba(1., 1., 1., 0.65);
            match mode {
                GameMode::Island => {
                    block(icon, 12., 39., 25., 21., 3., white);
                    block(icon, 39., 39., 21., 21., 3., pale);
                    block(icon, 25., 16., 24., 21., 3., white);
                }
                GameMode::Tetris => {
                    for (x, y) in [(15., 36.), (29., 36.), (43., 36.), (43., 22.)] {
                        block(icon, x, y, 13., 13., 2., white);
                    }
                }
                GameMode::Breakout => {
                    for (x, y) in [(12., 17.), (28., 17.), (44., 17.), (20., 30.), (36., 30.)] {
                        block(icon, x, y, 14., 10., 3., pale);
                    }
                    block(icon, 19., 53., 34., 7., 4., white);
                    block(icon, 32., 42., 8., 8., 4., white);
                }
                GameMode::ElementLab => {
                    for (x, y) in [
                        (11., 12.),
                        (31., 12.),
                        (51., 12.),
                        (11., 32.),
                        (31., 32.),
                        (51., 32.),
                    ] {
                        block(
                            icon,
                            x,
                            y,
                            15.,
                            15.,
                            3.,
                            if x == 31. { pale } else { white },
                        );
                    }
                    block(icon, 31., 52., 15., 8., 3., pale);
                }
                GameMode::LightPath => {
                    let beam = Color::srgb(0.45, 0.90, 1.);
                    block(icon, 9., 34., 27., 4., 2., beam);
                    block(icon, 33., 22., 4., 17., 2., beam);
                    block(icon, 33., 22., 20., 4., 2., beam);
                    block(icon, 49., 15., 13., 13., 7., white);
                }
                GameMode::GardenDefense => {
                    icon.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(4.),
                            top: Val::Px(4.),
                            width: Val::Px(64.),
                            height: Val::Px(64.),
                            ..default()
                        },
                        ImageNode {
                            image: art.garden_plants.clone(),
                            rect: Some(Rect::new(44., 34., 273., 297.)),
                            ..default()
                        },
                    ));
                }
                GameMode::Ball2048 | GameMode::BubbleIsland | GameMode::BubbleBomber => {
                    block(icon, 10., 31., 24., 24., 12., white);
                    block(icon, 35., 14., 26., 26., 13., pale);
                    block(icon, 39., 43., 17., 17., 9., white);
                }
                GameMode::PixelAssault => {
                    block(icon, 15., 25., 29., 15., 3., white);
                    block(icon, 42., 28., 20., 7., 2., white);
                    block(icon, 25., 38., 10., 18., 2., pale);
                    block(icon, 10., 28., 8., 8., 2., pale);
                }
                GameMode::MushroomRun => {
                    block(icon, 13., 20., 46., 26., 19., white);
                    block(icon, 29., 40., 14., 20., 4., pale);
                    block(icon, 25., 25., 7., 7., 4., pale);
                    block(icon, 43., 30., 6., 6., 3., pale);
                }
                GameMode::TankBattle => {
                    block(icon, 14., 34., 44., 20., 5., white);
                    block(icon, 23., 24., 24., 13., 5., pale);
                    block(icon, 45., 28., 17., 5., 2., white);
                    for x in [22., 45.] {
                        block(icon, x, 50., 9., 9., 5., pale);
                    }
                }
                GameMode::SkyRaid => {
                    block(icon, 30., 13., 12., 47., 6., white);
                    block(icon, 12., 35., 48., 10., 4., pale);
                    block(icon, 22., 54., 28., 6., 3., white);
                }
                GameMode::Fishing => {
                    block(icon, 13., 25., 39., 27., 14., white);
                    block(icon, 48., 29., 14., 19., 3., pale);
                    block(icon, 22., 33., 5., 5., 3., Color::srgb(0.08, 0.50, 0.68));
                }
                GameMode::HoneyHunt => {
                    block(icon, 19., 28., 34., 32., 8., pale);
                    block(icon, 16., 21., 40., 10., 3., white);
                    block(icon, 25., 36., 22., 16., 7., Color::srgb(0.86, 0.52, 0.15));
                }
                GameMode::Snake => {
                    for (x, y) in [(14., 42.), (28., 42.), (42., 42.), (42., 28.), (42., 14.)] {
                        block(icon, x, y, 16., 16., 8., white);
                    }
                    block(icon, 51., 20., 4., 4., 2., Color::srgb(0.28, 0.68, 0.34));
                }
                GameMode::Minesweeper => {
                    block(icon, 18., 18., 36., 36., 18., white);
                    block(icon, 33., 9., 6., 12., 2., pale);
                    block(icon, 24., 26., 8., 8., 4., Color::srgb(0.38, 0.44, 0.54));
                }
                GameMode::Detective => {
                    icon.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(13.),
                            top: Val::Px(12.),
                            width: Val::Px(32.),
                            height: Val::Px(32.),
                            border: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        BorderColor(white),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                    block(icon, 42., 42., 20., 8., 4., white);
                }
                GameMode::SpaceFarm => {
                    block(icon, 16., 16., 40., 40., 20., white);
                    block(icon, 28., 27., 16., 16., 8., Color::srgb(0.18, 0.56, 0.70));
                    block(icon, 32., 9., 8., 10., 4., pale);
                }
                GameMode::MusicFactory => {
                    for x in [12., 24., 36., 48.] {
                        block(icon, x, 16., 11., 43., 2., white);
                    }
                    for x in [20., 32., 44.] {
                        block(icon, x, 16., 7., 26., 2., Color::srgb(0.44, 0.24, 0.12));
                    }
                }
                GameMode::FruitNinja => {
                    block(icon, 15., 24., 42., 35., 18., white);
                    block(icon, 35., 15., 17., 9., 8., pale);
                    block(icon, 27., 31., 8., 8., 4., Color::srgb(0.84, 0.31, 0.24));
                }
                GameMode::Fortune => {
                    block(icon, 14., 14., 44., 44., 8., white);
                    for (x, y) in [(23., 23.), (42., 23.), (32., 32.), (23., 42.), (42., 42.)] {
                        block(icon, x, y, 7., 7., 4., Color::srgb(0.18, 0.55, 0.44));
                    }
                }
                GameMode::Slots | GameMode::FruitArcade => {
                    for x in [10., 29., 48.] {
                        block(icon, x, 20., 15., 33., 3., white);
                        block(icon, x + 4., 32., 7., 8., 4., Color::srgb(0.55, 0.23, 0.42));
                    }
                }
                GameMode::MoonFestival => {
                    block(icon, 15., 16., 42., 42., 21., white);
                    block(icon, 28., 13., 36., 41., 21., Color::srgb(0.78, 0.16, 0.18));
                }
                GameMode::Sudoku => {
                    for (col, x) in [15., 29., 43.].into_iter().enumerate() {
                        for (row, y) in [15., 29., 43.].into_iter().enumerate() {
                            block(
                                icon,
                                x,
                                y,
                                13.,
                                13.,
                                1.,
                                if (col + row) % 2 == 0 { white } else { pale },
                            );
                        }
                    }
                }
                GameMode::Classic2048 | GameMode::Klotski | GameMode::Sokoban => {
                    for (x, y) in [(13., 13.), (37., 13.), (13., 37.), (37., 37.)] {
                        block(icon, x, y, 21., 21., 4., white);
                    }
                    if mode == GameMode::Sokoban {
                        block(
                            icon,
                            22.,
                            22.,
                            28.,
                            28.,
                            4.,
                            Color::srgba(0.28, 0.20, 0.12, 0.85),
                        );
                    }
                }
                _ => {
                    let symbol = match mode {
                        GameMode::Math => "+",
                        GameMode::Literacy => "字",
                        GameMode::Firefighters => "火",
                        _ => "游",
                    };
                    icon.spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|center| label(center, art, symbol, 31., Color::WHITE));
                }
            }
        });
}

fn library_ui(commands: &mut Commands, _s: &Session, art: &Art, compact: bool) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                padding: UiRect::all(Val::Px(28.)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.07, 0.15, 0.19, 0.96)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|p| {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.),
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "游戏库", 32., Color::WHITE);
                    label(p, art, "选择一个小小世界，马上开始玩", 15., SOFT);
                });
                button(
                    p,
                    art,
                    "返回当前游戏",
                    Action::LibraryReturn,
                    150.,
                    44.,
                    SLATE,
                    Color::WHITE,
                );
            });
            let cards = Node {
                width: Val::Percent(100.),
                flex_grow: 1.,
                flex_shrink: 1.,
                min_height: Val::Px(0.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::FlexStart,
                column_gap: Val::Px(16.),
                row_gap: Val::Px(16.),
                flex_wrap: FlexWrap::Wrap,
                overflow: Overflow::scroll_y(),
                ..default()
            };
            root.spawn((cards, ScrollPosition::default(), LibraryCards))
                .with_children(|p| {
                    for (mode, color) in [
                        (GameMode::GardenDefense, Color::srgb(0.34, 0.57, 0.20)),
                        (GameMode::Island, Color::srgb(0.28, 0.62, 0.59)),
                        (GameMode::Ball2048, Color::srgb(0.89, 0.47, 0.25)),
                        (GameMode::Breakout, Color::srgb(0.18, 0.57, 0.72)),
                        (GameMode::ElementLab, Color::srgb(0.22, 0.54, 0.57)),
                        (GameMode::LightPath, Color::srgb(0.16, 0.50, 0.72)),
                        (GameMode::Tetris, Color::srgb(0.34, 0.38, 0.82)),
                        (GameMode::Math, Color::srgb(0.32, 0.38, 0.78)),
                        (GameMode::Literacy, Color::srgb(0.23, 0.63, 0.39)),
                        (GameMode::Firefighters, Color::srgb(0.88, 0.30, 0.22)),
                        (GameMode::Detective, Color::srgb(0.42, 0.30, 0.66)),
                        (GameMode::SpaceFarm, Color::srgb(0.18, 0.56, 0.70)),
                        (GameMode::MusicFactory, Color::srgb(0.90, 0.48, 0.18)),
                        (GameMode::PixelAssault, Color::srgb(0.28, 0.70, 0.58)),
                        (GameMode::MushroomRun, Color::srgb(0.90, 0.37, 0.24)),
                        (GameMode::TankBattle, Color::srgb(0.30, 0.55, 0.78)),
                        (GameMode::SkyRaid, Color::srgb(0.28, 0.45, 0.78)),
                        (GameMode::BubbleIsland, Color::srgb(0.78, 0.38, 0.62)),
                        (GameMode::BubbleBomber, Color::srgb(0.18, 0.54, 0.82)),
                        (GameMode::FruitNinja, Color::srgb(0.84, 0.31, 0.24)),
                        (GameMode::Fortune, Color::srgb(0.18, 0.55, 0.44)),
                        (GameMode::Slots, Color::srgb(0.45, 0.29, 0.64)),
                        (GameMode::FruitArcade, Color::srgb(0.75, 0.20, 0.28)),
                        (GameMode::Fishing, Color::srgb(0.08, 0.50, 0.68)),
                        (GameMode::HoneyHunt, Color::srgb(0.86, 0.52, 0.15)),
                        (GameMode::MoonFestival, Color::srgb(0.78, 0.16, 0.18)),
                        (GameMode::SpaceCourier, Color::srgb(0.21, 0.56, 0.86)),
                        (GameMode::Snake, Color::srgb(0.28, 0.68, 0.34)),
                        (GameMode::Minesweeper, Color::srgb(0.38, 0.44, 0.54)),
                        (GameMode::Classic2048, Color::srgb(0.82, 0.51, 0.22)),
                        (GameMode::Klotski, Color::srgb(0.66, 0.34, 0.20)),
                        (GameMode::Sokoban, Color::srgb(0.48, 0.34, 0.22)),
                        (GameMode::Sudoku, Color::srgb(0.23, 0.44, 0.68)),
                    ] {
                        let (title, subtitle) = if mode == GameMode::Island {
                            ("磁力方块探险家", "闯关 · 云桥 · 餐厅 · 自由搭建")
                        } else {
                            (mode.title(), mode.subtitle())
                        };
                        p.spawn((
                            Button,
                            ButtonTone(color),
                            Action::SelectGame(mode),
                            Node {
                                width: Val::Px(if compact { 278. } else { 320. }),
                                height: Val::Px(142.),
                                min_height: Val::Px(142.),
                                flex_shrink: 0.,
                                padding: UiRect::all(Val::Px(14.)),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(12.),
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderColor(EDGE),
                            BorderRadius::all(Val::Px(16.)),
                        ))
                        .with_children(|card| {
                            library_icon(card, art, mode);
                            card.spawn(Node {
                                flex_grow: 1.,
                                flex_shrink: 1.,
                                min_width: Val::Px(0.),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(6.),
                                ..default()
                            })
                            .with_children(|info| {
                                info.spawn((
                                    Node {
                                        max_width: Val::Percent(100.),
                                        ..default()
                                    },
                                    Text::new(title),
                                    TextFont {
                                        font: art.font.clone(),
                                        font_size: 20.,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                                info.spawn((
                                    Node {
                                        max_width: Val::Percent(100.),
                                        ..default()
                                    },
                                    Text::new(subtitle),
                                    TextFont {
                                        font: art.font.clone(),
                                        font_size: 12.,
                                        ..default()
                                    },
                                    TextColor(Color::srgba(1., 1., 1., 0.86)),
                                ));
                            });
                        });
                    }
                });
            label(
                root,
                art,
                "磁力方块探险家含四种玩法 · 其他小游戏点卡片直接开始",
                14.,
                SOFT,
            );
        });
}

// The world fills the phone; controls float above it in a collapsible bottom dock.
fn mobile_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let (frame, frame_edge, accent, accent_text, panel_color, ink, soft_text) = match s.mode {
        GameMode::CloudBridge => (
            Color::srgb(0.035, 0.10, 0.22),
            Color::srgb(0.10, 0.72, 0.91),
            Color::srgb(0.18, 0.82, 0.94),
            Color::srgb(0.02, 0.12, 0.19),
            Color::srgb(0.10, 0.22, 0.35),
            Color::srgb(0.89, 0.96, 1.0),
            Color::srgb(0.70, 0.84, 0.93),
        ),
        GameMode::CrystalGarden => (
            Color::srgb(0.34, 0.22, 0.15),
            Color::srgb(0.67, 0.42, 0.25),
            Color::srgb(0.78, 0.39, 0.25),
            Color::WHITE,
            Color::srgb(0.92, 0.87, 0.73),
            Color::srgb(0.25, 0.20, 0.15),
            Color::srgb(0.48, 0.42, 0.34),
        ),
        GameMode::FreeBuild => (
            Color::srgb(0.88, 0.89, 0.87),
            Color::srgb(0.69, 0.73, 0.72),
            Color::srgb(0.18, 0.50, 0.49),
            Color::WHITE,
            Color::srgb(0.91, 0.93, 0.91),
            Color::srgb(0.18, 0.22, 0.22),
            Color::srgb(0.48, 0.54, 0.52),
        ),
        _ => (
            Color::srgb(0.14, 0.40, 0.65),
            Color::srgb(0.30, 0.68, 0.87),
            Color::srgb(0.97, 0.72, 0.23),
            Color::srgb(0.22, 0.29, 0.22),
            Color::srgb(0.84, 0.94, 0.76),
            Color::srgb(0.14, 0.27, 0.24),
            Color::srgb(0.40, 0.53, 0.45),
        ),
    };
    commands.spawn((UiRoot, Node {
        width: Val::Percent(100.), height: Val::Percent(100.), ..default()
    })).with_children(|root| {
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                right: Val::Px(10.),
                top: Val::Px(8.),
                height: Val::Px(58.),
                ..default()
            },
            BackgroundColor(frame),
            BorderColor(frame_edge),
            BorderRadius::all(Val::Px(18.)),
        ));
        root.spawn((WorldInputBlocker, Node {
            position_type: PositionType::Absolute,
            left: Val::Px(24.), right: Val::Px(24.), top: Val::Px(18.),
            height: Val::Px(44.), justify_content: JustifyContent::SpaceBetween,
            ..row()
        },)).with_children(|p| {
            if s.mode == GameMode::FreeBuild {
                p.spawn(Node { flex_direction: FlexDirection::Column, row_gap: Val::Px(2.), ..default() }).with_children(|p| {
                    label(p, art, s.mode.title(), 17., ink);
                    label(p, art, "想搭什么就搭什么", 11., soft_text);
                });
            } else {
                p.spawn(row()).with_children(|p| {
                    button(p, art, "←", Action::Previous, 44., 44., panel_color, ink);
                    p.spawn(Node { flex_direction: FlexDirection::Column, row_gap: Val::Px(2.), ..default() }).with_children(|p| {
                        label(p, art, s.mode.title(), 11., soft_text);
                        label(p, art, game::LEVEL_NAMES[s.work.level], 19., ink);
                    });
                    button(p, art, "→", Action::Next, 44., 44., panel_color, ink);
                });
            }
            p.spawn(row()).with_children(|p| {
                if matches!(s.mode, GameMode::Island | GameMode::CloudBridge | GameMode::CrystalGarden | GameMode::FreeBuild) {
                    button(p, art, "玩法", Action::MagnetModes, 56., 44., panel_color, accent);
                }
                button(p, art, if s.save.active == 0 { "小芽" } else { "小树" }, Action::Profile(1-s.save.active), 64., 44., panel_color, accent);
                button(p, art, "保存", Action::Save, 64., 44., panel_color, ink);
                button(p, art, "游戏库", Action::Library, 76., 44., panel_color, accent);
                button(p, art, "转向", Action::Rotate, 64., 44., panel_color, ink);
                button(p, art, "放大", Action::ZoomIn, 62., 44., panel_color, ink);
                label(p, art, format!("{}%", (1600. / s.zoom).round() as i32), 13., accent);
                button(p, art, "缩小", Action::ZoomOut, 62., 44., panel_color, ink);
            });
        });
        root.spawn((WorldInputBlocker, Node {
            position_type: PositionType::Absolute, left: Val::Px(24.), top: Val::Px(70.),
            max_width: Val::Percent(if s.build { 66. } else { 92. }), padding: UiRect::axes(Val::Px(14.), Val::Px(8.)),
            ..row()
        }, BackgroundColor(panel_color), BorderColor(frame_edge), BorderRadius::all(Val::Px(12.))))
        .with_children(|p| {
            label(p, art, if s.mode == GameMode::CrystalGarden { "布置" } else { "目标" }, 13., accent);
            label(p, art, &s.message, 14., ink);
        });
        let mut dock = if s.build {
            panel(Val::Auto, Val::Px(18.), Val::Px(92.), Val::Auto, Val::Px(236.))
        } else {
            panel(Val::Px(24.), Val::Px(24.), Val::Auto, Val::Px(16.), Val::Auto)
        };
        dock.padding = UiRect::all(Val::Px(12.));
        dock.border = UiRect::ZERO;
        dock.row_gap = Val::Px(10.);
        if s.build {
            dock.flex_direction = FlexDirection::Column;
            dock.max_height = Val::Px(430.);
        }
        root.spawn((WorldInputBlocker, dock, BackgroundColor(frame),
            BorderColor(frame_edge), BorderRadius::all(Val::Px(20.))))
        .with_children(|p| {
            if s.build {
                p.spawn(if s.build { Node { flex_wrap: FlexWrap::Wrap, row_gap: Val::Px(7.), column_gap: Val::Px(7.), ..row() } } else { row() }).with_children(|p| {
                    for m in [Material::Grass, Material::Wood, Material::Stone, Material::Crystal, Material::Brick, Material::Sand, Material::Snow, Material::Glass, Material::Lamp, Material::Leaf] {
                        button(p, art, m.label(), Action::Material(m), 100., 44.,
                            if s.material == m && !s.erase { accent } else { panel_color },
                            if s.material == m && !s.erase { accent_text } else { ink });
                    }
                    button(p, art, "拆除", Action::Erase, 80., 44., if s.erase { Color::srgb(1.,0.78,0.70) } else { panel_color }, if s.erase { ink } else { soft_text });
                    button(p, art, "撤销", Action::Undo, 80., 44., panel_color, soft_text);
                });
            } else if s.editor_open {
                p.spawn(row()).with_children(|p| {
                    button(p, art, format!("重复 {} 次", s.work.repeat), Action::Repeat, 120., 44., Color::srgb(0.98,0.81,0.44), INK);
                    button(p, art, format!("结束前进：{}", if s.work.finish_forward { "开" } else { "关" }), Action::Finish, 170., 44., SLATE, Color::WHITE);
                    button(p, art, "清空", Action::Clear, 70., 44., SLATE, SOFT);
                    button(p, art, "示范路线", Action::GuideRoute, 100., 44., MINT, NAVY);
                });
                p.spawn(Node { height: Val::Px(48.), ..row() }).with_children(|p| {
                    if s.work.program.is_empty() { label(p, art, "点下方指令，告诉机器人怎么走", 17., SOFT); }
                    let active = s.runner.as_ref().map(|r| if r.faulted { r.pc } else { r.pc.saturating_sub(1) } % s.work.program.len().max(1));
                    for (i, op) in s.work.program.iter().enumerate() {
                        button(p, art, format!("{} {}", i+1, op.label()), Action::Remove(i), 101., 44.,
                            if active == Some(i) { Color::srgb(1.,0.80,0.40) } else { Color::srgb(0.79,0.88,0.77) }, INK);
                    }
                });
                p.spawn(row()).with_children(|p| {
                    for op in [Instruction::Place, Instruction::Forward, Instruction::Left, Instruction::Right, Instruction::Back, Instruction::Jump, Instruction::Wait] {
                        button(p, art, format!("＋{}",op.label()), Action::Add(op), 136., 44., Color::srgb(0.85,0.91,0.95), INK);
                    }
                });

            }
            if s.build {
                p.spawn(Node { flex_wrap: FlexWrap::Wrap, row_gap: Val::Px(7.), ..row() }).with_children(|p| {
                    button(p, art, "视角回正", Action::ResetView, 100., 44., panel_color, ink);
                    if s.mode != GameMode::FreeBuild {
                        button(p, art, if s.mode == GameMode::CrystalGarden { "继续布置" } else { "编程 →" }, Action::Code, 100., 44., accent, accent_text);
                    }
                });
                label(p, art,
                    if s.erase { "拆除中 · 点材料恢复搭建" }
                    else if s.mode == GameMode::FreeBuild { "自由创作 · 作品自动保存" }
                    else if s.mode == GameMode::CrystalGarden { "轻点布置餐厅 · 拖动转视角" }
                    else { "轻点搭建 · 拖动转视角" },
                    13., soft_text);
                return;
            }
            p.spawn(Node { justify_content: JustifyContent::SpaceBetween, ..row() }).with_children(|p| {
                p.spawn(row()).with_children(|p| {
                    button(p, art, "搭建", Action::Build, 80., 44., if s.build { accent } else { panel_color }, if s.build { accent_text } else { soft_text });
                    button(p, art, if s.editor_open && !s.build { "收起 ↓" } else { "编程 ↑" },
                        if s.build { Action::Code } else { Action::ToggleEditor }, 100., 44., if !s.build && s.editor_open { accent } else { panel_color }, if !s.build && s.editor_open { accent_text } else { soft_text });
                });
                if !s.build && !s.editor_open {
                    label(p, art, if s.mode == GameMode::CloudBridge { "拖动查看桥梁 · 滚轮缩放" } else { "拖动环视小岛 · 滚轮缩放" }, 13., soft_text);
                }
                p.spawn(row()).with_children(|p| {
                    button(p, art, "复位", Action::Reset, 72., 44., panel_color, soft_text);
                    button(p, art, "单步", Action::Step, 72., 44., panel_color, soft_text);
                    button(p, art, if s.runner.as_ref().is_some_and(|r| r.running) { "Ⅱ 暂停" } else { "▶ 运行" }, Action::Run, 112., 44., accent, accent_text);
                });
            });
        });
    });
}

#[cfg(test)]
mod library_gesture_tests {
    use super::*;
    #[test]
    fn card_swipe_scrolls_and_cannot_open_game() {
        for touch in [None, Some(7)] {
            let mut g = LibraryDrag::default();
            g.begin(Vec2::new(50., 200.), touch, true);
            g.pending_game = Some(GameMode::Slots);
            assert_eq!(g.move_to(Vec2::new(50., 160.), 2.), 20.);
            assert_eq!(g.move_to(Vec2::new(50., 100.), 2.), 30.);
            assert!(g.finish(Some(Vec2::new(50., 100.)), 2.).is_none());
        }
    }
    #[test]
    fn tap_jitter_does_not_scroll_and_selects_on_release() {
        let mut g = LibraryDrag::default();
        g.begin(Vec2::new(50., 100.), Some(1), true);
        g.pending_game = Some(GameMode::Slots);
        assert_eq!(g.move_to(Vec2::new(52., 102.), 1.), 0.);
        assert!(matches!(
            g.finish(Some(Vec2::new(52., 102.)), 1.),
            Some(GameMode::Slots)
        ));
    }
    #[test]
    fn canceled_or_release_only_swipe_never_selects() {
        for release in [
            None,
            Some(Vec2::new(100., 200.)),
            Some(Vec2::new(200., 100.)),
        ] {
            let mut g = LibraryDrag::default();
            g.begin(Vec2::new(100., 100.), None, true);
            g.pending_game = Some(GameMode::Slots);
            assert!(g.finish(release, 1.).is_none());
        }
        let mut g = LibraryDrag::default();
        g.begin(Vec2::ZERO, None, false);
        assert_eq!(g.move_to(Vec2::new(0., 100.), 1.), 0.);
    }
}

// Library/help state is considered separately: help stays with its game, but
// the library always returns to landscape even when a portrait game is selected.
fn game_wants_portrait(mode: GameMode, library_open: bool) -> bool {
    matches!(
        mode,
        GameMode::SkyRaid
            | GameMode::Ball2048
            | GameMode::Tetris
            | GameMode::MoonFestival
            | GameMode::Snake
            | GameMode::Classic2048
            | GameMode::Klotski
            | GameMode::Sudoku
            | GameMode::LightPath
    ) && !library_open
}
fn sync_game_orientation(
    s: Res<Session>,
    mut applied: Local<Option<(bool, bool)>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let portrait = game_wants_portrait(s.mode, s.library_open);
    let game_wide = matches!(
        s.mode,
        GameMode::TankBattle | GameMode::Firefighters | GameMode::Detective | GameMode::SpaceFarm
    ) && !s.library_open;
    if *applied == Some((portrait, game_wide)) {
        return;
    }
    #[cfg(target_os = "android")]
    {
        let Some(app) = bevy::window::ANDROID_APP.get() else {
            return;
        };
        // AndroidApp owns the VM and Activity global reference for this call.
        let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr().cast()) };
        let result: jni::errors::Result<()> = vm.attach_current_thread(|env| {
            let activity =
                unsafe { jni::objects::JObject::from_raw(env, app.activity_as_ptr().cast()) };
            env.call_method(
                &activity,
                jni::jni_str!("setRequestedOrientation"),
                jni::jni_sig!("(I)V"),
                &[jni::objects::JValue::Int(if portrait { 1 } else { 0 })],
            )?;
            Ok(())
        });
        if let Err(error) = result {
            eprintln!("Cannot update screen orientation: {error}");
            return;
        }
    }
    // Desktop follows the same transition. This makes the 2048 course read as
    // a long forward track during normal development too, not just on Android.
    if cfg!(not(target_os = "android")) {
        if let Ok(mut window) = windows.single_mut() {
            let phone_preview = std::env::var_os("MAGNET_PHONE_PREVIEW").is_some();
            let (width, height) = if portrait {
                if phone_preview {
                    (540., 960.)
                } else {
                    (560., 900.)
                }
            } else if phone_preview {
                (960., 540.)
            } else if game_wide {
                (1280., 720.)
            } else {
                (1280., 800.)
            };
            window.resolution.set(width, height);
        }
    }
    *applied = Some((portrait, game_wide));
}
#[cfg(test)]
mod orientation_tests {
    use super::*;
    #[test]
    fn portrait_games_request_portrait_and_library_restores_landscape() {
        assert!(game_wants_portrait(GameMode::SkyRaid, false));
        assert!(game_wants_portrait(GameMode::Ball2048, false));
        assert!(game_wants_portrait(GameMode::Tetris, false));
        assert!(game_wants_portrait(GameMode::MoonFestival, false));
        assert!(!game_wants_portrait(GameMode::GardenDefense, false));
        assert!(!game_wants_portrait(GameMode::Sokoban, false));
        assert!(!game_wants_portrait(GameMode::Minesweeper, false));
        assert!(!game_wants_portrait(GameMode::GardenDefense, true));
        for mode in [
            GameMode::Island,
            GameMode::CloudBridge,
            GameMode::CrystalGarden,
            GameMode::FreeBuild,
            GameMode::Ball2048,
            GameMode::Tetris,
            GameMode::Math,
            GameMode::Literacy,
            GameMode::Firefighters,
            GameMode::Detective,
            GameMode::SpaceFarm,
            GameMode::MusicFactory,
            GameMode::PixelAssault,
            GameMode::MushroomRun,
            GameMode::TankBattle,
            GameMode::SkyRaid,
            GameMode::BubbleIsland,
            GameMode::BubbleBomber,
            GameMode::FruitNinja,
            GameMode::Fortune,
            GameMode::Slots,
            GameMode::FruitArcade,
            GameMode::Fishing,
            GameMode::HoneyHunt,
            GameMode::MoonFestival,
            GameMode::GardenDefense,
        ] {
            assert!(!game_wants_portrait(mode, true));
            if !matches!(
                mode,
                GameMode::SkyRaid | GameMode::Ball2048 | GameMode::Tetris | GameMode::MoonFestival
            ) {
                assert!(!game_wants_portrait(mode, false));
            }
        }
    }
}

#[cfg(test)]
mod tank_stick_tests {
    use super::*;
    #[test]
    fn dead_zone_and_four_directions() {
        assert_eq!(tank_stick_direction(Vec2::new(5., 8.), 90.), None);
        for (point, expected) in [
            (Vec2::new(70., 20.), (1, 0)),
            (Vec2::new(-70., 20.), (-1, 0)),
            (Vec2::new(20., -70.), (0, -1)),
            (Vec2::new(20., 70.), (0, 1)),
        ] {
            assert_eq!(tank_stick_direction(point, 90.), Some(expected));
            assert_eq!(tank_stick_direction(point * 3., 90.), Some(expected));
        }
    }
}

#[cfg(test)]
mod bomber_joystick_tests {
    use super::*;

    #[test]
    fn joystick_has_dead_zone_and_uses_dominant_axis() {
        assert_eq!(bomber_joystick_direction(Vec2::ZERO, 56.), None);
        assert_eq!(bomber_joystick_direction(Vec2::new(5., -5.), 56.), None);
        assert_eq!(
            bomber_joystick_direction(Vec2::new(45., 20.), 56.),
            Some((1, 0))
        );
        assert_eq!(
            bomber_joystick_direction(Vec2::new(-45., 20.), 56.),
            Some((-1, 0))
        );
        assert_eq!(
            bomber_joystick_direction(Vec2::new(20., -45.), 56.),
            Some((0, -1))
        );
        assert_eq!(
            bomber_joystick_direction(Vec2::new(20., 45.), 56.),
            Some((0, 1))
        );
        // A captured finger may leave the ring without losing its direction.
        assert_eq!(
            bomber_joystick_direction(Vec2::new(500., 20.), 56.),
            Some((1, 0))
        );
    }
}

#[cfg(test)]
mod bomber_multitouch_tests {
    use super::*;
    use bevy::input::{
        InputPlugin,
        touch::{TouchInput, TouchPhase},
    };

    #[test]
    fn captured_joystick_survives_second_finger_and_stops_on_release() {
        let mut app = App::new();
        app.add_plugins(InputPlugin)
            .init_resource::<Time>()
            .init_resource::<BomberHold>()
            .add_systems(Update, bomber_joystick_input);
        let mut session = Session::new();
        session.mode = GameMode::BubbleBomber;
        session.library_open = false;
        session.help_open = false;
        session.bomber = BomberGame::new();
        session.bomber.bricks.clear();
        session.bomber.bushes.clear();
        session.bomber.pots.clear();
        app.insert_resource(session);
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();
        app.world_mut().spawn((
            BomberJoystick,
            ComputedNode {
                size: Vec2::splat(100.),
                ..default()
            },
            GlobalTransform::from_translation(Vec3::new(100., 100., 0.)),
        ));
        let knob = app
            .world_mut()
            .spawn((BomberJoystickKnob, Node::default()))
            .id();
        app.world_mut().spawn((
            BomberTouchPlace,
            ComputedNode {
                size: Vec2::splat(100.),
                ..default()
            },
            GlobalTransform::from_translation(Vec3::new(800., 100., 0.)),
        ));
        let send = |app: &mut App, id, phase, position| {
            app.world_mut().send_event(TouchInput {
                window,
                id,
                phase,
                position,
                force: None,
            });
        };
        send(&mut app, 11, TouchPhase::Started, Vec2::new(100., 70.));
        app.update();
        assert_eq!(app.world().resource::<Session>().bomber.player, (2, 7));
        assert_eq!(app.world().resource::<BomberHold>().touch_id, Some(11));
        send(&mut app, 22, TouchPhase::Started, Vec2::new(800., 100.));
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_millis(170));
        app.update();
        let session = app.world().resource::<Session>();
        assert_eq!(session.bomber.player, (2, 6));
        assert_eq!(session.bomber.bombs.len(), 1);
        assert_eq!(
            (session.bomber.bombs[0].x, session.bomber.bombs[0].y),
            (2, 6)
        );
        assert_eq!(app.world().resource::<BomberHold>().touch_id, Some(11));
        send(&mut app, 11, TouchPhase::Ended, Vec2::new(100., 70.));
        app.update();
        assert_eq!(app.world().resource::<Session>().bomber.player, (2, 6));
        assert_eq!(app.world().resource::<BomberHold>().direction, None);
        assert_eq!(
            app.world().get::<Node>(knob).unwrap().left,
            Val::Percent(27.)
        );
    }
}
