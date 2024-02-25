# storage-rs

Rust Client library to interact with Supabase Storage.

| :warning: WARNING          |
|:---------------------------|
| WARNING: This script is not ready for production use. |

## Installation

To install simple run the following command:

```bash
cargo add storage-supabase
```

or add the following to your `Cargo.toml` file:

```toml
[dependencies]
storage-supabase = "0.1.0"
```

## Usage

Add use statement to your code:

```rust
use storage_supabase::StorageClient;
```

Then that's it! You can now use the `StorageClient` to interact with Supabase Storage.

## Example

Here is an example of how to list all the buckets in your Supabase Storage asynchronously:

```rust
    let client = StorageClient::new("https://<your_project_id>.supabase.co", "<your_anon_key>");

    let bucket = client.list_buckets_async().await;

    match bucket {
        Ok(buckets) => {
            println!("Buckets: {:?}", buckets);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    } 
```
