use mini_redis::{Client, Result};


const ADDR: &str = "127.0.0.1:6379";

/// ouvre la connexion avec le serveur 'Redis' local
async fn connect() -> Result<Client> {
    Client::connect(ADDR).await
}

/// stocke une valeur dans 'Redis' via sa clé donnée
pub async fn set(cle: &str, val: &str) -> Result<()> {
    connect().await?.set(cle, val.to_string().into()).await
}

/// récupère la valeur associé à la clé donnée
#[allow(dead_code)]
pub async fn get(cle: &str) -> Result<Option<String>> {
    
    let val: Option<bytes::Bytes> = connect().await?.get(cle).await?;
    // convertit les bytes en strings
    Ok(val.and_then(|b| 
        String::from_utf8(b.to_vec()).ok()))
}