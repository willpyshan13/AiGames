use super::*;

const CARDS: &[(GameMode, Color)] = &[
    (GameMode::ForestTrain, Color::srgb(0.33, 0.54, 0.22)),
    (GameMode::GoldRunner, Color::srgb(0.64, 0.39, 0.12)),
    (GameMode::Descent, Color::srgb(0.25, 0.36, 0.44)),
    (GameMode::GardenDefense, Color::srgb(0.34, 0.57, 0.20)),
    (GameMode::Island, Color::srgb(0.28, 0.62, 0.59)),
    (GameMode::Ball2048, Color::srgb(0.89, 0.47, 0.25)),
    (GameMode::Breakout, Color::srgb(0.18, 0.57, 0.72)),
    (GameMode::ElementLab, Color::srgb(0.22, 0.54, 0.57)),
    (GameMode::LightPath, Color::srgb(0.16, 0.50, 0.72)),
    (GameMode::RabbitHome, Color::srgb(0.39, 0.60, 0.28)),
    (GameMode::Tetris, Color::srgb(0.34, 0.38, 0.82)),
    (GameMode::Math, Color::srgb(0.32, 0.38, 0.78)),
    (GameMode::Literacy, Color::srgb(0.23, 0.63, 0.39)),
    (GameMode::Firefighters, Color::srgb(0.88, 0.30, 0.22)),
    (GameMode::Detective, Color::srgb(0.42, 0.30, 0.66)),
    (GameMode::SpaceFarm, Color::srgb(0.18, 0.56, 0.70)),
    (GameMode::MusicFactory, Color::srgb(0.90, 0.48, 0.18)),
    (GameMode::PixelAssault, Color::srgb(0.28, 0.70, 0.58)),
    (GameMode::MushroomRun, Color::srgb(0.90, 0.37, 0.24)),
    (GameMode::TankBattle, Color::srgb(0.30, 0.55, 0.78)),
    (GameMode::SkyRaid, Color::srgb(0.28, 0.45, 0.78)),
    (GameMode::BubbleIsland, Color::srgb(0.78, 0.38, 0.62)),
    (GameMode::BubbleBomber, Color::srgb(0.18, 0.54, 0.82)),
    (GameMode::FruitNinja, Color::srgb(0.84, 0.31, 0.24)),
    (GameMode::Fortune, Color::srgb(0.18, 0.55, 0.44)),
    (GameMode::Slots, Color::srgb(0.45, 0.29, 0.64)),
    (GameMode::FruitArcade, Color::srgb(0.75, 0.20, 0.28)),
    (GameMode::Fishing, Color::srgb(0.08, 0.50, 0.68)),
    (GameMode::HoneyHunt, Color::srgb(0.86, 0.52, 0.15)),
    (GameMode::MoonFestival, Color::srgb(0.78, 0.16, 0.18)),
    (GameMode::SpaceCourier, Color::srgb(0.21, 0.56, 0.86)),
    (GameMode::Snake, Color::srgb(0.28, 0.68, 0.34)),
    (GameMode::Minesweeper, Color::srgb(0.38, 0.44, 0.54)),
    (GameMode::Classic2048, Color::srgb(0.82, 0.51, 0.22)),
    (GameMode::Klotski, Color::srgb(0.66, 0.34, 0.20)),
    (GameMode::Sokoban, Color::srgb(0.48, 0.34, 0.22)),
    (GameMode::Sudoku, Color::srgb(0.23, 0.44, 0.68)),
];

fn card_color(mode: GameMode) -> Color {
    CARDS
        .iter()
        .find(|(card, _)| *card == mode)
        .map_or(SLATE, |(_, color)| *color)
}

fn card_node(width: f32) -> Node {
    Node {
        width: Val::Px(width),
        height: Val::Px(142.),
        min_height: Val::Px(142.),
        flex_shrink: 0.,
        padding: UiRect::all(Val::Px(14.)),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        column_gap: Val::Px(12.),
        ..default()
    }
}

fn card_content(card: &mut ChildSpawnerCommands, art: &Art, mode: GameMode) {
    let (title, subtitle) = if mode == GameMode::Island {
        ("磁力方块探险家", "闯关 · 云桥 · 餐厅 · 自由搭建")
    } else {
        (mode.title(), mode.subtitle())
    };
    library_icon(card, art, mode);
    card.spawn(Node {
        flex_grow: 1.,
        flex_shrink: 1.,
        min_width: Val::Px(0.),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.),
        ..default()
    })
    .with_children(|info| {
        info.spawn((
            Node {
                max_width: Val::Percent(100.),
                ..default()
            },
            Text::new(title),
            TextFont {
                font: art.font.clone(),
                font_size: 20.,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        info.spawn((
            Node {
                max_width: Val::Percent(100.),
                ..default()
            },
            Text::new(subtitle),
            TextFont {
                font: art.font.clone(),
                font_size: 12.,
                ..default()
            },
            TextColor(Color::srgba(1., 1., 1., 0.86)),
        ));
    });
}

pub(super) fn library_ui(commands: &mut Commands, order: &LibraryOrder, art: &Art, compact: bool) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                padding: UiRect::all(Val::Px(28.)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(18, 38, 48)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..row()
            })
            .with_children(|p| {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.),
                    ..default()
                })
                .with_children(|p| {
                    label(p, art, "游戏库", 32., Color::WHITE);
                    label(p, art, "选择一个小小世界，马上开始玩", 15., SOFT);
                });
                button(
                    p,
                    art,
                    "返回当前游戏",
                    Action::LibraryReturn,
                    150.,
                    44.,
                    SLATE,
                    Color::WHITE,
                );
            });
            let cards = Node {
                width: Val::Percent(100.),
                flex_grow: 1.,
                flex_shrink: 1.,
                min_height: Val::Px(0.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::FlexStart,
                column_gap: Val::Px(16.),
                row_gap: Val::Px(16.),
                flex_wrap: FlexWrap::Wrap,
                overflow: Overflow::scroll_y(),
                ..default()
            };
            root.spawn((cards, ScrollPosition::default(), LibraryCards))
                .with_children(|p| {
                    for &mode in &order.0 {
                        let color = card_color(mode);
                        p.spawn((
                            LibraryCard(mode),
                            Button,
                            ButtonTone(color),
                            Action::SelectGame(mode),
                            card_node(if compact { 278. } else { 320. }),
                            BackgroundColor(color),
                            BorderColor(EDGE),
                            BorderRadius::all(Val::Px(16.)),
                        ))
                        .with_children(|card| card_content(card, art, mode));
                    }
                });
            label(
                root,
                art,
                "磁力方块探险家含四种玩法 · 其他小游戏点卡片直接开始",
                14.,
                SOFT,
            );
        });
}

#[derive(Resource, Default)]
pub(super) struct LibraryDrag {
    start: Option<Vec2>,
    last: Vec2,
    dragging: bool,
    touch: Option<u64>,
    in_list: bool,
    pending_game: Option<GameMode>,
    list: Option<Entity>,
    source: Option<Entity>,
    held_secs: f32,
    reorder: Option<Reorder>,
}
impl LibraryDrag {
    pub(super) fn captures_pointer(&self) -> bool {
        self.start.is_some()
    }

    fn begin(&mut self, point: Vec2, touch: Option<u64>, in_list: bool) {
        *self = Self {
            start: Some(point),
            last: point,
            touch,
            in_list,
            ..default()
        };
    }
    fn move_to(&mut self, point: Vec2, scale: f32) -> f32 {
        let Some(start) = self.start else {
            return 0.;
        };
        let was_dragging = self.dragging;
        if point.distance(start) / scale.max(0.01) >= 8. {
            self.dragging = true;
            self.pending_game = None;
        }
        let previous = if was_dragging { self.last } else { start };
        self.last = point;
        if self.dragging && self.in_list {
            (previous.y - point.y) / scale.max(0.01)
        } else {
            0.
        }
    }
    fn finish(&mut self, released: Option<Vec2>, scale: f32) -> Option<GameMode> {
        if let Some(point) = released {
            self.move_to(point, scale);
        }
        let selected = if released.is_some() && !self.dragging {
            self.pending_game
        } else {
            None
        };
        *self = Self::default();
        selected
    }
}

#[cfg(test)]
mod library_gesture_tests {
    use super::*;
    #[test]
    fn saved_order_keeps_known_cards_and_appends_missing_defaults() {
        let order = LibraryOrder::normalize([
            GameMode::Tetris,
            GameMode::CloudBridge,
            GameMode::Tetris,
            GameMode::Sudoku,
        ]);
        assert_eq!(
            &order.0[..3],
            &[GameMode::Tetris, GameMode::Sudoku, GameMode::ForestTrain]
        );
        assert_eq!(order.0.len(), CARDS.len());
    }

    #[test]
    fn moving_a_card_keeps_the_other_cards_in_relative_order() {
        let cards = [1, 2, 3, 4].map(Entity::from_raw);
        let mut current = cards;
        move_card(&mut current, 2, 0);
        assert_eq!(current, [cards[2], cards[0], cards[1], cards[3]]);
        move_card(&mut current, 0, 2);
        assert_eq!(current, cards);
    }

    #[test]
    fn card_swipe_scrolls_and_cannot_open_game() {
        for touch in [None, Some(7)] {
            let mut g = LibraryDrag::default();
            g.begin(Vec2::new(50., 200.), touch, true);
            g.pending_game = Some(GameMode::Slots);
            assert_eq!(g.move_to(Vec2::new(50., 160.), 2.), 20.);
            assert_eq!(g.move_to(Vec2::new(50., 100.), 2.), 30.);
            assert!(g.finish(Some(Vec2::new(50., 100.)), 2.).is_none());
        }
    }
    #[test]
    fn tap_jitter_does_not_scroll_and_selects_on_release() {
        let mut g = LibraryDrag::default();
        g.begin(Vec2::new(50., 100.), Some(1), true);
        g.pending_game = Some(GameMode::Slots);
        assert_eq!(g.move_to(Vec2::new(52., 102.), 1.), 0.);
        assert!(matches!(
            g.finish(Some(Vec2::new(52., 102.)), 1.),
            Some(GameMode::Slots)
        ));
    }
    #[test]
    fn canceled_or_release_only_swipe_never_selects() {
        for release in [
            None,
            Some(Vec2::new(100., 200.)),
            Some(Vec2::new(200., 100.)),
        ] {
            let mut g = LibraryDrag::default();
            g.begin(Vec2::new(100., 100.), None, true);
            g.pending_game = Some(GameMode::Slots);
            assert!(g.finish(release, 1.).is_none());
        }
        let mut g = LibraryDrag::default();
        g.begin(Vec2::ZERO, None, false);
        assert_eq!(g.move_to(Vec2::new(0., 100.), 1.), 0.);
    }
}

#[derive(Component)]
pub(super) struct LibraryCards;

#[derive(Component)]
pub(super) struct LibraryCard(GameMode);

#[derive(Component)]
pub(super) struct FloatingLibraryCard;

#[derive(Component)]
pub(super) struct LibraryDropLabel;

#[derive(Resource)]
pub(super) struct LibraryOrder(pub(super) Vec<GameMode>);

impl LibraryOrder {
    fn normalize(saved: impl IntoIterator<Item = GameMode>) -> Self {
        let defaults = CARDS.iter().map(|(mode, _)| *mode);
        let mut order = Vec::with_capacity(CARDS.len());
        for mode in saved.into_iter().chain(defaults) {
            if CARDS.iter().any(|(card, _)| *card == mode) && !order.contains(&mode) {
                order.push(mode);
            }
        }
        Self(order)
    }

    fn path() -> PathBuf {
        save_directory().join("library-order-v1.json")
    }

    pub(super) fn load() -> Self {
        // Parse entries separately so an obsolete game ID does not discard the rest.
        let values = fs::read(Self::path())
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Vec<serde_json::Value>>(&bytes).ok())
            .unwrap_or_default();
        Self::normalize(
            values
                .into_iter()
                .filter_map(|entry| serde_json::from_value(entry).ok()),
        )
    }

    fn save(&self) {
        let path = Self::path();
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            fs::create_dir_all(save_directory())?;
            let temp = path.with_extension("tmp");
            fs::write(&temp, serde_json::to_vec(&self.0)?)?;
            fs::rename(temp, path)?;
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("Could not save library order: {error}");
        }
    }
}

struct Reorder {
    original: usize,
    floating: Entity,
    placeholder: Entity,
    label: Entity,
    width: f32,
}

fn move_card(cards: &mut [Entity], from: usize, to: usize) {
    if from < to {
        cards[from..=to].rotate_left(1);
    } else {
        cards[to..=from].rotate_right(1);
    }
}

fn place_floating(node: &mut Node, point: Vec2, width: f32, screen: Vec2) {
    node.left = Val::Px((point.x - width * 0.1).clamp(0., (screen.x - width).max(0.)));
    node.top = Val::Px((point.y + 40.).clamp(0., (screen.y - 158.).max(0.)));
}

fn lift_card(
    commands: &mut Commands,
    art: &Art,
    root: Entity,
    card: Entity,
    mode: GameMode,
    index: usize,
    point: Vec2,
    width: f32,
    screen: Vec2,
) -> Reorder {
    let outline = Color::srgb_u8(185, 223, 255);
    let mut node = card_node(width);
    node.position_type = PositionType::Absolute;
    node.border = UiRect::all(Val::Px(2.));
    place_floating(&mut node, point, width, screen);
    let floating = commands
        .spawn((
            ChildOf(root),
            FloatingLibraryCard,
            GlobalZIndex(100),
            node,
            BackgroundColor(card_color(mode)),
            BorderColor(Color::srgb_u8(200, 211, 255)),
            BorderRadius::all(Val::Px(16.)),
            BoxShadow::new(
                Color::srgba(0., 0., 0., 0.64),
                Val::Px(0.),
                Val::Px(12.),
                Val::Px(0.),
                Val::Px(13.),
            ),
        ))
        .with_children(|p| {
            card_content(p, art, mode);
            p.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(-14.),
                    left: Val::Px(width / 2. - 17.),
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(219, 233, 255)),
                BorderColor(Color::srgb_u8(82, 77, 154)),
                BorderRadius::all(Val::Percent(50.)),
            ));
        })
        .id();
    let mut text = Entity::PLACEHOLDER;
    let placeholder = commands
        .spawn((
            ChildOf(card),
            ZIndex(2),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderColor(outline),
            BackgroundColor(Color::srgb_u8(29, 50, 66)),
            BorderRadius::all(Val::Px(16.)),
        ))
        .with_children(|p| {
            text = p
                .spawn((
                    LibraryDropLabel,
                    Text::new(format!("放到第 {} 个位置", index + 1)),
                    TextFont {
                        font: art.font.clone(),
                        font_size: 23.,
                        ..default()
                    },
                    TextColor(Color::srgb_u8(226, 244, 255)),
                ))
                .id();
            for top in [true, false] {
                p.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(19.),
                        right: Val::Px(19.),
                        top: if top { Val::Px(6.) } else { Val::Auto },
                        bottom: if top { Val::Auto } else { Val::Px(6.) },
                        height: Val::Px(5.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(149, 217, 255)),
                    BorderRadius::all(Val::Px(3.)),
                ));
            }
        })
        .id();
    Reorder {
        original: index,
        floating,
        placeholder,
        label: text,
        width,
    }
}

pub(super) fn library_scroll_input(
    mut commands: Commands,
    mut wheels: EventReader<MouseWheel>,
    touches: Res<Touches>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    art: Res<Art>,
    mut drag: ResMut<LibraryDrag>,
    mut order: ResMut<LibraryOrder>,
    mut s: ResMut<Session>,
    mut list: Query<
        (
            Entity,
            &ChildOf,
            &mut ScrollPosition,
            &ComputedNode,
            &GlobalTransform,
            &Children,
        ),
        With<LibraryCards>,
    >,
    cards: Query<(&LibraryCard, &ComputedNode, &GlobalTransform)>,
    mut floating_nodes: Query<&mut Node, With<FloatingLibraryCard>>,
    mut labels: Query<&mut Text, With<LibraryDropLabel>>,
) {
    if !s.library_open {
        *drag = LibraryDrag::default();
        wheels.clear();
        return;
    }
    let (Ok(window), Ok((list_entity, parent, mut scroll, node, transform, children))) =
        (windows.single(), list.single_mut())
    else {
        return;
    };
    if drag.list.is_some_and(|entity| entity != list_entity) {
        *drag = LibraryDrag::default();
    }
    let viewport = Rect::from_center_size(transform.translation().truncate(), node.size());
    let scale = node.inverse_scale_factor();
    let window_scale = window.scale_factor();
    let screen = Vec2::new(window.width(), window.height()) * window_scale * scale;
    let max_scroll = ((node.content_size().y - node.size().y) * scale).max(0.);
    let physical = |point: Vec2| point * window_scale;
    let inside = |point| viewport.contains(physical(point));
    let entries = |entity| {
        let (card, node, transform) = cards.get(entity).ok()?;
        let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
        Some((card.0, rect))
    };
    let hit = |point| {
        if !inside(point) {
            return None;
        }
        children.iter().find_map(|entity| {
            let (mode, rect) = entries(entity)?;
            rect.contains(physical(point)).then_some((entity, mode))
        })
    };

    for wheel in wheels.read() {
        if drag.reorder.is_none() && window.cursor_position().is_some_and(inside) {
            let distance = match wheel.unit {
                MouseScrollUnit::Line => wheel.y * 42.,
                MouseScrollUnit::Pixel => wheel.y * scale * window_scale,
            };
            scroll.offset_y = (scroll.offset_y - distance).clamp(0., max_scroll);
            if drag.start.is_some() {
                drag.dragging = true;
                drag.pending_game = None;
            }
        }
    }

    if drag.start.is_none() && window.focused {
        let pressed = touches
            .iter_just_pressed()
            .next()
            .map(|touch| (touch.position(), Some(touch.id())))
            .or_else(|| {
                mouse
                    .just_pressed(MouseButton::Left)
                    .then(|| window.cursor_position().map(|p| (p, None)))
                    .flatten()
            });
        if let Some((point, touch)) = pressed.filter(|(point, _)| inside(*point)) {
            drag.begin(point, touch, true);
            drag.list = Some(list_entity);
            if let Some((card, mode)) = hit(point) {
                drag.source = Some(card);
                drag.pending_game = Some(mode);
            }
        }
    }
    if drag.start.is_none() {
        return;
    }

    let canceled = !window.focused
        || keys.just_pressed(KeyCode::Escape)
        || drag.touch.is_some_and(|id| touches.just_canceled(id));
    let (held, point, released) = if canceled {
        (false, None, None)
    } else if let Some(id) = drag.touch {
        let pressed = touches.get_pressed(id);
        (
            pressed.is_some(),
            pressed.map(|touch| touch.position()),
            touches.get_released(id).map(|touch| touch.position()),
        )
    } else {
        let held = mouse.pressed(MouseButton::Left);
        let point = window.cursor_position();
        (
            held,
            point.filter(|_| held),
            point.filter(|_| mouse.just_released(MouseButton::Left)),
        )
    };
    let logical_scale = (window_scale * scale).recip();

    if drag.reorder.is_none() {
        if let Some(point) = point {
            scroll.offset_y =
                (scroll.offset_y + drag.move_to(point, logical_scale)).clamp(0., max_scroll);
            drag.held_secs += time.delta_secs();
            if !drag.dragging && drag.held_secs >= 0.38 {
                if let Some(card) = drag
                    .source
                    .filter(|card| hit(point).is_some_and(|(target, _)| target == *card))
                {
                    if let Some((index, (mode, rect))) = children
                        .iter()
                        .position(|entity| entity == card)
                        .zip(entries(card))
                    {
                        drag.reorder = Some(lift_card(
                            &mut commands,
                            &art,
                            parent.parent(),
                            card,
                            mode,
                            index,
                            physical(point) * scale,
                            rect.width() * scale,
                            screen,
                        ));
                    }
                }
            }
        } else if !held {
            if let Some(point) = released {
                scroll.offset_y =
                    (scroll.offset_y + drag.move_to(point, logical_scale)).clamp(0., max_scroll);
            }
            let released_on = released.and_then(hit).map(|(_, mode)| mode);
            let mode = drag.finish(released, logical_scale);
            if let Some(mode) = mode.filter(|mode| Some(*mode) == released_on) {
                if mode == GameMode::Island {
                    s.library_open = false;
                    s.magnet_modes_open = true;
                    s.ui_dirty = true;
                } else {
                    launch_game(&mut s, mode);
                }
            }
        }
    }

    let (Some(reorder), Some(source)) = (&drag.reorder, drag.source) else {
        return;
    };
    let Some(from) = children.iter().position(|entity| entity == source) else {
        *drag = LibraryDrag::default();
        return;
    };
    let hovered = point.or(released).filter(|&p| inside(p)).map(physical);
    let target = hovered.and_then(|position| {
        children
            .iter()
            .enumerate()
            .filter_map(|(i, entity)| {
                let (_, rect) = entries(entity)?;
                (rect.max.y > viewport.min.y && rect.min.y < viewport.max.y)
                    .then_some((i, rect.center().distance_squared(position)))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    });
    let to = if held {
        target.unwrap_or(from)
    } else {
        target.unwrap_or(reorder.original)
    };
    if from != to || (!held && to != reorder.original) {
        let mut reordered: Vec<_> = children.iter().collect();
        if from != to {
            move_card(&mut reordered, from, to);
            commands.entity(list_entity).replace_children(&reordered);
            if held {
                if let Ok(mut label) = labels.get_mut(reorder.label) {
                    label.0 = format!("放到第 {} 个位置", to + 1);
                }
            }
        }
        if !held && to != reorder.original {
            order.0 = reordered
                .into_iter()
                .filter_map(|entity| entries(entity).map(|(mode, _)| mode))
                .collect();
            order.save();
        }
    }

    if held {
        if let Some(point) = point {
            if let Ok(mut node) = floating_nodes.get_mut(reorder.floating) {
                place_floating(&mut node, physical(point) * scale, reorder.width, screen);
            }
        }
        if let Some(point) = hovered {
            let edge = (58. / scale).min(viewport.height() / 2.);
            let speed = ((point.y - (viewport.max.y - edge)) / edge).clamp(0., 1.)
                - (((viewport.min.y + edge) - point.y) / edge).clamp(0., 1.);
            scroll.offset_y =
                (scroll.offset_y + speed * 720. * time.delta_secs()).clamp(0., max_scroll);
        }
    } else {
        commands.entity(reorder.floating).despawn();
        commands.entity(reorder.placeholder).despawn();
        *drag = LibraryDrag::default();
    }
}
