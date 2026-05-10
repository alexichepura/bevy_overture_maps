use bevy::prelude::*;
use geo::algorithm::TriangulateEarcut;
use geo_types::Polygon;
use serde::{Deserialize, Serialize};
use std::f32::consts::FRAC_PI_2;
use std::ops::Sub;
use strum_macros::EnumIter;

use crate::KxyGeodesic;
use crate::material::MapMaterialHandle;

// https://docs.overturemaps.org/reference/buildings/building
// ["residential","outbuilding","agricultural","commercial","industrial","education","service","religious","civic","transportation","medical","entertainment","military"]

#[derive(EnumIter, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum BuildingClass {
    // #[serde(rename = "residential")]
    Residential,
    // #[serde(rename = "outbuilding")]
    Outbuilding,
    // #[serde(rename = "agricultural")]
    Agricultural,
    // #[serde(rename = "commercial")]
    Commercial,
    // #[serde(rename = "industrial")]
    Industrial,
    // #[serde(rename = "education")]
    Education,
    // #[serde(rename = "service")]
    Service,
    // #[serde(rename = "religious")]
    Religious,
    // #[serde(rename = "civic")]
    Civic,
    // #[serde(rename = "transportation")]
    Transportation,
    // #[serde(rename = "medical")]
    Medical,
    // #[serde(rename = "entertainment")]
    Entertainment,
    // #[serde(rename = "military")]
    Military,
}
impl BuildingClass {
    pub fn from_string(s: &String) -> BuildingClass {
        match s.as_str() {
            "residential" => BuildingClass::Residential,
            "outbuilding" => BuildingClass::Outbuilding,
            "agricultural" => BuildingClass::Agricultural,
            "commercial" => BuildingClass::Commercial,
            "industrial" => BuildingClass::Industrial,
            "education" => BuildingClass::Education,
            "service" => BuildingClass::Service,
            "religious" => BuildingClass::Religious,
            "civic" => BuildingClass::Civic,
            "transportation" => BuildingClass::Transportation,
            "medical" => BuildingClass::Medical,
            "entertainment" => BuildingClass::Entertainment,
            "military" => BuildingClass::Military,
            _ => BuildingClass::Residential,
        }
    }
}

#[derive(Component, Debug)]
pub struct Building {
    pub class: Option<BuildingClass>,
    pub translate: [f64; 2],
    pub height: Option<f64>,
    pub num_floors: Option<i32>,
    pub line: Vec<[f64; 2]>,
    pub k: KxyGeodesic,
    pub vertices: Vec<[f32; 3]>,
    pub triangle_indices: Vec<u32>,
}

impl Building {
    pub fn from_props(props: BuildingGeometryProps, class: Option<BuildingClass>) -> Self {
        Building {
            class,
            translate: props.translate,
            height: props.height,
            num_floors: props.num_floors,
            line: props.line,
            k: props.k,
            vertices: props.vertices,
            triangle_indices: props.triangle_indices,
        }
    }
}
#[derive(Debug)]
pub struct BuildingGeometryProps {
    pub translate: [f64; 2],
    pub height: Option<f64>,
    pub num_floors: Option<i32>,
    pub line: Vec<[f64; 2]>,
    pub k: KxyGeodesic,
    pub vertices: Vec<[f32; 3]>,
    pub triangle_indices: Vec<u32>,
}

#[derive(Resource, Debug)]
pub struct Buildings {
    pub buildings: Vec<Building>,
}
// pub fn polygon_base(polygon: &Polygon) -> (f64, [f64; 2]) {
//     let exterior = polygon.exterior();
//     let c1 = exterior
//         .coords()
//         .nth(0)
//         .expect("To take exterior:0 coordinate");
//     let p1 = geo::Point(*c1);
//     let c2 = exterior
//         .coords()
//         .nth(1)
//         .expect("To take exterior:1 coordinate");
//     let p2 = geo::Point(*c2);
//     let geodesic_distance = p1.geodesic_distance(&p2);
//     let coord_distance = c1.add(c2.neg()).magnitude();
//     let k = geodesic_distance / coord_distance;
//     let first_point_position: [f64; 2] = [c1.x * k, c1.y * k];
//     (k, first_point_position)
// }

pub fn polygon_building(
    polygon: Polygon,
    k: KxyGeodesic,
    center: [f64; 2],
    height: Option<f64>,
    num_floors: Option<i32>,
) -> BuildingGeometryProps {
    let exterior = polygon.exterior();
    let c1 = exterior
        .coords()
        .nth(0)
        .expect("To take exterior:0 coordinate");

    let translate: [f64; 2] = [c1.x * k[0] - center[0], -c1.y * k[1] - center[1]]; // Yto-Z

    let line: Vec<[f64; 2]> = exterior
        .coords()
        .map(|c| {
            [
                c.x * k[0] - center[0] - translate[0],
                -c.y * k[1] - center[1] - translate[1], // Yto-Z
            ]
        })
        .collect();

    // println!("line l:{} :{:?}", line.len(), &line);
    // for (i, l) in line.iter().enumerate() {
    //     if i > 0 {
    //         let dx = l[0] - line[i - 1][0];
    //         if dx > 1000. {
    //             println!("{i}dx>100:{dx}:{:?}", &line);
    //         }
    //         let dy = l[1] - line[i - 1][1];
    //         if dy > 1000. {
    //             println!("{i}dy>100:{dy}:{:?}", &line);
    //         }
    //     }
    // }
    // let is_last_eq_first = line.first() == line.last();
    // if !is_last_eq_first {
    //     println!("is_last_eq_first:{is_last_eq_first}-{:?}", &line);
    // }

    let triangles = polygon.earcut_triangles_raw();
    BuildingGeometryProps {
        translate,
        height,
        num_floors,
        line,
        k,
        vertices: triangles
            .vertices
            .iter()
            .map(|i| {
                [
                    (i[0] * k[0] - center[0] - translate[0]) as f32,
                    0.,
                    (-i[1] * k[1] - center[1] - translate[1]) as f32,
                ]
            })
            .collect(),
        triangle_indices: triangles
            .triangle_indices
            .iter()
            .map(|i| *i as u32)
            .collect(),
    }
}

pub fn buildings_start(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_materials: Res<MapMaterialHandle>,
    buildings_res: Res<Buildings>,
) {
    for b in buildings_res.buildings.iter() {
        spawn_building(&mut cmd, &mut meshes, &mut materials, b, &map_materials);
    }
}

impl From<&BuildingClass> for Color {
    fn from(building_class: &BuildingClass) -> Self {
        match building_class {
            BuildingClass::Residential => Color::srgb(0.5, 0.45, 0.4),
            BuildingClass::Outbuilding => Color::srgb(0.25, 0.25, 0.25),
            BuildingClass::Agricultural => Color::srgb(0.0, 0.5, 0.0),
            BuildingClass::Commercial => Color::srgb(0.3, 0.3, 0.4),
            BuildingClass::Industrial => Color::srgb(0.75, 0.75, 0.75),
            BuildingClass::Education => Color::srgb(0.98, 0.98, 0.91),
            BuildingClass::Service => Color::srgb(1.0, 0.73, 0.47),
            BuildingClass::Religious => Color::srgb(0.5, 0.93, 0.73),
            BuildingClass::Civic => Color::srgb(0.6, 0.6, 0.8),
            BuildingClass::Transportation => Color::srgb(0.8, 0.2, 0.8),
            BuildingClass::Medical => Color::srgb(1.0, 0.27, 0.0),
            BuildingClass::Entertainment => Color::srgb(0.94, 1.0, 1.0),
            BuildingClass::Military => Color::srgb(0.0, 0.0, 0.5),
        }
    }
}

pub fn _buildings_update(buildings_res: Res<Buildings>, mut gizmos: Gizmos) {
    for b in buildings_res.buildings.iter() {
        let height: f32 = match b.height {
            Some(h) => h as f32,
            None => 10.,
        };
        let wall = Wall::new(&b.line, height);

        for (i, n) in wall.normals.iter().enumerate() {
            let tr = Vec3::new(b.translate[0] as f32, 0., b.translate[1] as f32);
            let n = Vec3::new(n[0], n[1], n[2]);
            let v = wall.vertices[i];
            let v = Vec3::new(v[0], v[1] + 0.01, v[2]);
            gizmos.line(tr + v, tr + v + n, Color::srgb(0.5, 0.3, 0.3));
        }
    }
}

pub fn spawn_building(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    building: &Building,
    map_materials: &Res<MapMaterialHandle>,
) {
    let height: f32 = match building.height {
        Some(h) => h as f32,
        None => match building.num_floors {
            Some(floors) => floors as f32 * 3.,
            None => 10.,
        },
    };

    let wall = Wall::new(&building.line, height);
    let mut mesh = Mesh::new(
        bevy_mesh::PrimitiveTopology::TriangleList,
        bevy_asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, wall.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, wall.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, wall.uvs.clone());
    mesh.insert_indices(bevy_mesh::Indices::U32(wall.indices));

    let translate: Vec3 = Vec3::new(
        building.translate[0] as f32,
        0.,
        building.translate[1] as f32,
    );
    let transform = Transform::from_translation(translate);
    let handle: Handle<StandardMaterial> = match &building.class {
        Some(c) => map_materials.walls.get(c).unwrap().clone(),
        None => map_materials.unknown_building.clone(),
    };
    cmd.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(handle.clone()),
        transform,
    ));

    // ROOF
    let mut roof = Mesh::new(
        bevy_mesh::PrimitiveTopology::TriangleList,
        bevy_asset::RenderAssetUsages::RENDER_WORLD,
    );
    let vertices: Vec<[f32; 3]> = building
        .vertices
        .iter()
        .map(|v| v.map(|p| p as f32))
        .collect();
    roof.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
    roof.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        building
            .vertices
            .iter()
            .map(|_| [0., 1., 0.] as [f32; 3])
            .collect::<Vec<[f32; 3]>>(),
    );
    let uvs: Vec<[f32; 2]> = vertices.iter().map(|p| [p[0], p[2]]).collect();
    roof.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    let bs = building.triangle_indices.clone();
    roof.insert_indices(bevy_mesh::Indices::U32(bs));

    let translation = transform.translation + Vec3::new(0., height, 0.);
    let transform: Transform = Transform::from_translation(translation);

    let handle: Handle<StandardMaterial> = match &building.class {
        Some(c) => map_materials.roofs.get(c).unwrap().clone(),
        None => map_materials.unknown_building_roof.clone(),
    };
    cmd.spawn((Mesh3d(meshes.add(roof)), MeshMaterial3d(handle), transform));
}

#[derive(Component, Debug)]
pub struct Wall {
    pub points: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub norm: Vec<Vec3>,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
}

impl Wall {
    pub fn empty() -> Self {
        Wall {
            points: vec![],
            indices: vec![],
            norm: vec![],
            vertices: vec![],
            normals: vec![],
            uvs: vec![],
        }
    }
    pub fn new(line: &Vec<[f64; 2]>, height: f32) -> Self {
        let mut wall = Wall::empty();
        wall.points = line
            .iter()
            .map(|pos| Vec3::new(pos[0] as f32, 0., pos[1] as f32))
            .collect::<Vec<Vec3>>();

        let heightv: Vec3 = Vec3::Y * height;
        let material_lengh = 1.;
        let mut len: f32 = 0.;
        let points_len = wall.points.len();

        for (i, p) in wall.points.iter().enumerate() {
            // println!("{:?}", &point);
            let last: bool = i + 1 == points_len;
            let ix2: u32 = i as u32 * 4;
            if last {
            } else {
                let (i1, i2) = ([ix2, ix2 + 2, ix2 + 1], [ix2 + 2, ix2 + 3, ix2 + 1]); // Yto-Z
                wall.indices.extend(i1);
                wall.indices.extend(i2);
                let point_next = wall.points[i + 1];
                let dir: Vec3 = (point_next - *p).normalize();
                // println!("{:?}", &dir);
                let norm = Quat::from_rotation_y(-FRAC_PI_2).mul_vec3(dir); // Yto-Z
                wall.norm.push(norm);
                let i_next: usize = if last { 0 } else { i + 1 };
                let point: Vec3 = *p;
                let point_next: Vec3 = wall.points[i_next];
                wall.vertices.push((point).into());
                wall.vertices.push((point + heightv).into());
                wall.vertices.push((point_next).into());
                wall.vertices.push((point_next + heightv).into());

                let diff = point_next.sub(point).length();
                wall.uvs.push([len / material_lengh, 0.]);
                wall.uvs.push([len / material_lengh, 1.]);
                wall.uvs.push([len / material_lengh, 0.]);
                wall.uvs.push([len / material_lengh, 1.]);

                let norm_arr = norm.to_array();
                wall.normals.push(norm_arr);
                wall.normals.push(norm_arr);
                wall.normals.push(norm_arr);
                wall.normals.push(norm_arr);
                len += diff;
            }
        }
        wall
    }
}
