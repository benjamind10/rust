#[derive(Debug)]
enum Media {
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    Audiobook {
        title: String,
    },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        //if let Media::Book { title, author } = self {
        //format!("Book: {} by {}", titwle, author)
        //} else if let Media::Movie { title, director } = self {
        //format!("Movie: {} directed by {}", title, director)
        //} else if let Media::Audiobook { title } = self {
        //format!("Audiobook: {}", title)
        //} else {
        //String::from("Unknown media type")
        //}
        match self {
            Media::Book { title, author } => { format!("Book: {} by {}", title, author) }
            Media::Movie { title, director } => {
                format!("Movie: {} directed by {}", title, director)
            }
            Media::Audiobook { title } => { format!("Audiobook: {}", title) }
            Media::Podcast(id) => { format!("Podcast: {}", id) }
            Media::Placeholder => { format!("Placeholder") }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            // We don't have anything to return
            None
        }
    }
}

// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable,
// }

// fn print_media(media: Media) {
//     println!("{}", media.description());
// }

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
