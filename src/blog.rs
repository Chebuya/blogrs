use worker::{ *, kv::KvStore };
use chrono::{NaiveDate, Datelike};
use markdown;
const BLOG_ENTRY: &str = r#"<h1>POSTNAME</h1>"#;

pub async fn get_post(name: &String, storage: &KvStore) -> Option<(String, String, String, String)> {
  let content = storage.get(&name).text().await;

  match content {
    Ok(value) => {
      if let Some(post) = value {
        let (title, date, short_desc, content) = get_post_info(&post).await;
        let marked = markdown::to_html(&content.as_str());
        Some((title, date, short_desc, marked))
      } else {
        None
      }
    }
    Err(_) => None,
  }
}

pub async fn get_post_info(markdown: &String) -> (String, String, String, String) {
  let (title, date, short_desc, content) = markdown
    .split_once('\n')
    .map(|(short_title, long_description)| {
      let (a, bc) = short_title.split_once(' ').unwrap_or_else(|| panic!("invalid text format"));
      let (b, c) = bc.split_once(' ').unwrap_or_else(|| panic!("invalid text format"));
      (a.to_owned(), b.to_owned(), c.to_owned(), long_description.to_owned())
    })
    .unwrap();

  (title, date, short_desc, content)
}

pub async fn get_all_posts(context: &RouteContext<()>) -> String {
  let storage = context.kv("BLOG_POSTS").unwrap();
  let keys = storage.list().execute().await.unwrap().keys;
  let mut posts = String::new();

  for (_, value) in keys.iter().enumerate() {
    let (title, date, short_desc, _content) = get_post_info(
      &storage.get(&value.name).text().await.unwrap().unwrap()
    ).await;

    let date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").unwrap();
    let formatted_date = format!("{} {}", date.format("%B"), date.year());

    posts.push_str(r#"
<div class="post">
  <a href="http://localhost:8787/TITLE">
    <strong>
      * SHORT_DESC
    </strong>
  </a>
  <p class="date">DATE</p>
</div>
    "#
        .replace("TITLE", &title.as_str())
        .replace("SHORT_DESC", &short_desc.as_str())
        .replace("DATE", &formatted_date.as_str())
        .as_str()
    );
  }

  posts
}