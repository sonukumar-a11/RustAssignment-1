#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::asynch::asynchronous::compute;
    use crate::synchronously::equate;

    #[test]
    fn tables_check_success() {
        assert_eq!(block_on(compute()), ());
    }

    #[test]
    fn synchronously_check() {
        assert_eq!(block_on(equate()), ());
    }
}
