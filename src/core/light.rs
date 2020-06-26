// TODO Remove light.rs since there shouldn't be
//  any light unless generated by photons
// This file is only here so that I can see the
// objects I generate in the early stage of development.
// TBH though, I kind of think it should still stay... IDK
use amethyst::{
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb
    },
    prelude::{World, Builder, WorldExt},
    core::Transform
};

pub fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world.create_entity()
        .with(light)
        .with(transform)
        .build();
}
