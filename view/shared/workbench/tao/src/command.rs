use anyhow::Result;
use moss_hecs::MissingComponent;
use ts_rs::TS;

use moss_uikit::{primitive::Tooltip, state::Order};

use crate::Workbench;

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "dummy.ts")]
pub struct DescribeActivityOutput {
    pub tooltip: String,
    pub order: usize,
}

impl Workbench {
    // OPTIMIZE: consider to use a SmallVec type, as we don't expect tons of such elements
    pub fn command_get_all_activities(&self) -> Result<Vec<DescribeActivityOutput>> {
        let mut result = Vec::new();
        for entity in &self.known_activities {
            let entity_ref = self.frame.entity(*entity)?;

            let tooltip = entity_ref
                .get::<&Tooltip>()
                .ok_or_else(|| MissingComponent::new::<Tooltip>())?;

            let order = entity_ref
                .get::<&Order>()
                .ok_or_else(|| MissingComponent::new::<Order>())?;

            result.push(DescribeActivityOutput {
                tooltip: tooltip.text.to_string(),
                order: order.value.clone(),
            });
        }

        Ok(result)
    }
}
