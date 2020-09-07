use crate::animation::models::anime;

pub async fn list() -> Result<Vec<anime::Anime>, Box<dyn std::error::Error + Send + Sync + 'static>>
{
  let url = "http://api.moemoe.tokyo/anime/v1/master/2020/1";
  let mut res = surf::get(url).await?;
  let anime: Vec<anime::Anime> = res.body_json().await?;
  Ok(anime)
}
