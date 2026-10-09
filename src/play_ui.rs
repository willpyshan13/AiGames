//! Animated teaching scenes and feedback shared by the children's games.
use super::*;

pub(super) struct MissionQuestion {
    pub prompt: &'static str,
    pub choices: [&'static str; 3],
    pub correct: usize,
    pub hint: &'static str,
    pub success: &'static str,
}

pub(super) fn mission_question(mode: GameMode, step: usize) -> MissionQuestion {
    let (prompt, choices, hint, success) = match (mode, step % 4) {
        (GameMode::Firefighters, 0) => (
            "小猫被困在二楼，选工具帮它下来",
            ["云梯", "气球", "滑板"],
            "需要能从地面通到窗台的工具。",
            "云梯升起来了，小猫有路下楼啦！",
        ),
        (GameMode::Firefighters, 1) => (
            "小消防员准备灭火，接上哪件工具？",
            ["水管", "画笔", "风筝"],
            "想一想，哪件工具能喷出水？",
            "水流喷出来，火苗变小了！",
        ),
        (GameMode::Firefighters, 2) => (
            "前方有路障，消防车怎么通过？",
            ["绕路", "撞过去", "原地等待"],
            "旁边有空路，安全地从旁边走。",
            "消防车绕开了路障，顺利赶到！",
        ),
        (GameMode::Firefighters, _) => (
            "小猫获救了，接下来送到哪里？",
            ["安全区", "火场里", "屋顶上"],
            "找有绿色标志、远离火的地方。",
            "小猫平安到达，救援任务完成！",
        ),
        (GameMode::Detective, 0) => (
            "展品不见了，地上什么能告诉我们方向？",
            ["脚印", "糖果", "帽子"],
            "走路会在地上留下它。",
            "放大镜找到了脚印，跟着它走！",
        ),
        (GameMode::Detective, 1) => (
            "线索卡写着“蓝色展厅”，去哪儿找？",
            ["蓝色展厅", "厨房", "花园"],
            "看看场景中蓝色的门。",
            "蓝色展厅打开了，线索就在里面！",
        ),
        (GameMode::Detective, 2) => (
            "展柜旁有一根羽毛，谁来过这里？",
            ["小鸟", "小鱼", "小熊"],
            "它有翅膀，会在天上飞。",
            "小鸟飞来了，它帮我们找到了花瓶！",
        ),
        (GameMode::Detective, _) => (
            "找到了花瓶，放到哪里才合适？",
            ["展柜里", "背包里", "地板上"],
            "展品有自己的展示位置。",
            "花瓶回到展柜，小侦探破案啦！",
        ),
        (GameMode::SpaceFarm, 0) => (
            "小苗想长高，给它什么？",
            ["阳光", "冰块", "石头"],
            "天空中暖暖亮亮的是什么？",
            "阳光照下来，小苗舒展开叶子！",
        ),
        (GameMode::SpaceFarm, 1) => (
            "泥土干干的，用什么照顾小苗？",
            ["浇水", "吹风", "关门"],
            "小苗也会口渴，给泥土喝点水。",
            "小水滴落下来，植物长高了！",
        ),
        (GameMode::SpaceFarm, 2) => (
            "我们的太空番茄成熟后是什么颜色？",
            ["红色", "透明", "灰色"],
            "看看枝头慢慢变红的果实。",
            "番茄变红啦，可以收获了！",
        ),
        _ => (
            "番茄收获了，可以用来做什么？",
            ["制作补给", "扔掉", "藏起来"],
            "宇航员工作后需要吃东西。",
            "收获装进补给箱，太空农场完成！",
        ),
    };
    let mut choices = choices;
    let rotation = (step + 1) % 3;
    choices.rotate_left(rotation);
    MissionQuestion {
        prompt,
        choices,
        correct: (3 - rotation) % 3,
        hint,
        success,
    }
}

#[derive(Component)]
pub(super) struct DropIn {
    base: Vec3,
    age: f32,
    delay: f32,
}
impl DropIn {
    pub(super) fn new(base: Vec3, delay: f32) -> Self {
        Self {
            base,
            age: 0.,
            delay,
        }
    }
}

#[derive(Component)]
pub(super) struct PianoNote;

#[derive(Component)]
pub(super) struct SceneMotion {
    base: Vec2,
    amount: Vec2,
    speed: f32,
    phase: f32,
    falling: bool,
}

impl SceneMotion {
    pub(super) fn bob(base: Vec2) -> Self {
        Self {
            base,
            amount: Vec2::new(0., 2.5),
            speed: 2.8,
            phase: base.x * 0.05,
            falling: false,
        }
    }
}

#[derive(Component)]
pub(super) struct Confetti(usize);

fn shape(
    p: &mut ChildSpawnerCommands,
    rect: [f32; 4],
    color: Color,
    round: f32,
    motion: Option<(Vec2, f32, f32, bool)>,
) {
    let [x, y, w, h] = rect;
    let mut e = p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(w),
            height: Val::Px(h),
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::all(Val::Px(round)),
    ));
    if let Some((amount, speed, phase, falling)) = motion {
        e.insert(SceneMotion {
            base: Vec2::new(x, y),
            amount,
            speed,
            phase,
            falling,
        });
    }
}

fn cat(p: &mut ChildSpawnerCommands, x: f32, y: f32, rescued: bool) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(48.),
            height: Val::Px(50.),
            ..default()
        },
        SceneMotion {
            base: Vec2::new(x, y),
            amount: Vec2::new(
                if rescued { 8. } else { 0. },
                if rescued { 12. } else { 3. },
            ),
            speed: if rescued { 5. } else { 2. },
            phase: 0.,
            falling: false,
        },
    ))
    .with_children(|p| {
        let fur = Color::srgb(1., 0.72, 0.34);
        shape(p, [5., 10., 38., 34.], fur, 14., None);
        shape(p, [5., 0., 12., 22.], fur, 5., None);
        shape(p, [31., 0., 12., 22.], fur, 5., None);
        for x in [14., 30.] {
            shape(p, [x, 22., 5., 6.], NAVY, 3., None);
        }
        shape(
            p,
            [22., 32., 6., 4.],
            Color::srgb(0.92, 0.32, 0.36),
            3.,
            None,
        );
    });
}

fn mission_scene(p: &mut ChildSpawnerCommands, s: &Session, art: &Art) {
    let done = s.mission_done;
    let progress = s.mission_step + usize::from(done);
    p.spawn((
        Node {
            width: Val::Px(480.),
            height: Val::Px(222.),
            min_height: Val::Px(222.),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Color::srgb(0.80, 0.93, 0.97)),
        BorderRadius::all(Val::Px(20.)),
    ))
    .with_children(|p| {
        shape(
            p,
            [0., 188., 480., 34.],
            Color::srgb(0.52, 0.77, 0.57),
            0.,
            None,
        );
        match s.mode {
            GameMode::Firefighters => {
                shape(
                    p,
                    [290., 44., 115., 144.],
                    Color::srgb(0.91, 0.71, 0.53),
                    8.,
                    None,
                );
                shape(
                    p,
                    [315., 70., 52., 50.],
                    Color::srgb(0.31, 0.44, 0.56),
                    6.,
                    None,
                );
                if progress < 2 {
                    for i in 0..5 {
                        shape(
                            p,
                            [298. + i as f32 * 21., 144., 22., 45.],
                            Color::srgb(1., 0.36 + i as f32 * 0.03, 0.14),
                            14.,
                            Some((Vec2::new(2., 10.), 7., i as f32, false)),
                        );
                    }
                }
                if progress >= 1 {
                    for x in [265., 284.] {
                        shape(p, [x, 91., 5., 99.], PAPER, 2., None);
                    }
                    for y in [105., 125., 145., 165.] {
                        shape(p, [265., y, 24., 4.], PAPER, 2., None);
                    }
                }
                p.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(45.),
                        top: Val::Px(143.),
                        ..default()
                    },
                    SceneMotion {
                        base: Vec2::new(if progress >= 3 { 145. } else { 45. }, 143.),
                        amount: Vec2::new(if progress >= 3 { 30. } else { 5. }, 2.),
                        speed: 1.5,
                        phase: 0.,
                        falling: false,
                    },
                ))
                .with_children(|p| {
                    shape(
                        p,
                        [0., 0., 102., 39.],
                        Color::srgb(0.90, 0.26, 0.23),
                        8.,
                        None,
                    );
                    shape(
                        p,
                        [70., -20., 32., 32.],
                        Color::srgb(0.93, 0.36, 0.27),
                        6.,
                        None,
                    );
                    shape(
                        p,
                        [77., -14., 18., 18.],
                        Color::srgb(0.76, 0.93, 0.98),
                        4.,
                        None,
                    );
                    for x in [15., 74.] {
                        shape(p, [x, 28., 24., 24.], NAVY, 12., None);
                    }
                });
                if progress >= 2 {
                    for i in 0..10 {
                        shape(
                            p,
                            [170., 110. + i as f32 * 3., 8., 5.],
                            Color::srgb(0.18, 0.66, 0.96),
                            4.,
                            Some((Vec2::new(160., 24.), 1.7, i as f32 / 10., true)),
                        );
                    }
                }
                cat(
                    p,
                    if progress >= 4 { 420. } else { 317. },
                    if progress >= 4 { 139. } else { 70. },
                    done,
                );
            }
            GameMode::Detective => {
                shape(
                    p,
                    [324., 30., 100., 155.],
                    Color::srgb(0.26, 0.53, 0.86),
                    12.,
                    None,
                );
                shape(
                    p,
                    [345., 50., 58., 110.],
                    Color::srgb(0.57, 0.76, 0.97),
                    6.,
                    None,
                );
                shape(
                    p,
                    [70., 157., 90., 31.],
                    Color::srgb(0.57, 0.40, 0.67),
                    4.,
                    None,
                );
                if progress >= 3 {
                    shape(
                        p,
                        [96., 102., 38., 55.],
                        Color::srgb(0.98, 0.74, 0.28),
                        17.,
                        Some((Vec2::new(0., 6.), 3., 0., false)),
                    );
                    shape(
                        p,
                        [105., 90., 20., 22.],
                        Color::srgb(0.98, 0.74, 0.28),
                        4.,
                        Some((Vec2::new(0., 6.), 3., 0., false)),
                    );
                }
                for i in 0..6 {
                    shape(
                        p,
                        [185. + i as f32 * 22., 158. + (i % 2) as f32 * 9., 11., 18.],
                        Color::srgb(0.48, 0.39, 0.50),
                        7.,
                        Some((Vec2::new(0., 3.), 2., i as f32, false)),
                    );
                }
                p.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(180.),
                        top: Val::Px(86.),
                        ..default()
                    },
                    SceneMotion {
                        base: Vec2::new(180., 86.),
                        amount: Vec2::new(45., 8.),
                        speed: 1.8,
                        phase: 0.,
                        falling: false,
                    },
                ))
                .with_children(|p| {
                    shape(p, [0., 0., 56., 56.], NAVY, 28., None);
                    shape(
                        p,
                        [7., 7., 42., 42.],
                        Color::srgb(0.74, 0.93, 0.97),
                        21.,
                        None,
                    );
                    shape(p, [43., 43., 11., 34.], NAVY, 5., None);
                });
                if progress >= 2 {
                    shape(
                        p,
                        [360., 10., 30., 21.],
                        Color::srgb(0.95, 0.63, 0.24),
                        12.,
                        Some((Vec2::new(32., 8.), 3., 0., false)),
                    );
                }
            }
            _ => {
                shape(
                    p,
                    [355., 22., 54., 54.],
                    Color::srgb(1., 0.80, 0.26),
                    27.,
                    Some((Vec2::new(4., 8.), 1.5, 0., false)),
                );
                shape(
                    p,
                    [145., 172., 170., 29.],
                    Color::srgb(0.58, 0.36, 0.23),
                    12.,
                    None,
                );
                let height = 35. + progress as f32 * 19.;
                shape(
                    p,
                    [224., 172. - height, 9., height],
                    Color::srgb(0.25, 0.57, 0.29),
                    5.,
                    None,
                );
                for i in 0..(progress + 2) {
                    let left = i % 2 == 0;
                    shape(
                        p,
                        [
                            if left { 188. } else { 229. },
                            144. - i as f32 * 17.,
                            38.,
                            17.,
                        ],
                        Color::srgb(0.30, 0.71, 0.37),
                        12.,
                        Some((Vec2::new(3., 3.), 2., i as f32, false)),
                    );
                }
                if progress >= 2 {
                    for i in 0..7 {
                        shape(
                            p,
                            [175. + i as f32 * 17., 26., 6., 12.],
                            Color::srgb(0.20, 0.60, 0.92),
                            4.,
                            Some((Vec2::new(0., 140.), 0.7, i as f32 / 7., true)),
                        );
                    }
                }
                if progress >= 3 {
                    for x in [196., 240.] {
                        shape(
                            p,
                            [x, 113., 26., 28.],
                            Color::srgb(0.94, 0.29, 0.27),
                            14.,
                            Some((Vec2::new(2., 5.), 2.5, x, false)),
                        );
                    }
                }
                if progress >= 4 {
                    shape(
                        p,
                        [355., 150., 64., 38.],
                        Color::srgb(0.90, 0.64, 0.32),
                        7.,
                        Some((Vec2::new(0., 5.), 3., 0., false)),
                    );
                }
            }
        }
        p.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(14.),
            top: Val::Px(12.),
            ..default()
        })
        .with_children(|p| {
            label(
                p,
                art,
                if done {
                    "做到了！看看发生了什么"
                } else {
                    "观察场景，点下面的工具"
                },
                14.,
                NAVY,
            )
        });
    });
}

pub(super) fn mission_ui(commands: &mut Commands, s: &Session, art: &Art) {
    let task = mission_question(s.mode, s.mission_step);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.09, 0.18, 0.25)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(860.),
                    max_width: Val::Percent(94.),
                    padding: UiRect::all(Val::Px(16.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(6.),
                    ..default()
                },
                BackgroundColor(PAPER),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|p| {
                label(
                    p,
                    art,
                    format!("{} · 任务 {}/4", s.mode.title(), s.mission_step + 1),
                    24.,
                    TEAL,
                );
                mission_scene(p, s, art);
                label(p, art, task.prompt, 22., INK);
                p.spawn(row()).with_children(|p| {
                    if s.mission_done {
                        button(
                            p,
                            art,
                            if s.mission_step == 3 {
                                "全部完成！再玩一次"
                            } else {
                                "下一步 →"
                            },
                            Action::MissionNext,
                            260.,
                            52.,
                            MINT,
                            NAVY,
                        );
                    } else {
                        for (i, choice) in task.choices.iter().enumerate() {
                            button(
                                p,
                                art,
                                *choice,
                                Action::MissionChoice(i),
                                190.,
                                52.,
                                if s.mission_wrong == Some(i) {
                                    Color::srgb(1., 0.85, 0.77)
                                } else {
                                    Color::srgb(0.80, 0.89, 0.97)
                                },
                                NAVY,
                            );
                        }
                    }
                });
                p.spawn(row()).with_children(|p| {
                    if !s.mission_done {
                        button(
                            p,
                            art,
                            "给我提示",
                            Action::MissionHint,
                            120.,
                            44.,
                            PAPER,
                            TEAL,
                        );
                    }
                    button(
                        p,
                        art,
                        "怎么玩 / 暂停",
                        Action::Help,
                        150.,
                        44.,
                        PAPER,
                        TEAL,
                    );
                });
                label(
                    p,
                    art,
                    if s.mission_feedback.is_empty() {
                        "不限时 · 选错也没关系，会给你提示"
                    } else {
                        &s.mission_feedback
                    },
                    16.,
                    TEAL,
                );
            });
        });
}

pub(super) fn animate_play_ui(
    mut commands: Commands,
    time: Res<Time>,
    s: Res<Session>,
    scale: Res<UiScale>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut nodes: Query<(
        &mut Node,
        Option<&SceneMotion>,
        Option<&PianoNote>,
        Option<&Confetti>,
        Option<&mut BackgroundColor>,
    )>,
    mut initialized: Local<bool>,
    mut drops: Query<(&mut Transform, &mut DropIn)>,
) {
    if s.help_open {
        return;
    }
    for (mut transform, mut drop) in &mut drops {
        drop.age += time.delta_secs();
        let progress = ((drop.age - drop.delay) / 0.45).clamp(0., 1.);
        transform.translation = drop.base + Vec3::Y * (1. - progress).powi(2) * 2.4;
    }
    if !*initialized {
        for i in 0..24 {
            commands.spawn((
                Confetti(i),
                GlobalZIndex(100),
                bevy::ui::FocusPolicy::Pass,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(8.),
                    height: Val::Px(12.),
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(MINT),
            ));
        }
        *initialized = true;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let screen = Vec2::new(window.width(), window.height()) / scale.0;
    for (mut node, motion, piano, confetti, color) in &mut nodes {
        if let Some(m) = motion {
            let phase = time.elapsed_secs() * m.speed + m.phase;
            let factor = if m.falling {
                phase.fract()
            } else {
                phase.sin()
            };
            let pos = m.base + m.amount * factor;
            node.left = Val::Px(pos.x);
            node.top = Val::Px(pos.y);
        }
        if piano.is_some() {
            node.top = Val::Percent(8. + s.piano.fall / 3.2 * 66.);
        }
        if let Some(Confetti(index)) = confetti {
            node.display = if s.cheer > 0. && !s.library_open {
                Display::Flex
            } else {
                Display::None
            };
            let progress = (1.4 - s.cheer).max(0.);
            let angle = *index as f32 * 2.399;
            node.left = Val::Px(screen.x * 0.5 + angle.cos() * progress * 180.);
            node.top =
                Val::Px(screen.y * 0.4 + angle.sin() * progress * 130. + progress * progress * 65.);
            if let Some(mut color) = color {
                color.0 = [
                    MINT,
                    Color::srgb(1., 0.76, 0.25),
                    Color::srgb(0.97, 0.43, 0.47),
                    Color::srgb(0.40, 0.71, 0.98),
                ][*index % 4]
                    .with_alpha((s.cheer / 0.6).min(1.));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_mission_has_matching_choices_and_rotating_answer() {
        let expected = [
            ["云梯", "水管", "绕路", "安全区"],
            ["脚印", "蓝色展厅", "小鸟", "展柜里"],
            ["阳光", "浇水", "红色", "制作补给"],
        ];
        for (mode, answers) in [
            GameMode::Firefighters,
            GameMode::Detective,
            GameMode::SpaceFarm,
        ]
        .into_iter()
        .zip(expected)
        {
            for (step, expected) in answers.into_iter().enumerate() {
                let q = mission_question(mode, step);
                assert_eq!(q.choices[q.correct], expected);
                assert!(!q.hint.is_empty());
                assert_eq!(q.choices.iter().filter(|&&v| v == expected).count(), 1);
            }
        }
    }
}

pub(super) fn help_panel(commands: &mut Commands, s: &Session, art: &Art) {
    let (instruction, reminder) = match s.mode {
        GameMode::FruitArcade => (
            "1  点击下方水果，每次增加 10 点，可选多种\n2  点击启动，看亮灯绕圈减速\n3  停中已选水果，获得该水果投入 × 倍率",
            "20 个格子等概率停留，每种图案各 4 格。奖励含投入；未选中不返还。清空可重新分配，积分可免费补充。",
        ),
        GameMode::Slots => (
            "1  选择每次投入的虚拟积分\n2  点开始转动，或点右侧红色手柄\n3  等三列停下，看中间一行的图案",
            "樱桃、柠檬、葡萄、橙子、7 的三连奖励为投入的 4、5、6、8、12 倍；任意两枚相同返还 1 倍。积分可免费补充。",
        ),
        GameMode::Fortune => (
            "1  按住地图拖动，查看四周街区\n2  掷骰子，视角自动跟随角色\n3  空地可以买，自己的房子可以升级\n4  点「回到角色」恢复跟随",
            "每人初始 1200 金币。12 轮后比金币加房产的总资产；金币用完则提前结束。",
        ),
        GameMode::Math => (
            "1  读上方题目
2  点一个数字，马上知道对不对
3  不确定时点「给我提示」",
            "想慢慢数，可以展开「我想摆实物」。难度由你自己选择。",
        ),
        GameMode::Literacy => (
            "1  看左边的图和大字
2  点汉字作答
3  答错可以直接改选，答对再继续",
            "不计时。已经改对的题不用到最后再做一遍。",
        ),
        GameMode::MusicFactory => (
            "1  找亮起的彩色琴键
2  点琴键，或直接点落下的音符
3  跟着下一个亮键继续弹",
            "音符会等你。不用认识英文字母，也不用抢时间。",
        ),
        GameMode::BubbleIsland => (
            "1  按住棋盘，看看白色瞄准线
2  松手发射，连成三个同色就消除
3  也可以点「帮我瞄准」再点「发射」",
            "第一关不会下压。瞄准辅助只帮你选路线，不会自动发射。",
        ),
        GameMode::BubbleBomber => (
            "1  点身边的亮格走动，或按住方向键
2  点「放水泡」，看三秒倒计时
3  离开黄色危险格，躲过水花",
            "有五条命，复活后有护盾。看说明时，小怪和倒计时都会暂停。",
        ),
        GameMode::FruitNinja => (
            "1  按住屏幕，划过飞起来的水果
2  尽量一笔切过一整串，连击会加分
3  金色水果额外加 25 分，炸弹不能碰",
            "每次挥刀后的 1.15 秒内继续切，连击不会断。水果落地只扣一颗爱心；炸弹也会打断连击。",
        ),
        GameMode::Fishing => (
            "1  看准游过的鱼，直接点水面撒网
2  一张网能同时网住附近多条鱼
3  金鱼分数最高，捕到鱼会补回一些金币",
            "每张网消耗 2 金币。鱼会从两侧游过来；别急，等鱼群靠近再下网更划算。",
        ),
        GameMode::MoonFestival => (
            "1  点「摇一摇」，等待六颗骰子停稳
2  骰子里的红色四越多，获得的福气越多
3  五次免费机会结束后，可点「再来一局」重新开始",
            "这是离线趣味收藏小游戏。所有福礼只保存在本局里，不含真实兑换、奖品或付费内容。",
        ),
        _ => (
            "1  看动画，再读下面的小任务
2  点一个工具或答案
3  看发生了什么，再点「下一步」",
            "不用赶时间。拿不准就点「给我提示」，选错可以马上再试。",
        ),
    };
    commands
        .spawn((
            UiRoot,
            GlobalZIndex(200),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.10, 0.15, 0.92)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(740.),
                    max_width: Val::Percent(94.),
                    padding: UiRect::all(Val::Px(26.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.),
                    ..default()
                },
                BackgroundColor(PAPER),
                BorderRadius::all(Val::Px(24.)),
            ))
            .with_children(|p| {
                label(p, art, format!("{} · 已暂停", s.mode.title()), 25., TEAL);
                label(p, art, instruction, 23., INK);
                label(p, art, reminder, 17., MUTED);
                p.spawn(row()).with_children(|p| {
                    button(
                        p,
                        art,
                        "知道了，继续玩 →",
                        Action::CloseHelp,
                        260.,
                        56.,
                        MINT,
                        NAVY,
                    );
                    button(p, art, "回游戏库", Action::Library, 140., 56., PAPER, TEAL);
                });
            });
        });
}
