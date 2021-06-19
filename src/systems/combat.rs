use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Entity is required since we need to remove message
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    // Entities of Vec<WantToAttack, Attacker, Victim>
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(message, wants_to_attack)| {
            (*message, wants_to_attack.attacker, wants_to_attack.victim)
        })
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(attacker_entry) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = attacker_entry.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let damage = base_damage + weapon_damage;

        let attacker_name = if let Ok(att_ref) = ecs.entry_ref(*attacker) {
            if let Ok(name) = att_ref.get_component::<Name>() {
                name.0.clone()
            } else if att_ref.get_component::<Player>().is_ok() {
                "Player".to_owned()
            } else {
                "Somthing".to_owned()
            }
        } else {
            String::from("")
        };

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("{} is attcking with damage: {}", &attacker_name, &damage);
            health.current -= damage;
            // Do not remove Player entity
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });
}
