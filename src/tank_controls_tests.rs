use super::*;
use bevy::input::{
    ButtonState, InputPlugin,
    keyboard::{Key, KeyboardInput},
    touch::{TouchInput, TouchPhase},
};

fn fixture() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(InputPlugin)
        .init_resource::<Time>()
        .init_resource::<TankHold>()
        .add_systems(Update, (tank_key_input, tank_touch_hold).chain());
    let mut session = Session::new();
    session.mode = GameMode::TankBattle;
    session.library_open = false;
    session.tank.walls.clear();
    session.tank.steel.clear();
    session.tank.enemies.clear();
    session.tank.player = (3, 3);
    app.insert_resource(session);
    let window = app
        .world_mut()
        .spawn((Window::default(), PrimaryWindow))
        .id();
    app.world_mut().spawn((
        TankStick,
        ComputedNode {
            size: Vec2::splat(100.),
            ..default()
        },
        GlobalTransform::from_translation(Vec3::new(100., 100., 0.)),
    ));
    app.world_mut().spawn((
        TankFirePad,
        ComputedNode {
            size: Vec2::splat(100.),
            ..default()
        },
        GlobalTransform::from_translation(Vec3::new(800., 100., 0.)),
    ));
    (app, window)
}

fn frame(app: &mut App, dt: f32) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(dt));
    // The production tick runs after the input systems.
    app.update();
    let mut session = app.world_mut().resource_mut::<Session>();
    session.tank.cooldown = (session.tank.cooldown - dt).max(0.);
}

fn key(app: &mut App, window: Entity, code: KeyCode, state: ButtonState) {
    app.world_mut().send_event(KeyboardInput {
        window,
        key_code: code,
        state,
        logical_key: Key::Character("test".into()),
        text: None,
        repeat: false,
    });
}

fn touch(app: &mut App, window: Entity, id: u64, phase: TouchPhase, x: f32, y: f32) {
    app.world_mut().send_event(TouchInput {
        window,
        id,
        phase,
        position: Vec2::new(x, y),
        force: None,
    });
}

#[test]
fn held_keyboard_moves_repeats_fires_and_stops_without_os_repeat() {
    let (mut app, window) = fixture();
    key(&mut app, window, KeyCode::KeyW, ButtonState::Pressed);
    key(&mut app, window, KeyCode::Space, ButtonState::Pressed);
    frame(&mut app, 0.02);
    assert_eq!(app.world().resource::<Session>().tank.player, (3, 2));
    for _ in 0..19 {
        frame(&mut app, 0.02);
    }
    let tank = &app.world().resource::<Session>().tank;
    assert_eq!(tank.player, (3, 0));
    assert_eq!(tank.bullets.len(), 2);
    key(&mut app, window, KeyCode::KeyW, ButtonState::Released);
    key(&mut app, window, KeyCode::Space, ButtonState::Released);
    for _ in 0..20 {
        frame(&mut app, 0.02);
    }
    let tank = &app.world().resource::<Session>().tank;
    assert_eq!(tank.player, (3, 0));
    assert_eq!(tank.bullets.len(), 2);
}

#[test]
fn latest_direction_aims_immediately_without_an_extra_step_and_resumes_held_key() {
    let (mut app, window) = fixture();
    key(&mut app, window, KeyCode::KeyW, ButtonState::Pressed);
    frame(&mut app, 0.02);
    key(&mut app, window, KeyCode::KeyD, ButtonState::Pressed);
    key(&mut app, window, KeyCode::KeyJ, ButtonState::Pressed);
    frame(&mut app, 0.02);
    let tank = &app.world().resource::<Session>().tank;
    assert_eq!(
        tank.player,
        (3, 2),
        "turning must not add an immediate extra tile"
    );
    assert_eq!((tank.bullets[0].dx, tank.bullets[0].dy), (1., 0.));
    for _ in 0..9 {
        frame(&mut app, 0.02);
    }
    assert_eq!(app.world().resource::<Session>().tank.player, (4, 2));
    key(&mut app, window, KeyCode::KeyD, ButtonState::Released);
    frame(&mut app, 0.02);
    assert_eq!(app.world().resource::<Session>().tank.direction, (0, -1));
}

#[test]
fn diagonal_and_dead_zone_jitter_do_not_flip_or_restart_the_stick() {
    let right = Some((1, 0));
    assert_eq!(tank_stick_direction(Vec2::new(30., 31.), 50., right), right);
    assert_eq!(tank_stick_direction(Vec2::new(31., 30.), 50., right), right);
    assert_eq!(
        tank_stick_direction(Vec2::new(30., 40.), 50., right),
        Some((0, 1))
    );
    assert_eq!(tank_stick_direction(Vec2::new(8., 0.), 50., right), right);
    assert_eq!(tank_stick_direction(Vec2::new(6., 0.), 50., right), None);
    assert_eq!(tank_stick_direction(Vec2::new(8., 0.), 50., None), None);
}

#[test]
fn repeat_distance_is_consistent_at_30_60_and_120_fps() {
    for fps in [30, 60, 120] {
        let mut game = TankGame::new();
        game.walls.clear();
        game.steel.clear();
        game.enemies.clear();
        game.player = (3, 6);
        let mut hold = TankHold::default();
        for _ in 0..fps / 2 {
            hold.drive(&mut game, Some((0, -1)), false, 1. / fps as f32);
        }
        assert_eq!(game.player, (3, 3), "movement at {fps} fps");
    }
}

#[test]
fn two_fingers_can_move_and_fire_with_drift_and_independent_release() {
    let (mut app, window) = fixture();
    touch(&mut app, window, 11, TouchPhase::Started, 100., 70.);
    touch(&mut app, window, 22, TouchPhase::Started, 800., 100.);
    frame(&mut app, 0.02);
    assert_eq!(app.world().resource::<Session>().tank.player, (3, 2));
    touch(&mut app, window, 22, TouchPhase::Moved, 875., 130.);
    for _ in 0..19 {
        frame(&mut app, 0.02);
    }
    let tank = &app.world().resource::<Session>().tank;
    assert_eq!(tank.player, (3, 0));
    assert_eq!(
        tank.bullets.len(),
        2,
        "fire survives thumb drifting outside the button"
    );
    touch(&mut app, window, 11, TouchPhase::Ended, 100., 70.);
    frame(&mut app, 0.02);
    assert_eq!(app.world().resource::<TankHold>().direction, None);
    for _ in 0..12 {
        frame(&mut app, 0.02);
    }
    assert_eq!(app.world().resource::<Session>().tank.bullets.len(), 3);
    touch(&mut app, window, 22, TouchPhase::Canceled, 875., 130.);
    for _ in 0..20 {
        frame(&mut app, 0.02);
    }
    assert_eq!(app.world().resource::<Session>().tank.bullets.len(), 3);
}

#[test]
fn sliding_an_unrelated_finger_onto_fire_does_not_trigger_a_shot() {
    let (mut app, window) = fixture();
    touch(&mut app, window, 22, TouchPhase::Started, 600., 100.);
    frame(&mut app, 0.02);
    touch(&mut app, window, 22, TouchPhase::Moved, 800., 100.);
    for _ in 0..20 {
        frame(&mut app, 0.02);
    }
    assert!(app.world().resource::<Session>().tank.bullets.is_empty());
}

#[test]
fn help_and_focus_loss_clear_captured_controls() {
    for lose_focus in [false, true] {
        let (mut app, window) = fixture();
        touch(&mut app, window, 11, TouchPhase::Started, 100., 70.);
        touch(&mut app, window, 22, TouchPhase::Started, 800., 100.);
        frame(&mut app, 0.02);
        if lose_focus {
            app.world_mut().get_mut::<Window>(window).unwrap().focused = false;
        } else {
            app.world_mut().resource_mut::<Session>().help_open = true;
        }
        for _ in 0..20 {
            frame(&mut app, 0.02);
        }
        let hold = app.world().resource::<TankHold>();
        assert_eq!(hold.pointer, None);
        assert_eq!(hold.fire_pointer, None);
        let tank = &app.world().resource::<Session>().tank;
        assert_eq!(tank.player, (3, 2));
        assert_eq!(tank.bullets.len(), 1);
        app.world_mut().get_mut::<Window>(window).unwrap().focused = true;
        app.world_mut().resource_mut::<Session>().help_open = false;
        for _ in 0..20 {
            frame(&mut app, 0.02);
        }
        assert_eq!(app.world().resource::<Session>().tank.player, (3, 2));
        assert_eq!(app.world().resource::<Session>().tank.bullets.len(), 1);
    }
}
