#![feature(impl_trait_in_assoc_type)]
use std::collections::HashMap;
use std::sync::Mutex;
use volo_gen::mini_redis::RequestType;
use volo_gen::mini_redis::ResponseType;
use volo_gen::mini_redis::RedisResponse;
use anyhow::Error;
use anyhow::Ok;
pub struct S {
	pub data: Mutex<HashMap<String,String>>,
}

#[volo::async_trait]
impl volo_gen::mini_redis::RedisService for S {
	async fn redis_command(
		&self, 
		req: volo_gen::mini_redis::RedisRequest
	) -> ::core::result::Result<volo_gen::mini_redis::RedisResponse, ::volo_thrift::AnyhowError> {
		match req.req_type {
			RequestType::Get => {
				if let Some(get_result) = self.data.lock().unwrap().get(&req.key.unwrap().into_string()) {
					return Ok (
						RedisResponse {
							value: Some(get_result.clone().into()),
							resp_type: ResponseType::Output,
						}
					);
				}
				else {
					return Ok (
						RedisResponse {
							value: Some(format!("(nil)").into()),
							resp_type: ResponseType::Output,
						}
					);
				}
			}
			RequestType::Set => {
				self.data.lock().unwrap().insert(req.key.unwrap().into_string(), req.value.unwrap().into_string());
				Ok (
					RedisResponse {
						value: Some(format!("successfully set!",).into()),
						resp_type: ResponseType::Output,
					}
				)
			}
			RequestType::Del => {
				if let Some(_) = self.data.lock().unwrap().remove(&req.key.unwrap().into_string()) {
					return Ok (
						RedisResponse {
							value: Some(format!("successfully delete!").into()),
							resp_type: ResponseType::Output,
						}
					);
				}
				else {
					return Ok (
						RedisResponse {
							value: Some(format!("delete key not found.").into()),
							resp_type: ResponseType::Output,
						}
					);
				}
			}
			RequestType::Ping => {
				if req.key != None {
					Ok (
						RedisResponse {
							value: Some(req.key.unwrap().clone().into()),
							resp_type: ResponseType::Output,
						}
					)
				}
				else {
					Ok (
						RedisResponse {
							value: Some(format!("PONG").into()),
							resp_type: ResponseType::Output,
						}
					)
				}
			}
			RequestType::Subscribe => {Ok(Default::default())}
			RequestType::Publish => {Ok(Default::default())}
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
        //tracing::debug!("Received request {:?}", &req);
		let command = format!("{:?}", &req);
		if command.contains("illegal") {
			return Err(S::Error::from(Error::msg("Illegal instruction")));
		}
        let resp = self.0.call(cx, req).await;
        //tracing::debug!("Sent response {:?}", &resp);
        //tracing::debug!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}
pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}
