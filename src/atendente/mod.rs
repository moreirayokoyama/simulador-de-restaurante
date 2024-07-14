use bevy::{app::Plugin, prelude::{Commands, Component, Entity, Event, Query, Trigger}};

use crate::{mesa::MesaComPedido, restaurante::{Funcionario, NovoPedido, PedidoColetado, PedidoEncaminhado}};

#[derive(Component)]
pub struct Atendente;

#[derive(Event)]
pub struct AtendendoMesa{
    pub atendente: Entity,
    pub mesa: Entity,
}

pub struct AtendentePlugin;

impl Plugin for AtendentePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .observe(on_novo_pedido)
            .observe(on_atendimento_mesa)
            .observe(on_pedido_coletado)
            ;


    }
}

fn on_novo_pedido(trigger: Trigger<NovoPedido>, mut query: Query<(Entity, &mut Funcionario, &Atendente)>, mut commands: Commands) {
    for (entidade, mut funcionario, _) in query.iter_mut() {
        if funcionario.esta_livre {
            funcionario.esta_livre = false;
            commands.trigger(AtendendoMesa{atendente: entidade, mesa: trigger.event().mesa});
        }
    }
}

fn on_atendimento_mesa(trigger: Trigger<AtendendoMesa>, query_mesas: Query<&MesaComPedido>, mut commands: Commands) {
    let mesa_com_pedido = query_mesas.get(trigger.event().mesa).unwrap();
    commands.trigger(PedidoColetado{atendente: trigger.event().atendente, mesa: trigger.event().mesa, pedido: mesa_com_pedido.pedido});
}

fn on_pedido_coletado(trigger: Trigger<PedidoColetado>, mut query: Query<&mut Funcionario>, mut commands: Commands) {
    let mut funcionario = query.get_mut(trigger.event().atendente).unwrap();
    commands.trigger(PedidoEncaminhado{pedido: trigger.event().pedido});
    funcionario.esta_livre = true;
}