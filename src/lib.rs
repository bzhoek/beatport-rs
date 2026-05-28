use std::{fmt, fs};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Page<T> {
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<T>,
  pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Track {
  pub id: u32,
  pub isrc: Option<String>,
  pub artists: Vec<Artist>,
  pub bpm: u16,
  pub genre: Genre,
  #[serde(rename = "is_available_for_streaming")]
  pub streamed: bool,
  #[serde(rename = "publish_date")]
  pub published: String,
  #[serde(rename = "name")]
  pub title: String,
  #[serde(rename = "mix_name")]
  pub version: String,
  #[serde(rename = "length_ms")]
  pub duration: u32,
  pub key: Key,
}

impl Track {
  pub fn artist(&self) -> String {
    self.artists.iter()
      .map(|a| a.name.as_str())
      .collect::<Vec<&str>>()
      .join(", ")
  }

  pub fn title(&self) -> String {
    if self.version.is_empty() {
      self.title.clone()
    } else {
      format!("{} ({})", self.title, self.version)
    }
  }
}

impl fmt::Display for Track {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:08}: {} -- {} [{}:{:02}]", self.id, self.title(), self.artist(), self.duration / 60000, self.duration % 60000 / 1000)
  }
}

#[derive(Deserialize, Debug)]
pub struct Artist {
  pub id: u32,
  pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Genre {
  pub id: u32,
  pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Key {
  pub id: u32,
  pub name: String,
  #[serde(rename = "camelot_number")]
  pub camelot: u8,
  #[serde(rename = "camelot_letter")]
  pub suffix: String,
}

pub async fn cache_chart(id: u32, count: u32, access_token: &str) -> anyhow::Result<Page<Track>> {
  cache_chart_in(".", id, count, access_token).await
}

pub async fn cache_chart_in(folder: &str, id: u32, count: u32, access_token: &str) -> anyhow::Result<Page<Track>> {
  let folder = Path::new(&folder);
  let filename = format!("chart-{}-{}.json", id, count);
  let path = folder.join(filename);
  let body = if path.exists() {
    fs::read_to_string(path)?
  } else {
    let body = get_chart(id, count, access_token).await?;
    cache_body(body, &path)?
  };
  let page: Page<Track> = serde_json::from_str(&body)?;
  Ok(page)
}

pub async fn get_chart(id: u32, count: u32, access_token: &str) -> anyhow::Result<String> {
  let url = format!("https://api.beatport.com/v4/catalog/charts/{}/tracks/?per_page={}", id, count);
  get_body(&url, access_token).await
}

pub async fn cache_genre(id: u32, count: u32, access_token: &str) -> anyhow::Result<Page<Track>> {
  cache_genre_in(".", id, count, access_token).await
}

pub async fn cache_genre_in(folder: &str, id: u32, count: u32, access_token: &str) -> anyhow::Result<Page<Track>> {
  let folder = Path::new(&folder);
  let filename = format!("genre-{:02}-{:03}.json", id, count);
  let path = folder.join(filename);
  let body = if path.exists() {
    fs::read_to_string(path)?
  } else {
    let body = get_genre(id, count, access_token).await?;
    cache_body(body, &path.into())?
  };
  let page: Page<Track> = serde_json::from_str(&body)?;
  Ok(page)
}

pub async fn get_genre(id: u32, count: u32, access_token: &str) -> anyhow::Result<String> {
  let url = format!("https://api.beatport.com/v4/catalog/genres/{}/top/100/?per_page={}&hype=false", id, count);
  get_body(&url, access_token).await
}

pub fn parse_chart(path_buf: PathBuf) -> anyhow::Result<Page<Track>> {
  let file = File::open(path_buf)?;
  let result: Page<Track> = serde_json::from_reader(file)?;
  Ok(result)
}

fn cache_body(body: String, path: &PathBuf) -> anyhow::Result<String> {
  let mut file = File::create(path)?;
  file.write_all(body.as_bytes())?;
  Ok(body)
}

async fn get_body(url: &str, access_token: &str) -> anyhow::Result<String> {
  let client = reqwest::Client::new();
  let response = client.get(url)
    .bearer_auth(access_token)
    .send()
    .await?;

  let body = response.text().await?;
  Ok(body)
}
