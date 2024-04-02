use async_graphql::{Context, Error, Object, Result, ID};
#[allow(unused_imports)]
use tracing::*;

use crate::graphql::model::User;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(name = "addUser")]
    async fn add_user(
        &self,
        _ctx: &Context<'_>,
        #[graphql(name = "username")] username: Option<String>,
        #[graphql(name = "authId")] auth_id: String,
        #[graphql(name = "email")] email: String,
        #[graphql(name = "firstName")] first_name: Option<String>,
        #[graphql(name = "lastName")] last_name: Option<String>,
        #[graphql(name = "avatar")] avatar: Option<String>,
        #[graphql(name = "isAdmin")] is_admin: Option<bool>,
    ) -> Result<User> {
        // let sub = ctx.data_unchecked::<Sub>();
        let user_id = uuid::Uuid::new_v4();
        debug!(
            user_id = ?user_id,
            username = ?username,
            auth_id = ?auth_id,
            email = ?email,
            first_name = ?first_name,
            last_name = ?last_name,
            is_admin = ?is_admin,
            "add user"
        );
        let user = User {
            user_id,
            username: username.unwrap_or_else(|| "".to_string()),
            auth_id,
            email,
            first_name: first_name.unwrap_or_else(|| "".to_string()),
            last_name: last_name.unwrap_or_else(|| "".to_string()),
            avatar: avatar.unwrap_or_else(|| "".to_string()),
            is_admin: is_admin.unwrap_or(false),
        };
        Ok(user)
    }

    #[allow(clippy::too_many_arguments)]
    #[graphql(name = "updateUser")]
    async fn update_user(
        &self,
        _ctx: &Context<'_>,
        #[graphql(name = "id")] _user_id: Option<ID>,
        #[graphql(name = "username")] username: Option<String>,
        #[graphql(name = "email")] email: Option<String>,
        #[graphql(name = "firstName")] first_name: Option<String>,
        #[graphql(name = "lastName")] last_name: Option<String>,
        #[graphql(name = "avatar")] avatar: Option<String>,
        #[graphql(name = "isAdmin")] is_admin: Option<bool>,
    ) -> Result<User> {
        if email.is_some() {
            return Err(Error::from("update of email is not yet allowed"));
        }

        // let user_id = match user_id {
        //     Some(id) => id.parse::<uuid::Uuid>()?,
        //     None => sub.user_id.clone(),
        // };

        let user_id = uuid::Uuid::new_v4();
        debug!(
            user_id = ?user_id,
            username = ?username,
            email = ?email,
            first_name = ?first_name,
            last_name = ?last_name,
            is_admin = ?is_admin,
            "add user"
        );
        let user = User {
            user_id,
            username: username.unwrap_or_else(|| "".to_string()),
            auth_id: "some_id".to_string(),
            email: email.unwrap_or_else(|| "".to_string()),
            first_name: first_name.unwrap_or_else(|| "".to_string()),
            last_name: last_name.unwrap_or_else(|| "".to_string()),
            avatar: avatar.unwrap_or_else(|| "".to_string()),
            is_admin: is_admin.unwrap_or(false),
        };
        Ok(user)
    }

    /// remove user by id
    /// # Arguments
    /// * `id` - user id
    /// # Examples
    /// ```graphql
    /// mutation removeUser {
    ///    removeUser(id: "062334c3-40a7-4d04-ba18-7782b4e35a7c")
    /// }
    /// ```
    #[graphql(name = "deleteUser")]
    async fn delete_user(&self, _ctx: &Context<'_>, _id: ID) -> Result<bool> {
        Ok(true)
    }
}
