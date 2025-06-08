use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    // establish server connection with redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set key: "hello" and value "world"
    client.set("hello", "world".into()).await?;

    // get the value of "key=hello"
    let result = client.get("hello").await?;

    println!("get res from the server={:?}", result);
    Ok(())
}
