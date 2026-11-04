//! Full-width vertical mine descent. Physics use the approved 1024×1536 canvas.
use super::*;

const W: f32 = 1024.;
const H: f32 = 1536.;
const HALF: f32 = 22.;
const BODY: f32 = 66.;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Stone,
    Cracked,
    Conveyor,
    Spikes,
    Spring,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum DescentState {
    Ready,
    Playing,
    Paused,
    Over,
    Cleared,
}
#[derive(Clone)]
struct Ledge {
    x: f32,
    y: f32,
    w: f32,
    kind: Tile,
    crumble: Option<f32>,
    gone: bool,
}
impl Ledge {
    fn new(x: f32, y: f32, w: f32, kind: Tile) -> Self {
        Self {
            x,
            y,
            w,
            kind,
            crumble: None,
            gone: false,
        }
    }
}
pub(super) struct DescentGame {
    pub(super) state: DescentState,
    pub(super) floor: u32,
    pub(super) best: u32,
    ledges: Vec<Ledge>,
    x: f32,
    feet: f32,
    vy: f32,
    camera: f32,
    deepest: f32,
    hearts: u8,
    immunity: f32,
    standing: Option<usize>,
    seed: u64,
    next_y: f32,
    route_x: f32,
    elapsed: f32,
    direction: i8,
    moving: bool,
    endless: bool,
}
impl DescentGame {
    pub(super) fn new() -> Self {
        let best = fs::read_to_string(save_directory().join("descent-best.txt"))
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        Self::fresh(best)
    }
    fn fresh(best: u32) -> Self {
        use Tile::*;
        let ledges = vec![
            Ledge::new(64., 202., 160., Spikes),
            Ledge::new(821., 167., 140., Stone),
            Ledge::new(254., 376., 216., Stone),
            Ledge::new(646., 351., 200., Conveyor),
            Ledge::new(31., 520., 188., Cracked),
            Ledge::new(768., 480., 193., Stone),
            Ledge::new(509., 554., 138., Spring),
            Ledge::new(809., 661., 180., Spikes),
            Ledge::new(148., 697., 256., Stone),
            Ledge::new(554., 838., 210., Cracked),
            Ledge::new(33., 892., 222., Stone),
            Ledge::new(834., 936., 158., Stone),
            Ledge::new(110., 1080., 238., Conveyor),
            Ledge::new(686., 1040., 136., Spring),
            Ledge::new(360., 1164., 168., Stone),
            Ledge::new(769., 1281., 217., Stone),
            Ledge::new(31., 1287., 174., Stone),
            Ledge::new(529., 1358., 165., Spikes),
        ];
        Self {
            state: DescentState::Ready,
            floor: 1,
            best,
            ledges,
            x: 384.,
            feet: 376.,
            vy: 0.,
            camera: 0.,
            deepest: 376.,
            hearts: 5,
            immunity: 0.,
            standing: Some(2),
            seed: 0x93ab219,
            next_y: 1470.,
            route_x: 650.,
            elapsed: 0.,
            direction: 0,
            moving: false,
            endless: false,
        }
    }
    #[cfg(debug_assertions)]
    pub(super) fn qa_x(&self) -> f32 {
        self.x
    }
    pub(super) fn start(&mut self) {
        *self = Self::fresh(self.best);
        self.state = DescentState::Playing;
    }
    pub(super) fn toggle(&mut self) {
        self.state = match self.state {
            DescentState::Playing => DescentState::Paused,
            DescentState::Paused => DescentState::Playing,
            s => s,
        };
    }
    pub(super) fn continue_endless(&mut self) {
        self.endless = true;
        self.state = DescentState::Playing;
    }
    fn rand(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.seed >> 32) as u32) as f32 / u32::MAX as f32
    }
    fn generate(&mut self) {
        while self.next_y < self.camera + H + 360. {
            let y = self.next_y;
            let x = (self.route_x + (self.rand() - 0.5) * 560.).clamp(32., 792.);
            let width = 190. + self.rand() * 55.;
            let r = self.rand();
            let kind = if r < 0.18 {
                Tile::Cracked
            } else if r < 0.31 {
                Tile::Conveyor
            } else if r < 0.41 {
                Tile::Spring
            } else {
                Tile::Stone
            };
            self.ledges.push(Ledge::new(x, y, width, kind));
            if self.rand() < 0.4 {
                let hazard_x = if x < 430. { 790. } else { 40. };
                self.ledges
                    .push(Ledge::new(hazard_x, y + 30., 170., Tile::Spikes));
            }
            self.route_x = x;
            self.next_y += 128. + self.rand() * 40.;
        }
        // Bound memory even in endless mode; references are reacquired on the next step.
        if self.ledges.len() > 100 {
            self.ledges.retain(|p| p.y > self.camera - 180.);
            self.standing = None;
        }
    }
    fn remember(&mut self) {
        if self.floor > self.best {
            self.best = self.floor;
            let directory = save_directory();
            let result = fs::create_dir_all(&directory)
                .and_then(|_| fs::write(directory.join("descent-best.txt"), self.best.to_string()));
            if let Err(e) = result {
                eprintln!("Cannot save descent record: {e}");
            }
        }
    }
    fn damage(&mut self) {
        if self.immunity <= 0. {
            self.hearts = self.hearts.saturating_sub(1);
            self.immunity = 1.2;
            if self.hearts == 0 {
                self.state = DescentState::Over;
            }
        }
    }
    fn step(&mut self, dt: f32, input: i8) {
        self.elapsed += dt;
        self.immunity = (self.immunity - dt).max(0.);
        let speed = (64. + (self.floor as f32) * 1.1).min(190.);
        self.camera += speed * dt;
        self.moving = input != 0;
        self.direction = if input == 0 { self.direction } else { input };
        let belt = self
            .standing
            .and_then(|i| self.ledges.get(i))
            .filter(|p| !p.gone && p.kind == Tile::Conveyor)
            .map_or(0., |_| 170.);
        self.x = (self.x + (input as f32 * 400. + belt) * dt).clamp(26. + HALF, W - 26. - HALF);
        for p in &mut self.ledges {
            if let Some(t) = &mut p.crumble {
                *t -= dt;
                if *t <= 0. {
                    p.gone = true;
                }
            }
        }
        let supported = self
            .standing
            .and_then(|i| self.ledges.get(i))
            .is_some_and(|p| !p.gone && self.x + HALF > p.x && self.x - HALF < p.x + p.w);
        if !supported {
            self.standing = None;
        }
        if self.standing.is_none() {
            let before = self.feet;
            self.vy = (self.vy + 1250. * dt).min(850.);
            self.feet += self.vy * dt;
            if self.vy >= 0. {
                let hit = self
                    .ledges
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| {
                        !p.gone
                            && before <= p.y + 0.1
                            && self.feet >= p.y
                            && self.x + HALF > p.x
                            && self.x - HALF < p.x + p.w
                    })
                    .min_by(|(_, a), (_, b)| a.y.total_cmp(&b.y))
                    .map(|(i, _)| i);
                if let Some(i) = hit {
                    self.feet = self.ledges[i].y;
                    self.vy = 0.;
                    self.standing = Some(i);
                    match self.ledges[i].kind {
                        Tile::Spring => {
                            self.vy = -650.;
                            self.standing = None;
                        }
                        Tile::Cracked => {
                            if self.ledges[i].crumble.is_none() {
                                self.ledges[i].crumble = Some(0.65);
                            }
                        }
                        Tile::Spikes => self.damage(),
                        _ => {}
                    }
                }
            }
        } else if self
            .standing
            .is_some_and(|i| self.ledges[i].kind == Tile::Spikes)
        {
            self.damage();
        }
        self.deepest = self.deepest.max(self.feet);
        self.floor = 1 + ((self.deepest - 376.).max(0.) / 140.) as u32;
        if self.feet - self.camera - BODY <= 110. || self.feet - self.camera > H + BODY {
            self.state = DescentState::Over;
        }
        if self.floor >= 100 && !self.endless && self.state == DescentState::Playing {
            self.state = DescentState::Cleared;
        }
        self.generate();
    }
    fn advance(&mut self, dt: f32, input: i8) {
        if self.state != DescentState::Playing {
            return;
        }
        // Substeps preserve one-way landing even after a slow render frame.
        let mut left = dt.min(0.2);
        while left > 0.00001 && self.state == DescentState::Playing {
            let t = left.min(1. / 120.);
            self.step(t, input);
            left -= t;
        }
        if matches!(self.state, DescentState::Over | DescentState::Cleared) {
            self.remember();
        }
    }
}

#[derive(Resource)]
pub(super) struct DescentArt {
    background: Handle<Image>,
    sheet: Handle<Image>,
}
pub(super) fn setup_descent(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(DescentArt {
        background: assets.load(image_asset_path("descent/mine-background.png")),
        sheet: assets.load(image_asset_path("descent/mine-atlas.png")),
    });
}

pub(super) fn descent_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::Descent || s.library_open || s.help_open {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let mut left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let mut right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    for point in touches.iter().map(|t| t.position()).chain(
        mouse
            .pressed(MouseButton::Left)
            .then(|| window.cursor_position())
            .flatten(),
    ) {
        let p = point / Vec2::new(window.width(), window.height()) * Vec2::new(W, H);
        left |= p.distance(Vec2::new(140., 1406.)) < 100.;
        right |= p.distance(Vec2::new(884., 1406.)) < 100.;
    }
    let previous = s.descent.state;
    if std::env::var_os("MAGNET_CAPTURE").is_none()
        && !window.focused
        && s.descent.state == DescentState::Playing
    {
        s.descent.toggle();
    }
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        if matches!(s.descent.state, DescentState::Ready | DescentState::Over) {
            s.descent.start();
        } else if s.descent.state == DescentState::Cleared {
            s.descent.continue_endless();
        } else {
            s.descent.toggle();
        }
    }
    if keys.just_pressed(KeyCode::KeyP) || keys.just_pressed(KeyCode::Escape) {
        s.descent.toggle();
    }
    if keys.just_pressed(KeyCode::KeyR) {
        s.descent.start();
    }
    if cfg!(debug_assertions)
        && std::env::var_os("MAGNET_CAPTURE").is_some()
        && std::env::var_os("MAGNET_DESCENT_PREVIEW").is_some()
    {
        s.descent.state = DescentState::Playing;
        s.descent.floor = 27;
        s.descent.hearts = 4;
        s.ui_dirty = true;
        return;
    }
    s.descent.advance(
        time.delta_secs(),
        if left == right {
            0
        } else if left {
            -1
        } else {
            1
        },
    );
    s.ui_dirty |= previous != s.descent.state || s.descent.state == DescentState::Playing;
}

fn node(x: f32, y: f32, w: f32, h: f32, sx: f32, sy: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x * sx),
        top: Val::Px(y * sy),
        width: Val::Px(w * sx),
        height: Val::Px(h * sy),
        ..default()
    }
}
// Tight source rectangles are measured from the approved atlas, without altering it.
fn source(index: usize) -> Rect {
    let cells = [
        (30., 127., 307., 253.),
        (341., 127., 601., 243.),
        (636., 131., 925., 232.),
        (957., 90., 1218., 250.),
        (46., 354., 289., 594.),
        (343., 417., 607., 505.),
        (701., 401., 868., 546.),
        (1001., 401., 1164., 542.),
        (52., 679., 262., 879.),
        (361., 679., 584., 879.),
        (660., 682., 893., 883.),
        (985., 694., 1186., 892.),
        (38., 955., 290., 1202.),
        (352., 955., 603., 1202.),
        (697., 1006., 864., 1166.),
        (956., 1032., 1208., 1133.),
    ];
    let (x, y, right, bottom) = cells[index];
    Rect::new(x, y, right, bottom)
}
fn sprite(
    p: &mut ChildSpawnerCommands,
    a: &DescentArt,
    index: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    sx: f32,
    sy: f32,
    flip: bool,
) {
    p.spawn((
        node(x, y, w, h, sx, sy),
        ImageNode {
            image: a.sheet.clone(),
            rect: Some(source(index)),
            flip_x: flip,
            ..default()
        },
    ));
}
fn text_at(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    size: f32,
    color: Color,
    sx: f32,
    sy: f32,
) {
    p.spawn((
        node(x, y, w, size * 1.5, sx, sy),
        Text::new(text),
        TextFont {
            font: art.font.clone(),
            font_size: size * sx,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}
fn menu_button(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    a: &DescentArt,
    title: &str,
    action: Action,
    y: f32,
    sx: f32,
    sy: f32,
) {
    p.spawn((
        Button,
        action,
        node(238., y, 548., 104., sx, sy),
        BackgroundColor(Color::NONE),
    ))
    .with_children(|b| {
        sprite(b, a, 15, 0., 0., 548., 104., sx, sy, false);
        text_at(
            b,
            art,
            title,
            0.,
            17.,
            548.,
            42.,
            Color::srgb(0.16, 0.09, 0.02),
            sx,
            sy,
        );
    });
}
pub(super) fn descent_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    a: &DescentArt,
    width: f32,
    height: f32,
) {
    let g = &s.descent;
    let sx = width / W;
    let sy = height / H;
    commands
        .spawn((
            UiRoot,
            WorldInputBlocker,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.04, 0.07)),
        ))
        .with_children(|p| {
            p.spawn((
                node(0., 0., W, H, sx, sy),
                ImageNode::new(a.background.clone()),
            ));
            for ledge in &g.ledges {
                let y = ledge.y - g.camera;
                if ledge.gone || y < 0. || y > H + 80. {
                    continue;
                }
                let (index, h, offset) = match ledge.kind {
                    Tile::Stone => (0, 46., 0.),
                    Tile::Cracked => (1, 64., 0.),
                    Tile::Conveyor => (2, 55., 0.),
                    Tile::Spikes => (3, 78., -35.),
                    Tile::Spring => (4, 106., 0.),
                };
                let shake = if ledge.crumble.is_some() {
                    (g.elapsed * 80.).sin() * 3.
                } else {
                    0.
                };
                sprite(
                    p,
                    a,
                    index,
                    ledge.x + shake,
                    y + offset,
                    ledge.w,
                    h,
                    sx,
                    sy,
                    false,
                );
            }
            if g.immunity <= 0. || (g.elapsed * 14.) as u32 % 2 == 0 {
                let frame = if g.standing.is_none() {
                    11
                } else if g.moving {
                    9 + (g.elapsed * 8.) as usize % 2
                } else {
                    8
                };
                sprite(
                    p,
                    a,
                    frame,
                    g.x - 55.,
                    g.feet - g.camera - 78.,
                    94.,
                    78.,
                    sx,
                    sy,
                    g.direction < 0,
                );
            }
            p.spawn((
                node(22., 0., 980., 76., sx, sy),
                BackgroundColor(Color::srgb(0.025, 0.035, 0.055)),
            ));
            for i in 0..5 {
                sprite(
                    p,
                    a,
                    if i < g.hearts { 6 } else { 7 },
                    48. + i as f32 * 55.,
                    17.,
                    46.,
                    42.,
                    sx,
                    sy,
                    false,
                );
            }
            text_at(
                p,
                art,
                format!("第 {} 层", g.floor),
                397.,
                0.,
                230.,
                49.,
                Color::srgb(1., 0.81, 0.37),
                sx,
                sy,
            );
            p.spawn((
                Button,
                Action::DescentToggle,
                node(912., 10., 72., 58., sx, sy),
            ))
            .with_children(|b| sprite(b, a, 14, 0., 0., 72., 58., sx, sy, false));
            for i in 0..8 {
                sprite(
                    p,
                    a,
                    5,
                    22. + i as f32 * 122.5,
                    76.,
                    122.5,
                    36.,
                    sx,
                    sy,
                    false,
                );
            }
            sprite(p, a, 12, 42., 1308., 196., 196., sx, sy, false);
            sprite(p, a, 13, 786., 1308., 196., 196., sx, sy, false);
            if g.state != DescentState::Playing {
                p.spawn((
                    node(0., 112., W, H - 112., sx, sy),
                    BackgroundColor(Color::srgba(0.01, 0.02, 0.04, 0.77)),
                ))
                .with_children(|overlay| {
                    let (heading, primary, action) = match g.state {
                        DescentState::Ready => ("勇闯地下100层", "开始挑战", Action::DescentStart),
                        DescentState::Paused => ("暂停挑战", "继续挑战", Action::DescentToggle),
                        DescentState::Over => ("挑战结束", "再来一次", Action::DescentStart),
                        DescentState::Cleared => {
                            ("抵达100层！", "继续无尽挑战", Action::DescentEndless)
                        }
                        _ => unreachable!(),
                    };
                    text_at(
                        overlay,
                        art,
                        heading,
                        80.,
                        220.,
                        864.,
                        76.,
                        Color::srgb(0.98, 0.88, 0.67),
                        sx,
                        sy,
                    );
                    if matches!(g.state, DescentState::Over | DescentState::Cleared) {
                        text_at(
                            overlay,
                            art,
                            format!("{} 层", g.floor),
                            180.,
                            355.,
                            664.,
                            118.,
                            Color::srgb(1., 0.74, 0.22),
                            sx,
                            sy,
                        );
                    } else {
                        text_at(
                            overlay,
                            art,
                            "左右移动 · 持续向下",
                            150.,
                            365.,
                            724.,
                            31.,
                            Color::srgb(0.67, 0.76, 0.81),
                            sx,
                            sy,
                        );
                    }
                    text_at(
                        overlay,
                        art,
                        format!("最佳纪录 {} 层", g.best),
                        220.,
                        535.,
                        584.,
                        35.,
                        Color::srgb(0.87, 0.83, 0.73),
                        sx,
                        sy,
                    );
                    menu_button(overlay, art, a, primary, action, 720., sx, sy);
                    overlay
                        .spawn((
                            Button,
                            Action::Library,
                            node(308., 860., 408., 82., sx, sy),
                            BackgroundColor(Color::srgb(0.16, 0.19, 0.24)),
                            BorderColor(Color::srgb(0.47, 0.51, 0.56)),
                            BorderRadius::all(Val::Px(6.)),
                        ))
                        .with_children(|b| {
                            text_at(
                                b,
                                art,
                                "返回游戏库",
                                0.,
                                14.,
                                408.,
                                34.,
                                Color::WHITE,
                                sx,
                                sy,
                            )
                        });
                });
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn playing() -> DescentGame {
        let mut g = DescentGame::fresh(10000);
        g.state = DescentState::Playing;
        g
    }
    #[test]
    fn walk_off_and_land_without_tunneling() {
        let mut g = playing();
        g.ledges = vec![
            Ledge::new(250., 376., 220., Tile::Stone),
            Ledge::new(430., 600., 220., Tile::Stone),
        ];
        g.standing = Some(0);
        for _ in 0..22 {
            g.advance(1. / 60., 1);
        }
        for _ in 0..30 {
            g.advance(1. / 60., 0);
        }
        assert_eq!(g.standing, Some(1));
        assert_eq!(g.feet, 600.);
    }
    #[test]
    fn cracked_platform_expires_and_falls() {
        let mut g = playing();
        g.ledges[2].kind = Tile::Cracked;
        g.ledges[2].crumble = Some(0.1);
        g.advance(0.2, 0);
        assert!(g.ledges[2].gone);
        assert!(g.feet > 376.);
    }
    #[test]
    fn spikes_have_damage_cooldown() {
        let mut g = playing();
        g.ledges[2].kind = Tile::Spikes;
        g.advance(0.2, 0);
        assert_eq!(g.hearts, 4);
        g.advance(0.2, 0);
        assert_eq!(g.hearts, 4);
    }
    #[test]
    fn spring_bounces_on_landing() {
        let mut g = playing();
        g.ledges = vec![Ledge::new(250., 420., 220., Tile::Spring)];
        g.standing = None;
        g.feet = 400.;
        g.vy = 200.;
        g.advance(0.15, 0);
        assert!(g.vy < 0.);
        assert!(g.standing.is_none());
    }
    #[test]
    fn pause_freezes_world() {
        let mut g = playing();
        g.toggle();
        let feet = g.feet;
        g.advance(0.2, 1);
        assert_eq!(g.feet, feet);
        assert_eq!(g.camera, 0.);
        g.toggle();
        assert_eq!(g.state, DescentState::Playing);
    }
    #[test]
    fn ceiling_and_bottom_end_run() {
        let mut g = playing();
        g.camera = 230.;
        g.advance(0.01, 0);
        assert_eq!(g.state, DescentState::Over);
        let mut g = playing();
        g.standing = None;
        g.feet = H + 80.;
        g.advance(0.01, 0);
        assert_eq!(g.state, DescentState::Over);
    }
    #[test]
    fn generated_safe_route_stays_reachable() {
        let mut g = playing();
        g.camera = 15000.;
        g.generate();
        let safe: Vec<_> = g
            .ledges
            .iter()
            .filter(|p| p.y >= 1470. && p.kind != Tile::Spikes)
            .collect();
        for pair in safe.windows(2) {
            assert!((pair[0].x - pair[1].x).abs() <= 281.);
            assert!(pair[1].y - pair[0].y <= 169.);
        }
        assert!(g.next_y > g.camera + H);
    }
    #[test]
    fn full_width_movement_and_restart() {
        let mut g = playing();
        for _ in 0..80 {
            g.advance(0.01, -1);
        }
        assert!(g.x < 100.);
        g.start();
        assert_eq!(g.hearts, 5);
        assert_eq!(g.floor, 1);
        assert_eq!(g.best, 10000);
    }
    #[test]
    fn floor_100_can_continue_into_endless() {
        let mut g = playing();
        g.feet = 376. + 99. * 140.;
        g.camera = g.feet - 500.;
        g.standing = None;
        g.vy = 0.;
        g.advance(0.01, 0);
        assert_eq!(g.floor, 100);
        assert_eq!(g.state, DescentState::Cleared);
        let camera = g.camera;
        g.advance(0.1, 1);
        assert_eq!(g.camera, camera);
        g.continue_endless();
        g.advance(0.1, 0);
        assert_eq!(g.state, DescentState::Playing);
        assert!(g.camera > camera);
    }
    #[test]
    fn conveyor_moves_idle_player() {
        let mut g = playing();
        g.ledges[2].kind = Tile::Conveyor;
        let x = g.x;
        g.advance(0.1, 0);
        assert!(g.x > x + 15.);
    }
}
