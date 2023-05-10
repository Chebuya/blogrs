// use worker::{ *, worker_sys::{ response, request_init }, wasm_bindgen::{JsValue, UnwrapThrowExt} };
use worker::*;
mod asset;
mod utils;

// fn log_request(req: &Request) {
// 	console_log!(
// 		"{} - [{}], located at: {:?}, within: {}",
// 		Date::now().to_string(),
// 		req.path(),
// 		req.cf().coordinates().unwrap_or_default(),
// 		req
// 			.cf()
// 			.region()
// 			.unwrap_or_else(|| "unknown region".into())
// 	);
// }

#[event(fetch)]
pub async fn main(_req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
	utils::set_panic_hook();
	Router::new()
	  .get_async("/", |_req, _ctx| asset::serve(_req, _ctx))
	  .get_async("/:tttt", |_req, _ctx| asset::serve(_req, _ctx))
	  .run(_req, env).await
}