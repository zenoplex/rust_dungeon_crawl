use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut players = <(&Health, &Player)>::query();

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => *turn_state,
    };

    // Set state to GameOver if player health is less than 1
    players.iter(ecs).for_each(|(health, _)| {
        if health.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    *turn_state = new_state;
}
