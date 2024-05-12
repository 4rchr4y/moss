use types::id::NanoId;

#[cfg(feature = "graphql")]
use async_graphql::SimpleObject;

#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct IgnoredSource {
    pub id: NanoId,
    pub source: String,
}
