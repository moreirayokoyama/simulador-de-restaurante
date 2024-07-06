use bevy::{
    app::{Plugin, Startup},
    prelude::{Commands, Component, Entity, Event},
    time::{Timer, TimerMode},
};

use crate::mesa::criar_mesa;
use crate::{recepcao::Recepcao, recepcionista::Recepcionista, Atendente, Cozinheiro};

#[derive(Component)]
pub struct Funcionario {
    pub esta_livre: bool,
}

#[derive(Event, Default)]
pub struct NovoPedidoEvent;

#[derive(Event, Default)]
pub struct NovoClienteEvent;

#[derive(Event)]
pub struct ClienteChegouEvent {
    pub cliente: Entity,
}

#[derive(Event)]
pub struct ClienteAcomodado {
    pub cliente: Entity,
}

pub struct RestaurantePlugin;

impl Plugin for RestaurantePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<NovoPedidoEvent>()
            .add_systems(Startup, iniciar_restaurante);
    }
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
    criar_mesa(&mut commands);
    criar_mesa(&mut commands);
    criar_mesa(&mut commands);
    criar_mesa(&mut commands);
    criar_mesa(&mut commands);
    criar_mesa(&mut commands);
    criar_mesa(&mut commands);
}
