use bevy::prelude::Component;

#[derive(Component)]
pub struct Cliente {
    pub(crate) atendido: bool,
}

