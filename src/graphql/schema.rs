use crate::graphql::ql::{UserMutation, UserQuery};
use async_graphql::{EmptySubscription, MergedObject, MergedSubscription, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation);

// #[derive(MergedSubscription, Default)]
// pub struct SubscriptionRoot();

pub type ChatSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
