#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod bucket_error {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    pub struct BucketError {
        statusCode: String,
        error: String,
        message: String,
    }
    impl BucketError {
        pub fn new(status_code: String, error: String, message: String) -> Self {
            BucketError {
                statusCode: status_code,
                error,
                message,
            }
        }
    }
}
