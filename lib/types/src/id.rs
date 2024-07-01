use anyhow::anyhow;
use nanoid::nanoid;
use std::marker::PhantomData;
use std::ops::Deref;

#[cfg(feature = "graphql")]
use async_graphql::{Scalar, ScalarType};

#[cfg(feature = "specta")]
use specta::Type;

const NANOID_20: usize = 20;
const CHAR_SET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

// TODO: implement all traits to use NanoId as a SEA ORM model type

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct NanoId(#[serde(with = "bounded_string_serializer")] BoundedString<NanoIdDefault>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "specta", derive(Type))]
struct NanoIdDefault;

impl BoundedStringLength for NanoIdDefault {
    const LENGTH: usize = NANOID_20;
}

pub trait BoundedStringLength {
    const LENGTH: usize;
}

impl NanoId {
    pub fn new() -> Self {
        let id = BoundedString::new(&nanoid!(NANOID_20, &CHAR_SET)).unwrap();
        NanoId(id)
    }
}

impl std::fmt::Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.inner)
    }
}

impl Into<sea_orm::Value> for NanoId {
    fn into(self) -> sea_orm::Value {
        sea_orm::Value::String(Some(Box::new(self.0.to_string())))
    }
}

impl Into<sea_orm::Value> for &NanoId {
    fn into(self) -> sea_orm::Value {
        sea_orm::Value::String(Some(Box::new(self.0.to_string())))
    }
}

impl From<&str> for NanoId {
    fn from(value: &str) -> Self {
        NanoId(BoundedString::new(value).unwrap())
    }
}

impl From<String> for NanoId {
    fn from(value: String) -> Self {
        NanoId(BoundedString::new(value).unwrap())
    }
}

impl AsRef<str> for NanoId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Into<String> for NanoId {
    fn into(self) -> String {
        self.0.inner
    }
}

#[cfg(feature = "graphql")]
#[Scalar]
impl ScalarType for NanoId {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = &value {
            BoundedString::<NanoIdDefault>::new(value)
                .map(NanoId)
                .map_err(|e| async_graphql::InputValueError::custom(e.to_string()))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.inner.clone())
    }
}

// Bounded String implementation

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Hash)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct BoundedString<L: BoundedStringLength> {
    inner: String,

    #[serde(skip)]
    _marker: PhantomData<[L]>,
}

impl<L: BoundedStringLength> BoundedString<L> {
    pub fn new<S: AsRef<str>>(input: S) -> anyhow::Result<Self> {
        let input_ref = input.as_ref();
        if input_ref.chars().count() <= L::LENGTH {
            Ok(BoundedString {
                inner: input_ref.to_string(),
                _marker: PhantomData,
            })
        } else {
            Err(anyhow!(
                "invalid id format, allowed length is {}",
                L::LENGTH
            ))
        }
    }
}

impl<L: BoundedStringLength> Deref for BoundedString<L> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

mod bounded_string_serializer {
    use super::{BoundedString, BoundedStringLength};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S, L: BoundedStringLength>(
        value: &BoundedString<L>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.inner)
    }

    pub fn deserialize<'de, D, L: BoundedStringLength>(
        deserializer: D,
    ) -> Result<BoundedString<L>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BoundedString::new(s).map_err(serde::de::Error::custom)
    }
}
