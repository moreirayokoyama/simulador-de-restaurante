use bevy::app::App;
use bevy::ecs::component::Component;
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
        .run();
}
