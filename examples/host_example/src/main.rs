use bevy::prelude::*;
use bevy::{DefaultPlugins, app::App};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

// Get started by importing the prelude
use wasvy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ModloaderPlugin))
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component, Reflect, Debug, Clone)]
struct ColoredBox<T> {
    color: T,
}

fn a(boxx: &mut ColoredBox) {
    boxx.color.set_hue(0.5);
    println!("Box color: {:?}", box.color);
}
