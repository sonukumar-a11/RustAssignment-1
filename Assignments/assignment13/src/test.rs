#[cfg(test)]
mod tests {
    use crate::book_structure::structure_book::BookCollection;

    #[test]
    fn structure_book_check() {
        let mut book1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Marcel Proust".to_string()],
            book_title: vec!["In search of lost time".to_string()],
            flag: 0,
        };
        let book2 = BookCollection {
            accession_number: vec![02],
            author_name: vec!["James Joyce".to_string()],
            book_title: vec!["Ulysses".to_string()],
            flag: 0,
        };
        assert_eq!(book1.display_content(), ());
        assert_eq!(book1.add_new_book(&book2), ());
        assert_eq!(book1.display_author_name("Marcel Proust".to_string()), ());
        assert_eq!(book1.display_number_of_books("Ulysses".to_string()), ());
        assert_eq!(book1.issue_book(&book2), ())
    }

    #[test]
    fn structure_book_test() {
        let mut book1_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Leo Tolstoy".to_string()],
            book_title: vec!["War and Piece".to_string()],
            flag: 0,
        };
        let book_check_2 = BookCollection {
            accession_number: vec![02],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 0,
        };
        assert_eq!(book1_check_1.display_content(), ());
        assert_eq!(book1_check_1.add_new_book(&book_check_2), ());
        assert_eq!(
            book1_check_1.display_author_name("Dante Alighieri".to_string()),
            ()
        );
        assert_eq!(
            book1_check_1.display_number_of_books("The Divine Comedy".to_string()),
            ()
        );
        assert_eq!(book1_check_1.issue_book(&book_check_2), ())
    }
}
