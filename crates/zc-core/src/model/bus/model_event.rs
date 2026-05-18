use crate::model::{EntityType, Id, RelIds};

// region:    --- Types

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityAction {
	Created,
	Updated,
	#[allow(unused)] // for now
	Deleted,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelEvent {
	pub entity: EntityType,
	pub action: EntityAction,
	pub id: Option<Id>,
	pub rel_ids: RelIds,
}

// endregion: --- Types
