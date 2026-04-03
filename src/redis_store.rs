use mini_redis::{Client, Result};

const ADDR: &str = "127.0.0.1:6379";

async fn connect() -> Result<Client> {
    Client::connect(ADDR).await
}

pub async fn set(cle: &str, val: &str) -> Result<()> {
    connect().await?.set(cle, val.to_string().into()).await
}

#[allow(dead_code)]
pub async fn get(cle: &str) -> Result<Option<String>> {
    let cle = cle.to_string();
    let val: Option<bytes::Bytes> = connect().await?.get(&cle).await?;
    Ok(val.and_then(|b| String::from_utf8(b.to_vec()).ok()))
}