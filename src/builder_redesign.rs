//! The approved builder interfaces. The world is drawn from live cells, never a screenshot.
use super::*;

const BW: f32 = 1672.;
const BH: f32 = 941.;

#[derive(Component)]
pub(super) struct BuilderActor;

pub(super) fn animate_builder_actor(
    time: Res<Time>,
    s: Res<Session>,
    layout: Res<Layout>,
    scale: Res<UiScale>,
    mut actor: Query<&mut Node, With<BuilderActor>>,
    mut position: Local<Option<(GameMode, usize, Vec3)>>,
) {
    if !builder_illustrated(&s) {
        *position = None;
        return;
    }
    let target = Vec3::new(
        s.work.world.robot[0] as f32,
        0.5,
        s.work.world.robot[1] as f32,
    );
    let current = match *position {
        Some((mode, level, point))
            if mode == s.mode && level == s.work.level && s.runner.is_some() =>
        {
            point.lerp(target, 1. - (-12. * time.delta_secs()).exp())
        }
        _ => target,
    };
    *position = Some((s.mode, s.work.level, current));
    let proj = BuilderProjection::new(&s);
    let unit = proj.x * 2.;
    let point = proj.point(current);
    let (k, _) = builder_canvas(layout.width / scale.0, layout.height / scale.0);
    for mut node in &mut actor {
        *node = bn(
            point.x - unit * 0.39,
            point.y - unit * 1.02,
            unit * 0.78,
            unit * 1.02,
            k,
        );
    }
}

pub(super) struct BuilderArt {
    pub(super) cloud: Handle<Image>,
    pub(super) island: Handle<Image>,
    pub(super) tiles: Handle<Image>,
    frames: Handle<Image>,
    scenery: Handle<Image>,
}
pub(super) fn load_builder_art(server: &AssetServer) -> BuilderArt {
    let image = |path: &str| {
        server.load_with_settings(
            image_asset_path(path),
            |s: &mut bevy::image::ImageLoaderSettings| {
                s.sampler = bevy::image::ImageSampler::linear()
            },
        )
    };
    BuilderArt {
        cloud: image("builders/redesign-v1/cloud-background.png"),
        island: image("builders/redesign-v1/island-background.png"),
        tiles: image("builders/redesign-v1/tiles.png"),
        frames: image("builders/redesign-v1/frames.png"),
        scenery: image("builders/redesign-v1/scenery.png"),
    }
}
pub(super) fn builder_mode(s: &Session) -> bool {
    matches!(s.mode, GameMode::Island | GameMode::CloudBridge)
}
pub(super) fn builder_illustrated(s: &Session) -> bool {
    builder_mode(s) && !s.island_3d_view
}

#[derive(Clone, Copy)]
pub(super) struct BuilderProjection {
    origin: Vec2,
    x: f32,
    z: f32,
    y: f32,
}
impl BuilderProjection {
    pub(super) fn new(s: &Session) -> Self {
        let build = s.build;
        let cloud = s.mode == GameMode::CloudBridge;
        // Match the confirmed compact banks and descending bridge composition.
        let (unit, origin) = match (cloud, build) {
            (true, true) => (145., Vec2::new(810., 420.)),
            (true, false) => (122., Vec2::new(810., 330.)),
            (false, true) => (136., Vec2::new(745., 410.)),
            (false, false) => (112., Vec2::new(720., 325.)),
        };
        Self {
            origin,
            x: unit * 0.5,
            z: unit * 0.20,
            y: unit * 0.60,
        }
    }
    pub(super) fn point(self, p: Vec3) -> Vec2 {
        self.origin + Vec2::new((p.x - p.z) * self.x, (p.x + p.z) * self.z - p.y * self.y)
    }
    pub(super) fn ray(self, p: Vec2) -> Ray3d {
        let a = (p.x - self.origin.x) / self.x;
        let b = (p.y - self.origin.y) / self.z;
        let slope = self.y / self.z;
        let height = 12.;
        Ray3d::new(
            Vec3::new(
                (a + b + slope * height) * 0.5,
                height,
                (b + slope * height - a) * 0.5,
            ),
            Dir3::new(Vec3::new(-slope * 0.5, -1., -slope * 0.5)).unwrap(),
        )
    }
}
pub(super) fn builder_canvas(w: f32, h: f32) -> (f32, Vec2) {
    let k = (w / BW).min(h / BH);
    (k, (Vec2::new(w, h) - Vec2::new(BW, BH) * k) * 0.5)
}
pub(super) fn builder_pointer_to_world(s: &Session, p: Vec2, w: f32, h: f32) -> Option<Ray3d> {
    let (k, origin) = builder_canvas(w, h);
    let p = (p - origin) / k;
    (p.x >= 0. && p.x <= 1450. && p.y >= 104. && p.y <= if s.build { 780. } else { 595. })
        .then(|| BuilderProjection::new(s).ray(p))
}
pub(super) fn builder_cell_screen(s: &Session, p: Vec3, w: f32, h: f32) -> Vec2 {
    let (k, origin) = builder_canvas(w, h);
    origin + BuilderProjection::new(s).point(p) * k
}

pub(super) fn bn(x: f32, y: f32, w: f32, h: f32, k: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x * k),
        top: Val::Px(y * k),
        width: Val::Px(w * k),
        height: Val::Px(h * k),
        ..default()
    }
}
fn ink(cloud: bool) -> Color {
    if cloud {
        Color::srgb(0.10, 0.24, 0.37)
    } else {
        Color::srgb(0.09, 0.23, 0.16)
    }
}
fn accent(cloud: bool) -> Color {
    if cloud {
        Color::srgb(0.02, 0.49, 1.)
    } else {
        Color::srgb(0., 0.64, 0.29)
    }
}
fn ivory() -> Color {
    Color::srgb(1., 0.985, 0.943)
}
pub(super) fn text(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    value: impl Into<String>,
    r: [f32; 4],
    size: f32,
    color: Color,
    k: f32,
) {
    p.spawn((
        bn(r[0], r[1], r[2], r[3], k),
        Text::new(value),
        TextFont {
            font: art.font.clone(),
            font_size: size * k,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(JustifyText::Left),
    ));
}
pub(super) fn centered(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    value: impl Into<String>,
    r: [f32; 4],
    size: f32,
    color: Color,
    k: f32,
) {
    p.spawn(Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..bn(r[0], r[1], r[2], r[3], k)
    })
    .with_children(|p| {
        label(p, art, value, size * k, color);
    });
}
fn panel(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    r: [f32; 4],
    cloud: bool,
    k: f32,
    f: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let edge = if cloud {
        Color::srgba(0.87, 0.95, 1., 0.9)
    } else {
        Color::srgb(0.78, 0.51, 0.23)
    };
    let mut frame = p.spawn((
        WorldInputBlocker,
        Node {
            border: UiRect::all(Val::Px(if cloud { 1.5 } else { 0. } * k)),
            ..bn(r[0], r[1], r[2], r[3], k)
        },
        BackgroundColor(if cloud { ivory() } else { Color::NONE }),
        BorderColor(edge),
        BorderRadius::all(Val::Px(30. * k)),
    ));
    if !cloud {
        let rect = if r[1] < 100. {
            Rect::new(18., 8., 1658., 104.)
        } else if r[0] > 1250. {
            Rect::new(1333., 136., 1672., 600.)
        } else if r[1] >= 590. {
            Rect::new(18., 760., 1665., 929.)
        } else {
            Rect::new(15., 110., 328., 280.)
        };
        frame.insert(ImageNode {
            image: art.builder.frames.clone(),
            rect: Some(rect),
            ..default()
        });
    }
    frame.with_children(f);
}
fn control(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    title: impl Into<String>,
    action: Action,
    r: [f32; 4],
    fill: Color,
    color: Color,
    size: f32,
    k: f32,
) {
    let title = title.into();
    p.spawn((
        Button,
        action,
        ButtonTone(fill),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1. * k)),
            ..bn(r[0], r[1], r[2], r[3], k)
        },
        BackgroundColor(fill),
        BorderColor(Color::srgba(0.65, 0.66, 0.60, 0.22)),
        BorderRadius::all(Val::Px(18. * k)),
    ))
    .with_children(|p| {
        if matches!(action, Action::Finish) {
            p.spawn((
                bn(
                    if title.contains('开') {
                        r[2] - 43.
                    } else {
                        7.
                    },
                    7.,
                    r[3] - 14.,
                    r[3] - 14.,
                    k,
                ),
                BackgroundColor(Color::WHITE),
                BorderRadius::MAX,
            ));
        } else if let Some(name) = match action {
            Action::Erase => Some("拆除"),
            Action::Undo => Some("撤销"),
            Action::ResetView => Some("视角回正"),
            Action::Build if title.contains('搭') && r[2] >= 120. => Some("搭建"),
            Action::Step => Some("单步"),
            Action::Reset => Some("复位"),
            Action::Help => Some("怎么玩"),
            Action::Clear => Some("清空"),
            _ => None,
        } {
            let icon_size = (r[3] - 20.).min(46.).min(r[2] * 0.25);
            let x = 12.;
            let y = (r[3] - icon_size) * 0.5;
            toolbar_icon(p, art, action, [x, y, icon_size, icon_size], color, k);
            centered(
                p,
                art,
                name,
                [icon_size + 16., 0., r[2] - icon_size - 22., r[3]],
                size.min((r[2] - icon_size - 26.) / name.chars().count() as f32),
                color,
                k,
            );
        } else {
            label(p, art, title, size * k, color);
        }
    });
}

pub(super) fn stroke(p: &mut ChildSpawnerCommands, a: Vec2, b: Vec2, width: f32, color: Color, k: f32) {
    let delta = b - a;
    let center = (a + b) * 0.5;
    p.spawn((
        bn(
            center.x - delta.length() * 0.5,
            center.y - width * 0.5,
            delta.length(),
            width,
            k,
        ),
        BackgroundColor(color),
        BorderRadius::all(Val::Px(width * k * 0.5)),
        Transform::from_rotation(Quat::from_rotation_z(delta.y.atan2(delta.x))),
    ));
}
pub(super) fn path_icon(
    p: &mut ChildSpawnerCommands,
    points: &[(f32, f32)],
    r: [f32; 4],
    color: Color,
    k: f32,
) {
    for pair in points.windows(2) {
        stroke(
            p,
            Vec2::new(r[0] + pair[0].0 * r[2], r[1] + pair[0].1 * r[3]),
            Vec2::new(r[0] + pair[1].0 * r[2], r[1] + pair[1].1 * r[3]),
            r[2] * 0.15,
            color,
            k,
        );
    }
}
pub(super) fn command_icon(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    op: Instruction,
    r: [f32; 4],
    color: Color,
    k: f32,
) {
    match op {
        Instruction::Place => sprite(p, art, 1, r, k),
        Instruction::Forward => {
            path_icon(p, &[(0.5, 0.88), (0.5, 0.18)], r, color, k);
            path_icon(p, &[(0.2, 0.45), (0.5, 0.15), (0.8, 0.45)], r, color, k);
        }
        Instruction::Back => {
            path_icon(p, &[(0.5, 0.12), (0.5, 0.82)], r, color, k);
            path_icon(p, &[(0.2, 0.55), (0.5, 0.85), (0.8, 0.55)], r, color, k);
        }
        Instruction::Left => {
            path_icon(p, &[(0.78, 0.85), (0.78, 0.5), (0.25, 0.5)], r, color, k);
            path_icon(p, &[(0.46, 0.23), (0.19, 0.5), (0.46, 0.77)], r, color, k);
        }
        Instruction::Right => {
            path_icon(p, &[(0.22, 0.85), (0.22, 0.5), (0.75, 0.5)], r, color, k);
            path_icon(p, &[(0.54, 0.23), (0.81, 0.5), (0.54, 0.77)], r, color, k);
        }
        Instruction::Jump => {
            path_icon(p, &[(0.15, 0.84), (0.15, 0.35), (0.78, 0.35)], r, color, k);
            path_icon(p, &[(0.6, 0.13), (0.84, 0.35), (0.6, 0.57)], r, color, k);
        }
        Instruction::Wait => {
            for x in [0.25, 0.58] {
                p.spawn((
                    bn(
                        r[0] + x * r[2],
                        r[1] + r[3] * 0.20,
                        r[2] * 0.17,
                        r[3] * 0.63,
                        k,
                    ),
                    BackgroundColor(color),
                    BorderRadius::all(Val::Px(3. * k)),
                ));
            }
        }
    }
}
pub(super) fn toolbar_icon(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    action: Action,
    r: [f32; 4],
    color: Color,
    k: f32,
) {
    match action {
        Action::Undo | Action::Reset => command_icon(p, art, Instruction::Left, r, color, k),
        Action::Erase | Action::Clear => {
            for path in [
                &[(0.29, 0.35), (0.33, 0.88), (0.74, 0.88), (0.78, 0.35)][..],
                &[(0.19, 0.25), (0.88, 0.25)][..],
                &[(0.39, 0.12), (0.66, 0.12)][..],
            ] {
                path_icon(p, path, r, color, k);
            }
        }
        Action::ResetView => {
            p.spawn((
                Node {
                    border: UiRect::all(Val::Px(4. * k)),
                    ..bn(
                        r[0] + r[2] * 0.17,
                        r[1] + r[3] * 0.17,
                        r[2] * 0.66,
                        r[3] * 0.66,
                        k,
                    )
                },
                BorderColor(color),
                BorderRadius::MAX,
            ));
            for path in [
                &[(0.5, 0.03), (0.5, 0.28)][..],
                &[(0.5, 0.72), (0.5, 0.97)][..],
                &[(0.03, 0.5), (0.28, 0.5)][..],
                &[(0.72, 0.5), (0.97, 0.5)][..],
            ] {
                path_icon(p, path, r, color, k);
            }
        }
        Action::Build => {
            path_icon(
                p,
                &[
                    (0.15, 0.30),
                    (0.5, 0.10),
                    (0.85, 0.30),
                    (0.85, 0.72),
                    (0.5, 0.92),
                    (0.15, 0.72),
                    (0.15, 0.30),
                    (0.5, 0.51),
                    (0.85, 0.30),
                ],
                r,
                color,
                k,
            );
            path_icon(p, &[(0.5, 0.51), (0.5, 0.92)], r, color, k);
        }
        Action::Step => path_icon(
            p,
            &[(0.23, 0.15), (0.83, 0.5), (0.23, 0.85), (0.23, 0.15)],
            r,
            color,
            k,
        ),
        Action::Help => {
            p.spawn((
                bn(r[0], r[1], r[2], r[3], k),
                BackgroundColor(Color::srgb(0.2, 0.60, 1.)),
                BorderRadius::MAX,
            ))
            .with_children(|p| {
                centered(
                    p,
                    art,
                    "?",
                    [0., 0., r[2], r[3]],
                    r[3] * 0.8,
                    Color::WHITE,
                    k,
                )
            });
        }
        _ => {}
    }
}

fn tile_rect(i: usize) -> Rect {
    // Tight alpha bounds of the eight objects in the generated 1774x887 atlas.
    let [x, y, r, b] = [
        [64., 35., 444., 443.],
        [528., 47., 871., 432.],
        [954., 65., 1282., 433.],
        [1396., 75., 1701., 432.],
        [80., 465., 430., 850.],
        [532., 469., 887., 843.],
        [1016., 469., 1247., 843.],
        [1478., 553., 1621., 779.],
    ][i];
    Rect::new(x, y, r, b)
}
pub(super) fn sprite(p: &mut ChildSpawnerCommands, art: &Art, index: usize, r: [f32; 4], k: f32) {
    p.spawn((
        bn(r[0], r[1], r[2], r[3], k),
        ImageNode {
            image: art.builder.tiles.clone(),
            rect: Some(tile_rect(index)),
            ..default()
        },
    ));
}
fn image(p: &mut ChildSpawnerCommands, handle: Handle<Image>, r: [f32; 4], k: f32) {
    p.spawn((bn(r[0], r[1], r[2], r[3], k), ImageNode::new(handle)));
}
fn line(p: &mut ChildSpawnerCommands, a: Vec2, b: Vec2, color: Color, k: f32) {
    let v = b - a;
    let mid = (a + b) * 0.5;
    p.spawn((
        bn(mid.x - v.length() * 0.5, mid.y - 0.9, v.length(), 1.8, k),
        BackgroundColor(color),
        Transform::from_rotation(Quat::from_rotation_z(v.y.atan2(v.x))),
    ));
}
fn guide(p: &mut ChildSpawnerCommands, proj: BuilderProjection, x: i32, k: f32) {
    let e = 0.48;
    for axis in 0..3 {
        for a in [-e, e] {
            for b in [-e, e] {
                let mut from = Vec3::new(x as f32, 0., 0.);
                let mut to = from;
                from[axis] -= e;
                to[axis] += e;
                from[(axis + 1) % 3] += a;
                to[(axis + 1) % 3] += a;
                from[(axis + 2) % 3] += b;
                to[(axis + 2) % 3] += b;
                line(
                    p,
                    proj.point(from),
                    proj.point(to),
                    Color::srgba(0.66, 0.97, 1., 0.95),
                    k,
                );
            }
        }
    }
}

fn scenery(p: &mut ChildSpawnerCommands, art: &Art, index: usize, r: [f32; 4], k: f32) {
    let [x, y, right, bottom] = [
        [35., 177., 538., 562.],
        [612., 226., 1075., 552.],
        [1125., 150., 1710., 575.],
        [1820., 88., 2050., 666.],
    ][index];
    p.spawn((
        bn(r[0], r[1], r[2], r[3], k),
        ImageNode {
            image: art.builder.scenery.clone(),
            rect: Some(Rect::new(x, y, right, bottom)),
            ..default()
        },
    ));
}
fn world(p: &mut ChildSpawnerCommands, s: &Session, art: &Art, k: f32) {
    if !builder_illustrated(s) {
        return;
    }
    let proj = BuilderProjection::new(s);
    let unit = proj.x * 2.;
    let mut blocks: Vec<([i32; 3], usize)> = Vec::new();
    for b in &s.work.world.blocks {
        if b.material == Material::Grass && b.pos[1] == 0 {
            let layers = if (4..=5).contains(&b.pos[0].abs()) && b.pos[2].abs() <= 1 {
                3
            } else if (4..=5).contains(&b.pos[0].abs()) && b.pos[2].abs() <= 2 {
                2
            } else {
                1
            };
            for n in 1..=layers {
                blocks.push(([b.pos[0], -n, b.pos[2]], 4));
            }
        }
        let index = match b.material {
            Material::Grass => 0,
            Material::Wood => 1,
            Material::Stone => 2,
            Material::Crystal | Material::Glass => 3,
            _ => 2,
        };
        blocks.push((b.pos, index));
    }
    blocks.sort_by_key(|(p, _)| (p[0] + p[2], p[1]));
    for (cell, index) in blocks {
        let mut v = proj.point(Vec3::new(cell[0] as f32, cell[1] as f32, cell[2] as f32));
        if cell[1] < 0 {
            // Decorative overlapping cliff courses do not create buildable cells.
            v.y -= -cell[1] as f32 * unit * 0.30;
        }
        let h = proj.y + 2. * proj.z;
        sprite(p, art, index, [v.x - unit * 0.5, v.y - h * 0.5, unit, h], k);
    }
    if s.work.level == 3 {
        for x in -2..=2 {
            if s.work.world.material([x, 0, 0]).is_none() {
                guide(p, proj, x, k);
            }
        }
    }
    // These trees are scenery at the established outer corners, not obstacles.
    for [x, z] in [[-4, 0], [5, -1]] {
        if s.work.world.material([x, 0, z]) == Some(Material::Grass) {
            let v = proj.point(Vec3::new(x as f32, 0.5, z as f32));
            sprite(
                p,
                art,
                5,
                [v.x - unit * 0.50, v.y - unit * 0.80, unit, unit * 0.80],
                k,
            );
        }
    }
    // Restore the approved edge dressing. These are scenery, not obstacles;
    // hide each decoration if the supporting editable grass cell is removed.
    for (index, cells, width, height) in [
        (0, [[-5, 1], [5, -2]], 0.58, 0.44),
        (1, [[-4, 2], [5, 1]], 0.46, 0.32),
        (2, [[-3, 2], [4, 2]], 0.82, 0.56),
    ] {
        if index == 2 && s.mode == GameMode::CloudBridge {
            continue;
        }
        for [x, z] in cells {
            if s.work.world.material([x, 0, z]) != Some(Material::Grass)
                || s.work.world.material([x, 1, z]).is_some()
            {
                continue;
            }
            let v = proj.point(Vec3::new(x as f32, 0.5, z as f32));
            scenery(
                p,
                art,
                index,
                [
                    v.x - unit * width * 0.5,
                    v.y - unit * height,
                    unit * width,
                    unit * height,
                ],
                k,
            );
        }
    }
    if s.mode == GameMode::Island {
        for [x, z] in [[-5, 2], [-3, 2], [3, 2], [5, 2]] {
            if s.work.world.material([x, 0, z]) == Some(Material::Grass) {
                let v = proj.point(Vec3::new(x as f32, 0., z as f32));
                scenery(
                    p,
                    art,
                    3,
                    [v.x - unit * 0.07, v.y, unit * 0.14, unit * 0.77],
                    k,
                );
            }
        }
    }
    for [x, z] in &s.work.world.crystals {
        let v = proj.point(Vec3::new(*x as f32, 1.05, *z as f32));
        sprite(
            p,
            art,
            7,
            [
                v.x - unit * 0.23,
                v.y - unit * 0.55,
                unit * 0.46,
                unit * 0.70,
            ],
            k,
        );
    }
    if s.work.level != 5 {
        let [x, z] = s.work.world.goal;
        let v = proj.point(Vec3::new(x as f32, 1., z as f32));
        image(
            p,
            art.island_goal_star.clone(),
            [
                v.x - unit * 0.43,
                v.y - unit * 0.65,
                unit * 0.86,
                unit * 0.86,
            ],
            k,
        );
        let [x, z] = s.work.world.robot;
        let v = proj.point(Vec3::new(x as f32, 0.5, z as f32));
        p.spawn((
            BuilderActor,
            bn(
                v.x - unit * 0.39,
                v.y - unit * 1.02,
                unit * 0.78,
                unit * 1.02,
                k,
            ),
            ImageNode {
                image: art.builder.tiles.clone(),
                rect: Some(tile_rect(6)),
                ..default()
            },
        ));
    }
}

fn material(
    p: &mut ChildSpawnerCommands,
    s: &Session,
    art: &Art,
    m: Material,
    r: [f32; 4],
    cloud: bool,
    k: f32,
) {
    let selected = s.material == m && !s.erase;
    let col = accent(cloud);
    p.spawn((
        Button,
        Action::Material(m),
        Node {
            border: UiRect::all(Val::Px(if selected { 4. } else { 1.5 } * k)),
            ..bn(r[0], r[1], r[2], r[3], k)
        },
        BackgroundColor(if selected {
            Color::srgb(0.93, 0.98, 1.)
        } else {
            Color::WHITE
        }),
        BorderColor(if selected {
            col
        } else {
            Color::srgb(0.89, 0.89, 0.86)
        }),
        BorderRadius::all(Val::Px(24. * k)),
    ))
    .with_children(|p| {
        let index = match m {
            Material::Grass => 0,
            Material::Wood => 1,
            Material::Stone => 2,
            _ => 3,
        };
        let size = (r[3] - 50.).min(r[2] - 16.);
        sprite(p, art, index, [(r[2] - size) * 0.5, 6., size, size], k);
        centered(
            p,
            art,
            m.label(),
            [0., r[3] - 43., r[2], 37.],
            27.,
            ink(cloud),
            k,
        );
        if selected {
            centered(p, art, "✓", [r[2] - 37., 5., 30., 30.], 24., col, k);
        }
    });
}
fn modes(
    p: &mut ChildSpawnerCommands,
    s: &Session,
    art: &Art,
    x: f32,
    y: f32,
    w: f32,
    cloud: bool,
    k: f32,
) {
    let col = accent(cloud);
    let soft = Color::srgb(0.9, 0.95, 0.99);
    for (i, (title, action, selected)) in [
        ("搭建", Action::Build, s.build),
        ("编程", Action::Code, !s.build),
    ]
    .into_iter()
    .enumerate()
    {
        control(
            p,
            art,
            title,
            action,
            [x + i as f32 * w * 0.5, y, w * 0.5, 58.],
            if selected { col } else { soft },
            if selected { Color::WHITE } else { ink(cloud) },
            26.,
            k,
        );
    }
}
fn command_info(op: Instruction) -> (&'static str, &'static str, Color, Color) {
    match op {
        Instruction::Place => (
            "◆",
            "放木块",
            Color::srgb(1., 0.92, 0.75),
            Color::srgb(0.88, 0.46, 0.05),
        ),
        Instruction::Forward => (
            "⬆",
            "前进",
            Color::srgb(0.82, 0.93, 1.),
            Color::srgb(0.04, 0.52, 0.98),
        ),
        Instruction::Left => (
            "↶",
            "左转",
            Color::srgb(0.79, 0.96, 0.87),
            Color::srgb(0.03, 0.65, 0.34),
        ),
        Instruction::Right => (
            "↷",
            "右转",
            Color::srgb(1., 0.83, 0.86),
            Color::srgb(0.9, 0.25, 0.38),
        ),
        Instruction::Back => (
            "⬇",
            "后退",
            Color::srgb(0.89, 0.85, 1.),
            Color::srgb(0.54, 0.30, 0.87),
        ),
        Instruction::Jump => (
            "↪",
            "跳跃",
            Color::srgb(1., 0.89, 0.69),
            Color::srgb(0.9, 0.49, 0.03),
        ),
        Instruction::Wait => (
            "Ⅱ",
            "停留",
            Color::srgb(0.87, 0.91, 0.94),
            Color::srgb(0.43, 0.52, 0.61),
        ),
    }
}

pub(super) fn builder_ui(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    super::builder_puzzle_ui::puzzle_ui(commands,s,art,w,h);
}
#[allow(dead_code)]
fn legacy_builder_ui(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let cloud = s.mode == GameMode::CloudBridge;
    let (k, offset) = builder_canvas(w, h);
    let col = accent(cloud);
    let dark = ink(cloud);
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
                .spawn(bn(offset.x / k, offset.y / k, BW, BH, k))
                .with_children(|root| {
                    world(root, s, art, k);
                    panel(root, art, [22., 16., 1628., 82.], cloud, k, |bar| {
                        control(
                            bar,
                            art,
                            "←",
                            Action::Library,
                            [0., 0., 82., 76.],
                            ivory(),
                            dark,
                            39.,
                            k,
                        );
                        sprite(bar, art, 0, [111., 5., 58., 58.], k);
                        text(
                            bar,
                            art,
                            if cloud {
                                "云桥工程师"
                            } else {
                                "小小建造师"
                            },
                            [185., 10., 290., 55.],
                            39.,
                            dark,
                            k,
                        );
                        if !cloud {
                            text(bar, art, "磁力小岛", [416., 24., 150., 35.], 23., dark, k);
                        }
                        let progress_x = if cloud { 570. } else { 880. };
                        if !cloud {
                            text(
                                bar,
                                art,
                                format!(
                                    "第{}关 · {}",
                                    s.work.level + 1,
                                    ["新手村", "森林边", "风车岛", "云桥谷", "矿车站", "家园挑战"]
                                        [s.work.level]
                                ),
                                [623., 21., 247., 40.],
                                27.,
                                dark,
                                k,
                            );
                        }
                        for stage in 0..6 {
                            let available = !cloud || stage <= s.cloud_bridge_campaign.unlocked;
                            let fill = if stage == s.work.level {
                                Color::srgb(0.95, 0.65, 0.04)
                            } else if stage < s.work.level {
                                Color::srgb(0.02, 0.68, 0.34)
                            } else {
                                Color::srgb(0.60, 0.68, 0.73)
                            };
                            let x = progress_x + stage as f32 * if cloud { 92. } else { 59. };
                            control(
                                bar,
                                art,
                                if cloud && stage == 5 {
                                    "家".to_owned()
                                } else {
                                    (stage + 1).to_string()
                                },
                                if cloud {
                                    Action::CloudLevel(stage)
                                } else {
                                    Action::BuilderLevel(stage)
                                },
                                [x, 10., if cloud { 46. } else { 38. }, 46.],
                                fill,
                                Color::WHITE,
                                if cloud { 27. } else { 23. },
                                k,
                            );
                            if stage < 5 {
                                centered(
                                    bar,
                                    art,
                                    "—",
                                    [
                                        x + if cloud { 48. } else { 39. },
                                        12.,
                                        if cloud { 42. } else { 20. },
                                        42.,
                                    ],
                                    22.,
                                    if available {
                                        col
                                    } else {
                                        Color::srgb(0.7, 0.75, 0.8)
                                    },
                                    k,
                                );
                            }
                        }
                        if cloud {
                            text(
                                bar,
                                art,
                                format!(
                                    "第{}关 · {}",
                                    s.work.level + 1,
                                    ["新手村", "森林边", "风车岛", "云桥谷", "矿车站", "家园挑战"]
                                        [s.work.level]
                                ),
                                [1220., 18., 240., 42.],
                                27.,
                                dark,
                                k,
                            );
                        } else {
                            island_score_stars_at(bar, s, art, k);
                        }
                        control(
                            bar,
                            art,
                            "? 怎么玩",
                            Action::Help,
                            [
                                if cloud { 1460. } else { 1408. },
                                5.,
                                if cloud { 145. } else { 135. },
                                62.,
                            ],
                            if cloud {
                                Color::srgb(0.87, 0.94, 1.)
                            } else {
                                ivory()
                            },
                            dark,
                            25.,
                            k,
                        );
                        if !cloud {
                            control(
                                bar,
                                art,
                                "…",
                                Action::ToggleIslandMenu,
                                [1554., 5., 57., 62.],
                                ivory(),
                                dark,
                                30.,
                                k,
                            );
                        }
                    });
                    let total = game::LEVEL_CRYSTALS[s.work.level];
                    let got = total.saturating_sub(s.work.world.crystals.len());
                    if cloud {
                        panel(root, art, [30., 110., 425., 67.], true, k, |p| {
                            sprite(p, art, 7, [16., 9., 42., 48.], k);
                            text(
                                p,
                                art,
                                format!("星晶 {got}/{total}  →"),
                                [80., 19., 170., 35.],
                                27.,
                                dark,
                                k,
                            );
                            image(p, art.island_goal_star.clone(), [242., 8., 49., 49.], k);
                            text(
                                p,
                                art,
                                if s.work.level == 5 {
                                    "自由搭建"
                                } else {
                                    "抵达终点"
                                },
                                [300., 20., 130., 35.],
                                26.,
                                dark,
                                k,
                            );
                        });
                    } else {
                        panel(root, art, [25., 120., 296., 155.], false, k, |p| {
                            text(p, art, "本关目标", [25., 13., 245., 44.], 31., dark, k);
                            text(
                                p,
                                art,
                                if s.work.level == 5 {
                                    "搭10块，使用3种材料"
                                } else {
                                    "收集星晶，再到达星星。"
                                },
                                [25., 59., 250., 33.],
                                23.,
                                dark,
                                k,
                            );
                            sprite(p, art, 7, [28., 103., 38., 37.], k);
                            text(
                                p,
                                art,
                                if s.work.level == 5 {
                                    let (n, m) = s.work.creative_progress();
                                    format!("{n}/10 块 · {m}/3 种")
                                } else {
                                    format!("星晶 {got}/{total}")
                                },
                                [83., 105., 210., 35.],
                                27.,
                                dark,
                                k,
                            );
                        });
                    }
                    if cloud || s.build {
                        let rail = if cloud {
                            [1465., 120., 183., 636.]
                        } else {
                            [1345., 146., 310., 445.]
                        };
                        panel(root, art, rail, cloud, k, |p| {
                            text(
                                p,
                                art,
                                if cloud { "材料" } else { "积木盒" },
                                [if cloud { 66. } else { 25. }, 17., 240., 42.],
                                30.,
                                dark,
                                k,
                            );
                            if cloud {
                                for (i, m) in [Material::Wood, Material::Grass, Material::Stone]
                                    .into_iter()
                                    .enumerate()
                                {
                                    material(
                                        p,
                                        s,
                                        art,
                                        m,
                                        [19., 65. + i as f32 * 160., 144., 149.],
                                        cloud,
                                        k,
                                    );
                                }
                                modes(p, s, art, 12., 554., 158., true, k);
                            } else {
                                for (i, m) in [
                                    Material::Grass,
                                    Material::Wood,
                                    Material::Stone,
                                    Material::Crystal,
                                ]
                                .into_iter()
                                .enumerate()
                                {
                                    material(
                                        p,
                                        s,
                                        art,
                                        m,
                                        [
                                            16. + (i % 2) as f32 * 144.,
                                            78. + (i / 2) as f32 * 178.,
                                            132.,
                                            164.,
                                        ],
                                        false,
                                        k,
                                    );
                                }
                            }
                        });
                    } else {
                        panel(root, art, [1304., 118., 352., 470.], false, k, |p| {
                            modes(p, s, art, 18., 14., 311., false, k);
                            text(p, art, "执行设置", [24., 88., 260., 40.], 30., dark, k);
                            text(p, art, "重复次数", [27., 157., 145., 38.], 25., dark, k);
                            control(
                                p,
                                art,
                                "−",
                                Action::RepeatDelta(-1),
                                [145., 141., 52., 55.],
                                ivory(),
                                dark,
                                35.,
                                k,
                            );
                            centered(
                                p,
                                art,
                                s.work.repeat.to_string(),
                                [203., 142., 55., 55.],
                                35.,
                                dark,
                                k,
                            );
                            control(
                                p,
                                art,
                                "+",
                                Action::RepeatDelta(1),
                                [266., 141., 58., 55.],
                                col,
                                Color::WHITE,
                                35.,
                                k,
                            );
                            text(
                                p,
                                art,
                                "结束后\n前进一步",
                                [27., 227., 160., 76.],
                                25.,
                                dark,
                                k,
                            );
                            control(
                                p,
                                art,
                                if s.work.finish_forward {
                                    "●  开"
                                } else {
                                    "关  ○"
                                },
                                Action::Finish,
                                [224., 235., 104., 53.],
                                if s.work.finish_forward {
                                    col
                                } else {
                                    Color::srgb(0.82, 0.84, 0.82)
                                },
                                Color::WHITE,
                                25.,
                                k,
                            );
                            control(
                                p,
                                art,
                                "▶ 示范路线",
                                Action::GuideRoute,
                                [25., 315., 296., 57.],
                                Color::srgb(0.96, 0.96, 0.95),
                                dark,
                                27.,
                                k,
                            );
                            control(
                                p,
                                art,
                                "▷ 单步",
                                Action::Step,
                                [25., 385., 142., 62.],
                                ivory(),
                                dark,
                                27.,
                                k,
                            );
                            control(
                                p,
                                art,
                                "↶ 复位",
                                Action::Reset,
                                [177., 385., 144., 62.],
                                ivory(),
                                dark,
                                27.,
                                k,
                            );
                        });
                    }
                    if s.build {
                        panel(
                            root,
                            art,
                            [
                                22.,
                                if cloud { 786. } else { 768. },
                                1628.,
                                if cloud { 130. } else { 156. },
                            ],
                            cloud,
                            k,
                            |p| {
                                text(
                                    p,
                                    art,
                                    if cloud {
                                        "搭建云桥"
                                    } else {
                                        "搭建模式"
                                    },
                                    [27., if cloud { 39. } else { 47. }, 220., 55.],
                                    37.,
                                    dark,
                                    k,
                                );
                                if cloud {
                                    text(
                                        p,
                                        art,
                                        "选择材料，再点格子搭桥。",
                                        [260., 45., 360., 38.],
                                        26.,
                                        dark,
                                        k,
                                    );
                                } else {
                                    text(
                                        p,
                                        art,
                                        "✓ 已自动保存",
                                        [199., 56., 190., 38.],
                                        21.,
                                        Color::srgb(0.55, 0.59, 0.52),
                                        k,
                                    );
                                    text(
                                        p,
                                        art,
                                        "选材料，点格子放置。",
                                        [402., 8., 410., 34.],
                                        24.,
                                        dark,
                                        k,
                                    );
                                    control(
                                        p,
                                        art,
                                        "◆ 搭建",
                                        Action::Build,
                                        [367., 49., 213., 79.],
                                        col,
                                        Color::WHITE,
                                        30.,
                                        k,
                                    );
                                }
                                let x = if cloud { 621. } else { 594. };
                                let ww = if cloud { 181. } else { 204. };
                                let yy = if cloud { 24. } else { 49. };
                                for (i, (name, action)) in [
                                    ("▥ 拆除", Action::Erase),
                                    ("↶ 撤销", Action::Undo),
                                    ("◎ 视角回正", Action::ResetView),
                                ]
                                .into_iter()
                                .enumerate()
                                {
                                    control(
                                        p,
                                        art,
                                        name,
                                        action,
                                        [x + i as f32 * (ww + 15.), yy, ww, 79.],
                                        if s.erase && i == 0 {
                                            Color::srgb(1., 0.77, 0.74)
                                        } else if cloud {
                                            Color::srgb(0.89, 0.95, 1.)
                                        } else {
                                            ivory()
                                        },
                                        dark,
                                        29.,
                                        k,
                                    );
                                }
                                control(
                                    p,
                                    art,
                                    "编程 →",
                                    Action::Code,
                                    [1305., if cloud { 15. } else { 28. }, 302., 96.],
                                    col,
                                    Color::WHITE,
                                    40.,
                                    k,
                                );
                            },
                        );
                    } else {
                        let dock_width = if cloud { 1418. } else { 1628. };
                        panel(root, art, [22., 600., dock_width, 312.], cloud, k, |p| {
                            text(
                                p,
                                art,
                                if cloud {
                                    "我的行动"
                                } else {
                                    "我的指令"
                                },
                                [24., 10., 200., 46.],
                                33.,
                                dark,
                                k,
                            );
                            text(
                                p,
                                art,
                                format!("{}/16", s.work.program.len()),
                                [218., 17., 120., 35.],
                                25.,
                                Color::srgb(0.42, 0.49, 0.54),
                                k,
                            );
                            text(
                                p,
                                art,
                                &s.message,
                                [350., 6., dock_width - 510., 46.],
                                19.,
                                dark,
                                k,
                            );
                            control(
                                p,
                                art,
                                "▥ 清空",
                                Action::Clear,
                                [dock_width - 142., 5., 123., 43.],
                                if cloud {
                                    Color::srgb(0.89, 0.95, 1.)
                                } else {
                                    ivory()
                                },
                                dark,
                                23.,
                                k,
                            );
                            let gap = 10.;
                            let slot = (dock_width - 48. - 7. * gap) / 8.;
                            let active = s
                                .runner
                                .as_ref()
                                .map(|r| r.pc.saturating_sub(1) % s.work.program.len().max(1));
                            for i in 0..16 {
                                let r = [
                                    24. + (i % 8) as f32 * (slot + gap),
                                    60. + (i / 8) as f32 * 83.,
                                    slot,
                                    68.,
                                ];
                                if let Some(op) = s.work.program.get(i).copied() {
                                    let (_icon, title, bg, tone) = command_info(op);
                                    p.spawn((
                                        Button,
                                        Action::Remove(i),
                                        Node {
                                            border: UiRect::all(Val::Px(
                                                if active == Some(i) { 3. } else { 1. } * k,
                                            )),
                                            ..bn(r[0], r[1], r[2], r[3], k)
                                        },
                                        BackgroundColor(bg),
                                        BorderColor(if active == Some(i) {
                                            accent(cloud)
                                        } else {
                                            Color::srgba(0.6, 0.7, 0.7, 0.25)
                                        }),
                                        BorderRadius::all(Val::Px(15. * k)),
                                    ))
                                    .with_children(|p| {
                                        text(
                                            p,
                                            art,
                                            (i + 1).to_string(),
                                            [10., 7., 30., 30.],
                                            20.,
                                            tone,
                                            k,
                                        );
                                        command_icon(p, art, op, [44., 5., 40., 38.], tone, k);
                                        text(
                                            p,
                                            art,
                                            title,
                                            [
                                                if slot > 180. { 95. } else { 51. },
                                                if slot > 180. { 20. } else { 42. },
                                                90.,
                                                30.,
                                            ],
                                            if slot > 180. { 24. } else { 19. },
                                            dark,
                                            k,
                                        );
                                        text(p, art, "×", [slot - 28., 2., 26., 28.], 25., tone, k);
                                    });
                                } else {
                                    p.spawn((
                                        Node {
                                            border: UiRect::all(Val::Px(k)),
                                            ..bn(r[0], r[1], r[2], r[3], k)
                                        },
                                        BorderColor(Color::srgba(0.68, 0.69, 0.64, 0.38)),
                                        BorderRadius::all(Val::Px(15. * k)),
                                    ))
                                    .with_children(|p| {
                                        text(
                                            p,
                                            art,
                                            (i + 1).to_string(),
                                            [12., 8., 40., 32.],
                                            22.,
                                            Color::srgb(0.63, 0.65, 0.63),
                                            k,
                                        )
                                    });
                                }
                            }
                        });
                        // The available palette and run control have a separate fixed row.
                        let palette = |p: &mut ChildSpawnerCommands| {
                            for (i, op) in [
                                Instruction::Place,
                                Instruction::Forward,
                                Instruction::Left,
                                Instruction::Right,
                                Instruction::Back,
                                Instruction::Jump,
                                Instruction::Wait,
                            ]
                            .into_iter()
                            .enumerate()
                            {
                                let (_icon, title, bg, tone) = command_info(op);
                                let x = if cloud { 0. } else { 15. } + i as f32 * 180.;
                                control(
                                    p,
                                    art,
                                    "",
                                    Action::Add(op),
                                    [x, 6., 171., 79.],
                                    if cloud { bg } else { ivory() },
                                    tone,
                                    27.,
                                    k,
                                );
                                // Icon and label don't intercept their parent action.
                                command_icon(p, art, op, [x + 12., 18., 45., 49.], tone, k);
                                text(p, art, title, [x + 67., 30., 100., 42.], 27., dark, k);
                            }
                            control(
                                p,
                                art,
                                if s.runner.as_ref().is_some_and(|r| r.running) {
                                    "Ⅱ 暂停"
                                } else {
                                    "▶ 运行"
                                },
                                Action::Run,
                                [1305., 6., 302., 79.],
                                Color::srgb(0., 0.70, 0.31),
                                Color::WHITE,
                                38.,
                                k,
                            );
                        };
                        if cloud {
                            panel(root, art, [22., 813., 1628., 99.], true, k, palette);
                        } else {
                            root.spawn((WorldInputBlocker, bn(22., 813., 1628., 99., k)))
                                .with_children(palette);
                        }
                    }
                    if s.island_menu_open {
                        panel(root, art, [1334., 105., 310., 395.], cloud, k, |p| {
                            for (i, (title, action)) in [
                                ("上一关", Action::Previous),
                                ("下一关", Action::Next),
                                ("保存进度", Action::Save),
                                ("切换档案", Action::Profile(1 - s.save.active)),
                                ("切换立体视角", Action::ToggleIslandView),
                            ]
                            .into_iter()
                            .enumerate()
                            {
                                control(
                                    p,
                                    art,
                                    title,
                                    action,
                                    [15., 15. + i as f32 * 73., 270., 62.],
                                    ivory(),
                                    dark,
                                    25.,
                                    k,
                                );
                            }
                        });
                    }
                });
        });
}
fn island_score_stars_at(p: &mut ChildSpawnerCommands, s: &Session, art: &Art, k: f32) {
    p.spawn(bn(1260., 13., 132., 50., k))
        .with_children(|p| island_score_stars(p, art, s.work.stars[s.work.level], 39. * k));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn illustrated_projection_ray_hits_the_same_logical_cube() {
        let projection = BuilderProjection {
            origin: Vec2::new(600., 300.),
            x: 50.,
            z: 14.,
            y: 55.,
        };
        for point in [
            Vec3::new(-5., 0.5, -2.),
            Vec3::new(0., 0.5, 0.),
            Vec3::new(3., 2.5, 1.),
        ] {
            let ray = projection.ray(projection.point(point));
            let t = (point.y - ray.origin.y) / ray.direction.y;
            assert!((ray.origin + *ray.direction * t - point).length() < 0.0001);
        }
    }
}
