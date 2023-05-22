use std::str::FromStr;

use crate::models::{Support, UserMessages};
use async_graphql::{Context, FieldResult, Object, ID};

use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn messages(
        &self,
        ctx: &Context<'_>,
        thread_id: ID,
    ) -> FieldResult<Option<Vec<UserMessages>>> {
        Ok(ctx
            .data_unchecked::<crate::AppState>()
            .container
            .support
            .find_one(doc! {
                "_id":ObjectId::from_str(&thread_id)?
            })
            .await?
            .and_then(|support| Some(support.user_messages)))
    }

    async fn threads(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> FieldResult<Option<Vec<Support>>> {
        Ok(Some(
            ctx.data_unchecked::<crate::AppState>()
                .container
                .support
                .find_all(doc! {
                    "project_id":ObjectId::from_str(&project_id)?
                })
                .await?
                .try_collect()
                .await?,
        ))
    }

    async fn thread(&self, ctx: &Context<'_>, thread_id: ID) -> FieldResult<Option<Support>> {
        Ok(ctx
            .data_unchecked::<crate::AppState>()
            .container
            .support
            .find_one(doc! {
                "_id":ObjectId::from_str(&thread_id)?
            })
            .await?)
    }
}
