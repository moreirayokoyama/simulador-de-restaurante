use bevy::app::{App, Startup, Update};
use bevy::ecs::component::Component;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::prelude::{Commands, Entity, Query, Res};
use bevy::reflect::Reflect;
use bevy::time::Time;
use bevy::time::{Timer, TimerMode};
use bevy::DefaultPlugins;
use cliente::Cliente;
use rand::{thread_rng, Rng};
use recepcao::{Recepcao, RecepcaoPlugin};
use recepcionista::{Recepcionista, RecepcionistaPlugin};

mod recepcionista;
mod cliente;
mod recepcao;

#[derive(Component)]
struct Funcionario {
    esta_livre: bool,
}

#[derive(Component)]
struct Atendente;

#[derive(Component)]
struct Cozinheiro;

#[derive(Component)]
struct Mesa {
    ocupada: bool
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RecepcionistaPlugin, RecepcaoPlugin))
        .add_systems(Startup, iniciar_restaurante)
        .run();
}

fn iniciar_restaurante(mut commands: Commands) {
    commands.spawn((Funcionario { esta_livre: true }, Recepcionista));
    commands.spawn((Funcionario { esta_livre: true }, Atendente));
    commands.spawn((Funcionario { esta_livre: true }, Cozinheiro));
    commands.spawn((Funcionario { esta_livre: true }, Cozinheiro));
    commands.spawn(Recepcao {
        aberta: false,
        timer: Timer::from_seconds(3., TimerMode::Once),
    });
    commands.spawn(Mesa { ocupada: false });
    commands.spawn(Mesa { ocupada: false });
    commands.spawn(Mesa { ocupada: false });
    commands.spawn(Mesa { ocupada: false });

}


