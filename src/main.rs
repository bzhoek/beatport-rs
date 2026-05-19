use std::env;
use std::fs::File;
use std::io::Write;
use dotenvy::dotenv;
use beatport_rs::get_chart;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();
  let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");

  let chart = 880186;
  let body = get_chart(chart, 100, &access_token).await?;
  let filename = format!("response-{}.json", chart);
  let mut file = File::create(filename)?;
  file.write_all(body.as_bytes())?;
  Ok(())
}
