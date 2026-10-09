use super::*;

const SCENE_W: f32 = 1600.;
const SCENE_H: f32 = 900.;
const LAWN_X: f32 = 176.;
const LAWN_Y: f32 = 178.;
const LAWN_W: f32 = 1248.;
const LAWN_H: f32 = 628.;
const CELL_W: f32 = LAWN_W / 9.;
const CELL_H: f32 = LAWN_H / 5.;
// Generated poses are not exact grid cells. Explicit alpha bounds prevent a
// neighbouring pose from leaking into a walking frame, and align every foot.
const PLANT_FRAMES: [[(f32, f32, f32, f32); 5]; 4] = [
    [
        (47., 37., 270., 294.),
        (321., 62., 559., 290.),
        (593., 29., 828., 293.),
        (871., 44., 1120., 294.),
        (1176., 35., 1364., 295.),
    ],
    [
        (40., 340., 280., 557.),
        (322., 365., 566., 555.),
        (596., 333., 837., 555.),
        (886., 334., 1142., 555.),
        (1180., 330., 1354., 552.),
    ],
    [
        (30., 573., 279., 834.),
        (310., 615., 583., 825.),
        (601., 580., 834., 833.),
        (869., 580., 1137., 834.),
        (1160., 583., 1374., 828.),
    ],
    [
        (39., 849., 274., 1093.),
        (333., 876., 567., 1092.),
        (608., 848., 839., 1091.),
        (877., 859., 1130., 1093.),
        (1165., 849., 1364., 1093.),
    ],
];
const ENEMY_FRAMES: [[(f32, f32, f32, f32); 4]; 4] = [
    [
        (24., 84., 249., 314.),
        (354., 115., 571., 314.),
        (628., 41., 847., 312.),
        (919., 14., 1248., 317.),
    ],
    [
        (34., 367., 260., 606.),
        (356., 390., 574., 597.),
        (624., 333., 847., 605.),
        (915., 322., 1248., 609.),
    ],
    [
        (33., 663., 253., 908.),
        (360., 684., 586., 892.),
        (624., 634., 852., 902.),
        (925., 621., 1246., 908.),
    ],
    [
        (36., 957., 261., 1202.),
        (352., 985., 581., 1204.),
        (635., 929., 858., 1206.),
        (926., 922., 1253., 1215.),
    ],
];

#[derive(Component)]
pub(super) struct GardenBoard;

fn lawn_point(pos: Vec2) -> Vec2 {
    Vec2::new(LAWN_X + pos.x * CELL_W, LAWN_Y + pos.y * CELL_H)
}

fn sun_point(pos: Vec2, age: f32) -> Vec2 {
    lawn_point(pos) + Vec2::new(0., (age * 3.).sin() * 4.)
}

fn garden_stage_name(stage: u8) -> &'static str {
    match stage {
        1 => "晨曦",
        2 => "迷雾",
        3 => "巨人",
        4 => "疾风",
        5 => "铁桶",
        6 => "猛攻",
        7 => "夜袭",
        _ => "终章",
    }
}

pub(super) fn garden_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    boards: Query<(&ComputedNode, &GlobalTransform), With<GardenBoard>>,
    mut s: ResMut<Session>,
) {
    if s.mode != GameMode::GardenDefense || s.library_open || s.help_open {
        return;
    }
    let Ok(window) = windows.single() else { return };
    let Ok((node, transform)) = boards.single() else {
        return;
    };
    let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
    let touch = touches.iter_just_pressed().next();
    let pressed = touch.is_some() || mouse.just_pressed(MouseButton::Left);
    let screen = touch
        .map(|v| v.position())
        .or_else(|| window.cursor_position());
    let local = screen.map(|p| {
        let p = (p * window.scale_factor() - rect.min) / rect.size();
        Vec2::new(p.x * SCENE_W, p.y * SCENE_H)
    });
    let hover = local.and_then(|p| {
        if p.x >= LAWN_X && p.x < LAWN_X + LAWN_W && p.y >= LAWN_Y && p.y < LAWN_Y + LAWN_H {
            Some((
                ((p.y - LAWN_Y) / CELL_H) as usize,
                ((p.x - LAWN_X) / CELL_W) as usize,
            ))
        } else {
            None
        }
    });
    if s.garden.hover != hover {
        s.garden.hover = hover;
        s.ui_dirty = true;
    }
    if !pressed || s.garden.paused || s.garden.over || s.garden.won {
        return;
    }
    let Some(point) = local else { return };
    // Collectables sit above the planting grid and must consume the click first.
    if let Some(id) = s
        .garden
        .suns
        .iter()
        .rev()
        .find(|sun| sun_point(sun.pos, sun.age).distance(point) < 49.)
        .map(|sun| sun.id)
    {
        s.garden.collect(id);
        s.ui_dirty = true;
        return;
    }
    if let Some((row, col)) = hover {
        s.garden.plant(row, col);
        s.ui_dirty = true;
    }
}

fn area(x: f32, y: f32, w: f32, h: f32, scale: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x * scale),
        top: Val::Px(y * scale),
        width: Val::Px(w * scale),
        height: Val::Px(h * scale),
        ..default()
    }
}

fn patch(
    p: &mut ChildSpawnerCommands,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    k: f32,
    color: Color,
    radius: f32,
) {
    p.spawn((
        area(x, y, w, h, k),
        BackgroundColor(color),
        BorderRadius::all(Val::Px(radius * k)),
    ));
}

fn words(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    text: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    size: f32,
    scale: f32,
    color: Color,
) {
    p.spawn((
        Text::new(text),
        TextFont {
            font: art.font.clone(),
            font_size: size * scale,
            ..default()
        },
        TextColor(color),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x * scale),
            top: Val::Px(y * scale),
            width: Val::Px(w * scale),
            ..default()
        },
    ));
}

fn sprite(
    p: &mut ChildSpawnerCommands,
    image: &Handle<Image>,
    cols: usize,
    rows: usize,
    col: usize,
    frame: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    k: f32,
    tint: Color,
) {
    debug_assert_eq!(rows, 4);
    let (bounds, neutral) = if cols == 5 {
        (PLANT_FRAMES[frame][col], PLANT_FRAMES[0][col])
    } else {
        (ENEMY_FRAMES[frame][col], ENEMY_FRAMES[0][col])
    };
    let fit = (w / (neutral.2 - neutral.0 + 6.)).min(h / (neutral.3 - neutral.1 + 6.));
    let sw = (bounds.2 - bounds.0 + 6.) * fit;
    let sh = (bounds.3 - bounds.1 + 6.) * fit;
    p.spawn((
        area(x + (w - sw) * 0.5, y + h - sh, sw, sh, k),
        ImageNode {
            image: image.clone(),
            color: tint,
            rect: Some(Rect::new(
                bounds.0 - 3.,
                bounds.1 - 3.,
                bounds.2 + 3.,
                bounds.3 + 3.,
            )),
            ..default()
        },
    ));
}

fn wood_button(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    title: &str,
    action: Action,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    k: f32,
    active: bool,
) {
    let color = if active {
        Color::srgb(0.32, 0.51, 0.16)
    } else {
        Color::srgb(0.48, 0.27, 0.12)
    };
    p.spawn((
        Button,
        action,
        ButtonTone(color),
        Node {
            border: UiRect::all(Val::Px(3. * k)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..area(x, y, w, h, k)
        },
        BackgroundColor(color),
        BorderColor(Color::srgb(0.91, 0.69, 0.35)),
        BorderRadius::all(Val::Px(14. * k)),
    ))
    .with_children(|b| label(b, art, title, 22. * k, Color::srgb(1., 0.95, 0.77)));
}

fn prop(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    index: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    k: f32,
    alpha: f32,
) {
    let bounds = [
        (25., 82., 570., 636.),
        (579., 138., 1137., 604.),
        (1200., 245., 1535., 575.),
        (1692., 199., 2128., 597.),
    ][index];
    p.spawn((
        area(x, y, w, h, k),
        ImageNode {
            image: art.garden_props.clone(),
            color: Color::srgba(1., 1., 1., alpha),
            rect: Some(Rect::new(bounds.0, bounds.1, bounds.2, bounds.3)),
            ..default()
        },
    ));
}

fn glow_sun(p: &mut ChildSpawnerCommands, art: &Art, center: Vec2, r: f32, k: f32, alpha: f32) {
    patch(
        p,
        center.x - r * 1.7,
        center.y - r * 1.7,
        r * 3.4,
        r * 3.4,
        k,
        Color::srgba(1., 0.85, 0.25, 0.12 * alpha),
        r * 2.,
    );
    prop(
        p,
        art,
        0,
        center.x - r * 1.45,
        center.y - r * 1.45,
        r * 2.9,
        r * 2.9,
        k,
        alpha,
    );
}

pub(super) fn garden_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    viewport_width: f32,
    viewport_height: f32,
) {
    let game = &s.garden;
    let k = (viewport_width / SCENE_W).min(viewport_height / SCENE_H);
    let cream = Color::srgb(1., 0.95, 0.77);
    let dark = Color::srgb(0.25, 0.16, 0.06);
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
            BackgroundColor(Color::srgb(0.09, 0.17, 0.08)),
        ))
        .with_children(|root| {
            root.spawn((
                GardenBoard,
                WorldInputBlocker,
                Node {
                    width: Val::Px(SCENE_W * k),
                    height: Val::Px(SCENE_H * k),
                    position_type: PositionType::Relative,
                    flex_shrink: 0.,
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn((
                    area(0., 0., SCENE_W, SCENE_H, k),
                    ImageNode::new(art.garden_backdrop.clone()),
                ));

                // A preview shows the exact cell that will be planted, not a detached cursor.
                if let Some((row, col)) = game.hover {
                    if !game.paused && !game.over && !game.won {
                        let occupied = game
                            .plants
                            .iter()
                            .any(|plant| plant.row == row && plant.col == col);
                        let okay = game.shovel
                            || (!occupied
                                && game.sunlight >= game.selected.cost()
                                && game.cooldowns[game.selected.index()] <= 0.);
                        let color = if okay {
                            Color::srgba(0.62, 1., 0.24, 0.25)
                        } else {
                            Color::srgba(1., 0.24, 0.16, 0.23)
                        };
                        p.spawn((
                            Node {
                                border: UiRect::all(Val::Px(3. * k)),
                                ..area(
                                    LAWN_X + col as f32 * CELL_W + 4.,
                                    LAWN_Y + row as f32 * CELL_H + 3.,
                                    CELL_W - 8.,
                                    CELL_H - 6.,
                                    k,
                                )
                            },
                            BackgroundColor(color),
                            BorderColor(if okay {
                                Color::srgb(0.76, 1., 0.43)
                            } else {
                                Color::srgb(1., 0.50, 0.32)
                            }),
                            BorderRadius::all(Val::Px(12. * k)),
                        ));
                        if !occupied && !game.shovel {
                            let v = lawn_point(Vec2::new(col as f32 + 0.5, row as f32 + 0.5));
                            sprite(
                                p,
                                &art.garden_plants,
                                5,
                                4,
                                game.selected.index(),
                                0,
                                v.x - 66.,
                                v.y - 63.,
                                132.,
                                132.,
                                k,
                                Color::srgba(1., 1., 1., 0.36),
                            );
                        }
                    }
                }

                for row in 0..5 {
                    let y = LAWN_Y + (row as f32 + 0.5) * CELL_H;
                    if game.guards[row] || game.guard_x[row].is_some() {
                        let x = game.guard_x[row]
                            .map(|x| LAWN_X + x * CELL_W)
                            .unwrap_or(LAWN_X - 45.);
                        if game.guard_x[row].is_some() {
                            patch(
                                p,
                                x - 100.,
                                y - 30.,
                                115.,
                                68.,
                                k,
                                Color::srgba(0.95, 0.86, 0.28, 0.32),
                                34.,
                            );
                        }
                        patch(
                            p,
                            x - 26.,
                            y + 31.,
                            56.,
                            12.,
                            k,
                            Color::srgba(0.10, 0.25, 0.04, 0.22),
                            8.,
                        );
                        prop(p, art, 1, x - 38., y - 26., 80., 67., k, 1.);
                    }
                }
                // Render back-to-front per lane, so feet stay grounded and rows never cross.
                for row in 0..5 {
                    for plant in game.plants.iter().filter(|plant| plant.row == row) {
                        let v = lawn_point(Vec2::new(plant.col as f32 + 0.5, row as f32 + 0.5));
                        let sway = (plant.age * 2.8 + plant.col as f32).sin();
                        let recoil = plant.recoil.clamp(0., 0.3) / 0.3;
                        let frame = if recoil > 0.65 {
                            2
                        } else if recoil > 0.15 {
                            1
                        } else if (plant.age * 1.6) as usize % 4 == 3 {
                            3
                        } else {
                            0
                        };
                        let w = 132. + sway * 2. - recoil * 9.;
                        let h = 132. - sway * 2. + recoil * 4.;
                        patch(
                            p,
                            v.x - 39.,
                            v.y + 43.,
                            80.,
                            17.,
                            k,
                            Color::srgba(0.10, 0.23, 0.03, 0.20),
                            10.,
                        );
                        sprite(
                            p,
                            &art.garden_plants,
                            5,
                            4,
                            plant.kind.index(),
                            frame,
                            v.x - w * 0.5 - recoil * 7.,
                            v.y + 64. - h,
                            w,
                            h,
                            k,
                            if plant.flash > 0. {
                                Color::srgb(1.8, 1.5, 1.3)
                            } else {
                                Color::WHITE
                            },
                        );
                        if plant.hp < plant.max_hp * 0.94 {
                            patch(
                                p,
                                v.x - 36.,
                                v.y - 65.,
                                72.,
                                9.,
                                k,
                                Color::srgba(0.14, 0.18, 0.06, 0.8),
                                5.,
                            );
                            patch(
                                p,
                                v.x - 34.,
                                v.y - 63.,
                                68. * (plant.hp / plant.max_hp).clamp(0., 1.),
                                5.,
                                k,
                                Color::srgb(0.89, 0.83, 0.26),
                                3.,
                            );
                        }
                    }
                    for enemy in game.enemies.iter().filter(|enemy| enemy.row == row) {
                        let v = lawn_point(Vec2::new(enemy.x, row as f32 + 0.5));
                        let big = enemy.kind == GardenEnemyKind::Brute;
                        let frame =
                            (enemy.age * if enemy.slow > 0. { 3. } else { 7. }) as usize % 4;
                        let bob = (enemy.age * if enemy.slow > 0. { 6. } else { 14. }).sin()
                            * if enemy.biting { 2. } else { 4. };
                        let lunge = if enemy.biting {
                            (enemy.age * 12.).sin().max(0.) * 8.
                        } else {
                            0.
                        };
                        let w = if big { 175. } else { 140. };
                        let h = if big { 167. } else { 140. };
                        patch(
                            p,
                            v.x - w * 0.29,
                            v.y + 45.,
                            w * 0.64,
                            19.,
                            k,
                            Color::srgba(0.10, 0.19, 0.03, 0.27),
                            12.,
                        );
                        sprite(
                            p,
                            &art.garden_enemies,
                            4,
                            4,
                            enemy.kind.index(),
                            frame,
                            v.x - w * 0.5 - lunge,
                            v.y + 65. - h + bob,
                            w,
                            h,
                            k,
                            if enemy.flash > 0. {
                                Color::srgb(2., 1.7, 1.5)
                            } else if enemy.slow > 0. {
                                Color::srgb(0.60, 0.87, 1.)
                            } else {
                                Color::WHITE
                            },
                        );
                        if enemy.slow > 0. {
                            patch(
                                p,
                                v.x - 47.,
                                v.y + 39.,
                                94.,
                                22.,
                                k,
                                Color::srgba(0.54, 0.88, 1., 0.42),
                                14.,
                            );
                            for i in 0..3 {
                                let t = enemy.age * 2. + i as f32 * 2.;
                                words(
                                    p,
                                    art,
                                    "✦",
                                    v.x + t.cos() * 43. - 8.,
                                    v.y - 20. + t.sin() * 35.,
                                    22.,
                                    20.,
                                    k,
                                    Color::srgb(0.78, 0.99, 1.),
                                );
                            }
                        }
                        patch(
                            p,
                            v.x - 40.,
                            v.y + 49. - h + bob,
                            80.,
                            10.,
                            k,
                            Color::srgb(0.18, 0.20, 0.08),
                            5.,
                        );
                        patch(
                            p,
                            v.x - 38.,
                            v.y + 51. - h + bob,
                            76. * (enemy.hp / enemy.max_hp).clamp(0., 1.),
                            6.,
                            k,
                            Color::srgb(0.98, 0.36, 0.21),
                            3.,
                        );
                    }
                }
                for shot in &game.shots {
                    let v =
                        lawn_point(Vec2::new(shot.x, shot.row as f32 + 0.5)) + Vec2::new(0., -9.);
                    let c = if shot.frost {
                        Color::srgb(0.35, 0.85, 1.)
                    } else {
                        Color::srgb(0.64, 0.89, 0.17)
                    };
                    for n in 1..4 {
                        patch(
                            p,
                            v.x - 15. - n as f32 * 10.,
                            v.y - 8. + n as f32,
                            18.,
                            14.,
                            k,
                            c.with_alpha(0.18 / n as f32),
                            9.,
                        );
                    }
                    prop(
                        p,
                        art,
                        if shot.frost { 3 } else { 2 },
                        v.x - 15.,
                        v.y - 15.,
                        30.,
                        30.,
                        k,
                        1.,
                    );
                }
                for effect in &game.effects {
                    let v = lawn_point(effect.pos);
                    let t = effect.age / effect.kind.duration();
                    let alpha = (1. - t).clamp(0., 1.);
                    match effect.kind {
                        GardenEffectKind::Chili => {
                            patch(
                                p,
                                LAWN_X,
                                v.y - 32.,
                                LAWN_W,
                                64.,
                                k,
                                Color::srgba(1., 0.48, 0.09, alpha * 0.45),
                                32.,
                            );
                            for i in 0..24 {
                                let x = LAWN_X + i as f32 * LAWN_W / 24.;
                                let y = v.y - 12. - (i as f32 * 2.7).sin() * 32. - t * 55.;
                                patch(
                                    p,
                                    x,
                                    y,
                                    28. * alpha,
                                    48. * alpha,
                                    k,
                                    Color::srgba(1., 0.82, 0.22, alpha),
                                    18.,
                                );
                            }
                        }
                        GardenEffectKind::Collect => {
                            let target = Vec2::new(316., 95.);
                            let a = t * t;
                            let point = v.lerp(target, a)
                                - Vec2::new(0., (t * std::f32::consts::PI).sin() * 100.);
                            glow_sun(p, art, point, 19. * (1. - t * 0.5), k, alpha);
                            words(
                                p,
                                art,
                                "+25",
                                v.x - 25.,
                                v.y - 45. - t * 40.,
                                90.,
                                24.,
                                k,
                                cream.with_alpha(alpha),
                            );
                        }
                        _ => {
                            let frost = effect.kind == GardenEffectKind::Frost;
                            let color = if frost {
                                Color::srgba(0.70, 0.96, 1., alpha)
                            } else {
                                Color::srgba(1., 0.92, 0.44, alpha)
                            };
                            for i in 0..8 {
                                let angle = i as f32 * std::f32::consts::TAU / 8.;
                                let r = 8. + t * 48.;
                                let x = v.x + angle.cos() * r;
                                let y = v.y - 10. + angle.sin() * r + t * t * 20.;
                                patch(
                                    p,
                                    x - 4.,
                                    y - 4.,
                                    8. * alpha + 2.,
                                    8. * alpha + 2.,
                                    k,
                                    color,
                                    6.,
                                );
                            }
                        }
                    }
                }
                for sun in &game.suns {
                    let v = sun_point(sun.pos, sun.age);
                    let pulse = 1. + (sun.age * 3.).sin() * 0.05;
                    glow_sun(p, art, v, 28. * pulse, k, 1.);
                }

                // The toolbar is assembled over painted wood, keeping text sharp at all sizes.
                words(
                    p,
                    art,
                    "花园守卫",
                    30.,
                    52.,
                    280.,
                    52.,
                    k,
                    Color::srgb(0.23, 0.12, 0.04),
                );
                words(
                    p,
                    art,
                    "花园守卫",
                    27.,
                    47.,
                    280.,
                    52.,
                    k,
                    Color::srgb(1., 0.82, 0.29),
                );
                patch(
                    p,
                    284.,
                    43.,
                    174.,
                    102.,
                    k,
                    Color::srgba(0.30, 0.16, 0.04, 0.76),
                    19.,
                );
                glow_sun(p, art, Vec2::new(319., 96.), 25., k, 1.);
                words(p, art, "阳光", 354., 55., 92., 20., k, cream);
                words(
                    p,
                    art,
                    format!("{}", game.sunlight),
                    351.,
                    82.,
                    98.,
                    35.,
                    k,
                    Color::srgb(1., 0.86, 0.34),
                );
                wood_button(
                    p,
                    art,
                    "收集阳光 · C",
                    Action::GardenCollectAll,
                    306.,
                    120.,
                    150.,
                    42.,
                    k,
                    !game.suns.is_empty() && !game.paused && !game.over,
                );
                for (index, kind) in GardenPlantKind::ALL.into_iter().enumerate() {
                    let x = 478. + index as f32 * 125.;
                    let selected = game.selected == kind && !game.shovel;
                    let available = game.sunlight >= kind.cost() && game.cooldowns[index] <= 0.;
                    let background = if selected {
                        Color::srgb(0.89, 0.97, 0.61)
                    } else {
                        Color::srgb(0.96, 0.88, 0.64)
                    };
                    p.spawn((
                        Button,
                        Action::GardenSelect(index),
                        ButtonTone(background),
                        Node {
                            border: UiRect::all(Val::Px(if selected { 4. * k } else { 3. * k })),
                            ..area(x, 10., 116., 155., k)
                        },
                        BackgroundColor(background),
                        BorderColor(if selected {
                            Color::srgb(0.64, 0.98, 0.17)
                        } else {
                            Color::srgb(0.61, 0.39, 0.15)
                        }),
                        BorderRadius::all(Val::Px(13. * k)),
                    ))
                    .with_children(|card| {
                        sprite(
                            card,
                            &art.garden_plants,
                            5,
                            4,
                            index,
                            0,
                            9.,
                            3.,
                            94.,
                            91.,
                            k,
                            Color::WHITE,
                        );
                        words(card, art, kind.name(), 9., 93., 104., 20., k, dark);
                        words(
                            card,
                            art,
                            format!("☀ {}", kind.cost()),
                            21.,
                            121.,
                            90.,
                            21.,
                            k,
                            dark,
                        );
                        words(
                            card,
                            art,
                            format!("{}", index + 1),
                            5.,
                            4.,
                            20.,
                            14.,
                            k,
                            Color::srgb(0.51, 0.35, 0.12),
                        );
                        if !available {
                            patch(
                                card,
                                0.,
                                0.,
                                110.,
                                149.,
                                k,
                                Color::srgba(0.20, 0.19, 0.10, 0.37),
                                10.,
                            );
                            if game.cooldowns[index] > 0. {
                                let ratio = (game.cooldowns[index] / kind.cooldown()).clamp(0., 1.);
                                patch(
                                    card,
                                    0.,
                                    149. * (1. - ratio),
                                    110.,
                                    149. * ratio,
                                    k,
                                    Color::srgba(0.10, 0.17, 0.05, 0.32),
                                    8.,
                                );
                                words(
                                    card,
                                    art,
                                    format!("{:.0}s", game.cooldowns[index].ceil()),
                                    29.,
                                    50.,
                                    78.,
                                    29.,
                                    k,
                                    cream,
                                );
                            }
                        }
                    });
                }
                words(
                    p,
                    art,
                    format!("第 {} 关 · {}", game.stage, garden_stage_name(game.stage)),
                    1130.,
                    37.,
                    190.,
                    29.,
                    k,
                    cream,
                );
                words(
                    p,
                    art,
                    if game.ready {
                        "准备布阵".to_owned()
                    } else {
                        format!("第 {} / {} 波", game.wave, game.total_waves)
                    },
                    1130.,
                    77.,
                    195.,
                    23.,
                    k,
                    cream,
                );
                patch(
                    p,
                    1126.,
                    115.,
                    229.,
                    17.,
                    k,
                    Color::srgb(0.21, 0.14, 0.04),
                    9.,
                );
                patch(
                    p,
                    1130.,
                    119.,
                    221. * game.progress.clamp(0., 1.),
                    9.,
                    k,
                    Color::srgb(0.58, 0.86, 0.17),
                    6.,
                );
                wood_button(
                    p,
                    art,
                    if game.ready {
                        "开始守卫"
                    } else if game.paused {
                        "继续"
                    } else {
                        "暂停"
                    },
                    if game.ready {
                        Action::GardenStart
                    } else {
                        Action::GardenPause
                    },
                    1380.,
                    50.,
                    160.,
                    78.,
                    k,
                    game.ready,
                );

                wood_button(
                    p,
                    art,
                    "游戏库",
                    Action::Library,
                    25.,
                    835.,
                    148.,
                    48.,
                    k,
                    false,
                );
                wood_button(
                    p,
                    art,
                    "重来",
                    Action::GardenReset,
                    188.,
                    835.,
                    115.,
                    48.,
                    k,
                    false,
                );
                wood_button(
                    p,
                    art,
                    if game.shovel {
                        "铲子已选"
                    } else {
                        "铲除植物"
                    },
                    Action::GardenShovel,
                    1325.,
                    835.,
                    166.,
                    48.,
                    k,
                    game.shovel,
                );
                let hint = if game.shovel {
                    "点击植物铲除 · 返还一半阳光"
                } else {
                    game.hint.as_str()
                };
                words(p, art, hint, 465., 846., 820., 24., k, cream);
                words(
                    p,
                    art,
                    format!("{} 分", game.score),
                    1490.,
                    842.,
                    108.,
                    21.,
                    k,
                    cream,
                );

                if game.over || game.won || game.paused {
                    patch(
                        p,
                        0.,
                        174.,
                        1600.,
                        650.,
                        k,
                        Color::srgba(0.09, 0.17, 0.05, 0.49),
                        0.,
                    );
                    p.spawn((
                        Node {
                            border: UiRect::all(Val::Px(5. * k)),
                            ..area(477., 315., 646., 280., k)
                        },
                        BackgroundColor(Color::srgb(0.34, 0.23, 0.09)),
                        BorderColor(Color::srgb(0.92, 0.76, 0.40)),
                        BorderRadius::all(Val::Px(28. * k)),
                    ))
                    .with_children(|panel| {
                        let title = if game.won {
                            if game.stage >= 8 {
                                "终极大胜利！"
                            } else {
                                "守住花园啦！"
                            }
                        } else if game.over {
                            "再试一次吧"
                        } else {
                            "休息一下"
                        };
                        words(
                            panel,
                            art,
                            title,
                            165.,
                            24.,
                            425.,
                            42.,
                            k,
                            Color::srgb(1., 0.85, 0.32),
                        );
                        words(
                            panel,
                            art,
                            if game.won {
                                "植物们为你欢呼，新的挑战在前面"
                            } else if game.over {
                                "多种向日葵，用坚果挡住怪物"
                            } else {
                                "你的花园在这里等你回来"
                            },
                            108.,
                            100.,
                            480.,
                            24.,
                            k,
                            cream,
                        );
                        let (text, action) = if game.won && game.stage < 8 {
                            ("下一关", Action::GardenNext)
                        } else if game.paused {
                            ("继续守卫", Action::GardenPause)
                        } else {
                            ("重新挑战", Action::GardenReset)
                        };
                        wood_button(panel, art, text, action, 204., 179., 235., 66., k, true);
                    });
                }
            });
        });
}
