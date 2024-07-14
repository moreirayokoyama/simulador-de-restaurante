use bevy::{app::Plugin, prelude::{Commands, ResMut, Trigger}};

use crate::{atendente::Atendente, recepcionista::Recepcionista, restaurante::{ContadorFuncionario, Funcionario, NovoFuncionario, TipoFuncionario}, Cozinheiro};

pub struct FuncionarioPlugin;

impl Plugin for FuncionarioPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .observe(on_novo_funcionario)
            ;
    }
}

fn on_novo_funcionario(trigger: Trigger<NovoFuncionario>, mut contador: ResMut<ContadorFuncionario>, mut commands: Commands) {
    
    contador.0 += 1;
    let entidade = commands.spawn(Funcionario{id: contador.0, esta_livre: true}).id();
    match trigger.event().tipo {
        TipoFuncionario::Recepcionista => commands.entity(entidade).insert(Recepcionista),
        TipoFuncionario::Atendente => commands.entity(entidade).insert(Atendente),
        TipoFuncionario::Cozinheiro => commands.entity(entidade).insert(Cozinheiro),
    };
}