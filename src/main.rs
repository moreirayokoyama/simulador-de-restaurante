use bevy::app::{App, Update};
use bevy::ecs::component::Component;
use bevy::prelude::Query;
use bevy::DefaultPlugins;
use recepcao::RecepcaoPlugin;
use recepcionista::RecepcionistaPlugin;
use restaurante::RestaurantePlugin;

mod cliente;
mod recepcao;
mod recepcionista;
mod restaurante;

#[derive(Component)]
struct Atendente;

#[derive(Component)]
struct Cozinheiro;

#[derive(Component)]
struct Mesa {
    ocupada: bool,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RecepcionistaPlugin,
            RecepcaoPlugin,
            RestaurantePlugin,
        ))
        .add_systems(Update, debug)
        .run();
}

fn debug(query: Query<&Mesa>) {
    for mesa in &query {
        println!("Mesa ocupada: {}", mesa.ocupada);
    }
}
