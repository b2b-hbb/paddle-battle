use serde::{Serialize, Deserialize};
use alloy_primitives::{U256, I256};

#[derive(Debug, Clone)]
pub struct SerdeU256(pub U256);

#[derive(Debug, Clone)]
pub struct SerdeI256(pub I256);

impl From<U256> for SerdeU256 {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl From<SerdeU256> for U256 {
    fn from(value: SerdeU256) -> Self {
        value.0
    }
}

impl From<I256> for SerdeI256 {
    fn from(value: I256) -> Self {
        Self(value)
    }
}

impl From<SerdeI256> for I256 {
    fn from(value: SerdeI256) -> Self {
        value.0
    }
}

pub mod u256_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val: u32 = value.to::<u32>();
        val.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u32::deserialize(deserializer)?;
        Ok(U256::from(val))
    }
}

pub mod i256_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(value: &I256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val: i32 = value.as_i32();
        val.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<I256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = i32::deserialize(deserializer)?;
        Ok(I256::from_be_bytes(val.to_be_bytes()))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "U256")]
pub struct U256Def {
    #[serde(getter = "U256::to::<u32>")]
    value: u32,
}

impl From<U256Def> for U256 {
    fn from(def: U256Def) -> U256 {
        U256::from(def.value)
    }
}

impl From<U256> for U256Def {
    fn from(value: U256) -> U256Def {
        U256Def {
            value: value.to::<u32>()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "I256")]
pub struct I256Def {
    #[serde(getter = "I256::as_i32")]
    value: i32,
}

impl From<I256Def> for I256 {
    fn from(def: I256Def) -> I256 {
        I256::from_be_bytes(def.value.to_be_bytes())
    }
}

impl From<I256> for I256Def {
    fn from(value: I256) -> I256Def {
        I256Def {
            value: value.as_i32()
        }
    }
} 