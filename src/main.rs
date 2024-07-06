use bevy::app::{App, Update};
use bevy::ecs::component::Component;
use bevy::DefaultPlugins;
use cliente::ClientePlugin;
use cozinha::CozinhaPlugin;
use mesa::MesaPlugin;
use recepcao::RecepcaoPlugin;
use recepcionista::RecepcionistaPlugin;
use restaurante::RestaurantePlugin;

mod cliente;
mod cozinha;
mod mesa;
mod recepcao;
mod recepcionista;
mod restaurante;

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
            MesaPlugin,
            CozinhaPlugin,
            ClientePlugin,
        ))
        .add_systems(Update, debug)
        .run();
}

fn debug() {}
