use lazy_static::lazy_static;
use std::net::SocketAddr;
use redis::LogLayer;
use std::process;
use std::sync::Arc;
use volo::FastStr;
use volo_gen::mini_redis::{RedisRequest,RequestType,};
use std::io;


lazy_static! {
    static ref CLIENT: volo_gen::mini_redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
            .layer_outer(LogLayer)    
            .address(addr)
            .build()
    };
}
#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.strip_suffix("\n").unwrap().to_string();
        let arguments: Vec<String> = input.split(' ').map(|str| str.to_string()).collect();
        let mut flag = 0;
        //let mut arguments: Vec<String> = std::env::args().collect();
        if arguments[0] == "exit" {
            flag = 1;
            println!("successfully exit!");
            break;
        }
        let req = match arguments[0].as_str() {
            "get" => {
                RedisRequest {
                    key: Some(FastStr::from(Arc::new(arguments.get(1).unwrap().clone()))),
                    value: None,
                    extime: None,
                    req_type: RequestType::Get,
                }
            }
            "set" => {
                RedisRequest {
                   // key: Some(vec![arguments.get(1).unwrap().clone().into()]),
                    key: Some(FastStr::from(Arc::new(arguments.get(1).unwrap().clone()))),
                    value: Some(arguments.get(2).unwrap().clone().into()),
                    extime: None,
                    req_type: RequestType::Set,
                }
            }
            "del" => {
                RedisRequest {
                    key: Some(FastStr::from(Arc::new(arguments.get(1).unwrap().clone()))),
                    value: None,
                    extime: None,
                    req_type: RequestType::Del,
                }
            }
            "ping" => {
                if arguments.len() == 2 {
                    RedisRequest {
                        key: Some(FastStr::from(Arc::new(arguments.get(1).unwrap().clone()))),
                        value: None,
                        extime: None,
                        req_type: RequestType::Ping,
                    }
                }
                else {
                    RedisRequest {
                        key: None,
                        value: None,
                        extime: None,
                        req_type: RequestType::Ping,
                    }
                }
            }
            _ => {
                panic!("illegal command!");
            }
        };
        let resp = CLIENT.redis_command(req).await;
        if flag == 1 {
            process::exit(0);
        }
        match resp {
            Ok(info) => println!("{:?}", info.value.unwrap()),
            Err(e) => tracing::error!("{:?}", e),
        }
    }
    
}