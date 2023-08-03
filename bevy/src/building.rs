use bevy::{prelude::*, render::mesh::*};
use geo::algorithm::TriangulateEarcut;
use geo::algorithm::Vector2DOps;
use geo::geodesic_distance::GeodesicDistance;
use geo_types::Polygon;
use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Neg, Sub};

#[derive(Component, Debug)]
pub struct BevyBuilding {
    pub translate: [f64; 2],
    pub height: Option<f64>,
    pub line: Vec<[f64; 2]>,
    pub k: f64,
    pub vertices: Vec<[f64; 3]>,
    pub triangle_indices: Vec<u32>,
}

#[derive(Resource, Debug)]
pub struct BevyBuildings {
    pub buildings: Vec<BevyBuilding>,
}
pub fn polygon_base(polygon: &Polygon) -> (f64, [f64; 2]) {
    let exterior = polygon.exterior();
    let c1 = exterior
        .coords()
        .nth(0)
        .expect("To take exterior:0 coordinate");
    let p1 = geo::Point(*c1);
    let c2 = exterior
        .coords()
        .nth(1)
        .expect("To take exterior:1 coordinate");
    let p2 = geo::Point(*c2);
    let geodesic_distance = p1.geodesic_distance(&p2);
    let coord_distance = c1.add(c2.neg()).magnitude();
    let k = geodesic_distance / coord_distance;
    let first_point_position: [f64; 2] = [c1.x * k, c1.y * k];
    (k, first_point_position)
}

pub fn polygon_building(
    polygon: Polygon,
    k: f64,
    base: [f64; 2],
    height: Option<f64>,
) -> BevyBuilding {
    let exterior = polygon.exterior();
    let c1 = exterior
        .coords()
        .nth(0)
        .expect("To take exterior:0 coordinate");
    let translate: [f64; 2] = [c1.x * k - base[0], c1.y * k - base[1]];

    let line: Vec<[f64; 2]> = exterior
        .coords()
        .map(|c| {
            [
                c.x * k - base[0] - translate[0],
                c.y * k - base[1] - translate[1],
            ]
        })
        .collect();
    let triangles = polygon.earcut_triangles_raw();
    BevyBuilding {
        translate,
        height,
        line,
        k,
        vertices: triangles
            .vertices
            .chunks(2)
            .map(|i| {
                [
                    i[0] * k - base[0] - translate[0],
                    0.,
                    i[1] * k - base[1] - translate[1],
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
    buildings_res: Res<BevyBuildings>,
) {
    for b in buildings_res.buildings.iter() {
        spawn_building(&mut cmd, &mut meshes, &mut materials, b);
    }
}

// pub fn buildings_update(buildings_res: Res<BevyBuildings>, mut gizmos: Gizmos) {
//     for b in buildings_res.buildings.iter() {
//         let height: f32 = match b.height {
//             Some(h) => h as f32,
//             None => 10.,
//         };
//         let wall = Wall::new(&b.line, height);

//         for (i, n) in wall.normals.iter().enumerate() {
//             let n = Vec3::new(n[0], n[1], n[2]);
//             let v = wall.vertices[i];
//             let v = Vec3::new(v[0], v[1] + 0.01, v[2]);
//             gizmos.line(v, v + n, Color::rgb(0.5, 0.3, 0.3));
//         }
//     }
// }

pub fn spawn_building(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building: &BevyBuilding,
) {
    let height: f32 = match building.height {
        Some(h) => h as f32,
        None => 10.,
    };

    let wall = Wall::new(&building.line, height);

    // println!(
    //     "building - translate: {:?}, line: {}, vertices: {}, indices: {}, normals: {}",
    //     b.translate,
    //     b.line.len(),
    //     wall.vertices.len(),
    //     wall.indices.len(),
    //     wall.normals.len(),
    // );
    // println!("indices: {:?}", &wall.indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(wall.vertices),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::from(wall.normals),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(wall.uvs));
    mesh.set_indices(Some(Indices::U32(wall.indices)));

    let translate: Vec3 = Vec3::new(
        building.translate[0] as f32,
        0.,
        building.translate[1] as f32,
    );
    let transform = Transform::from_translation(translate);
    cmd.spawn((PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.5, 0.5, 0.3).into()),
        transform,
        ..Default::default()
    },));

    // ROOF
    let mut roof = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices: Vec<[f32; 3]> = building
        .vertices
        .iter()
        .map(|v| v.map(|p| p as f32))
        .collect();
    roof.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    roof.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::from(
            building
                .vertices
                .iter()
                .map(|_| [0., 1., 0.] as [f32; 3])
                .collect::<Vec<[f32; 3]>>(),
        ),
    );
    let uvs: Vec<[f32; 2]> = vertices.clone().iter().map(|p| [p[0], p[2]]).collect();
    roof.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    let bs = building.triangle_indices.clone();
    let rev: Vec<u32> = bs.into_iter().rev().collect();
    roof.set_indices(Some(Indices::U32(rev)));

    let translation = transform.translation + Vec3::new(0., height, 0.);
    let transform: Transform = Transform::from_translation(translation);
    cmd.spawn((PbrBundle {
        mesh: meshes.add(roof),
        material: materials.add(Color::rgb(0.3, 0.3, 0.2).into()),
        transform,
        ..Default::default()
    },));
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

        for (i, p) in wall.points.iter().enumerate() {
            // println!("{:?}", &point);
            let last: bool = i + 1 == wall.points.len();
            let ix2: u32 = i as u32 * 2;
            if last {
                let inx = if last { 0 } else { i + 1 };
                wall.norm.push(wall.norm[inx]);
            } else {
                let (i1, i2) = ([ix2, ix2 + 1, ix2 + 2], [ix2 + 2, ix2 + 1, ix2 + 3]);
                wall.indices.extend(i1);
                wall.indices.extend(i2);
                let point_next = wall.points[i + 1];
                let dir: Vec3 = (point_next - *p).normalize();
                // println!("{:?}", &dir);
                let left_norm = Quat::from_rotation_y(FRAC_PI_2).mul_vec3(dir);
                wall.norm.push(left_norm);
            }

            let i_next: usize = if last { 0 } else { i + 1 };
            let normal = wall.norm[i];
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
            wall.normals.push(normal.to_array());
            wall.normals.push(normal.to_array());
            wall.normals.push(normal.to_array());
            wall.normals.push(normal.to_array());
            len += diff;
        }
        let points_len = wall.points.len() as u32;
        wall.indices
            .extend(wall.indices.clone().iter().map(|ind| ind + points_len * 2));

        // let points_len = wall.points.len() as u32;
        // let mut indices: Vec<u32> = vec![];
        // indices.extend(wall.indices.clone());
        // // indices.extend(wall.indices.iter().map(|ind| ind + points_len * 2));
        // // indices.extend(wall.indices.iter().map(|ind| ind + points_len * 4));
        // wall.indices = indices;

        wall
    }
}
