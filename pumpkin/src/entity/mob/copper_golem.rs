use crate::entity::NBTStorage;
use crate::entity::mob::{Mob, MobEntity};
use pumpkin_data::entity::MobCategory;

pub struct CopperGolem {
    mob_entity: MobEntity,
    mob_category: MobCategory,
}

impl NBTStorage for CopperGolem {}

impl Mob for CopperGolem {
    fn get_mob_entity(&self) -> &MobEntity {
        todo!()
    }
}
