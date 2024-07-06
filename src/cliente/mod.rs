use bevy::{
    app::Plugin,
    prelude::{Commands, Component, Trigger},
};

use crate::restaurante::{ClienteChegouEvent, NovoClienteEvent};

#[derive(Component)]
pub struct Cliente {
    pub atendido: bool,
}

pub struct ClientePlugin;

impl Plugin for ClientePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.observe(on_novo_cliente);
    }
}

fn on_novo_cliente(_: Trigger<NovoClienteEvent>, mut commands: Commands) {
    let cliente_entity = commands.spawn(Cliente { atendido: false }).id();
    commands.trigger(ClienteChegouEvent {
        cliente: cliente_entity,
    })
}
