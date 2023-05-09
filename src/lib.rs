use worker::{ *, worker_sys::{ response, request_init }, wasm_bindgen::JsValue };
mod utils;

static BLOG_HTML: &'static str = include_str!("../html/index.html");
static BLOG_CSS: &'static str = include_str!("../html/style.css");

fn log_request(req: &Request) {
	console_log!(
		"{} - [{}], located at: {:?}, within: {}",
		Date::now().to_string(),
		req.path(),
		req.cf().coordinates().unwrap_or_default(),
		req
			.cf()
			.region()
			.unwrap_or_else(|| "unknown region".into())
	);
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
	log_request(&req);
	utils::set_panic_hook();
	let router = Router::new();
	router
		.get_async("/", |req, _ctx| async move {
      let kv = _ctx.kv("KV_FROM_RUST").unwrap();
      let value = kv.get("calium").text().await.unwrap();
      console_log!("{}", value.unwrap());
			let mut req_headers = Headers::new();
			req_headers.set("Content-Type", "text/html");
			Ok(Response::ok(BLOG_HTML.replace("/* TO_REPLACE */", BLOG_CSS))?.with_headers(req_headers))
		})
		.get_async("/worker-version", |_, ctx| async move {
			let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
			Response::ok(version)
		})
		.run(req, env).await
}