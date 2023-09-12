#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, sync::{RwLock, Arc}, str::FromStr};
use volo_gen::volo::example::{Item, ItemResponse};
use anyhow::Error;
pub struct S {
	item_dir : ItemDirc,
}

impl S {
	pub fn new() -> Self {
		Self { item_dir: ItemDirc::new() }
	}
}

struct ItemDirc {
	dir: Arc<RwLock<HashMap<String,String>>>,
}

impl ItemDirc {
	fn new() -> Self {
        Self {
            dir: Arc::new(RwLock::new(HashMap::new())),
        }
    }

	fn get(&self, key: &str) -> Option<String> {
		match self.dir.read().unwrap().get(key) {
			Some(value) => Some(value.clone()),
			None => None,
		}
	} 

	fn set(&self, key: &str, value: &str) -> Option<String> {
		self.dir.write().unwrap().insert(key.to_string(), value.to_string())
	}

	fn del(&self, key: &str) -> Option<String> {
        self.dir.write().unwrap().remove(key)
    }
}


#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
	async fn get(&self, _req: volo_gen::volo::example::KeyRequest) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>{
					let res = self.item_dir.get(&_req.key);
					match res {
						Some(value) => Ok(ItemResponse{
							item: Item { 
								key: _req.key.clone(), 
								value: Some(value.into()),
							},
						}),
						None => Ok(ItemResponse{
							item: Item { 
								key: _req.key.clone(), 
								value: None,
							},
						})
					}
				}
async fn set(&self, _req: volo_gen::volo::example::ItemRequest) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>{
					let _ = self.item_dir.set(&_req.item.key, &_req.item.value.unwrap());
					Ok(ItemResponse{
						item: Item { 
							key: _req.item.key.clone(), 
							value: None,
						},
					})
				}
async fn del(&self, _req: volo_gen::volo::example::KeyRequest) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>{
					let res = self.item_dir.del(&_req.key);
					match res {
						Some(_) => Ok(ItemResponse { item: {
							Item { 
								key: _req.key.clone(), 
								value: Some("-1".to_string().into()) }
						} }),
						None =>Ok(ItemResponse { item: {
							Item { 
								key: _req.key.clone(), 
								value: Some("-2".to_string().into()) }
						} }),
					}
				}
async fn ping(&self, _req: volo_gen::volo::example::ItemRequest) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>{
					match _req.item.value {
						Some(value) => Ok(ItemResponse {
							item: Item { key: _req.item.key.clone(), value: Some(value) }
						}),
						None => Ok(ItemResponse {
							item: Item { key: _req.item.key.clone(), value: Some("PONG".to_string().into()) }
						})
					}
				}
}


#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug + From<Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        let info: Vec<char> = format!("{req:?}").chars().collect();
		//过滤“12”
		let mut is_legal: bool = true;
		for i in 0..(info.len() - 1) {
			if info[i] == '1' && info[i + 1] == '2' {
				is_legal = false;
				break;
			}
		}
		if is_legal {
			let resp = self.0.call(cx, req).await;
			resp
		}else {
			Err(S::Error::from(Error::msg("连续的12过滤")))
		}
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

