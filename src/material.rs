use bevy::prelude::{default, Assets, Color, FromWorld, Handle, Resource, StandardMaterial, World};
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::{BuildingClass, RoadClass};

type Reflectance = f32;
type Roughness = f32;
pub trait MaterialForBuilding {
    fn to_material_params(self: &Self) -> (Reflectance, Roughness);
}

const DEFAULT_MATERIAL_PROPS: (Reflectance, Roughness) = (0.6, 0.5);
impl MaterialForBuilding for BuildingClass {
    fn to_material_params(self: &Self) -> (Reflectance, Roughness) {
        match self {
            BuildingClass::Commercial => (0.3, 0.8),
            _ => DEFAULT_MATERIAL_PROPS,
        }
    }
}

#[derive(Resource)]
pub struct MapMaterialHandle {
    pub roof: Handle<StandardMaterial>,
    pub walls: HashMap<BuildingClass, Handle<StandardMaterial>>,
    pub unknown_building: Handle<StandardMaterial>,
    pub road: HashMap<RoadClass, Handle<StandardMaterial>>,
}

impl FromWorld for MapMaterialHandle {
    fn from_world(world: &mut World) -> Self {
        let mut standard_materials = world.resource_mut::<Assets<StandardMaterial>>();

        let roof_color = Color::rgb(0.3, 0.3, 0.2);
        let roof = standard_materials.add(StandardMaterial {
            base_color: roof_color,
            depth_bias: 0.,
            reflectance: 0.5,
            perceptual_roughness: 0.75,
            ..default()
        });

        let mut walls: HashMap<BuildingClass, Handle<StandardMaterial>> = HashMap::new();
        for building_class in BuildingClass::iter() {
            let color = Color::from(&building_class);
            let (reflectance, roughness) = building_class.to_material_params();
            let wall_color_handle = standard_materials.add(StandardMaterial {
                base_color: color,
                depth_bias: 0.,
                reflectance,
                perceptual_roughness: roughness,
                ..default()
            });
            walls
                .entry(building_class)
                .or_insert_with_key(|_key| wall_color_handle);
        }

        let unknown_building_color = Color::GRAY;
        let unknown_building = standard_materials.add(StandardMaterial {
            base_color: unknown_building_color,
            depth_bias: 0.,
            reflectance: 0.5,
            perceptual_roughness: 0.7,
            ..default()
        });

        let mut road: HashMap<RoadClass, Handle<StandardMaterial>> = HashMap::new();
        for road_class in RoadClass::iter() {
            let color = Color::from(&road_class);
            let road_color_handle = standard_materials.add(StandardMaterial {
                base_color: color,
                depth_bias: road_class.depth_bias() * 100.,
                reflectance: 0.5,
                perceptual_roughness: 0.8,
                ..default()
            });
            road.entry(road_class)
                .or_insert_with_key(|_key| road_color_handle);
        }

        Self {
            roof,
            walls,
            unknown_building,
            road,
        }
    }
}
