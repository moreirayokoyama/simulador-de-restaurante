use bevy::{app::Plugin, prelude::Trigger};

use crate::restaurante::NovoPedidoEvent;

pub struct CozinhaPlugin;

impl Plugin for CozinhaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.observe(on_novo_pedido);
    }
}

fn on_novo_pedido(_: Trigger<NovoPedidoEvent>) {
    println!("Novo Pedido");
}
