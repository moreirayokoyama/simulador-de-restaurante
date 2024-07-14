use bevy::{
    app::Plugin,
    prelude::{Commands, Component, Entity, Event, Query, ResMut, Trigger, With},
};

use crate::restaurante::{AcomodandoCliente, ContadorMesa, ContadorPedido, NovaMesa, NovoPedido, PedidoColetado};

#[derive(Component)]
pub struct Mesa {
    pub id: u16,
    pub pediu: bool,
}

#[derive(Component)]
pub struct Pedido {
    pub id: u16,
    pub mesa: Entity,   
}

#[derive(Component)]
pub struct MesaComPedido {
    pub pedido: Entity,
}

#[derive(Component)]
pub struct AguardandoPedido {
    pub pedido: Entity,
}

#[derive(Component)]
pub struct MesaOcupada(Entity);

#[derive(Event)]
pub struct ClienteAcomodado
{
    pub recepcionista: Entity,
    pub cliente: Entity,
    pub mesa: Entity,
}


pub struct MesaPlugin;

impl Plugin for MesaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .observe(on_nova_mesa)
            .observe(on_acomodando_cliente)
            .observe(on_cliente_acomodado)
            .observe(on_pedido_coletado)
            ;
    }
}

fn on_cliente_acomodado(trigger: Trigger<ClienteAcomodado>, mut query: Query<(Entity, &mut Mesa), With<MesaOcupada>>, mut commands: Commands, mut contadorPedido: ResMut<ContadorPedido>) {
    contadorPedido.0 += 1;
    let pedido = commands.spawn(Pedido{id: contadorPedido.0, mesa: trigger.event().mesa}).id();
    commands.entity(trigger.event().mesa).insert(MesaComPedido{pedido});
    commands.trigger(NovoPedido {mesa: trigger.event().mesa});
}

pub fn on_nova_mesa(_: Trigger<NovaMesa>, mut commands: Commands, mut contador: ResMut<ContadorMesa>) {
    contador.0 += 1;
    commands
        .spawn(Mesa {
            id: contador.0,
            pediu: false,
        });
}

fn on_acomodando_cliente(trigger: Trigger<AcomodandoCliente>, mut commands: Commands) {
    let cliente = trigger.event().cliente;
    commands
        .entity(trigger.event().mesa)
        .insert(MesaOcupada(cliente));

    commands.trigger(ClienteAcomodado {
        recepcionista: trigger.event().recepcionista,
        cliente: cliente,
        mesa: trigger.event().mesa,
    })

}

fn on_pedido_coletado(trigger: Trigger<PedidoColetado>, mut commands: Commands) {
    commands.entity(trigger.event().mesa)
        .remove::<MesaComPedido>()
        .insert(AguardandoPedido{pedido: trigger.event().pedido});
}
