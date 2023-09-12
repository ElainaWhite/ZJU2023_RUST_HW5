use lazy_static::lazy_static;
use mini_redis_volo::LogLayer;
use pilota::FastStr;
use volo_gen::volo::example::{KeyRequest, ItemRequest, Item};
use std::{net::SocketAddr, str::FromStr};

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_outer(LogLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    


    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: Vec<&str> = input.trim()
                                    .split(" ")
                                    .filter(|s| !s.is_empty())
                                    .collect();
        if input.is_empty() {
            continue;
        }
        match input[0].to_uppercase().as_str() {
            "GET" => {
                if input.len() != 2 {
                    println!("\nWrong number of arguments for command");
                    continue;
                }
                let key = input[1];
                let req = KeyRequest{
                    key: FastStr::from_str(key).unwrap()
                };
                let res = CLIENT.get(req).await.unwrap().item.value;
                match res {
                    Some(value) => {println!("\n Get value = {}",value);},
                    None =>  {println!("\n Get nothing");},
                }

            },
            "SET" => {
                if input.len() != 3 {
                    println!("\nWrong number of arguments for command");
                    continue;
                }
                let key = input[1];
                let value = input[2];
                let req = ItemRequest{
                    item: Item { key: FastStr::from_str(key).unwrap(), value:Some(FastStr::from_str(value).unwrap()) }
                };
                let _ = CLIENT.set(req).await;
                println!("\nSetted.")
            }
            "DEL" => {
                if input.len() != 2 {
                    println!("\nWrong number of arguments for command");
                    continue;
                }
                let key = input[1];
                let req = KeyRequest{
                    key: FastStr::from_str(key).unwrap()
                };
                let res = CLIENT.get(req).await.unwrap().item.value;
                match res {
                    Some(value) => {
                        if value == "-1" {
                            println!("\nDel Successd.");
                        }else {
                            println!("\nDel Failed.");
                        }
                    }
                    None =>  {println!("\nError");},
                }

            }
            "PING" => {
                if input.len() != 2 {
                    println!("\nWrong number of arguments for command");
                    continue;
                }
                let key = input[1];
                let value = input[1];
                let req = ItemRequest{
                    item: Item { key: FastStr::from_str(key).unwrap(), value:Some(FastStr::from_str(value).unwrap()) }
                };
                let value = CLIENT.set(req).await.unwrap().item.value;
                match value {
                    Some(v) => {println!("\n{}", v);},
                    None => {println!("\n PONG");}
                }
                
                
            }
            _ =>  {continue;}
        }
    }
}
