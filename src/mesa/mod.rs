use bevy::{app::{Plugin, Update}, prelude::Component};

#[derive(Component)]
pub struct Mesa {
    pub ocupada: bool,
}

pub struct MesaPlugin;

impl Plugin for MesaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, criar_pedidos)
    }
}

fn criar_pedidos() {

}
