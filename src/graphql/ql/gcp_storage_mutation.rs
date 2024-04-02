use async_graphql::{Context, Error, Object, Result, ID};
#[allow(unused_imports)]
use tracing::*;

use crate::graphql::model::User;
use google_cloud_storage::client::Client;
use google_cloud_storage::client::ClientConfig;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use google_cloud_storage::sign::SignBy;
use google_cloud_storage::sign::SignedURLMethod;
use google_cloud_storage::sign::SignedURLOptions;

#[derive(Default)]
pub struct GcpStorageMutation;

#[Object]
impl GcpStorageMutation {
    #[graphql(name = "upload_file")]
    async fn add_user(
        &self,
        _ctx: &Context<'_>,
        #[graphql(name = "bucket")] bucket: String,
        #[graphql(name = "file_path")] file_path: String,
        #[graphql(name = "content")] content: String,
    ) -> Result<bool> {
        let config = ClientConfig::default().with_auth().await.unwrap();
        let client = Client::new(config);
        let upload_type = UploadType::Simple(Media::new(file_path.clone()));
        let _uploaded = client
            .upload_object(
                &UploadObjectRequest {
                    bucket: bucket.clone(),
                    ..Default::default()
                },
                content.clone(),
                &upload_type,
            )
            .await?;
        debug!(
            file_path = ?file_path,
            "upload file"
        );
        Ok(true)
    }
}
