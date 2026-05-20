use crate::ScalarStruct;
use crate::model::{Error, Result};
use macro_rules_attribute as mra;
use uuid::Uuid;

// Simple wrapper for SQLite Ids
#[mra::derive(Debug, ScalarStruct!)]
pub struct Id(Uuid);

impl Id {
	pub fn as_uuid(&self) -> &Uuid {
		&self.0
	}

	pub fn into_uuid(self) -> Uuid {
		self.0
	}
}

// from &i64
impl From<&Uuid> for Id {
	fn from(val: &Uuid) -> Id {
		Id(*val)
	}
}

impl TryFrom<String> for Id {
	type Error = Error;
	fn try_from(val: String) -> Result<Id> {
		let uuid =
			Uuid::parse_str(&val).map_err(|err| format!("id should be a valid UUID, was '{val}'.\nCause: {err}"))?;
		Ok(Id(uuid))
	}
}
