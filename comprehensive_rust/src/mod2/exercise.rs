#![allow(unused_variables, dead_code)]
pub mod q1 {
    /**
     * TODO: Design a library application in Rust.
     */

    #[derive(Debug)]
    pub struct Library {
        pub books: Vec<Book>,
    }

    #[derive(Debug)]
    pub struct Book {
        pub title: String,
        pub year: u16,
        pub isbn: u128,
    }

    impl Book {
        pub fn new(title: &str, year: u16, isbn: u128) -> Book {
            return Book {
                title: String::from(title),
                year: year,
                isbn: isbn,
            };
        }
    }

    impl std::fmt::Display for Book {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return write!(
                f,
                "{} (Year: {}, ISBN: {})",
                self.title, self.year, self.isbn
            );
        }
    }

    impl Library {
        pub fn new() -> Library {
            return Library { books: vec![] };
        }

        pub fn len(&self) -> usize {
            return self.books.len();
        }

        pub fn is_empty(&self) -> bool {
            return self.books.len() == 0;
        }

        pub fn add_book(&mut self, book: Book) {
            self.books.push(book);
        }

        pub fn print_books(&self) {
            for i in self.books.iter() {
                println!("{}", i);
            }
        }

        pub fn oldest_book(&self) -> Option<&Book> {
            // unimplemented!()
            return self.books.get(0);
        }
    }

    pub fn sample() {
        // This shows the desired behavior. Uncomment the code below and
        // implement the missing methods. You will need to update the
        // method signatures, including the "self" parameter!
        let mut library = Library::new();

        println!("Our library is empty: {}", library.is_empty());
        //
        library.add_book(Book::new("Lord of the Rings", 1954, 1));
        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865, 2));
        //
        library.print_books();
        //
        match library.oldest_book() {
            Some(book) => println!("My oldest book is {book}"),
            None => println!("My library is empty!"),
        }
        //
        println!("Our library has {} books", library.len());
    }
}
