use bevy::{prelude::*, render::mesh::*};
use geo::algorithm::TriangulateEarcut;
use geo::algorithm::Vector2DOps;
use geo::geodesic_distance::GeodesicDistance;
use geo_types::Polygon;
use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Neg, Sub};

#[derive(Component, Debug)]
pub struct BevyTransportation {
    pub translate: [f64; 2],
    pub height: Option<f64>,
    pub line: Vec<[f64; 2]>,
    pub k: f64,
    pub vertices: Vec<[f64; 3]>,
    pub triangle_indices: Vec<u32>,
}

#[derive(Resource, Debug)]
pub struct BevyTransportations {
    pub transportations: Vec<BevyTransportation>,
}

pub fn transportations_start(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transportations_res: Res<BevyTransportations>,
) {
    for item in transportations_res.transportations.iter() {
        spawn_transportation(&mut cmd, &mut meshes, &mut materials, item);
    }
}

pub fn spawn_transportation(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transportation: &BevyTransportation,
) {
}
