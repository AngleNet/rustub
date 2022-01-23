mod types;
mod catalog;
mod storage;
mod concurrency;
mod execution;
mod recovery;
mod common;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
