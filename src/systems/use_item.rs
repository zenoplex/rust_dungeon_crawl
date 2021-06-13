use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    let mut activate_items = <(Entity, &ActivateItem)>::query();
    activate_items
        .iter(ecs)
        .for_each(|(activate_item_entity, activate_item)| {
            let item = ecs.entry_ref(activate_item.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate_item.used_by, healing.amount));
                }

                if let Ok(_dungeon_map) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles
                        .iter_mut()
                        .for_each(|tile_visible| *tile_visible = true)
                }
            }

            commands.remove(activate_item.item);
            commands.remove(*activate_item_entity);
        });

    healing_to_apply.iter().for_each(|(entity, amount)| {
        if let Ok(mut target) = ecs.entry_mut(*entity) {
            let h = target.get_component::<Health>().unwrap();

            if let Ok(health) = target.get_component_mut::<Health>() {
                println!("{}, {}", health.current, amount);
                health.current = i32::min(health.max, health.current + amount);
            }
        }
    });
}
