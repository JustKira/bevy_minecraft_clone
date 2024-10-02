use bevy::color::palettes::css::WHITE;
use bevy::pbr::wireframe::WireframeColor;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_chunk);
    }
}

fn generate_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Generating chunk...");
    let noise_map = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 0],
        vec![1, 0, 1, 1],
        vec![0, 0, 1, 0],
    ];

    // let mut chunk: [[u8; 4]; 4] = [[0; 4]; 4];

    let mut vertices: Vec<[f32; 3]> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();

    let mut cell = 0;

    for (y, row) in noise_map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let bl = [x as f32, 0.0, y as f32];
            let br = [(x + 1) as f32, 0.0, y as f32];
            let tl = [x as f32, 0.0, (y + 1) as f32];
            let tr = [(x + 1) as f32, 0.0, (y + 1) as f32];

            vertices.push(bl);
            vertices.push(br);
            vertices.push(tl);
            vertices.push(tr);

            //first triangle 0 2 1
            indices.push(cell);
            indices.push(cell + 2);
            indices.push(cell + 1);

            //second triangle 1 2 3
            indices.push(cell + 1);
            indices.push(cell + 2);
            indices.push(cell + 3);

            cell += 4;

            // chunk[y][x] = *value;
        }
    }

    println!("{:?}", vertices);
    println!("{:?}", indices);

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh), // Add the custom cube mesh
            material: materials.add(Color::from(WHITE)),
            ..default()
        },
        WireframeColor {
            color: Color::WHITE,
        },
    ));
}
