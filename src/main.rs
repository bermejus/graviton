
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::AmbientLight,
    prelude::*,
    render::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions}
};

use components::*;
mod components;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Turn off wireframe rendering
    wireframe_config.global = false;

    // Load all textures
    let sun_texture_handle = asset_server.load("textures/sun/8k_sun.jpg");
    let mercury_texture_handle = asset_server.load("textures/mercury/8k_mercury.jpg");
    let earth_texture_handle = asset_server.load("textures/earth/8k_earth_daymap.jpg");

    // Sphere Handle
    let sphere_handle = meshes.add(
        Mesh::from(shape::Icosphere {
            radius: 3.0,
            subdivisions: 8
        })
    );

    // Create material handles
    let sun_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(sun_texture_handle),
        unlit: true,
        roughness: 0.9,
        metallic: 0.0,
        ..Default::default()
    });

    let mercury_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(mercury_texture_handle),
        ..Default::default()
    });

    let earth_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(earth_texture_handle),
        ..Default::default()
    });

    // Draw the Sun
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle.clone(),
        material: sun_material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        visible: Visible {
            is_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Wireframe);

    // Draw Mercury
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle.clone(),
        material: mercury_material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 8.0),
            ..Default::default()
        },
        visible: Visible {
            is_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    });
    
    // Draw Earth
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle.clone(),
        material: earth_material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -7.0),
            ..Default::default()
        },
        visible: Visible {
            is_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.33
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode]
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)

        // Show FPS
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_startup_system(setup.system())
        .add_startup_system(spawn_camera.system())
        .add_system(pan_orbit_camera.system())
        .run();
}
