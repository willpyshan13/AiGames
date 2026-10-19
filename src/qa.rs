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
            std::time::Duration::from_secs_f64(1. / 30.),
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
        _ => panic!("use the existing storybook verification for this game"),
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
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild
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
) {
    qa.frame += 1;
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
                    GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild
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
        GameMode::Island | GameMode::CloudBridge | GameMode::FreeBuild => {
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
        GameMode::GardenDefense => {
            json!({"stage": s.garden.stage, "plants": s.garden.plants.len(), "ready": s.garden.ready, "paused": s.garden.paused})
        }
        GameMode::SpaceCourier => serde_json::to_value(&s.courier).unwrap(),
        _ => json!({}),
    }
}
