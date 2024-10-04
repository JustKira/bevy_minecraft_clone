use bevy::color::palettes::css::{BLUE, RED, WHITE};
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
    const CHUNK_SIZE: u32 = 64;

    let mut noise = FastNoiseLite::with_seed(1337);

    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(5));
    noise.set_frequency(Some(0.035));
    noise.set_fractal_weighted_strength(Some(0.5));
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));

    let mut vertices: Vec<[f32; 3]> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();

    let mut cell = 0;
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let noise_normalized =
                ((noise.get_noise_2d(x as f32, y as f32) + 1.0) / 2.0).round() as u32;

            let height = if noise_normalized == 0 { 4.0 } else { 0.0 };

            // Generate the main cell surface
            let bl = [x as f32, height, y as f32];
            let br = [(x + 1) as f32, height, y as f32];
            let tl = [x as f32, height, (y + 1) as f32];
            let tr = [(x + 1) as f32, height, (y + 1) as f32];

            vertices.push(bl);
            vertices.push(br);
            vertices.push(tl);
            vertices.push(tr);
            // First triangle 0 2 1
            indices.push(cell);
            indices.push(cell + 2);
            indices.push(cell + 1);
            // Second triangle 1 2 3
            indices.push(cell + 1);
            indices.push(cell + 2);
            indices.push(cell + 3);

            cell += 4;

            // Check neighbors to place walls

            // Check north (y + 1)
            if y + 1 < CHUNK_SIZE {
                let north_height =
                    ((noise.get_noise_2d(x as f32, (y + 1) as f32) + 1.0) / 2.0).round() as u32;
                if north_height != noise_normalized {
                    // Create a north wall
                    let bottom_left = [x as f32, height, (y + 1) as f32];
                    let bottom_right = [(x + 1) as f32, height, (y + 1) as f32];
                    let top_left = [x as f32, 4.0, (y + 1) as f32];
                    let top_right = [(x + 1) as f32, 4.0, (y + 1) as f32];
                    // Push wall vertices and indices
                    vertices.push(bottom_left);
                    vertices.push(bottom_right);
                    vertices.push(top_left);
                    vertices.push(top_right);
                    indices.push(cell);
                    indices.push(cell + 2);
                    indices.push(cell + 1);
                    indices.push(cell + 1);
                    indices.push(cell + 2);
                    indices.push(cell + 3);
                    cell += 4;
                }
            }

            // Check east (x + 1)
            if x + 1 < CHUNK_SIZE {
                let east_height =
                    ((noise.get_noise_2d((x + 1) as f32, y as f32) + 1.0) / 2.0).round() as u32;
                if east_height != noise_normalized {
                    // Create an east wall
                    let bottom_left = [(x + 1) as f32, height, y as f32];
                    let bottom_right = [(x + 1) as f32, height, (y + 1) as f32];
                    let top_left = [(x + 1) as f32, 4.0, y as f32];
                    let top_right = [(x + 1) as f32, 4.0, (y + 1) as f32];
                    // Push wall vertices and indices
                    vertices.push(bottom_left);
                    vertices.push(bottom_right);
                    vertices.push(top_left);
                    vertices.push(top_right);
                    indices.push(cell);
                    indices.push(cell + 1);
                    indices.push(cell + 2);
                    indices.push(cell + 2);
                    indices.push(cell + 1);
                    indices.push(cell + 3);
                    cell += 4;
                }
            }

            // Check south (y - 1)
            if y > 0 {
                let south_height =
                    ((noise.get_noise_2d(x as f32, (y - 1) as f32) + 1.0) / 2.0).round() as u32;
                if south_height != noise_normalized {
                    // Create a south wall
                    let bottom_left = [x as f32, height, y as f32];
                    let bottom_right = [(x + 1) as f32, height, y as f32];
                    let top_left = [x as f32, 4.0, y as f32];
                    let top_right = [(x + 1) as f32, 4.0, y as f32];
                    // Push wall vertices and indices
                    vertices.push(bottom_left);
                    vertices.push(bottom_right);
                    vertices.push(top_left);
                    vertices.push(top_right);
                    indices.push(cell);
                    indices.push(cell + 1);
                    indices.push(cell + 2);
                    indices.push(cell + 2);
                    indices.push(cell + 1);
                    indices.push(cell + 3);
                    cell += 4;
                }
            }

            // Check west (x - 1)
            if x > 0 {
                let west_height =
                    ((noise.get_noise_2d((x - 1) as f32, y as f32) + 1.0) / 2.0).round() as u32;
                if west_height != noise_normalized {
                    // Create a west wall
                    let bottom_left = [x as f32, height, y as f32];
                    let bottom_right = [x as f32, height, (y + 1) as f32];
                    let top_left = [x as f32, 4.0, y as f32];
                    let top_right = [x as f32, 4.0, (y + 1) as f32];
                    // Push wall vertices and indices
                    vertices.push(bottom_left);
                    vertices.push(bottom_right);
                    vertices.push(top_left);
                    vertices.push(top_right);
                    indices.push(cell);
                    indices.push(cell + 2);
                    indices.push(cell + 1);
                    indices.push(cell + 1);
                    indices.push(cell + 2);
                    indices.push(cell + 3);
                    cell += 4;
                }
            }
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
