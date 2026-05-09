use bevy::prelude::*;
use geo_types::LineString;
use serde::{Deserialize, Serialize};
use std::f32::consts::FRAC_PI_2;
use std::ops::Sub;
use strum_macros::EnumIter;

use crate::{KxyGeodesic, MapMaterialHandle};

#[derive(Serialize, Deserialize, Debug)]
pub struct Road {
    pub class: String,
}
#[derive(EnumIter, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RoadClass {
    Motorway,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    LivingStreet,
    Trunk,
    Unclassified,
    ParkingAisle,
    Driveway,
    Pedestrian,
    Footway,
    Steps,
    Track,
    Cycleway,
    Bridleway,
    Unknown,
}
impl RoadClass {
    pub fn depth_bias(&self) -> f32 {
        match self {
            RoadClass::Motorway => 16.,
            RoadClass::Primary => 15.,
            RoadClass::Secondary => 14.,
            RoadClass::Tertiary => 13.,
            RoadClass::Residential => 12.,
            RoadClass::LivingStreet => 11.,
            RoadClass::Trunk => 10.,
            RoadClass::Unclassified => 9.,
            RoadClass::ParkingAisle => 8.,
            RoadClass::Driveway => 7.,
            RoadClass::Pedestrian => 6.,
            RoadClass::Footway => 5.,
            RoadClass::Steps => 4.,
            RoadClass::Track => 3.,
            RoadClass::Cycleway => 2.,
            RoadClass::Bridleway => 1.,
            RoadClass::Unknown => 0.1,
        }
    }
    pub fn from_string(s: &String) -> RoadClass {
        match s.as_str() {
            "motorway" => RoadClass::Motorway,
            "primary" => RoadClass::Primary,
            "secondary" => RoadClass::Secondary,
            "tertiary" => RoadClass::Tertiary,
            "residential" => RoadClass::Residential,
            "livingStreet" => RoadClass::LivingStreet,
            "trunk" => RoadClass::Trunk,
            "unclassified" => RoadClass::Unclassified,
            "parkingAisle" => RoadClass::ParkingAisle,
            "driveway" => RoadClass::Driveway,
            "pedestrian" => RoadClass::Pedestrian,
            "footway" => RoadClass::Footway,
            "steps" => RoadClass::Steps,
            "track" => RoadClass::Track,
            "cycleway" => RoadClass::Cycleway,
            "bridleway" => RoadClass::Bridleway,
            "unknown" => RoadClass::Unknown,
            _ => RoadClass::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Segment {
    pub translate: [f64; 2],
    pub line: Vec<[f64; 2]>,
    pub k: KxyGeodesic,
    pub road_class: RoadClass,
}

#[derive(Resource, Debug)]
pub struct SegmentsRes {
    pub segments: Vec<Segment>,
}
pub fn line_string_road(
    line_string: LineString,
    k: KxyGeodesic,
    center: [f64; 2],
) -> ([f64; 2], Vec<[f64; 2]>) {
    let c1 = line_string
        .coords()
        .nth(0)
        .expect("To take exterior:0 coordinate");
    let first_point_xz: [f64; 2] = [c1.x * k[0] - center[0], -c1.y * k[1] - center[1]];

    let line: Vec<[f64; 2]> = line_string
        .coords()
        .map(|c| {
            [
                c.x * k[0] - center[0] - first_point_xz[0],
                -c.y * k[1] - center[1] - first_point_xz[1],
            ]
        })
        .collect();
    (first_point_xz, line)
}

pub fn transportations_start(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transportations_res: Res<SegmentsRes>,
    map_materials: Res<MapMaterialHandle>,
) {
    for item in transportations_res.segments.iter() {
        spawn_transportation(&mut cmd, &mut meshes, &mut materials, item, &map_materials);
    }
}

impl From<&RoadClass> for Color {
    fn from(value: &RoadClass) -> Self {
        match value {
            RoadClass::Motorway => Color::srgb(0.2, 0.2, 0.2),
            RoadClass::Primary => Color::srgb(0.5, 0.5, 0.5),
            RoadClass::Secondary => Color::srgb(1.0, 1.0, 0.0),
            RoadClass::Tertiary => Color::srgb(0.98, 0.98, 0.91),
            RoadClass::Residential => Color::srgb(0.96, 0.87, 0.69),
            RoadClass::LivingStreet => Color::srgb(0.98, 0.43, 0.43),
            RoadClass::Trunk => Color::srgb(0.29, 0.0, 0.51),
            RoadClass::Unclassified => Color::srgb(1.0, 1.0, 1.0),
            RoadClass::ParkingAisle => Color::srgb(0.94, 1.0, 1.0),
            RoadClass::Driveway => Color::srgb(0.5, 0.5, 0.0),
            RoadClass::Pedestrian => Color::srgb(0.86, 0.08, 0.24),
            RoadClass::Footway => Color::srgb(1.0, 0.27, 0.0),
            RoadClass::Steps => Color::srgb(0.75, 0.75, 0.75),
            RoadClass::Track => Color::srgb(0.2, 0.8, 0.2),
            RoadClass::Cycleway => Color::srgb(0.0, 0.5, 0.0),
            RoadClass::Bridleway => Color::srgb(0.0, 0.39, 0.0),
            RoadClass::Unknown => Color::srgb(0.1, 0.1, 0.3),
        }
    }
}

type RoadWidth = f32;
impl From<&RoadClass> for RoadWidth {
    fn from(value: &RoadClass) -> RoadWidth {
        match value {
            RoadClass::Motorway => 12.,
            RoadClass::Primary => 10.,
            RoadClass::Secondary => 8.,
            RoadClass::Tertiary => 6.,
            RoadClass::Residential => 5.5,
            RoadClass::LivingStreet => 5.,
            RoadClass::Trunk => 4.5,
            RoadClass::Unclassified => 4.,
            RoadClass::ParkingAisle => 3.5,
            RoadClass::Driveway => 3.,
            RoadClass::Pedestrian => 2.5,
            RoadClass::Footway => 1.5,
            RoadClass::Steps => 1.4,
            RoadClass::Track => 1.3,
            RoadClass::Cycleway => 1.2,
            RoadClass::Bridleway => 1.1,
            RoadClass::Unknown => 1.,
        }
    }
}

pub fn spawn_transportation(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    transportation: &Segment,
    map_materials: &Res<MapMaterialHandle>,
) {
    let width = RoadWidth::from(&transportation.road_class);
    let segment = RoadSegment::new(&transportation.line, width);
    let mut mesh = Mesh::new(
        bevy_mesh::PrimitiveTopology::TriangleList,
        bevy_asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, segment.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, segment.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, segment.uvs.clone());
    mesh.insert_indices(bevy_mesh::Indices::U32(segment.indices));

    let translate: Vec3 = Vec3::new(
        transportation.translate[0] as f32,
        0.01,
        transportation.translate[1] as f32,
    );
    let transform = Transform::from_translation(translate);
    cmd.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(
            map_materials
                .road
                .get(&transportation.road_class)
                .unwrap()
                .clone(),
        ),
        transform,
    ));
}

#[derive(Component, Debug)]
pub struct RoadSegment {
    pub points: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub norm: Vec<Vec3>,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
}

impl RoadSegment {
    pub fn empty() -> Self {
        Self {
            points: vec![],
            indices: vec![],
            norm: vec![],
            vertices: vec![],
            normals: vec![],
            uvs: vec![],
        }
    }
    pub fn new(line: &Vec<[f64; 2]>, width: f32) -> Self {
        let half_width: f32 = width / 2.;
        let mut segm = Self::empty();
        segm.points = line
            .iter()
            .map(|pos| Vec3::new(pos[0] as f32, 0., pos[1] as f32))
            .collect::<Vec<Vec3>>();
        let material_length = 1.;
        let mut len: f32 = 0.;

        for (i, p) in segm.points.iter().enumerate() {
            let last: bool = i + 1 == segm.points.len();
            if last {
            } else {
                let ix2: u32 = i as u32 * 4;
                let (tri_1, tri_2) = ([ix2, ix2 + 1, ix2 + 2], [ix2 + 2, ix2 + 1, ix2 + 3]);
                segm.indices.extend(tri_1);
                segm.indices.extend(tri_2);
                segm.norm.push(Vec3::Y);

                let i_next: usize = i + 1;
                let point: Vec3 = *p;
                let point_next: Vec3 = segm.points[i_next];

                let dir: Vec3 = (point_next - point).normalize();
                let left_norm = Quat::from_rotation_y(FRAC_PI_2).mul_vec3(dir);
                let side = left_norm * half_width;
                let (l1, r1) = (point + side, point - side);
                let (l2, r2) = (point_next + side, point_next - side);
                segm.vertices.push((l1).into());
                segm.vertices.push((r1).into());
                segm.vertices.push((l2).into());
                segm.vertices.push((r2).into());

                let l_uv = len / material_length;
                segm.uvs.push([l_uv, 0.]);
                segm.uvs.push([l_uv, 0.]);
                segm.uvs.push([l_uv, 1.]);
                segm.uvs.push([l_uv, 1.]);

                let normal = segm.norm[i].to_array();
                segm.normals.push(normal);
                segm.normals.push(normal);
                segm.normals.push(normal);
                segm.normals.push(normal);

                let diff = point_next.sub(point).length();
                len += diff;
            }
        }
        segm
    }
}
