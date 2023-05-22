use async_graphql::{Context, Enum, Object, Result, Subscription, ID};
use futures_util::{Stream, StreamExt};
use std::time::Duration;

use crate::{models::UserMessages, queue::MessageBroker};

use super::Storage;

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum MutationType {
    Created,
}

#[derive(Clone)]
pub struct StreamChanged {
    pub mutation_type: MutationType,
    pub id: ID,
}

#[Object]
impl StreamChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &ID {
        &self.id
    }

    async fn user_messages(&self, ctx: &Context<'_>) -> Result<Option<UserMessages>> {
        let messsage = ctx.data_unchecked::<Storage>().lock().await;
        let id = self.id.parse::<usize>()?;
        Ok(messsage.get(id).cloned())
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn messages(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = StreamChanged> {
        MessageBroker::<StreamChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
