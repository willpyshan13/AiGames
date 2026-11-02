//! Runtime layers and controls positioned against the confirmed v2 artboards.
use super::builder_redesign::{
    bn, centered, command_icon, path_icon, sprite, stroke, text, toolbar_icon,
};
use super::*;
use crate::puzzle_game::{Bridge, Kind, Op, Progress, LEVEL_COUNT};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PuzzleAction {
    Cell(usize),
    Material(Bridge),
    Erase,
    Undo,
    Restart,
    Run,
    Step,
    Reset,
    Hint,
    Clear,
    Add(Op),
    Repeat,
    Select(usize),
    Build(bool),
    Speed,
    Stage(usize),
    Next,
}
pub(super) fn puzzle_index(s: &Session) -> usize {
    usize::from(s.mode == GameMode::Island)
}
pub(super) fn load_puzzle(kind: Kind) -> Puzzle {
    let path = puzzle_save_path(kind);
    let mut game = fs::read(path)
        .ok()
        .and_then(|b| serde_json::from_slice::<Progress>(&b).ok())
        .filter(|p| p.valid(kind))
        .map(Puzzle::restore)
        .unwrap_or_else(|| Puzzle::new(kind));
    game.message = if kind == Kind::Cloud {
        "比较两条路线，选建材再点蓝色缺口。"
    } else {
        "先补桥，再编排指令；点选卡片可调整重复。"
    }
    .into();
    game
}
fn puzzle_save_path(kind: Kind) -> PathBuf {
    save_directory().join(if kind == Kind::Cloud {
        "cloud-puzzle-v2.json"
    } else {
        "robot-puzzle-v2.json"
    })
}
pub(super) fn persist_puzzle(s: &mut Session) {
    if s.mode == GameMode::CloudBridge {
        persist_cloud_walk(s);
        return;
    }
    let index = puzzle_index(s);
    let game = &s.builder_puzzles[index];
    let mut path = puzzle_save_path(game.kind);
    let saved = game.snapshot();
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(save_directory())?;
        if let Ok(bytes) = fs::read(&path) {
            let previous = serde_json::from_slice::<Progress>(&bytes);
            let expanding = previous
                .as_ref()
                .is_ok_and(|p| p.valid(game.kind) && p.plans.len() < LEVEL_COUNT);
            if expanding || !previous.is_ok_and(|p| p.valid(game.kind)) {
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_millis();
                if fs::copy(
                    &path,
                    path.with_extension(format!(
                        "{}-{stamp}.json",
                        if expanding {
                            "before-expansion"
                        } else {
                            "unreadable"
                        }
                    )),
                )
                .is_err()
                {
                    path = path.with_extension(format!("recovery-{stamp}.json"));
                }
            }
        }
        let temp = path.with_extension("tmp");
        fs::write(&temp, serde_json::to_vec_pretty(&saved)?)?;
        fs::rename(temp, path)?;
        Ok(())
    })();
    if let Err(error) = result {
        s.builder_puzzles[index].message = format!("暂未保存，作品仍在当前游戏：{error}");
    }
}
pub(super) fn puzzle_action(s: &mut Session, a: PuzzleAction) {
    let game = &mut s.builder_puzzles[puzzle_index(s)];
    match a {
        PuzzleAction::Cell(n) => {
            if game.kind == Kind::Robot && !game.build {
                game.message = "先切到搭建，再点击缺口放桥。".into();
            } else {
                game.place(n);
            }
        }
        PuzzleAction::Material(b) => {
            game.material = b;
            game.erase = false;
            game.build = true;
            game.message = "已选择材料，点蓝色缺口放置。".into();
        }
        PuzzleAction::Erase => {
            game.erase = !game.erase;
            game.build = true;
            game.message = "点已放的桥块拆除，材料会退回。".into();
        }
        PuzzleAction::Undo => game.undo(),
        PuzzleAction::Restart => game.restart(),
        PuzzleAction::Run => game.run(),
        PuzzleAction::Step => {
            game.running = false;
            game.step();
        }
        PuzzleAction::Reset => {
            game.reset_run();
            game.message = "机器人与机关已复位，桥梁和指令保留。".into();
        }
        PuzzleAction::Hint => game.hint(),
        PuzzleAction::Clear => game.clear(),
        PuzzleAction::Add(op) => game.add(op),
        PuzzleAction::Repeat => game.repeat(),
        PuzzleAction::Select(i) => {
            game.selected = if game.selected == Some(i) {
                None
            } else {
                Some(i)
            };
            game.message = "点行动按钮替换这张卡，或点重复调整次数。".into();
        }
        PuzzleAction::Build(build) => {
            game.reset_run();
            game.build = build;
            game.erase = false;
        }
        PuzzleAction::Speed => {
            game.slow = !game.slow;
        }
        PuzzleAction::Stage(i) => game.choose(i),
        PuzzleAction::Next => game.choose(
            crate::puzzle_game::Level::index_for_number(
                game.kind,
                game.level.number % LEVEL_COUNT + 1,
            )
            .unwrap(),
        ),
    }
    if matches!(a, PuzzleAction::Stage(_) | PuzzleAction::Next) {
        s.island_menu_open = false;
    }
}
pub(super) struct PuzzleArt {
    pub(super) cloud: Handle<Image>,
    pub(super) island: Handle<Image>,
    actors: Handle<Image>,
    frames: Handle<Image>,
}
pub(super) fn load_puzzle_art(server: &AssetServer) -> PuzzleArt {
    let load = |name: &str| {
        server.load_with_settings(
            image_asset_path(&format!("builders/puzzle-v2/{name}.png")),
            |settings: &mut bevy::image::ImageLoaderSettings| {
                settings.sampler = bevy::image::ImageSampler::linear()
            },
        )
    };
    PuzzleArt {
        cloud: load("cloud-scene"),
        island: load("island-scene"),
        actors: load("actors"),
        frames: load("frames"),
    }
}
#[derive(Component)]
pub(super) struct PuzzleActor;
pub(super) fn animate_puzzle_actor(
    time: Res<Time>,
    s: Res<Session>,
    layout: Res<Layout>,
    scale: Res<UiScale>,
    mut actors: Query<&mut Node, With<PuzzleActor>>,
    mut position: Local<Option<(Kind, usize, Vec2)>>,
) {
    if !builder_mode(&s) || s.library_open {
        *position = None;
        return;
    }
    let game = &s.builder_puzzles[puzzle_index(&s)];
    let at = Vec2::from_array(game.level.cells[game.robot].point);
    let current = match *position {
        Some((kind, stage, old)) if kind == game.kind && stage == game.stage && game.started => {
            old.lerp(at, 1. - (-13. * time.delta_secs()).exp())
        }
        _ => at,
    };
    *position = Some((game.kind, game.stage, current));
    let (k, _) = builder_canvas(layout.width / scale.0, layout.height / scale.0);
    let [w, h] = if game.kind == Kind::Cloud {
        [76., 98.]
    } else {
        [55., 75.]
    };
    for mut node in &mut actors {
        *node = bn(current.x - w * 0.5, current.y - h, w, h, k);
    }
}
fn ink() -> Color {
    Color::srgb(0.07, 0.19, 0.23)
}
fn cream() -> Color {
    Color::srgb(1., 0.98, 0.93)
}
fn accent(cloud: bool) -> Color {
    if cloud {
        Color::srgb(0.01, 0.48, 1.)
    } else {
        Color::srgb(0., 0.64, 0.29)
    }
}
fn rounded(p: &mut ChildSpawnerCommands, r: [f32; 4], color: Color, k: f32) {
    p.spawn((
        bn(r[0], r[1], r[2], r[3], k),
        BackgroundColor(color),
        BorderRadius::all(Val::Px(22. * k)),
    ));
}
fn panel(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    r: [f32; 4],
    cloud: bool,
    part: usize,
    k: f32,
    f: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let mut node = p.spawn((
        WorldInputBlocker,
        bn(r[0], r[1], r[2], r[3], k),
        BackgroundColor(if cloud {
            Color::srgba(1., 1., 0.99, 0.96)
        } else {
            Color::NONE
        }),
        BorderRadius::all(Val::Px(32. * k)),
    ));
    if !cloud {
        let [x, y, right, bottom] = [
            [16., 7., 1663., 103.],
            [15., 108., 316., 325.],
            [1293., 107., 1670., 594.],
            [9., 590., 1666., 933.],
        ][part];
        node.insert(ImageNode {
            image: art.puzzle.frames.clone(),
            rect: Some(Rect::new(x, y, right, bottom)),
            ..default()
        });
    }
    node.with_children(f);
}
pub(super) fn button(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    title: &str,
    action: Action,
    r: [f32; 4],
    fill: Color,
    size: f32,
    light: bool,
    k: f32,
) {
    p.spawn((
        Button,
        action,
        ButtonTone(fill),
        Node {
            border: UiRect::all(Val::Px(1.5 * k)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..bn(r[0], r[1], r[2], r[3], k)
        },
        BackgroundColor(fill),
        BorderColor(if light {
            Color::srgba(1., 1., 1., 0.45)
        } else if matches!(action,Action::CloudWalk(CloudAction::Move(..))) {
            Color::srgb(0.56,0.78,0.93)
        } else {
            Color::srgba(0.70, 0.49, 0.28, 0.30)
        }),
        BorderRadius::all(Val::Px(20. * k)),
    ))
    .with_children(|p| {
        if light {
            rounded(
                p,
                [4., 3., r[2] - 8., 6.],
                Color::srgba(1., 1., 1., 0.17),
                k,
            );
        }
        let icon = match action {
            Action::Help => Some((Action::Help, "怎么玩")),
            Action::Puzzle(PuzzleAction::Erase) => Some((Action::Erase, "拆除")),
            Action::Puzzle(PuzzleAction::Undo) => Some((Action::Undo, "撤销")),
            Action::Puzzle(PuzzleAction::Restart) => Some((Action::Reset, "重来")),
            Action::Puzzle(PuzzleAction::Step) => Some((Action::Step, "单步")),
            Action::Puzzle(PuzzleAction::Reset) => Some((Action::Reset, "复位")),
            Action::Puzzle(PuzzleAction::Clear) => Some((Action::Clear, "清空")),
            Action::CloudWalk(CloudAction::Erase)=>Some((Action::Erase,"拆除")),
            Action::CloudWalk(CloudAction::Undo)=>Some((Action::Undo,"撤销")),
            Action::CloudWalk(CloudAction::Restart)=>Some((Action::Reset,"重来")),
            _ => None,
        };
        if let Some((symbol, name)) = icon {
            let icon_size = (r[3] - 22.).min(43.).min(r[2] * 0.27);
            toolbar_icon(
                p,
                art,
                symbol,
                [14., (r[3] - icon_size) * 0.5, icon_size, icon_size],
                if action == Action::Help {
                    Color::WHITE
                } else if action == Action::Puzzle(PuzzleAction::Erase) {
                    Color::srgb(0.92, 0.22, 0.34)
                } else if matches!(symbol, Action::Undo | Action::Reset) {
                    Color::srgb(0.04, 0.53, 0.96)
                } else {
                    ink()
                },
                k,
            );
            centered(
                p,
                art,
                name,
                [icon_size + 26., 0., r[2] - icon_size - 31., r[3]],
                size.min((r[2] - icon_size - 35.) / name.chars().count() as f32),
                if light { Color::WHITE } else { ink() },
                k,
            );
        } else if matches!(
            action,
            Action::Puzzle(PuzzleAction::Hint | PuzzleAction::Repeat | PuzzleAction::Speed)
        ) {
            let icon_r = [24., (r[3] - 40.) * 0.5, 40., 40.];
            match action {
                Action::Puzzle(PuzzleAction::Hint) => {
                    ring(p, [icon_r[0] + 7., icon_r[1] + 1., 26., 28.], ink(), k);
                    path_icon(p, &[(0.35, 0.93), (0.65, 0.93)], icon_r, ink(), k);
                }
                Action::Puzzle(PuzzleAction::Repeat) => {
                    let pink = Color::srgb(0.94, 0.12, 0.57);
                    path_icon(
                        p,
                        &[
                            (0.15, 0.58),
                            (0.15, 0.32),
                            (0.34, 0.12),
                            (0.65, 0.12),
                            (0.83, 0.32),
                            (0.83, 0.58),
                        ],
                        icon_r,
                        pink,
                        k,
                    );
                    path_icon(
                        p,
                        &[(0.15, 0.38), (0.02, 0.58), (0.15, 0.77), (0.29, 0.58)],
                        icon_r,
                        pink,
                        k,
                    );
                    path_icon(
                        p,
                        &[(0.83, 0.38), (0.7, 0.58), (0.83, 0.77), (0.97, 0.58)],
                        icon_r,
                        pink,
                        k,
                    );
                }
                _ => {
                    ring(p, icon_r, ink(), k);
                    path_icon(
                        p,
                        &[(0.5, 0.18), (0.5, 0.5), (0.74, 0.62)],
                        icon_r,
                        ink(),
                        k,
                    );
                    path_icon(
                        p,
                        &[(0.2, 0.3), (0.5, 0.62), (0.8, 0.3)],
                        [r[2] - 36., r[3] * 0.35, 22., 22.],
                        ink(),
                        k,
                    );
                }
            }
            centered(p, art, title, [75., 0., r[2] - 90., r[3]], size, ink(), k);
        } else {
            centered(
                p,
                art,
                title,
                [5., 0., r[2] - 10., r[3]],
                size,
                if light { Color::WHITE } else { ink() },
                k,
            );
        }
    });
}
fn action_button(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    title: &str,
    a: PuzzleAction,
    r: [f32; 4],
    fill: Color,
    size: f32,
    light: bool,
    k: f32,
) {
    button(p, art, title, Action::Puzzle(a), r, fill, size, light, k);
}
fn ring(p: &mut ChildSpawnerCommands, r: [f32; 4], color: Color, k: f32) {
    p.spawn((
        Node {
            border: UiRect::all(Val::Px(3. * k)),
            ..bn(r[0], r[1], r[2], r[3], k)
        },
        BorderColor(color),
        BorderRadius::MAX,
    ));
}
pub(super) fn actor(p: &mut ChildSpawnerCommands, art: &Art, index: usize, r: [f32; 4], k: f32) {
    let [x, y, right, bottom] = [
        [128., 83., 346., 410.],
        [515., 83., 819., 388.],
        [937., 131., 1276., 401.],
        [1383., 130., 1729., 407.],
        [64., 495., 415., 794.],
        [490., 494., 857., 792.],
        [963., 559., 1254., 773.],
        [1408., 562., 1700., 773.],
    ][index];
    p.spawn((
        bn(r[0], r[1], r[2], r[3], k),
        ImageNode {
            image: art.puzzle.actors.clone(),
            rect: Some(Rect::new(x, y, right, bottom)),
            ..default()
        },
    ));
}
fn play_icon(p: &mut ChildSpawnerCommands, r: [f32; 4], color: Color, k: f32) {
    for i in 0..28 {
        let t = i as f32 / 27.;
        let width = (1. - (t * 2. - 1.).abs()) * r[2] * 0.64 + 2.;
        stroke(
            p,
            Vec2::new(r[0] + r[2] * 0.15, r[1] + r[3] * (0.12 + t * 0.76)),
            Vec2::new(r[0] + r[2] * 0.15 + width, r[1] + r[3] * (0.12 + t * 0.76)),
            r[3] * 0.09,
            color,
            k,
        );
    }
}
fn op_instruction(op: Op) -> Instruction {
    match op {
        Op::Forward => Instruction::Forward,
        Op::Left => Instruction::Left,
        Op::Right => Instruction::Right,
        Op::Wait => Instruction::Wait,
    }
}
fn op_color(op: Op) -> Color {
    match op {
        Op::Forward => Color::srgb(0.02, 0.56, 1.),
        Op::Left => Color::srgb(0., 0.67, 0.30),
        Op::Right => Color::srgb(0.52, 0.24, 0.93),
        Op::Wait => Color::srgb(0.96, 0.62, 0.),
    }
}
fn ghost(p: &mut ChildSpawnerCommands, point: Vec2, size: f32, k: f32) {
    let a = Vec2::new(size * 0.60, size * 0.15);
    let b = Vec2::new(-size * 0.20, size * 0.23);
    let c = Vec2::new(0., size * 0.43);
    let top = point - c * 0.35;
    let pts = [
        top - a * 0.5 - b * 0.5,
        top + a * 0.5 - b * 0.5,
        top + a * 0.5 + b * 0.5,
        top - a * 0.5 + b * 0.5,
    ];
    for i in 0..4 {
        stroke(
            p,
            pts[i],
            pts[(i + 1) % 4],
            2.4,
            Color::srgba(0.8, 0.98, 1., 0.96),
            k,
        );
        stroke(
            p,
            pts[i],
            pts[i] + c,
            2.,
            Color::srgba(0.8, 0.98, 1., 0.90),
            k,
        );
        stroke(
            p,
            pts[i] + c,
            pts[(i + 1) % 4] + c,
            2.,
            Color::srgba(0.8, 0.98, 1., 0.90),
            k,
        );
    }
}
fn wind(p: &mut ChildSpawnerCommands, at: Vec2, k: f32) {
    for (w, h) in [(73., 34.), (47., 22.), (21., 10.)] {
        ring(
            p,
            [at.x - w * 0.5, at.y - h * 0.5, w, h],
            Color::srgb(0.35, 0.93, 1.),
            k,
        );
    }
}
fn outline_star(p: &mut ChildSpawnerCommands, r: [f32; 4], k: f32) {
    let points: Vec<Vec2> = (0..10)
        .map(|i| {
            let angle = i as f32 * std::f32::consts::PI / 5. - std::f32::consts::FRAC_PI_2;
            let radius = if i % 2 == 0 { 0.46 } else { 0.21 };
            Vec2::new(
                r[0] + r[2] * (0.5 + angle.cos() * radius),
                r[1] + r[3] * (0.5 + angle.sin() * radius),
            )
        })
        .collect();
    for i in 0..10 {
        stroke(
            p,
            points[i],
            points[(i + 1) % 10],
            1.8,
            Color::srgb(0.62, 0.65, 0.65),
            k,
        );
    }
}

fn scene(p: &mut ChildSpawnerCommands, game: &Puzzle, art: &Art, k: f32) {
    p.spawn((
        bn(0., 0., 1672., 941., k),
        ImageNode::new(if game.kind == Kind::Cloud {
            art.puzzle.cloud.clone()
        } else {
            art.puzzle.island.clone()
        }),
    ));
    for (i, cell) in game.level.cells.iter().enumerate() {
        if !cell.gap || !game.level.active(i) {
            continue;
        }
        let at = Vec2::from_array(cell.point);
        let cloud = game.kind == Kind::Cloud;
        let (w, h) = if cloud { (95., 83.) } else { (88., 65.) };
        if let Some(material) = game.plan.bridges.get(&i) {
            actor(
                p,
                art,
                if *material == Bridge::Wood { 2 } else { 3 },
                [at.x - w * 0.5, at.y - h * 0.48, w, h],
                k,
            );
        } else {
            ghost(p, at, if cloud { 119. } else { 105. }, k);
        }
        if cell.wind {
            wind(p, at - Vec2::new(0., 4.), k);
        }
        p.spawn((
            Button,
            Action::Puzzle(PuzzleAction::Cell(i)),
            bn(at.x - w * 0.5, at.y - h * 0.5, w, h, k),
            BackgroundColor(Color::NONE),
        ));
    }
    if let Some(pad) = game.level.pad {
        let at = Vec2::from_array(game.level.cells[pad].point);
        actor(
            p,
            art,
            if game.gate_open { 7 } else { 6 },
            [at.x - 33., at.y - 20., 66., 40.],
            k,
        );
    }
    if let Some(gate) = game.level.gate {
        let at = Vec2::from_array(game.level.cells[gate].point);
        actor(
            p,
            art,
            if game.gate_open { 5 } else { 4 },
            [at.x - 60., at.y - 115., 120., 125.],
            k,
        );
    }
    if let Some(bonus) = game.level.bonus.filter(|_| !game.bonus) {
        let at = Vec2::from_array(game.level.cells[bonus].point);
        sprite(p, art, 7, [at.x - 26., at.y - 67., 52., 70.], k);
    }
    let at = Vec2::from_array(game.level.cells[game.level.goal].point);
    actor(p, art, 1, [at.x - 44., at.y - 103., 88., 99.], k);
    let at = Vec2::from_array(game.level.cells[game.robot].point);
    let [w, h] = if game.kind == Kind::Cloud {
        [76., 98.]
    } else {
        [55., 75.]
    };
    // Preserve the approved cuboid face and white torso from the existing art,
    // rather than the unused robot variation in the new accessory atlas.
    p.spawn((
        PuzzleActor,
        bn(at.x - w * 0.5, at.y - h, w, h, k),
        ImageNode {
            image: art.builder.tiles.clone(),
            rect: Some(Rect::new(1016., 469., 1247., 843.)),
            flip_x: game.facing >= 2,
            ..default()
        },
    ));
}
pub(super) fn puzzle_ui(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    if s.mode == GameMode::CloudBridge {
        cloud_walk_ui(commands,s,art,w,h);
        return;
    }
    let game = &s.builder_puzzles[puzzle_index(s)];
    let cloud = game.kind == Kind::Cloud;
    let col = accent(cloud);
    let (k, offset) = builder_canvas(w, h);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Px(w),
                height: Val::Px(h),
                position_type: PositionType::Absolute,
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .with_children(|screen| {
            screen
                .spawn(bn(offset.x / k, offset.y / k, 1672., 941., k))
                .with_children(|root| {
                    scene(root, game, art, k);
                    panel(
                        root,
                        art,
                        if cloud {
                            [32., 20., 1608., 72.]
                        } else {
                            [22., 12., 1635., 88.]
                        },
                        cloud,
                        0,
                        k,
                        |p| {
                            button(
                                p,
                                art,
                                "←",
                                Action::Library,
                                [0., 0., 88., 72.],
                                cream(),
                                42.,
                                false,
                                k,
                            );
                            sprite(p, art, 0, [122., 12., 54., 54.], k);
                            text(
                                p,
                                art,
                                if cloud {
                                    "云桥工程师"
                                } else {
                                    "小小建造师"
                                },
                                [if cloud { 198. } else { 185. }, 12., 300., 60.],
                                39.,
                                ink(),
                                k,
                            );
                            if !cloud {
                                text(p, art, "机器人解谜", [420., 28., 170., 32.], 24., ink(), k);
                            }
                            let title = format!("第{}关 · {}", game.level.number, game.level.title);
                            text(
                                p,
                                art,
                                title,
                                [
                                    if cloud { 1185. } else { 625. },
                                    22.,
                                    if cloud { 280. } else { 330. },
                                    40.,
                                ],
                                25.,
                                ink(),
                                k,
                            );
                            let start = if cloud { 550. } else { 980. };
                            let spacing = if cloud { 94. } else { 44. };
                            let diameter = if cloud { 43. } else { 34. };
                            for number in 1..=6 {
                                let represented =
                                    crate::puzzle_game::Level::index_for_number(game.kind, number);
                                let current = number == game.level.number;
                                let done = represented.is_some_and(|i| game.progress.stars[i] > 0);
                                let color = if current {
                                    Color::srgb(0.96, 0.63, 0.)
                                } else if done {
                                    Color::srgb(0., 0.67, 0.32)
                                } else {
                                    Color::srgb(0.58, 0.66, 0.72)
                                };
                                let x = start + (number - 1) as f32 * spacing;
                                if number < 6 {
                                    stroke(
                                        p,
                                        Vec2::new(x + diameter, 36.),
                                        Vec2::new(x + spacing - 3., 36.),
                                        2.,
                                        Color::srgba(0.3, 0.6, 0.7, 0.4),
                                        k,
                                    );
                                }
                                if let Some(i) = represented {
                                    action_button(
                                        p,
                                        art,
                                        &number.to_string(),
                                        PuzzleAction::Stage(i),
                                        [x, 15., diameter, diameter],
                                        color,
                                        25.,
                                        true,
                                        k,
                                    );
                                } else {
                                    rounded(p, [x, 15., diameter, diameter], color, k);
                                    centered(
                                        p,
                                        art,
                                        &number.to_string(),
                                        [x, 15., diameter, diameter],
                                        24.,
                                        Color::WHITE,
                                        k,
                                    );
                                }
                            }
                            if !cloud {
                                for i in 0..3 {
                                    let r = [1285. + i as f32 * 42., 26., 36., 36.];
                                    if game.progress.stars[game.stage] > i as u8 {
                                        actor(p, art, 1, r, k);
                                    } else {
                                        outline_star(p, r, k);
                                    }
                                }
                            }
                            button(
                                p,
                                art,
                                "? 怎么玩",
                                Action::Help,
                                [
                                    if cloud { 1460. } else { 1414. },
                                    8.,
                                    if cloud { 135. } else { 136. },
                                    60.,
                                ],
                                if cloud {
                                    Color::srgb(0.89, 0.95, 1.)
                                } else {
                                    cream()
                                },
                                24.,
                                false,
                                k,
                            );
                            if !cloud {
                                button(
                                    p,
                                    art,
                                    "…",
                                    Action::ToggleIslandMenu,
                                    [1561., 8., 60., 60.],
                                    cream(),
                                    33.,
                                    false,
                                    k,
                                );
                            }
                        },
                    );
                    if cloud {
                        cloud_controls(root, game, art, col, k);
                    } else {
                        robot_controls(root, game, art, col, k);
                    }
                    if s.island_menu_open && !cloud {
                        panel(root, art, [1310., 105., 345., 340.], false, 2, k, |p| {
                            for (i, (label, a)) in [
                                (
                                    "上一关",
                                    PuzzleAction::Stage(
                                        crate::puzzle_game::Level::index_for_number(
                                            game.kind,
                                            (game.level.number + LEVEL_COUNT - 2) % LEVEL_COUNT + 1,
                                        )
                                        .unwrap(),
                                    ),
                                ),
                                ("下一关", PuzzleAction::Next),
                                ("重来本关", PuzzleAction::Restart),
                            ]
                            .into_iter()
                            .enumerate()
                            {
                                action_button(
                                    p,
                                    art,
                                    label,
                                    a,
                                    [18., 20. + i as f32 * 83., 305., 70.],
                                    cream(),
                                    24.,
                                    false,
                                    k,
                                );
                            }
                            button(
                                p,
                                art,
                                "返回游戏库",
                                Action::Library,
                                [18., 270., 305., 54.],
                                col,
                                24.,
                                true,
                                k,
                            );
                        });
                    }
                });
        });
}
fn cloud_controls(root: &mut ChildSpawnerCommands, game: &Puzzle, art: &Art, col: Color, k: f32) {
    panel(root, art, [32., 110., 355., 98.], true, 1, k, |p| {
        actor(p, art, 1, [20., 22., 58., 60.], k);
        text(p, art, "本关目标", [95., 13., 245., 36.], 29., ink(), k);
        text(p, art, "搭桥抵达终点", [95., 53., 245., 35.], 26., ink(), k);
    });
    if game.level.bonus.is_some() {
        panel(root, art, [32., 216., 201., 58.], true, 1, k, |p| {
            sprite(p, art, 7, [20., 9., 37., 43.], k);
            text(
                p,
                art,
                if game.bonus {
                    "星晶已收集"
                } else {
                    "额外星晶"
                },
                [83., 16., 126., 35.],
                22.,
                ink(),
                k,
            );
        });
    }
    panel(root, art, [1465., 120., 183., 635.], true, 2, k, |p| {
        centered(p, art, "剩余建材", [7., 17., 170., 45.], 29., ink(), k);
        for (i, b) in [Bridge::Wood, Bridge::Stone].into_iter().enumerate() {
            let selected = game.material == b && !game.erase;
            let y = 70. + i as f32 * 184.;
            p.spawn((
                Button,
                Action::Puzzle(PuzzleAction::Material(b)),
                Node {
                    border: UiRect::all(Val::Px(if selected { 4. } else { 1. } * k)),
                    ..bn(17., y, 147., 166., k)
                },
                BackgroundColor(Color::WHITE),
                BorderColor(if selected {
                    col
                } else {
                    Color::srgb(0.85, 0.87, 0.88)
                }),
                BorderRadius::all(Val::Px(24. * k)),
            ))
            .with_children(|p| {
                actor(
                    p,
                    art,
                    if b == Bridge::Wood { 2 } else { 3 },
                    [15., 13., 116., 111.],
                    k,
                );
                centered(
                    p,
                    art,
                    format!(
                        "{} ×{}",
                        if b == Bridge::Wood {
                            "木桥"
                        } else {
                            "石桥"
                        },
                        game.remaining(b)
                    ),
                    [2., 122., 143., 38.],
                    25.,
                    ink(),
                    k,
                );
                if selected {
                    rounded(p, [110., 12., 32., 32.], col, k);
                    centered(p, art, "✓", [110., 12., 32., 32.], 25., Color::WHITE, k);
                }
            });
        }
        wind(p, Vec2::new(90., 493.), k);
        centered(
            p,
            art,
            if game.level.cells.iter().any(|c| c.wind) {
                "风口请用石桥"
            } else {
                "拆除会退回建材"
            },
            [7., 531., 171., 45.],
            23.,
            ink(),
            k,
        );
    });
    panel(root, art, [24., 785., 1620., 130.], true, 3, k, |p| {
        text(
            p,
            art,
            if game.started
                || game.won
                || game.message.contains("用完")
                || game.message.contains("还没接通")
            {
                game.message.as_str()
            } else {
                "选一条路，规划建材"
            },
            [32., 32., 560., 64.],
            if game.started || game.won { 25. } else { 33. },
            ink(),
            k,
        );
        for (i, (title, action)) in [
            ("拆除", PuzzleAction::Erase),
            ("撤销", PuzzleAction::Undo),
            ("重来", PuzzleAction::Restart),
        ]
        .into_iter()
        .enumerate()
        {
            action_button(
                p,
                art,
                title,
                action,
                [605. + i as f32 * 211., 24., 191., 80.],
                if i == 0 {
                    Color::srgb(1., 0.91, 0.92)
                } else {
                    Color::srgb(0.88, 0.95, 1.)
                },
                30.,
                false,
                k,
            );
        }
        action_button(
            p,
            art,
            if game.running {
                "暂停"
            } else if game.won {
                "再走一次"
            } else {
                "试走"
            },
            PuzzleAction::Run,
            [1280., 19., 324., 91.],
            col,
            42.,
            true,
            k,
        );
        play_icon(p, [1301., 38., 51., 51.], Color::WHITE, k);
    });
}
fn robot_controls(root: &mut ChildSpawnerCommands, game: &Puzzle, art: &Art, col: Color, k: f32) {
    panel(root, art, [24., 112., 281., 206.], false, 1, k, |p| {
        text(p, art, "本关目标", [40., 21., 230., 42.], 31., ink(), k);
        text(
            p,
            art,
            game.level.objective,
            [40., 65., 240., 35.],
            22.,
            ink(),
            k,
        );
        actor(p, art, 2, [40., 110., 45., 34.], k);
        text(
            p,
            art,
            format!("木桥 {}/{}", game.plan.bridges.len(), game.level.stock[0]),
            [95., 109., 170., 38.],
            29.,
            ink(),
            k,
        );
        text(
            p,
            art,
            format!("指令 {}/{}", game.plan.cards.len(), game.level.card_limit),
            [95., 155., 170., 38.],
            29.,
            ink(),
            k,
        );
    });
    panel(root, art, [1305., 112., 355., 478.], false, 2, k, |p| {
        for (i, (name, build)) in [("搭建", true), ("编程", false)].into_iter().enumerate() {
            action_button(
                p,
                art,
                name,
                PuzzleAction::Build(build),
                [28. + i as f32 * 150., 25., 150., 56.],
                if game.build == build { col } else { cream() },
                29.,
                game.build == build,
                k,
            );
        }
        text(
            p,
            art,
            if game.build {
                "有限建材"
            } else {
                "试运行"
            },
            [43., 108., 250., 42.],
            33.,
            ink(),
            k,
        );
        if game.build {
            action_button(
                p,
                art,
                &format!("木桥 · 剩余{}", game.remaining(Bridge::Wood)),
                PuzzleAction::Material(Bridge::Wood),
                [41., 158., 277., 69.],
                cream(),
                28.,
                false,
                k,
            );
            action_button(
                p,
                art,
                "拆除并退回",
                PuzzleAction::Erase,
                [41., 239., 277., 66.],
                cream(),
                28.,
                false,
                k,
            );
            action_button(
                p,
                art,
                "撤销",
                PuzzleAction::Undo,
                [41., 319., 277., 66.],
                cream(),
                28.,
                false,
                k,
            );
        } else {
            action_button(
                p,
                art,
                if game.slow { "慢速" } else { "常速" },
                PuzzleAction::Speed,
                [41., 158., 277., 58.],
                cream(),
                28.,
                false,
                k,
            );
            action_button(
                p,
                art,
                "单步",
                PuzzleAction::Step,
                [41., 230., 132., 66.],
                cream(),
                28.,
                false,
                k,
            );
            action_button(
                p,
                art,
                "复位",
                PuzzleAction::Reset,
                [181., 230., 137., 66.],
                cream(),
                28.,
                false,
                k,
            );
            action_button(
                p,
                art,
                "提示",
                PuzzleAction::Hint,
                [41., 309., 277., 66.],
                cream(),
                28.,
                false,
                k,
            );
        }
        text(
            p,
            art,
            if game.started || game.hint > 0 || game.message.contains("材料") {
                game.message.as_str()
            } else {
                "失败可回到出错前一步。"
            },
            [42., 412., 279., 57.],
            20.,
            ink(),
            k,
        );
    });
    panel(root, art, [16., 593., 1645., 331.], false, 3, k, |p| {
        text(p, art, "我的指令", [36., 17., 210., 45.], 32., ink(), k);
        text(
            p,
            art,
            format!("{}/{}", game.plan.cards.len(), game.level.card_limit),
            [218., 24., 110., 35.],
            27.,
            Color::srgb(0.6, 0.52, 0.43),
            k,
        );
        action_button(
            p,
            art,
            "清空",
            PuzzleAction::Clear,
            [1488., 19., 134., 45.],
            cream(),
            25.,
            false,
            k,
        );
        for i in 0..16 {
            let x = 38. + (i % 8) as f32 * 201.;
            let y = 64. + (i / 8) as f32 * 82.;
            let r = [x, y, 190., 71.];
            if let Some(card) = game.plan.cards.get(i) {
                let tone = op_color(card.op);
                p.spawn((
                    Button,
                    Action::Puzzle(PuzzleAction::Select(i)),
                    Node {
                        border: UiRect::all(Val::Px(
                            if game.selected == Some(i) || (game.started && game.pc == i) {
                                3.
                            } else {
                                1.5
                            } * k,
                        )),
                        ..bn(x, y, 190., 71., k)
                    },
                    BackgroundColor(match card.op {
                        Op::Forward => Color::srgb(0.89, 0.96, 1.),
                        Op::Right => Color::srgb(0.97, 0.92, 1.),
                        Op::Left | Op::Wait => Color::srgb(1., 0.97, 0.86),
                    }),
                    BorderColor(tone),
                    BorderRadius::all(Val::Px(18. * k)),
                ))
                .with_children(|p| {
                    text(
                        p,
                        art,
                        (i + 1).to_string(),
                        [10., 8., 25., 26.],
                        21.,
                        tone,
                        k,
                    );
                    command_icon(
                        p,
                        art,
                        op_instruction(card.op),
                        [41., 12., 41., 45.],
                        tone,
                        k,
                    );
                    text(
                        p,
                        art,
                        if card.repeats > 1 {
                            format!("{} ×{}", card.op.label(), card.repeats)
                        } else {
                            card.op.label().into()
                        },
                        [93., 25., 96., 35.],
                        25.,
                        ink(),
                        k,
                    );
                });
            } else {
                p.spawn((
                    Node {
                        border: UiRect::all(Val::Px(k)),
                        ..bn(x, y, 190., 71., k)
                    },
                    BorderColor(Color::srgba(0.7, 0.55, 0.37, 0.4)),
                    BorderRadius::all(Val::Px(18. * k)),
                ))
                .with_children(|p| {
                    text(
                        p,
                        art,
                        (i + 1).to_string(),
                        [11., 9., 35., 30.],
                        24.,
                        Color::srgb(0.66, 0.61, 0.56),
                        k,
                    );
                    if i >= game.level.card_limit {
                        rounded(p, [85., 32., 24., 24.], Color::srgb(0.77, 0.73, 0.65), k);
                        path_icon(
                            p,
                            &[
                                (0.25, 0.7),
                                (0.25, 0.3),
                                (0.5, 0.12),
                                (0.75, 0.3),
                                (0.75, 0.7),
                            ],
                            [85., 17., 24., 25.],
                            Color::srgb(0.77, 0.73, 0.65),
                            k,
                        );
                    }
                });
            }
            let _ = r;
        }
        for (i, op) in [Op::Forward, Op::Left, Op::Right, Op::Wait]
            .into_iter()
            .enumerate()
        {
            let x = 38. + i as f32 * 202.;
            action_button(
                p,
                art,
                "",
                PuzzleAction::Add(op),
                [x, 226., 186., 80.],
                cream(),
                28.,
                false,
                k,
            );
            command_icon(
                p,
                art,
                op_instruction(op),
                [x + 17., 242., 47., 49.],
                op_color(op),
                k,
            );
            text(
                p,
                art,
                op.label(),
                [x + 80., 253., 100., 40.],
                28.,
                ink(),
                k,
            );
        }
        action_button(
            p,
            art,
            "重复",
            PuzzleAction::Repeat,
            [848., 226., 198., 80.],
            cream(),
            28.,
            false,
            k,
        );
        action_button(
            p,
            art,
            if game.running { "暂停" } else { "运行" },
            PuzzleAction::Run,
            [1312., 226., 299., 80.],
            Color::srgb(0., 0.70, 0.31),
            38.,
            true,
            k,
        );
        play_icon(p, [1337., 244., 48., 46.], Color::WHITE, k);
    });
}
