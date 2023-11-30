

use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIExtensionsPlugin, events::OneShot};
use bevy::{prelude::*, window::PrimaryWindow};

const FONT: &str = "fonts/rajdhani/Rajdhani-Regular.ttf";
const BTN: &str = "images/main_menu/button.png";

fn main() {
    App::new()
        // Game boilerplate
        .add_plugins((DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Aouipunk".into(),
                    ..default()
                }),
                ..default()
            }
        ), bevy::diagnostic::FrameTimeDiagnosticsPlugin))
        
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIExtensionsPlugin)
        .add_systems(Startup, (setup, apply_deferred).chain())
        .run();
}

fn button<M: Component>(commands: &mut Commands, marker: M) -> impl Bundle {
    use bevy_aoui_widgets::dsl::prelude::*;
    (
        EventFlags::Click | EventFlags::Hover,
        SetCursor { 
            flags: EventFlags::Click | EventFlags::Hover, 
            icon: CursorIcon::Hand,
        },
        Interpolate::<Offset>::ease(EaseFunction::SineInOut, Vec2::ZERO, 0.3),
        Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ZERO, 0.3),
        marker,
        OneShot::new(Hover, oneshot!(commands => fn on_hover<M: Component>(
                mut offset: Query<&mut Interpolate<Offset>, With<M>>,
                mut color: Query<&mut Interpolate<Color>, (With<M>, Without<Text>)>,
                mut color2: Query<&mut Interpolate<Color>, (With<M>, With<Text>)>,
            ) {
                offset.single_mut().interpolate_to_or_reverse(Vec2::new(20.0, 0.0));
                color.single_mut().interpolate_or_reverse([
                    (color!(transparent).into(), 0.0),
                    (color!(red).into(), 0.2),
                    (color!(yellow).into(), 1.0),
                ]);
                color2.single_mut().interpolate_or_reverse([
                    (color!(white).into(), 0.0),
                    (color!(red).into(), 0.2),
                    (color!(yellow).into(), 1.0),
                ]);
            }
        )),
        OneShot::new(LoseFocus, oneshot!(commands => fn on_hover<M: Component>(
                mut offset: Query<&mut Interpolate<Offset>, With<M>>,
                mut color: Query<&mut Interpolate<Color>, (With<M>, Without<Text>)>,
                mut color2: Query<&mut Interpolate<Color>, (With<M>, With<Text>)>,
            ){
                offset.single_mut().interpolate_to_or_reverse(Vec2::new(0.0, 0.0));
                color.single_mut().interpolate_or_reverse([
                    (color!(yellow).into(), 0.0),
                    (color!(red).into(), 0.8),
                    (color!(transparent).into(), 1.0),
                ]);
                color2.single_mut().interpolate_or_reverse([
                    (color!(yellow).into(), 0.0),
                    (color!(red).into(), 0.8),
                    (color!(white).into(), 1.0),
                ]);
            }
        )),
    )
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut window: Query<&mut Window, With<PrimaryWindow>>) {

    // Start playing the main menu music
    commands.spawn(
        AudioBundle {
            source: assets.load("sounds/main_menu.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new_relative(0.5)),
        }
    );
    
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    window.single_mut().cursor.visible = false;

    use bevy_aoui_widgets::dsl::prelude::*;

    marker!(Continue, NewGame, Load, Settings, Dlc, Credits, QuitGame);

    sprite!((commands) {
        sprite: assets.load("images/main_menu/background.png"),
        dimension: size2!([1 + 40 px, 100%]),
        z: -10,
        extra: Interpolate::<Offset>::repeat(
            Some(EaseFunction::CubicInOut), 
            [(Vec2::new(-20.0, 0.0), 0.0), (Vec2::new(20.0, 0.0), 1.0)],
            5.0,
        )
    });

    sprite!((commands) {
        sprite: assets.load("images/pointer.png"),
        extra: CustomCursor::new(CursorIcon::Arrow, Vec2::new(5.0, -5.0)),
        dimension: [20, 20],
        color: color!(gold),
        z: 500
    });

    sprite!((commands) {
        sprite: assets.load("images/hand.png"),
        extra: CustomCursor::new(CursorIcon::Hand, Vec2::new(5.0, -5.0)),
        dimension: [20, 20],
        color: color!(gold),
        z: 500
    });

    frame!((commands) {
        dimension: size2!([50%, 100%]),
        anchor: CenterLeft,
        child: sprite! {
            sprite: assets.load("images/main_menu/board.png"),
            dimension: size2!([100%, 100%]),
        },
        child: vbox! {
            margin: [0, 5],
            dimension: size2!([80%, 100%]),
            font_size: px(24),
            child: sprite! {
                sprite: assets.load("images/main_menu/bevypunk.png"),
                scale: [0.4, 0.4],
                hitbox: Rect(1),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "CONTINUE",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: Continue,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, Continue),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "NEW GAME",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: NewGame,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, NewGame),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "LOAD GAME",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: Load,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, Load),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "SETTINGS",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: Settings,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, Settings),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "ADDITIONAL CONTENT",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: Dlc,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, Dlc),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "CREDITS",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: Credits,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, Credits),
            },
            child: sprite! {
                sprite: assets.load(BTN),
                hitbox: Rect(1),
                dimension: size2!([80%, 1.6 em]),
                child: textbox! {
                    text: "QUIT GAME",
                    font: assets.load(FONT),
                    anchor: Left,
                    offset: size2!([1 em, 0]),
                    extra: QuitGame,
                    extra: Interpolate::<Color>::ease(EaseFunction::SineInOut, Vec4::ONE, 0.3),
                },
                extra: button(&mut commands, QuitGame),
                extra: handler! { LeftClick => 
                    fn exit_game(mut exit: EventWriter<bevy::app::AppExit>) {
                    exit.send(bevy::app::AppExit);
                }}
            },
        }
    });
}
