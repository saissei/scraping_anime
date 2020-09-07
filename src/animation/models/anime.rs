use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Anime {
  pub id: usize,
  pub title: String,
  pub title_short1: String,
  pub title_short2: String,
  pub title_short3: String,
  pub public_url: String,
  pub twitter_account: String,
  pub twitter_hash_tag: String,
  pub cours_id: usize,
  pub created_at: String,
  pub updated_at: String,
  pub sex: usize,
  pub sequel: usize,
  pub city_code: usize,
  pub city_name: String,
  pub product_companies: String,
}
