use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn get_square(
    width: f32,
    pos_x: f32,
    pos_y: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        // mesh: meshes.add(RegularPolygon::new(100., 4)).into(),
        mesh: meshes.add(Rectangle::new(width, width)).into(),
        material: materials.add(Color::rgb(6.25, 9.4, 9.1)),
        transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
        ..default()
    }
}
