#[cfg(test)]
mod tests {
    use crate::book_structure::structure_book::BookCollection;

    #[test]
    fn structure_book_check() {
        env_logger::init();
        let book_check_1 = BookCollection {
            accession_number: vec![01, 02],
            author_name: vec!["Leo Tolstoy".to_string(), "Dante Alighieri".to_string()],
            book_title: vec!["War and Piece".to_string(), "The Divine Comedy".to_string()],
            flag: 2,
        };
        assert_eq!(book_check_1.display_content(), Ok(true));
    }

    #[test]
    fn structure_book_displayed_check_next() {
        let book_check_1 = BookCollection {
            accession_number: vec![],
            author_name: vec![],
            book_title: vec![],
            flag: 0,
        };
        assert_eq!(book_check_1.display_content(), Err(0));
    }

    #[test]
    pub fn add_new_book_check() {
        let mut book_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 1,
        };
        let book_check_2 = BookCollection {
            accession_number: vec![02],
            author_name: vec!["Leo Tolstoy".to_string()],
            book_title: vec!["Lolita".to_string()],
            flag: 1,
        };
        assert_eq!(book_check_1.add_new_book(&book_check_2), Ok(true));
    }

    #[test]
    pub fn display_author_name_check() {
        let book_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 1,
        };
        assert_eq!(
            book_check_1.display_author_name("Dante Alighieri".to_string()),
            Ok(true)
        );
    }

    #[test]
    pub fn display_author_name_check_next() {
        let book_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 1,
        };
        assert_eq!(book_check_1.display_author_name("Sonu".to_string()), Err(0));
    }

    #[test]
    fn display_book_title_check() {
        let book_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 1,
        };

        assert_eq!(
            book_check_1.display_related_to_title("The Divine Comedy".to_string()),
            Ok(1)
        );
    }

    #[test]
    fn display_book_title_check_next() {
        let book_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 1,
        };

        assert_eq!(
            book_check_1.display_related_to_title("Samsung".parse().unwrap()),
            Err(0)
        );
    }

    #[test]
    fn count_book_check() {
        let book_check_1 = BookCollection {
            accession_number: vec![01],
            author_name: vec!["Dante Alighieri".to_string()],
            book_title: vec!["The Divine Comedy".to_string()],
            flag: 1,
        };

        assert_eq!(book_check_1.display_total_number_book(), Ok(1));
    }

    #[test]
    fn count_book_check_next() {
        let book_check_1 = BookCollection {
            accession_number: vec![],
            author_name: vec![],
            book_title: vec![],
            flag: 1,
        };

        assert_eq!(book_check_1.display_total_number_book(), Err(0));
    }

    #[test]
    fn issue_book_check() {
        let mut book_check_1 = BookCollection {
            accession_number: vec![01, 02],
            author_name: vec!["Leo Tolstoy".to_string(), "Dante Alighieri".to_string()],
            book_title: vec!["War and Piece".to_string(), "The Divine Comedy".to_string()],
            flag: 2,
        };

        assert_eq!(
            book_check_1.issue_book("The Divine Comedy".parse().unwrap()),
            ()
        );
    }
}
