use super::*;

const LEVELS: [(&str, [&str; 8]); 6] = [
    (
        "仓库入门",
        [
            "########", "#@#  . #", "# #$ # #", "#      #", "# # $# #", "# #  . #", "#      #",
            "########",
        ],
    ),
    (
        "绕过货架",
        [
            "########", "#    . #", "# # # .#", "# #$   #", "# # $# #", "#@     #", "#      #",
            "########",
        ],
    ),
    (
        "转角搬运",
        [
            "########", "#..#   #", "#   $# #", "#      #", "## $ # #", "#    # #", "#     @#",
            "########",
        ],
    ),
    (
        "双箱协作",
        [
            "########", "#@#  # #", "# #  # #", "#  $$  #", "# #    #", "#  # ..#", "#      #",
            "########",
        ],
    ),
    (
        "窄道调度",
        [
            "########", "#     .#", "# #$## #", "# #    #", "# # $  #", "#   ##.#", "#@     #",
            "########",
        ],
    ),
    (
        "仓库挑战",
        [
            "########", "#@    .#", "# ## # #", "#  $   #", "# # $  #", "#  ##  #", "#.     #",
            "########",
        ],
    ),
];

type Position = (i8, i8);
#[derive(Clone)]
struct SokobanStep {
    player: Position,
    crates: Vec<Position>,
    moves: u32,
}

pub(super) struct SokobanGame {
    pub(super) player: Position,
    pub(super) crates: Vec<Position>,
    pub(super) moves: u32,
    pub(super) won: bool,
    pub(super) level: usize,
    pub(super) choosing: bool,
    completed: [bool; 6],
    history: Vec<SokobanStep>,
}

impl SokobanGame {
    pub(super) fn new() -> Self {
        let mut game = Self {
            player: (1, 1),
            crates: vec![],
            moves: 0,
            won: false,
            level: 0,
            choosing: false,
            completed: [false; 6],
            history: vec![],
        };
        game.reset();
        game
    }
    pub(super) fn reset(&mut self) {
        self.crates.clear();
        for (y, row) in LEVELS[self.level].1.iter().enumerate() {
            for (x, tile) in row.bytes().enumerate() {
                match tile {
                    b'@' => self.player = (x as i8, y as i8),
                    b'$' => self.crates.push((x as i8, y as i8)),
                    _ => {}
                }
            }
        }
        self.moves = 0;
        self.won = false;
        self.choosing = false;
        self.history.clear();
    }
    pub(super) fn select_level(&mut self, level: usize) {
        if level < LEVELS.len() {
            self.level = level;
            self.reset();
        }
    }
    #[cfg(debug_assertions)]
    pub(super) fn capture_demo(&mut self, state: &str) {
        if state == "selection" {
            self.choosing = true;
            return;
        }
        let route = if state == "last" {
            self.select_level(5);
            "DDRRDRLURRDDURUUDDDDLLLL"
        } else if state == "win" {
            "DDRRRUULDRDDLDDLLUUURRDDRUUULUR"
        } else if state == "playing" {
            "DDRRRUULDRDD"
        } else {
            return;
        };
        for c in route.bytes() {
            let (dx, dy) = match c {
                b'U' => (0, -1),
                b'D' => (0, 1),
                b'L' => (-1, 0),
                _ => (1, 0),
            };
            self.move_player(dx, dy);
        }
    }
    pub(super) fn next(&mut self) {
        if !self.won || self.choosing {
            return;
        }
        if self.level + 1 < LEVELS.len() {
            self.select_level(self.level + 1);
        } else {
            self.choosing = true;
        }
    }
    pub(super) fn undo(&mut self) -> bool {
        if self.choosing {
            return false;
        }
        if let Some(step) = self.history.pop() {
            self.player = step.player;
            self.crates = step.crates;
            self.moves = step.moves;
            self.won = false;
            return true;
        }
        false
    }
    fn wall(&self, (x, y): Position) -> bool {
        !(0..8).contains(&x)
            || !(0..8).contains(&y)
            || LEVELS[self.level].1[y as usize].as_bytes()[x as usize] == b'#'
    }
    fn target(&self, (x, y): Position) -> bool {
        (0..8).contains(&x)
            && (0..8).contains(&y)
            && LEVELS[self.level].1[y as usize].as_bytes()[x as usize] == b'.'
    }
    fn delivered(&self) -> usize {
        self.crates.iter().filter(|&&p| self.target(p)).count()
    }
    pub(super) fn move_player(&mut self, dx: i8, dy: i8) -> bool {
        if self.won || self.choosing || !matches!((dx, dy), (1, 0) | (-1, 0) | (0, 1) | (0, -1)) {
            return false;
        }
        let next = (self.player.0 + dx, self.player.1 + dy);
        if self.wall(next) {
            return false;
        }
        let index = self.crates.iter().position(|&p| p == next);
        let pushed = (next.0 + dx, next.1 + dy);
        if index.is_some() && (self.wall(pushed) || self.crates.contains(&pushed)) {
            return false;
        }
        self.history.push(SokobanStep {
            player: self.player,
            crates: self.crates.clone(),
            moves: self.moves,
        });
        if let Some(index) = index {
            self.crates[index] = pushed;
        }
        self.player = next;
        self.moves += 1;
        self.won = self.delivered() == self.crates.len();
        if self.won {
            self.completed[self.level] = true;
        }
        true
    }
}

// All artwork is sampled from the approved 1536×1024 design. The board is
// redrawn from independent wall, floor, worker, goal and crate tiles each turn.
// Shared coordinates keep rendered controls and their hit regions aligned.
fn sokoban_rect(rect: [f32; 4], scale: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(rect[0] * scale),
        top: Val::Px(rect[1] * scale),
        width: Val::Px(rect[2] * scale),
        height: Val::Px(rect[3] * scale),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}
fn sokoban_image(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    source: [f32; 4],
    destination: [f32; 4],
    scale: f32,
) {
    let mut image = ImageNode::new(art.sokoban_atlas.clone());
    image.rect = Some(Rect::new(
        source[0],
        source[1],
        source[0] + source[2],
        source[1] + source[3],
    ));
    image.image_mode = NodeImageMode::Stretch;
    parent.spawn((
        sokoban_rect(destination, scale),
        image,
        bevy::ui::FocusPolicy::Pass,
    ));
}
fn sokoban_text(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    rect: [f32; 4],
    text: impl Into<String>,
    size: f32,
    scale: f32,
) {
    parent
        .spawn((sokoban_rect(rect, scale), bevy::ui::FocusPolicy::Pass))
        .with_children(|p| label(p, art, text, size * scale, Color::srgb(0.055, 0.17, 0.27)));
}
fn sokoban_number(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    rect: [f32; 4],
    text: String,
    size: f32,
    scale: f32,
) {
    for (dx, dy) in [(0., 0.), (-0.8, 0.), (0.8, 0.), (0., -0.8), (0., 0.8)] {
        sokoban_text(
            parent,
            art,
            [rect[0] + dx, rect[1] + dy, rect[2], rect[3]],
            text.clone(),
            size,
            scale,
        );
    }
}
#[derive(Component)]
pub(super) struct SokobanHotspot;
fn sokoban_hotspot(parent: &mut ChildSpawnerCommands, action: Action, rect: [f32; 4], scale: f32) {
    parent.spawn((
        Button,
        action,
        SokobanHotspot,
        sokoban_rect(rect, scale),
        BackgroundColor(Color::NONE),
        BorderRadius::all(Val::Px(26. * scale)),
        ZIndex(5),
    ));
}
pub(super) fn sokoban_feedback(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SokobanHotspot>),
    >,
) {
    for (interaction, mut color) in &mut buttons {
        color.0 = match interaction {
            Interaction::Pressed => Color::srgba(0., 0.08, 0.15, 0.20),
            Interaction::Hovered => Color::srgba(1., 1., 1., 0.12),
            Interaction::None => Color::NONE,
        };
    }
}
fn sokoban_panel_button(
    parent: &mut ChildSpawnerCommands,
    art: &Art,
    rect: [f32; 4],
    text: impl Into<String>,
    action: Action,
    scale: f32,
    active: bool,
) {
    let color = if active {
        Color::srgb(0.12, 0.47, 0.49)
    } else {
        Color::srgb(0.24, 0.48, 0.66)
    };
    parent
        .spawn((
            sokoban_rect(rect, scale),
            BackgroundColor(color),
            BorderRadius::all(Val::Px(24. * scale)),
            BorderColor(Color::srgb(0.62, 0.77, 0.76)),
        ))
        .with_children(|p| {
            label(
                p,
                art,
                text,
                if matches!(action, Action::SokobanNext) {
                    42. * scale
                } else {
                    28. * scale
                },
                Color::WHITE,
            )
        });
    sokoban_hotspot(parent, action, rect, scale);
}

pub(super) fn sokoban_ui(commands: &mut Commands, s: &Session, art: &Art, width: f32, height: f32) {
    let g = &s.sokoban;
    let scale = (width / 1536.).min(height / 1024.);
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
            BackgroundColor(Color::srgb(0.34, 0.22, 0.13)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(1536. * scale),
                height: Val::Px(1024. * scale),
                flex_shrink: 0.,
                ..default()
            })
            .with_children(|stage| {
                sokoban_image(
                    stage,
                    art,
                    [0., 0., 1536., 1024.],
                    [0., 0., 1536., 1024.],
                    scale,
                );
                for y in 0..8_i8 {
                    for x in 0..8_i8 {
                        let p = (x, y);
                        let tile = if g.wall(p) {
                            [400., 290., 96., 96.]
                        } else if g.crates.contains(&p) {
                            [496., 290., 96., 96.]
                        } else if g.player == p {
                            [305., 193., 95., 108.]
                        } else if g.target(p) {
                            [687., 193., 96., 96.]
                        } else {
                            [496., 386., 96., 96.]
                        };
                        let rect = [
                            210. + x as f32 * 95.25,
                            105. + y as f32 * 94.75,
                            95.25,
                            94.75,
                        ];
                        sokoban_image(stage, art, tile, rect, scale);
                        if g.target(p) && (g.crates.contains(&p) || g.player == p) {
                            let mut node = sokoban_rect(rect, scale);
                            node.border = UiRect::all(Val::Px(4. * scale));
                            stage.spawn((
                                node,
                                BorderColor(Color::srgb(1., 0.76, 0.18)),
                                BorderRadius::all(Val::Px(8. * scale)),
                                bevy::ui::FocusPolicy::Pass,
                            ));
                        }
                    }
                }
                // The approved first-level title is preserved verbatim. Other levels
                // replace only the text capsule, not the surrounding header artwork.
                if g.level != 0 {
                    stage.spawn((
                        sokoban_rect([580., 25., 380., 58.], scale),
                        BackgroundColor(Color::srgb(0.96, 0.83, 0.64)),
                        BorderRadius::all(Val::Px(30. * scale)),
                    ));
                    sokoban_text(
                        stage,
                        art,
                        [580., 25., 380., 58.],
                        format!("第 {} 关 · {}", g.level + 1, LEVELS[g.level].0),
                        29.,
                        scale,
                    );
                }
                sokoban_hotspot(stage, Action::Library, [223., 16., 91., 73.], scale);
                sokoban_hotspot(stage, Action::SokobanChoose, [1120., 20., 190., 65.], scale);
                if g.choosing {
                    stage.spawn((
                        sokoban_rect([1044., 132., 361., 809.], scale),
                        BackgroundColor(Color::srgb(0.99, 0.94, 0.82)),
                        BorderRadius::all(Val::Px(28. * scale)),
                    ));
                    sokoban_text(stage, art, [1060., 142., 330., 62.], "选择关卡", 40., scale);
                    for (index, (name, _)) in LEVELS.iter().enumerate() {
                        let status = if g.completed[index] {
                            " · 已完成"
                        } else if index == g.level {
                            " · 当前"
                        } else {
                            ""
                        };
                        sokoban_panel_button(
                            stage,
                            art,
                            [1060., 221. + index as f32 * 95., 330., 80.],
                            format!("{}  {}{}", index + 1, name, status),
                            Action::SokobanLevel(index),
                            scale,
                            index == g.level,
                        );
                    }
                    sokoban_panel_button(
                        stage,
                        art,
                        [1060., 825., 330., 80.],
                        "返回当前关",
                        Action::SokobanChoose,
                        scale,
                        true,
                    );
                } else {
                    // Blank paper sampled from within each counter erases only its
                    // original digits; all displayed numbers below are live state.
                    sokoban_image(
                        stage,
                        art,
                        [1067., 348., 20., 61.],
                        [1077., 346., 140., 64.],
                        scale,
                    );
                    sokoban_image(
                        stage,
                        art,
                        [1254., 348., 20., 61.],
                        [1260., 346., 126., 64.],
                        scale,
                    );
                    sokoban_number(
                        stage,
                        art,
                        [1077., 342., 140., 68.],
                        format!("{}/{}", g.delivered(), g.crates.len()),
                        53.,
                        scale,
                    );
                    sokoban_number(
                        stage,
                        art,
                        [1260., 342., 126., 68.],
                        g.moves.to_string(),
                        if g.moves < 1000 { 53. } else { 37. },
                        scale,
                    );
                    if !g.history.is_empty() {
                        sokoban_hotspot(
                            stage,
                            Action::SokobanUndo,
                            [1053., 441., 168., 87.],
                            scale,
                        );
                    }
                    sokoban_hotspot(stage, Action::SokobanReset, [1238., 441., 166., 87.], scale);
                    if !g.won {
                        for (action, rect) in [
                            (Action::SokobanMove(0, -1), [1166., 551., 112., 99.]),
                            (Action::SokobanMove(-1, 0), [1054., 622., 115., 104.]),
                            (Action::SokobanMove(1, 0), [1280., 622., 114., 104.]),
                            (Action::SokobanMove(0, 1), [1168., 690., 109., 101.]),
                        ] {
                            sokoban_hotspot(stage, action, rect, scale);
                        }
                    } else {
                        let last = g.level + 1 == LEVELS.len();
                        sokoban_panel_button(
                            stage,
                            art,
                            [1055., 808., 341., 87.],
                            if last { "返回选关" } else { "下一关" },
                            Action::SokobanNext,
                            scale,
                            true,
                        );
                        sokoban_image(
                            stage,
                            art,
                            [1032., 286., 12., 14.],
                            [1060., 905., 332., 31.],
                            scale,
                        );
                        sokoban_text(
                            stage,
                            art,
                            [1060., 905., 332., 31.],
                            if last {
                                "全部完成，继续自由挑战"
                            } else {
                                "全部到位，可以继续"
                            },
                            22.,
                            scale,
                        );
                    }
                }
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    const ROUTES: [&str; 6] = [
        "DDRRRUULDRDDLDDLLUUURRDDRUUULUR",
        "RRUUUDDDLLUUUURRRLDDRDLDRRDRUUU",
        "UUUUULLDURRDDLLDLUDDLUUUDRRULLDLU",
        "DDDDDRRRUUULUURDDLDRURDLDDLLULUURRURDLDRRURD",
        "URRUUUDDRRURDLLLDLLUUUURRRR",
        "DDRRDRLURRDDURUUDDDDLLLL",
    ];
    fn solve(g: &mut SokobanGame) {
        for c in ROUTES[g.level].bytes() {
            let (dx, dy) = match c {
                b'U' => (0, -1),
                b'D' => (0, 1),
                b'L' => (-1, 0),
                _ => (1, 0),
            };
            assert!(
                g.move_player(dx, dy),
                "invalid move in level {}",
                g.level + 1
            );
        }
        assert!(g.won);
    }
    #[test]
    fn every_level_is_solvable_and_advances_only_after_completion() {
        let mut game = SokobanGame::new();
        for level in 0..6 {
            assert_eq!(game.level, level);
            game.next();
            assert_eq!(game.level, level);
            assert!(!game.choosing);
            solve(&mut game);
            assert_eq!(game.delivered(), 2);
            assert!(game.completed[level]);
            assert!(!game.move_player(0, 1));
            game.next();
        }
        assert!(game.choosing);
        assert_eq!(game.level, 5);
    }
    #[test]
    fn undo_restores_pushes_steps_and_win_state() {
        let mut game = SokobanGame::new();
        let initial = (game.player, game.crates.clone());
        solve(&mut game);
        assert!(game.undo());
        assert!(!game.won);
        assert_eq!(game.moves as usize, ROUTES[0].len() - 1);
        while game.undo() {}
        assert_eq!((game.player, game.crates.clone()), initial);
        assert_eq!(game.moves, 0);
    }
    #[test]
    fn blocked_moves_do_not_change_history_and_selection_is_bounded() {
        let mut game = SokobanGame::new();
        assert!(!game.move_player(1, 0));
        assert!(!game.move_player(2, 0));
        assert!(!game.undo());
        game.choosing = true;
        assert!(!game.move_player(0, 1));
        game.select_level(4);
        assert!(!game.choosing);
        assert_eq!(game.moves, 0);
        game.select_level(99);
        assert_eq!(game.level, 4);
        assert!(game.wall((-1, 3)));
        assert!(game.wall((8, 3)));
    }
}
