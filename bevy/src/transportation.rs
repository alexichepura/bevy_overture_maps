use bevy::{prelude::*, render::mesh::*};
use geo::algorithm::TriangulateEarcut;
use geo::algorithm::Vector2DOps;
use geo::geodesic_distance::GeodesicDistance;
use geo_types::LineString;
use geo_types::Polygon;
use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Neg, Sub};

#[derive(Component, Debug)]
pub struct BevyTransportation {
    pub translate: [f64; 2],
    pub line: Vec<[f64; 2]>,
    pub k: f64,
    // pub vertices: Vec<[f64; 3]>,
    // pub triangle_indices: Vec<u32>,
}

#[derive(Resource, Debug)]
pub struct BevyTransportations {
    pub transportations: Vec<BevyTransportation>,
}
pub fn line_string_road(line_string: LineString, k: f64, base: [f64; 2]) -> BevyTransportation {
    let c1 = line_string
        .coords()
        .nth(0)
        .expect("To take exterior:0 coordinate");
    let translate: [f64; 2] = [c1.x * k - base[0], c1.y * k - base[1]];

    let line: Vec<[f64; 2]> = line_string
        .coords()
        .map(|c| {
            [
                c.x * k - base[0] - translate[0],
                c.y * k - base[1] - translate[1],
            ]
        })
        .collect();
    BevyTransportation { translate, line, k }
}
pub fn line_string_base(line_string: &LineString) -> (f64, [f64; 2]) {
    let c1 = line_string
        .coords()
        .nth(0)
        .expect("To take line_string:0 coordinate");
    let p1 = geo::Point(*c1);
    let c2 = line_string
        .coords()
        .nth(1)
        .expect("To take line_string:1 coordinate");
    let p2 = geo::Point(*c2);
    let geodesic_distance = p1.geodesic_distance(&p2);
    let coord_distance = c1.add(c2.neg()).magnitude();
    let k = geodesic_distance / coord_distance;
    let first_point_position: [f64; 2] = [c1.x * k, c1.y * k];
    (k, first_point_position)
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
    let segment = RoadSegment::new(&transportation.line);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(segment.vertices),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::from(segment.normals),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::from(segment.uvs),
    );
    mesh.set_indices(Some(Indices::U32(segment.indices)));

    let translate: Vec3 = Vec3::new(
        transportation.translate[0] as f32,
        0.,
        transportation.translate[1] as f32,
    );
    let transform = Transform::from_translation(translate);
    cmd.spawn((PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.1, 0.1, 0.3).into()),
        transform,
        ..Default::default()
    },));
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
    pub fn new(line: &Vec<[f64; 2]>) -> Self {
        let mut road_segment = RoadSegment::empty();

        road_segment
    }
}
