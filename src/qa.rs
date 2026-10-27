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
            std::time::Duration::from_secs_f64(if std::env::var_os("MAGNET_RABBIT_LONG_QA").is_some() {1. / 20.} else {1. / 30.}),
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
fn resize(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(size) = std::env::var("MAGNET_QA_SIZE") {
        if let Some((w, h)) = size
            .split_once('x')
            .and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?)))
        {
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
    Bomber,
    BomberFire,
    Tank,
    Fire,
    Bubble,
    Garden,
}
#[derive(Clone)]
enum QaStep {
    Click(Action),
    Optional(Action),
    Point(Area, Vec2, Vec2, u32),
    Keys(Vec<KeyCode>, u32),
    RabbitRunUntil(f32),
    BuildCell,
    FruitSlice,
    Wait(u32),
    Check(&'static str, fn(&Session) -> bool),
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
    let mut p = VecDeque::from([QaStep::Wait(35), QaStep::Shot("initial")]);
    match s.mode {
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild => {
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
                click(&mut p, Run);
                check(&mut p, "运行指令", |s| s.runner.is_some());
                click(&mut p, Build);
                check(&mut p, "搭建世界有效", |s| s.work.world.validate());
            } else {
                check(&mut p, "自动保存", |s| {
                    s.work.world.validate() && save_directory().join("island-v1.json").exists()
                });
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
            click(&mut p, ArcadeBet(0));
            click(&mut p, ArcadeBet(1));
            click(&mut p, ArcadeSpin);
            check(&mut p, "跑灯启动并扣分", |s| {
                s.arcade.qa_state()["running"] == true && s.arcade.qa_state()["points"] == 980
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
                !s.snake.running && s.snake.body[0] == (5, 6)
            });
            click(&mut p, SnakeMove(1, 0));
            p.push_back(QaStep::Wait(12));
            check(&mut p, "吃果实增长", |s| {
                s.snake.score == 1 && s.snake.body.len() == 4
            });
            click(&mut p, SnakeToggle);
            check(&mut p, "暂停", |s| !s.snake.running && !s.snake.over);
            p.push_back(QaStep::Shot("played"));
            click(&mut p, SnakeReset);
            check(&mut p, "重新开始", |s| {
                s.snake.score == 0 && s.snake.body.len() == 3 && !s.snake.running
            });
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
    p.push_back(QaStep::Help);
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
    p.push_back(QaStep::Library);
    p.push_back(QaStep::Wait(15));
    p.push_back(QaStep::SelectFromLibrary);
    p.push_back(QaStep::Shot("reentered"));
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
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    surfaces: Surfaces,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    cards: Query<(&ComputedNode, &GlobalTransform), With<LibraryCards>>,
    mut wheels: EventWriter<MouseWheel>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
) {
    qa.frame += 1;
    for key in [KeyCode::ArrowRight, KeyCode::ArrowLeft, KeyCode::Space] {
        let wanted = matches!(&qa.active, Some(QaStep::Keys(held, _)) if qa.remaining > 0 && held.contains(&key));
        if !wanted { keys.release(key); }
    }
    if qa.remaining <= 5
        || !matches!(
            qa.active,
            Some(
                QaStep::Click(_)
                    | QaStep::Optional(_)
                    | QaStep::Point(..)
                    | QaStep::BuildCell
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
            Some(QaStep::Click(a)) | Some(QaStep::Optional(a)) if qa.remaining >= 5 => {
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
            "completed": self.exit_frame.is_some(), "records": self.records});
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
