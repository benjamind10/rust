#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        //if let Media::Book { title, author } = self {
        //format!("Book: {} by {}", title, author)
        //} else if let Media::Movie { title, director } = self {
        //format!("Movie: {} directed by {}", title, director)
        //} else if let Media::Audiobook { title } = self {
        //format!("Audiobook: {}", title)
        //} else {
        //String::from("Unknown media type")
        //}
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} directed by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{}", media.description());
}

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

    print_media(audiobook);
    print_media(book);
    print_media(movie);
}
