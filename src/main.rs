#[macro_use]
extern crate serde_derive;

mod animation;

use animation::getter::list;
mod settings;

struct MinimalItem {
  pub id: usize,
  pub title: String,
}

#[async_std::main]
async fn main() {
  // Reslt型で返却
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
  let year = "2020".to_string();
  let quoter = "1".to_string();
  let anime = list(year, quoter).await;
  match anime {
    Ok(anime_iter) => {
      // for item in anime_iter {
      //  if item.id == 1103 {
      //    println!("{:#?}", item)
      //  }
      // }
      let minimal = anime_iter.iter().map(|item| {
        let id = item.id;
        let title = &item.title;
        MinimalItem {
          id: id,
          title: title.to_string(),
        };
      });
      println!("{:#?}", minimal);
    }
    _ => println!("failure"),
  }
}
