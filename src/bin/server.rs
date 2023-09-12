#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
use redis::LogLayer;
use redis::{S};
use std::sync::Mutex;
use std::collections::HashMap;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::mini_redis::RedisServiceServer::new(S {
        data: Mutex::new(HashMap::<String,String>::new()),
    })
        .layer_front(LogLayer)
        .run(addr)
        .await
        .unwrap();
}
