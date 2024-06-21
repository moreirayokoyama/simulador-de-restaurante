use bevy::app::{App, Update};

fn main() {
    App::new()
        .add_systems(Update, print_helloworld)
        .run();
}

fn print_helloworld() {
    println!("Olá mundo");
}
