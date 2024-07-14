use bevy::{app::Plugin, prelude::{Query, Trigger}};

use crate::{atendente::AtendendoMesa, cliente::Cliente, mesa::{Mesa, Pedido}, recepcionista::RecebendoCliente, restaurante::{AcomodandoCliente, ClienteChegou, Funcionario, NovoPedido, PedidoColetado, PedidoEncaminhado}};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .observe(on_cliente_chegou)
            .observe(on_recebendo_cliente)
            .observe(on_acomodando_cliente)
            .observe(on_cliente_acomodado)
            .observe(on_novo_pedido)
            .observe(on_atendendo_mesa)
            .observe(on_pedido_coletado)
            .observe(on_pedido_encaminhado)
            ;
        
    }
}

fn on_cliente_chegou(trigger: Trigger<ClienteChegou>, query: Query<&Cliente>) {
    let cliente = query.get(trigger.event().cliente).unwrap();
    println!("Cliente {0} Chegou.", cliente.id);
}

fn on_recebendo_cliente(trigger: Trigger<RecebendoCliente>, query_funcionario: Query<&Funcionario>, query_cliente: Query<&Cliente>) {
    let funcionario = query_funcionario.get(trigger.event().recepcionista).unwrap();
    let cliente = query_cliente.get(trigger.event().cliente).unwrap();
    println!("Cliente {0} está sendo recebido pelo Funcionário {1}", cliente.id, funcionario.id);
}

fn on_acomodando_cliente(trigger: Trigger<AcomodandoCliente>, query_funcionario: Query<&Funcionario>, query_cliente: Query<&Cliente>, query_mesa: Query<&Mesa>) {
    let funcionario = query_funcionario.get(trigger.event().recepcionista).unwrap();
    let cliente = query_cliente.get(trigger.event().cliente).unwrap();
    let mesa = query_mesa.get(trigger.event().mesa).unwrap();

    println!("Cliente {0} está sendo acomodado pelo Funcionário {1} na mesa {2}", cliente.id, funcionario.id, mesa.id);
}

fn on_cliente_acomodado(trigger: Trigger<AcomodandoCliente>, query_funcionario: Query<&Funcionario>, query_cliente: Query<&Cliente>, query_mesa: Query<&Mesa>) {
    let funcionario = query_funcionario.get(trigger.event().recepcionista).unwrap();
    let cliente = query_cliente.get(trigger.event().cliente).unwrap();
    let mesa = query_mesa.get(trigger.event().mesa).unwrap();

    println!("Cliente {0} está foi acomodado pelo Funcionário {1} na mesa {2}", cliente.id, funcionario.id, mesa.id);
}

fn on_novo_pedido(trigger: Trigger<NovoPedido>, query_mesa: Query<&Mesa>) {
    let mesa = query_mesa.get(trigger.event().mesa).unwrap();

    println!("Mesa {0} tem um novo pedido.", mesa.id);
}

fn on_atendendo_mesa(trigger: Trigger<AtendendoMesa>, query_mesa: Query<&Mesa>, query_funcionario: Query<&Funcionario>) {
    let mesa = query_mesa.get(trigger.event().mesa).unwrap();
    let funcionario = query_funcionario.get(trigger.event().atendente).unwrap();

    println!("Funcionario {0} está atendendo a Mesa {1}", funcionario.id, mesa.id)
}

fn on_pedido_coletado(trigger: Trigger<PedidoColetado>, query_mesa: Query<&Mesa>, query_funcionario: Query<&Funcionario>, query_pedido: Query<&Pedido>) {
    let mesa = query_mesa.get(trigger.event().mesa).unwrap();
    let funcionario = query_funcionario.get(trigger.event().atendente).unwrap();
    let pedido = query_pedido.get(trigger.event().pedido).unwrap();

    println!("Funcionario {0} coletou da Mesa {1} o Pedido {2}", funcionario.id, mesa.id, pedido.id);
}

fn on_pedido_encaminhado(trigger: Trigger<PedidoEncaminhado>, query_pedido: Query<&Pedido>) {
    let pedido = query_pedido.get(trigger.event().pedido).unwrap();

    println!("Pedido {0} encaminhado para a cozinha", pedido.id);
}
