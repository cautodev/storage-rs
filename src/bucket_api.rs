#[allow(async_fn_in_trait)]

pub mod bucket_api {
    use crate::bucket::bucket::*;
    use crate::bucket_error::bucket_error::BucketError;

    pub trait BucketApi {
        async fn list_buckets_async(&self) -> Result<Vec<Bucket>, BucketError>;
        async fn create_bucket_async(&self, bucket_request: Bucket) -> Result<(), BucketError>;
        //async fn update_bucket_async(&self, bucket_id:&str, update_request:BucketConfig) -> Result<(), BucketError>;
        async fn delete_bucket_async(&self, bucket_id: &str) -> Result<(), BucketError>;
        async fn clear_bucket_async(&self, bucket_id: &str) -> Result<(), BucketError>;
        async fn get_bucket_async(&self, bucket_id: &str) -> Result<BucketResponse, BucketError>;
    }
}
