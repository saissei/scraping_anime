mod animation;
use animation::getter::list;
use serde::{Deserialize, Serialize};

#[async_std::main]
async fn main() {
  #[derive(Deserialize, Serialize)]
  let anime = list().await;
  for item in anime {
    let data = &item[0];
    println!("{:?}", data)
  }
}
