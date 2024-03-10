
use bytes::Bytes;
use mini_redis::{client,Result};
use tokio::sync::mpsc;


enum Command {
    Get{
        key : String,
    },
    Set {
        key : String,
        value : Bytes,
    }
}

#[tokio::main]
async fn main() -> Result<()>{

    let (tx,mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move { 
        let cmd = Command::Get { key: "hello".to_string() };
        tx.send(cmd).await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
             key: "foo".to_string(), 
             value: "bar".into() };
        tx2.send(cmd).await;
    });
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get{key} => {
                    client.get(&key).await;
                }
                Set{key , value} => {
                    client.set(&key, value).await;
                }
    
            }
        }
    });


    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
    Ok(())

}