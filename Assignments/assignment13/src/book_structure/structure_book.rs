use log::*;

///BookCollection structure which used to encapsulate the menu of library
///
/// #field
///
/// accession_number:-a nth_value is usize object consider as remove value
///
/// author_name:-A iterable is Vector object which contains the list of numbers
///
/// book_title:-
///
/// flag:-
pub struct BookCollection {
    pub accession_number: Vec<i32>,
    pub author_name: Vec<String>,
    pub book_title: Vec<String>,
    pub flag: i32,
}

impl BookCollection {
    pub fn display_content(&self) {
        for index in 0..self.author_name.len() {
            info!(
                "accession_number:{},Author_name: {} , book_title: {} , ",
                self.accession_number[index], self.author_name[index], self.book_title[index]
            );
        }
    }

    pub fn add_new_book(&mut self, book_name: &BookCollection) {
        self.accession_number.push(book_name.accession_number[0]);

        let dir = book_name.author_name[0].clone();
        self.author_name.push(dir);
        let dir = book_name.book_title[0].clone();
        self.book_title.push(dir);
        self.flag += 1;
    }
    pub fn display_author_name(&self, author_name_check: String) {
        for index in 0..self.author_name.len() {
            if author_name_check == self.author_name[index] {
                info!("Author_name {}", self.book_title[index]);
            }
        }
    }
    pub fn display_number_of_books(&self, title_check: String) {
        let mut book_count = 0;
        for index in 0..self.book_title.len() {
            if title_check == self.book_title[index] {
                book_count += 1;
            }
        }
        if book_count == 0 {
            error!("Please give Something to check");
        } else {
            info!("number of bookCount {}", book_count)
        }
    }
    pub fn display_total_number_book(&self) {
        if self.book_title.is_empty() {
            error!("Empty Database");
        } else {
            info!("number of total_number_book {}", self.book_title.len())
        }
    }
    pub fn issue_book(&mut self, book_name: &BookCollection) {
        let index = self
            .book_title
            .iter()
            .position(|value| *value == book_name.book_title[0])
            .unwrap();
        {
            self.book_title.remove(index);
            self.author_name.remove(index);
            self.accession_number.remove(index);
            self.flag -= 1;
        }
    }
}
