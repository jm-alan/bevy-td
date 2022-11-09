use bevy::prelude::*;

pub struct Scene;

impl Scene {
  pub fn plane_generator(
    color: Color,
    size: f32,
  ) -> impl Fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>)
  {
    move |mut commands, mut meshes, mut materials| {
      commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size })),
        material: materials.add(color.into()),
        ..Default::default()
      });
    }
  }

  pub fn generate_shape<C, M>(
    mesh_basis: M,
    component: C,
    name: String,
    transform: Transform,
    color: Color,
  ) -> impl Fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>)
  where
    C: Component,
    C: Clone,
    Mesh: From<M>,
    M: Copy,
  {
    move |mut commands, mut meshes, mut materials| {
      commands
        .spawn_bundle(PbrBundle {
          mesh: meshes.add(Mesh::from(mesh_basis)),
          material: materials.add(color.into()),
          transform,
          ..Default::default()
        })
        .insert(component.clone())
        .insert(Name::new(name.clone()));
    }
  }
}
