#![allow(dead_code)]
pub mod bucket {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct BucketResponse {
        id: String,
        name: String,
        owner: String,
        public: bool,
        file_size_limit: Option<i64>,
        allowed_mime_types: Option<Vec<String>>,
        created_at: String,
        updated_at: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Bucket {
        id: String,
        name: String,
        public: bool,
        file_size_limit: Option<i64>,
        allowed_mime_types: Option<Vec<String>>,
    }

    impl Bucket {
        pub fn new(
            id: String,
            name: String,
            public: bool,
            file_size_limit: Option<i64>,
            allowed_mime_types: Option<Vec<String>>,
        ) -> Self {
            Bucket {
                id,
                name,
                public,
                file_size_limit,
                allowed_mime_types,
            }
        }
    }
    #[derive(Debug, Deserialize)]
    pub struct CreateBucketResponse {
        name: String,
    }

    #[derive(Debug, Serialize)]
    pub struct BucketConfig {
        public: bool,
        file_size_limit: Option<i64>,
        allowed_mime_types: Option<Vec<String>>,
    }
    impl BucketConfig {
        pub fn new(
            public: bool,
            file_size_limit: Option<i64>,
            allowed_mime_types: Option<Vec<String>>,
        ) -> Self {
            BucketConfig {
                public,
                file_size_limit,
                allowed_mime_types,
            }
        }
    }
}
