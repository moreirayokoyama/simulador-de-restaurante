use bevy::app::{App, Startup, Update};
use bevy::ecs::component::Component;
use bevy::prelude::{Commands, Query, Res};
use bevy::time::Time;
use bevy::time::{Timer, TimerMode};
use bevy::DefaultPlugins;
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
struct Cliente {
    atendido: bool,
}

#[derive(Component)]
struct Recepcao {
    aberta: bool,
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, iniciar_restaurante)
        .add_systems(Update, (eventos_recepcao, count_clientes))
        .run();
}

fn iniciar_restaurante(mut commands: Commands) {
    commands.spawn((Funcionario { esta_livre: true }, Recepcionista));
    commands.spawn((Funcionario { esta_livre: true }, Atendente));
    commands.spawn((Funcionario { esta_livre: true }, Cozinheiro));
    commands.spawn((Funcionario { esta_livre: true }, Cozinheiro));
    commands.spawn(Recepcao {
        aberta: false,
        timer: Timer::from_seconds(3., TimerMode::Once),
    });
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

fn count_clientes(query: Query<&Cliente>) {
    println!("Número de clientes: {}", query.iter().count());
}
