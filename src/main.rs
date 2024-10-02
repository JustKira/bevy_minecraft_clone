mod world_generator;

use bevy::color::palettes::css::WHITE;

use bevy::pbr::wireframe::{WireframeColor, WireframeConfig, WireframePlugin};
use bevy::prelude::*;

use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use world_generator::world_generator::WorldGeneratorPlugin;

mod bevy_basic_camera;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WireframePlugin,
            WorldGeneratorPlugin,
            CameraControllerPlugin,
        ))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        }) // Loads default Bevy plugins (rendering, input, etc.)
        .add_systems(Startup, (setup, setup_cube))
        // .add_systems(Update, rotate_cube) // Startup systems for camera and cube
        .run();
}

fn setup(mut commands: Commands) {
    // Cube

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 6.0),
        ..default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0)
                .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
            ..default()
        })
        .insert(
            CameraController {
                orbit_mode: true,
                orbit_focus: Vec3::new(0.0, 0.5, 0.0),
                ..default()
            }
            .print_controls(),
        );
}

// fn rotate_cube(time: Res<Time>, mut query: Query<(&RotatingCube, &mut Transform)>) {
//     for (rotating_cube, mut transform) in query.iter_mut() {
//         transform.rotate(Quat::from_rotation_y(
//             rotating_cube.speed * time.delta_seconds(),
//         ));
//     }
// }

// // System to create and spawn the cube with a texture
// fn setup_cube(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     // Load the texture for the cube
//     let texture_handle = asset_server.load("texture.png");

//     // Create the cube mesh
//     let cube_mesh2 = create_simple_cube_mesh();
//     // Add the cube to the scene with a texture applied

//     // Add the cube to the scene with a texture applied
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(cube_mesh2), // Add the custom cube mesh
//             material: materials.add(StandardMaterial {
//                 base_color_texture: Some(texture_handle),
//                 // Apply the texture to the cube
//                 ..default()
//             }),
//             transform: Transform::from_xyz(1.25, 0.0, -1.25),
//             ..default()
//         },
//         RotatingCube { speed: 0.15 },
//         WireframeColor {
//             color: Color::WHITE,
//         },
//     ));
// }

// System to create and spawn the cube with a texture
fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load the texture for the cube
    let texture_handle = asset_server.load("texture.png");

    // Create the cube mesh
    let cube_mesh2 = create_simple_cube_mesh();
    // Add the cube to the scene with a texture applied

    // Add the cube to the scene with a texture applied
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(cube_mesh2), // Add the custom cube mesh
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle),
                // Apply the texture to the cube
                ..default()
            }),
            transform: Transform::from_xyz(1.25, 0.0, -1.25),
            ..default()
        },
        WireframeColor {
            color: Color::WHITE,
        },
    ));
}

// Function to create a simple cube mesh
fn create_simple_cube_mesh() -> Mesh {
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
