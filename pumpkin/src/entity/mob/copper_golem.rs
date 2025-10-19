use crate::entity::mob::{Mob, MobEntity};
use crate::entity::{Entity, NBTStorage};
use std::sync::{Arc, Weak};

pub struct CopperGolem {
    mob_entity: MobEntity,
}

impl CopperGolem {
    pub fn make(entity: Entity) -> Arc<Self> {
        let mob_entity = MobEntity::new(entity);
        let golem = Self { mob_entity };
        let mob_arc = Arc::new(golem);
        let mob_weak: Weak<dyn Mob> = {
            let mob_arc: Arc<dyn Mob> = mob_arc.clone();
            Arc::downgrade(&mob_arc)
        };
        {
            let mut goal_selector = mob_arc.mob_entity.goals_selector.lock().await;
            let mut target_selector = mob_arc.mob_entity.target_selector.lock().await;
        }
        mob_arc
    }
}

impl NBTStorage for CopperGolem {}

impl Mob for CopperGolem {
    fn get_mob_entity(&self) -> &MobEntity {
        &self.mob_entity
    }
}
