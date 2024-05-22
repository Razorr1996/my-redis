use mini_redis::{client, Result as MiniResult};

#[tokio::main]
async fn main() -> MiniResult<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("localhost:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("get value from the server; result={:?}", result);

    Ok(())
}
