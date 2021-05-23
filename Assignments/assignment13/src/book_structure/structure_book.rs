use log::*;

///BookCollection structure which used to encapsulate the menu of library
///
/// #field
///
/// accession_number:-a nth_value is usize object consider as remove value
///
/// author_name:-A author_name is Vector of string which is used to store the author of book.
///
/// book_title:-A book_title is Vector of string which is used to store the title of book
///
/// flag:-A flag is used to count the number of book in Collection.....
pub struct BookCollection {
    pub accession_number: Vec<i32>,
    pub author_name: Vec<String>,
    pub book_title: Vec<String>,
    pub flag: i32,
}

impl BookCollection {
    /// display_content which is used to display the content of BookCollection
    ///
    /// #Arguments
    ///
    /// No Arguments.
    ///
    /// #Return
    ///
    /// Returns Result type object which signify the Detail Displayed or not
    pub fn display_content(&self) -> Result<bool, i32> {
        if self.accession_number.is_empty() {
            error!("Can't do anything");
            return Err(0);
        }
        for index in 0..self.accession_number.len() {
            info!(
                "accession_number:{},Author_name: {} , book_title: {} , ",
                self.accession_number[index], self.author_name[index], self.book_title[index]
            );
        }
        Ok(true)
    }
    /// add_new_book which is used to add the another book detail
    ///
    /// #Arguments
    ///
    /// book_name :- A book_name is the Structure Object.
    ///
    /// #Return
    ///
    /// Returns Result type object which signify that add a new book or not.
    pub fn add_new_book(&mut self, book_name: &BookCollection) -> Result<bool, i32> {
        if self
            .accession_number
            .contains(&book_name.accession_number[0])
        {
            error!("Book already exists");
            return Err(0);
        }
        self.accession_number.push(book_name.accession_number[0]);

        let dir = book_name.author_name[0].clone();
        self.author_name.push(dir);
        let dir = book_name.book_title[0].clone();
        self.book_title.push(dir);
        self.flag += book_name.flag;

        Ok(true)
    }
    /// display_author_name which is used to display the number of book which related to given author
    ///
    /// #Arguments
    ///
    /// author_name_check :- A author_name_check is the String Object which is store the author_name.
    ///
    /// #Return
    ///
    /// Returns Result type object which signify that given author book is exist or not....
    pub fn display_author_name(&self, author_name_check: String) -> Result<bool, i32> {
        if !self.author_name.contains(&author_name_check) {
            error!("No data for this author");
            return Err(0);
        }
        for index in 0..self.author_name.len() {
            if author_name_check == self.author_name[index] {
                info!(
                    "accession_number:{},Author_name: {} , book_title: {} , ",
                    self.accession_number[index], self.author_name[index], self.book_title[index]
                )
            }
        }
        Ok(true)
    }
    /// display_related_to_title which is used to display the number of book which related to given title
    ///
    /// #Arguments
    ///
    /// title_check :- A title_check is the String Object which is store the title_name
    ///
    /// #Return
    ///
    /// Returns Result type object which signify that given title book is existed
    pub fn display_related_to_title(&self, title_check: String) -> Result<i32, i32> {
        if !self.book_title.contains(&title_check) {
            error!("No data releted to for this title");
            return Err(0);
        }
        let mut book_count = 0;
        for index in 0..self.book_title.len() {
            if title_check == self.book_title[index] {
                book_count += 1;
            }
        }
        if book_count == 0 {
            error!("Please give SomethingValid to check");
        } else {
            info!("number of bookCount {}", book_count)
        }
        Ok(1)
    }
    /// display_total_number_book which is used to display the total number of book.
    ///
    /// #Arguments
    ///
    /// No Args
    ///
    /// #Return
    ///
    /// Returns Result type object which signify that total number of books in library....
    pub fn display_total_number_book(&self) -> Result<i32, i32> {
        if self.book_title.is_empty() {
            error!("No book present their");
            Err(0)
        } else {
            Ok(self.book_title.len() as i32)
        }
    }
    /// issue_book which is used to issued the book for someone..
    ///
    /// #Arguments
    ///
    /// book_name:- A book_name is the Structure Object means which book issued
    ///
    /// #Return
    ///
    /// Returns Option<T> type object which issued book exist or not..........
    pub fn issue_book(&mut self, book_name: String) -> Option<i32> {
        let book_containing_index = self
            .book_title
            .iter()
            .position(|value| *value == book_name)
            .unwrap();
        {
            self.book_title.remove(book_containing_index);
            self.author_name.remove(book_containing_index);
            self.accession_number.remove(book_containing_index);
            self.flag -= 1;
            info!("book issued SuccessFully");
            Some(1)
        }
    }
}
