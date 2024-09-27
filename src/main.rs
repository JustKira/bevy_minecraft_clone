use bevy::color::palettes::css::{BLUE, WHITE};
use bevy::pbr::wireframe::{WireframeColor, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

#[derive(Component, Debug)]
struct RotatingCube {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        }) // Loads default Bevy plugins (rendering, input, etc.)
        .add_systems(Startup, (camera_setup, setup_cube))
        .add_systems(Update, rotate_cube) // Startup systems for camera and cube
        .run();
}

// Setup the camera and light
fn camera_setup(mut commands: Commands) {
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    // Light up the scene.
    commands.spawn(PointLightBundle {
        transform: camera_and_light_transform,
        ..default()
    });
}

fn rotate_cube(time: Res<Time>, mut query: Query<(&RotatingCube, &mut Transform)>) {
    for (rotating_cube, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(
            rotating_cube.speed * time.delta_seconds(),
        ));
    }
}

// System to create and spawn the cube with a texture
fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    // Load the texture for the cube
    // let texture_handle = asset_server.load("texture.png");

    // Create the cube mesh
    let cube_mesh = create_simple_cube_mesh();

    // Add the cube to the scene with a texture applied
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(cube_mesh), // Add the custom cube mesh
            material: materials.add(Color::from(BLUE)),
            ..default()
        },
        RotatingCube { speed: 1.0 },
        WireframeColor {
            color: Color::WHITE,
        },
    ));
}

// Function to create a simple cube mesh
fn create_simple_cube_mesh() -> Mesh {
    return Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5],  // vertex with index 1
            [0.5, 0.5, 0.5],   // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // UV mapping for the front face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0], // Front face
            // UV mapping for the back face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0], // Back face
            // UV mapping for the left face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0], // Left face
            // UV mapping for the right face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0], // Right face
            // UV mapping for the top face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0], // Top face
            // UV mapping for the bottom face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0], // Bottom face
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_inserted_indices(Indices::U32(vec![
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ]));
}
