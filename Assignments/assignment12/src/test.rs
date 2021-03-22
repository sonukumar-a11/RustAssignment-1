#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::asynch::asynchronous::check_table;
    use crate::asynchronously::equate;

    #[test]
    fn tables_check_success() {
        assert_eq!(block_on(check_table()), ());
    }

    #[test]
    fn synchronously_check() {
        assert_eq!(block_on(equate()), ());
    }
}
