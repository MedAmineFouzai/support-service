use async_graphql::{Object, ID};
use bson::oid::ObjectId;
use serde::{self, Deserialize, Serialize, Serializer};

pub fn serialize_object_id<S>(
    object_id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
        Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMessages {
    pub id: String,
    pub username: String,
    pub text: String,
}

#[Object]
impl UserMessages {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Support {
    #[serde(serialize_with = "serialize_object_id")]
    pub _id: Option<ObjectId>,
    #[serde(serialize_with = "serialize_object_id")]
    pub project_id: Option<ObjectId>,
    pub title: String,
    pub thread_description: String,
    pub user_messages: Vec<UserMessages>,
}

#[Object]
impl Support {
    async fn _id(&self) -> ID {
        ID::from(self._id.unwrap())
    }

    async fn project_id(&self) -> ID {
        ID::from(self._id.unwrap())
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn thread_description(&self) -> &str {
        &self.thread_description
    }
    async fn user_messages(&self) -> &Vec<UserMessages> {
        &self.user_messages
    }
}
