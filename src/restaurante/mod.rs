use bevy::{
    app::{Plugin, Startup},
    prelude::{Commands, Component, Entity, Event, Resource},
    time::{Timer, TimerMode},
};

use crate::recepcao::Recepcao;

#[derive(Resource)]
pub struct ContadorMesa(pub u16);

#[derive(Resource)]
pub struct ContadorCliente(pub u16);

#[derive(Resource)]
pub struct ContadorFuncionario(pub u16);

#[derive(Resource)]
pub struct ContadorPedido(pub u16);


#[derive(Component)]
pub struct Funcionario {
    pub id: u16,
    pub esta_livre: bool,
}

#[derive(Event)]
pub struct NovoPedido{
    pub mesa: Entity,
}

#[derive(Event)]
pub struct PedidoColetado{
    pub atendente: Entity,
    pub mesa: Entity,
    pub pedido: Entity,
}

#[derive(Event)]
pub struct PedidoEncaminhado {
    pub pedido: Entity,
}

#[derive(Event, Default)]
pub struct NovoClienteEvent;

#[derive(Event, Default)]
pub struct NovaMesa;

#[derive(Event)]
pub struct ClienteChegou {
    pub cliente: Entity,
}

#[derive(Event)]
pub struct AcomodandoCliente {
    pub recepcionista: Entity,
    pub cliente: Entity,
    pub mesa: Entity,
}

#[derive(Event)]
pub struct NovoFuncionario {
    pub tipo: TipoFuncionario
}

pub enum TipoFuncionario {
    Recepcionista,
    Atendente,
    Cozinheiro,
}


pub struct RestaurantePlugin;

impl Plugin for RestaurantePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(ContadorCliente(0))
            .insert_resource(ContadorMesa(0))
            .insert_resource(ContadorFuncionario(0))
            .insert_resource(ContadorPedido(0))
            .add_event::<NovoPedido>()
            .add_systems(Startup, iniciar_restaurante);
    }
}

fn iniciar_restaurante(mut commands: Commands) {
    commands.trigger(NovoFuncionario{tipo: TipoFuncionario::Recepcionista});
    commands.trigger(NovoFuncionario{tipo: TipoFuncionario::Atendente});
    commands.trigger(NovoFuncionario{tipo: TipoFuncionario::Cozinheiro});
    commands.trigger(NovoFuncionario{tipo: TipoFuncionario::Cozinheiro});

    commands.spawn(Recepcao {
        aberta: false,
        timer: Timer::from_seconds(3., TimerMode::Once),
    });

    commands.trigger(NovaMesa{});
    commands.trigger(NovaMesa{});
    commands.trigger(NovaMesa{});
    commands.trigger(NovaMesa{});
    commands.trigger(NovaMesa{});
    commands.trigger(NovaMesa{});
    commands.trigger(NovaMesa{});
}
