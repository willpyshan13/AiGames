//! Shared, uniformly scaled storybook canvases. Gameplay remains in its original models.
use super::*;

pub(super) struct StorybookArt {
    platform: Handle<Image>,
    assault: Handle<Image>,
    breakout: Handle<Image>,
    pub(super) breakout_atlas: Handle<Image>,
    actors: Handle<Image>,
    elements: Handle<Image>,
    restaurant: Handle<Image>,
    sudoku_nine: Handle<Image>,
    classic: Handle<Image>,
    literacy_map: Handle<Image>,
    literacy_lesson: Handle<Image>,
    literacy_words: Handle<Image>,
    math: Handle<Image>,
    math_map: Handle<Image>,
    math_atlas: Handle<Image>,
    piano: Handle<Image>,
    moon: Handle<Image>,
    moon_atlas: Handle<Image>,
    klotski: Handle<Image>,
    pieces: Handle<Image>,
    sudoku: Handle<Image>,
}
pub(super) fn load_storybook(assets: &AssetServer) -> StorybookArt {
    let load = |name: &str| {
        assets.load_with_settings(
            image_asset_path(name),
            |s: &mut bevy::image::ImageLoaderSettings| {
                s.sampler = bevy::image::ImageSampler::linear()
            },
        )
    };
    StorybookArt {
        platform: load("storybook/platform-plate.png"),
        assault: load("storybook/assault-plate.png"),
        breakout: load("storybook/breakout-plate.png"),
        breakout_atlas: load("storybook/breakout-atlas.png"),
        actors: load("storybook/actor-atlas.png"),
        elements: load("storybook/elements-plate.png"),
        restaurant: load("storybook/restaurant-frame.png"),
        sudoku_nine: load("storybook/sudoku-nine.png"),
        literacy_map: load("storybook/literacy-map.png"),
        literacy_lesson: load("storybook/literacy-lesson.png"),
        literacy_words: load("storybook/literacy-words.png"),
        math: load("storybook/math-plate.png"),
        math_map: load("storybook/math-map.png"),
        math_atlas: load("storybook/math-atlas.png"),
        piano: load("storybook/piano-plate.png"),
        moon: load("storybook/moon-plate.png"),
        moon_atlas: load("storybook/moon-atlas.png"),
        classic: load("storybook/classic2048-plate.png"),
        klotski: load("storybook/klotski-plate.png"),
        pieces: load("storybook/klotski-atlas.png"),
        sudoku: load("storybook/sudoku-plate.png"),
    }
}
pub(super) fn area(r: [f32; 4], k: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(r[0] * k),
        top: Val::Px(r[1] * k),
        width: Val::Px(r[2] * k),
        height: Val::Px(r[3] * k),
        ..default()
    }
}
pub(super) fn sample(
    p: &mut ChildSpawnerCommands,
    h: &Handle<Image>,
    r: [f32; 4],
    src: [f32; 4],
    k: f32,
) {
    p.spawn((
        ImageNode {
            rect: Some(Rect::new(src[0], src[1], src[2], src[3])),
            ..ImageNode::new(h.clone())
        },
        area(r, k),
    ));
}
pub(super) fn words(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    r: [f32; 4],
    text: impl Into<String>,
    size: f32,
    color: Color,
    k: f32,
) {
    p.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..area(r, k)
        },
        ZIndex(3),
    ))
    .with_children(|p| {
        p.spawn((
            Text::new(text),
            TextFont {
                font: art.font.clone(),
                font_size: size * k,
                ..default()
            },
            TextColor(color),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    });
}
pub(super) fn hit(p: &mut ChildSpawnerCommands, r: [f32; 4], action: Action, k: f32) {
    p.spawn((Button, WorldInputBlocker, action, area(r, k), ZIndex(8)));
}
pub(super) fn outline(p: &mut ChildSpawnerCommands, r: [f32; 4], k: f32) {
    p.spawn((
        Node {
            border: UiRect::all(Val::Px(4. * k)),
            ..area(r, k)
        },
        BorderColor(Color::srgb(0.45, 0.60, 0.35)),
        BorderRadius::all(Val::Px(18. * k)),
        ZIndex(4),
    ));
}
pub(super) fn canvas(
    commands: &mut Commands,
    h: &Handle<Image>,
    w: f32,
    hh: f32,
    design: Vec2,
    f: impl FnOnce(&mut ChildSpawnerCommands, f32),
) {
    let k = (w / design.x).min(hh / design.y);
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.84, 0.85, 0.72)),
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(design.x * k),
                height: Val::Px(design.y * k),
                flex_shrink: 0.,
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    ImageNode::new(h.clone()),
                    area([0., 0., design.x, design.y], k),
                ));
                f(p, k);
            });
        });
}
pub(super) fn storybook_ui(
    commands: &mut Commands,
    s: &Session,
    art: &Art,
    w: f32,
    h: f32,
) -> bool {
    let b = &art.storybook;
    match s.mode {
        GameMode::MushroomRun => platform_book(commands, s, art, w, h),
        GameMode::PixelAssault => assault_book(commands, s, art, w, h),
        GameMode::Breakout => breakout_book(commands, s, art, w, h),
        GameMode::ElementLab => elements_book(commands, s, art, w, h),
        GameMode::CrystalGarden => restaurant_book(commands, s, art, w, h),
        GameMode::Literacy => literacy_book(commands, s, art, w, h),
        GameMode::Math => math_book(commands, s, art, w, h),
        GameMode::MusicFactory => piano_book(commands, s, art, w, h),
        GameMode::MoonFestival => moon_book(commands, s, art, w, h),
        GameMode::Classic2048 => canvas(
            commands,
            &b.classic,
            w,
            h,
            Vec2::new(941., 1672.),
            |p, k| {
                let g = &s.classic_2048;
                words(
                    p,
                    art,
                    [210., 338., 200., 65.],
                    g.score.to_string(),
                    55.,
                    INK,
                    k,
                );
                words(
                    p,
                    art,
                    [535., 338., 200., 65.],
                    g.best.to_string(),
                    55.,
                    INK,
                    k,
                );
                p.spawn((ClassicBoard, area([142., 501., 660., 660.], k)));
                for i in 0..16 {
                    let v = g.cells[i];
                    let r = [
                        145. + (i % 4) as f32 * 167.,
                        507. + (i / 4) as f32 * 167.,
                        153.,
                        153.,
                    ];
                    let source = match v {
                        0 => [145., 674., 298., 827.],
                        2 => [145., 507., 298., 660.],
                        4 => [478., 507., 631., 660.],
                        _ => [645., 507., 798., 660.],
                    };
                    sample(p, &b.classic, r, source, k);
                    if v > 0 {
                        words(
                            p,
                            art,
                            r,
                            v.to_string(),
                            if v < 100 {
                                76.
                            } else if v < 1000 {
                                64.
                            } else {
                                52.
                            },
                            INK,
                            k,
                        );
                    }
                }
                hit(p, [70., 80., 100., 110.], Action::Library, k);
                hit(p, [320., 1480., 300., 110.], Action::ClassicReset, k);
                for (i, (dx, dy)) in [(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter().enumerate() {
                    hit(
                        p,
                        [145. + i as f32 * 167., 1330., 153., 120.],
                        Action::ClassicMove(dx, dy),
                        k,
                    );
                }
                if g.won || g.over {
                    sample(
                        p,
                        &b.classic,
                        [130., 1225., 680., 70.],
                        [170., 705., 280., 780.],
                        k,
                    );
                    words(
                        p,
                        art,
                        [130., 1225., 680., 70.],
                        if g.over {
                            "没有空位了，再试一次吧"
                        } else {
                            "合成 2048！继续挑战高分"
                        },
                        30.,
                        INK,
                        k,
                    );
                }
            },
        ),
        GameMode::Klotski => canvas(
            commands,
            &b.klotski,
            w,
            h,
            Vec2::new(941., 1672.),
            |p, k| {
                let src = [
                    [318., 382., 620., 651.],
                    [145., 375., 300., 657.],
                    [639., 375., 796., 657.],
                    [312., 665., 628., 827.],
                    [145., 665., 300., 987.],
                    [639., 665., 796., 987.],
                    [312., 835., 465., 987.],
                    [476., 835., 628., 987.],
                ];
                for (i, piece) in s.klotski.pieces.iter().enumerate() {
                    let r = [
                        145. + piece.x as f32 * 163.,
                        375. + piece.y as f32 * 155.,
                        piece.w as f32 * 163. - 7.,
                        piece.h as f32 * 155. - 7.,
                    ];
                    sample(p, &b.pieces, r, src[i], k);
                    hit(p, r, Action::KlotskiSelect(i), k);
                    if i == s.klotski.selected {
                        outline(p, r, k);
                    }
                }
                words(
                    p,
                    art,
                    [203., 1220., 535., 65.],
                    if s.klotski.won {
                        "成功出阵！"
                    } else {
                        "先选棋子，再按方向移动"
                    },
                    30.,
                    INK,
                    k,
                );
                hit(p, [45., 65., 105., 105.], Action::Library, k);
                hit(p, [400., 1298., 138., 110.], Action::KlotskiMove(0, -1), k);
                hit(p, [294., 1382., 142., 110.], Action::KlotskiMove(-1, 0), k);
                hit(p, [502., 1382., 142., 110.], Action::KlotskiMove(1, 0), k);
                hit(p, [402., 1470., 138., 110.], Action::KlotskiMove(0, 1), k);
                hit(p, [730., 1440., 120., 110.], Action::KlotskiReset, k);
            },
        ),
        GameMode::Sudoku => canvas(
            commands,
            if s.sudoku.size == 9 {
                &b.sudoku_nine
            } else {
                &b.sudoku
            },
            w,
            h,
            Vec2::new(941., 1672.),
            |p, k| {
                let g = &s.sudoku;
                let n = g.size;
                let cell = 644. / n as f32;
                let boxn = if n == 4 { 2 } else { 3 };
                sample(
                    p,
                    &b.sudoku,
                    [149., 422., 644., 661.],
                    [163., 435., 290., 570.],
                    k,
                );
                for y in 0..n {
                    for x in 0..n {
                        let i = y * n + x;
                        let v = g.cells[i];
                        let r = [
                            149. + x as f32 * cell,
                            422. + y as f32 * 661. / n as f32,
                            cell,
                            661. / n as f32,
                        ];
                        let wrong = g.is_wrong(i);
                        if g.selected == Some(i) || wrong {
                            p.spawn((
                                area(r, k),
                                BackgroundColor(if wrong {
                                    Color::srgba(0.85, 0.40, 0.28, 0.25)
                                } else {
                                    Color::srgba(0.5, 0.7, 0.45, 0.25)
                                }),
                            ));
                        }
                        if v > 0 {
                            words(
                                p,
                                art,
                                r,
                                v.to_string(),
                                if n == 4 { 78. } else { 43. },
                                if wrong {
                                    Color::srgb(0.65, 0.25, 0.2)
                                } else if g.is_given(i) {
                                    INK
                                } else {
                                    TEAL
                                },
                                k,
                            );
                        }
                        hit(p, r, Action::SudokuSelect(i), k);
                    }
                }
                for i in 0..=n {
                    let thick = if i % boxn == 0 { 4. } else { 1.5 };
                    p.spawn((
                        area([149. + i as f32 * cell, 422., thick, 661.], k),
                        BackgroundColor(Color::srgb(0.50, 0.43, 0.32)),
                    ));
                    p.spawn((
                        area([149., 422. + i as f32 * 661. / n as f32, 644., thick], k),
                        BackgroundColor(Color::srgb(0.50, 0.43, 0.32)),
                    ));
                }
                let filled = g.cells[..n * n].iter().filter(|v| **v > 0).count();
                words(
                    p,
                    art,
                    [211., 1145., 518., 68.],
                    if g.won {
                        "全部填对了！".into()
                    } else {
                        format!("已填 {filled}/{} 格 · 第 {} 题", n * n, g.variant + 1)
                    },
                    30.,
                    INK,
                    k,
                );
                hit(p, [43., 80., 102., 110.], Action::Library, k);
                hit(p, [152., 270., 325., 106.], Action::SudokuSize(4), k);
                hit(p, [486., 280., 309., 91.], Action::SudokuSize(9), k);
                outline(
                    p,
                    if n == 4 {
                        [152., 270., 325., 106.]
                    } else {
                        [486., 280., 309., 91.]
                    },
                    k,
                );
                if n == 4 {
                    for i in 0..4 {
                        hit(
                            p,
                            [177. + i as f32 * 151., 1240., 130., 135.],
                            Action::SudokuDigit((i + 1) as u8),
                            k,
                        );
                    }
                } else {
                    for i in 0..9 {
                        let r = if i < 5 {
                            [171. + i as f32 * 124., 1227., 110., 80.]
                        } else {
                            [224. + (i - 5) as f32 * 126., 1315., 110., 80.]
                        };
                        hit(p, r, Action::SudokuDigit((i + 1) as u8), k);
                    }
                }
                hit(p, [78., 1415., 240., 110.], Action::SudokuClear, k);
                hit(p, [339., 1415., 261., 110.], Action::SudokuHint, k);
                hit(p, [617., 1415., 248., 110.], Action::SudokuNew, k);
                hit(p, [375., 1545., 190., 70.], Action::SudokuReset, k);
            },
        ),
        _ => return false,
    }
    true
}

fn paper(p: &mut ChildSpawnerCommands, art: &Art, r: [f32; 4], k: f32) {
    sample(
        p,
        &art.storybook.literacy_lesson,
        r,
        [340., 270., 700., 600.],
        k,
    );
}
fn book_key(
    p: &mut ChildSpawnerCommands,
    art: &Art,
    r: [f32; 4],
    text: impl Into<String>,
    action: Action,
    k: f32,
) {
    sample(
        p,
        &art.storybook.literacy_lesson,
        r,
        [1175., 313., 1420., 492.],
        k,
    );
    words(p, art, r, text, 34., INK, k);
    hit(p, r, action, k);
}
fn literacy_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let b = &art.storybook;
    let g = &s.literacy;
    if g.map {
        canvas(
            commands,
            &b.literacy_map,
            w,
            h,
            Vec2::new(1672., 941.),
            |p, k| {
                words(
                    p,
                    art,
                    [680., 63., 310., 61.],
                    format!(
                        "已完成 {}/3",
                        g.progress.completed.iter().filter(|&&v| v).count()
                    ),
                    36.,
                    INK,
                    k,
                );
                for (i, r) in [
                    [130., 185., 460., 430.],
                    [610., 190., 530., 480.],
                    [1160., 250., 460., 400.],
                ]
                .into_iter()
                .enumerate()
                {
                    hit(p, r, Action::LiteracyStart(i), k);
                    if g.progress.completed[i] {
                        words(
                            p,
                            art,
                            [r[0] + 100., r[1] + 30., 230., 60.],
                            "✓ 已完成",
                            32.,
                            TEAL,
                            k,
                        );
                    }
                }
                hit(p, [1220., 60., 198., 70.], Action::Help, k);
                hit(p, [1430., 60., 200., 70.], Action::Library, k);
            },
        );
        return;
    }
    canvas(
        commands,
        &b.literacy_lesson,
        w,
        h,
        Vec2::new(1672., 941.),
        |p, k| {
            words(
                p,
                art,
                [590., 27., 505., 65.],
                format!(
                    "{} · 第 {}/4 个字",
                    crate::literacy::UNITS[g.unit],
                    g.card + 1
                ),
                38.,
                INK,
                k,
            );
            hit(p, [40., 25., 230., 65.], Action::LiteracyMap, k);
            hit(p, [1240., 25., 185., 65.], Action::Help, k);
            hit(p, [1440., 25., 190., 65.], Action::Library, k);
            words(
                p,
                art,
                [1240., 795., 180., 65.],
                if g.finished {
                    "返回地图"
                } else {
                    "下一步"
                },
                if g.finished { 38. } else { 44. },
                Color::WHITE,
                k,
            );
            for (i, (x, name)) in [
                (356., "认识汉字"),
                (610., "看图选字"),
                (888., "拼音配对"),
                (1150., "组词填空"),
            ]
            .into_iter()
            .enumerate()
            {
                let r = [x, 159., if i == 1 { 214. } else { 199. }, 53.];
                if i == g.stage {
                    outline(p, r, k);
                }
                words(
                    p,
                    art,
                    r,
                    name,
                    30.,
                    if i == g.stage { TEAL } else { INK },
                    k,
                );
            }
            let index = g.word_index();
            let x = (index % 4) as f32 * 362.;
            let y = (index / 4) as f32 * 362.;
            sample(
                p,
                &b.literacy_words,
                [280., 239., 548., 570.],
                [x, y, x + 362., y + 362.],
                k,
            );
            let word = g.word();
            if g.finished {
                paper(p, art, [890., 240., 548., 530.], k);
                words(
                    p,
                    art,
                    [900., 290., 520., 90.],
                    "这座小岛学完啦！",
                    46.,
                    TEAL,
                    k,
                );
                words(
                    p,
                    art,
                    [915., 410., 500., 160.],
                    format!(
                        "{}\n已收集 {} XP",
                        crate::literacy::UNITS[g.unit],
                        g.progress.xp
                    ),
                    38.,
                    INK,
                    k,
                );
                words(
                    p,
                    art,
                    [945., 610., 440., 95.],
                    "每天学一点，明天再来玩",
                    30.,
                    TEAL,
                    k,
                );
                hit(p, [1215., 779., 280., 90.], Action::LiteracyMap, k);
                return;
            }
            if g.stage == 0 {
                paper(p, art, [890., 240., 548., 530.], k);
                words(p, art, [940., 260., 450., 75.], word.pinyin, 52., TEAL, k);
                words(
                    p,
                    art,
                    [950., 340., 430., 200.],
                    word.character,
                    170.,
                    INK,
                    k,
                );
                words(p, art, [920., 545., 500., 100.], word.meaning, 36., INK, k);
                words(
                    p,
                    art,
                    [920., 650., 500., 65.],
                    format!("一起读：{}", word.phrase),
                    36.,
                    TEAL,
                    k,
                );
                #[cfg(target_os = "macos")]
                {
                    hit(p, [280., 239., 548., 570.], Action::LiteracySpeak, k);
                }
            } else {
                let prompt = match g.stage {
                    1 => "看看图片，选出对应的汉字".to_owned(),
                    2 => format!("“{}” 对应哪个汉字？", word.pinyin),
                    _ => word.blank.to_owned(),
                };
                words(p, art, [896., 239., 540., 64.], prompt, 40., INK, k);
                for (i, index) in g.options().into_iter().enumerate() {
                    let r = [
                        900. + (i % 2) as f32 * 275.,
                        313. + (i / 2) as f32 * 193.,
                        254.,
                        177.,
                    ];
                    if g.selected == Some(index) {
                        let color = if g.correct {
                            TEAL
                        } else {
                            Color::srgb(0.72, 0.3, 0.22)
                        };
                        p.spawn((
                            Node {
                                border: UiRect::all(Val::Px(3. * k)),
                                ..area(r, k)
                            },
                            BackgroundColor(if g.correct {
                                Color::srgba(0.55, 0.78, 0.30, 0.2)
                            } else {
                                Color::srgba(0.9, 0.5, 0.35, 0.15)
                            }),
                            BorderColor(color),
                            BorderRadius::all(Val::Px(25. * k)),
                        ));
                        if g.correct {
                            p.spawn((
                                area([r[0] + r[2] - 52., r[1] + 13., 38., 38.], k),
                                BackgroundColor(TEAL),
                                BorderRadius::all(Val::Percent(50.)),
                            ));
                            words(
                                p,
                                art,
                                [r[0] + r[2] - 52., r[1] + 13., 38., 38.],
                                "✓",
                                27.,
                                Color::WHITE,
                                k,
                            );
                        }
                    }
                    words(
                        p,
                        art,
                        r,
                        crate::literacy::WORDS[index].character,
                        120.,
                        if g.selected == Some(index) && g.correct {
                            TEAL
                        } else {
                            INK
                        },
                        k,
                    );
                    hit(p, r, Action::LiteracySelect(index), k);
                }
            }
            let feedback = if g.checked {
                if g.correct {
                    format!("答对啦！「{}」{}", word.character, word.meaning)
                } else {
                    "再看看图片，换一个试试吧".into()
                }
            } else if g.reviewing {
                "温习一下，再试一次吧".into()
            } else {
                "不计时，慢慢想也没关系".into()
            };
            words(
                p,
                art,
                [966., 699., 458., 68.],
                feedback,
                28.,
                if g.checked && !g.correct {
                    Color::srgb(0.65, 0.3, 0.2)
                } else {
                    TEAL
                },
                k,
            );
            if g.stage == 0 || g.checked {
                hit(p, [1215., 779., 280., 90.], Action::LiteracyNext, k);
            } else {
                p.spawn((
                    area([1215., 779., 280., 90.], k),
                    BackgroundColor(Color::srgba(1., 1., 1., 0.3)),
                ));
            }
        },
    );
}
fn math_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let b = &art.storybook;
    let g = &s.math;
    canvas(
        commands,
        if g.choosing_level {
            &b.math_map
        } else {
            &b.math
        },
        w,
        h,
        Vec2::new(1672., 941.),
        |p, k| {
            words(
                p,
                art,
                [725., 34., 305., 75.],
                if g.choosing_level {
                    "数学学习地图".into()
                } else {
                    format!("{} · {}/8题", crate::math::LEVELS[g.level], g.index + 1)
                },
                36.,
                INK,
                k,
            );
            hit(p, [1140., 45., 240., 70.], Action::MathLevels, k);
            hit(p, [1410., 45., 220., 70.], Action::Library, k);
            if g.choosing_level {
                words(p, art, [345., 210., 450., 75.], "从这里开始", 42., INK, k);
                words(
                    p,
                    art,
                    [945., 230., 470., 75.],
                    "准备好挑战了吗？",
                    42.,
                    INK,
                    k,
                );
                for i in 0..5 {
                    let r = if i < 3 {
                        [355., 300. + i as f32 * 140., 420., 118.]
                    } else {
                        [960., 360. + (i - 3) as f32 * 170., 440., 130.]
                    };
                    book_key(
                        p,
                        art,
                        r,
                        format!("{} · 已完成 {}/8", crate::math::LEVELS[i], g.stars[i]),
                        Action::MathLevel(i),
                        k,
                    );
                }
                words(
                    p,
                    art,
                    [355., 732., 235., 111.],
                    "选择难度，\n慢慢练习吧",
                    29.,
                    INK,
                    k,
                );
                hit(p, [920., 786., 282., 106.], Action::MathResume, k);
                hit(p, [1250., 786., 340., 106.], Action::MathResume, k);
                return;
            }
            let q = g.question();
            words(p, art, [920., 203., 515., 106.], q.prompt, 34., INK, k);
            if g.level == 0 && g.index == 0 {
                sample(
                    p,
                    &b.math_atlas,
                    [315., 214., 523., 496.],
                    [315., 214., 838., 710.],
                    k,
                );
            } else {
                words(
                    p,
                    art,
                    [360., 240., 440., 160.],
                    format!("{} · 第{}题", crate::math::LEVELS[g.level], g.index + 1),
                    44.,
                    TEAL,
                    k,
                );
                words(
                    p,
                    art,
                    [355., 420., 450., 150.],
                    if g.solved {
                        q.explanation
                    } else if g.hinted {
                        q.hint
                    } else {
                        "读一读题目，\n用自己的方法算一算。"
                    },
                    36.,
                    INK,
                    k,
                );
            }
            if s.math_tools {
                paper(p, art, [330., 210., 510., 500.], k);
                words(
                    p,
                    art,
                    [365., 225., 435., 65.],
                    format!("摆一摆：已放 {} 个", g.placed),
                    36.,
                    TEAL,
                    k,
                );
                for i in 0..g.placed.min(30) {
                    let x = 380. + (i % 6) as f32 * 65.;
                    let y = 310. + (i / 6) as f32 * 42.;
                    p.spawn((
                        area([x, y, 30., 30.], k),
                        BackgroundColor(Color::srgb(0.86, 0.61, 0.28)),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                }
                for (i, (t, d)) in [("−1", -1), ("+1", 1), ("+10", 10)].into_iter().enumerate() {
                    book_key(
                        p,
                        art,
                        [355. + i as f32 * 150., 560., 135., 65.],
                        t,
                        Action::MathPlace(d),
                        k,
                    );
                }
                book_key(
                    p,
                    art,
                    [410., 630., 350., 65.],
                    "检查数量",
                    Action::MathCheck,
                    k,
                );
            }
            hit(p, [60., 650., 260., 260.], Action::MathTools, k);
            book_key(
                p,
                art,
                [350., 860., 260., 58.],
                if s.math_tools {
                    "收起实物"
                } else {
                    "摆实物"
                },
                Action::MathTools,
                k,
            );
            for (i, value) in g.choices().into_iter().enumerate() {
                let r = [
                    920. + (i % 2) as f32 * 255.,
                    323. + (i / 2) as f32 * 186.,
                    238.,
                    170.,
                ];
                words(
                    p,
                    art,
                    r,
                    value.to_string(),
                    110.,
                    if g.solved && value == q.answer {
                        TEAL
                    } else {
                        INK
                    },
                    k,
                );
                hit(p, r, Action::MathAnswer(value), k);
                if (g.solved && value == q.answer) || g.wrong == Some(value) {
                    outline(p, r, k);
                }
            }
            words(
                p,
                art,
                [355., 732., 235., 111.],
                if g.solved {
                    "答对啦！\n继续出发吧"
                } else if g.wrong.is_some() {
                    "再想一想，\n你可以的"
                } else if g.hinted {
                    "提示在书里，\n慢慢来"
                } else {
                    "开动脑筋，\n一起数一数"
                },
                29.,
                INK,
                k,
            );
            hit(p, [920., 786., 282., 106.], Action::MathHint, k);
            if g.solved {
                hit(p, [1250., 786., 340., 106.], Action::MathNext, k);
            } else {
                p.spawn((
                    area([1250., 786., 340., 106.], k),
                    BackgroundColor(Color::srgba(1., 1., 1., 0.3)),
                ));
            }
            if g.hinted && !g.solved {
                paper(p, art, [355., 240., 450., 170.], k);
                words(p, art, [355., 240., 450., 170.], q.hint, 34., TEAL, k);
            }
        },
    );
}
fn piano_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let b = &art.storybook;
    let g = &s.piano;
    canvas(commands, &b.piano, w, h, Vec2::new(1672., 941.), |p, k| {
        words(
            p,
            art,
            [690., 35., 430., 60.],
            format!(
                "第{}个音符 · {}连击",
                g.step % PIANO_NOTES.len() + 1,
                g.combo
            ),
            36.,
            INK,
            k,
        );
        words(
            p,
            art,
            [800., 114., 210., 40.],
            format!("得分 {}", g.score),
            28.,
            INK,
            k,
        );
        words(p, art, [1460., 55., 150., 70.], "游戏库", 38., INK, k);
        hit(p, [1400., 48., 220., 80.], Action::Library, k);
        words(p, art, [580., 838., 560., 68.], &g.feedback, 28., INK, k);
        let note = g.target();
        p.spawn(Node {
            overflow: Overflow::clip(),
            ..area([337., 209., 1100., 391.], k)
        })
        .with_children(|stage| {
            stage
                .spawn((
                    PianoNote,
                    Button,
                    Action::PianoKey(note),
                    Node {
                        left: Val::Percent(note as f32 * 100. / 7. + 1.2),
                        top: Val::Percent(8. + g.fall / 3.2 * 66.),
                        width: Val::Percent(100. / 7. - 2.4),
                        height: Val::Px(80. * k),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.62, 0.85, 0.54)),
                    BorderRadius::all(Val::Px(18. * k)),
                ))
                .with_children(|n| {
                    words(
                        n,
                        art,
                        [0., 0., 138., 80.],
                        ["C", "D", "E", "F", "G", "A", "B"][note],
                        50.,
                        INK,
                        k,
                    )
                });
        });
        for key in 0..7 {
            let r = [300. + key as f32 * 162., 636., 158., 185.];
            hit(p, r, Action::PianoKey(key), k);
            if key == note || (g.last_key == Some(key) && g.flash > 0.) {
                outline(p, r, k);
            }
        }
        book_key(
            p,
            art,
            [1190., 48., 175., 75.],
            "重来",
            Action::PianoReset,
            k,
        );
    });
}
fn moon_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let b = &art.storybook;
    let g = &s.moon;
    canvas(commands, &b.moon, w, h, Vec2::new(941., 1672.), |p, k| {
        words(
            p,
            art,
            [203., 380., 205., 70.],
            g.score.to_string(),
            66.,
            INK,
            k,
        );
        words(
            p,
            art,
            [540., 380., 205., 70.],
            g.rolls_left.to_string(),
            66.,
            INK,
            k,
        );
        hit(p, [20., 20., 110., 100.], Action::Library, k);
        for (i, value) in g.dice.iter().enumerate() {
            let r = [
                282. + (i % 3) as f32 * 142.,
                678. + (i / 3) as f32 * 142.,
                110.,
                116.,
            ];
            sample(p, &b.moon_atlas, r, [282., 678., 392., 794.], k);
            let dots: &[(f32, f32)] = match value {
                1 => &[(0.5, 0.5)],
                2 => &[(0.25, 0.25), (0.75, 0.75)],
                3 => &[(0.25, 0.25), (0.5, 0.5), (0.75, 0.75)],
                4 => &[(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)],
                5 => &[
                    (0.25, 0.25),
                    (0.75, 0.25),
                    (0.5, 0.5),
                    (0.25, 0.75),
                    (0.75, 0.75),
                ],
                _ => &[
                    (0.25, 0.2),
                    (0.75, 0.2),
                    (0.25, 0.5),
                    (0.75, 0.5),
                    (0.25, 0.8),
                    (0.75, 0.8),
                ],
            };
            for (x, y) in dots {
                p.spawn((
                    area(
                        [
                            r[0] + 13. + x * 78. - 10.,
                            r[1] + 10. + y * 76. - 10.,
                            20.,
                            20.,
                        ],
                        k,
                    ),
                    BackgroundColor(Color::srgb(0.65, 0.08, 0.05)),
                    BorderRadius::all(Val::Percent(50.)),
                ));
            }
        }
        // The feedback scroll stays in front of the porcelain bowl.
        let message = g
            .feedback
            .chars()
            .collect::<Vec<_>>()
            .chunks(18)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        words(p, art, [270., 1100., 400., 80.], message, 22., INK, k);
        hit(p, [220., 1180., 500., 165.], Action::MoonShake, k);
        hit(p, [300., 1510., 340., 100.], Action::MoonReset, k);
        for i in 0..3 {
            words(
                p,
                art,
                [185. + i as f32 * 285., 1410., 110., 66.],
                g.collection[i].to_string(),
                52.,
                INK,
                k,
            );
        }
    });
}

fn actor(p: &mut ChildSpawnerCommands, art: &Art, index: usize, r: [f32; 4], k: f32, flip: bool) {
    // The atlas uses equal square cells with generous transparent gutters.
    let side = 443.5;
    let x = (index % 4) as f32 * side;
    let y = (index / 4) as f32 * side;
    p.spawn((
        ImageNode {
            rect: Some(Rect::new(x, y, x + side, y + side)),
            flip_x: flip,
            ..ImageNode::new(art.storybook.actors.clone())
        },
        area(r, k),
    ));
}
fn platform_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let g = &s.platform;
    canvas(
        commands,
        &art.storybook.platform,
        w,
        h,
        Vec2::new(1672., 941.),
        |p, k| {
            words(
                p,
                art,
                [550., 60., 615., 75.],
                format!(
                    "路程 {}m · 金币 {} · 爱心 {}",
                    g.distance.min(300.).round(),
                    g.coins,
                    g.lives
                ),
                38.,
                INK,
                k,
            );
            words(p, art, [640., 788., 540., 75.], &g.feedback, 30., INK, k);
            hit(p, [1275., 55., 172., 75.], Action::PlatformReset, k);
            hit(p, [1460., 55., 194., 75.], Action::Library, k);
            p.spawn(Node {
                overflow: Overflow::clip(),
                ..area([105., 205., 1490., 500.], k)
            })
            .with_children(|stage| {
                for thing in &g.things {
                    let idx = match thing.kind {
                        PlatformThingKind::Coin => 1,
                        PlatformThingKind::Walker => 2,
                        PlatformThingKind::Block => 3,
                    };
                    let size = if idx == 1 { 60. } else { 86. };
                    actor(
                        stage,
                        art,
                        idx,
                        [
                            (thing.x - g.camera_x()) * 1490.,
                            396. - thing.y * 300. - size,
                            size,
                            size,
                        ],
                        k,
                        false,
                    );
                }
                let bob = if g.walk_timer > 0. {
                    g.walk_phase.sin().abs() * 4.
                } else {
                    0.
                };
                actor(
                    stage,
                    art,
                    0,
                    [
                        g.player_x * 1490. - 10.,
                        396. - g.player_y * 300. - 110. - bob,
                        110.,
                        110.,
                    ],
                    k,
                    g.walk_direction < 0,
                );
                stage.spawn((
                    area([1415., 208., 6., 190.], k),
                    BackgroundColor(Color::srgb(0.5, 0.3, 0.15)),
                ));
                stage.spawn((
                    area([1420., 208., 55., 40.], k),
                    BackgroundColor(Color::srgb(0.84, 0.3, 0.2)),
                    BorderRadius::all(Val::Px(5. * k)),
                ));
                if g.failed || g.finished {
                    book_key(
                        stage,
                        art,
                        [370., 110., 730., 150.],
                        if g.finished {
                            "到达终点！再玩一次"
                        } else {
                            "再试一次，你可以的！"
                        },
                        Action::PlatformReset,
                        k,
                    );
                }
            });
            hit(p, [255., 767., 135., 120.], Action::PlatformLeft, k);
            hit(p, [403., 767., 135., 120.], Action::PlatformRight, k);
            hit(p, [1295., 767., 320., 120.], Action::PlatformJump, k);
        },
    );
}
fn assault_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let g = &s.assault;
    canvas(
        commands,
        &art.storybook.assault,
        w,
        h,
        Vec2::new(1672., 941.),
        |p, k| {
            words(
                p,
                art,
                [580., 50., 510., 85.],
                format!("第{}关 · 得分{} · 护盾{}", g.stage + 1, g.score, g.lives),
                36.,
                INK,
                k,
            );
            words(p, art, [650., 790., 510., 85.], &g.feedback, 25., INK, k);
            hit(p, [1260., 55., 174., 75.], Action::AssaultReset, k);
            hit(p, [1455., 55., 190., 75.], Action::Library, k);
            p.spawn(Node {
                overflow: Overflow::clip(),
                ..area([120., 207., 1430., 539.], k)
            })
            .with_children(|stage| {
                let lane_y = |lane: i8| match lane {
                    0 => 539. * 0.25,
                    1 => 539. * 0.48,
                    _ => 539. * 0.67,
                };
                for (left, top, width) in [(35., 369., 24.), (71., 245., 19.), (4., 245., 20.)] {
                    let x = ((left - g.camera_x * 26.).rem_euclid(130.) - 15.) * 14.3;
                    sample(
                        stage,
                        &art.storybook.actors,
                        [x, top, width * 14.3, 75.],
                        [1333., 598., 1764., 785.],
                        k,
                    );
                }
                let bob = if g.moving { g.run_phase.sin() * 3. } else { 0. };
                actor(
                    stage,
                    art,
                    4,
                    [
                        (g.player_x - g.camera_x) * 1430.,
                        lane_y(g.lane) - g.jump * 183. + bob,
                        118.,
                        118.,
                    ],
                    k,
                    false,
                );
                for enemy in &g.enemies {
                    actor(
                        stage,
                        art,
                        if enemy.heavy { 6 } else { 5 },
                        [
                            (enemy.x - g.camera_x) * 1430.,
                            lane_y(enemy.lane),
                            if enemy.heavy { 112. } else { 100. },
                            if enemy.heavy { 112. } else { 100. },
                        ],
                        k,
                        false,
                    );
                }
                for b in &g.bullets {
                    stage.spawn((
                        area(
                            [(b.x - g.camera_x) * 1430., lane_y(b.lane) + 68., 30., 10.],
                            k,
                        ),
                        BackgroundColor(Color::srgb(1., 0.8, 0.3)),
                        BorderRadius::all(Val::Px(8. * k)),
                    ));
                }
                for spark in &g.sparks {
                    let size = if spark.large { 70. } else { 36. };
                    stage.spawn((
                        area(
                            [
                                (spark.x - g.camera_x) * 1430.,
                                lane_y(spark.lane) + 45.,
                                size,
                                size,
                            ],
                            k,
                        ),
                        BackgroundColor(Color::srgba(1., 0.8, 0.2, (spark.life * 3.).min(1.))),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                }
                if g.failed || g.finished {
                    book_key(
                        stage,
                        art,
                        [350., 130., 750., 150.],
                        if g.finished {
                            "三关通关！再玩一次"
                        } else {
                            "护盾用完了，重新出发"
                        },
                        Action::AssaultReset,
                        k,
                    );
                }
            });
            hit(p, [245., 790., 126., 100.], Action::AssaultLeft, k);
            hit(p, [372., 790., 88., 100.], Action::AssaultJump, k);
            hit(p, [463., 790., 126., 100.], Action::AssaultRight, k);
            hit(p, [1245., 785., 350., 108.], Action::AssaultFire, k);
        },
    );
}
fn breakout_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    canvas(
        commands,
        &art.storybook.breakout,
        w,
        h,
        Vec2::new(1672., 941.),
        |p, k| {
            words(
                p,
                art,
                [590., 40., 500., 70.],
                format!(
                    "第{}关 · 得分{} · 爱心{}",
                    s.breakout.stage, s.breakout.score, s.breakout.lives
                ),
                36.,
                INK,
                k,
            );
            hit(p, [1240., 30., 175., 72.], Action::BreakoutReset, k);
            hit(p, [1435., 30., 195., 72.], Action::Library, k);
            words(
                p,
                art,
                [440., 745., 790., 60.],
                if s.breakout.won {
                    "全部通关啦！"
                } else if s.breakout.over {
                    "爱心用完了，点重来再试一次"
                } else if s.breakout.paused {
                    "已暂停，点暂停按钮继续"
                } else {
                    "左右拖动挡板，接住小球"
                },
                32.,
                INK,
                k,
            );
            breakout::storybook_board(p, s, art, k);
            hit(p, [425., 825., 252., 83.], Action::BreakoutLaunch, k);
            hit(p, [713., 825., 247., 83.], Action::BreakoutToggle, k);
            hit(p, [997., 825., 247., 83.], Action::BreakoutReset, k);
        },
    );
}
fn elements_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let g = &s.elements;
    let m = g.mission();
    canvas(
        commands,
        &art.storybook.elements,
        w,
        h,
        Vec2::new(1672., 941.),
        |p, k| {
            words(
                p,
                art,
                [600., 23., 470., 70.],
                format!("小实验 {} · {}", g.stage + 1, m.title),
                38.,
                INK,
                k,
            );
            hit(p, [1238., 23., 185., 70.], Action::Help, k);
            hit(p, [1440., 23., 195., 70.], Action::Library, k);
            for e in &elements::ELEMENTS {
                let r = [
                    216. + e.column as f32 * 54.5,
                    257. + e.period as f32 * 76.,
                    51.,
                    71.,
                ];
                let tint = match e.family {
                    1 => Color::srgb(0.94, 0.74, 0.72),
                    2 | 3 => Color::srgb(0.97, 0.84, 0.57),
                    4 | 6 | 7 => Color::srgb(0.75, 0.86, 0.64),
                    _ => Color::srgb(0.65, 0.85, 0.87),
                };
                p.spawn((
                    area(r, k),
                    BackgroundColor(tint),
                    BorderRadius::all(Val::Px(8. * k)),
                    BorderColor(Color::srgb(0.65, 0.60, 0.40)),
                ));
                words(
                    p,
                    art,
                    [r[0], r[1] + 2., r[2], 16.],
                    e.number.to_string(),
                    14.,
                    INK,
                    k,
                );
                words(p, art, [r[0], r[1] + 15., r[2], 33.], e.symbol, 28., INK, k);
                words(p, art, [r[0], r[1] + 48., r[2], 22.], e.name, 20., INK, k);
                hit(p, r, Action::ElementPick(e.number), k);
                if g.last_seen == Some(e.number) {
                    outline(p, r, k);
                }
            }
            let number = g.last_seen.unwrap_or(m.atoms[0]);
            let e = &elements::ELEMENTS[number as usize - 1];
            if g.observing {
                words(
                    p,
                    art,
                    [1270., 235., 320., 75.],
                    format!("{} · {}", e.symbol, e.name),
                    54.,
                    TEAL,
                    k,
                );
                let shells = elements::electron_shells(number)
                    .split(',')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                let center = Vec2::new(1430., 450.);
                for (i, count) in shells.iter().enumerate() {
                    let radius = 36. + i as f32 * 19.;
                    p.spawn((
                        Node {
                            border: UiRect::all(Val::Px(2. * k)),
                            ..area(
                                [
                                    center.x - radius,
                                    center.y - radius,
                                    radius * 2.,
                                    radius * 2.,
                                ],
                                k,
                            )
                        },
                        BorderColor(Color::srgb(0.50, 0.65, 0.65)),
                        BorderRadius::all(Val::Percent(50.)),
                    ));
                    for j in 0..*count {
                        let angle =
                            j as f32 / *count as f32 * std::f32::consts::TAU + i as f32 * 0.4;
                        let point = center + Vec2::new(angle.cos(), angle.sin()) * radius;
                        p.spawn((
                            area([point.x - 5., point.y - 5., 10., 10.], k),
                            BackgroundColor(Color::srgb(0.3, 0.6, 0.8)),
                            BorderRadius::all(Val::Percent(50.)),
                        ));
                    }
                }
                p.spawn((
                    area([center.x - 22., center.y - 22., 44., 44.], k),
                    BackgroundColor(Color::srgb(0.83, 0.45, 0.32)),
                    BorderRadius::all(Val::Percent(50.)),
                ));
                words(
                    p,
                    art,
                    [center.x - 22., center.y - 22., 44., 44.],
                    format!("+{number}"),
                    19.,
                    Color::WHITE,
                    k,
                );
                words(
                    p,
                    art,
                    [1270., 585., 320., 75.],
                    format!(
                        "电子分布：{}
原子结构示意",
                        elements::electron_shells(number)
                    ),
                    25.,
                    INK,
                    k,
                );
            } else {
                words(p, art, [1270., 240., 320., 130.], e.symbol, 100., TEAL, k);
                words(p, art, [1270., 365., 320., 65.], e.name, 42., INK, k);
                words(
                    p,
                    art,
                    [1270., 440., 320., 120.],
                    format!(
                        "原子序数 {}\n{} · 相对原子质量 {}\n电子层 {}",
                        number,
                        elements::family_name(e.family),
                        elements::atomic_mass(number),
                        elements::electron_shells(number)
                    ),
                    24.,
                    INK,
                    k,
                );
                words(
                    p,
                    art,
                    [1270., 570., 320., 90.],
                    elements::element_fact(number),
                    27.,
                    TEAL,
                    k,
                );
            }
            // The task and collection contents are live, including recipes with more than three atoms.
            paper(p, art, [380., 278., 440., 170.], k);
            words(p, art, [390., 282., 420., 100.], m.clue, 30., INK, k);
            words(p, art, [390., 380., 420., 60.], m.formula, 43., TEAL, k);
            for (i, n) in g.picked.iter().enumerate() {
                let symbol = elements::ELEMENTS[*n as usize - 1].symbol;
                let r = [
                    407. + (i % 3) as f32 * 215.,
                    754. + (i / 3) as f32 * 48.,
                    150.,
                    45.,
                ];
                words(p, art, r, symbol, 35., TEAL, k);
            }
            paper(p, art, [365., 875., 1120., 35.], k);
            words(p, art, [365., 875., 1120., 35.], g.feedback, 23., TEAL, k);
            hit(p, [1070., 760., 145., 113.], Action::ElementObserve, k);
            if g.complete {
                p.spawn((
                    area([1244., 814., 112., 45.], k),
                    BackgroundColor(Color::srgb(0.16, 0.53, 0.82)),
                    BorderRadius::all(Val::Px(8. * k)),
                ));
                words(
                    p,
                    art,
                    [1244., 814., 112., 45.],
                    if g.finished { "再玩" } else { "下一关" },
                    30.,
                    Color::WHITE,
                    k,
                );
            } else {
                p.spawn((
                    area([1230., 760., 140., 113.], k),
                    BackgroundColor(Color::srgba(1., 1., 1., 0.3)),
                ));
            }
            hit(
                p,
                [1230., 760., 140., 113.],
                if g.finished {
                    Action::ElementReset
                } else {
                    Action::ElementNext
                },
                k,
            );
            hit(p, [1390., 760., 140., 113.], Action::ElementReset, k);
        },
    );
}
fn restaurant_book(commands: &mut Commands, s: &Session, art: &Art, w: f32, h: f32) {
    let design = Vec2::new(1672., 941.);
    let k = (w / design.x).min(h / design.y);
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
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(design.x * k),
                height: Val::Px(design.y * k),
                flex_shrink: 0.,
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    ImageNode::new(art.storybook.restaurant.clone()),
                    area([0., 0., 1672., 941.], k),
                ));
                for r in [
                    [0., 0., 1672., 125.],
                    [0., 125., 355., 655.],
                    [1317., 125., 355., 655.],
                    [0., 780., 1672., 161.],
                ] {
                    p.spawn((WorldInputBlocker, area(r, k)));
                }
                let (built, kinds) = s.work.creative_progress();
                words(
                    p,
                    art,
                    [580., 28., 520., 65.],
                    format!("搭建 {built}/10 块 · 材料 {kinds}/3 种"),
                    34.,
                    INK,
                    k,
                );
                hit(p, [1240., 23., 185., 70.], Action::RestaurantCheck, k);
                hit(p, [1440., 23., 195., 70.], Action::Library, k);
                for (i, a) in [
                    Action::Build,
                    Action::Erase,
                    Action::Undo,
                    Action::RestaurantReset,
                ]
                .into_iter()
                .enumerate()
                {
                    let r = [52., 247. + i as f32 * 111., 269., 100.];
                    hit(p, r, a, k);
                    if (i == 0 && !s.erase) || (i == 1 && s.erase) {
                        outline(p, r, k);
                    }
                }
                for (i, m) in [
                    Material::Wood,
                    Material::Stone,
                    Material::Glass,
                    Material::Brick,
                    Material::Grass,
                    Material::Crystal,
                ]
                .into_iter()
                .enumerate()
                {
                    let r = [
                        1354. + (i % 2) as f32 * 142.,
                        257. + (i / 2) as f32 * 162.,
                        130.,
                        148.,
                    ];
                    hit(p, r, Action::Material(m), k);
                    if m == s.material {
                        outline(p, r, k);
                    }
                    if i == 5 {
                        paper(p, art, [1500., 690., 112., 38.], k);
                        words(p, art, [1500., 690., 112., 38.], "水晶", 28., INK, k);
                    }
                }
                hit(p, [355., 810., 230., 80.], Action::Rotate, k);
                hit(p, [605., 810., 225., 80.], Action::ZoomIn, k);
                hit(p, [850., 810., 225., 80.], Action::ZoomOut, k);
                hit(p, [1140., 795., 360., 110.], Action::RestaurantCheck, k);
                if s.message.contains("完成")
                    || s.message.contains("撤销")
                    || s.message.contains("布置")
                {
                    book_key(
                        p,
                        art,
                        [430., 710., 800., 65.],
                        &s.message,
                        Action::Build,
                        k,
                    );
                }
            });
        });
}

pub(super) fn restaurant_starter() -> Work {
    let mut work = Work::default();
    work.choose_level(5);
    work.program.clear();
    work.world = game::World {
        blocks: vec![],
        robot: [0, 0],
        facing: 0,
        goal: [0, 0],
        crystals: vec![],
        free_build: true,
    };
    for x in -5..=5 {
        for z in -4..=4 {
            work.world.put([x, 0, z], Material::Grass);
        }
    }
    work
}
pub(super) fn load_restaurant_work() -> Work {
    fs::read(save_directory().join("restaurant-v1.json"))
        .ok()
        .and_then(|b| serde_json::from_slice::<Work>(&b).ok())
        .filter(|w| w.valid() && w.level == 5 && w.world.free_build)
        .unwrap_or_else(restaurant_starter)
}

/// Opt-in real UI hit-testing for the capture workflow; never active in normal play.
pub(super) fn smoke_input(
    mut frame: Local<u32>,
    s: Res<Session>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    controls: Query<(&Action, &ComputedNode, &GlobalTransform), With<Button>>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
) {
    if std::env::var_os("MAGNET_STORYBOOK_SMOKE").is_none()
        || std::env::var_os("MAGNET_CAPTURE").is_none()
    {
        return;
    }
    *frame += 1;
    let f = *frame;
    let mut raw: Option<Vec2> = None;
    let wanted = match s.mode {
        GameMode::Literacy => match f {
            20 => Some(Action::LiteracyStart(0)),
            40 | 80 => Some(Action::LiteracyNext),
            60 => Some(Action::LiteracySelect(s.literacy.word_index())),
            _ => None,
        },
        GameMode::Math => match f {
            20 => Some(Action::MathAnswer(s.math.question().answer)),
            40 => Some(Action::MathNext),
            60 => Some(Action::MathLevels),
            80 => Some(Action::MathLevel(2)),
            _ => None,
        },
        GameMode::MusicFactory if [20, 40, 60, 80].contains(&f) => {
            Some(Action::PianoKey(s.piano.target()))
        }
        GameMode::MoonFestival if f == 20 => Some(Action::MoonShake),
        GameMode::Classic2048 => match f {
            20 => Some(Action::ClassicMove(-1, 0)),
            40 => Some(Action::ClassicMove(0, -1)),
            60 => Some(Action::ClassicMove(1, 0)),
            80 => Some(Action::ClassicMove(0, 1)),
            _ => None,
        },
        GameMode::Klotski => match f {
            20 => Some(Action::KlotskiSelect(6)),
            40 => Some(Action::KlotskiMove(0, 1)),
            _ => None,
        },
        GameMode::Sudoku => match f {
            20 => Some(Action::SudokuSize(4)),
            40 => Some(Action::SudokuSelect(1)),
            60 => Some(Action::SudokuDigit(2)),
            80 => Some(Action::SudokuHint),
            _ => None,
        },
        GameMode::MushroomRun if (20..=70).contains(&f) => Some(Action::PlatformRight),
        GameMode::MushroomRun if f == 80 => Some(Action::PlatformJump),
        GameMode::PixelAssault if (20..=60).contains(&f) => Some(Action::AssaultFire),
        GameMode::PixelAssault if (80..=100).contains(&f) => Some(Action::AssaultRight),
        GameMode::Breakout => match f {
            20 => Some(Action::BreakoutLaunch),
            30 => {
                raw = Some(Vec2::new(1100., 620.));
                None
            }
            60 => Some(Action::BreakoutToggle),
            _ => None,
        },
        GameMode::ElementLab => match f {
            20 | 40 => Some(Action::ElementPick(1)),
            60 => Some(Action::ElementPick(8)),
            80 => Some(Action::ElementNext),
            100 => Some(Action::ElementObserve),
            _ => None,
        },
        GameMode::CrystalGarden => match f {
            20 => Some(Action::Material(Material::Stone)),
            40 => {
                raw = Some(Vec2::new(836., 445.));
                None
            }
            60 => Some(Action::Material(Material::Wood)),
            80 => {
                raw = Some(Vec2::new(880., 460.));
                None
            }
            100 => Some(Action::RestaurantCheck),
            _ => None,
        },
        _ => None,
    };
    if let Ok(mut window) = windows.single_mut() {
        let point = wanted
            .and_then(|wanted| {
                controls
                    .iter()
                    .find(|(a, _, _)| same_action(**a, wanted))
                    .map(|(_, _, t)| t.translation().truncate() / window.scale_factor())
            })
            .or_else(|| {
                raw.map(|r| {
                    let size = Vec2::new(window.width(), window.height());
                    let k = (size.x / 1672.).min(size.y / 941.);
                    (size - Vec2::new(1672., 941.) * k) * 0.5 + r * k
                })
            });
        if let Some(point) = point {
            window.set_cursor_position(Some(point));
            mouse.press(MouseButton::Left);
        } else {
            mouse.release(MouseButton::Left);
        }
    }
    if f == 120 {
        let valid = match s.mode {
            GameMode::Literacy => s.literacy.stage == 2 && !s.literacy.map,
            GameMode::Math => s.math.level == 2 && !s.math.choosing_level,
            GameMode::MusicFactory => s.piano.step == 4,
            GameMode::MoonFestival => s.moon.rolls_left == 4 || s.moon.rolling,
            GameMode::Classic2048 => s.classic_2048.score >= 4,
            GameMode::Klotski => s.klotski.pieces[6].y == 4,
            GameMode::Sudoku => s.sudoku.size == 4 && s.sudoku.selected.is_some(),
            GameMode::MushroomRun => s.platform.distance > 0.,
            GameMode::PixelAssault => s.assault.distance > 18.,
            GameMode::Breakout => s.breakout.paused && s.breakout.paddle_x > 500.,
            GameMode::ElementLab => s.elements.stage == 1 && s.elements.observing,
            GameMode::CrystalGarden => s.work.creative_progress().0 > 0,
            _ => false,
        };
        eprintln!(
            "STORYBOOK_SMOKE {} {}",
            s.mode.title(),
            if valid { "PASS" } else { "FAIL" }
        );
        assert!(valid, "storybook UI interaction smoke failed");
    }
}
fn same_action(a: Action, b: Action) -> bool {
    match (a, b) {
        (Action::LiteracyStart(x), Action::LiteracyStart(y))
        | (Action::LiteracySelect(x), Action::LiteracySelect(y))
        | (Action::MathLevel(x), Action::MathLevel(y))
        | (Action::PianoKey(x), Action::PianoKey(y))
        | (Action::KlotskiSelect(x), Action::KlotskiSelect(y))
        | (Action::SudokuSize(x), Action::SudokuSize(y))
        | (Action::SudokuSelect(x), Action::SudokuSelect(y)) => x == y,
        (Action::MathAnswer(x), Action::MathAnswer(y)) => x == y,
        (Action::SudokuDigit(x), Action::SudokuDigit(y))
        | (Action::ElementPick(x), Action::ElementPick(y)) => x == y,
        (Action::ClassicMove(x, y), Action::ClassicMove(xx, yy))
        | (Action::KlotskiMove(x, y), Action::KlotskiMove(xx, yy)) => x == xx && y == yy,
        (Action::Material(x), Action::Material(y)) => x == y,
        _ => std::mem::discriminant(&a) == std::mem::discriminant(&b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restaurant_starts_on_a_valid_single_building_island() {
        let w = restaurant_starter();
        assert!(w.valid());
        assert!(w.world.free_build);
        assert_eq!(w.creative_progress(), (0, 0));
        assert_eq!(w.world.blocks.len(), 99);
    }
    #[test]
    fn restaurant_round_trip_preserves_placed_materials() {
        let mut w = restaurant_starter();
        w.world.edit([0, 1, 0], Material::Wood, false).unwrap();
        w.world.edit([1, 1, 0], Material::Stone, false).unwrap();
        let saved = serde_json::to_vec(&w).unwrap();
        let loaded: Work = serde_json::from_slice(&saved).unwrap();
        assert!(loaded.valid());
        assert_eq!(loaded.creative_progress(), (2, 2));
    }
}
