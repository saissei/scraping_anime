mod animation;
use animation::getter::list;

/* struct MinimalItem {
  pub id: usize,
  pub title: String,
} */

#[async_std::main]
async fn main() {
  // Reslt型で返却
  let anime = list().await;
  /* let minimal = anime.iter().map(|item| {
    let id = item[0].id;
    let title = &item[0].title;
    MinimalItem {
      id: id,
      title: title.to_string(),
    };
  });
  println!("{:#?}", minimal); */
  /* println!("{:#?}", anime); */

  match anime {
    Ok(anime_iter) => {
      for item in anime_iter {
        if item.id == 1103 {
          println!("{:#?}", item)
        }
      }
    }
    _ => println!("failure"),
  }
}
