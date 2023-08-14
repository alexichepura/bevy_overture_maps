use bevy::prelude::Resource;

#[derive(Resource)]
pub struct SceneConfig {
    pub size: f32,
}

impl Default for SceneConfig {
    fn default() -> Self {
        SceneConfig { size: 20000. }
    }
}
