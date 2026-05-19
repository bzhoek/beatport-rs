#[cfg(test)]
mod tests {
  use beatport_rs::{Page, Track};
  use std::fs::File;

  #[test]
  fn test_parsing_100() {
    // https://api.beatport.com/v4/catalog/genres/90/top/100/?per_page=100&hype=false
    let result: Page<Track> = serde_json::from_reader(File::open("tests/melodic-top-100.json").unwrap()).unwrap();
    assert_eq!(result.results.len(), 100);
    let first = &result.results[0];
    println!("{}", first);
    assert_eq!(first.id, 28525970);
    assert_eq!(first.artist(), "Kapuchon, Miss Monique, GLZ");
  }
}
