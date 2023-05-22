use std::str::FromStr;

use async_graphql::{Context, FieldResult, Object, ID};
use bson::oid::ObjectId;

use crate::{
    models::{Support, UserMessages},
    queue::MessageBroker,
};

use super::{
    subscription::{MutationType, StreamChanged},
    Storage,
};

// use super::{MessageBroker, MutationType, Storage, StreamChanged};
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_thread(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        title: String,
        thread_description: String,
    ) -> FieldResult<Support> {
        let thread = Support {
            _id: Some(ObjectId::new()),
            project_id: Some(ObjectId::from_str(&project_id)?),
            title: title,
            thread_description: thread_description,
            user_messages: vec![],
        };
        ctx.data_unchecked::<crate::AppState>()
            .container
            .support
            .insert_one(&thread)
            .await?;
        Ok(thread)
    }

    async fn delete_thread(
        &self,
        ctx: &Context<'_>,
        thread_id: String,
    ) -> FieldResult<Option<Support>> {
        let thread = ctx
            .data_unchecked::<crate::AppState>()
            .container
            .support
            .delete_one(&thread_id)
            .await?;

        Ok(thread)
    }

    async fn send_message(
        &self,
        ctx: &Context<'_>,
        thread_id: String,
        username: String,
        text: String,
    ) -> ID {
        let mut messages = ctx.data_unchecked::<Storage>().lock().await;
        let entry = messages.vacant_entry();
        let id: ID = entry.key().into();
        let message = UserMessages {
            id: id.to_string().clone(),
            text: text.clone(),
            username: username.clone(),
        };
        match ctx
            .data_unchecked::<crate::AppState>()
            .container
            .support
            .add_message(&thread_id, message.clone())
            .await
        {
            _ => (),
        }
        entry.insert(message.clone());
        MessageBroker::publish(StreamChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
        });
        id
    }
}
