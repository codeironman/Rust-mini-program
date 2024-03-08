#![allow(unused)]

use tokio::{net::{TcpListener,TcpStream}, sync::RwLockMappedWriteGuard};
use mini_redis::{Connection,Frame};



#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6479");
    loop{
        let ()
    }

}