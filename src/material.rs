use bevy::prelude::{default, Assets, Color, FromWorld, Handle, Resource, StandardMaterial, World};

#[derive(Resource)]
pub struct MapMaterialHandle {
    pub roof: Handle<StandardMaterial>,
}

impl FromWorld for MapMaterialHandle {
    fn from_world(world: &mut World) -> Self {
        let roof_color = Color::rgb(0.3, 0.3, 0.2);

        let mut standard_materials = world.resource_mut::<Assets<StandardMaterial>>();
        let roof_color_handle = standard_materials.add(StandardMaterial {
            base_color: roof_color,
            depth_bias: 0.,
            reflectance: 0.5,
            perceptual_roughness: 0.75,
            ..default()
        });

        Self {
            roof: roof_color_handle,
        }
    }
}
