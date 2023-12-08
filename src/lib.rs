use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{WindowResized, WindowTheme};
use bevy::{
    core::FrameCount,
    log::{Level, LogPlugin},
    window::{PresentMode, WindowResolution},
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_svg::prelude::*;
// use web_time::{Instant, SystemTime};

shadow_rs::shadow!(build);
mod development;

const TITLE: &str = "[DaT] Destruction and Tranquility";
// const CLEAR_COLOR_BOOT: ClearColor = ClearColor(Color::BLACK);
const CLEAR_COLOR_BOOT: ClearColor = ClearColor(Color::rgb(0.0, 0.1, 0.2));

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum AppState {
    #[default]
    Booting,
    Running,
}

pub fn run() {
    let window = Window {
        title: TITLE.into(),
        window_theme: Some(WindowTheme::Dark),
        resolution: WindowResolution::new(800., 600.),
        present_mode: PresentMode::AutoVsync,
        visible: false,
        // wasm
        prevent_default_event_handling: false,
        fit_canvas_to_parent: true,
        ..default()
    };

    let default_plugins = DefaultPlugins
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "debug,wgpu=error,wgpu_core=error,wgpu_hal=error,naga=warn,bevy_app=info,bevy_render=info,bevy_ecs=debug".to_string(),
            })
            .set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            })
            .build();

    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(CLEAR_COLOR_BOOT)
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            default_plugins,
            SvgPlugin,
            FrameTimeDiagnosticsPlugin,
            development::ExitonEscPlugin,
        ))
        .add_state::<AppState>()
        .insert_resource(Time::<Fixed>::from_seconds(1. / 20.))
        .add_systems(
            Startup,
            (
                setup_assets,
                setup_cursor,
                setup_camera,
                setup_test_assets,
                setup_fps_display,
                setup_debug_build_info,
            ),
        )
        .add_systems(Update, make_visible.run_if(in_state(AppState::Booting)))
        .add_systems(Update, (animate_ferris, absolute_positioning))
        .add_systems(FixedUpdate, (update_fps_display,))
        .run();
}

fn make_visible(
    mut next_state: ResMut<'_, NextState<AppState>>,
    mut window: Query<'_, '_, &mut Window>,
    frames: Res<'_, FrameCount>,
) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
        next_state.set(AppState::Running);
    }
}

fn setup_assets(asset_server: Res<'_, AssetServer>) {
    // let _fonts: Handle<LoadedFolder> = asset_server.load_folder("fonts");
    // let _textures: Handle<LoadedFolder> = asset_server.load_folder("textures");
    // let _svgs: Handle<LoadedFolder> = asset_server.load_folder("svgs");
    let _font: Handle<Font> = asset_server.load("fonts/FiraMonoNerdFontPropo-Regular.otf");
    let _ferris: Handle<Image> = asset_server.load("textures/ferris_and_friends/ferris-wave.png");
    let _m: Handle<Svg> = asset_server.load("svgs/m.svg");
}

fn setup_cursor(mut windows: Query<'_, '_, &mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Hand;
}

fn setup_camera(mut commands: Commands<'_, '_>) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    },));
    // .add(InitWorldTracking).insert(MainCamera)
}

#[derive(Component)]
struct Ferris;

#[derive(Component)]
struct M;

fn setup_test_assets(mut commands: Commands<'_, '_>, asset_server: Res<'_, AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("textures/ferris_and_friends/ferris-wave.png"),
            ..default()
        })
        .insert(Ferris);

    commands
        .spawn(Svg2dBundle {
            svg: asset_server.load("svgs/m.svg"),
            transform: Transform::from_scale(Vec3::splat(0.004))
                .with_translation(Vec3::new(350.0, -255.0, 0.0)),
            ..default()
        })
        .insert(M);
}

fn default_font(asset_server: Res<'_, AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/FiraMonoNerdFontPropo-Regular.otf")
}

#[derive(Component)]
struct FpsDisplay;

fn setup_fps_display(mut commands: Commands<'_, '_>, asset_server: Res<'_, AssetServer>) {
    let font = default_font(asset_server);
    let style = TextStyle {
        font: font.clone(),
        font_size: 40.,
        color: Color::rgba(0.5, 0.9, 1.0, 1.0),
    };
    commands
        .spawn(TextBundle {
            text: Text::from_sections([
                TextSection::new("FPS: ".to_string(), style.clone()),
                TextSection::new("0".to_string(), style.clone()),
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            },
            ..default()
        })
        .insert(FpsDisplay);
}

fn setup_debug_build_info(mut commands: Commands<'_, '_>, asset_server: Res<'_, AssetServer>) {
    let font = default_font(asset_server);

    commands.spawn(TextBundle {
        text: Text::from_sections([TextSection::new(
            format!(
                "v{} ({}{}{}; {})\n{}\n{}",
                build::PKG_VERSION,
                build::SHORT_COMMIT,
                if build::TAG.is_empty() { "" } else { ", " },
                build::TAG,
                build::BUILD_TIME,
                build::RUST_VERSION,
                build::RUST_CHANNEL,
            ),
            TextStyle {
                font: font.clone(),
                font_size: 12.0,
                color: Color::GOLD,
            },
        )])
        .with_alignment(TextAlignment::Right),
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            right: Val::Px(60.0),
            ..default()
        },
        ..default()
    });
}

// UPDATEs

fn animate_ferris(time: Res<'_, Time>, mut ferris: Query<'_, '_, &mut Transform, With<Ferris>>) {
    if let Some(mut transform) = ferris.iter_mut().next() {
        transform.rotation = Quat::from_rotation_z(0.25 * (time.elapsed_seconds() / 4.0).sin());
        transform.scale = Vec3::splat(1.0 + 0.5 * (time.elapsed_seconds() / 2.0).sin());
    }
}

fn absolute_positioning(
    mut query: Query<'_, '_, &mut Transform, With<M>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    const M_X_OFFSET: f32 = 50.0;
    const M_Y_OFFSET: f32 = 50.0;

    for e in resize_reader.read() {
        if let Some(mut transform) = query.iter_mut().next() {
            transform.translation = Vec3::new(
                (0.5 * e.width) - M_X_OFFSET,
                -(0.5 * e.height) + M_Y_OFFSET,
                0.0,
            );
        }
    }
}

fn update_fps_display(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsDisplay>>,
) {
    if let Some(mut text) = query.iter_mut().next() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            text.sections[1].value = format!("{:.0}", fps.average().unwrap_or_default());
        }
    }
}
