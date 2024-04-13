use async_graphql::{Context, Object, Result as GraphqlResult};
use chrono::{Duration, Utc};
use std::sync::Arc;

use crate::domain::{model::project::RecentProject, service::PortalService};

#[derive(Default)]
pub(super) struct PortalQuery;

#[Object]
impl PortalQuery {
    #[graphql(name = "selectPortalResentList")]
    async fn select_resent_list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default_with = "(Utc::now() - Duration::days(30)).timestamp()")] start_time: i64,
        #[graphql(validator(minimum = 1, maximum = 10), default = 10)] limit: u64,
    ) -> GraphqlResult<Vec<RecentProject>> {
        let portal_service = ctx.data::<Arc<PortalService>>()?;
        let result = portal_service.select_resent_list(start_time, limit).await?;

        Ok(result)
    }
}
