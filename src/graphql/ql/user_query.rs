use crate::graphql::model::User;
use async_graphql::{Context, Object, Result, ID};
#[allow(unused_imports)]
use tracing::*;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(name = "user")]
    async fn user(&self, _ctx: &Context<'_>) -> Result<User> {
        debug!("query user");

        Ok(User {
            user_id: uuid::Uuid::new_v4(),
            username: "test".to_string(),
            auth_id: "test".to_string(),
            email: "test".to_string(),
            first_name: "test".to_string(),
            last_name: "test".to_string(),
            avatar: "test".to_string(),
            is_admin: false,
        })
    }
}
