//unused warning
#![allow(dead_code)]

pub mod storage_client {

    use crate::bucket::bucket::*;
    use crate::bucket_api::bucket_api::BucketApi;
    use crate::bucket_error::bucket_error::BucketError;

    const STORAGE_API_VERSION: &str = "v1";
    const STORAGE_API_URL: &str = "storage";

    pub struct StorageClient {
        superbase_url: String,
        superbase_key: String,
    }

    impl StorageClient {
        pub fn new(superbase_url: String, superbase_key: String) -> Self {
            StorageClient {
                superbase_url,
                superbase_key,
            }
        }
    }

    impl BucketApi for StorageClient {
        async fn list_buckets_async(&self) -> Result<Vec<Bucket>, BucketError> {
            let url = format!(
                "{}/{}/{}/bucket",
                self.superbase_url, STORAGE_API_URL, STORAGE_API_VERSION
            );

            let response = reqwest::Client::new()
                .get(&url)
                .header("Authorization", &self.superbase_key)
                .send()
                .await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        let body = response.text().await.unwrap();
                        let buckets: Vec<Bucket> = serde_json::from_str(&body).unwrap();
                        Ok(buckets)
                    } else {
                        let body = response.text().await.unwrap();
                        let bucket_error: BucketError = serde_json::from_str(&body).unwrap();
                        Err(bucket_error)
                    }
                }
                Err(e) => {
                    let status_code = "500".to_string();
                    let error = "Internal Server Error".to_string();
                    let message = e.to_string();
                    Err(BucketError::new(status_code, error, message))
                }
            }
        }

        async fn create_bucket_async(&self, bucket_request: Bucket) -> Result<(), BucketError> {
            let url = format!(
                "{}/{}/{}/bucket",
                self.superbase_url, STORAGE_API_URL, STORAGE_API_VERSION
            );
            let serialized: String = serde_json::to_string(&bucket_request).unwrap();
            let response = reqwest::Client::new()
                .post(&url)
                .header("Authorization", &self.superbase_key)
                .body(serialized)
                .send()
                .await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        Ok(())
                    } else {
                        let body = response.text().await.unwrap();
                        let bucket_error: BucketError = serde_json::from_str(&body).unwrap();
                        Err(bucket_error)
                    }
                }
                Err(e) => {
                    let status_code = "500".to_string();
                    let error = "Internal Server Error".to_string();
                    let message = e.to_string();
                    Err(BucketError::new(status_code, error, message))
                }
            }
        }

        async fn clear_bucket_async(&self, bucket_id: &str) -> Result<(), BucketError> {
            let url = format!(
                "{}/{}/{}/bucket/{}/empty",
                self.superbase_url, STORAGE_API_URL, STORAGE_API_VERSION, bucket_id
            );

            let response = reqwest::Client::new()
                .post(&url)
                .header("Authorization", &self.superbase_key)
                .send()
                .await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        Ok(())
                    } else {
                        let body = response.text().await.unwrap();
                        let bucket_error: BucketError = serde_json::from_str(&body).unwrap();
                        Err(bucket_error)
                    }
                }
                Err(e) => {
                    let status_code = "500".to_string();
                    let error = "Internal Server Error".to_string();
                    let message = e.to_string();
                    Err(BucketError::new(status_code, error, message))
                }
            }
        }

        //TODO put doesn't add body for some reason
        // async fn update_bucket_async(&self, bucket_id:&str, update_request:BucketConfig) -> Result<(), BucketError> {
        //     let url = format!("{}/{}/{}/bucket/{}", self.superbase_url, STORAGE_API_URL, STORAGE_API_VERSION, bucket_id);
        //     let serialized: String = serde_json::to_string(&update_request).unwrap();
        //     println!("{}", serialized);

        //     let response = reqwest::Client::new()
        //         .put(&url)
        //         .header("Authorization", &self.superbase_key)
        //         .body("update_request")
        //         .send()
        //         .await;

        //     match response {
        //         Ok(response) => {
        //             if response.status().is_success() {
        //                 Ok(())
        //             } else {
        //                 let body = response.text().await.unwrap();
        //                 let bucket_error: BucketError = serde_json::from_str(&body).unwrap();
        //                 Err(bucket_error)
        //             }
        //         }
        //         Err(e) => {
        //             let status_code = "500".to_string();
        //             let error = "Internal Server Error".to_string();
        //             let message = e.to_string();
        //             Err(BucketError::new(status_code, error, message))
        //         }
        //     }
        // }

        async fn delete_bucket_async(&self, bucket_id: &str) -> Result<(), BucketError> {
            let url = format!(
                "{}/{}/{}/bucket/{}",
                self.superbase_url, STORAGE_API_URL, STORAGE_API_VERSION, bucket_id
            );

            let response = reqwest::Client::new()
                .delete(&url)
                .header("Authorization", &self.superbase_key)
                .send()
                .await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        Ok(())
                    } else {
                        let body = response.text().await.unwrap();
                        let bucket_error: BucketError = serde_json::from_str(&body).unwrap();
                        Err(bucket_error)
                    }
                }
                Err(e) => {
                    let status_code = "500".to_string();
                    let error = "Internal Server Error".to_string();
                    let message = e.to_string();
                    Err(BucketError::new(status_code, error, message))
                }
            }
        }

        async fn get_bucket_async(&self, bucket_id: &str) -> Result<BucketResponse, BucketError> {
            let url = format!(
                "{}/{}/{}/bucket/{}",
                self.superbase_url, STORAGE_API_URL, STORAGE_API_VERSION, bucket_id
            );

            let response = reqwest::Client::new()
                .get(&url)
                .header("Authorization", &self.superbase_key)
                .send()
                .await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        let body = response.text().await.unwrap();
                        let bucket: BucketResponse = serde_json::from_str(&body).unwrap();
                        Ok(bucket)
                    } else {
                        let body = response.text().await.unwrap();
                        let bucket_error: BucketError = serde_json::from_str(&body).unwrap();
                        Err(bucket_error)
                    }
                }
                Err(e) => {
                    let status_code = "500".to_string();
                    let error = "Internal Server Error".to_string();
                    let message = e.to_string();
                    Err(BucketError::new(status_code, error, message))
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_creates_a_new_storage_client() {
            let superbase_url = "https://supabase.com".to_string();
            let superbase_key = "superbase_key".to_string();
            let storage_client = StorageClient::new(superbase_url, superbase_key);
            assert_eq!(storage_client.superbase_url, "https://supabase.com");
            assert_eq!(storage_client.superbase_key, "superbase_key");
        }
    }
}
