use super::*;

const RUN_SECONDS: f32 = 480.;
const GOAL_X: f32 = 2085.;
const WORLD_WIDTH: f32 = 2100.;
const CARROT_COUNT: usize = 126;
const CARROT_X: [f32; CARROT_COUNT] = carrot_positions();
const CARROT_Y: [f32; CARROT_COUNT] = carrot_heights();
const fn carrot_positions() -> [f32; CARROT_COUNT] {
    let mut result = [0.; CARROT_COUNT];
    let mut i = 0;
    while i < CARROT_COUNT {
        result[i] = 24. + i as f32 * 16.25;
        i += 1;
    }
    result
}
const fn carrot_heights() -> [f32; CARROT_COUNT] {
    let mut result = [0.; CARROT_COUNT];
    let mut i = 0;
    while i < CARROT_COUNT {
        result[i] = if i % 7 == 3 { 28. } else { 13. };
        i += 1;
    }
    result
}
const OBSTACLES: [(f32, RabbitObstacleKind); 9] = [
    (307., RabbitObstacleKind::Log),
    (392., RabbitObstacleKind::Log),
    (443., RabbitObstacleKind::Hedgehog),
    (783., RabbitObstacleKind::Log),
    (936., RabbitObstacleKind::Log),
    (984., RabbitObstacleKind::Hedgehog),
    (1245., RabbitObstacleKind::Log),
    (1341., RabbitObstacleKind::Log),
    (1545., RabbitObstacleKind::Hedgehog),
];
// The one river is part of the approved first panorama. All other hazards
// are sparse obstacles; the rejected stone-ring pit is absent.
const HAZARDS: [(f32, f32, f32); 10] = [
    (176., 201., 1.),
    (303.5, 310.5, 8.),
    (388.5, 395.5, 8.),
    (440., 446., 7.5),
    (779.5, 786.5, 8.),
    (932.5, 939.5, 8.),
    (981., 987., 7.5),
    (1241.5, 1248.5, 8.),
    (1337.5, 1344.5, 8.),
    (1542., 1548., 7.5),
];
// Sampled path heights from each approved panorama, in world/screen units.
const GROUND_PROFILE: &[(f32, f32)] = &[
    (0., 79.),
    (175., 80.),
    (201., 80.),
    (350., 82.),
    (550., 81.),
    (685., 80.),
    (700., 70.),
    (770., 66.),
    (840., 76.),
    (980., 78.),
    (1120., 78.),
    (1225., 65.),
    (1320., 69.),
    (1385., 65.),
    (1400., 75.),
    (1540., 81.),
    (1630., 81.),
    (1730., 68.),
    (1820., 67.),
    (1920., 58.),
    (2010., 71.),
    (2100., 76.),
];
const ORCHARD_OBSTACLES: [(f32, RabbitObstacleKind); 11] = [
    (160., RabbitObstacleKind::Hay),
    (548., RabbitObstacleKind::Hay),
    (590., RabbitObstacleKind::Branch),
    (795., RabbitObstacleKind::Branch),
    (1027., RabbitObstacleKind::Hay),
    (1090., RabbitObstacleKind::Branch),
    (1240., RabbitObstacleKind::Branch),
    (1355., RabbitObstacleKind::Hay),
    (1465., RabbitObstacleKind::Hay),
    (1750., RabbitObstacleKind::Hay),
    (1865., RabbitObstacleKind::Branch),
];
const ORCHARD_HAZARDS: [(f32, f32, f32); 12] = [
    (156.5, 163.5, 7.),
    (544.5, 551.5, 7.),
    (586., 594., 6.5),
    (791., 799., 6.5),
    (878., 954., 1.),
    (1023.5, 1030.5, 7.),
    (1086., 1094., 6.5),
    (1236., 1244., 6.5),
    (1351.5, 1358.5, 7.),
    (1461.5, 1468.5, 7.),
    (1746.5, 1753.5, 7.),
    (1861., 1869., 6.5),
];
const ORCHARD_GROUND: &[(f32, f32)] = &[
    (0., 79.),
    (170., 82.),
    (290., 81.),
    (320., 78.),
    (345., 74.),
    (380., 76.),
    (415., 80.),
    (550., 82.),
    (700., 73.),
    (780., 80.),
    (860., 86.),
    (878., 84.),
    (954., 84.),
    (1040., 85.),
    (1170., 86.),
    (1280., 68.),
    (1360., 75.),
    (1400., 77.),
    (1450., 80.),
    (1540., 84.),
    (1650., 70.),
    (1750., 66.),
    (1815., 80.),
    (1880., 84.),
    (1970., 85.),
    (2040., 76.),
    (2100., 77.),
];
const MAX_HAZARDS: usize = 12;
fn interpolate_ground(profile: &[(f32, f32)], x: f32) -> f32 {
    for points in profile.windows(2) {
        if x <= points[1].0 {
            let u = ((x - points[0].0) / (points[1].0 - points[0].0)).clamp(0., 1.);
            return points[0].1 + (points[1].1 - points[0].1) * u;
        }
    }
    profile.last().unwrap().1
}

#[derive(Clone, Copy)]
enum RabbitObstacleKind {
    Log,
    Hedgehog,
    Hay,
    Branch,
}

pub(super) struct RabbitHomeGame {
    level: u8,
    position: f32,
    seconds_left: f32,
    lives: u8,
    collected: [bool; CARROT_COUNT],
    combo: u8,
    cleared: [bool; MAX_HAZARDS],
    jump_left: f32,
    jump_buffer: f32,
    jump_forward_speed: f32,
    invulnerable: f32,
    outcome: Option<bool>,
    walk_clock: f32,
}

#[derive(Component)]
pub(super) struct RabbitMovingSprite(pub usize);
#[derive(Component)]
pub(super) struct RabbitShadow(pub usize);
#[derive(Component)]
pub(super) struct RabbitCollectible(pub usize);
#[derive(Component)]
pub(super) struct RabbitProgressMarker;

#[derive(Component)]
pub(super) struct RabbitWorldItem(pub f32);
fn chapter_rect(index: usize) -> Rect {
    // The atlas stores the three approved route strips without their dividers.
    let (top, bottom) = match index {
        0 => (0., 251.),
        1 => (258., 473.),
        _ => (481., 724.),
    };
    Rect::from_corners(Vec2::new(0., top), Vec2::new(2172., bottom))
}

const AIR_TIME: f32 = 1.2;
const LOAD_TIME: f32 = 0.06;
const LAND_TIME: f32 = 0.12;
const BOXES: [[f32; 4]; 12] = [
    [8., 45., 166., 173.],
    [15., 40., 161., 173.],
    [8., 35., 168., 166.],
    [3., 32., 175., 158.],
    [8., 21., 181., 140.],
    [17., 20., 168., 153.],
    [9., 28., 172., 162.],
    [13., 38., 173., 165.],
    [13., 21., 170., 153.],
    [19., 0., 173., 153.],
    [9., 0., 158., 132.],
    [10., 22., 166., 155.],
];
fn rabbit_frame(frame: usize) -> Rect {
    let b = BOXES[frame];
    let origin = Vec2::new((frame % 4) as f32 * 362., (frame / 4) as f32 * 362.);
    Rect::from_corners(
        origin + Vec2::new(b[0], b[1]) * 2.,
        origin + Vec2::new(b[2], b[3]) * 2.,
    )
}
fn smooth(v: f32) -> f32 {
    let v = v.clamp(0., 1.);
    v * v * (3. - 2. * v)
}

fn prop_frame(index: usize) -> Rect {
    let (left, top, right, bottom) = match index {
        0 => (35., 125., 838., 495.),
        1 => (890., 120., 1450., 495.),
        _ => (240., 520., 675., 920.),
    };
    Rect::from_corners(Vec2::new(left, top), Vec2::new(right, bottom))
}

fn orchard_prop_frame(index: usize) -> Rect {
    let b = if index == 0 {
        (65., 135., 984., 625.)
    } else {
        (1145., 276., 2096., 619.)
    };
    Rect::from_corners(Vec2::new(b.0, b.1), Vec2::new(b.2, b.3))
}

impl RabbitHomeGame {
    pub(super) fn new() -> Self {
        Self {
            level: 1,
            position: 10.,
            seconds_left: RUN_SECONDS,
            lives: 3,
            collected: [false; CARROT_COUNT],
            combo: 0,
            cleared: [false; MAX_HAZARDS],
            jump_left: 0.,
            jump_buffer: 0.,
            jump_forward_speed: 18.,
            invulnerable: 0.,
            outcome: None,
            walk_clock: 0.,
        }
    }

    pub(super) fn move_player(&mut self, direction: i8, dt: f32) -> bool {
        if self.outcome.is_some() {
            return false;
        }
        let stage_bonus = ((self.level.saturating_sub(1) / 2) as f32 * 0.30).min(1.2);
        let ground_speed = match direction {
            -1 => 2.3,
            1 => 10.5,
            _ => 5.5,
        } + stage_bonus;
        // All platforms use the long course's river width and jump distance.
        // Touch users can clear it without holding a second acceleration button.
        let speed = if self.jump_left > 0. {
            self.jump_forward_speed + stage_bonus
        } else {
            ground_speed
        };
        let airborne = self.jump_left > LAND_TIME && self.jump_left <= AIR_TIME + LAND_TIME;
        let step = speed
            * dt.min(0.05)
            * if airborne {
                (2. - LOAD_TIME - LAND_TIME) / AIR_TIME
            } else {
                1.
            };
        let next = (self.position + step).min(GOAL_X);
        self.walk_clock += dt.min(0.05) * ground_speed.min(6.4) / 6.4;
        let mut changed = false;

        let jump_height = self.jump_height();
        for (index, &(left, right, clearance)) in self.hazards().iter().enumerate() {
            if (left..=right).contains(&next) && jump_height < clearance {
                if self.invulnerable > 0. {
                    if clearance <= 1. {
                        return false;
                    }
                    continue;
                }
                self.jump_left = 0.;
                self.jump_buffer = 0.;
                self.lives = self.lives.saturating_sub(1);
                self.invulnerable = 1.05;
                self.position = left - 3.;
                self.combo = 0;
                self.cleared[index] = true;
                if self.lives == 0 {
                    self.outcome = Some(false);
                }
                return true;
            }
            if !self.cleared[index] && self.position <= right && next > right {
                self.cleared[index] = true;
                // Crossing is safe once every sample inside the hazard passed.
                // The first sample beyond its edge may already be landing.
                self.combo = self.combo.saturating_add(1);
                changed = true;
            }
        }

        self.position = next;
        for index in 0..CARROT_COUNT {
            let carrot_x = self.carrot_x(index);
            let feet = self.ground_y(self.position) - jump_height;
            let within_height = self.ground_y(carrot_x) - self.carrot_height(index) >= feet - 24.
                && self.ground_y(carrot_x) - self.carrot_height(index) <= feet + 3.;
            if !self.collected[index] && (self.position - carrot_x).abs() < 2.4 && within_height {
                self.collected[index] = true;
                changed = true;
            }
        }
        if self.position >= GOAL_X {
            self.outcome = Some(true);
            changed = true;
        }
        changed
    }

    pub(super) fn jump(&mut self) {
        if self.outcome.is_some() {
            return;
        }
        if self.jump_left <= LAND_TIME {
            // A wide orchard creek uses its own takeoff impulse; the intact
            // bridge is solid ground and never appears in the hazard list.
            self.jump_forward_speed = self
                .hazards()
                .iter()
                .find(|(left, right, clearance)| {
                    self.map_index() == 1
                        && *clearance <= 1.
                        && *left >= self.position
                        && *left - self.position <= 14.
                        && *right > self.position
                })
                .map(|(left, right, _)| ((right - left + 16.) / 2.).max(18.))
                .unwrap_or(18.);
            self.jump_left = LOAD_TIME + AIR_TIME + LAND_TIME;
            self.jump_buffer = 0.;
        } else {
            // Remember a press just before touching down instead of losing it.
            self.jump_buffer = 0.25;
        }
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        if self.outcome.is_some() {
            return false;
        }
        let previous_second = self.seconds_left.ceil() as u32;
        self.seconds_left = (self.seconds_left - dt).max(0.);
        let was_landing = self.jump_left > 0.;
        self.jump_left = (self.jump_left - dt.min(0.05)).max(0.);
        if was_landing && self.jump_left <= LAND_TIME && self.jump_buffer > 0. {
            self.jump();
        } else if was_landing && self.jump_left == 0. {
            self.walk_clock = 0.;
        }
        self.jump_buffer = (self.jump_buffer - dt.min(0.05)).max(0.);
        self.invulnerable = (self.invulnerable - dt).max(0.);
        if self.seconds_left <= 0. {
            self.outcome = Some(false);
        }
        self.seconds_left.ceil() as u32 != previous_second || self.outcome.is_some()
    }

    fn jump_height(&self) -> f32 {
        if self.jump_left > LAND_TIME && self.jump_left <= AIR_TIME + LAND_TIME {
            let u = 1. - (self.jump_left - LAND_TIME) / AIR_TIME;
            4. * u * (1. - u) * 18.
        } else {
            0.
        }
    }

    // Frame pair, crossfade, lift, horizontal/vertical squash and tilt.
    fn pose(&self) -> (usize, usize, f32, f32, f32, f32, f32) {
        let t = self.jump_left;
        if t > AIR_TIME + LAND_TIME {
            let u = 1. - (t - AIR_TIME - LAND_TIME) / LOAD_TIME;
            return (
                8,
                8,
                0.,
                0.,
                1. + 0.05 * smooth(u),
                1. - 0.07 * smooth(u),
                0.,
            );
        }
        if t > LAND_TIME {
            let u = 1. - (t - LAND_TIME) / AIR_TIME;
            let keys = [(0., 9), (0.3, 10), (0.65, 10), (0.86, 6), (1., 11)];
            let mut k = 0;
            while k < 3 && u > keys[k + 1].0 {
                k += 1;
            }
            let e = (-u * 13.).exp() * 0.045;
            return (
                keys[k].1,
                keys[k + 1].1,
                smooth((u - keys[k + 1].0 + 0.065) / 0.065),
                self.jump_height(),
                1. - e * 0.6,
                1. + e,
                -0.035 * (1. - 2. * u),
            );
        }
        if t > 0. {
            let u = 1. - t / LAND_TIME;
            let squash = (std::f32::consts::PI * u).sin() * (-u).exp() * 0.12;
            return (
                11,
                0,
                smooth((u - 0.55) / 0.45),
                0.,
                1. + squash,
                1. - squash,
                0.012 * (u * std::f32::consts::TAU).sin(),
            );
        }
        let phase = (self.walk_clock / 0.44).fract();
        let step = phase * 8.;
        let wave = (phase * std::f32::consts::TAU).sin();
        let lift = 1.6
            * (std::f32::consts::PI * ((phase - 0.2) / 0.6).clamp(0., 1.))
                .sin()
                .max(0.);
        (
            step as usize,
            (step as usize + 1) % 8,
            smooth((step.fract() - 0.65) / 0.35),
            lift,
            1. + 0.018 * wave,
            1. - 0.018 * wave,
            0.018 * wave,
        )
    }

    fn sprite_node(&self, frame: usize) -> Node {
        let (_, _, _, lift, sx, sy, _) = self.pose();
        let b = BOXES[frame];
        // Same cell scale and foot/nose anchor as the approved motion preview.
        let unit_x = 12. / 181.;
        let unit_y = 23. / 181.;
        let w = (b[2] - b[0]) * unit_x * sx;
        let h = (b[3] - b[1]) * unit_y * sy;
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(self.screen_x() + (b[0] - b[2] + 0.24 * 181.) * unit_x * sx),
            top: Val::Percent(self.ground_y(self.position) - lift - h),
            width: Val::Percent(w),
            height: Val::Percent(h),
            ..default()
        }
    }

    pub(super) fn map_index(&self) -> usize {
        ((self.level.saturating_sub(1)) % 2) as usize
    }
    pub(super) fn map_name(&self) -> &'static str {
        if self.map_index() == 0 {
            "森林归途"
        } else {
            "溪谷果园"
        }
    }
    fn hazards(&self) -> &'static [(f32, f32, f32)] {
        if self.map_index() == 0 {
            &HAZARDS
        } else {
            &ORCHARD_HAZARDS
        }
    }
    fn obstacles(&self) -> &'static [(f32, RabbitObstacleKind)] {
        if self.map_index() == 0 {
            &OBSTACLES
        } else {
            &ORCHARD_OBSTACLES
        }
    }
    fn ground_y(&self, x: f32) -> f32 {
        interpolate_ground(
            if self.map_index() == 0 {
                GROUND_PROFILE
            } else {
                ORCHARD_GROUND
            },
            x,
        )
    }
    fn carrot_x(&self, index: usize) -> f32 {
        if self.map_index() == 0 {
            CARROT_X[index]
        } else {
            24. + index as f32 * 16. + if index % 8 == 2 { 2. } else { 0. }
        }
    }
    fn carrot_height(&self, index: usize) -> f32 {
        if self.map_index() == 0 {
            CARROT_Y[index]
        } else {
            [13., 15., 21., 13., 28., 15., 13.][index % 7]
        }
    }

    fn camera_x(&self) -> f32 {
        (self.position - 32.).clamp(0., WORLD_WIDTH - 100.)
    }
    fn screen_x(&self) -> f32 {
        self.position - self.camera_x()
    }
    pub(super) fn qa_camera(&self) -> f32 {
        self.camera_x()
    }
    pub(super) fn qa_position(&self) -> f32 {
        self.position
    }
    pub(super) fn qa_should_jump(&self) -> bool {
        self.jump_left <= LAND_TIME
            && self
                .qa_next_hazard()
                .is_some_and(|left| left - self.position <= 7. && left > self.position)
    }
    pub(super) fn qa_next_hazard(&self) -> Option<f32> {
        self.hazards()
            .iter()
            .find(|(_, right, _)| *right > self.position)
            .map(|(left, _, _)| *left)
    }

    pub(super) fn progress(&self) -> f32 {
        (self.position / GOAL_X).clamp(0., 1.)
    }

    pub(super) fn collected_count(&self) -> usize {
        self.collected
            .iter()
            .filter(|collected| **collected)
            .count()
    }

    pub(super) fn carrot_collected(&self, index: usize) -> bool {
        self.collected[index]
    }

    pub(super) fn lives(&self) -> u8 {
        self.lives
    }

    pub(super) fn combo(&self) -> u8 {
        self.combo
    }

    pub(super) fn seconds_left(&self) -> u32 {
        self.seconds_left.ceil() as u32
    }

    pub(super) fn outcome(&self) -> Option<bool> {
        self.outcome
    }

    pub(super) fn level(&self) -> u8 {
        self.level
    }

    pub(super) fn next(&mut self) {
        if self.outcome.is_none() {
            return;
        }
        if self.outcome == Some(true) {
            self.level = self.level.saturating_add(1).min(10);
        }
        self.position = 10.;
        self.seconds_left = RUN_SECONDS;
        self.lives = 3;
        self.collected = [false; CARROT_COUNT];
        self.combo = 0;
        self.cleared = [false; MAX_HAZARDS];
        self.jump_left = 0.;
        self.jump_buffer = 0.;
        self.jump_forward_speed = 18.;
        self.invulnerable = 0.;
        self.outcome = None;
        self.walk_clock = 0.;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::{IntoSystem, System};
    #[test]
    fn full_course_is_playable_at_every_speed_level() {
        for direction in [0, 1] {
            for level in 1..=10 {
                let mut game = RabbitHomeGame::new();
                game.level = level;
                let mut elapsed = 0.;
                while elapsed < 480. && game.outcome.is_none() {
                    if game.jump_left <= LAND_TIME
                        && game
                            .qa_next_hazard()
                            .is_some_and(|left| left - game.position <= 7. && left > game.position)
                    {
                        game.jump();
                    }
                    let before = (game.position, game.jump_left);
                    game.move_player(direction, 1. / 60.);
                    game.advance(1. / 60.);
                    elapsed += 1. / 60.;
                    assert_eq!(
                        game.lives, 3,
                        "level={level} direction={direction} before={before:?}"
                    );
                }
                assert_eq!(
                    game.outcome,
                    Some(true),
                    "level={level} direction={direction}"
                );
                assert_eq!(game.combo, game.hazards().len() as u8);
                eprintln!("LONG_COURSE level={level} direction={direction} seconds={elapsed:.1}");
                assert!(game.collected_count() > 90);
                assert!((140. ..380.).contains(&elapsed), "duration={elapsed}");
                assert_eq!(game.camera_x(), 2000.);
                game.next();
                assert_eq!(game.level, (level + 1).min(10));
                assert_eq!(game.collected_count(), 0);
            }
        }
    }
    #[test]
    fn maps_change_on_winning_and_failure_retries_current_map() {
        let mut game = RabbitHomeGame::new();
        assert_eq!(game.map_name(), "森林归途");
        game.outcome = Some(true);
        game.next();
        assert_eq!(game.level, 2);
        assert_eq!(game.map_name(), "溪谷果园");
        assert_eq!(game.hazards().len(), 12);
        assert_eq!(game.obstacles().len(), 11);
        assert_ne!(game.carrot_x(5), CARROT_X[5]);
        game.outcome = Some(false);
        game.position = 1000.;
        game.next();
        assert_eq!(game.level, 2);
        assert_eq!(game.map_index(), 1);
        assert_eq!(game.position, 10.);
        game.outcome = Some(true);
        game.next();
        assert_eq!(game.level, 3);
        assert_eq!(game.map_index(), 0);
        game.level = 10;
        game.outcome = Some(true);
        game.next();
        assert_eq!(game.level, 10);
        assert_eq!(game.map_index(), 1);
    }
    #[test]
    fn orchard_bridge_is_solid_and_creek_is_jumpable() {
        let mut game = RabbitHomeGame::new();
        game.level = 2;
        game.position = 315.;
        while game.position < 420. {
            game.move_player(1, 1. / 60.);
            game.advance(1. / 60.);
            assert_eq!(game.lives, 3);
        }
        assert_eq!(game.combo, 0);
        game.position = 871.;
        game.jump();
        assert!(game.jump_forward_speed > 40.);
        for _ in 0..120 {
            game.move_player(1, 1. / 60.);
            game.advance(1. / 60.);
            assert_eq!(game.lives, 3);
            if game.position > 955. {
                break;
            }
        }
        assert!(game.position > 955.);
    }

    #[test]
    fn camera_follows_only_after_start_and_stops_at_end() {
        let mut game = RabbitHomeGame::new();
        assert_eq!(game.camera_x(), 0.);
        for x in [100., 650., 710., 1390., 1450., 1900.] {
            game.position = x;
            assert!((game.screen_x() - 32.).abs() < 0.001);
        }
        game.position = GOAL_X;
        assert_eq!(game.camera_x(), 2000.);
        assert!(game.screen_x() > 80.);
        assert!(CARROT_X.last().unwrap() < &GOAL_X);
        assert!(HAZARDS.windows(2).all(|h| h[1].0 - h[0].1 > 35.));
    }
    #[test]
    fn early_second_press_and_collision_retry_work() {
        let mut game = RabbitHomeGame::new();
        game.jump_left = LAND_TIME + 0.15;
        game.jump();
        for _ in 0..12 {
            game.advance(1. / 60.);
        }
        assert!(game.jump_left > 1.);
        game.position = 175.95;
        game.jump_left = 0.01;
        game.move_player(1, 1. / 60.);
        assert_eq!(game.lives, 2);
        assert_eq!(game.jump_left, 0.);
        game.jump();
        assert!(game.jump_left > 1.);
    }
    #[test]
    fn river_takeoff_is_forgiving() {
        for launch in [168., 169., 170., 172.] {
            let mut game = RabbitHomeGame::new();
            game.position = launch;
            game.jump();
            for _ in 0..160 {
                game.move_player(1, 1. / 60.);
                game.advance(1. / 60.);
                assert_eq!(game.lives, 3, "launch={launch}");
                if game.position > 202. {
                    break;
                }
            }
            assert!(game.position > 202.);
        }
    }
    #[test]
    fn no_hazard_in_former_right_pit_or_chapter_transitions() {
        for start in [65., 685., 1385.] {
            let mut game = RabbitHomeGame::new();
            game.position = start;
            for _ in 0..180 {
                game.move_player(1, 1. / 60.);
                game.advance(1. / 60.);
            }
            assert_eq!(game.lives, 3);
        }
    }
    #[test]
    fn retry_dialog_uses_current_run_values_and_preserves_difficulty() {
        let mut game = RabbitHomeGame::new();
        game.level = 4;
        game.position = GOAL_X * 0.427;
        for i in 0..36 {
            game.collected[i] = true;
        }
        game.outcome = Some(false);
        assert_eq!(retry_stats(&game), ("42%".into(), "36".into()));
        game.next();
        assert_eq!(game.level, 4);
        assert_eq!(game.outcome, None);
        assert_eq!(game.lives, 3);
        assert_eq!(game.collected_count(), 0);
        assert_eq!(game.seconds_left(), 480);
        assert!(RETRY_BUTTON.3 < RETRY_LIBRARY.1);
    }

    #[test]
    fn timeout_and_restart_preserve_level() {
        let mut game = RabbitHomeGame::new();
        game.level = 3;
        game.advance(481.);
        assert_eq!(game.outcome, Some(false));
        game.next();
        assert_eq!(game.level, 3);
        assert_eq!(game.seconds_left(), 480);
    }
    #[test]
    fn acceleration_has_real_effect() {
        let mut normal = RabbitHomeGame::new();
        let mut fast = RabbitHomeGame::new();
        for _ in 0..60 {
            normal.move_player(0, 1. / 60.);
            fast.move_player(1, 1. / 60.);
        }
        assert!((normal.position - 15.5).abs() < 0.01);
        assert!((fast.position - 20.5).abs() < 0.01);
    }
    #[test]
    fn animation_queries_are_disjoint() {
        let mut system = IntoSystem::into_system(rabbit_home_animation);
        system.initialize(&mut World::new());
    }
}

pub(super) fn rabbit_home_animation(
    s: Res<Session>,
    mut rabbit: Query<
        (
            &RabbitMovingSprite,
            &mut Node,
            &mut ImageNode,
            &mut Transform,
        ),
        (
            With<RabbitMovingSprite>,
            Without<RabbitProgressMarker>,
            Without<RabbitShadow>,
            Without<RabbitWorldItem>,
        ),
    >,
    mut carrots: Query<(&RabbitCollectible, &mut Visibility)>,
    mut progress: Query<
        &mut Node,
        (
            With<RabbitProgressMarker>,
            Without<RabbitMovingSprite>,
            Without<RabbitShadow>,
            Without<RabbitWorldItem>,
        ),
    >,
    mut effects: Query<
        (&RabbitShadow, &mut Node, &mut BackgroundColor),
        (
            Without<RabbitMovingSprite>,
            Without<RabbitProgressMarker>,
            Without<RabbitWorldItem>,
        ),
    >,
    mut world_items: Query<
        (&RabbitWorldItem, &mut Node),
        (
            Without<RabbitMovingSprite>,
            Without<RabbitProgressMarker>,
            Without<RabbitShadow>,
        ),
    >,
) {
    if s.mode != GameMode::RabbitHome {
        return;
    }
    let game = &s.rabbit_home;
    for (item, mut node) in &mut world_items {
        node.left = Val::Percent(item.0 - game.camera_x());
    }
    let (a, b, mix, _, _, _, angle) = game.pose();
    for (layer, mut node, mut image, mut transform) in &mut rabbit {
        let frame = if layer.0 == 0 { a } else { b };
        *node = game.sprite_node(frame);
        image.rect = Some(rabbit_frame(frame));
        image.color = Color::srgba(1., 1., 1., if layer.0 == 0 { 1. - mix } else { mix });
        transform.rotation = Quat::from_rotation_z(angle);
    }
    for (effect, mut node, mut color) in &mut effects {
        let (n, c) = rabbit_effect(game, effect.0);
        *node = n;
        color.0 = c;
    }
    for (carrot, mut visibility) in &mut carrots {
        *visibility = if s.rabbit_home.carrot_collected(carrot.0) {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
    for mut node in &mut progress {
        node.width = Val::Percent((s.rabbit_home.progress() * 100.).max(1.));
    }
}

fn rabbit_effect(game: &RabbitHomeGame, index: usize) -> (Node, Color) {
    let lift = game.pose().3;
    let (x, y, w, h, alpha) = if index == 0 {
        (
            game.screen_x() + 0.2,
            game.ground_y(game.position) + 0.5,
            4.6 * (1. - lift / 41.6),
            0.8 * (1. - lift / 48.),
            0.15 * (1. - lift / 27.2),
        )
    } else {
        let u = 1. - game.jump_left / LAND_TIME;
        let active = game.jump_left > 0. && game.jump_left <= LAND_TIME && u < 0.7;
        let i = (index - 1) as f32;
        (
            game.screen_x() - 2. - i * 0.5 - u * 1.3,
            game.ground_y(game.position) - 0.5 - i * 0.48 - u * 1.44,
            0.3 + u * 0.5,
            0.64 + u * 0.96,
            if active { (1. - u / 0.7) * 0.13 } else { 0. },
        )
    };
    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(x - w / 2.),
            top: Val::Percent(y - h / 2.),
            width: Val::Percent(w),
            height: Val::Percent(h),
            ..default()
        },
        Color::srgba(0.18, 0.13, 0.08, alpha),
    )
}

fn reference_image(art: &Art, bounds: (f32, f32, f32, f32)) -> ImageNode {
    let mut image = ImageNode::new(art.rabbit_home_ui.clone());
    image.rect = Some(Rect::from_corners(
        Vec2::new(bounds.0, bounds.1),
        Vec2::new(bounds.2, bounds.3),
    ));
    image
}

fn reference_node(bounds: (f32, f32, f32, f32)) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(bounds.0 / 1586. * 100.),
        top: Val::Percent(bounds.1 / 992. * 100.),
        width: Val::Percent((bounds.2 - bounds.0) / 1586. * 100.),
        height: Val::Percent((bounds.3 - bounds.1) / 992. * 100.),
        ..default()
    }
}

// Pixel bounds map the extracted artwork back to the approved 1586×992 mockup.
const RETRY_BUTTON: (f32, f32, f32, f32) = (575., 674., 1014., 802.);
const RETRY_LIBRARY: (f32, f32, f32, f32) = (670., 809., 917., 864.);
fn retry_stats(game: &RabbitHomeGame) -> (String, String) {
    (
        format!("{:.0}%", (game.progress() * 100.).floor()),
        game.collected_count().to_string(),
    )
}
fn rabbit_retry_dialog(
    root: &mut ChildSpawnerCommands,
    game: &RabbitHomeGame,
    art: &Art,
    font_scale: f32,
) {
    root.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.07, 0.035, 0.64)),
        ZIndex(30),
    ))
    .with_children(|overlay| {
        let mut artwork = ImageNode::new(art.rabbit_retry_dialog.clone());
        // Crop the extracted panel's transparent padding; preserve its approved
        // on-screen size instead of stretching the extraction to the viewport.
        artwork.rect = Some(Rect::from_corners(
            Vec2::new(301., 62.),
            Vec2::new(1286., 966.),
        ));
        overlay.spawn((reference_node((374., 140., 1212., 900.)), artwork));
        let (progress, carrots) = retry_stats(game);
        for (bounds, text) in [
            ((711., 591., 796., 642.), progress),
            ((990., 591., 1051., 642.), carrots),
        ] {
            let mut node = reference_node(bounds);
            node.align_items = AlignItems::Center;
            node.justify_content = JustifyContent::Center;
            overlay.spawn(node).with_children(|value| {
                label(
                    value,
                    art,
                    text,
                    26. * font_scale,
                    Color::srgb(0.27, 0.13, 0.045),
                )
            });
        }
        for (bounds, action) in [
            (RETRY_BUTTON, Action::RabbitNext),
            (RETRY_LIBRARY, Action::Library),
        ] {
            overlay.spawn((
                Button,
                action,
                reference_node(bounds),
                BackgroundColor(Color::NONE),
            ));
        }
    });
}

pub(super) fn rabbit_home_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    width: f32,
    _height: f32,
) {
    let game = &s.rabbit_home;
    let font_scale = (width / 1280.).clamp(0.6, 1.6);
    let root = commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                overflow: Overflow::clip(),
                ..default()
            },
        ))
        .id();
    commands.entity(root).with_children(|root| {
        for chapter in 0..3 {
            let mut image = ImageNode::new(if game.map_index() == 0 {
                art.rabbit_home_scene.clone()
            } else {
                art.rabbit_orchard_scene.clone()
            });
            image.rect = Some(chapter_rect(chapter));
            root.spawn((
                RabbitWorldItem(chapter as f32 * 700.),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(chapter as f32 * 700. - game.camera_x()),
                    top: Val::Percent(0.),
                    width: Val::Percent(700.),
                    height: Val::Percent(100.),
                    ..default()
                },
                image,
            ));
        }
        // Interface artwork comes directly from the confirmed design. Only
        // counters, life states and the progress fill are rendered dynamically.
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ImageNode::new(art.rabbit_home_ui.clone()),
        ));
        for (bounds, left, counter_width, text, color) in [
            (
                (545., 22., 760., 100.),
                40.,
                51.,
                format!(
                    "{:02}:{:02}",
                    game.seconds_left() / 60,
                    game.seconds_left() % 60
                ),
                Color::srgb(0.25, 0.13, 0.06),
            ),
            (
                (779., 22., 1021., 100.),
                68.,
                24.,
                format!("{:02}", game.collected_count()),
                Color::srgb(0.25, 0.13, 0.06),
            ),
            (
                (1042., 21., 1275., 100.),
                58.,
                35.,
                format!("x{}", game.combo()),
                Color::srgb(0.13, 0.24, 0.57),
            ),
        ] {
            root.spawn(reference_node(bounds)).with_children(|panel| {
                panel
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(left),
                            top: Val::Percent(24.),
                            width: Val::Percent(counter_width),
                            height: Val::Percent(60.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        reference_image(art, (850., 34., 940., 42.)),
                    ))
                    .with_children(|value| label(value, art, text, 26. * font_scale, color));
            });
        }
        let hearts = (1295., 20., 1550., 101.);
        root.spawn(reference_node(hearts)).with_children(|panel| {
            for index in game.lives()..3 {
                panel.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(10. + index as f32 * 25.5),
                        top: Val::Percent(15.),
                        width: Val::Percent(25.),
                        height: Val::Percent(70.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1., 0.93, 0.76, 0.85)),
                    BorderRadius::all(Val::Px(15.)),
                ));
            }
        });
        let track = (648., 100., 1395., 150.);
        root.spawn(reference_node(track)).with_children(|panel| {
            panel
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(8.6),
                        top: Val::Percent(36.),
                        width: Val::Percent(80.6),
                        height: Val::Percent(40.),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.39, 0.22, 0.07)),
                    BorderRadius::all(Val::Px(8.)),
                ))
                .with_children(|track| {
                    track.spawn((
                        RabbitProgressMarker,
                        Node {
                            width: Val::Percent(game.progress() * 100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        reference_image(art, (715., 119., 920., 132.)),
                        BorderRadius::all(Val::Px(8.)),
                    ));
                });
        });

        for index in 0..CARROT_COUNT {
            let x = game.carrot_x(index);
            let mut image = ImageNode::new(art.rabbit_home_props.clone());
            image.rect = Some(prop_frame(2));
            root.spawn((
                RabbitCollectible(index),
                RabbitWorldItem(x - 2.25),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(x - 2.25 - game.camera_x()),
                    top: Val::Percent(game.ground_y(x) - game.carrot_height(index) - 4.25),
                    width: Val::Percent(4.5),
                    height: Val::Percent(8.5),
                    ..default()
                },
                image,
            ));
        }
        for &(x, kind) in game.obstacles() {
            let (frame, width, height) = match kind {
                RabbitObstacleKind::Log => (0, 15., 11.5),
                RabbitObstacleKind::Hedgehog => (1, 9., 10.5),
                RabbitObstacleKind::Hay => (0, 13.5, 10.5),
                RabbitObstacleKind::Branch => (1, 16., 9.5),
            };
            let orchard_prop = matches!(kind, RabbitObstacleKind::Hay | RabbitObstacleKind::Branch);
            let mut image = ImageNode::new(if orchard_prop {
                art.rabbit_orchard_props.clone()
            } else {
                art.rabbit_home_props.clone()
            });
            image.rect = Some(if orchard_prop {
                orchard_prop_frame(frame)
            } else {
                prop_frame(frame)
            });
            root.spawn((
                RabbitWorldItem(x - width / 2.),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(x - width / 2. - game.camera_x()),
                    top: Val::Percent(game.ground_y(x) - height),
                    width: Val::Percent(width),
                    height: Val::Percent(height),
                    ..default()
                },
                image,
            ));
        }
        for index in 0..4 {
            let (node, color) = rabbit_effect(game, index);
            root.spawn((
                RabbitShadow(index),
                node,
                BackgroundColor(color),
                BorderRadius::MAX,
            ));
        }
        let (a, b, mix, _, _, _, angle) = game.pose();
        for (layer, frame, alpha) in [(0, a, 1. - mix), (1, b, mix)] {
            let mut image = ImageNode::new(art.rabbit_home_sprite.clone());
            image.rect = Some(rabbit_frame(frame));
            image.color = Color::srgba(1., 1., 1., alpha);
            root.spawn((
                RabbitMovingSprite(layer),
                game.sprite_node(frame),
                image,
                Transform::from_rotation(Quat::from_rotation_z(angle)),
            ));
        }

        // Physical touch hit areas survive UI refreshes and support simultaneous
        // speed control with one finger and jumping with the other.
        for (bounds, action) in [
            ((76., 776., 189., 898.), Action::RabbitMove(-1)),
            ((237., 776., 357., 898.), Action::RabbitMove(1)),
            ((1304., 699., 1549., 941.), Action::RabbitJump),
            ((36., 14., 492., 119.), Action::Library),
        ] {
            root.spawn((
                Button,
                action,
                reference_node(bounds),
                BackgroundColor(Color::NONE),
            ));
        }
        if game.outcome() == Some(false) {
            rabbit_retry_dialog(root, game, art, font_scale);
        } else if let Some(won) = game.outcome() {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(32.),
                    top: Val::Percent(27.),
                    width: Val::Percent(36.),
                    height: Val::Percent(39.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(13.),
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.98, 0.93, 0.78, 0.97)),
                BorderRadius::all(Val::Px(26.)),
            ))
            .with_children(|panel| {
                label(
                    panel,
                    art,
                    if won {
                        "冲到家啦！"
                    } else {
                        "再试一次吧！"
                    },
                    28. * font_scale,
                    Color::srgb(0.25, 0.43, 0.23),
                );
                label(
                    panel,
                    art,
                    format!(
                        "第 {} 关 · {} · 胡萝卜 {} · 连跳 x{}",
                        game.level(),
                        game.map_name(),
                        game.collected_count(),
                        game.combo()
                    ),
                    16. * font_scale,
                    Color::srgb(0.34, 0.25, 0.15),
                );
                panel
                    .spawn((
                        Button,
                        Action::RabbitNext,
                        Node {
                            width: Val::Px(210. * font_scale),
                            height: Val::Px(56. * font_scale),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.38, 0.62, 0.23)),
                        BorderRadius::all(Val::Px(16.)),
                    ))
                    .with_children(|button| {
                        label(
                            button,
                            art,
                            if won { "下一关 →" } else { "重新冲刺" },
                            20. * font_scale,
                            Color::WHITE,
                        )
                    });
            });
        }
    });
}
