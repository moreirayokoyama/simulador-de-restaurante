use bevy::{
    app::Plugin,
    prelude::{Commands, Component, Entity, Event, Query, Trigger, Without},
};

use crate::{cliente::Cliente, mesa::ClienteAcomodado, restaurante::Funcionario};
use crate::{
    mesa::{Mesa, MesaOcupada},
    restaurante::{AcomodandoCliente, ClienteChegou},
};

pub struct RecepcionistaPlugin;

#[derive(Component)]
pub struct Recepcionista;

#[derive(Event)]
pub struct RecebendoCliente{
    pub recepcionista: Entity,
    pub cliente: Entity
}

impl Plugin for RecepcionistaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .observe(on_cliente_chegou)
            .observe(on_recebendo_cliente)
            .observe(on_cliente_acomodado)
            ;
    }
}

fn on_cliente_chegou(
    trigger: Trigger<ClienteChegou>,
    mut query_recepcionista: Query<(Entity, &mut Funcionario, &Recepcionista)>,
    mut query_clientes: Query<&mut Cliente>,
    mut commands: Commands,
) {
    for (entidade, mut funcionario, _) in &mut query_recepcionista {
        if funcionario.esta_livre {
            let entidade_cliente = commands.entity(trigger.event().cliente).id();
            let mut cliente = query_clientes.get_mut(entidade_cliente).unwrap();
            cliente.atendido = true;
            funcionario.esta_livre = false;
            commands
                .trigger(RecebendoCliente{
                    recepcionista: entidade,
                    cliente: entidade_cliente,
                });
            break;
        }
    }
}

fn on_recebendo_cliente(
    trigger: Trigger<RecebendoCliente>,
    mut query: Query<&mut Funcionario>,
    query_mesa: Query<(Entity, &Mesa), Without<MesaOcupada>>,
    mut commands: Commands,
) {
    let mut mesa_iter: bevy::ecs::query::QueryIter<(Entity, &Mesa), Without<MesaOcupada>> = query_mesa.iter();
    let funcionario = &mut query.get_mut(trigger.event().recepcionista).unwrap();
    if let Some((entidade_mesa, _)) = mesa_iter.next() {
        funcionario.esta_livre = true;
        commands.trigger(
            AcomodandoCliente {
                recepcionista: trigger.event().recepcionista,
                cliente: trigger.event().cliente,
                mesa: entidade_mesa
            });
    }
}

fn on_cliente_acomodado(trigger: Trigger<ClienteAcomodado>, mut query: Query<&mut Funcionario>) {
    let funcionario = &mut query.get_mut(trigger.event().recepcionista).unwrap();
    funcionario.esta_livre = true;
}
