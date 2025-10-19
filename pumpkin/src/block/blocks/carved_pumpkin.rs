use crate::block::{BlockBehaviour, BlockMetadata, OnPlaceArgs, PlacedArgs};
use crate::entity::Entity;
use crate::entity::mob::copper_golem::CopperGolem;
use async_trait::async_trait;
use pumpkin_data::block_properties::ChestLikeProperties;
use pumpkin_data::entity::EntityType;
use pumpkin_data::{
    Block,
    block_properties::{BlockProperties, WallTorchLikeProperties},
};
use pumpkin_world::BlockStateId;
use pumpkin_world::world::BlockFlags;
use uuid::Uuid;

pub struct CarvedPumpkinBlock;

impl BlockMetadata for CarvedPumpkinBlock {
    fn namespace(&self) -> &'static str {
        "minecraft"
    }

    fn ids(&self) -> &'static [&'static str] {
        &[Block::JACK_O_LANTERN.name, Block::CARVED_PUMPKIN.name]
    }
}

#[async_trait]
impl BlockBehaviour for CarvedPumpkinBlock {
    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = WallTorchLikeProperties::default(args.block);
        props.facing = args
            .player
            .living_entity
            .entity
            .get_horizontal_facing()
            .opposite();
        props.to_state_id(args.block)
    }

    async fn placed(&self, args: PlacedArgs<'_>) {
        if args.world.get_block(&args.position.down()).await.id == Block::COPPER_BLOCK.id {
            args.world
                .set_block_state(
                    args.position,
                    Block::AIR.default_state.id,
                    BlockFlags::NOTIFY_LISTENERS,
                )
                .await;
            let facing_prop = WallTorchLikeProperties::from_state_id(args.state_id, args.block);
            let mut chest_prop = ChestLikeProperties::default(&Block::COPPER_CHEST);
            chest_prop.facing = facing_prop.facing;
            args.world
                .set_block_state(
                    &args.position.down(),
                    chest_prop.to_state_id(&Block::COPPER_CHEST),
                    BlockFlags::NOTIFY_LISTENERS,
                )
                .await;
            let entity = Entity::new(
                Uuid::new_v4(),
                args.world.clone(),
                args.position.to_f64(),
                &EntityType::COPPER_GOLEM,
                false,
            );
            let golem = CopperGolem::make(entity);
            args.world.spawn_entity(golem).await;
        }
    }
}
