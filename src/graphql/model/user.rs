use async_graphql::{Object, ID};
use serde::{Deserialize, Serialize};
use tracing_json2::JsonTracing;
use uuid::Uuid;

#[derive(JsonTracing, Clone, Default, Serialize, Deserialize)]
pub struct User {
    /// record unique id
    pub user_id: Uuid,
    /// reference to auth provider
    pub auth_id: String,

    /// username
    pub username: String,
    /// admin flag
    pub is_admin: bool,

    /// user email address
    pub email: String,

    /// avatar url (todo: implement)
    pub avatar: String,

    /// first name
    pub first_name: String,
    /// last name
    pub last_name: String,
}

#[Object]
/// user object
impl User {
    /// unique id of the record
    #[graphql(name = "id")]
    async fn id(&self) -> ID {
        self.user_id.into()
    }

    /// user name
    #[graphql(name = "username")]
    async fn username(&self) -> &str {
        &self.username
    }
    /// admin flag
    #[graphql(name = "isAdmin")]
    async fn is_admin(&self) -> bool {
        self.is_admin
    }
    /// user email address
    #[graphql(name = "email")]
    async fn email(&self) -> &str {
        &self.email
    }

    /// first name
    #[graphql(name = "firstName")]
    async fn first_name(&self) -> &str {
        &self.first_name
    }
    /// last name
    #[graphql(name = "lastName")]
    async fn last_name(&self) -> &str {
        &self.last_name
    }

    #[graphql(name = "avatar")]
    async fn avatar(&self) -> &str {
        &self.avatar
    }
}
