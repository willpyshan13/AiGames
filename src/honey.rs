//! A warm, toy-like take on the classic hook-and-grab game.
//! The scene is all retained-mode UI so it scales cleanly on phones.
use super::*;

#[derive(Clone)]
struct Treasure {
    x: f32,
    y: f32,
    kind: u8,
}

pub(super) struct HoneyGame {
    treasures: Vec<Treasure>,
    pub(super) score: u32,
    pub(super) jars: u32,
    swing_phase: f32,
    angle: f32,
    hook: f32,
    flying: bool,
    pulling: bool,
    caught: Option<Treasure>,
    sparkle: f32,
    redraw: f32,
    feedback: String,
}

impl HoneyGame {
    pub(super) fn new() -> Self {
        Self {
            treasures: vec![
                Treasure {
                    x: 0.39,
                    y: 0.60,
                    kind: 0,
                },
                Treasure {
                    x: 0.56,
                    y: 0.82,
                    kind: 1,
                },
                Treasure {
                    x: 0.93,
                    y: 0.64,
                    kind: 2,
                },
            ],
            score: 0,
            jars: 0,
            swing_phase: 0.,
            angle: 0.,
            hook: 0.12,
            flying: false,
            pulling: false,
            caught: None,
            sparkle: 0.,
            redraw: 0.,
            feedback: "等抓钩对准蜂蜜，轻点画面发射！".into(),
        }
    }
    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    fn hook_pos(&self) -> Vec2 {
        // The pivot sits under the bear's little crane, near the centre top.
        Vec2::new(
            0.56 + self.angle.sin() * self.hook,
            0.12 + self.angle.cos() * self.hook,
        )
    }
    pub(super) fn launch(&mut self) {
        if self.flying || self.pulling {
            return;
        }
        self.flying = true;
        self.feedback = "抓钩出发！".into();
    }
    fn value(kind: u8) -> u32 {
        [20, 45, 90][kind as usize]
    }
    fn name(kind: u8) -> &'static str {
        ["蜂蜜罐", "金蜂蜜", "彩虹蜜"][kind as usize]
    }
    pub(super) fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0., 0.05);
        self.sparkle = (self.sparkle - dt).max(0.);
        if !self.flying && !self.pulling {
            self.swing_phase += dt * 1.65;
            self.angle = self.swing_phase.sin() * 0.90;
        }
        if self.flying {
            self.hook += dt * 0.68;
            let p = self.hook_pos();
            if let Some(index) = self
                .treasures
                .iter()
                .position(|t| p.distance(Vec2::new(t.x, t.y)) < 0.075)
            {
                let t = self.treasures.remove(index);
                let kind = t.kind;
                self.caught = Some(t);
                self.pulling = true;
                self.flying = false;
                self.feedback = format!("抓到{}啦，快拉回来！", Self::name(kind));
            } else if self.hook > 0.95 {
                self.pulling = true;
                self.flying = false;
                self.feedback = "差一点点，再瞄准蜂蜜试试！".into();
            }
        }
        if self.pulling {
            self.hook -= dt * 0.92;
            if self.hook <= 0.12 {
                self.hook = 0.12;
                self.pulling = false;
                if let Some(t) = self.caught.take() {
                    let points = Self::value(t.kind);
                    self.score += points;
                    self.jars += 1;
                    self.sparkle = 0.85;
                    self.feedback = format!("太棒了！{} +{} 蜂蜜分", Self::name(t.kind), points);
                }
            }
        }
        self.redraw += dt;
        if self.redraw >= 1. / 30. {
            self.redraw = 0.;
            true
        } else {
            self.flying || self.pulling || self.sparkle > 0.
        }
    }
}

pub(super) fn honey_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let game = &s.honey;
    let px = if compact { 14. } else { 32. };
    let title = if compact { 22. } else { 28. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.honey_story.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            for t in &game.treasures {
                let size = if compact { 65. } else { 88. };
                root.spawn((
                    ImageNode::new(art.honey_jar.clone()),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(t.x * 100.),
                        top: Val::Percent(t.y * 100.),
                        width: Val::Px(size),
                        height: Val::Px(size),
                        margin: UiRect::all(Val::Px(-size * 0.5)),
                        ..default()
                    },
                    ZIndex(2),
                ));
            }
            if game.caught.is_some() {
                let hook = game.hook_pos();
                let size = if compact { 52. } else { 68. };
                root.spawn((
                    ImageNode::new(art.honey_jar.clone()),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(hook.x * 100.),
                        top: Val::Percent(hook.y * 100.),
                        width: Val::Px(size),
                        height: Val::Px(size),
                        margin: UiRect::all(Val::Px(-size * 0.5)),
                        ..default()
                    },
                    ZIndex(5),
                ));
            }
            // Rope and hook are recomposed every animation frame. Their endpoint is also the collision point.
            let hook = game.hook_pos();
            let pivot = Vec2::new(0.56, 0.12);
            let delta = hook - pivot;
            let rope_w = (delta.length() * 100.).max(1.);
            // One continuous rope, laid out at its exact centre and rotated around it.
            // The line and claw share the same two endpoint coordinates.
            let rope_top = (pivot.y + hook.y) * 50. - rope_w * 0.5;
            let rope_left = (pivot.x + hook.x) * 50.;
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(rope_left),
                    top: Val::Percent(rope_top),
                    width: Val::Px(6.),
                    height: Val::Percent(rope_w),
                    margin: UiRect::all(Val::Px(-3.)),
                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_z(-game.angle)),
                BackgroundColor(Color::srgb(0.17, 0.10, 0.06)),
                BorderRadius::all(Val::Px(4.)),
                ZIndex(6),
            ));
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(rope_left),
                    top: Val::Percent(rope_top),
                    width: Val::Px(2.),
                    height: Val::Percent(rope_w),
                    margin: UiRect::all(Val::Px(-1.)),
                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_z(-game.angle)),
                BackgroundColor(Color::srgb(0.56, 0.34, 0.17)),
                BorderRadius::all(Val::Px(2.)),
                ZIndex(7),
            ));
            let claw_size = if compact { 58. } else { 76. };
            root.spawn((
                ImageNode::new(art.honey_claw.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(hook.x * 100.),
                    top: Val::Percent(hook.y * 100.),
                    width: Val::Px(claw_size),
                    height: Val::Px(claw_size),
                    margin: UiRect::all(Val::Px(-claw_size * 0.5)),
                    ..default()
                },
                ZIndex(7),
            ));
            if game.sparkle > 0. {
                for (x, y) in [(38., 34.), (62., 32.), (48., 44.), (56., 50.), (43., 52.)] {
                    root.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(x),
                            top: Val::Percent(y),
                            ..default()
                        },
                        ZIndex(20),
                    ))
                    .with_children(|p| {
                        label(
                            p,
                            art,
                            "✦",
                            if compact { 22. } else { 30. },
                            Color::srgba(1., 0.94, 0.35, game.sparkle / 0.85),
                        )
                    });
                }
            }
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(px),
                    right: Val::Px(px),
                    top: Val::Px(if compact { 12. } else { 18. }),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                ZIndex(30),
            ))
            .with_children(|top| {
                top.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::axes(Val::Px(14.), Val::Px(8.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.98, 0.88, 0.65, 0.92)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|p| {
                    label(p, art, "小熊寻蜜", title, Color::srgb(0.34, 0.22, 0.11));
                    label(
                        p,
                        art,
                        "摇摇抓钩 · 收集甜甜蜂蜜",
                        if compact { 12. } else { 15. },
                        Color::srgb(0.42, 0.31, 0.16),
                    );
                });
                top.spawn((
                    Node {
                        padding: UiRect::all(Val::Px(8.)),
                        column_gap: Val::Px(8.),
                        ..row()
                    },
                    BackgroundColor(Color::srgba(0.98, 0.88, 0.65, 0.92)),
                    BorderRadius::all(Val::Px(12.)),
                ))
                .with_children(|p| {
                    label(
                        p,
                        art,
                        format!("蜂蜜分 {}", game.score),
                        if compact { 14. } else { 17. },
                        Color::srgb(0.33, 0.23, 0.08),
                    );
                    button(
                        p,
                        art,
                        "?",
                        Action::Help,
                        38.,
                        36.,
                        Color::srgb(0.46, 0.29, 0.15),
                        Color::WHITE,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        if compact { 70. } else { 86. },
                        36.,
                        Color::srgb(0.46, 0.29, 0.15),
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.),
                    top: Val::Percent(if compact { 61. } else { 68. }),
                    max_width: Val::Percent(72.),
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ZIndex(30),
            ))
            .with_children(|p| {
                label(
                    p,
                    art,
                    &game.feedback,
                    if compact { 14. } else { 17. },
                    Color::srgb(1., 0.96, 0.76),
                )
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(px),
                    right: Val::Px(px),
                    bottom: Val::Px(14.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                ZIndex(30),
            ))
            .with_children(|bottom| {
                label(
                    bottom,
                    art,
                    format!("收集 {} 罐 · 金蜂蜜 +45 · 彩虹蜜 +90", game.jars),
                    if compact { 12. } else { 15. },
                    Color::srgb(0.90, 0.97, 0.82),
                );
                button(
                    bottom,
                    art,
                    "放下抓钩",
                    Action::HoneyLaunch,
                    if compact { 110. } else { 140. },
                    48.,
                    Color::srgb(0.54, 0.31, 0.13),
                    Color::WHITE,
                );
                button(
                    bottom,
                    art,
                    "重来",
                    Action::HoneyReset,
                    62.,
                    34.,
                    Color::srgb(0.23, 0.45, 0.34),
                    Color::WHITE,
                );
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn launch_moves_hook() {
        let mut game = HoneyGame::new();
        game.launch();
        for _ in 0..10 {
            game.advance(0.05);
        }
        assert!(game.hook > 0.05);
    }
    #[test]
    fn reset_restores_honey() {
        let mut game = HoneyGame::new();
        game.score = 99;
        game.reset();
        assert_eq!(game.score, 0);
        assert_eq!(game.treasures.len(), 3);
    }
}
