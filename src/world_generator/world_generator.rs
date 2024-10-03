use bevy::color::palettes::css::WHITE;
use bevy::pbr::wireframe::WireframeColor;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
use fastnoise_lite::*;

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
    const CHUNK_SIZE: u32 = 16;

    let mut noise = FastNoiseLite::with_seed(1325);

    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(5));
    noise.set_frequency(Some(0.035));
    noise.set_fractal_weighted_strength(Some(-0.5));
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));

    let mut vertices: Vec<[f32; 3]> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();

    for x in 0..CHUNK_SIZE + 1 {
        for y in 0..CHUNK_SIZE + 1 {
            let ne = (noise.get_noise_2d(x as f32, (y as i32 + 1) as f32) + 1.0) / 2.0;
            let nw = (noise.get_noise_2d((x as i32 - 1) as f32, (y as i32 + 1) as f32) + 1.0) / 2.0;
            let se = (noise.get_noise_2d(x as f32, y as f32) + 1.0) / 2.0;
            let sw = (noise.get_noise_2d((x as i32 - 1) as f32, y as f32) + 1.0) / 2.0;

            let mut debug_noise: Vec<[f32; 2]> = Vec::new();
            let avg = (ne.round() + nw.round() + se.round() + sw.round()) / 4.0;

            if avg >= 0.25 {
                debug_noise.push([nw.round(), ne.round()]);
                debug_noise.push([sw.round(), se.round()]);

                println!("[{:?},{:?}] : {:?}", x, y, debug_noise);
                vertices.push([x as f32, 0.0, y as f32]);

                let debug_cube = debug_cube();

                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(debug_cube), // Add the custom cube mesh
                        transform: Transform::from_xyz(x as f32, 0.0, y as f32)
                            .with_scale(Vec3::from([0.25, 0.25, 0.25])),
                        ..default()
                    },
                    WireframeColor {
                        color: Color::WHITE,
                    },
                ));
            } else {
                vertices.push([x as f32, 0.0, y as f32]);
            }
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
            color: Color::BLACK,
        },
    ));
}

fn debug_cube() -> Mesh {
    let vertices = vec![
        [-0.5, -0.5, -0.5], // 0: left  bottom back
        [0.5, -0.5, -0.5],  // 1: right bottom back
        [0.5, 0.5, -0.5],   // 2: right top    back
        [-0.5, 0.5, -0.5],  // 3: left  top    back
        [-0.5, -0.5, 0.5],  // 4: left  bottom front
        [0.5, -0.5, 0.5],   // 5: right bottom front
        [0.5, 0.5, 0.5],    // 6: right top    front
        [-0.5, 0.5, 0.5],   // 7: left  top    front
    ];

    let indices = vec![
        // Front face
        4, 5, 6, 4, 6, 7, // Back face
        1, 0, 3, 1, 3, 2, // Left face
        0, 4, 7, 0, 7, 3, // Right face
        5, 1, 2, 5, 2, 6, // Top face
        3, 7, 6, 3, 6, 2, // Bottom face
        4, 0, 1, 4, 1, 5,
    ];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices));

    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    return mesh;
}
