use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::process::Command;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed to open file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        resources: &mut Resources,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::with_capacity(self.entities.len());
        self.entities
            .iter()
            .filter(|t| t.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pos| {
            // Choose random entity from vector
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pos, entity, &mut commands);
            }
        });

        // Apply command buffer
        commands.flush(ecs, resources);
    }

    pub fn spawn_entity(&self, pos: &Point, template: &Template, commands: &mut CommandBuffer) {
        let ent = commands.push((
            (),
            *pos,
            Name(template.name.clone()),
            Render {
                glyph: to_cp437(template.glyph),
                color: ColorPair::new(WHITE, BLACK),
            },
        ));

        match template.entity_type {
            EntityType::Item => {
                commands.add_component(ent, Item {});

                if let Some(effects) = &template.provides {
                    effects
                        .iter()
                        .for_each(|(provide, n)| match provide.as_str() {
                            "Healing" => {
                                commands.add_component(ent, ProvidesHealing { amount: *n });
                            }
                            "MagicMap" => {
                                commands.add_component(ent, ProvidesDungeonMap);
                            }
                            _ => {
                                println!("Warning: effect type not found.");
                            }
                        })
                } else {
                    println!("huh");
                }
            }
            EntityType::Enemy => {
                commands.add_component(ent, Enemy {});
                commands.add_component(ent, ChasingPlayer);
                commands.add_component(ent, FieldOfView::new(6));
                commands.add_component(
                    ent,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }
    }
}
