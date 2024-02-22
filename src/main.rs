// Publication enum definition
enum Publication {
    Book(Book),
    Magazine(Magazine),
}

// Book struct definition
struct Book {
    title: String,
    author: String,
    page_count: u32,
}

//Magazine struct definition
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

// A function that creates examples of books and magazines and adds them to a Vec<Publication> array.
fn create_publications() -> Vec<Publication> {
    let book1 = Book {
        title: String::from("Rust Programming By Example"),
        author: String::from("A. Sharma"),
        page_count: 350,
    };

    let book2 = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("S. Klabnik"),
        page_count: 550,
    };

    let magazine1 = Magazine {
        title: String::from("Rust Gazette"),
        issue: 10,
        topic: String::from("Rust Community News"),
    };

    let magazine2 = Magazine {
        title: String::from("Rust Journal"),
        issue: 5,
        topic: String::from("Advanced Rust Techniques"),
    };

    let mut publications = Vec::new();
    publications.push(Publication::Book(book1));
    publications.push(Publication::Book(book2));
    publications.push(Publication::Magazine(magazine1));
    publications.push(Publication::Magazine(magazine2));

    publications
}

// A function that prints the publication based on its type.
fn print_publication(publication: &Publication) {
    match publication {
        Publication::Book(book) => {
            println!(
                "Book: {} Author: {}, {} Page Count:",
                book.title, book.author, book.page_count
            );
        }
        Publication::Magazine(magazine) => {
            println!(
                "Magazine: {} - Issue: {}, Topic: {}",
                magazine.title, magazine.issue, magazine.topic
            );
        }
    }
}

fn main() {
    let publications = create_publications();

    for publication in &publications {
        print_publication(publication);
    }
}
