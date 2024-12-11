pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
        }
    }

    pub fn display(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Year: {}", self.year);
    }
}
