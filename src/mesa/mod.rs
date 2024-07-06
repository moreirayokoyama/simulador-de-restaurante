use bevy::{
    app::{Plugin, Update},
    prelude::{Commands, Component, Entity, Query, Trigger, With},
};

use crate::restaurante::{ClienteAcomodado, NovoPedidoEvent};

#[derive(Component, Default)]
pub struct Mesa {
    pub pediu: bool,
}

#[derive(Component)]
pub struct MesaOcupada(Entity);

pub struct MesaPlugin;

impl Plugin for MesaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, criar_pedidos);
    }
}

fn criar_pedidos(mut query: Query<&mut Mesa, With<MesaOcupada>>, mut commands: Commands) {
    for mut mesa in &mut query {
        if !mesa.pediu {
            mesa.pediu = true;
            commands.trigger(NovoPedidoEvent {})
        }
    }
}

pub fn criar_mesa(commands: &mut Commands) {
    commands
        .spawn(Mesa {
            ..Default::default()
        })
        .observe(on_cliente_acomodado);
}

fn on_cliente_acomodado(trigger: Trigger<ClienteAcomodado>, mut commands: Commands) {
    let cliente = trigger.event().cliente;
    commands
        .entity(trigger.entity())
        .insert(MesaOcupada(cliente));
}
