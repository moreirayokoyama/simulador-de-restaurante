use bevy::{
    app::{Plugin, Update},
    prelude::EventReader,
};

use crate::restaurante::NovoPedidoEvent;

pub struct CozinhaPlugin;

impl Plugin for CozinhaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, receber_pedidos);
    }
}

fn receber_pedidos(mut novos_pedidos: EventReader<NovoPedidoEvent>) {
    if !novos_pedidos.is_empty() {
        for _ in novos_pedidos.read() {
            println!("Novo Pedido");
        }
    }
}
