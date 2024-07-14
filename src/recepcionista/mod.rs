use bevy::{
    app::{Plugin, Update},
    prelude::{Commands, Component, Entity, Query, Trigger, Without},
};

use crate::{cliente::Cliente, restaurante::Funcionario};
use crate::{
    mesa::{Mesa, MesaOcupada},
    restaurante::{ClienteAcomodado, ClienteChegouEvent},
};

pub struct RecepcionistaPlugin;

#[derive(Component)]
pub struct Recepcionista;

#[derive(Component)]
pub struct RecebendoCliente(Entity);

impl Plugin for RecepcionistaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.observe(on_cliente_chegou)
            .add_systems(Update, recepcao_cliente);
    }
}

fn on_cliente_chegou(
    trigger: Trigger<ClienteChegouEvent>,
    mut query_recepcionista: Query<(Entity, &mut Funcionario, &Recepcionista)>,
    mut query_clientes: Query<&mut Cliente>,
    mut commands: Commands,
) {
    for (entity_recepcionista, mut funcionario, _) in &mut query_recepcionista {
        if funcionario.esta_livre {
            let cliente_entity = commands.entity(trigger.event().cliente).id();
            let mut cliente = query_clientes.get_mut(cliente_entity).unwrap();
            cliente.atendido = true;
            funcionario.esta_livre = false;
            commands
                .entity(entity_recepcionista)
                .insert(RecebendoCliente(cliente_entity));
            break;
        }
    }
}

fn recepcao_cliente(
    mut query: Query<(Entity, &mut Funcionario, &Recepcionista, &RecebendoCliente)>,
    query_mesa: Query<(Entity, &Mesa), Without<MesaOcupada>>,
    mut commands: Commands,
) {
    let mut mesa_iter = query_mesa.iter();
    for (entidade_recepcionista, mut funcionario, _, recebendo_cliente) in &mut query {
        if let Some((entidade_mesa, _)) = mesa_iter.next() {
            commands
                .entity(entidade_recepcionista)
                .remove::<RecebendoCliente>();
            funcionario.esta_livre = true;
            commands.trigger_targets(
                ClienteAcomodado {
                    cliente: recebendo_cliente.0,
                },
                entidade_mesa,
            );
        } else {
            return;
        }
    }
}
