use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::BindableAddr;

impl Serialize for BindableAddr {
	fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
		self.to_string().serialize(s)
	}
}

impl<'de> Deserialize<'de> for BindableAddr {
	fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>
	where
		D::Error: de::Error,
	{
		String::deserialize(d)?.parse().map_err(de::Error::custom)
	}
}
