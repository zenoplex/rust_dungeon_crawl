use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut players = <(&Health, &Point)>::query().filter(component::<Player>());

    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => *turn_state,
    };

    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet.iter(ecs).next().unwrap_or(&amulet_default);

    // Set state to GameOver if player health is less than 1
    players.iter(ecs).for_each(|(health, pos)| {
        if health.current < 1 {
            new_state = TurnState::GameOver;
        }

        if amulet_pos == pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
