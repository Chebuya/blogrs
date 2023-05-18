use std::{ collections::HashMap, ffi::OsStr, path::Path };
use lazy_static::lazy_static;
use base64;
use worker::*;
mod blog;

#[derive(Debug)]
enum AssetType<'a> {
	Bytes(&'a [u8]),
	Str(&'a str),
}

static AUTH: &str = include_str!("/run/agenix/blogrs");
static POST: &str = include_str!("../../html/post.html");
static EDITOR: &str = include_str!("../../html/editor.html");

lazy_static! {
  static ref ASSETS: HashMap<String, AssetType<'static>> = {
      let mut m = HashMap::new();
      m.insert(String::from("index.html"), AssetType::Str(include_str!("../../html/index.html")));
      m.insert(String::from("style.css"), AssetType::Str(include_str!("../../html/style.css")));
      m.insert(String::from("post.html"), AssetType::Bytes(include_bytes!("../../html/post.html")));
      m.insert(String::from("avatar.png"), AssetType::Bytes(include_bytes!("../../html/avatar.png")));
      m.insert(String::from("boymoder.png"), AssetType::Bytes(include_bytes!("../../html/boymoder.png")));
      m
  };
}

#[allow(deprecated)]
fn get_auth(req: &Request) -> String {
	String::from_utf8(
		base64
			::decode(req.headers().get("Authorization").unwrap().unwrap().replace("Basic ", ""))
			.unwrap()
	).unwrap()
}

fn get_mime(ext: &str) -> &'static str {
	let mime_type = match ext {
		"html" => "text/html",
		"css" => "text/css",
		"js" => "text/javascript",
		"json" => "application/json",
		"png" => "image/png",
		"jpg" => "image/jpeg",
		"jpeg" => "image/jpeg",
		"ico" => "image/x-icon",
		"wasm" => "application/wasm",
		_ => "text/plain",
	};

	mime_type
}

pub async fn serve(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
	let asset = _ctx.param("tttt").map(String::as_str).unwrap_or("index.html");
	let extension = Path::new(asset)
		.extension()
		.unwrap_or(OsStr::new("html"))
		.to_string_lossy()
		.into_owned();
	let mut headers = Headers::new();
	let mime_type = get_mime(extension.as_str());
	headers.set("Content-Type", mime_type).unwrap();

	if ASSETS.contains_key(asset) {
		let asset_raw = ASSETS.get(asset).unwrap();

		match asset_raw {
			AssetType::Bytes(asset_raw) => {
				Ok(Response::from_bytes(asset_raw.to_vec())?.with_headers(headers))
			}
			AssetType::Str(asset_raw) => {
				if asset == "index.html" {
					Ok(
						Response::ok(
							asset_raw
								.to_owned()
								.replace("<!-- BLOG_POSTS -->", blog::get_all_posts(&_ctx).await.as_str())
						)?.with_headers(headers)
					)
				} else {
					Ok(Response::ok(asset_raw.to_owned())?.with_headers(headers))
				}
			}
		}
	} else {
		if asset == "editor.html" {
			if _req.headers().has("Authorization").unwrap() {
				let credentials = get_auth(&_req);

				if credentials == AUTH.replace("\n", "") {
					Ok(Response::ok(EDITOR)?.with_headers(headers))
				} else {
					headers.set("WWW-Authenticate", "Basic realm=\"example\"").unwrap();
					Ok(Response::error("Unauthorized", 401)?.with_headers(headers))
				}
			} else {
				headers.set("WWW-Authenticate", "Basic realm=\"example\"").unwrap();
				Ok(Response::error("Unauthorized", 401)?.with_headers(headers))
			}
		} else {
			Response::error("Not found", 404)
		}
	}
}

pub async fn serve_post(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
	let asset = _ctx.param("bbbb").map(String::as_str);
	let mut headers = Headers::new();
	headers.set("Content-Type", "text/html").unwrap();

	match asset {
		Some(value) => {
			match blog::get_post(&value.to_string(), &_ctx.kv("BLOG_POSTS").unwrap()).await {
				Some(value) => {
					let mut post_final = POST.replace("<!-- POST -->", value.3.as_str()).replace("TITLETOREPLACE", value.2.as_str());

					if _req.headers().has("Authorization").unwrap() {
						let credentials = get_auth(&_req);

						if credentials == AUTH.replace("\n", "") {
							post_final = post_final.replace("display: none; /* EDIT */", "display: visible;");
						}
					}

					Ok(Response::ok(post_final)?.with_headers(headers))
				}
				None => { Response::error("Not found", 404) }
			}
		}
		_ => { Response::error("Not found", 404) }
	}
}

pub async fn post(mut _req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
	if _req.headers().has("Authorization").unwrap() {
		let credentials = get_auth(&_req);
		// (title, date, short_desc, _content
		if credentials == AUTH.replace("\n", "") {
			let content = &_req.text().await.unwrap();
			let post = blog::get_post_info(&content).await;

			match post {
				Some(value) => {
					_ctx.kv("BLOG_POSTS").unwrap().put(value.0.as_str(), content).unwrap().execute().await.unwrap();
					Response::ok("Ok")
				}
				_ => { Response::error("Malformed string", 400) }
			}
		} else {
			Response::error("Unauthorized", 401)
		}
	} else {
		Response::error("Unauthorized", 401)
	}
}