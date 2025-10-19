use crate::entity::mob::{Mob, MobEntity};
use crate::entity::{Entity, NBTStorage};
use std::sync::Arc;

pub struct CopperGolem {
    mob_entity: MobEntity,
}

impl CopperGolem {
    pub fn make(entity: Entity) -> Arc<Self> {
        let mob_entity = MobEntity::new(entity);
        let golem = Self { mob_entity };
        let mob_arc = Arc::new(golem);
        mob_arc
    }
}

impl NBTStorage for CopperGolem {}

impl Mob for CopperGolem {
    fn get_mob_entity(&self) -> &MobEntity {
        &self.mob_entity
    }
}
