use bevy::{pbr::NotShadowCaster, prelude::*, render::mesh::*};
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
    // highway=motorway > trunk > primary > secondary > ... > living streets > ... > footway
    Motorway,     // - motorway
    Primary,      // - primary
    Secondary,    // - secondary
    Tertiary,     // - tertiary
    Residential,  // - residential
    LivingStreet, // - livingStreet # similar as residential but has implied legal restriction for motor vehicles (which can vary country by country)
    Trunk,        // - trunk
    Unclassified, // - unclassified # known roads, paved but of low importance which does not meet definition of being motorway, trunk, primary, secondary, tertiary
    ParkingAisle, // - parkingAisle # service road intended for parking
    Driveway,     // - driveway # service road intended for deliveries
    Pedestrian,   // - pedestrian
    Footway,      // - footway
    Steps,        // - steps
    Track,        // - track
    Cycleway,     // - cycleway
    Bridleway,    // - bridleway # similar as track but has implied access only for horses
    Unknown,      // - unknown
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
    // pub vertices: Vec<[f64; 3]>,
    // pub triangle_indices: Vec<u32>,
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
    let first_point_xz: [f64; 2] = [c1.x * k[0] - center[0], -c1.y * k[1] - center[1]]; // Yto-Z

    let line: Vec<[f64; 2]> = line_string
        .coords()
        .map(|c| {
            [
                c.x * k[0] - center[0] - first_point_xz[0],
                -c.y * k[1] - center[1] - first_point_xz[1], // Yto-Z
            ]
        })
        .collect();
    (first_point_xz, line)
}
// pub fn line_string_base(line_string: &LineString) -> (f64, [f64; 2]) {
//     let c1 = line_string
//         .coords()
//         .nth(0)
//         .expect("To take line_string:0 coordinate");
//     let p1 = geo::Point(*c1);
//     let c2 = line_string
//         .coords()
//         .nth(1)
//         .expect("To take line_string:1 coordinate");
//     let p2 = geo::Point(*c2);
//     let geodesic_distance = p1.geodesic_distance(&p2);
//     let coord_distance = c1.add(c2.neg()).magnitude();
//     let k = geodesic_distance / coord_distance;
//     let first_point_position: [f64; 2] = [c1.x * k, c1.y * k];
//     (k, first_point_position)
// }

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
            RoadClass::Motorway => Color::DARK_GRAY,
            RoadClass::Primary => Color::GRAY,
            RoadClass::Secondary => Color::YELLOW,
            RoadClass::Tertiary => Color::ANTIQUE_WHITE,
            RoadClass::Residential => Color::BEIGE,
            RoadClass::LivingStreet => Color::SALMON,
            RoadClass::Trunk => Color::INDIGO,
            RoadClass::Unclassified => Color::WHITE,
            RoadClass::ParkingAisle => Color::AZURE,
            RoadClass::Driveway => Color::OLIVE,
            RoadClass::Pedestrian => Color::CRIMSON,
            RoadClass::Footway => Color::ORANGE_RED,
            RoadClass::Steps => Color::SILVER,
            RoadClass::Track => Color::LIME_GREEN,
            RoadClass::Cycleway => Color::GREEN,
            RoadClass::Bridleway => Color::DARK_GREEN,
            RoadClass::Unknown => Color::rgb(0.1, 0.1, 0.3),
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
        0.01,
        transportation.translate[1] as f32,
    );
    let transform = Transform::from_translation(translate);
    // let color = Color::from(&transportation.road_class);
    // let material = StandardMaterial {
    //     base_color: color,
    //     depth_bias: transportation.road_class.depth_bias() * 100.,
    //     reflectance: 0.5,
    //     perceptual_roughness: 0.7,
    //     ..default()
    // };

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            // material: materials.add(material),
            material: map_materials
                .road
                .get(&transportation.road_class)
                .unwrap()
                .clone(),
            transform,
            ..Default::default()
        },
        NotShadowCaster,
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
        // let mut road_segment = RoadSegment::empty();
        let mut segm = Self::empty();
        segm.points = line
            .iter()
            .map(|pos| Vec3::new(pos[0] as f32, 0., pos[1] as f32))
            .collect::<Vec<Vec3>>();

        // let heightv: Vec3 = Vec3::Y * height;
        let material_lengh = 1.;
        let mut len: f32 = 0.;

        for (i, p) in segm.points.iter().enumerate() {
            // println!("{:?}", &point);
            let last: bool = i + 1 == segm.points.len();
            let ix2: u32 = i as u32 * 2;
            if last {
                let inx = if last { 0 } else { i + 1 };
                segm.norm.push(segm.norm[inx]);
            } else {
                let (i1, i2) = ([ix2, ix2 + 1, ix2 + 2], [ix2 + 2, ix2 + 1, ix2 + 3]);
                segm.indices.extend(i1);
                segm.indices.extend(i2);
                // let point_next = segm.points[i + 1];
                // let dir: Vec3 = (point_next - *p).normalize();
                // println!("{:?}", &dir);
                // let left_norm = Quat::from_rotation_y(FRAC_PI_2).mul_vec3(dir);
                // segm.norm.push(left_norm);
                segm.norm.push(Vec3::Y);
            }

            let i_next: usize = if last { 0 } else { i + 1 };
            let normal = segm.norm[i];
            let point: Vec3 = *p;
            let point_next: Vec3 = segm.points[i_next];

            let dir: Vec3 = (point_next - point).normalize();
            let left_norm = Quat::from_rotation_y(FRAC_PI_2).mul_vec3(dir);
            let right_norm = -left_norm;

            // track.left.push(point + left_norm * width);
            // track.right.push(point + right_norm * width);
            let l1 = point + left_norm * half_width;
            let r1 = point + right_norm * half_width;
            let l2 = point_next + left_norm * half_width;
            let r2 = point_next + right_norm * half_width;
            segm.vertices.push((l1).into());
            segm.vertices.push((r1).into());
            segm.vertices.push((l2).into());
            segm.vertices.push((r2).into());

            // segm.vertices.push((point).into());
            // segm.vertices.push((point + heightv).into());
            // segm.vertices.push((point_next).into());
            // segm.vertices.push((point_next + heightv).into());

            let diff = point_next.sub(point).length();
            segm.uvs.push([len / material_lengh, 0.]);
            segm.uvs.push([len / material_lengh, 1.]);
            segm.uvs.push([len / material_lengh, 0.]);
            segm.uvs.push([len / material_lengh, 1.]);
            segm.normals.push(normal.to_array());
            segm.normals.push(normal.to_array());
            segm.normals.push(normal.to_array());
            segm.normals.push(normal.to_array());
            len += diff;
        }
        let points_len = segm.points.len() as u32;
        segm.indices
            .extend(segm.indices.clone().iter().map(|ind| ind + points_len * 2));

        // let points_len = wall.points.len() as u32;
        // let mut indices: Vec<u32> = vec![];
        // indices.extend(wall.indices.clone());
        // // indices.extend(wall.indices.iter().map(|ind| ind + points_len * 2));
        // // indices.extend(wall.indices.iter().map(|ind| ind + points_len * 4));
        // wall.indices = indices;

        segm
    }
}
