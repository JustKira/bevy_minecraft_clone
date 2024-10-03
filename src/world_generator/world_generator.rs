use bevy::color::palettes::css::WHITE;
use bevy::pbr::wireframe::WireframeColor;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
// use fastnoise_lite::*;

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
    const CHUNK_SIZE: u32 = 32;

    // let mut noise = FastNoiseLite::with_seed(1325);

    // noise.set_fractal_type(Some(FractalType::FBm));
    // noise.set_fractal_octaves(Some(5));
    // noise.set_frequency(Some(0.035));
    // noise.set_fractal_weighted_strength(Some(-0.5));
    // noise.set_noise_type(Some(NoiseType::OpenSimplex2));

    // let mut chunk: [[u8; 4]; 4] = [[0; 4]; 4];

    let mut vertices: Vec<[f32; 3]> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();

    for x in 0..CHUNK_SIZE + 1 {
        for y in 0..CHUNK_SIZE + 1 {
            vertices.push([x as f32, 0.0, y as f32]);
        }
    }

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let top_left = y * (CHUNK_SIZE + 1) + x;
            let top_right = top_left + 1;
            let bottom_left = (y + 1) * (CHUNK_SIZE + 1) + x;
            let bottom_right = bottom_left + 1;

            // First triangle
            indices.push(bottom_right);
            indices.push(bottom_left);
            indices.push(top_left);

            indices.push(top_right);
            indices.push(bottom_right);
            indices.push(top_left);
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
