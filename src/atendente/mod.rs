use bevy::{app::Plugin, prelude::{Commands, Component, Entity, Query, Trigger, With, Without}};

use crate::restaurante::{Funcionario, NovoPedidoEvent};

#[derive(Component)]
pub struct Atendente;

#[derive(Component)]
pub struct AtendendoPedido;

pub struct AtendentePlugin;

impl Plugin for AtendentePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.
            observe(on_novo_pedido);
    }
}

fn on_novo_pedido(trigger: Trigger<NovoPedidoEvent>, query: Query<(Entity, &Funcionario, &AtendendoPedido), Without<AtendendoPedido>>, mut commands: Commands) {
    for (entity, funcionario, _) in query.iter() {
        if funcionario.esta_livre {
            commands.entity(entity).insert(AtendendoPedido{});
        }
    }
}