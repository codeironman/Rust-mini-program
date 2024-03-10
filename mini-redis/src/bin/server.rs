#![allow(unused)]

use bytes::Bytes;
use tokio::{net::{TcpListener,TcpStream}, sync::RwLockMappedWriteGuard};
use mini_redis::{cmd, frame, Connection, Frame};
use mini_redis::Command::{Get,Set,self};
use std::{collections::HashMap, path::PrefixComponent};
use std::sync::{Arc,Mutex};

type Database = Arc<Mutex<HashMap<String,Bytes>>>;

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listernning");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop{
        let (socket,_) = listener.accept().await.unwrap();
        let  db = db.clone();
        println!("Already Accept");
        tokio::spawn(async move{
            process(socket,db).await;
        });
    }

}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
enum Command{
    Get {
        key : String,
        resp : Resp
    }
}

async fn process(socket : TcpStream, db : Database){
    let mut connect = Connection::new(socket);
    
    while let Some(frame) = connect.read_frame().await.unwrap(){
        let responce = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(),cmd.value().clone() );
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let mut db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }
                else{
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented{:?}",cmd),
        };
        connect.write_frame(&responce).await.unwrap();
    }

}