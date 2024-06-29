use bevy::{
    app::{Plugin, Update},
    hierarchy::{BuildChildren, Children},
    prelude::{Commands, Component, Entity, Query},
};

use crate::{cliente::Cliente, restaurante::Funcionario, Mesa};

pub struct RecepcionistaPlugin;

#[derive(Component)]
pub struct Recepcionista;

impl Plugin for RecepcionistaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (receber_clientes, recepcao_cliente));
    }
}

fn receber_clientes(
    mut query_cliente: Query<(Entity, &mut Cliente)>,
    mut query_recepcionista: Query<(Entity, &mut Funcionario, &Recepcionista)>,
    mut commands: Commands,
) {
    for (entity_cliente, mut cliente) in &mut query_cliente {
        if !cliente.atendido {
            for (entity_recepcionista, mut funcionario, _) in &mut query_recepcionista {
                if funcionario.esta_livre {
                    cliente.atendido = true;
                    funcionario.esta_livre = false;
                    commands
                        .entity(entity_recepcionista)
                        .push_children(&[entity_cliente]);
                    break;
                }
            }
        }
    }
}

fn recepcao_cliente(mut query_mesa: Query<(Entity, &mut Mesa)>, mut query: Query<(Entity, &mut Funcionario, &Recepcionista, &Children)>, query_clientes: Query<&Cliente>, mut commands: Commands) {
    for (entidade_mesa, mut mesa) in &mut query_mesa {
        if !mesa.ocupada {
            for (entidade_recepcionista, mut funcionario, recepcionista, children) in &mut query {
                if !funcionario.esta_livre {
                    for &child in children {
                        commands.entity(entidade_recepcionista).remove_children(&[child]);
                        funcionario.esta_livre = true;
                        commands.entity(entidade_mesa).push_children(&[child]);
                        mesa.ocupada = true;
                    }
                }
            }
        }
    }
}
