use super::*;

#[derive(Clone)]
struct Fish {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    kind: u8,
    phase: f32,
}

#[derive(Clone)]
struct NetBurst {
    x: f32,
    y: f32,
    life: f32,
    score: u32,
}

pub(super) struct FishingGame {
    fish: Vec<Fish>,
    nets: Vec<NetBurst>,
    pub(super) score: u32,
    pub(super) coins: u32,
    pub(super) catches: u32,
    spawn_clock: f32,
    redraw: f32,
    seed: u32,
    feedback: String,
}

impl FishingGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            fish: vec![],
            nets: vec![],
            score: 0,
            coins: 60,
            catches: 0,
            spawn_clock: 0.15,
            redraw: 0.,
            seed: 7,
            feedback: "点水面撒网，网住游过的鱼！".into(),
        };
        for _ in 0..6 {
            game.spawn_fish();
        }
        for (index, fish) in game.fish.iter_mut().enumerate() {
            fish.x = 0.18 + (index % 3) as f32 * 0.29;
            fish.y = 0.28 + (index / 3) as f32 * 0.31;
        }
        game
    }

    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    fn next(&mut self) -> u32 {
        self.seed = self
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        self.seed
    }

    fn spawn_fish(&mut self) {
        let seed = self.next();
        let kind = match seed % 16 {
            0 => 3, // Gold fish: rare and immediately recognizable.
            1..=3 => 2,
            4..=7 => 1,
            _ => 0,
        };
        let from_left = seed & 1 == 0;
        let speed = 0.075 + ((seed >> 8) % 7) as f32 * 0.010;
        self.fish.push(Fish {
            x: if from_left { -0.08 } else { 1.08 },
            y: 0.23 + ((seed >> 16) % 58) as f32 / 100.,
            vx: if from_left { speed } else { -speed },
            vy: (((seed >> 24) % 9) as f32 - 4.) * 0.004,
            kind,
            phase: (seed % 628) as f32 / 100.,
        });
    }

    fn reward(kind: u8) -> u32 {
        [8, 14, 24, 50][kind as usize]
    }

    pub(super) fn fire_at(&mut self, target: Vec2) {
        if self.coins < 2 {
            self.feedback = "金币不够啦，重新开始可补满渔网金币。".into();
            return;
        }
        // Keep the play area clear of the HUD and the instruction strip.
        if !(0.16..=0.91).contains(&target.y) {
            return;
        }
        self.coins -= 2;
        let mut caught = 0;
        let mut reward = 0;
        self.fish.retain(|fish| {
            let radius = match fish.kind {
                3 => 0.066,
                2 => 0.078,
                _ => 0.090,
            };
            if Vec2::new(fish.x, fish.y).distance(target) < radius + 0.072 {
                caught += 1;
                reward += Self::reward(fish.kind);
                false
            } else {
                true
            }
        });
        self.nets.push(NetBurst {
            x: target.x,
            y: target.y,
            life: 0.54,
            score: reward,
        });
        if caught > 0 {
            self.score += reward;
            self.coins += reward / 4 + caught;
            self.catches += caught;
            self.feedback = if caught >= 2 {
                format!("一网 {} 条鱼！+{} 分", caught, reward)
            } else if reward >= 50 {
                format!("金鱼入网！+{} 分", reward)
            } else {
                format!("捕到鱼啦！+{} 分", reward)
            };
        } else {
            self.feedback = "渔网落空，跟着鱼群再试一次！".into();
        }
    }

    pub(super) fn advance(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0., 0.05);
        for net in &mut self.nets {
            net.life -= dt;
        }
        self.nets.retain(|net| net.life > 0.);
        for fish in &mut self.fish {
            fish.x += fish.vx * dt;
            fish.y += fish.vy * dt + (fish.phase + fish.x * 7.).sin() * dt * 0.012;
        }
        self.fish
            .retain(|fish| (-0.18..=1.18).contains(&fish.x) && (0.16..=0.91).contains(&fish.y));
        self.spawn_clock -= dt;
        if self.spawn_clock <= 0. && self.fish.len() < 9 {
            self.spawn_fish();
            self.spawn_clock = 0.48;
        }
        self.redraw += dt;
        if self.redraw >= 1. / 30. {
            self.redraw = 0.;
            true
        } else {
            !self.nets.is_empty()
        }
    }
}

pub(super) fn fishing_ui(commands: &mut Commands, s: &Session, art: &Art, compact: bool) {
    let game = &s.fishing;
    let fish_size = if compact { 76. } else { 138. };
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.025, 0.29, 0.53)),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(art.fishing_aquarium.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
            ));
            for (x, angle) in [(12., -0.20), (38., -0.08), (67., 0.12), (90., 0.24)] {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Px(-40.),
                        width: Val::Px(80.),
                        height: Val::Percent(78.),
                        ..default()
                    },
                    Transform::from_rotation(Quat::from_rotation_z(angle)),
                    BackgroundColor(Color::srgba(0.58, 0.95, 1., 0.065)),
                ));
            }
            // Quiet current lines and bubbles make movement under the fish
            // legible without competing with the player's target.
            for (x, y, width) in [
                (4., 29., 22.),
                (46., 43., 18.),
                (72., 67., 16.),
                (16., 78., 25.),
            ] {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        width: Val::Percent(width),
                        height: Val::Px(2.),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.45, 0.87, 0.92, 0.24)),
                ));
            }
            for (x, y, size) in [
                (10., 53., 9.),
                (28., 34., 6.),
                (68., 25., 8.),
                (87., 57., 5.),
            ] {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        width: Val::Px(size),
                        height: Val::Px(size),
                        border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    BorderColor(Color::srgba(0.72, 0.96, 1., 0.46)),
                    BorderRadius::all(Val::Px(size)),
                ));
            }
            for fish in &game.fish {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(fish.x * 100.),
                        top: Val::Percent(fish.y * 100.),
                        width: Val::Px(fish_size * (1. + fish.kind as f32 * 0.08)),
                        height: Val::Px(fish_size * 0.62),
                        ..default()
                    },
                    ZIndex(1),
                ))
                .with_children(|node| fish_sprite(node, art, fish));
            }
            for net in &game.nets {
                let fade = (net.life / 0.54).clamp(0., 1.);
                let expansion = 1. - fade;
                let net_size = fish_size * (0.64 + expansion * 1.70);
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(net.x * 100.),
                        top: Val::Percent(net.y * 100.),
                        width: Val::Px(net_size),
                        height: Val::Px(net_size),
                        margin: UiRect::all(Val::Px(-net_size * 0.5)),
                        border: UiRect::all(Val::Px(3.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::srgba(0.75, 0.98, 1., fade)),
                    BackgroundColor(Color::srgba(0.80, 0.96, 1., fade * 0.12)),
                    BorderRadius::all(Val::Px(net_size)),
                    ZIndex(3),
                ))
                .with_children(|burst| {
                    if net.score > 0 {
                        label(
                            burst,
                            art,
                            format!("+{}", net.score),
                            fish_size * 0.34,
                            Color::srgba(1., 0.93, 0.48, fade),
                        );
                    }
                });
            }
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(if compact { 43. } else { 44. }),
                    bottom: Val::Px(if compact { -30. } else { -42. }),
                    width: Val::Px(if compact { 112. } else { 176. }),
                    height: Val::Px(if compact { 126. } else { 190. }),
                    border: UiRect::all(Val::Px(if compact { 5. } else { 8. })),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.035, 0.16, 0.29)),
                BorderColor(Color::srgb(0.95, 0.67, 0.24)),
                BorderRadius::all(Val::Px(if compact { 32. } else { 48. })),
                ZIndex(4),
            ))
            .with_children(|cannon| {
                cannon.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(30.),
                        top: Val::Px(if compact { -42. } else { -62. }),
                        width: Val::Percent(40.),
                        height: Val::Px(if compact { 72. } else { 108. }),
                        border: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.31, 0.49)),
                    BorderColor(Color::srgb(0.95, 0.67, 0.24)),
                    BorderRadius::all(Val::Px(30.)),
                ));
                label(
                    cannon,
                    art,
                    "网",
                    if compact { 27. } else { 38. },
                    Color::srgb(1., 0.86, 0.40),
                );
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(if compact { 16. } else { 36. }),
                    right: Val::Px(if compact { 16. } else { 36. }),
                    top: Val::Px(if compact { 12. } else { 18. }),
                    padding: UiRect::axes(
                        Val::Px(if compact { 10. } else { 18. }),
                        Val::Px(if compact { 7. } else { 10. }),
                    ),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                BackgroundColor(Color::srgba(0.025, 0.16, 0.31, 0.72)),
                BorderRadius::all(Val::Px(18.)),
                ZIndex(10),
            ))
            .with_children(|top| {
                top.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    label(
                        p,
                        art,
                        "捕鱼达人",
                        if compact { 22. } else { 28. },
                        Color::srgb(0.92, 0.98, 0.92),
                    );
                    label(
                        p,
                        art,
                        format!("得分 {} · 捕获 {} 条", game.score, game.catches),
                        if compact { 13. } else { 16. },
                        Color::srgb(0.58, 0.91, 0.88),
                    );
                });
                top.spawn(row()).with_children(|p| {
                    label(
                        p,
                        art,
                        format!("金币 {}", game.coins),
                        if compact { 14. } else { 17. },
                        Color::srgb(1., 0.88, 0.42),
                    );
                    button(
                        p,
                        art,
                        "?",
                        Action::Help,
                        38.,
                        36.,
                        Color::srgb(0.10, 0.37, 0.46),
                        Color::WHITE,
                    );
                    button(
                        p,
                        art,
                        "游戏库",
                        Action::Library,
                        if compact { 70. } else { 86. },
                        36.,
                        Color::srgb(0.10, 0.37, 0.46),
                        Color::WHITE,
                    );
                });
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.),
                    top: Val::Px(if compact { 66. } else { 80. }),
                    max_width: Val::Percent(74.),
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ZIndex(10),
            ))
            .with_children(|p| {
                label(
                    p,
                    art,
                    &game.feedback,
                    if compact { 14. } else { 17. },
                    Color::srgb(1., 0.90, 0.52),
                )
            });
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.),
                    right: Val::Px(18.),
                    bottom: Val::Px(14.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..row()
                },
                ZIndex(10),
            ))
            .with_children(|bottom| {
                label(
                    bottom,
                    art,
                    "点水面撒网 · 每网 2 金币 · 金鱼 +50",
                    if compact { 13. } else { 16. },
                    Color::srgb(0.72, 0.94, 0.94),
                );
                button(
                    bottom,
                    art,
                    "重来",
                    Action::FishingReset,
                    62.,
                    34.,
                    Color::srgb(0.12, 0.37, 0.46),
                    Color::WHITE,
                );
            });
        });
}

fn fish_sprite(parent: &mut ChildSpawnerCommands, art: &Art, fish: &Fish) {
    let cell = 443.5;
    let inset = 7.;
    let column = fish.kind.min(3) as f32;
    let mut image = ImageNode::new(art.fishing_fish.clone());
    image.rect = Some(Rect::from_corners(
        Vec2::new(column * cell + inset, inset),
        Vec2::new((column + 1.) * cell - inset, cell - inset),
    ));
    image.flip_x = fish.vx < 0.;
    parent.spawn((
        image,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_collects_fish_and_refunds_some_coins() {
        let mut game = FishingGame::new();
        game.fish = vec![Fish {
            x: 0.50,
            y: 0.50,
            vx: 0.,
            vy: 0.,
            kind: 3,
            phase: 0.,
        }];
        game.fire_at(Vec2::new(0.50, 0.50));
        assert!(game.fish.is_empty());
        assert_eq!(game.score, 50);
        assert!(game.coins > 60);
        assert_eq!(game.catches, 1);
        assert_eq!(game.nets.len(), 1);
    }

    #[test]
    fn net_ignores_hud_taps_without_spending() {
        let mut game = FishingGame::new();
        game.fire_at(Vec2::new(0.5, 0.08));
        assert_eq!(game.coins, 60);
        assert!(game.nets.is_empty());
    }
}
