//! Approved v3: authored maps, one ground projection, and player-controlled walking.
use super::builder_puzzle_ui::{actor, button};
use super::builder_redesign::{bn, centered, path_icon, sprite, stroke, text};
use super::*;
use crate::cloud_walk_game::{CloudWalk, Map, Progress, COUNT};
use crate::puzzle_game::Bridge;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CloudAction {
    Material(Bridge),
    Erase,
    Undo,
    Restart,
    Enter,
    Back,
    Move(i32, i32),
    Stage(usize),
}
pub(super) struct CloudArt {
    pub sky: Handle<Image>,
    island: Handle<Image>,
    tiles: Handle<Image>,
}
pub(super) fn load_cloud_art(a: &AssetServer) -> CloudArt {
    CloudArt {
        sky: a.load("builders/redesign-v1/cloud-background.png"),
        island: a.load("builders/cloud-walk-v3/island.png"),
        tiles: a.load("builders/cloud-walk-v3/tiles.png"),
    }
}
pub(super) fn load_cloud_walk() -> CloudWalk {
    fs::read(save_directory().join("cloud-walk-v3.json"))
        .ok()
        .and_then(|b| serde_json::from_slice::<Progress>(&b).ok())
        .filter(Progress::valid)
        .map(CloudWalk::restore)
        .unwrap_or_default()
}
pub(super) fn persist_cloud_walk(s: &mut Session) {
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(save_directory())?;
        let mut path = save_directory().join("cloud-walk-v3.json");
        if let Ok(b) = fs::read(&path) {
            if !serde_json::from_slice::<Progress>(&b).is_ok_and(|p| p.valid()) {
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_millis();
                if fs::copy(&path, path.with_extension(format!("unreadable-{t}.json"))).is_err() {
                    path = path.with_extension(format!("recovery-{t}.json"));
                }
            }
        }
        let temp = path.with_extension("tmp");
        fs::write(&temp, serde_json::to_vec_pretty(&s.cloud_walk.snapshot())?)?;
        fs::rename(temp, path)?;
        Ok(())
    })();
    if let Err(e) = result {
        s.cloud_walk.message = format!("暂未保存，搭建仍在当前游戏：{e}");
    }
}
pub(super) fn cloud_action(s: &mut Session, a: CloudAction) {
    let g = &mut s.cloud_walk;
    match a {
        CloudAction::Material(b) => {
            if !g.walking {
                g.material = b;
                g.erase = false;
                g.message = "选中材料，点蓝色缺口搭桥。".into();
            }
        }
        CloudAction::Erase => {
            if !g.walking {
                g.erase = !g.erase;
                g.message = "点桥块拆除，材料会退回。".into();
            }
        }
        CloudAction::Undo => g.undo(),
        CloudAction::Restart => g.restart(),
        CloudAction::Enter => g.enter(),
        CloudAction::Back => g.to_build(),
        CloudAction::Move(x, z) => {
            g.move_dir(x, z);
        }
        CloudAction::Stage(n) => g.choose(n),
    }
}
fn canvas(w: f32, h: f32) -> (f32, Vec2) {
    let k = (w / 1280.).min(h / 720.);
    (k, Vec2::new((w - 1280. * k) * 0.5, (h - 720. * k) * 0.5))
}
#[derive(Clone, Copy)]
struct Projection {
    origin: Vec2,
    u: f32,
}
impl Projection {
    fn new(m: &Map) -> Self {
        let mut lo = Vec2::splat(f32::MAX);
        let mut hi = Vec2::splat(f32::MIN);
        for &[x, z] in &m.islands {
            for dx in [-1.5, 1.5] {
                for dz in [-1.5, 1.5] {
                    let p = Vec2::new(
                        x as f32 + dx - z as f32 - dz,
                        (x as f32 + dx + z as f32 + dz) * 0.48,
                    );
                    lo = lo.min(p);
                    hi = hi.max(p);
                }
            }
        }
        // Same bounds, inset and projection as the approved manual-crossing diagram.
        // Three extra world units below the cap reserve room for natural rock columns.
        let size = hi - lo;
        let u = (705. / size.x).min(405. / (size.y + 3.)).min(49.);
        Self {
            origin: Vec2::new(280. + (770. - size.x * u) * 0.5 - lo.x * u, 128. - lo.y * u),
            u,
        }
    }
    fn point(self, x: f32, z: f32) -> Vec2 {
        self.origin + Vec2::new(x - z, (x + z) * 0.48) * self.u
    }
    fn grid(self, p: Vec2) -> [i32; 2] {
        let v = (p - self.origin) / self.u;
        [
            ((v.x + v.y / 0.48) * 0.5).round() as i32,
            ((v.y / 0.48 - v.x) * 0.5).round() as i32,
        ]
    }
}
pub(super) fn cloud_cell_screen(s: &Session, id: usize, w: f32, h: f32) -> Vec2 {
    let (k, o) = canvas(w, h);
    let c = s.cloud_walk.map.cells[id].at;
    o + Projection::new(&s.cloud_walk.map).point(c[0] as f32, c[1] as f32) * k
}
fn ink() -> Color {
    Color::srgb(0.06, 0.20, 0.24)
}
fn blue() -> Color {
    Color::srgb(0.02, 0.48, 0.98)
}
fn cream() -> Color {
    Color::srgb(1., 0.99, 0.95)
}
fn panel(p: &mut ChildSpawnerCommands, r: [f32; 4], k: f32) {
    p.spawn((
        WorldInputBlocker,
        bn(r[0], r[1], r[2], r[3], k),
        BackgroundColor(Color::srgba(1., 1., 0.98, 0.96)),
        BorderRadius::all(Val::Px(23. * k)),
    ));
}
fn bridge(p: &mut ChildSpawnerCommands, art: &Art, b: Bridge, r: [f32; 4], k: f32) {
    p.spawn((
        bn(r[0], r[1], r[2], r[3], k),
        ImageNode {
            image: art.cloud_walk.tiles.clone(),
            rect: Some(if b == Bridge::Wood {
                Rect::new(48., 197., 850., 736.)
            } else {
                Rect::new(924., 197., 1727., 737.)
            }),
            ..default()
        },
    ));
}
fn diamond(
    p: &mut ChildSpawnerCommands,
    q: Projection,
    x: f32,
    z: f32,
    color: Color,
    width: f32,
    k: f32,
) {
    let a = [
        q.point(x - 0.5, z - 0.5),
        q.point(x + 0.5, z - 0.5),
        q.point(x + 0.5, z + 0.5),
        q.point(x - 0.5, z + 0.5),
    ];
    for i in 0..4 {
        stroke(p, a[i], a[(i + 1) % 4], width, color, k);
    }
}
#[derive(Component)]
pub(super) struct CloudWalkActor;
pub(super) fn cloud_walk_ui(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let g = &s.cloud_walk;
    let (k, o) = canvas(w, h);
    let q = Projection::new(&g.map);
    let u = q.u;
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
        .with_children(|p| {
            p.spawn(bn(o.x / k, o.y / k, 1280., 720., k))
                .with_children(|p| {
                    // BuilderBackdrop draws one continuous sky over the full window,
                    // including wide-screen margins; a second canvas image makes seams.

                    let mut islands = g.map.islands.clone();
                    islands.sort_by_key(|c| c[0] + c[1]);
                    // The generated cap occupies 84% of sprite width and 47% of height.
                    // Calibrate that footprint to exactly 3 × 3 ground cells.
                    for [x, z] in islands {
                        let c = q.point(x as f32, z as f32);
                        p.spawn((
                            bn(
                                c.x - 0.49 * 7.142 * u,
                                c.y - 0.35 * 6.15 * u,
                                7.142 * u,
                                6.15 * u,
                                k,
                            ),
                            ImageNode::new(art.cloud_walk.island.clone()),
                        ));
                    }
                    for (id, c) in g.map.cells.iter().enumerate() {
                        let [x, z] = c.at;
                        let at = q.point(x as f32, z as f32);
                        if c.gap {
                            if let Some(b) = g.plan.bridges.get(&id) {
                                bridge(
                                    p,
                                    art,
                                    *b,
                                    [at.x - u, at.y - 0.48 * u, 2. * u, 1.13 * u],
                                    k,
                                );
                            } else {
                                diamond(
                                    p,
                                    q,
                                    x as f32,
                                    z as f32,
                                    Color::srgba(0.04, 0.65, 0.96, 0.8),
                                    1.6,
                                    k,
                                );
                            }
                            if c.wind {
                                for d in [-0.11, 0.09] {
                                    stroke(
                                        p,
                                        at + Vec2::new(-0.3 * u, d * u),
                                        at + Vec2::new(0.3 * u, d * u - 0.1 * u),
                                        2.,
                                        Color::srgb(0.1, 0.67, 0.97),
                                        k,
                                    );
                                }
                            }
                        } else {
                            diamond(
                                p,
                                q,
                                x as f32,
                                z as f32,
                                Color::srgba(0.13, 0.46, 0.22, 0.19),
                                0.8,
                                k,
                            );
                        }
                    }
                    let c = g.map.cells[g.map.goal].at;
                    let at = q.point(c[0] as f32, c[1] as f32);
                    actor(
                        p,
                        art,
                        1,
                        [at.x - 0.47 * u, at.y - 0.92 * u, 0.94 * u, 0.94 * u],
                        k,
                    );
                    if let Some(id) = g.map.gem.filter(|_| !g.gem_taken) {
                        let c = g.map.cells[id].at;
                        let at = q.point(c[0] as f32, c[1] as f32);
                        sprite(
                            p,
                            art,
                            7,
                            [at.x - 0.29 * u, at.y - 0.95 * u, 0.58 * u, 0.92 * u],
                            k,
                        );
                    }
                    let c = g.map.cells[g.robot].at;
                    let at = q.point(c[0] as f32, c[1] as f32);
                    p.spawn((
                        CloudWalkActor,
                        bn(at.x - 0.4 * u, at.y - 1.26 * u, 0.8 * u, 1.26 * u, k),
                        ImageNode {
                            image: art.builder.tiles.clone(),
                            rect: Some(Rect::new(1016., 469., 1247., 843.)),
                            flip_x: g.face_left,
                            ..default()
                        },
                    ));
                    panel(p, [20., 16., 1240., 65.], k);
                    button(
                        p,
                        art,
                        "←",
                        Action::Library,
                        [30., 25., 64., 47.],
                        cream(),
                        34.,
                        false,
                        k,
                    );
                    text(p, art, "云桥工程师", [110., 29., 280., 43.], 30., ink(), k);
                    for n in 0..COUNT {
                        button(
                            p,
                            art,
                            &format!("{}", n + 1),
                            Action::CloudWalk(CloudAction::Stage(n)),
                            [419. + n as f32 * 48., 32., 34., 34.],
                            if n == g.stage {
                                Color::srgb(1., 0.66, 0.17)
                            } else if g.progress.stars[n] > 0 {
                                Color::srgb(0.31, 0.78, 0.50)
                            } else {
                                Color::srgb(0.87, 0.94, 0.95)
                            },
                            20.,
                            false,
                            k,
                        );
                    }
                    text(
                        p,
                        art,
                        format!("第{}关 · {}", g.stage + 1, g.map.spec.title),
                        [790., 34., 340., 36.],
                        22.,
                        ink(),
                        k,
                    );
                    button(
                        p,
                        art,
                        "",
                        Action::Help,
                        [1140., 27., 99., 43.],
                        blue(),
                        18.,
                        true,
                        k,
                    );
                    panel(p, [22., 105., 250., 132.], k);
                    text(p, art, "本关目标", [42., 121., 210., 34.], 25., ink(), k);
                    text(
                        p,
                        art,
                        if g.map.spec.required {
                            "先取星晶，再到终点"
                        } else if g.map.gem.is_some() {
                            "到达终点，可选取星晶"
                        } else {
                            "补齐断桥，走到终点"
                        },
                        [42., 162., 217., 30.],
                        18.,
                        ink(),
                        k,
                    );
                    text(
                        p,
                        art,
                        if g.walking {
                            "当前：过桥模式"
                        } else {
                            "当前：搭桥模式"
                        },
                        [42., 202., 215., 27.],
                        19.,
                        blue(),
                        k,
                    );
                    panel(p, [1092., 106., 166., 452.], k);
                    centered(p, art, "剩余建材", [1092., 124., 166., 39.], 23., ink(), k);
                    for (i, b) in [Bridge::Wood, Bridge::Stone].into_iter().enumerate() {
                        let y = 176. + i as f32 * 150.;
                        let fill = if !g.walking && !g.erase && g.material == b {
                            Color::srgb(0.79, 0.91, 1.)
                        } else {
                            cream()
                        };
                        if g.walking {
                            p.spawn((
                                bn(1108., y, 134., 130., k),
                                BackgroundColor(fill),
                                BorderColor(Color::srgb(0.87, 0.83, 0.77)),
                                BorderRadius::all(Val::Px(20. * k)),
                            ));
                        } else {
                            button(
                                p,
                                art,
                                "",
                                Action::CloudWalk(CloudAction::Material(b)),
                                [1108., y, 134., 130.],
                                fill,
                                20.,
                                false,
                                k,
                            );
                        }
                        bridge(p, art, b, [1129., y + 9., 92., 61.], k);
                        centered(
                            p,
                            art,
                            format!(
                                "{} × {}",
                                if b == Bridge::Wood {
                                    "木桥"
                                } else {
                                    "石桥"
                                },
                                g.remaining(b)
                            ),
                            [1110., y + 82., 130., 35.],
                            23.,
                            ink(),
                            k,
                        );
                    }
                    panel(p, [20., 591., 1240., 111.], k);
                    text(
                        p,
                        art,
                        if g.won {
                            "到达终点！"
                        } else if g.walking {
                            "亲自带机器人到终点"
                        } else {
                            "选路线，补好桥"
                        },
                        [42., 606., 365., 34.],
                        26.,
                        ink(),
                        k,
                    );
                    let mut message = g.message.clone();
                    if g.walking {
                        message = g.message.chars().take(22).collect();
                    }
                    text(
                        p,
                        art,
                        message,
                        [42., 646., 365., if g.walking { 23. } else { 46. }],
                        16.,
                        ink(),
                        k,
                    );
                    if g.walking {
                        text(
                            p,
                            art,
                            "返回搭桥：回起点，保留桥梁",
                            [42., 674., 365., 23.],
                            14.,
                            ink(),
                            k,
                        );
                    }
                    if g.walking {
                        for (i, (dx, dz, label, ax, az)) in [
                            (-1, 0, "左上", 0.2, 0.2),
                            (0, -1, "右上", 0.8, 0.2),
                            (0, 1, "左下", 0.2, 0.8),
                            (1, 0, "右下", 0.8, 0.8),
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            let x = 435. + 130. * i as f32;
                            button(
                                p,
                                art,
                                "",
                                Action::CloudWalk(CloudAction::Move(dx, dz)),
                                [x, 607., 112., 79.],
                                Color::srgb(0.89, 0.95, 1.),
                                18.,
                                false,
                                k,
                            );
                            path_icon(
                                p,
                                &[
                                    (1. - ax, 1. - az),
                                    (ax, az),
                                    (ax + (if ax < 0.5 { 0.32 } else { -0.32 }), az),
                                    (ax, az),
                                    (ax, az + (if az < 0.5 { 0.32 } else { -0.32 })),
                                ],
                                [x + 37., 617., 38., 29.],
                                blue(),
                                k,
                            );
                            centered(p, art, label, [x, 652., 112., 26.], 17., ink(), k);
                        }
                        button(
                            p,
                            art,
                            "返回搭桥",
                            Action::CloudWalk(CloudAction::Back),
                            [989., 607., 248., 79.],
                            blue(),
                            29.,
                            true,
                            k,
                        );
                    } else {
                        for (i, (a, t)) in [
                            (CloudAction::Erase, "拆除"),
                            (CloudAction::Undo, "撤销"),
                            (CloudAction::Restart, "重来"),
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            button(
                                p,
                                art,
                                t,
                                Action::CloudWalk(a),
                                [435. + 175. * i as f32, 607., 153., 79.],
                                if a == CloudAction::Erase && g.erase {
                                    Color::srgb(0.8, 0.91, 1.)
                                } else {
                                    cream()
                                },
                                23.,
                                false,
                                k,
                            );
                        }
                        button(
                            p,
                            art,
                            "开始过桥",
                            Action::CloudWalk(CloudAction::Enter),
                            [989., 607., 248., 79.],
                            blue(),
                            29.,
                            true,
                            k,
                        );
                    }
                });
        });
}
pub(super) fn animate_cloud_walk(
    s: Res<Session>,
    layout: Res<Layout>,
    scale: Res<UiScale>,
    time: Res<Time>,
    mut actors: Query<(&mut Node, &mut ImageNode), With<CloudWalkActor>>,
    mut position: Local<Option<(usize, Vec2)>>,
) {
    if s.mode != GameMode::CloudBridge {
        return;
    }
    let g = &s.cloud_walk;
    let q = Projection::new(&g.map);
    let c = g.map.cells[g.robot].at;
    let at = q.point(c[0] as f32, c[1] as f32);
    let current = match *position {
        Some((stage, old)) if stage == g.stage && g.walking => {
            old.lerp(at, 1. - (-17. * time.delta_secs()).exp())
        }
        _ => at,
    };
    *position = Some((g.stage, current));
    let (k, _) = canvas(layout.width / scale.0, layout.height / scale.0);
    for (mut node, mut image) in &mut actors {
        *node = bn(
            current.x - 0.4 * q.u,
            current.y - 1.26 * q.u,
            0.8 * q.u,
            1.26 * q.u,
            k,
        );
        image.flip_x = g.face_left;
    }
}
pub(super) fn cloud_walk_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    scale: Res<UiScale>,
    layout: Res<Layout>,
    blockers: Query<(&ComputedNode, &GlobalTransform), Or<(With<WorldInputBlocker>, With<Button>)>>,
    controls: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut hold: Local<(Option<(i32, i32)>, f32)>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::CloudBridge || s.library_open || s.help_open {
        *hold = (None, 0.);
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    if !s.cloud_walk.walking {
        *hold = (None, 0.);
        let p = touches
            .iter_just_pressed()
            .next()
            .map(|t| t.position())
            .or_else(|| {
                mouse
                    .just_pressed(MouseButton::Left)
                    .then(|| window.cursor_position())
                    .flatten()
            });
        if let Some(p) = p {
            let physical = p * window.scale_factor();
            if !blockers.iter().any(|(n, t)| {
                Rect::from_center_size(t.translation().truncate(), n.size()).contains(physical)
            }) {
                let (k, o) = canvas(layout.width / scale.0, layout.height / scale.0);
                let grid = Projection::new(&s.cloud_walk.map).grid((p / scale.0 - o) / k);
                if let Some(id) = s.cloud_walk.map.id(grid) {
                    s.cloud_walk.place(id);
                    s.ui_dirty = true;
                    persist_cloud_walk(&mut s);
                }
            }
        }
        return;
    }
    let dirs = [
        (KeyCode::KeyA, KeyCode::ArrowLeft, (-1, 0)),
        (KeyCode::KeyW, KeyCode::ArrowUp, (0, -1)),
        (KeyCode::KeyS, KeyCode::ArrowDown, (0, 1)),
        (KeyCode::KeyD, KeyCode::ArrowRight, (1, 0)),
    ];
    let key = dirs
        .iter()
        .find(|(a, b, _)| keys.pressed(*a) || keys.pressed(*b));
    let key_first = key.is_some_and(|(a, b, _)| keys.just_pressed(*a) || keys.just_pressed(*b));
    let points: Vec<_> = touches
        .iter()
        .map(|t| t.position() * window.scale_factor())
        .chain(
            mouse
                .pressed(MouseButton::Left)
                .then(|| window.cursor_position())
                .flatten()
                .map(|p| p * window.scale_factor()),
        )
        .collect();
    let pointer = controls.iter().find_map(|(a, n, t)| {
        if let Action::CloudWalk(CloudAction::Move(x, z)) = a {
            points
                .iter()
                .any(|p| Rect::from_center_size(t.translation().truncate(), n.size()).contains(*p))
                .then_some((*x, *z))
        } else {
            None
        }
    });
    let dir = key.map(|(_, _, d)| *d).or(pointer);
    let mut step = key_first;
    if dir != hold.0 {
        *hold = (dir, 0.);
    } else if dir.is_some() {
        hold.1 += time.delta_secs();
        if hold.1 >= 0.29 {
            hold.1 -= 0.17;
            step = true;
        }
    }
    if step {
        if let Some((x, z)) = dir {
            s.cloud_walk.move_dir(x, z);
            s.ui_dirty = true;
            if s.cloud_walk.won {
                persist_cloud_walk(&mut s);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn projected_cells_roundtrip() {
        for i in 0..COUNT {
            let m = Map::new(i);
            let q = Projection::new(&m);
            for c in &m.cells {
                assert_eq!(q.grid(q.point(c.at[0] as f32, c.at[1] as f32)), c.at);
            }
        }
    }
}
