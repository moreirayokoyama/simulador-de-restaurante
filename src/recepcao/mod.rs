use bevy::{
    app::{Plugin, Update},
    prelude::{Commands, Component, Query, Res},
    time::{Time, Timer, TimerMode},
};
use rand::{thread_rng, Rng};

use crate::cliente::Cliente;

#[derive(Component)]
pub struct Recepcao {
    pub aberta: bool,
    pub timer: Timer,
}

pub struct RecepcaoPlugin;

impl Plugin for RecepcaoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, eventos_recepcao);
    }
}

fn eventos_recepcao(mut query: Query<&mut Recepcao>, mut commands: Commands, time: Res<Time>) {
    let mut rng = thread_rng();
    for mut recepcao in &mut query {
        recepcao.timer.tick(time.delta());
        if recepcao.timer.finished() {
            recepcao.aberta = true;
            commands.spawn(Cliente { atendido: false });
            recepcao.timer = Timer::from_seconds(rng.gen::<f32>() * 5., TimerMode::Once)
        }
    }
}
