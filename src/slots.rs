//! Free, offline fruit reels. Points have no purchase or redemption path.
use super::*;

const SYMBOLS: usize = 5;
const STOP: [f32; 3] = [1.8, 2.3, 2.8];
const GOLD: Color = Color::srgb(1., 0.78, 0.27);
const PURPLE: Color = Color::srgb(0.05, 0.45, 0.49);
const DARK: Color = Color::srgb(0.03, 0.23, 0.27);

pub(super) struct SlotsGame {
    points: u32,
    bet: u32,
    pub(super) spinning: bool,
    clock: f32,
    age: f32,
    prize_age: f32,
    offsets: [f32; 3],
    starts: [f32; 3],
    travels: [f32; 3],
    result: [usize; 3],
    prize: u32,
    turns: u32,
    seed: u64,
}
impl SlotsGame {
    pub(super) fn new() -> Self {
        Self {
            points: 1000,
            bet: 10,
            spinning: false,
            clock: 0.,
            age: 0.,
            prize_age: 10.,
            offsets: [0., 1., 2.],
            starts: [0.; 3],
            travels: [0.; 3],
            result: [0, 1, 2],
            prize: 0,
            turns: 0,
            seed: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(519)
                | 1,
        }
    }
    fn random(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    fn multiplier(result: [usize; 3]) -> u32 {
        if result[0] == result[1] && result[1] == result[2] {
            [4, 5, 6, 8, 12][result[0]]
        } else if result[0] == result[1] || result[0] == result[2] || result[1] == result[2] {
            1
        } else {
            0
        }
    }
    pub(super) fn set_bet(&mut self, bet: u32) {
        if !self.spinning && [10, 20, 50].contains(&bet) {
            self.bet = bet;
        }
    }
    pub(super) fn refill(&mut self) {
        if !self.spinning {
            self.points = self.points.saturating_add(1000);
        }
    }
    pub(super) fn spin(&mut self) {
        if self.spinning || self.points < self.bet {
            return;
        }
        let results = std::array::from_fn(|_| (self.random() % SYMBOLS as u64) as usize);
        self.begin(results);
    }
    fn begin(&mut self, result: [usize; 3]) {
        self.points -= self.bet;
        self.result = result;
        self.clock = 0.;
        self.spinning = true;
        self.prize = 0;
        self.prize_age = 10.;
        for i in 0..3 {
            self.starts[i] = self.offsets[i].rem_euclid(SYMBOLS as f32);
            self.travels[i] = ((result[i] as f32 - self.starts[i]).rem_euclid(SYMBOLS as f32))
                + (4 + i) as f32 * SYMBOLS as f32;
        }
    }
    pub(super) fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0., 0.1);
        self.age += dt;
        self.prize_age += dt;
        if !self.spinning {
            return false;
        }
        self.clock += dt;
        for i in 0..3 {
            let t = (self.clock / STOP[i]).clamp(0., 1.);
            self.offsets[i] = self.starts[i] + self.travels[i] * (1. - (1. - t).powi(3));
        }
        if self.clock >= STOP[2] {
            self.spinning = false;
            self.turns += 1;
            self.offsets = self.result.map(|n| n as f32);
            self.prize = Self::multiplier(self.result) * self.bet;
            self.points = self.points.saturating_add(self.prize);
            self.prize_age = 0.;
            return true;
        }
        false
    }
    pub(super) fn preview(&mut self, scene: &str) {
        match scene {
            "win" => self.begin([4, 4, 4]),
            "pair" => self.begin([0, 0, 1]),
            "spin" => self.begin([0, 2, 3]),
            _ => {}
        }
    }
}
#[derive(Component)]
pub(super) enum SlotsMotion {
    Fruit(usize, usize),
    Lamp(usize),
    Coin(usize),
    Lever,
    WinGlow,
}

fn rect(p: &mut ChildSpawnerCommands, x: f32, y: f32, w: f32, h: f32, c: Color, r: f32) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(w),
            height: Val::Px(h),
            ..default()
        },
        BackgroundColor(c),
        BorderRadius::all(Val::Px(r)),
    ));
}
fn fruit(p: &mut ChildSpawnerCommands, symbol: usize) {
    match symbol {
        0 => {
            rect(p, 32., 10., 5., 35., TEAL, 3.);
            rect(p, 55., 13., 5., 30., TEAL, 3.);
            rect(p, 32., 8., 28., 7., TEAL, 3.);
            rect(p, 12., 35., 42., 42., Color::srgb(0.91, 0.22, 0.32), 25.);
            rect(p, 45., 39., 40., 40., Color::srgb(0.98, 0.34, 0.38), 25.);
            rect(p, 22., 41., 9., 13., Color::srgb(1., 0.68, 0.66), 8.);
            rect(p, 54., 46., 8., 10., Color::srgb(1., 0.73, 0.7), 8.);
        }
        1 => {
            rect(p, 8., 29., 78., 47., GOLD, 28.);
            rect(p, 3., 47., 18., 15., GOLD, 8.);
            rect(p, 77., 42., 16., 15., GOLD, 8.);
            rect(p, 24., 38., 36., 8., Color::srgb(1., 0.94, 0.61), 5.);
            rect(p, 51., 15., 29., 14., MINT, 10.);
        }
        2 => {
            rect(p, 43., 4., 5., 20., TEAL, 3.);
            rect(p, 46., 6., 28., 13., MINT, 10.);
            for (x, y) in [
                (20., 25.),
                (45., 25.),
                (12., 44.),
                (36., 44.),
                (60., 44.),
                (25., 63.),
                (48., 63.),
                (37., 79.),
            ] {
                rect(p, x, y, 23., 23., Color::srgb(0.57, 0.36, 0.81), 14.);
                rect(p, x + 4., y + 3., 7., 6., Color::srgb(0.79, 0.64, 0.94), 5.);
            }
        }
        3 => {
            rect(p, 16., 21., 65., 65., Color::srgb(1., 0.55, 0.17), 36.);
            rect(p, 49., 7., 29., 15., MINT, 10.);
            rect(p, 43., 13., 5., 14., TEAL, 3.);
            rect(p, 26., 33., 9., 18., Color::srgb(1., 0.78, 0.4), 7.);
        }
        _ => {
            rect(p, 13., 10., 73., 78., GOLD, 18.);
            rect(p, 27., 25., 43., 12., DARK, 3.);
            rect(p, 57., 35., 12., 13., DARK, 2.);
            rect(p, 48., 46., 12., 15., DARK, 2.);
            rect(p, 39., 59., 12., 16., DARK, 2.);
            rect(p, 20., 17., 35., 4., PAPER, 3.);
        }
    }
}

pub(super) fn slots_ui(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let g = &s.slots;
    let cabinet_w = (width - 360.).min(720.);
    let side_x = cabinet_w + 52.;
    let side_w = width - side_x - 24.;
    let reel_w = (cabinet_w - 76.) / 3.;
    let reel_y = 126.;
    let reel_h = 200.;
    commands.spawn((UiRoot,Node {width:Val::Percent(100.),height:Val::Percent(100.),..default()},BackgroundColor(Color::srgb(0.96,0.91,0.77))))
    .with_children(|root|{
        root.spawn(Node {position_type:PositionType::Absolute,left:Val::Px(24.),right:Val::Px(24.),top:Val::Px(15.),justify_content:JustifyContent::SpaceBetween,align_items:AlignItems::Center,..row()})
        .with_children(|p|{
            p.spawn(Node {flex_direction:FlexDirection::Column,..default()}).with_children(|p|{label(p,art,"水果转转乐",30.,DARK);label(p,art,"免费虚拟积分 · 看中间一行的图案",13.,PURPLE);});
            p.spawn(row()).with_children(|p|{button(p,art,"怎么玩",Action::Help,86.,44.,PURPLE,PAPER);button(p,art,"游戏库",Action::Library,86.,44.,PURPLE,PAPER);});
        });
        root.spawn((Node {position_type:PositionType::Absolute,left:Val::Px(24.),top:Val::Px(92.),width:Val::Px(cabinet_w),height:Val::Px(height-115.),border:UiRect::all(Val::Px(5.)),..default()},BackgroundColor(PURPLE),BorderColor(GOLD),BorderRadius::all(Val::Px(28.))))
        .with_children(|cab|{
            rect(cab,10.,10.,cabinet_w-20.,height-139.,Color::srgb(0.09,0.56,0.58),22.);
            cab.spawn(Node {position_type:PositionType::Absolute,left:Val::Px(0.),right:Val::Px(0.),top:Val::Px(22.),justify_content:JustifyContent::Center,..row()})
        .with_children(|p|label(p,art,"水果转转乐",30.,Color::srgb(1.,0.94,0.71)));
            for i in 0..3 {
                cab.spawn((SlotsMotion::Lamp(i),Node {position_type:PositionType::Absolute,left:Val::Px(32.+i as f32*(reel_w+6.)+reel_w/2.-7.),top:Val::Px(70.),width:Val::Px(14.),height:Val::Px(14.),..default()},BackgroundColor(GOLD),BorderRadius::all(Val::Px(10.))));
                cab.spawn((Node {position_type:PositionType::Absolute,left:Val::Px(32.+i as f32*(reel_w+6.)),top:Val::Px(reel_y-30.),width:Val::Px(reel_w),height:Val::Px(reel_h),overflow:Overflow::clip(),border:UiRect::all(Val::Px(4.)),..default()},BackgroundColor(Color::srgb(1.,0.96,0.84)),BorderColor(GOLD),BorderRadius::all(Val::Px(14.))))
                .with_children(|reel|{
                    for symbol in 0..SYMBOLS {
                        reel.spawn((SlotsMotion::Fruit(i,symbol),Node {position_type:PositionType::Absolute,left:Val::Px(reel_w/2.-48.),width:Val::Px(96.),height:Val::Px(104.),..default()}))
                        .with_children(|p|fruit(p,symbol));
                    }
                    rect(reel,0.,0.,reel_w,32.,Color::srgba(0.05,0.36,0.37,0.13),0.);
                    rect(reel,0.,168.,reel_w,32.,Color::srgba(0.05,0.36,0.37,0.13),0.);
                });
            }
            cab.spawn((SlotsMotion::WinGlow,Node {position_type:PositionType::Absolute,left:Val::Px(26.),top:Val::Px(reel_y+19.),width:Val::Px(cabinet_w-64.),height:Val::Px(104.),border:UiRect::all(Val::Px(3.)),..default()},BorderColor(GOLD),BorderRadius::all(Val::Px(10.)),BackgroundColor(Color::NONE)));
            cab.spawn(Node {position_type:PositionType::Absolute,left:Val::Px(20.),right:Val::Px(20.),top:Val::Px(312.),justify_content:JustifyContent::Center,..row()}).with_children(|p|{
                label(p,art,if g.spinning{"转轮依次停下…".into()}else if g.turns==0{"点下手柄，开始转动！".into()}else if g.prize>g.bet{format!("恭喜！获得 +{} 积分",g.prize)}else if g.prize>0{format!("两个相同！返还 {} 积分",g.prize)}else{"这次没有连上，下次再转！".into()},22.,GOLD);
            });
            for i in 0..24 {cab.spawn((SlotsMotion::Coin(i),Node {position_type:PositionType::Absolute,width:Val::Px(13.),height:Val::Px(13.),..default()},BackgroundColor(GOLD),BorderRadius::all(Val::Px(10.))));}
        });
        root.spawn(Node {position_type:PositionType::Absolute,left:Val::Px(side_x),top:Val::Px(92.),width:Val::Px(side_w),flex_direction:FlexDirection::Column,row_gap:Val::Px(12.),..default()})
        .with_children(|side|{
            side.spawn((Node {width:Val::Percent(100.),padding:UiRect::all(Val::Px(18.)),justify_content:JustifyContent::SpaceBetween,align_items:AlignItems::Center,..row()},BackgroundColor(Color::srgb(1.,0.96,0.85)),BorderRadius::all(Val::Px(18.))))
            .with_children(|p|{label(p,art,"我的积分",17.,DARK);label(p,art,format!("{}",g.points),30.,PURPLE);});
            label(side,art,"每次投入",15.,DARK);
            side.spawn(row()).with_children(|p|{for bet in [10,20,50] {button(p,art,format!("{}",bet),Action::SlotsBet(bet),(side_w-16.)/3.,44.,if g.bet==bet{Color::srgb(0.96,0.32,0.22)}else{Color::srgb(0.87,0.81,0.70)},if g.bet==bet{PAPER}else{DARK});}});
            button(side,art,if g.spinning{"转动中…"}else if g.points<g.bet{"积分不足，先免费补充"}else{"开始转动"},Action::SlotsSpin,side_w,66.,if g.spinning{PURPLE}else{Color::srgb(0.96,0.29,0.20)},PAPER);
            button(side,art,"免费补充 +1000",Action::SlotsRefill,side_w,44.,Color::srgb(0.52,0.78,0.93),DARK);
            label(side,art,"三枚相同：×4 / ×5 / ×6 / ×8 / ×12\n任意两枚相同：×1\n奖励为返还总额，包含本次投入",12.,DARK);
        });
        // A moving handle is also a large secondary spin control.
        rect(root,cabinet_w+18.,height-160.,8.,70.,SOFT,4.);
        root.spawn((Button,Action::SlotsSpin,ButtonTone(Color::srgb(0.98,0.35,0.43)),SlotsMotion::Lever,Node {position_type:PositionType::Absolute,left:Val::Px(cabinet_w),top:Val::Px(height-175.),width:Val::Px(44.),height:Val::Px(44.),..default()},BackgroundColor(Color::srgb(0.98,0.35,0.43)),BorderRadius::all(Val::Px(24.))));
    });
}

pub(super) fn animate_slots(
    s: Res<Session>,
    layout: Res<Layout>,
    scale: Res<UiScale>,
    mut nodes: Query<(&SlotsMotion, &mut Node, Option<&mut BackgroundColor>)>,
) {
    if s.mode != GameMode::Slots || s.library_open {
        return;
    }
    let g = &s.slots;
    let cabinet_w = (layout.width / scale.0 - 360.).min(720.);
    for (motion, mut node, color) in &mut nodes {
        match *motion {
            SlotsMotion::Fruit(reel, symbol) => {
                let mut relative = (symbol as f32 - g.offsets[reel]).rem_euclid(SYMBOLS as f32);
                if relative > 2.5 {
                    relative -= 5.;
                }
                node.top = Val::Px(48. + relative * 104.);
            }
            SlotsMotion::Lamp(i) => {
                if let Some(mut c) = color {
                    c.0 = if g.spinning && g.clock < STOP[i] {
                        if ((g.age * 9.) as usize + i) % 2 == 0 {
                            GOLD
                        } else {
                            PURPLE
                        }
                    } else {
                        MINT
                    };
                }
            }
            SlotsMotion::WinGlow => {
                if let Some(mut c) = color {
                    c.0 = GOLD.with_alpha(
                        if !g.spinning
                            && (g.prize > 0 && SlotsGame::multiplier(g.result) > 1)
                            && g.prize_age < 2.
                        {
                            (g.prize_age * 12.).sin().abs() * 0.12
                        } else {
                            0.
                        },
                    );
                }
            }
            SlotsMotion::Coin(i) => {
                let t = g.prize_age - i as f32 * 0.025;
                node.display = if (g.prize > 0 && SlotsGame::multiplier(g.result) > 1)
                    && !g.spinning
                    && (0.0..1.8).contains(&t)
                {
                    Display::Flex
                } else {
                    Display::None
                };
                let angle = i as f32 * 2.399;
                node.left = Val::Px(cabinet_w * 0.5 + angle.cos() * t * 160.);
                node.top = Val::Px(180. + angle.sin() * t * 100. + t * t * 90.);
                node.width = Val::Px(4. + 9. * (t * 18.).cos().abs());
                if let Some(mut c) = color {
                    c.0 = GOLD.with_alpha((1.8 - t).clamp(0., 1.));
                }
            }
            SlotsMotion::Lever => {
                node.top = Val::Px(
                    layout.height / scale.0 - 175.
                        + if g.spinning && g.clock < 0.65 {
                            (g.clock / 0.65 * std::f32::consts::PI).sin() * 40.
                        } else {
                            0.
                        },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rewards_match_every_symbol_and_pair_position() {
        for (symbol, m) in [4, 5, 6, 8, 12].into_iter().enumerate() {
            assert_eq!(SlotsGame::multiplier([symbol; 3]), m);
        }
        for result in [[0, 0, 1], [0, 1, 0], [1, 0, 0]] {
            assert_eq!(SlotsGame::multiplier(result), 1);
        }
        assert_eq!(SlotsGame::multiplier([0, 1, 2]), 0);
    }
    #[test]
    fn stake_is_charged_once_and_prize_settles_only_after_all_reels() {
        let mut g = SlotsGame::new();
        g.begin([4; 3]);
        g.spin();
        g.set_bet(50);
        g.refill();
        assert_eq!((g.points, g.bet), (990, 10));
        for _ in 0..25 {
            g.advance(0.1);
        }
        assert!(g.spinning);
        assert_eq!(g.points, 990);
        for _ in 0..5 {
            g.advance(0.1);
        }
        assert!(!g.spinning);
        assert_eq!(g.points, 1110);
        assert_eq!(g.offsets, [4.; 3]);
        for _ in 0..30 {
            g.advance(0.1);
        }
        assert_eq!(g.points, 1110);
    }
    #[test]
    fn reels_stop_in_order_and_insufficient_points_can_be_refilled() {
        let mut g = SlotsGame::new();
        g.points = 0;
        g.spin();
        assert!(!g.spinning);
        g.refill();
        g.begin([1, 3, 0]);
        for _ in 0..19 {
            g.advance(0.1);
        }
        assert!((g.offsets[0].rem_euclid(5.) - 1.).abs() < 0.001);
        assert!(g.clock < STOP[1]);
        for _ in 0..15 {
            g.advance(0.1);
        }
        assert_eq!(g.points, 990);
        assert_eq!(g.offsets, [1., 3., 0.]);
    }
}
