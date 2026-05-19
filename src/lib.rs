use std::fmt;
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
  pub isrc: String,
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
