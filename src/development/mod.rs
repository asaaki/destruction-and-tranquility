use bevy::prelude::*;

/// Enables the Esc key to exit the game in debug mode;
/// it's always disabled in WASM.
pub struct ExitonEscPlugin;

impl Plugin for ExitonEscPlugin {
    #[cfg(any(target_arch = "wasm32", not(debug_assertions)))]
    fn build(&self, _app: &mut App) {}

    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bevy::window::close_on_esc);
    }
}
