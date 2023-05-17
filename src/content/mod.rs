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

#[allow(deprecated)]
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
		if asset == "editor" {
			if _req.headers().has("Authorization").unwrap() {
				let credentials = String::from_utf8(
					base64
						::decode(_req.headers().get("Authorization").unwrap().unwrap().replace("Basic ", ""))
						.unwrap()
				).unwrap();

				console_log!("{}|{}", AUTH.replace("\n", ""), credentials);
				if credentials == AUTH.replace("\n", "") {
					Response::ok(EDITOR)
				} else {
					headers.set("WWW-Authenticate", "Basic realm=\"example\"").unwrap();
					Ok(Response::error("Unauthorized", 401)?.with_headers(headers))
				}
			} else {
				headers.set("WWW-Authenticate", "Basic realm=\"example\"").unwrap();
				Ok(Response::error("Unauthorized", 401)?.with_headers(headers))
			}
		} else {
			// let post = _ctx.kv("BLOG_POSTS").unwrap().get(asset).text().await;
			match blog::get_post(&asset.to_string(), &_ctx.kv("BLOG_POSTS").unwrap()).await {
				Some(value) => {
					Ok(Response::ok(POST.replace("<!-- POST -->", value.3.as_str()))?.with_headers(headers))
				}
				None => { Response::error("Not found", 404) }
			}
		}
	}
}