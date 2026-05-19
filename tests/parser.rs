#[cfg(test)]
mod tests {
  use std::env;
  use dotenvy::dotenv;
  use beatport_rs::{cache_chart, cache_genre, parse_chart};

  #[test]
  fn test_parsing_100() {
    // https://api.beatport.com/v4/catalog/genres/90/top/100/?per_page=100&hype=false
    let result = parse_chart("tests/melodic-top-100.json".into()).unwrap();
    assert_eq!(result.results.len(), 100);
    let first = &result.results[0];
    assert_eq!(format!("{}", first), "28525970: Hot Sauce (Extended) -- Kapuchon, Miss Monique, GLZ [5:36]");
  }
  
  #[tokio::test]
  async fn test_genre_caching() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");
    let result = cache_genre(90, 100, &access_token).await.unwrap();
    assert_eq!(result.results.len(), 100);
    let first = &result.results[0];
    assert_eq!(format!("{}", first), "28525970: Hot Sauce (Extended) -- Kapuchon, Miss Monique, GLZ [5:36]");
  }
  
  #[tokio::test]
  async fn test_chart_caching() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");
    let result = cache_chart(880186, 100, &access_token).await.unwrap();
    assert_eq!(result.results.len(), 40);
    let first = &result.results[0];
    assert_eq!(format!("{}", first), "24258511: Dragons (Original Mix) -- Promising Youngster [6:00]");
  }
}
