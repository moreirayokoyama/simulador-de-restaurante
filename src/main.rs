use bevy::app::{App, Startup, Update};
use bevy::ecs::component::Component;
use bevy::prelude::{Commands, Query};
use bevy::time::{Timer, TimerMode};
use rand::{thread_rng, Rng};

#[derive(Component)]
struct Funcionario {
    esta_livre: bool,
}

#[derive(Component)]
struct Recepcionista;

#[derive(Component)]
struct Atendente;

#[derive(Component)]
struct Cozinheiro;

#[derive(Component)]
struct Recepcao
{   
    aberta: bool,
    timer: Timer,
}

fn main() {
    App::new()
        .add_systems(Startup, iniciar_restaurante)
        .add_systems(Update, eventos_recepcao)
        .run();
}

fn iniciar_restaurante(mut commands: Commands) {
    commands.spawn((Funcionario { esta_livre: true }, Recepcionista));
    commands.spawn((Funcionario { esta_livre: true }, Atendente));
    commands.spawn((Funcionario { esta_livre: true }, Cozinheiro));
    commands.spawn((Funcionario { esta_livre: true }, Cozinheiro));
    commands.spawn(Recepcao {aberta: false, timer: Timer::from_seconds(30., TimerMode::Once)});
}

fn eventos_recepcao(mut query: Query<&mut Recepcao>) {
    let mut rng = thread_rng();
    for mut recepcao in &mut query {
        if recepcao.timer.finished() {
            println!("Evento da Recepção");
            recepcao.aberta = true;
            recepcao.timer = Timer::from_seconds(rng.gen(), TimerMode::Once)
        }
    }
}
