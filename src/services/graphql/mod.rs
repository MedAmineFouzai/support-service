mod mutations;
mod queries;
mod subscription;

use async_graphql::Schema;

use crate::models::UserMessages;

pub use mutations::MutationRoot;
pub use queries::QueryRoot;
pub use subscription::SubscriptionRoot;

use futures_util::lock::Mutex;
use slab::Slab;
use std::sync::Arc;

#[derive(Debug)]
pub struct MyToken(pub String);
pub type Storage = Arc<Mutex<Slab<UserMessages>>>;
pub type SupportSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
