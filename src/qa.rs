//! Opt-in desktop verification. Compiled only in debug builds, with isolated saves.
//! Pointer input goes through the real focus, hit testing and game input systems.
use super::*;
use std::collections::VecDeque;

pub(super) fn enabled() -> bool {
    std::env::var_os("MAGNET_QA").is_some() && std::env::var_os("MAGNET_CAPTURE").is_some()
}
pub(super) struct QaPlugin;
impl Plugin for QaPlugin {
    fn build(&self, app: &mut App) {
        if !enabled() {
            return;
        }
        assert!(
            std::env::var_os("MAGNET_SAVE_DIR").is_some(),
            "QA requires an isolated save directory"
        );
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(if std::env::var("MAGNET_START_GAME").as_deref()==Ok("train") {1. / 10.} else if std::env::var_os("MAGNET_RABBIT_LONG_QA").is_some() {1. / 20.} else {1. / 30.}),
        ))
        .init_resource::<Qa>()
        .add_systems(
            Update,
            resize.after(sync_game_orientation).before(layout_camera),
        )
        .add_systems(
            PreUpdate,
            input
                .after(bevy::input::InputSystem)
                .before(bevy::ui::UiSystem::Focus),
        )
        .add_systems(Update, observe.after(capture_preview));
    }
}
fn resize(s: Res<Session>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(size) = std::env::var("MAGNET_QA_SIZE") {
        if let Some((mut w, mut h)) = size
            .split_once('x')
            .and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?)))
        {
            // A portrait game still returns to the landscape library on device.
            if s.library_open && h > w {
                std::mem::swap(&mut w, &mut h);
            }
            if let Ok(mut window) = windows.single_mut() {
                if (window.width() - w).abs() > 0.5 || (window.height() - h).abs() > 0.5 {
                    window.resolution.set(w, h);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Area {
    Window,
    Forest,
    Bomber,
    BomberFire,
    Tank,
    Fire,
    Bubble,
    Garden,
}
#[derive(Clone)]
enum QaStep {
    WindowSize(f32,f32),
    VisualFit,
    Click(Action),
    Hold(Action,u32),
    Optional(Action),
    Point(Area, Vec2, Vec2, u32),
    Keys(Vec<KeyCode>, u32),
    RabbitRunUntil(f32),
    BuildCell,
    BuilderPlace([i32;3]),
    CloudPlace(usize),
    FruitSlice,
    Wait(u32),
    Check(&'static str, fn(&Session) -> bool),
    ForestCheck(&'static str, fn(&ForestTrain) -> bool),
    Shot(&'static str),
    Help,
    Library,
    SelectFromLibrary,
}
#[derive(Resource, Default)]
struct Qa {
    initialized: bool,
    frame: u32,
    remaining: u32,
    steps: VecDeque<QaStep>,
    active: Option<QaStep>,
    point: Option<Vec2>,
    mode: Option<GameMode>,
    records: Vec<serde_json::Value>,
    root: PathBuf,
    failed: bool,
    exit_frame: Option<u32>,
    select_attempts: u32,
}
fn click(steps: &mut VecDeque<QaStep>, a: Action) {
    steps.push_back(QaStep::Click(a));
}
fn check(steps: &mut VecDeque<QaStep>, name: &'static str, f: fn(&Session) -> bool) {
    steps.push_back(QaStep::Check(name, f));
}
fn scenario(s: &Session) -> VecDeque<QaStep> {
    use Action::*;
    if s.mode==GameMode::ForestTrain{return forest_scenario();}
    if s.mode==GameMode::CloudBridge{return cloud_walk_scenario(s);}
    if builder_mode(s) {return puzzle_scenario(s);}
    let mut p = VecDeque::from([QaStep::Wait(35), QaStep::Shot("initial")]);
    if matches!(s.mode,GameMode::CloudBridge|GameMode::FruitArcade|GameMode::SpaceCourier|GameMode::BubbleBomber|GameMode::Snake) {p.push_back(QaStep::VisualFit);}
    match s.mode {
        GameMode::ForestTrain => unreachable!(),
        GameMode::GoldRunner => {
            p.push_back(QaStep::Keys(vec![KeyCode::KeyX],1));
            check(&mut p,"向右挖坑",|s|s.gold_runner.qa_holes()==1);
            p.push_back(QaStep::Shot("digging"));
            p.push_back(QaStep::Keys(vec![KeyCode::ArrowRight],16));
            p.push_back(QaStep::Wait(24));
            check(&mut p,"进入挖开的坑后自然下落",|s|s.gold_runner.qa_position().y>740.);
            p.push_back(QaStep::Keys(vec![KeyCode::KeyR],1));
            p.push_back(QaStep::Keys(vec![KeyCode::ArrowLeft],21));
            p.push_back(QaStep::Keys(vec![KeyCode::ArrowUp],28));
            check(&mut p,"梯子向上爬到上一层",|s|s.gold_runner.qa_position().y<530.);
            p.push_back(QaStep::Shot("climbed"));
            p.push_back(QaStep::Keys(vec![KeyCode::KeyR],1));
            p.push_back(QaStep::Point(Area::Window,Vec2::new(0.155,0.865),Vec2::new(0.155,0.865),15));
            check(&mut p,"长按屏幕方向键",|s|s.gold_runner.qa_position().x>710.);
            p.push_back(QaStep::Point(Area::Window,Vec2::new(0.84,0.89),Vec2::new(0.84,0.89),1));
            check(&mut p,"屏幕左挖按钮",|s|s.gold_runner.qa_holes()==1);
            click(&mut p,GoldToggle);
            check(&mut p,"暂停",|s|s.gold_runner.state==GoldState::Paused);
            p.push_back(QaStep::Shot("paused"));
            click(&mut p,GoldToggle);
            click(&mut p,GoldToggle);
            click(&mut p,GoldReset);
            check(&mut p,"重开复位",|s|s.gold_runner.lives==3&&s.gold_runner.collected==0&&s.gold_runner.qa_holes()==0);
            click(&mut p,GoldToggle);
        }
        GameMode::Descent => {
            click(&mut p, DescentStart);
            check(&mut p, "开始挑战", |s| s.descent.state == DescentState::Playing);
            p.push_back(QaStep::Keys(vec![KeyCode::ArrowRight], 20));
            check(&mut p, "键盘向右移动", |s| s.descent.qa_x() > 550.);
            p.push_back(QaStep::Shot("playing"));
            click(&mut p, DescentToggle);
            check(&mut p, "暂停", |s| s.descent.state == DescentState::Paused);
            p.push_back(QaStep::Shot("paused"));
            click(&mut p, DescentToggle);
            p.push_back(QaStep::Point(Area::Window, Vec2::new(0.137,0.915),Vec2::new(0.137,0.915),20));
            check(&mut p, "按住屏幕左方向键", |s|s.descent.qa_x()<500.);
            p.push_back(QaStep::Keys(vec![KeyCode::KeyR],1));
            check(&mut p, "重新开局", |s|s.descent.floor==1&&s.descent.state==DescentState::Playing);
            p.push_back(QaStep::Wait(180));
            check(&mut p, "顶部尖刺结束与纪录保存", |s|s.descent.state==DescentState::Over&&save_directory().join("descent-best.txt").exists());
            p.push_back(QaStep::Shot("over"));
            click(&mut p,DescentStart);
            click(&mut p,DescentToggle);
        }
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild => {
            if builder_mode(s) {
                click(&mut p,Build);
                p.push_back(QaStep::Shot("building"));
            }
            click(&mut p, Material(crate::game::Material::Stone));
            check(&mut p, "材料切换", |s| {
                s.material == crate::game::Material::Stone
            });
            p.push_back(QaStep::BuildCell);
            check(&mut p, "实际搭建", |s| !s.undo.is_empty());
            p.push_back(QaStep::Shot("placed"));
            click(&mut p, Undo);
            check(&mut p, "撤销搭建", |s| s.undo.is_empty());
            if s.mode == GameMode::CloudBridge {
                click(&mut p, Code);
                for _ in 0..16 {click(&mut p, Add(Instruction::Forward));}
                p.push_back(QaStep::VisualFit);
                p.push_back(QaStep::Shot("full-program"));
                click(&mut p, Run);
                check(&mut p, "运行指令", |s| s.runner.is_some());
                click(&mut p, Build);
                check(&mut p, "搭建世界有效", |s| s.work.world.validate());
            } else {
                check(&mut p, "自动保存", |s| {
                    s.work.world.validate() && save_directory().join("island-v1.json").exists()
                });
                if s.mode==GameMode::Island {
                    click(&mut p,Code);
                    p.push_back(QaStep::Shot("programming"));
                    click(&mut p,Clear);
                    for _ in 0..16 {click(&mut p,Add(Instruction::Forward));}
                    check(&mut p,"16条指令完整编排",|s|s.work.program.len()==16&&s.work.valid());
                    p.push_back(QaStep::VisualFit);
                    p.push_back(QaStep::Shot("full-program"));
                    click(&mut p,Remove(15));
                    check(&mut p,"删除第16条指令",|s|s.work.program.len()==15);
                    click(&mut p,RepeatDelta(1));
                    check(&mut p,"增加重复次数",|s|s.work.repeat==6);
                    click(&mut p,RepeatDelta(-1));
                    check(&mut p,"减少重复次数",|s|s.work.repeat==5);
                    click(&mut p,Build);
                }
            }
        }
        GameMode::Ball2048 => {
            click(&mut p, BallLeft);
            check(&mut p, "左换道", |s| s.ball.lane == 0);
            click(&mut p, BallRight);
            check(&mut p, "右换道", |s| s.ball.lane == 1);
            click(&mut p, BallStart);
            check(&mut p, "暂停", |s| !s.ball.running);
            click(&mut p, BallReset);
            check(&mut p, "重新开始", |s| {
                s.ball.score == 0 && s.ball.number == 2
            });
        }
        GameMode::Tetris => {
            click(&mut p, TetrisLeft);
            check(&mut p, "左移", |s| s.tetris.x == 2);
            click(&mut p, TetrisRotate);
            check(&mut p, "旋转", |s| s.tetris.rotation == 1);
            click(&mut p, TetrisDrop);
            check(&mut p, "硬降落地", |s| {
                s.tetris.board.iter().any(|v| *v != 0)
            });
            p.push_back(QaStep::Shot("played"));
            click(&mut p, TetrisReset);
            check(&mut p, "重新开始", |s| {
                s.tetris.board.iter().all(|v| *v == 0)
            });
        }
        GameMode::Firefighters | GameMode::Detective | GameMode::SpaceFarm => {
            let q = mission_question(s.mode, 0);
            click(&mut p, MissionChoice((q.correct + 1) % 3));
            check(&mut p, "错误可重试", |s| {
                s.mission_wrong.is_some() && !s.mission_done
            });
            p.push_back(QaStep::Shot("wrong"));
            click(&mut p, MissionHint);
            check(&mut p, "提示", |s| !s.mission_feedback.is_empty());
            for stage in 0..4 {
                click(
                    &mut p,
                    MissionChoice(mission_question(s.mode, stage).correct),
                );
                check(&mut p, "正确反馈", |s| {
                    s.mission_done && s.mission_wrong.is_none()
                });
                p.push_back(QaStep::Shot(if stage == 3 {
                    "finished"
                } else {
                    "correct"
                }));
                click(&mut p, MissionNext);
            }
            check(&mut p, "四阶段衔接并重玩", |s| {
                s.mission_step == 0 && !s.mission_done
            });
        }
        GameMode::TankBattle => {
            p.push_back(QaStep::Keys(vec![KeyCode::KeyA], 14));
            check(&mut p, "键盘长按连续移动", |s| s.tank.player == (0, 6));
            p.push_back(QaStep::Keys(vec![KeyCode::KeyD, KeyCode::Space], 14));
            check(&mut p, "键盘边走边连发", |s| {
                s.tank.player == (3, 6) && s.tank.bullets.iter().any(|b| b.friendly && b.dx == 1.)
            });
            p.push_back(QaStep::Wait(12));
            check(&mut p, "松开方向键停止", |s| s.tank.player == (3, 6));
            click(&mut p, TankReset);
            p.push_back(QaStep::Point(
                Area::Tank,
                Vec2::new(0.5, 0.2),
                Vec2::new(0.5, 0.2),
                14,
            ));
            check(&mut p, "摇杆连续移动", |s| s.tank.player != (3, 6));
            p.push_back(QaStep::Point(
                Area::Fire,
                Vec2::splat(0.5),
                Vec2::splat(0.5),
                1,
            ));
            check(&mut p, "触摸开火", |s| s.tank.cooldown > 0.);
            p.push_back(QaStep::Shot("played"));
            click(&mut p, TankReset);
            check(&mut p, "重新开始", |s| {
                s.tank.score == 0 && s.tank.wave == 1 && s.tank.player == (3, 6)
            });
        }
        GameMode::SkyRaid => {
            click(&mut p, SkyMove(-1));
            check(&mut p, "左移与自动开火", |s| {
                s.sky.x < 0.5 && s.sky.shots.iter().any(|b| b.friendly)
            });
            click(&mut p, SkyMove(1));
            p.push_back(QaStep::Shot("played"));
            click(&mut p, SkyReset);
            check(&mut p, "重新起飞", |s| {
                s.sky.score == 0 && s.sky.lives == 3
            });
        }
        GameMode::BubbleIsland => {
            p.push_back(QaStep::Point(
                Area::Bubble,
                Vec2::new(0.5, 0.3),
                Vec2::new(0.5, 0.3),
                1,
            ));
            check(&mut p, "点按棋盘发射", |s| s.bubble.shot_count == 1);
            p.push_back(QaStep::Wait(45));
            p.push_back(QaStep::Point(
                Area::Bubble,
                Vec2::new(0.25, 0.3),
                Vec2::new(0.7, 0.3),
                8,
            ));
            check(&mut p, "拖动瞄准松开发射", |s| {
                s.bubble.shot_count == 2
            });
            p.push_back(QaStep::Shot("played"));
        }
        GameMode::BubbleBomber => {
            p.push_back(QaStep::Point(
                Area::Bomber,
                Vec2::new(0.8, 0.5),
                Vec2::new(0.8, 0.5),
                15,
            ));
            check(&mut p, "摇杆按住连续移动", |s| {
                s.bomber.player.0 >= 4
            });
            p.push_back(QaStep::Point(
                Area::BomberFire,
                Vec2::splat(0.5),
                Vec2::splat(0.5),
                1,
            ));
            check(&mut p, "放水泡", |s| !s.bomber.bombs.is_empty());
            click(&mut p, Help);
            check(&mut p, "暂停", |s| s.help_open);
            p.push_back(QaStep::Shot("paused"));
            click(&mut p, CloseHelp);
            p.push_back(QaStep::Wait(75));
            check(&mut p, "水泡到时爆开", |s| s.bomber.bombs.is_empty());
            for _ in 0..2 {
                p.push_back(QaStep::Point(
                    Area::BomberFire,
                    Vec2::splat(0.5),
                    Vec2::splat(0.5),
                    1,
                ));
                p.push_back(QaStep::Wait(90));
            }
            check(&mut p, "耗尽生命显示结束", |s| s.bomber.failed);
            click(&mut p, BomberReset);
            check(&mut p, "重新开始", |s| {
                s.bomber.player == (2, 8) && s.bomber.lives == 3 && !s.bomber.started
            });
        }
        GameMode::FruitNinja => {
            p.push_back(QaStep::FruitSlice);
            check(&mut p, "划水果得分", |s| s.fruit.score > 0);
            p.push_back(QaStep::Shot("played"));
        }
        GameMode::Fortune => {
            click(&mut p, FortuneRoll);
            p.push_back(QaStep::Wait(110));
            p.push_back(QaStep::Optional(FortuneBuy));
            p.push_back(QaStep::Wait(680));
            check(&mut p, "电脑完成回合并交还玩家", |s| {
                s.fortune.qa_state()["phase"] == "Ready" && s.fortune.qa_state()["turns"] == 4
            });
            p.push_back(QaStep::Shot("played"));
        }
        GameMode::Slots => {
            click(&mut p, SlotsBet(20));
            check(&mut p, "投入切换", |s| s.slots.qa_state()["bet"] == 20);
            click(&mut p, SlotsSpin);
            check(&mut p, "启动扣分一次", |s| {
                s.slots.spinning && s.slots.qa_state()["points"] == 980
            });
            p.push_back(QaStep::Wait(100));
            check(&mut p, "停轮结算", |s| {
                !s.slots.spinning && s.slots.qa_state()["turns"] == 1
            });
            p.push_back(QaStep::Shot("played"));
            click(&mut p, SlotsRefill);
            check(&mut p, "免费补分", |s| {
                s.slots.qa_state()["points"].as_u64().unwrap() >= 1980
            });
        }
        GameMode::FruitArcade => {
            click(&mut p,ArcadeRefill);
            check(&mut p,"完整显示的免费补充按钮可用",|s|s.arcade.qa_state()["points"]==2000);
            click(&mut p, ArcadeBet(0));
            click(&mut p, ArcadeBet(1));
            click(&mut p, ArcadeSpin);
            check(&mut p, "跑灯启动并扣分", |s| {
                s.arcade.qa_state()["running"] == true && s.arcade.qa_state()["points"] == 1980
            });
            p.push_back(QaStep::Wait(160));
            check(&mut p, "跑灯停下结算", |s| {
                s.arcade.qa_state()["running"] == false && s.arcade.qa_state()["played"] == true
            });
            p.push_back(QaStep::Shot("played"));
            click(&mut p, ArcadeClear);
            check(&mut p, "清空投入", |s| {
                s.arcade.qa_state()["bets"] == serde_json::json!([0, 0, 0, 0, 0])
            });
        }
        GameMode::Fishing => {
            click(&mut p, Help);
            check(&mut p, "说明按钮不会误撒网扣币", |s| {
                s.help_open && s.fishing.coins == 60
            });
            click(&mut p, CloseHelp);
            p.push_back(QaStep::Point(
                Area::Window,
                Vec2::new(0.45, 0.3),
                Vec2::new(0.45, 0.3),
                1,
            ));
            check(&mut p, "水面撒网", |s| {
                s.fishing.coins != 60 || s.fishing.catches > 0
            });
            p.push_back(QaStep::Shot("played"));
            click(&mut p, FishingReset);
            check(&mut p, "重新开始", |s| {
                s.fishing.coins == 60 && s.fishing.score == 0
            });
        }
        GameMode::HoneyHunt => {
            click(&mut p, Help);
            check(&mut p, "说明按钮不会误发射抓钩", |s| {
                s.help_open && s.honey.qa_state()["flying"] == false
            });
            click(&mut p, CloseHelp);
            p.push_back(QaStep::Point(
                Area::Window,
                Vec2::splat(0.5),
                Vec2::splat(0.5),
                1,
            ));
            check(&mut p, "轻点发射抓钩", |s| {
                s.honey.qa_state()["flying"] == true || s.honey.qa_state()["pulling"] == true
            });
            p.push_back(QaStep::Shot("played"));
            p.push_back(QaStep::Wait(85));
            check(&mut p, "抓钩收回", |s| {
                s.honey.qa_state()["flying"] == false && s.honey.qa_state()["pulling"] == false
            });
            click(&mut p, HoneyReset);
            check(&mut p, "重新开始", |s| {
                s.honey.score == 0 && s.honey.jars == 0
            });
        }
        GameMode::Snake => {
            check(&mut p, "新局等待开始", |s| {
                !s.snake.running && s.snake.body[0] == (8,12)
            });
            click(&mut p, SnakeMove(1, 0));
            p.push_back(QaStep::Wait(28));
            check(&mut p, "吃果实增长", |s| {
                s.snake.score == 1 && s.snake.body.len() == 4
            });
            p.push_back(QaStep::Wait(180));
            check(&mut p,"穿越边界后继续游戏",|s|s.snake.running&&!s.snake.over);
            click(&mut p,SnakeMove(0,-1));
            p.push_back(QaStep::Wait(12));
            check(&mut p,"转向向上",|s|s.snake.body[0].1<12);
            p.push_back(QaStep::Shot("turning"));
            click(&mut p, SnakeToggle);
            check(&mut p, "暂停", |s| !s.snake.running && !s.snake.over);
            p.push_back(QaStep::Shot("played"));
            click(&mut p, SnakeReset);
            check(&mut p, "重新开始", |s| {
                s.snake.score == 0 && s.snake.body.len() == 3 && !s.snake.running
            });
            if std::env::var_os("MAGNET_QA_SNAKE_RESIZE").is_some() {
                for (w,h,name) in [(1280.,800.,"wide"),(900.,900.,"square"),(540.,900.,"portrait-return")] {
                    p.push_back(QaStep::WindowSize(w,h));p.push_back(QaStep::VisualFit);p.push_back(QaStep::Shot(name));
                }
            }

        }
        GameMode::Minesweeper => {
            click(&mut p, MineReveal(2));
            check(&mut p, "首开地雷也安全", |s| {
                s.minesweeper.revealed[2] && !s.minesweeper.lost
            });
            click(&mut p, MineToggleFlag);
            click(&mut p, MineReveal(27));
            check(&mut p, "标记模式插旗", |s| {
                s.minesweeper.flagged[27] && !s.minesweeper.revealed[27]
            });
            p.push_back(QaStep::Shot("flag"));
            click(&mut p, MineReveal(27));
            click(&mut p, MineToggleFlag);
            click(&mut p, MineReveal(27));
            check(&mut p, "踩雷结束", |s| s.minesweeper.lost);
            p.push_back(QaStep::Shot("lost"));
            click(&mut p, MineReset);
            check(&mut p, "重新开始", |s| {
                !s.minesweeper.lost && !s.minesweeper.revealed.iter().any(|v| *v)
            });
            for cell in 0..64 {
                if !s.minesweeper.mines[cell] {
                    click(&mut p, MineReveal(cell));
                }
            }
            check(&mut p, "全部安全格胜利", |s| {
                s.minesweeper.won && !s.minesweeper.lost
            });
            p.push_back(QaStep::Shot("won"));
        }
        GameMode::Sokoban => {
            for c in b"DDRRRUULDRDDLDDLLUUURRDDRUUULUR" {
                let (dx, dy) = match c {
                    b'U' => (0, -1),
                    b'D' => (0, 1),
                    b'L' => (-1, 0),
                    _ => (1, 0),
                };
                click(&mut p, SokobanMove(dx, dy));
            }
            check(&mut p, "推箱子通关", |s| s.sokoban.won);
            p.push_back(QaStep::Shot("won"));
            click(&mut p, SokobanUndo);
            check(&mut p, "通关后可撤销", |s| !s.sokoban.won);
            click(&mut p, SokobanMove(1, 0));
            click(&mut p, SokobanNext);
            check(&mut p, "下一关", |s| {
                s.sokoban.level == 1 && s.sokoban.moves == 0
            });
            click(&mut p, SokobanChoose);
            click(&mut p, SokobanLevel(5));
            check(&mut p, "选第六关", |s| {
                s.sokoban.level == 5 && !s.sokoban.choosing
            });
        }
        GameMode::LightPath => {
            for level in 0..12 {
                for index in 0..if level < 2 {
                    2
                } else if level < 6 {
                    4
                } else {
                    6
                } {
                    click(&mut p, LightRotate(index));
                }
                p.push_back(QaStep::Wait(100));
                check(&mut p, "真实光束到达水晶通关", |s| {
                    s.light_path.is_solved()
                });
                if level == 0 || level == 11 {
                    p.push_back(QaStep::Shot(if level == 0 { "won" } else { "final" }));
                }
                if level < 11 {
                    click(&mut p, LightNext);
                }
            }
            check(&mut p, "十二关完成", |s| {
                s.light_path.qa_state()["level"] == 11
                    && s.light_path.qa_state()["completed"] == 4095
            });
        }
        GameMode::GardenDefense => {
            click(&mut p, GardenSelect(0));
            let pos = Vec2::new(
                (176. + 0.5 * 1248. / 9.) / 1600.,
                (178. + 0.5 * 628. / 5.) / 900.,
            );
            p.push_back(QaStep::Point(Area::Garden, pos, pos, 1));
            check(&mut p, "草地种植", |s| s.garden.plants.len() == 1);
            click(&mut p, GardenStart);
            check(&mut p, "开始守卫", |s| {
                !s.garden.ready && s.garden.elapsed > 0.
            });
            click(&mut p, GardenPause);
            check(&mut p, "暂停", |s| s.garden.paused);
            p.push_back(QaStep::Shot("played"));
            click(&mut p, GardenReset);
            check(&mut p, "重置本关", |s| {
                s.garden.ready && s.garden.plants.is_empty()
            });
        }
        GameMode::SpaceCourier => {
            click(&mut p, Courier(CourierAction::ChallengeRoute));
            click(&mut p, Courier(CourierAction::EventAnswer(0)));
            check(&mut p, "航线错误可重试", |s| {
                matches!(s.courier.stage, CourierStage::Event(_))
            });
            click(&mut p, Courier(CourierAction::EventAnswer(1)));
            check(&mut p, "挑战航线抵达", |s| {
                s.courier.planet == Some(1) && s.courier.stage == CourierStage::Map
            });
            for planet in [1, 0, 2] {
                if planet != 1 {
                    click(&mut p, Courier(CourierAction::Planet(planet)));
                    click(&mut p, Courier(CourierAction::SafeRoute));
                }
                for slot in planet * 3..planet * 3 + 3 {
                    click(&mut p, Courier(CourierAction::Delivery(slot)));
                    click(
                        &mut p,
                        Courier(CourierAction::Answer([0, 1, 2, 0, 0, 1, 0, 0, 1][slot])),
                    );
                }
            }
            check(&mut p, "九件包裹送完", |s| {
                s.courier.delivered.iter().all(|v| *v)
            });
            click(&mut p, Courier(CourierAction::Planet(0)));
            click(&mut p, Courier(CourierAction::FinalAnswer(1)));
            check(&mut p, "终点挑战完成", |s| {
                s.courier.stage == CourierStage::Complete
            });
            p.push_back(QaStep::Shot("finished"));
            click(&mut p, Courier(CourierAction::NextChapter));
            check(&mut p, "进入下一章", |s| {
                s.courier.chapter == 2 && s.courier.delivered.iter().all(|v| !*v)
            });
        }
        GameMode::Math => {
            click(&mut p, MathAnswer(6));
            check(&mut p, "答错可重试", |s| {
                s.math.wrong == Some(6) && !s.math.solved
            });
            click(&mut p, MathHint);
            check(&mut p, "提示可用", |s| s.math.hinted);
            click(&mut p, MathAnswer(5));
            check(&mut p, "直接答题成功", |s| {
                s.math.solved && s.math.correct_total == 1
            });
            click(&mut p, MathNext);
            check(&mut p, "下一题保留难度", |s| {
                s.math.index == 1 && !s.math.solved && s.math.level == 0
            });
        }
        GameMode::Literacy => {
            click(&mut p, LiteracyStart(0));
            click(&mut p, LiteracyNext);
            click(&mut p, LiteracySelect(1));
            check(&mut p, "识字错误可修改", |s| {
                s.literacy.checked && !s.literacy.correct
            });
            click(&mut p, LiteracySelect(0));
            check(&mut p, "改选立即答对", |s| {
                s.literacy.checked && s.literacy.correct
            });
            click(&mut p, LiteracyNext);
            check(&mut p, "进入下一练习", |s| {
                s.literacy.stage == 2 && !s.literacy.checked
            });
        }
        GameMode::MusicFactory => {
            click(&mut p, PianoKey((s.piano.target() + 1) % 7));
            check(&mut p, "错音不跳过目标", |s| {
                s.piano.step == 0 && s.piano.score == 0
            });
            for note in PIANO_NOTES.iter().take(4) {
                click(&mut p, PianoKey(*note));
            }
            check(&mut p, "连续正确弹奏", |s| {
                s.piano.step == 4 && s.piano.combo == 4
            });
            click(&mut p, PianoReset);
            check(&mut p, "重新练习", |s| {
                s.piano.step == 0 && s.piano.score == 0
            });
        }
        GameMode::PixelAssault => {
            click(&mut p, AssaultRight);
            check(&mut p, "向右移动", |s| s.assault.player_x > 0.18);
            click(&mut p, AssaultJump);
            check(&mut p, "跳跃", |s| s.assault.jump > 0.);
            click(&mut p, AssaultFire);
            check(&mut p, "开火命中或子弹在飞行", |s| {
                !s.assault.bullets.is_empty() || s.assault.score > 0
            });
            click(&mut p, AssaultReset);
            check(&mut p, "重新开始", |s| {
                s.assault.lives == 3 && s.assault.score == 0
            });
        }
        GameMode::MushroomRun => {
            click(&mut p, PlatformRight);
            check(&mut p, "按住前进", |s| s.platform.distance > 0.);
            click(&mut p, PlatformJump);
            check(&mut p, "跳跃离地", |s| s.platform.player_y > 0.);
            click(&mut p, PlatformReset);
            check(&mut p, "重新开始", |s| {
                s.platform.distance == 0. && s.platform.lives == 3
            });
        }
        GameMode::MoonFestival => {
            click(&mut p, MoonShake);
            check(&mut p, "开始摇骰子", |s| s.moon.rolling);
            p.push_back(QaStep::Wait(30));
            check(&mut p, "完成一次结算", |s| {
                !s.moon.rolling && s.moon.rolls_left == 4 && s.moon.score > 0
            });
            click(&mut p, MoonReset);
            check(&mut p, "免费重开", |s| {
                s.moon.rolls_left == 5 && s.moon.score == 0
            });
        }
        GameMode::Classic2048 => {
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                click(&mut p, ClassicMove(dx, dy));
            }
            check(&mut p, "方向移动可用", |s| s.classic_2048.moves > 0);
            click(&mut p, ClassicReset);
            check(&mut p, "重置棋盘", |s| {
                s.classic_2048.moves == 0
                    && s.classic_2048.cells.iter().filter(|v| **v != 0).count() == 2
            });
        }
        GameMode::Klotski => {
            click(&mut p, KlotskiSelect(6));
            click(&mut p, KlotskiMove(0, 1));
            check(&mut p, "选中棋子下移", |s| s.klotski.pieces[6].y == 4);
            click(&mut p, KlotskiMove(0, 1));
            check(&mut p, "不能移出边界", |s| s.klotski.pieces[6].y == 4);
            click(&mut p, KlotskiReset);
            check(&mut p, "重置布局", |s| s.klotski.pieces[6].y == 3);
        }
        GameMode::Sudoku => {
            click(&mut p, SudokuSize(4));
            click(&mut p, SudokuSelect(1));
            click(&mut p, SudokuDigit(1));
            check(&mut p, "错误标记", |s| {
                s.sudoku.is_wrong(1) && s.sudoku.mistakes == 1
            });
            click(&mut p, SudokuClear);
            check(&mut p, "清除错误", |s| {
                s.sudoku.cells[1] == 0 && s.sudoku.mistakes == 0
            });
            click(&mut p, SudokuHint);
            check(&mut p, "提示填入正确数字", |s| {
                s.sudoku.cells[1] == 2 && !s.sudoku.is_wrong(1)
            });
            click(&mut p, SudokuNew);
            check(&mut p, "更换题目", |s| {
                s.sudoku.variant == 1 && s.sudoku.size == 4
            });
        }
        GameMode::Breakout => {
            click(&mut p, BreakoutLaunch);
            check(&mut p, "发球开始", |s| {
                !s.breakout.ready && !s.breakout.over
            });
            click(&mut p, BreakoutToggle);
            check(&mut p, "暂停", |s| s.breakout.paused);
            click(&mut p, BreakoutReset);
            check(&mut p, "重新开始", |s| {
                s.breakout.ready && s.breakout.lives == 3
            });
        }
        GameMode::ElementLab => {
            click(&mut p, ElementPick(2));
            check(&mut p, "错误元素不消耗进度", |s| {
                s.elements.picked.is_empty() && !s.elements.complete
            });
            for atom in [1, 1, 8] {
                click(&mut p, ElementPick(atom));
            }
            check(&mut p, "水分子配方完成", |s| {
                s.elements.complete && s.elements.score > 0
            });
            click(&mut p, ElementNext);
            check(&mut p, "下一实验", |s| {
                s.elements.stage == 1 && !s.elements.complete
            });
        }
        GameMode::CrystalGarden => {
            click(&mut p, Material(crate::game::Material::Stone));
            p.push_back(QaStep::BuildCell);
            check(&mut p, "餐厅搭建", |s| !s.undo.is_empty());
            click(&mut p, Undo);
            check(&mut p, "撤销搭建", |s| s.undo.is_empty());
            click(&mut p, RestaurantCheck);
            check(&mut p, "检查布置进度", |s| s.message.contains("餐厅"));
        }
        GameMode::RabbitHome => {
            for (target,shot) in [(45.,"start"),(186.,"river"),(600.,"forest"),(740.,"birch"),(1050.,"roots"),(1460.,"pine"),(1775.,"hollow-tree"),(2050.,"cottage"),(2085.,"finished")] {
                p.push_back(QaStep::RabbitRunUntil(target));
                check(&mut p,"长地图镜头与无伤行程",|s| s.rabbit_home.lives()==3 && s.rabbit_home.qa_camera()>=0.);
                p.push_back(QaStep::Shot(shot));
            }
            check(&mut p,"21屏完整通关",|s|s.rabbit_home.outcome()==Some(true) && s.rabbit_home.progress()==1. && s.rabbit_home.combo()==10);

        }
    }
    p.push_back(QaStep::Shot("checked"));
    if !matches!(s.mode,GameMode::Descent|GameMode::GoldRunner) { p.push_back(QaStep::Help); }
    p.push_back(QaStep::Library);
    p.push_back(QaStep::Wait(15));
    check(&mut p, "返回游戏库", |s| s.library_open);
    p.push_back(QaStep::Shot("library"));
    click(&mut p, LibraryReturn);
    check(&mut p, "从游戏库返回对局", |s| !s.library_open);
    p.push_back(QaStep::Wait(15));
    p.push_back(QaStep::Shot("returned"));
    if matches!(
        s.mode,
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild | GameMode::CrystalGarden
    ) {
        p.push_back(QaStep::BuildCell);
        check(&mut p, "游戏库返回后仍可搭建", |s| {
            !s.undo.is_empty()
        });
    }
    // The tank library card launches the online game. Native controls use the
    // development entry and the return-to-current-game button tested above.
    if s.mode != GameMode::TankBattle {
        p.push_back(QaStep::Library);
        p.push_back(QaStep::Wait(15));
        p.push_back(QaStep::SelectFromLibrary);
        p.push_back(QaStep::Shot("reentered"));
    }
    p
}

fn cloud_qa_route(p:&mut VecDeque<QaStep>,g:&mut crate::cloud_walk_game::CloudWalk,target:usize,keyboard:bool){
 let mut from=vec![None;g.map.cells.len()];let mut queue=VecDeque::from([g.robot]);from[g.robot]=Some(g.robot);
 while let Some(id)=queue.pop_front(){if id==target{break;}let [x,z]=g.map.cells[id].at;for(dx,dz)in [(-1,0),(1,0),(0,-1),(0,1)]{if let Some(n)=g.map.id([x+dx,z+dz]){if from[n].is_none()&&g.passable(n){from[n]=Some(id);queue.push_back(n);}}}}
 let mut route=vec![];let mut n=target;while n!=g.robot{route.push(n);n=from[n].expect("reference route");}route.reverse();
 for n in route{let a=g.map.cells[g.robot].at;let b=g.map.cells[n].at;let(dx,dz)=(b[0]-a[0],b[1]-a[1]);
  if keyboard{p.push_back(QaStep::Keys(vec![if dx>0{KeyCode::KeyD}else if dx<0{KeyCode::KeyA}else if dz>0{KeyCode::KeyS}else{KeyCode::KeyW}],1));}else{p.push_back(QaStep::Click(Action::CloudWalk(CloudAction::Move(dx,dz))));}g.move_dir(dx,dz);
 }
}
fn cloud_walk_scenario(_s:&Session)->VecDeque<QaStep>{
 use crate::cloud_walk_game::CloudWalk;
 use CloudAction::*;
 let mut p=VecDeque::from([QaStep::Wait(35),QaStep::Shot("initial")]);
 let click=|p:&mut VecDeque<QaStep>,a|p.push_back(QaStep::Click(Action::CloudWalk(a)));
 if std::env::var_os("MAGNET_QA_PUZZLE_RELOAD").is_some(){check(&mut p,"新地图存档独立恢复",|s|s.cloud_walk.snapshot().valid()&&s.cloud_walk.progress.stars.iter().all(|n|*n==3)&&!s.cloud_walk.plan.bridges.is_empty());p.push_back(QaStep::Shot("restored"));return p;}
 for stage in 0..6{
  let mut g=CloudWalk::default();g.choose(stage);let plan=g.verification_plan().unwrap();
  click(&mut p,Stage(stage));click(&mut p,Restart);
  if stage==0{
   click(&mut p,Enter);p.push_back(QaStep::Keys(vec![KeyCode::KeyD],1));p.push_back(QaStep::Keys(vec![KeyCode::KeyD],1));
   check(&mut p,"缺桥停在岸边",|s|s.cloud_walk.map.cells[s.cloud_walk.robot].at==[1,0]&&!s.cloud_walk.won);
   click(&mut p,Back);
   let id=*plan.bridges.keys().next().unwrap();p.push_back(QaStep::CloudPlace(id));click(&mut p,Erase);p.push_back(QaStep::CloudPlace(id));
   check(&mut p,"拆除退回材料",|s|s.cloud_walk.plan.bridges.is_empty());click(&mut p,Undo);
   check(&mut p,"撤销恢复桥块",|s|s.cloud_walk.plan.bridges.len()==1);click(&mut p,Restart);
  }
  for (&id,&b) in &plan.bridges{click(&mut p,Material(b));p.push_back(QaStep::CloudPlace(id));}
  check(&mut p,"地图桥梁与材料合法",|s|s.cloud_walk.snapshot().valid()&&!s.cloud_walk.plan.bridges.is_empty());
  p.push_back(QaStep::VisualFit);p.push_back(QaStep::Shot(["map-1-plan","map-2-plan","map-3-plan","map-4-plan","map-5-plan","map-6-plan"][stage]));
  click(&mut p,Enter);p.push_back(QaStep::Wait(45));
  check(&mut p,"进入过桥后等待不会自动移动",|s|s.cloud_walk.walking&&s.cloud_walk.robot==s.cloud_walk.map.start&&!s.cloud_walk.won);
  p.push_back(QaStep::VisualFit);g.plan=plan;g.enter();
  if stage==0{
   p.push_back(QaStep::Keys(vec![KeyCode::KeyD],24));check(&mut p,"键盘长按连续移动且松开停止",|s|s.cloud_walk.map.cells[s.cloud_walk.robot].at==[4,0]);
   g.move_dir(1,0);g.move_dir(1,0);g.move_dir(1,0);g.move_dir(1,0);
   p.push_back(QaStep::Wait(20));check(&mut p,"松开不会继续前进",|s|s.cloud_walk.map.cells[s.cloud_walk.robot].at==[4,0]);
  }
  if stage==0{
   p.push_back(QaStep::Hold(Action::CloudWalk(Move(-1,0)),24));p.push_back(QaStep::Wait(20));check(&mut p,"长按屏幕方向键并松开停在起点",|s|s.cloud_walk.robot==s.cloud_walk.map.start);
   for _ in 0..4{g.move_dir(-1,0);}
  }
  if stage==5{let goal=g.map.goal;cloud_qa_route(&mut p,&mut g,goal,false);check(&mut p,"终点会挡住尚未取星晶的玩家",|s|!s.cloud_walk.won&&!s.cloud_walk.gem_taken&&s.cloud_walk.robot!=s.cloud_walk.map.goal);p.push_back(QaStep::Shot("required-crystal-blocked"));}
  let targets=g.map.gem.into_iter().chain(std::iter::once(g.map.goal)).collect::<Vec<_>>();
  for target in targets{cloud_qa_route(&mut p,&mut g,target,stage==0);}
  p.push_back(QaStep::Wait(12));check(&mut p,"真实手动输入到达终点并收集星晶",|s|s.cloud_walk.won&&s.cloud_walk.progress.stars[s.cloud_walk.stage]==3);
  p.push_back(QaStep::Shot(["map-1-won","map-2-won","map-3-won","map-4-won","map-5-won","map-6-won"][stage]));
  click(&mut p,Back);check(&mut p,"返回搭桥回起点且保留桥梁",|s|!s.cloud_walk.walking&&s.cloud_walk.robot==s.cloud_walk.map.start&&!s.cloud_walk.plan.bridges.is_empty());
 }
 check(&mut p,"六张独立地图全部完成",|s|s.cloud_walk.progress.stars.iter().all(|n|*n==3));p.push_back(QaStep::Help);p.push_back(QaStep::Library);p.push_back(QaStep::SelectFromLibrary);check(&mut p,"重新进入保留新地图进度",|s|s.cloud_walk.progress.stars.iter().all(|n|*n==3));p
}
fn puzzle_scenario(s:&Session)->VecDeque<QaStep> {
    use crate::puzzle_game::{Bridge,Op};
    use PuzzleAction::*;
    let mut p=VecDeque::from([QaStep::Wait(35),QaStep::Shot("initial")]);
    let click=|p:&mut VecDeque<QaStep>,a|p.push_back(QaStep::Click(Action::Puzzle(a)));
    if std::env::var_os("MAGNET_QA_PUZZLE_RELOAD").is_some() {
        check(&mut p,"重新启动后读取桥梁、指令和星级",|s|{let g=&s.builder_puzzles[puzzle_index(s)];g.snapshot().valid(g.kind)&&g.progress.stars.iter().any(|n|*n>0)&&!g.plan.bridges.is_empty()});
        p.push_back(QaStep::Shot("restored"));p.push_back(QaStep::Library);return p;
    }
    let fill=|p:&mut VecDeque<QaStep>,nodes:&[usize],material|{click(p,Material(material));for &n in nodes{click(p,Cell(n));}};
    let guide=|p:&mut VecDeque<QaStep>,stage|{
        click(p,Build(false));click(p,Clear);
        if stage==5{click(p,Add(Op::Right));click(p,Repeat);}
        click(p,Add(Op::Forward));click(p,Repeat);click(p,Repeat);
        if stage==4 {
            for (op,repeats) in [(Op::Right,1),(Op::Forward,1),(Op::Left,1),(Op::Forward,3),(Op::Right,1),(Op::Forward,1)] {
                click(p,Add(op));for _ in 1..repeats{click(p,Repeat);}
            }
        } else if matches!(stage,1|2|5) {click(p,Add(Op::Right));click(p,Add(Op::Forward));click(p,Repeat);click(p,Add(Op::Left));click(p,Add(Op::Forward));click(p,Repeat);click(p,Repeat);}
    };
    if std::env::var_os("MAGNET_QA_PUZZLE_CAMPAIGN").is_some()||std::env::var_os("MAGNET_QA_PUZZLE_ADDED").is_some() {
        let stages:Vec<usize>=if std::env::var_os("MAGNET_QA_PUZZLE_ADDED").is_some(){vec![3,4,5]}else{(1..=crate::puzzle_game::LEVEL_COUNT).map(|n|crate::puzzle_game::Level::index_for_number(if s.mode==GameMode::CloudBridge{PuzzleKind::Cloud}else{PuzzleKind::Robot},n).unwrap()).collect()};
        for stage in stages {
            click(&mut p,Stage(stage));if s.mode==GameMode::CloudBridge {click(&mut p,Restart);}
            if s.mode==GameMode::CloudBridge {
                if stage<=3 {fill(&mut p,&[2,4],Bridge::Wood);fill(&mut p,&[3],if matches!(stage,2|3){Bridge::Stone}else{Bridge::Wood});}
                else {let stone=if stage==4{8}else{10};for node in [8,9,10,11,15,16,17]{fill(&mut p,&[node],if node==stone{Bridge::Stone}else{Bridge::Wood});}}
            } else {
                click(&mut p,Build(true));fill(&mut p,if matches!(stage,0|3){&[1,2]}else if stage==4{&[1,2,9,10]}else{&[1,2,6]},Bridge::Wood);guide(&mut p,stage);
            }
            check(&mut p,"关卡库存与指令容量合法",|s|{let g=&s.builder_puzzles[puzzle_index(s)];g.snapshot().valid(g.kind)});
            p.push_back(QaStep::VisualFit);p.push_back(QaStep::Shot(["stage-1-plan","stage-2-plan","stage-3-plan","added-a-plan","added-b-plan","added-c-plan"][stage]));
            click(&mut p,Run);p.push_back(QaStep::Wait(450));
            check(&mut p,"关卡通过真实搭桥与指令操作通关",|s|{let g=&s.builder_puzzles[puzzle_index(s)];g.won&&g.progress.stars[g.stage]>0});
            p.push_back(QaStep::Shot(["stage-1-won","stage-2-won","stage-3-won","added-a-won","added-b-won","added-c-won"][stage]));
            if stage==1 {
                if s.mode==GameMode::CloudBridge {click(&mut p,Restart);fill(&mut p,&[8,9,10,11,15,16,17],Bridge::Wood);}
                else {
                    click(&mut p,Build(true));click(&mut p,Erase);click(&mut p,Cell(6));fill(&mut p,&[9,10],Bridge::Wood);
                    click(&mut p,Build(false));click(&mut p,Clear);
                    for (op,repeats) in [(Op::Forward,3),(Op::Right,1),(Op::Forward,1),(Op::Left,1),(Op::Forward,3),(Op::Right,1),(Op::Forward,1)] {
                        click(&mut p,Add(op));for _ in 1..repeats{click(&mut p,Repeat);}
                    }
                }
                click(&mut p,Run);p.push_back(QaStep::Wait(600));check(&mut p,"分支关第二条路线也可通关并收集星晶",|s|{let g=&s.builder_puzzles[puzzle_index(s)];g.won&&g.bonus});
                p.push_back(QaStep::Shot("branch-alternative-won"));
            }
        }
    } else if s.mode==GameMode::CloudBridge {
        click(&mut p,Stage(2));click(&mut p,Restart);p.push_back(QaStep::VisualFit);p.push_back(QaStep::Shot("confirmed-cloud"));
        fill(&mut p,&[2,3,4],Bridge::Wood);click(&mut p,Run);p.push_back(QaStep::Wait(180));
        check(&mut p,"木桥无法通过风口，保留搭建",|s|{let g=&s.builder_puzzles[0];!g.won&&g.robot==2&&g.plan.bridges.len()==3});
        p.push_back(QaStep::Shot("wind-error"));fill(&mut p,&[3],Bridge::Stone);
        check(&mut p,"替换石桥退回木桥",|s|s.builder_puzzles[0].remaining(Bridge::Wood)==5&&s.builder_puzzles[0].remaining(Bridge::Stone)==0);
        click(&mut p,Undo);check(&mut p,"撤销恢复材料",|s|s.builder_puzzles[0].remaining(Bridge::Stone)==1);
        fill(&mut p,&[3],Bridge::Stone);click(&mut p,Run);p.push_back(QaStep::Wait(400));
        check(&mut p,"近路可以独立通关",|s|s.builder_puzzles[0].won&&!s.builder_puzzles[0].bonus);p.push_back(QaStep::Shot("direct-won"));
        click(&mut p,Restart);fill(&mut p,&[8,9,10,11,15,16,17],Bridge::Wood);click(&mut p,Cell(2));
        check(&mut p,"库存不足不能继续铺桥",|s|s.builder_puzzles[0].remaining(Bridge::Wood)==0&&s.builder_puzzles[0].plan.bridges.len()==7);
        click(&mut p,Undo);check(&mut p,"耗尽材料后撤销退款",|s|s.builder_puzzles[0].remaining(Bridge::Wood)==1);click(&mut p,Cell(17));
        click(&mut p,Run);p.push_back(QaStep::Wait(600));check(&mut p,"绕路通关并获得额外星晶",|s|s.builder_puzzles[0].won&&s.builder_puzzles[0].bonus);p.push_back(QaStep::Shot("detour-won"));
    } else {
        click(&mut p,Stage(2));click(&mut p,Build(true));fill(&mut p,&[1,2,6],Bridge::Wood);
        click(&mut p,Undo);check(&mut p,"有限木桥撤销退款",|s|s.builder_puzzles[1].remaining(Bridge::Wood)==1);click(&mut p,Cell(6));guide(&mut p,2);
        check(&mut p,"可见5张指令包含重复次数",|s|{let g=&s.builder_puzzles[1];g.plan.cards.len()==5&&g.plan.cards[0].repeats==3&&g.plan.cards[2].repeats==2&&g.plan.cards[4].repeats==3});
        p.push_back(QaStep::VisualFit);p.push_back(QaStep::Shot("confirmed-program"));
        for _ in 0..6{click(&mut p,Step);}
        check(&mut p,"第6个动作踩机关开门",|s|s.builder_puzzles[1].gate_open&&s.builder_puzzles[1].robot==5);p.push_back(QaStep::Shot("gate-open"));
        click(&mut p,Reset);check(&mut p,"复位关门并保留桥梁指令",|s|{let g=&s.builder_puzzles[1];!g.gate_open&&g.plan.bridges.len()==3&&g.plan.cards.len()==5});
        click(&mut p,Run);p.push_back(QaStep::Wait(450));check(&mut p,"实际程序运行通关",|s|s.builder_puzzles[1].won);p.push_back(QaStep::Shot("program-won"));
        click(&mut p,Clear);for _ in 0..9{click(&mut p,Add(Op::Wait));}
        check(&mut p,"指令上限8张且可保存",|s|{let g=&s.builder_puzzles[1];g.plan.cards.len()==8&&g.snapshot().valid(PuzzleKind::Robot)});p.push_back(QaStep::VisualFit);p.push_back(QaStep::Shot("full-program"));
    }
    p.push_back(QaStep::Help);
    p.push_back(QaStep::Library);check(&mut p,"返回游戏库",|s|s.library_open);
    p.push_back(QaStep::SelectFromLibrary);check(&mut p,"再次进入保留新关卡进度",|s|{let g=&s.builder_puzzles[puzzle_index(s)];g.snapshot().valid(g.kind)&&g.progress.stars.iter().any(|n|*n>0)});
    p.push_back(QaStep::Shot("reentered"));p
}

fn builder_campaign(s:&Session)->VecDeque<QaStep> {
    use Action::*;
    let mut p=VecDeque::from([QaStep::Wait(35),QaStep::Shot("initial")]);
    for level in 0..5 {
        click(&mut p,if s.mode==GameMode::CloudBridge {CloudLevel(level)}else{BuilderLevel(level)});
        click(&mut p,Code);click(&mut p,Clear);
        let mut work=Work::default();work.choose_level(level);assert!(work.load_guide_route());
        let operations=Runner::new(&work).ops;
        if s.mode==GameMode::Island&&matches!(level,3|4){click(&mut p,Finish);}
        for instruction in &operations {click(&mut p,Add(*instruction));}
        p.push_back(QaStep::VisualFit);
        p.push_back(QaStep::Shot(["program-1","program-2","program-3","program-4","program-5"][level]));
        click(&mut p,Run);p.push_back(QaStep::Wait((operations.len() as u32+1)*26));
        check(&mut p,"真实操作完成关卡、收集星晶并抵达目标",|s|s.work.world.won()&&s.work.completed.contains(&s.work.level));
        p.push_back(QaStep::Shot(["won-1","won-2","won-3","won-4","won-5"][level]));
    }
    click(&mut p,if s.mode==GameMode::CloudBridge {CloudLevel(5)}else{BuilderLevel(5)});
    click(&mut p,Build);
    for (i,cell) in (4..=5).flat_map(|x|(-2..=2).map(move|z|[x,0,z])).enumerate() {
        click(&mut p,Material([crate::game::Material::Wood,crate::game::Material::Stone,crate::game::Material::Grass][i%3]));
        p.push_back(QaStep::BuilderPlace(cell));
    }
    check(&mut p,"终章通过真实搭建完成10块3种材料",|s|{let(n,m)=s.work.creative_progress();n>=10&&m>=3&&s.work.completed.contains(&5)});
    p.push_back(QaStep::Shot("creative-complete"));
    p.push_back(QaStep::Library);check(&mut p,"返回游戏库",|s|s.library_open);
    p
}

type Surfaces<'w, 's> = Query<
    'w,
    's,
    (
        &'static ComputedNode,
        &'static GlobalTransform,
        Option<&'static Action>,
        Option<&'static BomberJoystick>,
        Option<&'static TankStick>,
        Option<&'static TankFirePad>,
        Option<&'static BubbleBoard>,
        Option<&'static GardenBoard>,
        Option<&'static bevy::ui::CalculatedClip>,
        Option<&'static BomberTouchPlace>,
    ),
>;

fn input(
    mut qa: ResMut<Qa>,
    s: Res<Session>,
    forest: Res<ForestTrain>,
    courier_canvases: Query<&ComputedNode,With<CourierCanvas>>,
    snake_boards: Query<&ComputedNode,With<SnakePhoneBoard>>,
    texts: Query<&Text>,
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    surfaces: Surfaces,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    cards: Query<(&ComputedNode, &GlobalTransform), With<LibraryCards>>,
    mut wheels: EventWriter<MouseWheel>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
) {
    qa.frame += 1;
    let mut synthetic_keys=vec![KeyCode::ArrowRight, KeyCode::ArrowLeft, KeyCode::Space];
    if s.mode==GameMode::CloudBridge {synthetic_keys.extend([KeyCode::KeyW,KeyCode::KeyA,KeyCode::KeyS,KeyCode::KeyD,KeyCode::ArrowUp,KeyCode::ArrowDown]);}
    if s.mode==GameMode::TankBattle {synthetic_keys.extend([KeyCode::KeyW,KeyCode::KeyA,KeyCode::KeyS,KeyCode::KeyD,KeyCode::ArrowUp,KeyCode::ArrowDown,KeyCode::KeyJ]);}
    if s.mode==GameMode::GoldRunner {synthetic_keys.extend([KeyCode::ArrowUp,KeyCode::ArrowDown,KeyCode::KeyR,KeyCode::KeyX,KeyCode::KeyZ,KeyCode::KeyP,KeyCode::Enter]);}
    if s.mode==GameMode::ForestTrain {synthetic_keys.extend([KeyCode::KeyP,KeyCode::Enter,KeyCode::KeyR,KeyCode::KeyZ,KeyCode::Digit1,KeyCode::Digit2,KeyCode::KeyA,KeyCode::KeyB]);}
    for key in synthetic_keys {
        let wanted = matches!(&qa.active, Some(QaStep::Keys(held, _)) if qa.remaining > 0 && held.contains(&key));
        if !wanted { keys.release(key); }
    }
    if qa.remaining <= 5
        || !matches!(
            qa.active,
            Some(
                QaStep::Click(_)
                    | QaStep::Hold(..)
                    | QaStep::Optional(_)
                    | QaStep::Point(..)
                    | QaStep::BuildCell
                    | QaStep::BuilderPlace(_)
                    | QaStep::FruitSlice
            )
        )
    {
        mouse.release(MouseButton::Left);
    }
    if !qa.initialized {
        qa.initialized = true;
        qa.mode = Some(s.mode);
        qa.root = PathBuf::from(std::env::var("MAGNET_CAPTURE").unwrap())
            .parent()
            .unwrap()
            .to_owned();
        qa.steps = scenario(&s);
    }
    if qa.failed || qa.exit_frame.is_some() {
        return;
    }
    let Ok((window_entity, mut window)) = windows.single_mut() else {
        return;
    };
    if qa.remaining > 0 {
        qa.remaining -= 1;
        match qa.active.clone() {
            Some(QaStep::RabbitRunUntil(target)) => {
                if s.rabbit_home.qa_position()>=target || s.rabbit_home.outcome().is_some() {qa.remaining=0;}
                else {keys.press(KeyCode::ArrowRight);if s.rabbit_home.qa_should_jump(){keys.press(KeyCode::Space);}}
            }
            Some(QaStep::Keys(held, _)) => {
                for key in held { keys.press(key); }
            }
            Some(QaStep::Click(a)) | Some(QaStep::Hold(a,_)) | Some(QaStep::Optional(a)) if qa.remaining >= 5 => {
                if let Some(pos) = action_point(&surfaces, &window, a) {
                    window.set_cursor_position(Some(pos));
                    mouse.press(MouseButton::Left);
                }
            }
            Some(QaStep::Point(area, from, to, frames)) if qa.remaining >= 5 => {
                let t = 1. - (qa.remaining - 5) as f32 / frames.max(1) as f32;
                if let Some(pos) = area_point(&surfaces, &window, area, from.lerp(to, t)) {
                    window.set_cursor_position(Some(pos));
                    mouse.press(MouseButton::Left);
                }
            }
            Some(QaStep::BuildCell | QaStep::FruitSlice) if qa.remaining >= 5 => {
                if let Some(point) = qa.point {
                    window.set_cursor_position(Some(point));
                    mouse.press(MouseButton::Left);
                }
            }
            _ => {}
        }
        return;
    }
    qa.active = None;
    let Some(step) = qa.steps.pop_front() else {
        qa.exit_frame = Some(qa.frame + 15);
        return;
    };
    let available = |a| action_point(&surfaces, &window, a).is_some();
    match step.clone() {
        QaStep::Click(a) | QaStep::Optional(a) => {
            if !available(a) {
                if matches!(step, QaStep::Optional(_)) {
                    return;
                }
                qa.fail(&s, &format!("找不到可见操作 {a:?}"));
                return;
            }
            qa.records
                .push(serde_json::json!({"input": format!("{a:?}")}));
            qa.remaining = 6;
            qa.active = Some(step);
        }
        QaStep::Hold(a,frames) => {if !available(a){qa.fail(&s,"长按按钮不可见");return;}qa.remaining=frames+5;qa.active=Some(step);}
        QaStep::Point(.., frames) => {
            qa.remaining = frames + 5;
            qa.active = Some(step);
        }
        QaStep::RabbitRunUntil(_) => {
            qa.remaining=20000;qa.active=Some(step);
        }
        QaStep::Keys(_, frames) => {
            qa.remaining = frames;
            qa.active = Some(step);
        }
        QaStep::BuildCell => {
            if let Ok((camera, transform)) = cameras.single() {
                qa.point = s
                    .work
                    .world
                    .blocks
                    .iter()
                    .filter(|b| {
                        b.pos[1] == 0
                            && [b.pos[0], b.pos[2]] != s.work.world.robot
                            && [b.pos[0], b.pos[2]] != s.work.world.goal
                    })
                    .filter_map(|b| {
                        if builder_illustrated(&s) {
                            return Some(builder_cell_screen(&s,Vec3::new(b.pos[0] as f32,0.5,b.pos[2] as f32),window.width(),window.height()));
                        }
                        camera
                            .world_to_viewport(
                                transform,
                                Vec3::new(b.pos[0] as f32, 0.48, b.pos[2] as f32),
                            )
                            .ok()
                    })
                    .filter(|p| {
                        p.x > window.width() * 0.3
                            && p.x < window.width() * 0.65
                            && p.y > window.height() * 0.25
                            && p.y < window.height() * 0.65
                    })
                    .min_by(|a, b| {
                        a.distance_squared(Vec2::new(window.width() * 0.5, window.height() * 0.45))
                            .total_cmp(&b.distance_squared(Vec2::new(
                                window.width() * 0.5,
                                window.height() * 0.45,
                            )))
                    });
            }
            if qa.point.is_none() {
                qa.fail(&s, "找不到搭建区域");
                return;
            }
            qa.remaining = 7;
            qa.active = Some(step);
        }
        QaStep::CloudPlace(id) => {
            qa.point=Some(cloud_cell_screen(&s,id,window.width(),window.height()));qa.remaining=7;qa.active=Some(QaStep::BuildCell);
        }
        QaStep::BuilderPlace(cell) => {
            qa.point=Some(builder_cell_screen(&s,Vec3::new(cell[0] as f32,cell[1] as f32+0.5,cell[2] as f32),window.width(),window.height()));
            qa.remaining=7;
            qa.active=Some(QaStep::BuildCell);
        }
        QaStep::FruitSlice => {
            qa.point = s
                .fruit
                .fruits
                .iter()
                .find(|f| !f.bomb && f.y > 0.18 && f.y < 0.9)
                .map(|f| Vec2::new(f.x * window.width(), f.y * window.height()));
            if qa.point.is_none() {
                qa.fail(&s, "尚无可切水果");
                return;
            }
            qa.remaining = 8;
            qa.active = Some(step);
        }
        QaStep::Wait(frames) => {
            qa.remaining = frames;
        }
        QaStep::WindowSize(w,h) => {window.resolution.set(w,h);qa.remaining=24;}
        QaStep::VisualFit => {
            let mut wanted=vec![Action::Library];
            match s.mode {
                GameMode::Island => {
                    wanted.extend([Action::Puzzle(PuzzleAction::Run),Action::Puzzle(PuzzleAction::Clear),Action::Puzzle(PuzzleAction::Repeat)]);
                    for op in [crate::puzzle_game::Op::Forward,crate::puzzle_game::Op::Left,crate::puzzle_game::Op::Right,crate::puzzle_game::Op::Wait]{wanted.push(Action::Puzzle(PuzzleAction::Add(op)));}
                },
                GameMode::CloudBridge => {use CloudAction::*;wanted.push(Action::CloudWalk(if s.cloud_walk.walking{Back}else{Enter}));for a in if s.cloud_walk.walking{vec![Move(-1,0),Move(0,-1),Move(0,1),Move(1,0)]}else{vec![Undo,Erase,Restart]}{wanted.push(Action::CloudWalk(a));}},
                GameMode::FruitArcade => {wanted.extend([Action::ArcadeRefill,Action::ArcadeClear,Action::ArcadeSpin]);for i in 0..5{wanted.push(Action::ArcadeBet(i));}},
                GameMode::BubbleBomber => {
                    let hearts=texts.iter().filter(|t|t.0=="♥"||t.0=="♡").count();
                    if hearts!=3 {qa.fail(&s,&format!("生命栏应为3槽，实际{hearts}"));return;}
                },
                GameMode::Snake => {
                    wanted.extend([Action::SnakeReset,Action::SnakeToggle,Action::SnakeMove(-1,0),Action::SnakeMove(1,0),Action::SnakeMove(0,-1),Action::SnakeMove(0,1)]);
                    if let Ok(board)=snake_boards.single() {let size=board.size();let viewport=Vec2::new(window.width(),window.height())*window.scale_factor();
                        // On narrow portrait screens the 18:24 board is width-limited;
                        // requiring 65% of screen height can exceed the maximum that fits.
                        let min_height = (viewport.y * 0.65).min(viewport.x * 24. / 18. * 0.9);
                        if (size.x/size.y-18./24.).abs()>0.002||size.y<min_height||size.x<viewport.x.min(viewport.y*0.64)*0.7 {qa.fail(&s,"贪吃蛇大棋盘比例或可玩面积不足");return;}
                    } else {qa.fail(&s,"没有接入贪吃蛇大棋盘");return;}
                },
                GameMode::SpaceCourier => {
                    let Ok(canvas)=courier_canvases.single() else {qa.fail(&s,"缺少快递员等比画布");return;};
                    if (canvas.size().x/canvas.size().y-1672./940.).abs()>0.002 {qa.fail(&s,"快递员主画布比例不正确");return;}
                },
                _=>{}
            }
            let screen=Rect::from_corners(Vec2::ZERO,Vec2::new(window.width(),window.height())*window.scale_factor());
            for action in wanted {
                let fully_visible=surfaces.iter().any(|(n,t,a,_,_,_,_,_,clip,_)| {
                    if a!=Some(&action){return false;}
                    let r=Rect::from_center_size(t.translation().truncate(),n.size());
                    let clipped=clip.map_or(r,|c|r.intersect(c.clip)).intersect(screen);
                    r.size().min_element()>1. && (r.size()-clipped.size()).abs().max_element()<2.
                });
                if !fully_visible {qa.fail(&s,&format!("按钮未完整显示：{action:?}"));return;}
            }
            qa.records.push(serde_json::json!({"check":"画布比例与按钮完整可见","passed":true}));
        }
        QaStep::ForestCheck(name, test) => {
            let passed=test(&forest);
            qa.records.push(serde_json::json!({"check":name,"passed":passed,"state":forest.qa_state()}));
            if !passed {qa.fail(&s,name);}
        }
        QaStep::Check(name, test) => {
            let passed = test(&s);
            let state = snapshot(&s);
            qa.records
                .push(serde_json::json!({"check": name, "passed": passed, "state": state}));
            if !passed {
                qa.fail(&s, name);
            }
        }
        QaStep::Help => {
            if available(Action::Help) {
                qa.steps.push_front(QaStep::Click(Action::CloseHelp));
                qa.steps.push_front(QaStep::Shot("help"));
                qa.steps
                    .push_front(QaStep::Check("说明可打开", |s| s.help_open));
                qa.steps.push_front(QaStep::Click(Action::Help));
            }
        }
        QaStep::Library => {
            if available(Action::Library) {
                qa.steps.push_front(QaStep::Click(Action::Library));
            } else if s.mode == GameMode::BubbleBomber && available(Action::BomberTogglePause) {
                qa.steps.push_front(QaStep::Click(Action::Library));
                qa.steps
                    .push_front(QaStep::Click(Action::BomberTogglePause));
            } else if available(Action::ToggleIslandMenu) {
                qa.steps.push_front(QaStep::Click(Action::Library));
                qa.steps.push_front(QaStep::Click(Action::ToggleIslandMenu));
            } else if available(Action::Help) {
                qa.steps.push_front(QaStep::Click(Action::Library));
                qa.steps.push_front(QaStep::Click(Action::Help));
            } else {
                qa.fail(&s, "找不到返回游戏库入口");
            }
        }
        QaStep::SelectFromLibrary => {
            let wanted = qa.mode.unwrap();
            if !s.library_open && !s.magnet_modes_open {
                let valid = s.mode == wanted;
                qa.records
                    .push(serde_json::json!({"check": "游戏库滚动选卡重新进入", "passed": valid}));
                if !valid {
                    qa.fail(&s, "游戏库选中了另一款游戏");
                }
            } else {
                qa.select_attempts += 1;
                if qa.select_attempts > 35 {
                    qa.fail(&s, "无法从游戏库选中目标");
                    return;
                }
                qa.steps.push_front(QaStep::SelectFromLibrary);
                if available(Action::SelectGame(wanted)) {
                    qa.steps
                        .push_front(QaStep::Click(Action::SelectGame(wanted)));
                } else if matches!(
                    wanted,
                    GameMode::Island
                        | GameMode::CloudBridge
                        | GameMode::FreeBuild
                        | GameMode::CrystalGarden
                ) && s.library_open
                    && available(Action::SelectGame(GameMode::Island))
                {
                    qa.steps
                        .push_front(QaStep::Click(Action::SelectGame(GameMode::Island)));
                } else if let Ok((node, t)) = cards.single() {
                    let cursor = t.translation().truncate() / window.scale_factor();
                    window.set_cursor_position(Some(cursor));
                    let target_y = surfaces.iter().find_map(|(_, t, a, ..)| {
                        (a == Some(&Action::SelectGame(wanted))).then_some(t.translation().y)
                    });
                    let direction = if target_y.is_some_and(|y| y < t.translation().y) {
                        1.
                    } else {
                        -1.
                    };
                    wheels.write(MouseWheel {
                        unit: MouseScrollUnit::Pixel,
                        x: 0.,
                        y: direction * node.size().y * 0.65,
                        window: window_entity,
                    });
                    qa.remaining = 5;
                } else {
                    qa.fail(&s, "没有可滚动的游戏库区域");
                }
            }
        }
        QaStep::Shot(_) => {
            qa.active = Some(step);
        }
    }
    qa.save();
}
fn action_point(surfaces: &Surfaces, window: &Window, wanted: Action) -> Option<Vec2> {
    surfaces
        .iter()
        .find_map(|(n, t, a, _, _, _, _, _, clip, _)| {
            let rect = Rect::from_center_size(t.translation().truncate(), n.size());
            let rect = clip.map_or(rect, |clip| rect.intersect(clip.clip));
            let rect = rect.intersect(Rect::from_corners(
                Vec2::ZERO,
                Vec2::new(window.width(), window.height()) * window.scale_factor(),
            ));
            let p = rect.center() / window.scale_factor();
            (a == Some(&wanted)
                && rect.size().min_element() > 1.
                && p.x > 0.
                && p.y > 0.
                && p.x < window.width()
                && p.y < window.height())
            .then_some(p)
        })
}
fn area_point(surfaces: &Surfaces, w: &Window, area: Area, point: Vec2) -> Option<Vec2> {
    if matches!(area, Area::Forest) { return Some(forest_screen_point(point,Vec2::new(w.width(),w.height()))); }
    if matches!(area, Area::Window) {
        return Some(point * Vec2::new(w.width(), w.height()));
    }
    surfaces.iter().find_map(
        |(n, t, _, b, stick, fire, bubble, garden, _, bomber_fire)| {
            let found = match area {
                Area::Bomber => b.is_some(),
                Area::Tank => stick.is_some(),
                Area::BomberFire => bomber_fire.is_some(),
                Area::Fire => fire.is_some(),
                Area::Bubble => bubble.is_some(),
                Area::Garden => garden.is_some(),
                _ => false,
            };
            (found && n.size().min_element() > 1.).then_some(
                (t.translation().truncate() + (point - Vec2::splat(0.5)) * n.size())
                    / w.scale_factor(),
            )
        },
    )
}
impl Qa {
    fn save(&self) {
        let result = serde_json::json!({"game": self.mode.map(|m| m.title()), "failed": self.failed,
            "completed": self.exit_frame.is_some(), "frame":self.frame, "remaining":self.remaining, "records": self.records});
        let _ = fs::create_dir_all(&self.root);
        let _ = fs::write(
            self.root.join("result.json"),
            serde_json::to_vec_pretty(&result).unwrap(),
        );
    }
    fn fail(&mut self, s: &Session, message: &str) {
        eprintln!("GAME_QA {} FAIL: {message} {}", s.mode.title(), snapshot(s));
        self.failed = true;
        self.records
            .push(serde_json::json!({"failure": message, "state": snapshot(s)}));
        self.exit_frame = Some(self.frame + 15);
        self.active = Some(QaStep::Shot("failure"));
        self.save();
    }
}
fn observe(
    mut qa: ResMut<Qa>,
    mut commands: Commands,
    mut exit: EventWriter<bevy::app::AppExit>,
    s: Res<Session>,
) {
    if let Some(QaStep::Shot(name)) = qa.active.clone() {
        qa.active = None;
        commands
            .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(
                qa.root.join(format!("{name}.png")),
            ));
    }
    if qa.exit_frame.is_some_and(|frame| qa.frame >= frame) {
        qa.save();
        eprintln!(
            "GAME_QA {} {}",
            s.mode.title(),
            if qa.failed { "FAIL" } else { "PASS" }
        );
        exit.write(if qa.failed {
            bevy::app::AppExit::error()
        } else {
            bevy::app::AppExit::Success
        });
    }
}
fn snapshot(s: &Session) -> serde_json::Value {
    use serde_json::json;
    if s.mode==GameMode::CloudBridge{let g=&s.cloud_walk;return json!({"stage":g.stage+1,"bridges":g.plan.bridges,"robot":g.map.cells[g.robot].at,"walking":g.walking,"won":g.won,"gem":g.gem_taken,"stars":g.progress.stars,"message":g.message});}
    if builder_mode(s) {let g=&s.builder_puzzles[puzzle_index(s)];return json!({"stage":g.level.number,"bridges":g.plan.bridges,"cards":g.plan.cards,"remaining_wood":g.remaining(crate::puzzle_game::Bridge::Wood),"robot":g.robot,"gate_open":g.gate_open,"pc":g.pc,"substep":g.substep,"won":g.won,"bonus":g.bonus,"message":g.message});}
    match s.mode {
        GameMode::Island
        | GameMode::CloudBridge
        | GameMode::FreeBuild
        | GameMode::CrystalGarden => {
            json!({"blocks": s.work.world.blocks.len(), "undo": s.undo.len(), "message": s.message, "level": s.work.level})
        }
        GameMode::Ball2048 => {
            json!({"lane": s.ball.lane, "score": s.ball.score, "running": s.ball.running})
        }
        GameMode::Tetris => {
            json!({"x": s.tetris.x, "rotation": s.tetris.rotation, "score": s.tetris.score})
        }
        GameMode::Firefighters | GameMode::Detective | GameMode::SpaceFarm => {
            json!({"stage": s.mission_step, "done": s.mission_done, "wrong": s.mission_wrong, "feedback": s.mission_feedback})
        }
        GameMode::TankBattle => {
            json!({"player": s.tank.player, "bullets": s.tank.bullets.len(), "wave": s.tank.wave})
        }
        GameMode::PixelAssault => json!({"x": s.assault.player_x, "camera": s.assault.camera_x,
            "score": s.assault.score, "bullets": s.assault.bullets.len(), "feedback": s.assault.feedback}),
        GameMode::SkyRaid => json!({"x": s.sky.x, "score": s.sky.score, "lives": s.sky.lives}),
        GameMode::BubbleIsland => {
            json!({"shots": s.bubble.shot_count, "score": s.bubble.score, "aim": [s.bubble.aim.x, s.bubble.aim.y]})
        }
        GameMode::BubbleBomber => {
            json!({"player": s.bomber.player, "paused": s.bomber.paused, "bombs": s.bomber.bombs.len(), "lives": s.bomber.lives})
        }
        GameMode::FruitNinja => json!({"score": s.fruit.score, "lives": s.fruit.lives}),
        GameMode::Fortune => s.fortune.qa_state(),
        GameMode::Slots => s.slots.qa_state(),
        GameMode::FruitArcade => s.arcade.qa_state(),
        GameMode::HoneyHunt => s.honey.qa_state(),
        GameMode::Fishing => json!({"coins": s.fishing.coins, "catches": s.fishing.catches}),
        GameMode::Snake => {
            json!({"head": s.snake.body[0], "score": s.snake.score, "running": s.snake.running, "over": s.snake.over})
        }
        GameMode::Minesweeper => {
            json!({"revealed": s.minesweeper.revealed.iter().filter(|v| **v).count(), "won": s.minesweeper.won, "lost": s.minesweeper.lost})
        }
        GameMode::Sokoban => {
            json!({"level": s.sokoban.level, "moves": s.sokoban.moves, "won": s.sokoban.won})
        }
        GameMode::GoldRunner => json!({"position":[s.gold_runner.qa_position().x,s.gold_runner.qa_position().y],"holes":s.gold_runner.qa_holes(),"gold":s.gold_runner.collected,"level":s.gold_runner.level,"lives":s.gold_runner.lives,"state":format!("{:?}",s.gold_runner.state)}),
        GameMode::LightPath => s.light_path.qa_state(),
        GameMode::RabbitHome => json!({
            "progress": s.rabbit_home.progress(),
            "map": s.rabbit_home.map_name(),
            "stage": s.rabbit_home.level(),
            "world_position": s.rabbit_home.qa_position(),
            "camera_position": s.rabbit_home.qa_camera(),
            "hearts": s.rabbit_home.lives(),
            "carrots": s.rabbit_home.collected_count(),
            "combo": s.rabbit_home.combo(),
            "outcome": s.rabbit_home.outcome(),
        }),
        GameMode::GardenDefense => {
            json!({"stage": s.garden.stage, "plants": s.garden.plants.len(), "ready": s.garden.ready, "paused": s.garden.paused})
        }
        GameMode::SpaceCourier => serde_json::to_value(&s.courier).unwrap(),
        _ => json!({}),
    }
}


fn forest_scenario() -> VecDeque<QaStep> {
    use QaStep::*;
    fn point(p: &mut VecDeque<QaStep>, x:f32,y:f32) { p.push_back(Point(Area::Forest,Vec2::new(x,y),Vec2::new(x,y),1)); }
    let mut p=VecDeque::from([Wait(40),Shot("initial")]);
    point(&mut p,612.,432.);
    p.push_back(ForestCheck("点击岔道切到环线",|g|g.qa_state()["switches"][0]==true));
    point(&mut p,612.,432.);
    point(&mut p,1460.,877.);
    p.push_back(Wait(110));
    p.push_back(ForestCheck("两车抢桥触发冲突",|g|g.qa_state()["phase"]=="Failed"));
    p.push_back(Shot("collision"));
    point(&mut p,1165.,877.);
    p.push_back(ForestCheck("撤回发车恢复本轮",|g|g.qa_state()["phase"]=="Ready"&&g.qa_state()["delivered"]==0));
    for wave in 0..3 {
        if wave==1 { point(&mut p,1126.,432.); }
        point(&mut p,220.,205.);
        p.push_back(ForestCheck("点击兔子村安排红车等待",|g|g.qa_state()["held"][0]==true));
        point(&mut p,1460.,877.);
        if wave==0 {
            p.push_back(Wait(20));p.push_back(Shot("moving"));
            point(&mut p,1615.,40.);
            p.push_back(ForestCheck("点击右上暂停",|g|g.qa_state()["phase"]=="Paused"));
            p.push_back(Wait(30));p.push_back(Shot("paused"));
            point(&mut p,1615.,40.);
        }
        p.push_back(Wait(170));
        p.push_back(ForestCheck("蓝车独立送达兔子村",|g|g.qa_state()["delivered"].as_u64().unwrap()%2==1&&g.qa_state()["phase"]=="Running"));
        if wave!=1 {point(&mut p,1126.,432.);}
        point(&mut p,220.,205.);
        p.push_back(Wait(175));
        p.push_back(ForestCheck("本轮两车全部正确送达",|g|g.qa_state()["delivered"].as_u64().unwrap()>0&&g.qa_state()["delivered"].as_u64().unwrap()%2==0&&g.qa_state()["phase"]!="Failed"));
        p.push_back(Shot(if wave==0{"round-one"}else if wave==1{"round-two"}else{"won"}));
    }
    p.push_back(ForestCheck("六次送达通关",|g|g.qa_state()["delivered"]==6&&g.qa_state()["phase"]=="Won"));
    point(&mut p,220.,875.);
    p.push_back(ForestCheck("重新开始恢复初始状态",|g|g.qa_state()["delivered"]==0&&g.qa_state()["phase"]=="Ready"));
    point(&mut p,57.,40.);p.push_back(Wait(20));
    p.push_back(Check("返回游戏库",|s|s.library_open));p.push_back(Shot("library"));
    p.push_back(SelectFromLibrary);p.push_back(Wait(20));p.push_back(Shot("returned"));
    p
}
