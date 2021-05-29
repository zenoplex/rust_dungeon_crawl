use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Entity is required since we need to remove message
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(message, wants_to_attack)| (*message, wants_to_attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });
}
