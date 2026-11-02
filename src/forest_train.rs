//! The approved forest railway screen, with independently moving trains.
use super::*;

const W: f32 = 1672.;
const H: f32 = 941.;
const SPEED: f32 = 86.;
const JUNCTIONS: [Vec2; 2] = [Vec2::new(612., 432.), Vec2::new(1126., 432.)];
const DIRECT: &[(f32, f32)] = &[
    (190., 390.),
    (300., 389.),
    (430., 390.),
    (500., 397.),
    (552., 414.),
    (612., 432.),
];
const BRIDGE: &[(f32, f32)] = &[(612., 432.), (1126., 432.)];
const UPPER: &[(f32, f32)] = &[
    (1126., 432.),
    (1153., 383.),
    (1175., 315.),
    (1199., 283.),
    (1235., 273.),
    (1280., 277.),
    (1320., 285.),
];
const LOWER: &[(f32, f32)] = &[
    (1126., 432.),
    (1160., 465.),
    (1191., 523.),
    (1233., 579.),
    (1290., 616.),
    (1374., 648.),
];
// The waiting circuit follows the complete oval back into the west approach.
const LOOP: &[(f32, f32)] = &[
    (612., 432.),
    (564., 465.),
    (539., 512.),
    (514., 555.),
    (480., 583.),
    (430., 600.),
    (340., 606.),
    (249., 600.),
    (177., 581.),
    (128., 554.),
    (108., 521.),
    (111., 495.),
    (135., 466.),
    (180., 434.),
    (222., 407.),
    (255., 390.),
];

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rail {
    Direct,
    Bridge,
    Upper,
    Lower,
    Loop,
}
impl Rail {
    fn points(self) -> &'static [(f32, f32)] {
        match self {
            Self::Direct => DIRECT,
            Self::Bridge => BRIDGE,
            Self::Upper => UPPER,
            Self::Lower => LOWER,
            Self::Loop => LOOP,
        }
    }
    fn length(self) -> f32 {
        self.points()
            .windows(2)
            .map(|p| Vec2::from(p[0]).distance(Vec2::from(p[1])))
            .sum()
    }
    fn at(self, mut distance: f32) -> (Vec2, Vec2) {
        for pair in self.points().windows(2) {
            let a = Vec2::from(pair[0]);
            let b = Vec2::from(pair[1]);
            let length = a.distance(b);
            if distance <= length {
                return (
                    a.lerp(b, (distance / length).clamp(0., 1.)),
                    (b - a).normalize(),
                );
            }
            distance -= length;
        }
        let pts = self.points();
        (
            Vec2::from(*pts.last().unwrap()),
            (Vec2::from(pts[pts.len() - 1]) - Vec2::from(pts[pts.len() - 2])).normalize(),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum TrainStatus {
    Station,
    Moving,
    Delivered,
}
#[derive(Clone, Debug)]
struct Train {
    rail: Rail,
    distance: f32,
    direction: f32,
    status: TrainStatus,
}
impl Train {
    fn new(blue: bool) -> Self {
        let rail = if blue { Rail::Upper } else { Rail::Direct };
        Self {
            rail,
            distance: if blue { rail.length() } else { 0. },
            direction: if blue { -1. } else { 1. },
            status: TrainStatus::Station,
        }
    }
    fn point(&self) -> Vec2 {
        self.rail.at(self.distance).0
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Phase {
    Ready,
    Running,
    Paused,
    Failed,
    Won,
}
#[derive(Clone, Debug)]
struct Dispatch {
    trains: [Train; 2],
    held: [bool; 2],
    switches: [bool; 2], // loop / bear branch
    delivered: u8,
    wave: u8,
    phase: Phase,
    loops: u32,
}
impl Default for Dispatch {
    fn default() -> Self {
        Self {
            trains: [Train::new(false), Train::new(true)],
            held: [false; 2],
            switches: [false; 2],
            delivered: 0,
            wave: 0,
            phase: Phase::Ready,
            loops: 0,
        }
    }
}
#[derive(Resource, Default)]
pub(super) struct ForestTrain {
    run: Dispatch,
    snapshot: Option<Dispatch>,
    note: String,
    note_time: f32,
    clock: f32,
}
impl ForestTrain {
    fn note(&mut self, message: &str) {
        self.note = message.into();
        self.note_time = 3.5;
    }
    fn destination(&self) -> &'static str {
        if self.run.wave == 1 {
            "松鼠林"
        } else {
            "小熊镇"
        }
    }
    fn launch(&mut self) {
        if self.run.phase != Phase::Ready {
            return;
        }
        if self.run.held == [true, true] {
            self.note("两辆车都在等待，点击车站允许发车");
            return;
        }
        self.snapshot = Some(self.run.clone());
        self.run.phase = Phase::Running;
        self.release();
        self.note("让火车交替过桥，注意岔道方向");
    }
    fn release(&mut self) {
        for i in 0..2 {
            if !self.run.held[i] && self.run.trains[i].status == TrainStatus::Station {
                self.run.trains[i].status = TrainStatus::Moving;
            }
        }
    }
    fn station(&mut self, i: usize) {
        if !matches!(
            self.run.phase,
            Phase::Ready | Phase::Running | Phase::Paused
        ) {
            return;
        }
        if self.run.trains[i].status != TrainStatus::Station {
            self.note("火车已离站，撤回发车可重新安排");
            return;
        }
        self.run.held[i] = !self.run.held[i];
        self.note(if self.run.held[i] {
            "已安排等待，再点车站允许发车"
        } else {
            "已允许发车"
        });
        if self.run.phase == Phase::Running {
            self.release();
        }
    }
    fn occupied(&self, junction: usize) -> bool {
        self.run.trains.iter().any(|t| {
            t.status == TrainStatus::Moving && t.point().distance(JUNCTIONS[junction]) < 133.
        })
    }
    fn switch(&mut self, i: usize) {
        if matches!(self.run.phase, Phase::Failed | Phase::Won) {
            return;
        }
        if self.occupied(i) {
            self.note("火车正在通过，请等整列车离开岔道");
            return;
        }
        self.run.switches[i] = !self.run.switches[i];
        self.note(match (i, self.run.switches[i]) {
            (0, true) => "J1：驶入环线等待",
            (0, false) => "J1：沿主线前往木桥",
            (1, true) => "J2：通往小熊镇",
            _ => "J2：通往松鼠林",
        });
    }
    fn toggle_pause(&mut self) {
        self.run.phase = match self.run.phase {
            Phase::Running => Phase::Paused,
            Phase::Paused => Phase::Running,
            _ => return,
        };
        if self.run.phase == Phase::Running {
            self.release();
        }
    }
    fn undo(&mut self) {
        if let Some(state) = &self.snapshot {
            self.run = state.clone();
            self.note("已退回本轮发车前，可以重新安排");
        } else {
            self.note("先发车，再撤回这一轮安排");
        }
    }
    fn fail(&mut self, text: &str) {
        self.run.phase = Phase::Failed;
        self.note(text);
        self.note_time = f32::INFINITY;
    }
    fn enter(&mut self, i: usize, rail: Rail, direction: f32, distance: Option<f32>) {
        let t = &mut self.run.trains[i];
        t.rail = rail;
        t.direction = direction;
        t.distance = distance.unwrap_or(if direction > 0. { 0. } else { rail.length() });
    }
    fn arrive(&mut self, i: usize) {
        let t = self.run.trains[i].clone();
        match (i, t.rail, t.direction > 0.) {
            (0, Rail::Direct, true) => {
                if self.run.switches[0] {
                    self.enter(i, Rail::Loop, 1., None);
                    self.run.loops += 1;
                } else {
                    self.enter(i, Rail::Bridge, 1., None);
                }
            }
            (0, Rail::Loop, _) => self.enter(i, Rail::Direct, 1., Some(65.)),
            (0, Rail::Bridge, _) => self.enter(
                i,
                if self.run.switches[1] {
                    Rail::Lower
                } else {
                    Rail::Upper
                },
                1.,
                None,
            ),
            (0, Rail::Upper | Rail::Lower, _) => {
                let expected = if self.run.wave == 1 {
                    Rail::Upper
                } else {
                    Rail::Lower
                };
                if t.rail != expected {
                    self.fail("红车进错车站，点击“撤回发车”再安排");
                } else {
                    self.deliver(i);
                }
            }
            // An arriving train must see its own branch selected at the points.
            (1, Rail::Upper, false) => {
                if self.run.switches[1] {
                    self.enter(i, Rail::Lower, 1., None);
                } else {
                    self.enter(i, Rail::Bridge, -1., None);
                }
            }
            (1, Rail::Bridge, _) => {
                if self.run.switches[0] {
                    self.fail("蓝车回村需要 J1 主线，撤回后调整岔道");
                } else {
                    self.enter(i, Rail::Direct, -1., None);
                }
            }
            (1, Rail::Direct, false) => self.deliver(i),
            (1, Rail::Lower, _) => self.fail("蓝车要去兔子村，点击“撤回发车”再安排"),
            _ => self.fail("路线未接通，点击“撤回发车”重新安排"),
        }
    }
    fn deliver(&mut self, i: usize) {
        self.run.trains[i].status = TrainStatus::Delivered;
        self.run.delivered += 1;
        self.note(if i == 0 {
            "红车到站了！继续接送另一辆车"
        } else {
            "蓝车到兔子村了！继续接送另一辆车"
        });
        if self.run.delivered == 6 {
            self.run.phase = Phase::Won;
            self.note("全部送达！三颗星点亮，点击重开再挑战");
            self.note_time = f32::INFINITY;
        } else if self
            .run
            .trains
            .iter()
            .all(|t| t.status == TrainStatus::Delivered)
        {
            self.run.wave += 1;
            self.run.trains = [Train::new(false), Train::new(true)];
            self.run.held = [false; 2];
            self.run.phase = Phase::Ready;
            self.note("本轮全部到站，安排好下一轮再发车");
        }
    }
    fn step(&mut self, dt: f32) {
        if self.run.phase != Phase::Running {
            return;
        }
        for i in 0..2 {
            if self.run.trains[i].status != TrainStatus::Moving {
                continue;
            }
            let train = &mut self.run.trains[i];
            train.distance += train.direction * SPEED * dt;
            if train.distance < 0. || train.distance > train.rail.length() {
                self.arrive(i);
                if self.run.phase != Phase::Running {
                    return;
                }
            }
        }
        let [a, b] = &self.run.trains;
        if a.status == TrainStatus::Moving
            && b.status == TrainStatus::Moving
            && a.point().distance(b.point()) < 174.
        {
            self.fail("两辆车相遇了，撤回发车后安排一辆等待");
        }
    }
    fn advance(&mut self, dt: f32) {
        self.clock += dt;
        self.note_time = (self.note_time - dt).max(0.);
        let mut remaining = dt.min(0.2);
        while remaining > 0.00001 {
            let step = remaining.min(1. / 120.);
            self.step(step);
            remaining -= step;
        }
    }
    fn hint(&self) -> String {
        if self.note_time > 0. {
            return self.note.clone();
        }
        if self.run.phase == Phase::Ready && (self.clock as u32 / 4) % 2 == 0 {
            return "点击岔道切换方向 · 点击车站等待".into();
        }
        let phase = if self.run.phase == Phase::Paused {
            "暂停中 · "
        } else {
            ""
        };
        format!("{phase}红车→{} · 蓝车→兔子村", self.destination())
    }
    #[cfg(debug_assertions)]
    pub(super) fn qa_state(&self) -> serde_json::Value {
        serde_json::json!({"phase":format!("{:?}",self.run.phase), "delivered":self.run.delivered, "wave":self.run.wave,
            "held":self.run.held,"switches":self.run.switches,"loops":self.run.loops,
            "positions":self.run.trains.iter().map(|t|[t.point().x,t.point().y]).collect::<Vec<_>>()})
    }
}

#[derive(Resource)]
pub(super) struct ForestArt {
    approved: Handle<Image>,
    empty: Handle<Image>,
    trains: Handle<Image>,
    controls: Handle<Image>,
}
pub(super) fn setup_forest(mut commands: Commands, assets: Res<AssetServer>) {
    let load = |path: &str| {
        assets.load_with_settings(
            image_asset_path(path),
            |s: &mut bevy::image::ImageLoaderSettings| {
                s.sampler = bevy::image::ImageSampler::linear();
            },
        )
    };
    commands.insert_resource(ForestArt {
        approved: load("forest_train/approved.png"),
        empty: load("forest_train/empty-tracks.png"),
        trains: load("forest_train/trains-exact.png"),
        controls: load("forest_train/controls.png"),
    });
}
fn fitted(size: Vec2) -> (f32, Vec2) {
    let scale = (size.x / W).min(size.y / H);
    (scale, (size - Vec2::new(W, H) * scale) * 0.5)
}
#[cfg(debug_assertions)]
pub(super) fn forest_screen_point(p: Vec2, size: Vec2) -> Vec2 {
    let (k, offset) = fitted(size);
    offset + p * k
}
fn pointer(p: Vec2, size: Vec2) -> Option<Vec2> {
    let (scale, offset) = fitted(size);
    let p = (p - offset) / scale;
    Rect::new(0., 0., W, H).contains(p).then_some(p)
}
fn hit(g: &mut ForestTrain, s: &mut Session, p: Vec2) {
    if Rect::new(14., 0., 101., 81.).contains(p) {
        if g.run.phase == Phase::Running {
            g.toggle_pause();
        }
        s.library_open = true;
        s.world_dirty = true;
    } else if Rect::new(1571., 0., 1667., 81.).contains(p) {
        g.toggle_pause();
    } else if Rect::new(80., 831., 382., 922.).contains(p) {
        *g = ForestTrain::default();
    } else if Rect::new(1050., 830., 1294., 922.).contains(p) {
        g.undo();
    } else if Rect::new(1304., 829., 1625., 925.).contains(p) {
        g.launch();
    } else if p.distance(JUNCTIONS[0]) < 53. {
        g.switch(0);
    } else if p.distance(JUNCTIONS[1]) < 53. {
        g.switch(1);
    } else if Rect::new(45., 123., 496., 321.).contains(p) {
        g.station(0);
    } else if Rect::new(1355., 112., 1628., 309.).contains(p) {
        g.station(1);
    }
}
pub(super) fn forest_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut s: ResMut<Session>,
    mut g: ResMut<ForestTrain>,
) {
    if s.mode != GameMode::ForestTrain || s.library_open || s.help_open {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        if g.run.phase == Phase::Ready {
            g.launch();
        } else {
            g.toggle_pause();
        }
    }
    if keys.just_pressed(KeyCode::KeyP) || keys.just_pressed(KeyCode::Escape) {
        g.toggle_pause();
    }
    if keys.just_pressed(KeyCode::KeyR) {
        *g = ForestTrain::default();
    }
    if keys.just_pressed(KeyCode::KeyZ) {
        g.undo();
    }
    if keys.just_pressed(KeyCode::Digit1) {
        g.switch(0);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        g.switch(1);
    }
    if keys.just_pressed(KeyCode::KeyA) {
        g.station(0);
    }
    if keys.just_pressed(KeyCode::KeyB) {
        g.station(1);
    }
    // Native touch taps and mouse taps share the same aspect-fitted hit boxes.
    for point in touches.iter_just_pressed().map(|t| t.position()).chain(
        (touches.iter().next().is_none() && mouse.just_pressed(MouseButton::Left))
            .then(|| window.cursor_position())
            .flatten(),
    ) {
        if let Some(point) = pointer(point, Vec2::new(window.width(), window.height())) {
            hit(&mut g, &mut s, point);
        }
    }
    if !window.focused
        && std::env::var_os("MAGNET_CAPTURE").is_none()
        && g.run.phase == Phase::Running
    {
        g.toggle_pause();
    }
    g.advance(time.delta_secs());
    s.ui_dirty = true;
}
fn node(x: f32, y: f32, w: f32, h: f32, k: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x * k),
        top: Val::Px(y * k),
        width: Val::Px(w * k),
        height: Val::Px(h * k),
        ..default()
    }
}
fn picture(
    p: &mut ChildSpawnerCommands,
    image: &Handle<Image>,
    source: Rect,
    target: Rect,
    k: f32,
    flip: bool,
) {
    p.spawn((
        node(
            target.min.x,
            target.min.y,
            target.width(),
            target.height(),
            k,
        ),
        ImageNode {
            image: image.clone(),
            rect: Some(source),
            flip_x: flip,
            ..default()
        },
    ));
}
fn text(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    value: impl Into<String>,
    rect: Rect,
    size: f32,
    k: f32,
    color: Color,
) {
    p.spawn((
        node(rect.min.x, rect.min.y, rect.width(), rect.height(), k),
        Text::new(value),
        TextFont {
            font: art.font.clone(),
            font_size: size * k,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}
pub(super) fn forest_ui(
    commands: &mut Commands,
    g: &ForestTrain,
    art: &Art,
    a: &ForestArt,
    width: f32,
    height: f32,
) {
    let (k, offset) = fitted(Vec2::new(width, height));
    commands
        .spawn((
            UiRoot,
            WorldInputBlocker,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.13, 0.21, 0.11)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(offset.x),
                top: Val::Px(offset.y),
                width: Val::Px(W * k),
                height: Val::Px(H * k),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|p| {
                p.spawn((node(0., 0., W, H, k), ImageNode::new(a.approved.clone())));
                // Only replace the two cleared train regions. All other artwork is
                // sampled directly from the approved image, without regeneration.
                for rect in [
                    Rect::new(57., 285., 309., 415.),
                    Rect::new(1209., 127., 1452., 310.),
                    Rect::new(922., 22., 954., 62.),
                ] {
                    picture(p, &a.empty, rect, rect, k, false);
                }
                text(
                    p,
                    art,
                    g.run.delivered.to_string(),
                    Rect::new(923., 21., 954., 65.),
                    33.,
                    k,
                    Color::srgb(0.98, 0.40, 0.02),
                );
                for i in 0..(g.run.delivered / 2) as usize {
                    let x = [1113., 1174., 1233.][i];
                    picture(
                        p,
                        &a.controls,
                        Rect::new(220., 440., 360., 580.),
                        Rect::new(x - 48., -5., x + 48., 91.),
                        k,
                        false,
                    );
                }
                for (i, rect) in [
                    Rect::new(568., 391., 656., 475.),
                    Rect::new(1082., 391., 1171., 475.),
                ]
                .iter()
                .enumerate()
                {
                    if g.run.switches[i] {
                        let source = if i == 0 {
                            Rect::new(556., 388., 666., 485.)
                        } else {
                            Rect::new(1070., 388., 1180., 485.)
                        };
                        picture(p, &a.controls, source, *rect, k, true);
                    }
                    if g.occupied(i) {
                        p.spawn((
                            node(rect.min.x, rect.min.y, rect.width(), rect.height(), k),
                            BackgroundColor(Color::srgba(0.24, 0.16, 0.06, 0.22)),
                            BorderRadius::all(Val::Px(45. * k)),
                        ));
                    }
                }
                let blue_on_bridge = g.run.trains[1].status == TrainStatus::Moving
                    && g.run.trains[1].rail == Rail::Bridge;
                if blue_on_bridge || g.run.phase == Phase::Failed {
                    let green = Rect::new(689., 340., 707., 359.);
                    let red = Rect::new(1029., 340., 1047., 359.);
                    picture(p, &a.approved, red, green, k, false);
                    if blue_on_bridge && g.run.phase != Phase::Failed {
                        picture(p, &a.approved, green, red, k, false);
                    }
                }
                for (i, train) in g.run.trains.iter().enumerate() {
                    if train.status == TrainStatus::Delivered {
                        continue;
                    }
                    let foot = train.point();
                    let sprite = if i == 0 {
                        Rect::new(78., 292., 351., 451.)
                    } else {
                        Rect::new(1197., 175., 1460., 320.)
                    };
                    let size = if i == 0 {
                        Vec2::new(220., 128.)
                    } else {
                        Vec2::new(240., 132.)
                    };
                    let (_, tangent) = train.rail.at(train.distance);
                    let moving_right = tangent.x * train.direction > 0.;
                    let flip = if i == 0 { !moving_right } else { moving_right };
                    picture(
                        p,
                        &a.trains,
                        sprite,
                        Rect::from_corners(
                            foot - Vec2::new(size.x * 0.5, size.y - 20.),
                            foot + Vec2::new(size.x * 0.5, 20.),
                        ),
                        k,
                        flip,
                    );
                    if i == 1 {
                        for j in 0..3 {
                            let age = (g.clock * 0.6 + j as f32 / 3.).fract();
                            let diameter = 15. + age * 17.;
                            let chimney_x = foot.x + if moving_right { 66. } else { -66. };
                            p.spawn((
                                node(
                                    chimney_x + age * 27. - diameter * 0.5,
                                    foot.y - 96. - age * 65.,
                                    diameter,
                                    diameter,
                                    k,
                                ),
                                BackgroundColor(Color::srgba(0.94, 0.94, 0.90, (1. - age) * 0.6)),
                                BorderRadius::all(Val::Px(30. * k)),
                            ));
                        }
                    }
                }
                for i in 0..2 {
                    if g.run.held[i] && g.run.trains[i].status == TrainStatus::Station {
                        let rect = if i == 0 {
                            Rect::new(344., 260., 451., 298.)
                        } else {
                            Rect::new(1473., 261., 1580., 299.)
                        };
                        p.spawn((
                            node(rect.min.x, rect.min.y, rect.width(), rect.height(), k),
                            BackgroundColor(Color::srgba(0.99, 0.93, 0.79, 0.94)),
                            BorderRadius::all(Val::Px(13. * k)),
                        ));
                        text(
                            p,
                            art,
                            "Ⅱ 等待",
                            rect,
                            23.,
                            k,
                            Color::srgb(0.40, 0.24, 0.12),
                        );
                    }
                }
                // Match the approved cream instruction strip and preserve its frame.
                picture(
                    p,
                    &a.approved,
                    Rect::new(482., 857., 521., 899.),
                    Rect::new(516., 857., 989., 899.),
                    k,
                    false,
                );
                text(
                    p,
                    art,
                    g.hint(),
                    Rect::new(508., 861., 1007., 902.),
                    20.,
                    k,
                    Color::srgb(0.47, 0.29, 0.17),
                );
                if g.run.phase == Phase::Paused {
                    p.spawn((
                        node(1594., 17., 42., 43., k),
                        BackgroundColor(Color::srgb(0.85, 0.56, 0.31)),
                        BorderRadius::all(Val::Px(8. * k)),
                    ));
                    text(
                        p,
                        art,
                        "▶",
                        Rect::new(1591., 14., 1642., 61.),
                        33.,
                        k,
                        Color::srgb(0.27, 0.13, 0.04),
                    );
                }
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run_until(g: &mut ForestTrain, predicate: impl Fn(&ForestTrain) -> bool) {
        for _ in 0..16000 {
            if predicate(g) {
                return;
            }
            g.advance(1. / 60.);
        }
        panic!("Timed out: {:?}", g.run);
    }
    #[test]
    fn simultaneous_departures_collide_and_undo_restores_both_trains() {
        let mut g = ForestTrain::default();
        g.launch();
        run_until(&mut g, |g| g.run.phase == Phase::Failed);
        assert!(g.note.contains("相遇"));
        g.undo();
        assert_eq!(g.run.phase, Phase::Ready);
        assert_eq!(g.run.delivered, 0);
        assert!(
            g.run
                .trains
                .iter()
                .all(|t| t.status == TrainStatus::Station)
        );
    }
    #[test]
    fn all_six_deliveries_require_correct_switches_and_each_wave_can_be_finished() {
        let mut g = ForestTrain::default();
        for wave in 0..3 {
            g.run.switches = [false, false];
            g.station(0);
            g.launch();
            run_until(&mut g, |g| {
                g.run.trains[1].status == TrainStatus::Delivered || g.run.phase == Phase::Failed
            });
            assert_ne!(g.run.phase, Phase::Failed);
            if wave != 1 {
                g.switch(1);
            }
            g.station(0);
            run_until(&mut g, |g| g.run.phase != Phase::Running);
            assert_eq!(g.run.delivered, (wave + 1) * 2);
        }
        assert_eq!(g.run.phase, Phase::Won);
    }
    #[test]
    fn wrong_destination_does_not_count_as_a_delivery() {
        let mut g = ForestTrain::default();
        g.station(1);
        g.launch();
        run_until(&mut g, |g| g.run.phase == Phase::Failed);
        assert_eq!(g.run.delivered, 0);
        assert!(g.note.contains("进错"));
        g.undo();
        assert_eq!(g.run.phase, Phase::Ready);
        assert_eq!(g.run.delivered, 0);
    }
    #[test]
    fn rewind_keeps_finished_waves_but_removes_this_waves_partial_delivery() {
        let mut g = ForestTrain::default();
        g.station(0);
        g.launch();
        run_until(&mut g, |g| g.run.delivered == 1);
        g.switch(1);
        g.station(0);
        run_until(&mut g, |g| g.run.phase == Phase::Ready);
        assert_eq!(g.run.delivered, 2);
        g.switch(1);
        g.station(0);
        g.launch();
        run_until(&mut g, |g| g.run.delivered == 3);
        g.switch(1); // Second-wave red train needs Upper, not Lower.
        g.station(0);
        run_until(&mut g, |g| g.run.phase == Phase::Failed);
        g.undo();
        assert_eq!(g.run.delivered, 2);
        assert_eq!(g.run.wave, 1);
        assert_eq!(g.run.phase, Phase::Ready);
        assert_eq!(g.run.held, [true, false]);
    }
    #[test]
    fn switch_is_locked_while_a_whole_train_occupies_junction() {
        let mut g = ForestTrain::default();
        g.run.trains[0].status = TrainStatus::Moving;
        g.run.trains[0].distance = Rail::Direct.length() - 100.;
        g.switch(0);
        assert!(!g.run.switches[0]);
        assert!(g.note.contains("正在通过"));
    }
    #[test]
    fn paused_trains_do_not_move_and_station_hold_does_not_release_until_resumed() {
        let mut g = ForestTrain::default();
        g.station(0);
        g.launch();
        g.toggle_pause();
        let distance = g.run.trains[1].distance;
        g.station(0);
        g.advance(0.2);
        assert_eq!(g.run.trains[0].status, TrainStatus::Station);
        assert_eq!(g.run.trains[1].distance, distance);
        g.toggle_pause();
        g.advance(0.1);
        assert_eq!(g.run.trains[0].status, TrainStatus::Moving);
    }
    #[test]
    fn red_train_can_take_the_loop_and_return_to_mainline() {
        let mut g = ForestTrain::default();
        g.station(1);
        g.switch(0);
        g.launch();
        run_until(&mut g, |g| g.run.trains[0].rail == Rail::Loop);
        run_until(&mut g, |g| !g.occupied(0));
        g.switch(0);
        g.switch(1);
        run_until(&mut g, |g| {
            g.run.trains[0].status == TrainStatus::Delivered || g.run.phase == Phase::Failed
        });
        assert_ne!(g.run.phase, Phase::Failed);
        assert_eq!(g.run.delivered, 1);
        assert_eq!(g.run.loops, 1);
    }
    #[test]
    fn aspect_fit_rejects_letterbox_and_preserves_touch_coordinates() {
        for size in [
            Vec2::new(960., 540.),
            Vec2::new(2000., 900.),
            Vec2::new(1280., 800.),
        ] {
            let (k, offset) = fitted(size);
            assert!(
                pointer(offset + JUNCTIONS[0] * k, size)
                    .unwrap()
                    .distance(JUNCTIONS[0])
                    < 0.001
            );
            if offset.y > 0. {
                assert!(pointer(Vec2::new(1., 0.), size).is_none());
            }
            if offset.x > 0. {
                assert!(pointer(Vec2::ZERO, size).is_none());
            }
        }
    }
}
