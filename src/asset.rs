use std::collections::HashMap;
use lazy_static::lazy_static;
use worker::*;

#[derive(Debug)]
enum AssetType<'a> {
	Bytes(&'a [u8]),
	Str(&'a str),
}

lazy_static! {
  static ref ASSETS: HashMap<String, AssetType<'static>> = {
      let mut m = HashMap::new();
      m.insert(String::from("index.html"), AssetType::Str(include_str!("../html/index.html")));
      m.insert(String::from("style.css"), AssetType::Str(include_str!("../html/style.css")));
      m.insert(String::from("boymoder.png"), AssetType::Bytes(include_bytes!("../html/boymoder.png")));
      m
  };
}

fn get_mime(path: &str) -> &'static str {
	let ext = if let Some((_, ext)) = path.rsplit_once(".") {
		ext
	} else {
		return "text/plain";
	};

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
	let mut headers = Headers::new();
	let mime_type = get_mime(asset);
	headers.set("Content-Type", mime_type).unwrap();

  let body_raw = ASSETS.get(asset).unwrap();

  match body_raw {
    AssetType::Bytes(asset) => {
      Ok(Response::from_bytes(asset.to_vec())?.with_headers(headers))
    },
    AssetType::Str(asset) => {
      Ok(Response::ok(asset.to_owned())?.with_headers(headers))
    }
  }
}