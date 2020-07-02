mod tags;
pub use tags::*;
mod calendar;
pub use calendar::*;
mod species;
pub use species::*;
mod spatial;
pub use spatial::*;
mod render;
pub use render::*;
mod text;
pub use text::*;
mod gamesys;
pub use gamesys::*;
mod items;
pub use items::*;
mod identity;
pub use identity::*;
mod field_of_view;
pub use field_of_view::*;

pub mod spawner;

mod serialize;
pub use serialize::{deserialize_world, serialize_world};

pub(crate) mod prelude {
    pub use serde::{
        de::{self, DeserializeSeed, IgnoredAny, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    pub use std::{
        any::TypeId, cell::RefCell, collections::HashMap, marker::PhantomData, ptr::NonNull,
    };
    pub use type_uuid::TypeUuid;
}

use serialize::{ComponentRegistration, TagRegistration};

fn component_registration() -> (Vec<ComponentRegistration>, Vec<TagRegistration>) {
    let comp_registrations = vec![
        ComponentRegistration::of::<Position>(),
        ComponentRegistration::of::<CameraOptions>(),
        ComponentRegistration::of::<Calendar>(),
        ComponentRegistration::of::<Dimensions>(),
        ComponentRegistration::of::<VoxelModel>(),
        ComponentRegistration::of::<WorldPosition>(),
        ComponentRegistration::of::<Name>(),
        ComponentRegistration::of::<Description>(),
        ComponentRegistration::of::<Tint>(),
        ComponentRegistration::of::<Species>(),
        ComponentRegistration::of::<CompositeRender>(),
        ComponentRegistration::of::<Tagline>(),
        ComponentRegistration::of::<Attributes>(),
        ComponentRegistration::of::<ItemWorn>(),
        ComponentRegistration::of::<ItemStored>(),
        ComponentRegistration::of::<ItemCarried>(),
        ComponentRegistration::of::<Identity>(),
        ComponentRegistration::of::<Light>(),
        ComponentRegistration::of::<FieldOfView>(),
        ComponentRegistration::of::<Initiative>(),
    ];
    let tag_registrations = vec![
        TagRegistration::of::<Cordex>(),
        TagRegistration::of::<Building>(),
        TagRegistration::of::<Item>(),
        TagRegistration::of::<Sentient>(),
        TagRegistration::of::<MyTurn>(),
    ];
    (comp_registrations, tag_registrations)
}
