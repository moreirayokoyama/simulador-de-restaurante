use bevy::{
    app::Plugin,
    prelude::{Commands, Component, ResMut, Trigger},
};

use crate::restaurante::{ClienteChegou, ContadorCliente, NovoClienteEvent};

#[derive(Component)]
pub struct Cliente {
    pub id: u16,
    pub atendido: bool,
}

pub struct ClientePlugin;

impl Plugin for ClientePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.observe(on_novo_cliente);
    }
}

fn on_novo_cliente(_: Trigger<NovoClienteEvent>, mut commands: Commands, mut contador: ResMut<ContadorCliente>) {
    contador.0 += 1;
    let entidade = commands.spawn(Cliente { id: contador.0, atendido: false }).id();
    commands.trigger(ClienteChegou {
        cliente: entidade,
    });
}
