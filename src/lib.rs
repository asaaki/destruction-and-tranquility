use bevy::prelude::*;
use bevy::{
    asset::LoadedFolder,
    core::FrameCount,
    log::{Level, LogPlugin},
    window::{PresentMode, WindowResolution},
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
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
        resolution: WindowResolution::new(1920., 1080.),
        present_mode: PresentMode::AutoVsync,
        visible: false,
        // wasm
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        // canvas: Some("#bevy-canvas".to_string()),
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
            development::ExitonEscPlugin,
        ))
        .add_state::<AppState>()
        .add_systems(
            Startup,
            (
                build_info,
                setup_assets,
                setup_cursor,
                setup_camera,
                setup_test_assets,
            ),
        )
        .add_systems(Update, make_visible.run_if(in_state(AppState::Booting)))
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

fn build_info() {
    info!("built @ {}", build::BUILD_TIME);
    info!("built with {}", build::RUST_VERSION);
    // info!("committed @ {}", build::COMMIT_DATE);
    // info!("commit: {}", build::SHORT_COMMIT);
}

fn setup_assets(asset_server: Res<'_, AssetServer>) {
    // let _fonts: Handle<LoadedFolder> = asset_server.load_folder("fonts");
    let _textures: Handle<LoadedFolder> = asset_server.load_folder("textures");
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

fn setup_test_assets(mut commands: Commands<'_, '_>, asset_server: Res<'_, AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/ferris_and_friends/ferris-wave.png"),
        ..default()
    });
}
