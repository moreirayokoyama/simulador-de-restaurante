use bevy::{
    app::{Plugin, Update},
    prelude::{Component, EventWriter, Query},
};

use crate::restaurante::NovoPedidoEvent;

#[derive(Component, Default)]
pub struct Mesa {
    pub ocupada: bool,
    pub pediu: bool,
}

pub struct MesaPlugin;

impl Plugin for MesaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, criar_pedidos);
    }
}

fn criar_pedidos(mut query: Query<&mut Mesa>, mut novo_pedido: EventWriter<NovoPedidoEvent>) {
    for mut mesa in &mut query {
        if mesa.ocupada && !mesa.pediu {
            mesa.pediu = true;
            novo_pedido.send_default();
        }
    }
}
