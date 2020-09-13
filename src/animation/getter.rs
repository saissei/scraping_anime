use super::super::settings::config;

use crate::animation::models::anime;

pub async fn list(
  year: String,
  quoter: String,
) -> Result<Vec<anime::Anime>, Box<dyn std::error::Error + Send + Sync + 'static>> {
  let conf: config::AnimeConfig = config::load_config();
  let url = conf.url + "/" + &year + "/" + &quoter;
  let mut res = surf::get(url).await?;
  let anime: Vec<anime::Anime> = res.body_json().await?;
  Ok(anime)
}
