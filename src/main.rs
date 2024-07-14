use atendente::AtendentePlugin;
use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::DefaultPlugins;
use cliente::ClientePlugin;
use cozinha::CozinhaPlugin;
use debug::DebugPlugin;
use funcionario::FuncionarioPlugin;
use mesa::MesaPlugin;
use recepcao::RecepcaoPlugin;
use recepcionista::RecepcionistaPlugin;
use restaurante::RestaurantePlugin;

mod cliente;
mod cozinha;
mod mesa;
mod recepcao;

//módulos de funcionários
mod recepcionista;
mod atendente;

mod restaurante;

mod debug;
mod funcionario;

#[derive(Component)]
struct Cozinheiro;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FuncionarioPlugin,
            AtendentePlugin,
            RecepcionistaPlugin,
            RecepcaoPlugin,
            RestaurantePlugin,
            MesaPlugin,
            CozinhaPlugin,
            ClientePlugin,
            DebugPlugin,
        ))
        .run();
}