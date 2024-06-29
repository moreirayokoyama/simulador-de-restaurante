use bevy::app::{App, Update};
use bevy::ecs::component::Component;
use bevy::prelude::Query;
use bevy::DefaultPlugins;
use mesa::Mesa;
use recepcao::RecepcaoPlugin;
use recepcionista::RecepcionistaPlugin;
use restaurante::RestaurantePlugin;

mod cliente;
mod recepcao;
mod recepcionista;
mod restaurante;
mod mesa;

#[derive(Component)]
struct Atendente;

#[derive(Component)]
struct Cozinheiro;

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
