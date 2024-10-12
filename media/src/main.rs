mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };

    let book = Media::Book {
        title: String::from("Rust Programming"),
        author: String::from("Steve Klabnik"),
    };

    let movie = Media::Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;

    // println!("{:#?}", item.unwrap())
    // println!("{:#?}", item.expect("expected there to be an item here!"))
    println!("{:#?}", item.unwrap_or(&placeholder))
}
